# WhatsApp

Connect OpenCrabs to WhatsApp via QR code pairing.

## Setup

### Configure via the Onboarding Wizard

Run `/onboard:channels` (or `/onboard` and navigate to the Channels step):

1. Use `↑`/`↓` to focus **WhatsApp**
2. Press `Space` to toggle it on
3. Press `Enter` to open the WhatsApp setup screen
4. The setup screen has two fields:
   - **Connection** — shows the QR code or connection status
   - **Phone Allowlist** — comma-separated phone numbers in E.164 format (e.g. `+15551234567`). Leave empty to accept all messages.
5. When not yet paired, a **QR code** appears in the terminal:
   - Open WhatsApp on your phone → Settings → Linked Devices → Link a Device
   - Scan the QR code
   - The status updates to "Connected" automatically
6. When already paired, press `R` on the Connection field to **reset** and re-pair
7. Press `Enter` to save

The pairing session persists across restarts — no need to re-scan.

### Manual Configuration (advanced)

```toml
# config.toml
[channels.whatsapp]
enabled = true
allowed_phones = ["+15551234567"]
```

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
