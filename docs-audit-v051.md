# Docs/Landing Audit — OpenCrabs v0.5.1 (read-only, 2026-09-13)

Source of truth: `~/srv/rs/opencrabs` @ v0.5.1 (e878e569), `gh release view v0.5.1`, live README on main.

## 1. Release facts

- **v0.5.1** published 2026-09-12T20:06:53Z. 213 commits since v0.5.0 (243 incl. merges/fixups), 6 contributors, 373 files, +27,981/−2,729.
- Character: hardening window. Per-job cron sessions, config write guard from the Config struct, skill-globs gate, plan-design write refusal, temporal grounding, Telegram edit governor, WhatsApp send-path rewrite, security batch (A2A token binding, owner-only config, OC-01..OC-08).
- Live README test count: **8,412 tests / 863 modules**. Crabsland intro already says 8,412. ✓ correct.

## 2. Crabsland state (already done by concurrent session, pushed)

origin/main = `0683c0d`, working tree clean except 3 untracked old audit files (v76-v78, v80, v81).

| Commit | What |
|---|---|
| c8cebad | intro: v0.5.1 highlights (7 bullets) + test count 8,412 |
| 9fa7130 | i18n: POT refresh + tier-1 delta translated pt-PT/es/fr/ru/id |
| 985d1a1 | book rebuild, all locales |
| 0683c0d | landing: v0.5.1 notes on tools/sessions/headless cards, release note ×6 locales |

## 3. DEPLOY GAP (headline)

| Surface | State | Action needed |
|---|---|---|
| GitHub origin/main | 0683c0d ✓ | none |
| neuraprodnew checkout | **3fb03ef (4 behind)** — live docs are v0.5.0-era | `git pull` on server |
| docs.opencrabs.com | 200, but NO "Temporal grounding" / "8,412" strings | fixed by server pull |
| live landing wasm | has v0.5.0 strings ("Structural code memory"), **zero v0.5.1 strings** | `crabslandlogin` + `crabslanddeploy` (docker rebuild) |

Live docs verified: `curl` of docs root shows v0.5.0 content; live landing `/crabsland-e0fff471e07f06a8_bg.wasm` (1.16 MB) strings: nav-toggle=1, Structural code memory=1, Per-job cron sessions=0, Temporal grounding=0.

## 4. Docs coverage vs v0.5.1 release notes

