# v0.3.80 Docs Audit

**Release:** v0.3.80 — 2026-08-12 (tag `405ad70`)
**Scope:** 96 commits since v0.3.79, 191 files, +12,486 / -1,472 lines, 6,399 tests
**Docs.opencrabs.com HEAD:** `53f6355` (Community & Support section) — last synced to v0.3.79

---

## 1. v0.3.80 headline features (what shipped)

| # | Feature | Issue | Docs home |
|---|---------|-------|-----------|
| 1 | **memory_search scope** — `memory` (daily logs), `brain` (rules/policy), `all` | #1020 | `brain/memory.md` |
| 2 | **Owner-gated channel commands** — user commands + skills owner-gated on the catch-all arm | #975 | `channels/overview.md` |
| 3 | **Chunked memory retrieval** — docs chunked before embedding, later chunks searchable, multi-byte safe | #998-1002 | `brain/memory.md` |
| 4 | **Slack-native rendering** — tables + headings render in Slack's own shape | #1016 | `channels/slack.md` |
| 5 | **RSI convergence gates** — pause on convergence, back off on zero-improvement, hash stable findings | #977 | `features/self-improvement.md` |
| 6 | **Loop guards** — cross-turn + bash near-match announcement guards, generalized to all tools | #957, #961 | `features/self-healing.md` |
| 7 | **Browser inventory mode** for `browser_find` | #1022 | `features/browser.md` |
| 8 | **Proactive fallback chain setup** (one-shot suggestion) | #1008 | `brain/providers.md` |
| 9 | **Spoken stop honored** (not just the bare word) | — | `getting-started/cli-commands.md` |
| 10 | **cron_manage update action** — patch jobs in place | #966 | `features/cron-jobs.md` |
| 11 | **VBA macro extraction** + **xlsm/xlsb/ods, legacy .doc** (doc_parser) | #955 | `features/companion-tools.md` |
| 12 | **Deployment isolation rule** — preamble says where deployments go (VPS/container, never host env) | #1026 | (behavioral, preamble-level) |

**Upgrade notes (must surface):**
- Brain-file templates changed (AGENTS.md, BOOT.md, SOUL.md) — #1003, #992. Seeding never overwrites existing files; manual merge against `src/docs/reference/templates/` required.
- Qwen 3.8 Max thinking-loop tuning: `agent.thinking_loop_timeout_secs = 120`.

---

## 2. Post-release README updates (2 commits AFTER v0.3.80 tag)

These dropped into `origin/main` after the tag but before the next release:

| Commit | Change |
|--------|--------|
| `cd2dcf48` | Document **memory_search scope** (memory/brain/all) + refresh test counts |
| `58863be9` | Follow the **qmd drop** — qmd dependency removed, memory store now in-tree (SQLite FTS5 + vector); refresh counts |

**What they actually changed in README.md:**
1. `memory_search` tool description → added `scope` explanation (memory/brain/all, brain-first ranking, rule-vs-history distinction)
2. Memory architecture table → scope noted in the search row
3. New paragraph: scope matters more than query wording; daily logs bury rules; brain indexed on write (searchable immediately, not boot-time snapshot)
4. `qmd` → **"in-tree store (SQLite FTS5 + vector embeddings)"** everywhere (architecture table, memory section, memory search heading)
5. Test counts: 5,071 → 6,383 tests, 453 → 600 modules

---

## 3. Gaps found in docs.opencrabs.com (the audit)

### 🔴 STALE / BROKEN (factually wrong now)
| File | Line | Problem |
|------|------|---------|
| `development/architecture.md` | 80 | `\| qmd + llama-cpp-2 \| Memory search` — **qmd was dropped post-release**. Must read "in-tree store". |

### 🟠 MISSING — high-value gaps
| File | Gap |
|------|-----|
| `brain/memory.md` | No `scope` parameter docs. Biggest v0.3.80 memory feature. Also missing chunked retrieval (#998-1002) and "indexed on write" (no more boot-time snapshot). |
| `introduction.md` | Stops at v0.3.79. No .80 bullets. |
| `channels/slack.md` | No Slack-native table/heading rendering (#1016). |
| `channels/overview.md` | No owner-gate on channel commands/skills (#975) — security-relevant. |
| `features/self-improvement.md` | No RSI convergence gates / back-off / stable-identity hashing (#977). |
| `features/self-healing.md` | No cross-turn + generalized loop guards (#957, #961). |
| `features/browser.md` | No inventory mode for browser_find (#1022). |
| `features/cron-jobs.md` | No cron_manage update action (#966). |
| `features/companion-tools.md` | No VBA macro / legacy .doc / xlsm-xlsb-ods doc_parser additions (#955). |
| `getting-started/cli-commands.md` | No spoken-stop (channels/TUI). |
| `brain/providers.md` | No proactive fallback-chain setup suggestion (#1008). |
| `main.rs` landing | No .80 highlights on feature cards. |

### ✅ Already current
- `introduction.md` v0.3.79 bullets present.
- No stale test counts in docs (docs don't quote test numbers).

---

## 4. Proposed update order

1. **architecture.md** — kill the stale `qmd` ref (correctness fix, do first)
2. **brain/memory.md** — scope + chunked retrieval + indexed-on-write (the meat of .80)
3. **channels/slack.md** + **channels/overview.md** — Slack rendering + owner-gate
4. **self-improvement.md** + **self-healing.md** — RSI gates + loop guards
5. **browser.md** + **cron-jobs.md** + **companion-tools.md** + **cli-commands.md** + **providers.md** — remaining features
6. **introduction.md** + **main.rs** — .80 bullets + version bump
7. mdbook build + cargo check, commit per file group, push
