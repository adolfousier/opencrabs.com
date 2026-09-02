#!/usr/bin/env python3
"""Extract untranslated msgids for given source files from a .po file.

Usage: po_extract.py <locale.po> <src/file.md> [<src/file2.md> ...]
Prints JSON array of msgids (deduped, in file order).
"""
import json, re, sys

def block_msgid(block):
    ids = re.findall(r'^msgid ((?:"(?:[^"\\]|\\.)*"\n?)+)', block, re.M)
    if not ids:
        return None
    parts = re.findall(r'"((?:[^"\\]|\\.)*)"', ids[0])
    s = "".join(p.replace('\\"', '"').replace("\\n", "\n").replace("\\\\", "\\") for p in parts)
    return s

def main():
    po = sys.argv[1]
    targets = sys.argv[2:]
    raw = open(po, encoding="utf-8").read()
    blocks = re.split(r"\n\n", raw)
    seen, result = set(), []
    for b in blocks:
        refs = re.findall(r'^#: (\S+)', b, re.M)
        if not any(any(t in r for t in targets) for r in refs):
            continue
        if re.search(r'^msgstr "[^"]', b, re.M):
            continue  # already translated
        mid = block_msgid(b)
        if mid and mid not in seen and mid != "":
            seen.add(mid)
            result.append(mid)
    print(json.dumps(result, ensure_ascii=False, indent=0))

if __name__ == "__main__":
    main()
