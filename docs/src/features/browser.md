# Browser Automation

OpenCrabs includes native headless Chrome control via the Chrome DevTools Protocol (CDP). No Selenium, no Playwright — direct browser control built into the binary.

## Requirements

- A Chromium-based browser installed (Chrome, Brave, Edge, or Chromium)
- Feature flag: `browser` (enabled by default)

OpenCrabs auto-detects your default Chromium browser — no manual path configuration needed.

## Browser Tools

| Tool | Description |
|------|-------------|
| `navigate` | Open a URL in the browser |
| `click` | Click an element by CSS selector |
| `type` | Type text into an input field |
| `screenshot` | Capture a screenshot of the page |
| `eval_js` | Execute JavaScript in the page context |
| `extract_content` | Extract text content from elements |
| `wait_for_element` | Wait for an element to appear |

## How It Works

The browser is lazy-initialized as a singleton — it only launches when the agent first needs it. It runs in stealth mode with a persistent profile directory, so cookies and sessions survive across tool calls.

On macOS, display auto-detection enables headed mode when a display is available, falling back to headless in CI or daemon environments.

## Example

Ask the agent:

> "Go to our staging site, log in with the test account, navigate to the dashboard, and take a screenshot"

The agent will chain `navigate` → `type` (username) → `type` (password) → `click` (login button) → `navigate` (dashboard) → `screenshot` — all autonomously.

## Configuration

No configuration needed. The browser feature is enabled by default. To disable it at build time:

```bash
cargo build --release --no-default-features --features "telegram,discord,slack"
```
