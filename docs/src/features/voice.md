# Voice (TTS & STT)

OpenCrabs supports text-to-speech and speech-to-text for voice interactions.

## Text-to-Speech (TTS)

### Supported Providers

| Provider | Voices | Setup |
|----------|--------|-------|
| ElevenLabs | Rachel, Adam, Antoni, Bella, etc. | API key in `keys.toml` |
| OpenAI | alloy, echo, fable, onyx, nova, shimmer | API key in `keys.toml` |
| Coqui | Open-source voices | Local installation |
| GTTS | Google Translate voices | No key needed |

### Configuration

```toml
# config.toml
[voice.tts]
provider = "elevenlabs"
voice_id = "Rachel"
speed = 1.0
```

```toml
# keys.toml
[voice]
elevenlabs_api_key = "your-key"
```

## Speech-to-Text (STT)

### WhisperCrabs

OpenCrabs integrates with WhisperCrabs for floating voice-to-text input in the TUI. Type `/whisper` to launch.

**Supported backends:**
- OpenAI Whisper API
- Local whisper.cpp
- FasterWhisper

### Voice Messages

When receiving a voice message on WhatsApp or Telegram:
1. Audio is downloaded and transcribed via STT
2. Text response is sent first (keeps chat searchable)
3. TTS audio response is generated and sent

## Per-User Voice Preferences

```toml
# config.toml
[voice.preferences.username]
voice_id = "Adam"
speed = 1.2
```
