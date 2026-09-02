#!/usr/bin/env python3
"""Fill msgstr entries in a .po file from a JSON {msgid: msgstr} map.

Usage: po_fill.py <locale.po> <translations.json>
- Matches msgid blocks exactly (handles multiline, escapes).
- Refuses to overwrite an already-translated entry (idempotent).
- Prints msgfmt stats after.
"""
import json, re, subprocess, sys

def parse_po(path):
    raw = open(path, encoding="utf-8").read()
    # split into blocks separated by blank lines
    blocks = re.split(r"\n\n", raw)
    return blocks

def unescape(s):
    return s.replace('\\"', '"').replace("\\n", "\n").replace("\\\\", "\\")

def escape(s):
    return s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")

def block_msgid(block):
    ids = re.findall(r'^msgid ((?:"(?:[^"\\]|\\.)*"\n?)+)', block, re.M)
    if not ids:
        return None
    parts = re.findall(r'"((?:[^"\\]|\\.)*)"', ids[0])
    return "".join(unescape(p) for p in parts)

def fill_block(block, tr):
    mid = block_msgid(block)
    if mid is None or mid not in tr or not tr[mid]:
        return block, False
    if re.search(r'^msgstr "[^"]', block, re.M):
        return block, False  # already translated
    val = escape(tr[mid])
    if len(val) > 70 or "\\n" in val:
        # multiline msgstr
        chunks = val.split("\\n")
        lines = []
        for i, c in enumerate(chunks):
            c2 = c + ("\\n" if i < len(chunks) - 1 else "")
            lines.append(f'"{c2}"')
        body = "\n".join(lines)
        return re.sub(r'^msgstr ""\n?', (lambda _m: 'msgstr ""\n' + body + '\n'), block, count=1, flags=re.M), True
    return re.sub(r'^msgstr ""', (lambda _m: 'msgstr "' + val + '"'), block, count=1, flags=re.M), True

def main():
    po, jf = sys.argv[1], sys.argv[2]
    tr = json.load(open(jf, encoding="utf-8"))
    blocks = parse_po(po)
    filled = 0
    out = []
    for b in blocks:
        nb, did = fill_block(b, tr)
        filled += did
        out.append(nb)
    open(po, "w", encoding="utf-8").write("\n\n".join(out))
    print(f"filled {filled} entries")
    r = subprocess.run(["msgfmt", "--statistics", "-o", "/dev/null", po], capture_output=True, text=True)
    print(r.stderr.strip())

if __name__ == "__main__":
    main()