### Covered
- Intro highlights: temporal grounding (#1516), per-job cron sessions (#1511), skill-glob gating (#1515), config write guard (#1508), plan design refusal (#1507), provider budget backpressure (#1518), A2A binding + owner-only config.
- Landing cards: same set on tools/sessions/agents cards + release note ×6 locales.
- providers.md already on `zai` naming (predates rename commit 2464ed7c).

### GAPS — user-facing, not documented anywhere in docs/src

| # | Feature (commit) | Where it belongs | Severity |
|---|---|---|---|
| G1 | Theme catalog + 31-theme curated pack + converter (#1461, bd82198a) | NEW themes page — issue opencrabs.com#28 already open for exactly this | **HIGH** |
| G2 | WhatsApp outbound rate limiter: pacing, rolling daily cap, queue, owner alert (#1407, 28f7e1a3) | channels/whatsapp.md | **HIGH** |
| G3 | Security batch OC-02/04/06/08: deny-by-default allowlists Discord/Slack/WhatsApp, unified SSRF guard, execute_code blocklist, confidential-path read_file deny (06e9f647, fb19870d, b9816f6b, ab39bfd8) | channels/overview.md + a security note; behavior change users WILL hit | **HIGH** |
| G4 | Memory search: deterministic order, offset pagination, depth-2 impact chains, repo-relative paths, provenance tags (#1503/#89) | brain/memory.md | MED |
| G5 | Cron `session:` deliver_to target + `oc://` scheme + central target resolver (31b97ee4, 16aa0b9a) | features/cron-jobs.md (no "session:" anywhere) | MED |
| G6 | Headless hardening (#129 leshchenko): interactive-only tools excluded from headless roster, HEADLESS_PREAMBLE, children headless by law, session_notify/suggest_options hard ToolError | features/sessions.md / multi-agent.md (zero headless coverage; landing card only) | MED |
| G7 | Durable parked pushes: notify_queue table, boot redelivery, shared delivery policy (#111, 9a1de684) | features/sessions.md or a2a.md | MED |
| G8 | Skill-globs `globs:` frontmatter authoring guide (#1515) — intro bullet exists but features/skills.md has zero "globs" | features/skills.md | MED |
| G9 | bash tool optional `background` param (#1502, b66a6a66) | brain/tools.md | LOW |
| G10 | Sub-agent `include_brain` param + context gap disclosure (7a31d350) | features/multi-agent.md | LOW |
| G11 | Projects: identity by repo remote + session→project link on first turn (8a0538e4, 3b03ab70) | features/profiles.md or sessions.md | LOW |
| G12 | Forum-thread cron delivery opt-in + origin thread persistence (#104, d7e19ae4) — telegram.md covers forum topics for telegram_send only | features/cron-jobs.md + channels/telegram.md | LOW |
| G13 | load_brain_file slug form + seen-skills registry (#131, 7a4108dd) | brain docs / skills.md | LOW |
| G14 | zhipu→zai rename back-compat note (2464ed7c) — old `[providers.zhipu]` configs still work | brain/providers.md one-liner | LOW |
| G15 | Telegram polish: markdown-plane table merges, mid-turn notify folding, G2/G4 governor (#1523) | channels/telegram.md changelog row | LOW |
| G16 | Compaction/provider internals: fallback-walk report (#1521), stream-path compaction (#1519), RetryAttempt notices (#1520), CLI ledger cleanup (#1522), output-token reserve bound (#1518 — intro has it) | optional, development/ or none | SKIP-ish |

## 5. Open issues

### adolfousier/opencrabs.com (3 open)
| Issue | Status | Verdict |
|---|---|---|
| #26 use case: host-outage forensics | Story SHIPPED in 840f437 (user-stories.md line ~524, live on server at 3fb03ef) | **close with remarks** |
| #27 use case: family butler flight-status | Story SHIPPED in 840f437 (Azimuth story, line ~258) | **close with remarks** |
| #28 /themes page (43-key TOML spec, validation rules, presets, gallery, /theme picker; mirror opencode structure) | NOT started; no themes page in docs/src or SUMMARY.md; pairs with shipped #1461 | **build** — biggest docs item, maps to gap G1 |

### adolfousier/opencrabs (8 open — code track, not docs)
| Issue | Author | What |
|---|---|---|
| #1534 | adolfousier (today) | test(read): fixture builders do 100k unbuffered writeln syscalls, 63s between them |
| #1533 | @adi805 (today) | bug(whatsapp): LID sender identity drops tool approvals (OC-01) + splits session context — security-adjacent |
| #1529 | adolfousier | feat(whatsapp): receive newsletter posts |
| #1525 | adolfousier | feat(whatsapp): bounded opt-in history sync import |
| #1412 | adolfousier | feat(whatsapp): Telegram-parity roadmap (tracking) |
| #1411 | adolfousier | feat(whatsapp): outbound interactive buttons (3-button) |
| #1299 | adolfousier | feat: auto-compact on provider/model switch over threshold |
| #933 | adolfousier | feat(tools): /attach + /architecture file tools with safety gates |

## 6. Recommended sequence (on your go)

1. **Deploy the already-done work**: server pull (docs v0.5.1 live) + landing docker rebuild (v0.5.1 cards live). Zero new code.
2. **Close #26/#27 with remarks** (shipped in 840f437).
3. **#28 /themes page** (G1): new docs/src/themes page + SUMMARY entry — format spec, validation, 8 presets + 31 curated pack, converter/import path, /theme picker. Rebuild book via build-all.sh, POT refresh + tier-1 delta ×5 locales.
4. **G2+G3 high-severity gap fills**: whatsapp rate limiter section; security behavior-changes note (deny-by-default allowlists, SSRF guard, confidential-path deny).
5. **MED batch G4-G8** in one docs round; LOW batch G9-G15 optional same round.
6. Main-repo code issues (#1533, #1534 first — both fresh, one is a security-adjacent WhatsApp bug from an external contributor) are a separate track in ~/srv/rs/opencrabs.
7. Housekeeping: 3 stale untracked docs-audit-*.md files in crabsland root (v76-v78, v80, v81) — trash or gitignore.

## 7. Still pending from earlier rounds (unchanged)

- i18n step in drop-release skill (agreed, not wired).
- X post re multilingual docs (3 drafts, never posted).
- nginx `.bak` landmine in sites-enabled on neuraprodnew (dash.meetneura.ai.bak.* — re-breaks nginx on DNS flake).
- CH (zh-CN) + JP docs locales (round 2).
