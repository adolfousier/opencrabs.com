# Dynamic Tools

Define custom tools at runtime without recompiling. Tools are defined in `~/.opencrabs/tools.toml` and can be created, removed, and reloaded on the fly.

## Defining Tools

Create `~/.opencrabs/tools.toml`:

```toml
[[tools]]
name = "deploy"
description = "Deploy the application to production"
executor = "shell"
command = "cd {{project_dir}} && ./deploy.sh {{environment}}"

[[tools]]
name = "check-status"
description = "Check service health"
executor = "http"
method = "GET"
url = "https://api.example.com/health"
```

## Executors

| Executor | Description |
|----------|-------------|
| `shell` | Runs a shell command |
| `http` | Makes an HTTP request |

## Template Parameters

Use `{{param}}` syntax for dynamic values. The agent fills these in when calling the tool:

```toml
[[tools]]
name = "search-logs"
description = "Search application logs for a pattern"
executor = "shell"
command = "grep -r '{{pattern}}' /var/log/myapp/ --include='*.log' -l"
```

## Runtime Management

The `tool_manage` meta-tool lets the agent manage dynamic tools during a session:

- **Create** — Add a new tool definition
- **Remove** — Delete an existing dynamic tool
- **Reload** — Re-read `tools.toml` without restarting

Dynamic tools appear alongside built-in tools in the agent's tool list. Enable or disable individual tools without restarting the process.
