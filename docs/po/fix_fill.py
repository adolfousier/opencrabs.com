import re
# 1. fix po_fill.py: lambda replacements so backslashes survive re.sub
s = open('po_fill.py').read()
s = s.replace("return re.sub(r'^msgstr \"\"\\n?', f'msgstr \"\"\\n{body}\\n', block, count=1, flags=re.M), True",
              "return re.sub(r'^msgstr \"\"\\n?', (lambda _m: 'msgstr \"\"\\n' + body + '\\n'), block, count=1, flags=re.M), True")
s = s.replace("return re.sub(r'^msgstr \"\"', f'msgstr \"{val}\"', block, count=1, flags=re.M), True",
              "return re.sub(r'^msgstr \"\"', (lambda _m: 'msgstr \"' + val + '\"'), block, count=1, flags=re.M), True")
open('po_fill.py','w').write(s)
print("po_fill patched:", "lambda" in s)
# 2. repair the two broken msgstr blocks in pt-PT.po
p = open('pt-PT.po').read()
broken1 = 'msgstr ""\n"# Instalar (Linux/macOS)\n"\n""'
broken2 = 'msgstr ""\n"# Ou via Cargo (requer Rust 1.94+)\n"\n""'
p = p.replace(broken1, 'msgstr ""\n"# Instalar (Linux/macOS)\\n"')
p = p.replace(broken2, 'msgstr ""\n"# Ou via Cargo (requer Rust 1.94+)\\n"')
open('pt-PT.po','w').write(p)
print("repaired:", broken1 not in p and broken2 not in p)
