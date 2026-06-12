# RTK — Rust Token Killer

RTK is natively bundled into OpenCrabs as a built-in feature. It intercepts bash commands before they run, filters and compresses their output, and returns a token-optimized version to the LLM context. The result: **60-90% token savings** on common development commands, which directly translates to lower API costs and faster responses.

## What It Does

When OpenCrabs runs a bash command like `git status` or `cargo build`, RTK:

1. **Intercepts** the command before execution
2. **Runs** it through RTK's filtering engine
3. **Compresses** verbose output (removes noise, keeps signal)
4. **Returns** the optimized output to the agent

The agent sees the same information, but using a fraction of the tokens.

## Real-World Impact

From production usage (1,360 commands executed):

| Metric | Value |
|--------|-------|
| **Total Commands** | 1,360 |
| **Input Tokens** | 24.5M |
| **Output Tokens** | 3.5M |
| **Tokens Saved** | 20.9M |
| **Savings Rate** | **85.6%** |
| **Total Exec Time** | 245m 30s (avg 10.8s per command) |

### Top Commands by Savings

| Command | Count | Tokens Saved | Avg Savings |
|---------|-------|--------------|-------------|
| `rtk grep` | 192 | 17.5M | 16.3% |
| `rtk find` | 167 | 2.4M | 71.1% |
| `rtk cargo test --all-...` | 9 | 541.2K | 99.8% |
| `rtk cargo test` | 4 | 208.4K | 100.0% |
| `rtk:toml ps aux` | 3 | 121.9K | 97.9% |

## Supported Commands

RTK supports 40+ common development and sysadmin commands:

### Version Control
- `git`, `gh` (GitHub CLI), `glab` (GitLab CLI)

### Package Managers
- `npm`, `npx`, `pnpm`, `cargo`, `dotnet`

### Build & Test
- `jest`, `vitest`, `tsc`, `next`, `prisma`, `prettier`, `eslint`, `playwright`

### Cloud & Infrastructure
- `aws`, `docker`, `kubectl`, `psql`

### System Inspection (Sysadmin)
- `ps`, `top`, `lsof`, `netstat`, `ss`, `journalctl`, `dmesg`, `dig`, `nslookup`, `host`, `traceroute`

### File Operations
- `grep`, `find`, `ls`, `tree`, `diff`, `curl`, `wget`

### Blocked Commands

RTK never rewrites these (too interactive, security-sensitive, or already RTK meta-commands):

`sudo`, `ssh`, `scp`, `sftp`, `rsync`, `vim`, `vi`, `nvim`, `nano`, `emacs`, `less`, `more`, `man`, `python`, `python3`, `node`, `mysql`, `redis-cli`, `psql`

## How It Works

### Command Rewriting

When the agent runs `git status`, OpenCrabs automatically rewrites it to `rtk git status`. RTK then:

1. Executes `git status` internally
2. Parses the output structure
3. Applies command-specific filters (e.g., for `git diff`: show file names and change stats, not full diffs)
4. Returns the compressed output

### Smart Filtering

RTK uses different strategies per command:

- **Git commands**: Show file-level summaries, not full diffs
- **Cargo build/test**: Show errors and warnings, skip successful compilations
- **System commands** (`ps`, `top`): Use TOML filter templates to extract key metrics
- **File searches** (`grep`, `find`): Limit output length, show context only when relevant

### /rtk Command

In the TUI, type `/rtk` to see your token savings dashboard:

```
═══ RTK Token Savings Report ═══

Total Commands: 1360
Total Tokens Saved: 20.9M
Average Savings: 85.6%
Tracking Since: 2026-05-15 10:30:00 UTC

Savings by Command Type:
  grep: 192 cmds, 17.5M tokens saved, 16.3% avg
  find: 167 cmds, 2.4M tokens saved, 71.1% avg
  cargo: 13 cmds, 749.6K tokens saved, 99.9% avg
  ...
```

## Installation

RTK is **enabled by default** in all OpenCrabs builds. No setup required.

If you're building from source and want to disable it:

```bash
cargo build --no-default-features --features telegram,whatsapp,discord,slack,trello,local-stt,local-tts,browser
```

To verify RTK is active:

```bash
# In the TUI, type:
/rtk

# Or check if the binary is available:
which rtk
```

## Why It Matters

LLM API costs are based on token count. A typical `git diff` on a large repo can produce 50,000+ tokens of output. With RTK, that same diff might use only 5,000 tokens — a **90% reduction**.

Over a day of heavy development work:
- **Without RTK**: ~25M tokens consumed by command outputs
- **With RTK**: ~3.5M tokens consumed
- **Savings**: ~21.5M tokens per day

At typical API pricing ($3-15 per 1M input tokens), that's **$60-300+ saved per day** in token costs alone.

## Learn More

- RTK source: [github.com/rtk-ai/rtk](https://github.com/rtk-ai/rtk)
- RTK is developed by [fast-rlm](https://github.com/fast-rlm) and integrated natively into OpenCrabs
