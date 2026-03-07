# WhatsApp

Connect OpenCrabs to WhatsApp via QR code pairing.

## Setup

1. Enable in `config.toml`:

```toml
[channels.whatsapp]
enabled = true
```

2. On first run, a QR code appears in the terminal
3. Open WhatsApp on your phone → Settings → Linked Devices → Link a Device
4. Scan the QR code

The session persists across restarts — no need to re-scan.

## Features

- **Personal and group chats** — Works in DMs and group conversations
- **Image support** — Send and receive images
- **Voice messages** — STT transcription + TTS response
- **Plain text UI** — No buttons (WhatsApp limitation), uses text-based menus
- **Slash commands** — All built-in and custom commands work

## Formatting Notes

- No markdown tables — use bullet lists
- No headers — use **bold** or CAPS for emphasis
- Links render natively

## Voice Message Handling

When receiving a voice message:
1. Agent downloads and transcribes via STT
2. Sends text response first (searchable)
3. Optionally generates TTS audio response
