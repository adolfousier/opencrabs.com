# Usage Dashboard

The Usage Dashboard shows your token usage, costs, models, tools, and project breakdown. Open it with `/usage`.

## Overview

The header shows your totals:

| Metric | Description |
|--------|-------------|
| **Tokens** | Total tokens consumed (in millions) |
| **Cost** | Total spend in USD |
| **Sessions** | Number of sessions |
| **Calls** | Total API calls made |

## The Four Panels

The dashboard is a 2x2 grid of panels:

### Daily Activity (top-left)

A horizontal bar chart showing token usage per day. Peak days stand out clearly. Useful for spotting burst activity or debugging unexpected spikes.

### By Project (top-right)

A ranked table of projects by cost:

| Column | Description |
|--------|-------------|
| Project | Working directory name |
| `$` | Total cost |
| `M` | Tokens in millions |
| `s` | Total session time |

### By Model (bottom-left)

A ranked table of every model used:

| Column | Description |
|--------|-------------|
| Model | Provider + model name |
| `$` | Total cost |
| `M` | Tokens in millions |
| `C` | Number of API calls |

The selected row is highlighted in orange. Use this to spot expensive models or optimize your provider mix.

### By-Model Quantization Tree View (v0.3.20)

Model variants grouped under parent rows with tree connectors:

| Column | Description |
|--------|-------------|
| Model | Provider + model name (parent row, bold) |
| `├─` / `└─` | Variant rows (e.g. qwen3.6-35b-a3b-gguf-oq2, qwen3.6-35b-a3b-gguf-oq4) |

Parent rows show **aggregated stats** (total tokens, cost, calls) across all quant variants. This eliminates the noisy duplication where `qwen3.6-35b-a3b-gguf`, `-oq2`, `-oq4`, `-iq4_xs` each appeared as separate rows.

**Before:** 6 separate rows for one model family
**After:** 1 parent row + 3 variant rows with aggregated parent stats

### Core Tools (bottom-right)

A horizontal bar chart ranking your most-used tools. `bash` and `read_file` typically dominate. Useful for understanding your agent's workflow patterns.

### By Activity (footer)

A summary table showing cost and turns by activity category (Development, CI/Deploy, Features), plus the 1-shot success rate for each.

## Cache Efficiency Card (v0.3.36)

A new card on the dashboard shows your **cache hit rate** as a percentage. Providers that support prompt caching (like Anthropic and Z.AI) return `cache_creation_input_tokens` and `cache_read_input_tokens` in their usage data. These are now persisted to the messages table (DB migration #25), and the dashboard aggregates them into a hit-rate percentage.

When cache data is unavailable (provider doesn't report it, or no cached tokens yet), the card degrades gracefully with a dash instead of showing `0%`.

## Time Filters

| Key | Filter |
|-----|--------|
| `T` | Today |
| `W` | This week |
| `M` | This month |
| `A` | All time |
| `Esc` | Close dashboard |

## Navigation

| Key | Action |
|-----|--------|
| `Tab` | Cycle focus between panels |
| `Enter` | Open details for selected item |
| `Esc` | Close dashboard |
