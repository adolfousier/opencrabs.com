#!/usr/bin/env python3
"""Generate the theme gallery table for docs/src/features/themes.md.

Reads the OpenCrabs source tree and emits one markdown table row per
built-in theme: the 8 hand-built presets (crab-dark first) and the 31
curated pack files, in the same order /theme list uses. Swatch columns
are real values parsed from the source - never hand-maintained.

Usage:  python3 gen_theme_gallery.py [path-to-opencrabs-repo]
Paste the output between the GALLERY markers in themes.md after a
theme-system change upstream.
"""
import pathlib
import re
import sys

home = pathlib.Path.home()
src = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else home / "srv/rs/opencrabs")
FIELDS = ["accent", "text_primary", "success", "ink"]

# 1. palette.rs constants -> (r, g, b)
ANSI_APPROX = {
    "White": (255, 255, 255), "Black": (0, 0, 0), "Cyan": (0, 170, 170),
    "DarkCyan": (0, 128, 128), "Yellow": (255, 255, 0), "DarkYellow": (208, 160, 0),
    "Red": (255, 0, 0), "DarkRed": (170, 0, 0), "Green": (0, 255, 0),
    "Blue": (0, 0, 255), "Magenta": (255, 0, 255),
}
palette = {}
pal_txt = (src / "src/tui/render/palette.rs").read_text()
for m in re.finditer(r"pub const (\w+): Color = Color::Rgb\((\d+),\s*(\d+),\s*(\d+)\)", pal_txt):
    palette[m.group(1)] = tuple(int(x) for x in m.groups()[1:])
for m in re.finditer(r"pub const (\w+): Color = Color::(\w+)", pal_txt):
    if m.group(1) not in palette and m.group(2) in ANSI_APPROX:
        palette[m.group(1)] = ANSI_APPROX[m.group(2)]

rows = []


def dot(rgb):
    h = "#{:02x}{:02x}{:02x}".format(*rgb)
    return (f'<span style="background:{h};width:.9em;height:.9em;'
            f'display:inline-block;border-radius:2px"></span> `{h}`')


# 2. crab-dark default: theme.rs CRAB_DARK block, fields point at palette consts
theme_txt = (src / "src/tui/render/theme.rs").read_text()
block = theme_txt.split("static CRAB_DARK: Theme", 1)[1].split("ansi:", 1)[0]
crab = {}
for m in re.finditer(r"(\w+): palette::(\w+),", block):
    if m.group(1) in FIELDS:
        crab[m.group(1)] = palette[m.group(2)]
rows.append(("crab-dark", "default", "built-in",
             [crab[f] for f in FIELDS]))

# 3. presets.rs: hand-built presets, fields carry rgb(0xRRGGBB)
pre_txt = (src / "src/tui/render/presets.rs").read_text()
for m in re.finditer(r'pub static \w+: Theme = Theme \{\s*name: "([\w-]+)",(.*?)ansi:', pre_txt, re.S):
    name, body = m.group(1), m.group(2)
    vals = {}
    for fm in re.finditer(r"(\w+): rgb\(0x([0-9A-Fa-f]{6})\)", body):
        if fm.group(1) in FIELDS:
            v = int(fm.group(2), 16)
            vals[fm.group(1)] = ((v >> 16) & 255, (v >> 8) & 255, v & 255)
    if all(f in vals for f in FIELDS):
        rows.append((name, "hand-built", "built-in", [vals[f] for f in FIELDS]))

# 4. curated pack: TOML files with a `# source:` provenance header,
# emitted in the binary's own PACK_SOURCES order (the /theme list order).
pack = src / "src/tui/theme_catalog/pack"
tp_txt = (src / "src/tui/theme_catalog/theme_pack.rs").read_text()
names = re.findall(r'\(\s*"([\w-]+)",\s*\n?\s*include_str!', tp_txt) or \
    re.findall(r'\("([\w-]+)", include_str!', tp_txt)
for name in names:
    text = (pack / f"{name}.toml").read_text()
    m = re.search(r"# source: (.+)", text)
    prov = ""
    if m:
        line = m.group(1)
        up = "opencode" if "sst/opencode" in line else "alacritty-theme" if "alacritty" in line else "?"
        date = re.search(r"fetched (\d{4}-\d{2}-\d{2})", line)
        variant = re.search(r"(dark|light) variant", line)
        prov = up + (", " + variant.group(1) if variant else "") + \
               (", " + date.group(1) if date else "")
    vals = {}
    for f in FIELDS:
        fm = re.search(rf'^{f} = "#([0-9A-Fa-f]{{6}})"', text, re.M)
        if fm:
            v = int(fm.group(1), 16)
            vals[f] = ((v >> 16) & 255, (v >> 8) & 255, v & 255)
    assert all(f in vals for f in FIELDS), name
    rows.append((name, "curated pack", prov, [vals[f] for f in FIELDS]))

print("| Theme | Type | accent | text | success | ink | Source |")
print("|---|---|---|---|---|---|---|")
for name, kind, prov, vals in rows:
    print(f"| **{name}** | {kind} | " + " | ".join(dot(v) for v in vals) + f" | {prov} |")
print(f"<!-- total: {len(rows)} -->", file=sys.stderr)
