# Cron Jobs

Schedule tasks to run on a recurring schedule. Cron jobs can run in isolated sessions or wake the main session.

## CLI Management

```bash
# Add a job
opencrabs cron add \
  --name "Morning Report" \
  --cron "0 9 * * *" \
  --tz "Europe/London" \
  --prompt "Check emails, calendar, and give me a morning briefing" \
  --deliver-to telegram:123456

# List all jobs
opencrabs cron list

# Enable/disable
opencrabs cron enable --name "Morning Report"
opencrabs cron disable --name "Morning Report"

# Remove
opencrabs cron remove --name "Morning Report"
```

## Agent Management

The agent can also manage cron jobs via the `cron_manage` tool:

```
"Create a cron job that checks my emails every morning at 9am"
```

## Options

| Flag | Description |
|------|-------------|
| `--name` | Job name (unique identifier) |
| `--cron` | Cron expression (e.g. `0 9 * * *`) |
| `--tz` | Timezone (e.g. `America/New_York`) |
| `--prompt` | The prompt to send to the agent |
| `--provider` | AI provider to use (optional) |
| `--model` | Model to use (optional) |
| `--thinking` | Thinking mode: `on`, `off`, `budget_XXk` |
| `--deliver-to` | Channel delivery: `telegram:CHAT_ID`, `discord:CHANNEL_ID`, etc. |
| `--auto-approve` | Auto-approve tool use for this job |

## Heartbeat vs Cron

**Use heartbeat (`HEARTBEAT.md`) when:**
- Checks are periodic but timing is flexible (~30 min)
- You want to reduce API calls by batching
- Tasks share the main session context

**Use cron when:**
- Exact timing matters ("9:00 AM every Monday")
- Task needs isolation from main session
- You want a different model or thinking level
- Output should deliver to a specific channel
