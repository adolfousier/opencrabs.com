# Docs Audit: v0.3.76 / v0.3.77 / v0.3.78

Generated: 2026-08-01. Source: gh release notes for each tag.

## Mapping: Feature → Docs Page

### features/mission-control.md
| Release | Item | Type |
|---------|------|------|
| .78 | Analytics filter + card grid (D/W/M, 3-across, model status icons) | Feature |
| .78 | Analytics panel tabs + scroll (phantom/model/D-W-M) | Feature |
| .78 | Analytics queries + report sections | Feature |
| .78 | Analytics emitters wired across agent | Feature |
| .78 | Analytics events DB layer | Feature |
| .78 | Analytics model recording fix | Fix |
| .78 | Journal blocks corruption fix | Fix |

### features/voice.md
| Release | Item | Type |
|---------|------|------|
| .78 | OpenAI TTS onboarding (voice selector + API key field) | Feature |
| .78 | Voice OGG/Opus → WAV conversion before STT | Fix |

### features/self-healing.md
| Release | Item | Type |
|---------|------|------|
| .76 | Phantom file-delivery detection | Fix |
| .76 | Phantom null-effect calls | Fix |
| .76 | Phantom command verification | Fix |
| .76 | Phantom evidence check | Fix |
| .76 | Phantom investigation claim | Fix |
| .76 | Phantom both-ends check | Fix |
| .76 | Phantom self-update claim | Fix |
| .78 | Thinking-loop timeout + phantom enforcement retry | Feature |
| .78 | Phantom shipped-and-tracked claim | Fix |
| .78 | Phantom colloquial claims | Fix |
| .78 | Phantom media-delivery claims | Fix |

### features/self-improvement.md
| Release | Item | Type |
|---------|------|------|
| .76 | RSI /evolve proposal (surface available release) | Feature |
| .78 | TOML brain verification (post-write) | Feature |
| .78 | RSI brain_verify Orient gate | Fix |
| .78 | RSI digest totals (raw vs surfaced) | Fix |
| .78 | RSI rule size cap + consolidation path | Fix |
| .78 | brain_verify contradiction scope (per-entry) | Fix |
| .78 | RSI capability-gap disposition | Fix |

### features/plans.md
| Release | Item | Type |
|---------|------|------|
| .76 | TUI finished checklist (keep on screen after complete) | Feature |
| .76 | Plan completed-card render | Fix |
| .77 | Plan-card lock deadlock | Fix |
| .77 | Plan prose duplication (card + flow block) | Fix |
| .78 | Epistemic Orient gate (gate plan start) | Feature |
| .78 | Criteria-aware verification gate (Ralph) | Feature |
| .78 | Ralph verification gate + iteration cap | Feature |
| .78 | Epistemic engine (belief tracking + confidence) | Feature |

### features/sessions.md
| Release | Item | Type |
|---------|------|------|
| .78 | session_context concurrency fix | Fix |
| .78 | session_context atomic write (trailing-char corruption) | Fix |

### features/split-panes.md (TUI)
| Release | Item | Type |
|---------|------|------|
| .76 | TUI duplicate-submit guard | Fix |
| .76 | TUI command labelling | Fix |
| .76 | TUI harness-status handling | Fix |
| .76 | TUI steps-and-tools display | Fix |
| .76 | TUI fold row count | Fix |
| .77 | TUI bg-task position (left → right on input border) | Fix |
| .78 | Deliverable report visibility (keep visible when fold) | Fix |
| .78 | Background running indicator (clear before delivering) | Fix |

### features/usage-dashboard.md
| Release | Item | Type |
|---------|------|------|
| .76 | Telegram /usage cache breakdown (per-model) | Feature |
| .76 | Usage Opus 5 pricing | Fix |
| .76 | Usage qwen3.8 pricing | Fix |
| .76 | Usage cost attribution (provider that served) | Fix |

### brain/providers.md
| Release | Item | Type |
|---------|------|------|
| .76 | Models natural naming (accept colloquial model names) | Feature |
| .76 | /execute provider routing | Feature |
| .76 | /plan provider routing | Feature |

