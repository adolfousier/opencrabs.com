# Agent-to-Agent (A2A) Protocol

OpenCrabs includes a built-in A2A gateway implementing the [A2A Protocol RC v1.0](https://google.github.io/A2A/) for peer-to-peer agent communication.

## Enabling

```toml
# config.toml
[a2a]
enabled = true
bind = "127.0.0.1"   # Loopback only (default)
port = 18790
# allowed_origins = ["http://localhost:3000"]  # CORS
```

No API keys required — A2A is config-only.

## Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/.well-known/agent.json` | GET | Agent Card — discover capabilities |
| `/a2a/v1` | POST | JSON-RPC 2.0 — `message/send`, `tasks/get`, `tasks/cancel` |
| `/a2a/health` | GET | Health check |

## Examples

```bash
# Discover the agent
curl http://127.0.0.1:18790/.well-known/agent.json | jq .

# Send a message (creates a task)
curl -X POST http://127.0.0.1:18790/a2a/v1 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "message/send",
    "params": {
      "message": {
        "role": "user",
        "parts": [{"kind": "text", "text": "What tools do you have?"}]
      }
    }
  }'

# Poll a task
curl -X POST http://127.0.0.1:18790/a2a/v1 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":2,"method":"tasks/get","params":{"id":"TASK_ID"}}'
```

## Bee Colony Debate

Multi-agent structured debate via confidence-weighted voting (based on [ReConcile, ACL 2024](https://arxiv.org/abs/2309.13007)). Multiple "bee" agents argue across configurable rounds, enriched with knowledge context, then converge on a consensus answer.

## Security

- **Loopback only** by default — binds to `127.0.0.1`
- **CORS locked down** — no cross-origin requests unless `allowed_origins` is set
- **No authentication** — do not expose to public internet without a reverse proxy + auth layer
