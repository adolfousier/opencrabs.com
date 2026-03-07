# Plans

Plans provide structured multi-step task execution with approval workflow.

## Creating a Plan

Ask the agent to plan a complex task:

```
"Plan the migration from PostgreSQL to SQLite"
```

The agent creates a plan with:
- Title and description
- Technical stack
- Risk assessment
- Test strategy
- Ordered tasks with dependencies and complexity ratings

## Plan Lifecycle

1. **Draft** — Agent creates the plan
2. **Pending Approval** — Plan is presented for your review
3. **Approved** — You approve, agent begins execution
4. **In Progress** — Tasks execute in dependency order
5. **Completed** — All tasks done

## Task States

Each task in a plan can be:
- `Pending` — Waiting for dependencies
- `InProgress` — Currently executing
- `Completed` — Done
- `Skipped` — Manually skipped
- `Failed` — Execution failed
- `Blocked` — Dependencies not met

## Commands

| Command | Description |
|---------|-------------|
| `/plan` | View current plan status |
| `/plan approve` | Approve pending plan |
| `/plan reject` | Reject pending plan |
