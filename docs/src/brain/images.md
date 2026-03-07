# Image Generation & Vision

OpenCrabs supports image generation and vision analysis via Google Gemini. These features are independent of the main chat provider — use Claude for chat and Gemini for images.

## Setup

1. Get a free API key from [aistudio.google.com](https://aistudio.google.com)
2. Run `/onboard:image` in chat to configure interactively
3. Or add manually:

```toml
# keys.toml
[image]
api_key = "AIza..."
```

```toml
# config.toml
[image.generation]
enabled = true
model = "gemini-3.1-flash-image-preview"

[image.vision]
enabled = true
model = "gemini-3.1-flash-image-preview"
```

## Agent Tools

When enabled, two tools become available automatically:

| Tool | Description |
|------|-------------|
| `generate_image` | Generate an image from a text prompt — saves to `~/.opencrabs/images/` |
| `analyze_image` | Analyze an image file or URL via Gemini vision |

**Example prompts:**
- *"Generate a pixel art crab logo"* — agent calls `generate_image`, returns file path
- *"What's in this image: /tmp/screenshot.png"* — agent calls `analyze_image` via Gemini

## Incoming Images

When a user sends an image from any channel, it arrives as `<<IMG:/tmp/path>>` in the message. The file is already downloaded — the agent can:

- See it directly (if the model supports vision)
- Pass the path to `analyze_image` for Gemini analysis
- Use the path in `bash` commands or any tool that accepts file paths
- Reference it in replies with `<<IMG:path>>` to forward to channels

## Model

Both tools use `gemini-3.1-flash-image-preview` — Gemini's dedicated image model that supports both vision input and image output in a single request.

## Per-Provider Vision Model

Separately from the Gemini `analyze_image` tool, any provider can have its own vision tool via `vision_model`. When the user sends an image and the chat model can't handle it natively, the agent calls the provider's vision tool — which sends the image to the `vision_model` on the same provider, gets a text description back, and uses that context to reply.

Example: User sends an image to MiniMax M2.5 (no native vision). The agent calls the vision tool, which sends the image to MiniMax-Text-01, gets the description, and M2.5 replies using that context.

See [Provider Setup — Vision Model](./providers.md#vision-model).
