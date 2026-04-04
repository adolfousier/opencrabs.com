# Multi-Agent Orchestration

OpenCrabs supports spawning specialized sub-agents that run autonomously in isolated sessions. Each child agent gets its own context, tool registry, and cancel token. Introduced in **v0.2.97** with a typed agent system and team orchestration.

## Agent Types

When spawning an agent, an `agent_type` parameter selects a specialized role with a curated tool set:

| Type | Role | Tools |
|------|------|-------|
| `general` | Full-capability agent (default) | All parent tools minus recursive/dangerous |
| `explore` | Fast codebase navigation (read-only) | `read_file`, `glob`, `grep`, `ls` |
| `plan` | Architecture planning (read + analysis) | `read_file`, `glob`, `grep`, `ls`, `bash` |
| `code` | Implementation (full write access) | All parent tools minus recursive/dangerous |
| `research` | Web search + documentation lookup | `read_file`, `glob`, `grep`, `ls`, `web_search`, `http_request` |

Each type receives a role-specific system prompt that shapes its behavior. Explore agents are fast and lightweight -- they only read files. Code agents can modify anything. Research agents can search the web but not touch your filesystem.

### Safety: ALWAYS_EXCLUDED Tools

No agent type has access to these tools, preventing dangerous or recursive operations:

- `spawn_agent` -- no spawning agents from agents
- `resume_agent`, `wait_agent`, `send_input`, `close_agent` -- no managing siblings
- `rebuild` -- no building from source
- `evolve` -- no self-updating

## Five Orchestration Tools

| Tool | Description |
|------|-------------|
| `spawn_agent` | Create a typed child agent to handle a sub-task autonomously in the background |
| `wait_agent` | Wait for a spawned agent to complete and return its output (configurable timeout) |
| `send_input` | Send follow-up instructions to a running agent (multi-turn conversation) |
| `close_agent` | Terminate a running agent and clean up its resources |
| `resume_agent` | Resume a completed or failed agent with a new prompt (preserves prior context) |

### Spawn an Agent

```
spawn_agent(
  label: "refactor-auth",      # Human-readable label
  agent_type: "code",          # general | explore | plan | code | research
  prompt: "Refactor auth..."   # Task instruction
)
```

The agent runs in its own session with auto-approved tools. No blocking -- it executes in the background while the parent continues.

### Wait for Completion

```
wait_agent(
  agent_id: "abc-123",
  timeout_secs: 300            # Max wait time (default: 300s)
)
```

### Multi-Turn with send_input

After spawning, you can send additional instructions without restarting:

```
send_input(
  agent_id: "abc-123",
  text: "Also add unit tests for the new module"
)
```

The child agent processes the input on its next iteration. This enables iterative workflows -- review the agent's output, then ask it to refine or continue.

### Resume a Completed Agent

```
resume_agent(
  agent_id: "abc-123",
  prompt: "Now port the same changes to the other two files"
)
```

The agent continues in its original session, preserving all prior context. No need to re-explain the codebase.

## Team Orchestration

The `TeamManager` coordinates named groups of agents for parallel execution. Three team-specific tools:

### Create a Team

```
team_create(
  team_name: "backend-refactor",
  agents: [
    { label: "auth", agent_type: "code", prompt: "Refactor auth module" },
    { label: "tests", agent_type: "code", prompt: "Write tests for auth" },
    { label: "docs", agent_type: "general", prompt: "Update documentation" }
  ]
)
```

All agents spawn simultaneously and run in parallel. Returns the team name and all agent IDs.

### Broadcast to a Team

```
team_broadcast(
  team_name: "backend-refactor",
  message: "Use the new AuthError enum instead of plain strings"
)
```

Sends a message to all running agents in the team. Non-running agents are skipped. Useful for sharing context or direction changes.

### Delete a Team

```
team_delete(team_name: "backend-refactor")
```

Cancels all running agents and cleans up resources. Completed agents are left in the subagent manager for reference.

## Subagent Provider/Model Config

By default, spawned agents inherit the parent's provider and model. You can override this globally in `config.toml`:

```toml
[agent]
subagent_provider = "openrouter"   # Provider for child agents
subagent_model = "qwen/qwen3-235b" # Model override
```

This lets you run a powerful model (e.g. Claude Opus) for the main session while using a cheaper/faster model (e.g. Qwen) for sub-tasks. The override applies to all `spawn_agent` and `resume_agent` calls.

If `subagent_provider` or `subagent_model` is not set, the spawned agent loads from the global default provider.

## Workflow Patterns

### Parallel Research + Implementation

```
team_create("feature-research", [
  { label: "research", agent_type: "research", prompt: "Find best practices for rate limiting in Rust" },
  { label: "explore", agent_type: "explore", prompt: "Find all middleware files in the codebase" }
])
```

Wait for results, then spawn a code agent with the combined context.

### Iterative Code Review

```
# 1. Spawn a code agent
spawn_agent(label: "impl", agent_type: "code", prompt: "Implement rate limiting middleware")

# 2. Wait for completion
wait_agent(agent_id: "impl-id")

# 3. Resume with refinements
resume_agent(agent_id: "impl-id", prompt: "Add tests for the edge cases we discussed")
```

### Large-Scale Refactoring

```
team_create("refactor-team", [
  { label: "module-a", agent_type: "code", prompt: "Refactor module A to use the new trait" },
  { label: "module-b", agent_type: "code", prompt: "Refactor module B to use the new trait" },
  { label: "module-c", agent_type: "code", prompt: "Refactor module C to use the new trait" },
  { label: "tests", agent_type: "code", prompt: "Update all tests for the new trait signature" }
])
```

## Testing

84 tests cover the entire multi-agent system:
- Manager state machine (spawn, wait, close lifecycle)
- `SendInput` wiring and input loop
- `CloseAgent` cleanup
- `WaitAgent` timeout behavior
- `AgentType` tool filtering
- `TeamManager`, `TeamDelete`, `TeamBroadcast`
- Registry exclusion (ALWAYS_EXCLUDED enforcement)