### brain/memory.md
| Release | Item | Type |
|---------|------|------|
| .76 | Brain proactive memory (surface relevant memory unprompted) | Feature |

### brain/brain-files.md
| Release | Item | Type |
|---------|------|------|
| .76 | Brain query loading (load_brain_file returns matching sections only) | Feature |

### channels/telegram.md
| Release | Item | Type |
|---------|------|------|
| .76 | /new gating (owner-only + scoped menu) | Feature |
| .76 | /start closed-group recognition | Fix |
| .76 | Plan-card serialisation/flood/persistence | Fix |
| .76 | Follow-up tools turn (real tools turn on tap) | Fix |
| .77 | HTML conversion for all telegram_send actions | Fix |
| .77 | Flow block posting every tick | Fix |
| .77 | Echoed plan title drop | Fix |
| .77 | flush_intermediates gating | Fix |
| .78 | Rich delivery (unified markdown) | Fix |
| .78 | Follow-up attribution (name the member) | Fix |
| .78 | Callback routing (originating session) | Fix |
| .78 | Open-group registration gate | Fix |
| .78 | Background streaming (one turn per session) | Fix |
| .78 | Follow-up recording (own block) | Fix |
| .78 | Cowork members recording | Fix |

### channels/discord.md
| Release | Item | Type |
|---------|------|------|
| .76 | Sustained typing indicator | Fix |

### channels/overview.md
| Release | Item | Type |
|---------|------|------|
| .76 | Typing persistence through background tasks | Fix |
| .78 | Channel chunk boundaries (no open markup) | Fix |

### getting-started/onboarding.md
| Release | Item | Type |
|---------|------|------|
| .78 | Model filter hint (always render above list) | Fix |
| .78 | Exact model match (beats substring) | Fix |

### getting-started/configuration.md
| Release | Item | Type |
|---------|------|------|
| .78 | TOML bash blocklist (runtime-configurable safety gates) | Feature |
| .78 | Safety TOML reload on change | Fix |
| .78 | config_manager nested paths resolution | Fix |

### development/security.md
| Release | Item | Type |
|---------|------|------|
| .78 | Safety gates documentation | Feature |
| .78 | Sanitize quoted secrets (redaction) | Fix |

### features/skills.md
| Release | Item | Type |
|---------|------|------|
| .78 | slash_command resolution (skills + /onboard:<topic>) | Fix |

### SKIP (internal-only, no docs needed)
| Release | Item | Reason |
|---------|------|--------|
| .76 | RSI config-example tracking | Internal RSI |
| .76 | Brain template-sync merge | Internal RSI |
| .76 | Brain clarify-before-build | Brain prompt text |
| .76 | Memory UTC timestamps | Internal format |
| .76 | RSI finding hash | Internal RSI |
| .76 | RSI content-gated sync | Internal RSI |
| .76 | RSI capability-gap routing | Internal RSI |
| .76 | RSI dead-cycle guard | Internal RSI |
| .76 | Brain unrun-command quote | Brain prompt |
| .76 | Brain phantom-belief correction | Brain prompt |
| .76 | Brain reasoning-cannot-tools | Brain prompt |
| .76 | Brain preamble iteration fix | Brain prompt |
| .76 | Brain no-announce rule | Brain prompt |
| .76 | Repetition fenced-code exclusion | Internal streaming |
| .78 | Belief tracking wiring | Internal epistemic |
| .78 | Reasoning-block redaction | Internal delivery |
| .78 | generate_image extension | Trivial |
| .78 | clean_auto_title UTF-8 panic | Internal |
| .78 | Truncation continuation join | Internal |
| .78 | Telegram rich verdict logging | Internal logging |
| .78 | Telegram admin logging | Internal logging |
| .78 | RSI capability-gap disposition | Internal RSI |

## Landing page (src/main.rs) candidates
Top .78 highlights for feature cards:
1. Mission Control analytics (D/W/M filter, card grid, tabs)
2. Epistemic engine + Ralph verification loop
3. OpenAI TTS onboarding
4. TOML safety gates (bash blocklist)
5. Thinking-loop timeout (anti-fabrication)
