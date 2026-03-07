# Plans

Plans provide structured multi-step task execution with a live progress widget in the TUI.

## Creating a Plan

Ask the agent to plan a complex task:

```
"Plan the migration from PostgreSQL to SQLite"
```

The agent uses the `plan` tool internally to create a plan with:
- Title and description
- Technical stack
- Risk assessment
- Test strategy
- Ordered tasks with dependencies and complexity ratings

## Plan Lifecycle

1. **Draft** — Agent creates the plan and adds tasks
2. **Finalize** — Agent calls `finalize` which triggers the tool approval dialog
3. **Approved** — You approve in the tool dialog, plan status becomes `Approved`, and the agent begins executing tasks immediately
4. **In Progress** — Tasks execute in dependency order
5. **Completed** — All tasks done

In **ask mode** (default), the `finalize` step triggers the tool approval dialog — you review the full plan before execution begins. In **auto-approve mode**, finalize is auto-approved and the agent plans and executes without pausing.

## Task States

Each task in a plan can be:
- `Pending` (·) — Waiting for dependencies
- `InProgress` (▶) — Currently executing
- `Completed` (✓) — Done
- `Skipped` (✓) — Manually skipped
- `Failed` (✗) — Execution failed
- `Blocked` (·) — Dependencies not met

## TUI Plan Widget

When a plan is active, a live checklist panel appears above the input box showing:

- **Plan title** and progress counter (e.g. `3/7`)
- **Progress bar** — Visual `██████░░░░` bar with percentage
- **Task list** — Up to 6 tasks visible with status icons and task numbers
- **Overflow indicator** — `... (N more)` when tasks exceed the visible limit

The widget updates in real-time as the agent completes each task.

## Managing Plans

Plans are managed through natural language:

```
"Approve the plan"
"Reject the plan"
"What's the plan status?"
"Skip task 3"
```

The agent handles plan creation, approval, execution, and status reporting through the `plan` tool.
