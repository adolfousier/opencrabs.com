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

## Self-Healing vs Self-Improvement

| Self-Healing | Self-Improvement |
|---|---|
| Fixes runtime errors (config corruption, DB issues) | Fixes behavioral patterns (bad habits, user corrections) |
| Automatic, no analysis needed | Requires feedback analysis first |
| Protects the system from crashing | Makes the agent better over time |
| Immediate | Accumulates across sessions |
