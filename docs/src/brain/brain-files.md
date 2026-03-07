# Brain Files

Brain files define the agent's personality, knowledge, and behavior. They live at `~/.opencrabs/` and are loaded on every session start.

## Startup Read Order

1. `SOUL.md` — Personality and values
2. `USER.md` — Your profile and preferences
3. `memory/YYYY-MM-DD.md` — Today's notes
4. `MEMORY.md` — Long-term memory
5. `AGENTS.md` — Agent behavior guidelines
6. `TOOLS.md` — Tool reference and custom notes
7. `SECURITY.md` — Security policies
8. `HEARTBEAT.md` — Periodic check tasks

## File Reference

### SOUL.md
Agent personality. Core truths: strong opinions, brevity, resourcefulness, honesty. Hard rules: never delete files without approval, never send emails without request, never commit code directly.

### IDENTITY.md
Agent identity created during bootstrap: name, creature type, vibe, emoji, prohibited patterns.

### USER.md
Your profile: name, location, timezone, role, specialties, communication preferences, pet peeves.

### AGENTS.md
Comprehensive agent behavior docs: memory system, safety rules, git rules, workspace vs repository separation, cron best practices, platform formatting, heartbeat guidelines.

### TOOLS.md
Tool parameter reference, system CLI tools, provider configuration, integration details for all channels and services.

### SECURITY.md
Security policies: third-party code review, attack playbook awareness, network security, data handling, incident response.

### HEARTBEAT.md
Tasks for periodic proactive checks. Keep empty to skip heartbeat API calls. Add tasks for the agent to rotate through (email checks, calendar, weather, etc.).

### BOOT.md
Startup procedures: check git log, verify build, greet human with context awareness.

## Customization

These files are **yours**. The agent reads them but you control the content. Templates are at `src/docs/reference/templates/` in the source repo — compare your local files against templates when updating to pick up new sections without losing custom content.

> **Upgrading:** Brain files are never overwritten by `/evolve` or `/rebuild`. After updating, ask your agent to compare templates against local files and patch in new sections.
