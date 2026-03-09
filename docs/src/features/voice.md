# Voice (TTS & STT)

OpenCrabs supports text-to-speech and speech-to-text with three modes: **Off**, **API** (cloud), or **Local** (on-device, zero cost).

## Quick Setup

Run `/onboard:voice` in the TUI to configure everything interactively — model downloads, voice previews, and API keys are all handled by the wizard.

## Speech-to-Text (STT)

### Modes

| Mode | Engine | Cost | Latency | Setup |
|------|--------|------|---------|-------|
| **API** | Groq Whisper (`whisper-large-v3-turbo`) | Per-minute pricing | ~1s | API key in `keys.toml` |
| **Local** | whisper.cpp (on-device) | Free | ~2-5s | Auto-downloads model |

### Local STT Models

| Model | Size | Quality | Speed |
|-------|------|---------|-------|
| `local-tiny` | ~75 MB | Good for short messages | Fastest |
| `local-base` | ~142 MB | Better accuracy | Fast |
| `local-small` | ~466 MB | High accuracy | Moderate |
| `local-medium` | ~1.5 GB | Best accuracy | Slower |

Models auto-download from HuggingFace to `~/.local/share/opencrabs/models/whisper/` on first use.

### Configuration

```toml
# config.toml
[voice]
stt_enabled = true
stt_mode = "local"              # "api" or "local"
local_stt_model = "local-tiny"  # local-tiny, local-base, local-small, local-medium
```

For API mode:
```toml
# keys.toml
[providers.stt.groq]
api_key = "your-groq-key"       # From console.groq.com
```

## Text-to-Speech (TTS)

### Modes

| Mode | Engine | Cost | Voices | Setup |
|------|--------|------|--------|-------|
| **API** | OpenAI TTS (`gpt-4o-mini-tts`) | Per-character pricing | alloy, echo, fable, onyx, nova, shimmer | API key in `keys.toml` |
| **Local** | Piper (on-device) | Free | 6 voices | Auto-downloads model |

### Local TTS Voices (Piper)

| Voice | Description |
|-------|-------------|
| `ryan` | US Male (default) |
| `amy` | US Female |
| `lessac` | US Female |
| `kristin` | US Female |
| `joe` | US Male |
| `cori` | UK Female |

Models auto-download from HuggingFace to `~/.local/share/opencrabs/models/piper/`. A Python venv is created automatically for the Piper runtime.

### Configuration

```toml
# config.toml
[voice]
tts_enabled = true
tts_mode = "local"              # "api" or "local"
local_tts_voice = "ryan"        # ryan, amy, lessac, kristin, joe, cori
```

For API mode:
```toml
# config.toml
[voice]
tts_mode = "api"
tts_voice = "echo"              # OpenAI voice name
tts_model = "gpt-4o-mini-tts"   # OpenAI model
```

```toml
# keys.toml
[providers.tts.openai]
api_key = "your-openai-key"
```

## Full Configuration Reference

```toml
# config.toml
[voice]
# Speech-to-Text
stt_enabled = true
stt_mode = "local"              # "api" or "local"
local_stt_model = "local-tiny"  # local-tiny, local-base, local-small, local-medium

# Text-to-Speech
tts_enabled = true
tts_mode = "local"              # "api" or "local"
tts_voice = "echo"              # API mode: OpenAI voice
tts_model = "gpt-4o-mini-tts"   # API mode: OpenAI model
local_tts_voice = "ryan"        # Local mode: Piper voice
```

## How Voice Messages Work

When a voice message arrives on Telegram, WhatsApp, Discord, or Slack:

1. Audio is decoded (OGG/Opus or WAV)
2. Transcribed via STT (local whisper.cpp or Groq API)
3. Agent processes the text and generates a response
4. Response is converted to speech via TTS (local Piper or OpenAI API)
5. Audio is encoded as OGG/Opus and sent back as a voice message

Local mode handles everything on-device — no API calls, no cost, no data leaves your machine.

## Building Without Voice

Voice features are enabled by default. To build without them (smaller binary):

```bash
cargo build --release --no-default-features --features telegram,whatsapp,discord,slack,trello
```

Feature flags: `local-stt` (whisper.cpp), `local-tts` (Piper).
