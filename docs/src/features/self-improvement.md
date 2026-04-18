# Self-Improvement (RSI)

OpenCrabs improves itself over time through **Recursive Self-Improvement (RSI)**. The agent analyzes its own performance, identifies patterns, and autonomously updates its own brain files.

## How It Works

### 1. Feedback Collection
Every tool execution, user correction, and interaction is automatically logged to the feedback ledger. Categories include:
- `tool_success` / `tool_failure` — whether tool calls worked
- `user_correction` — when you corrected the agent's behavior
- `provider_error` — LLM stream drops, rate limits, timeouts
- `pattern_observed` — recurring behaviors the agent notices

### 2. Pattern Analysis
The agent calls `feedback_analyze` to review its performance:
- Per-tool success rates
- Recent failure patterns
- User correction frequency
- Provider reliability trends

### 3. Autonomous Improvement
When patterns are identified, the agent calls `self_improve` to:
- **read**: Load a brain file (SOUL.md, TOOLS.md, etc.) before modifying
- **apply**: Append new instructions based on observed patterns
- **update**: Surgically replace existing sections that need refinement
- **list**: Show all previously applied improvements

### 4. Change Tracking
Every improvement is logged to `~/.opencrabs/rsi/improvements.md` with:
- Timestamp
- Target file modified
- Description of the change
- Rationale (which feedback event triggered it)

Old improvements are archived to `~/.opencrabs/rsi/history/` to keep the active file lean.

## Example

```
User: "stop including testing steps in your output"
  → feedback_record(event_type="user_correction", dimension="output_hygiene")
  
Agent notices pattern of 5+ corrections on output hygiene:
  → feedback_analyze(query="failures")
  → self_improve(action="apply", target_file="SOUL.md", 
    content="Never include testing steps or verification commands in user-facing output.")
  → Logged to rsi/improvements.md
```

## Key Rules

- **No human approval needed** for self-improvements — the agent identifies patterns and applies fixes directly
- **Surgical updates only** — replaces specific sections, doesn't rewrite entire files
- **Always reads before modifying** — never blindly overwrites brain files
- **Archives old improvements** — keeps the improvement log manageable

## RSI Hardening (v0.3.10)

- **Cycle summaries no longer truncated** — full text displays in TUI instead of cutting off mid-sentence
- **Phantom detection reduced to 2-signal requirement** — needs both intent keyphrase AND zero tool calls before flagging, eliminating spurious self-heal triggers
- **Uses active provider** — respects current provider/model config instead of hardcoded Anthropic
- **Persistent session reuse** — one session per cycle, survives app restarts by persisting `last_cycle` timestamp
- **Skips unchanged feedback** — if feedback count hasn't changed, skips analysis to avoid wasted LLM calls

## v0.3.11 Additions

- **DashScope migration** — Qwen OAuth rotation replaced with simple API-key provider, deleting ~2,500 lines of complexity
- **Local model tool-call extraction** — auto-extracts tool calls from text content: bare JSON `{"tool_calls":[...]}`, Claude-style XML `<TOOLNAME><PARAM>value</PARAM></TOOLNAME>`, and Qwen-specific `<!-- tool_calls -->` markers
- **40+ TUI/self-heal fixes** — narrowed phantom gate, split thinking per iteration, anti-code-block nudge for local models, tighter phantom scope, mid-turn "Let me see:" catch, backtick code reference detection
- **Per-session provider isolation** — each session carries its own provider instance; no global swap affecting all sessions
- **Sub-agent `AwaitingInput` state** — `wait_agent` polls state and returns partial progress on timeout instead of deadlocking

## Self-Healing vs Self-Improvement

| Self-Healing | Self-Improvement |
|---|---|
| Fixes runtime errors (config corruption, DB issues) | Fixes behavioral patterns (bad habits, user corrections) |
| Automatic, no analysis needed | Requires feedback analysis first |
| Protects the system from crashing | Makes the agent better over time |
| Immediate | Accumulates across sessions |
