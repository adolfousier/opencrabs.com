use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use serde::Deserialize;
use web_sys::{HtmlElement, window};

fn copy_terminal_code() {
    let active_tab = document()
        .query_selector(".terminal-tabs button.active")
        .ok()
        .flatten()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok());
    let tab_idx = match active_tab {
        Some(el) => {
            let text = el.inner_text();
            if text == "Binary" {
                0
            } else if text == "Cargo" {
                1
            } else {
                2
            }
        }
        None => 0,
    };

    let os = detect_os();
    let _tag = document()
        .query_selector(".hero-badge")
        .ok()
        .flatten()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
        .map(|el| el.inner_text())
        .unwrap_or_default();

    let commands: Vec<String> = match (tab_idx, os) {
        (0, "windows") => vec![
            "Invoke-WebRequest -Uri https://github.com/adolfousier/opencrabs/releases/latest/download/opencrabs-windows-amd64.zip -OutFile opencrabs.zip".into(),
            "Expand-Archive opencrabs.zip -DestinationPath . && .\\opencrabs.exe".into(),
        ],
        (0, "macos") => vec![
            format!("TAG=$(curl -s https://api.github.com/repos/adolfousier/opencrabs/releases/latest | jq -r .tag_name)"),
            format!("curl -fsSL https://github.com/adolfousier/opencrabs/releases/download/$TAG/opencrabs-$TAG-macos-arm64.tar.gz | tar xz"),
            "./opencrabs".into(),
        ],
        (0, _) => vec![
            "sudo apt install libgomp1".into(),
            format!("TAG=$(curl -s https://api.github.com/repos/adolfousier/opencrabs/releases/latest | jq -r .tag_name)"),
            "curl -fsSL https://github.com/adolfousier/opencrabs/releases/download/$TAG/opencrabs-$TAG-linux-amd64.tar.gz | tar xz".into(),
            "./opencrabs".into(),
        ],
        (1, _) => vec![
            "cargo install opencrabs".into(),
            "opencrabs".into(),
        ],
        (2, _) => vec![
            "curl -fsSL https://raw.githubusercontent.com/adolfousier/opencrabs/main/src/scripts/setup.sh | bash".into(),
            "git clone https://github.com/adolfousier/opencrabs.git && cd opencrabs".into(),
            "cargo build --release && ./target/release/opencrabs".into(),
        ],
        _ => vec![],
    };

    let text = commands.join("\n");
    if let Some(w) = window() {
        let _ = w.navigator().clipboard().write_text(&text);
    }
}

fn detect_os() -> &'static str {
    let window = leptos::prelude::window();
    let nav: web_sys::Navigator = window.navigator();
    let ua = nav.user_agent().unwrap_or_default().to_lowercase();
    let platform = nav.platform().unwrap_or_default().to_lowercase();

    if platform.contains("win") || ua.contains("windows") {
        "windows"
    } else if platform.contains("mac") || ua.contains("macintosh") {
        "macos"
    } else {
        "linux"
    }
}

fn download_asset(os: &str) -> &'static str {
    match os {
        "macos" => "opencrabs-macos-arm64.tar.gz",
        "windows" => "opencrabs-windows-amd64.zip",
        _ => "opencrabs-linux-amd64.tar.gz",
    }
}

fn build_download_url_full(tag: &str, os: &str) -> String {
    let asset = download_asset(os);
    let name = asset.replace("opencrabs-", &format!("opencrabs-{}-", tag));
    format!(
        "https://github.com/adolfousier/opencrabs/releases/download/{}/{}",
        tag, name
    )
}

fn download_label(os: &str) -> &'static str {
    match os {
        "macos" => "Download for macOS",
        "windows" => "Download for Windows",
        _ => "Download for Linux",
    }
}

#[derive(Deserialize, Clone, Debug)]
struct GitHubRepo {
    stargazers_count: u32,
}

async fn fetch_star_count() -> Option<u32> {
    Request::get("https://api.github.com/repos/adolfousier/opencrabs")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .ok()?
        .json::<GitHubRepo>()
        .await
        .ok()
        .map(|r| r.stargazers_count)
}

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let stars = LocalResource::new(fetch_star_count);
    let stars_signal = Signal::derive(move || stars.get().flatten().unwrap_or(0));

    let release = LocalResource::new(fetch_latest_release);
    let tag_signal = Signal::derive(move || {
        release
            .get()
            .flatten()
            .map(|r| r.tag_name.clone())
            .unwrap_or_default()
    });

    view! {
        <Nav stars=stars_signal tag=tag_signal />
        <Hero />
        <Testimonials />
        <QuickStart tag=tag_signal />
        <Features />
        <Integrations />
        <Community />
        // <Newsletter /> — coming soon
        <Footer stars=stars_signal />
    }
}

// ── Navigation ──────────────────────────────────────────────────────────────

#[component]
fn Nav(stars: Signal<u32>, tag: Signal<String>) -> impl IntoView {
    let star_label = move || {
        let count = stars.get();
        if count > 0 {
            format!(" ★ {}", count)
        } else {
            String::new()
        }
    };

    view! {
        <nav>
            <div class="container">
                <a href="/" class="nav-logo">
                    <img class="crab-icon" src="public/opencrabs-logo.png" alt="" />
                    "OpenCrabs"
                </a>
                <ul class="nav-links">
                    <li><a href="https://docs.opencrabs.com" target="_blank">"Docs"</a></li>
                    <li><a href="#features">"Features"</a></li>
                    <li><a href="#integrations">"Integrations"</a></li>
                    <li>
                        {move || {
                            let t = tag.get();
                            let os = detect_os();
                            let url = if t.is_empty() {
                                "https://github.com/adolfousier/opencrabs/releases/latest".to_string()
                            } else {
                                build_download_url_full(&t, os)
                            };
                            view! { <a href=url class="nav-download">"Download"</a> }
                        }}
                    </li>
                    <li>
                        <a href="https://github.com/adolfousier/opencrabs" class="btn-github" target="_blank">
                            "GitHub"
                            <span class="github-stars">{star_label}</span>
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

// ── Hero ────────────────────────────────────────────────────────────────────

#[derive(Deserialize, Clone, Debug)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    published_at: Option<String>,
}

async fn fetch_latest_release() -> Option<GitHubRelease> {
    Request::get("https://api.github.com/repos/adolfousier/opencrabs/releases/latest")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .ok()?
        .json::<GitHubRelease>()
        .await
        .ok()
}

fn format_release_date(published_at: &str) -> String {
    // "2026-03-07T01:44:48Z" → "Mar 7, 2026"
    let parts: Vec<&str> = published_at
        .split('T')
        .next()
        .unwrap_or("")
        .split('-')
        .collect();
    if parts.len() == 3 {
        let month = match parts[1] {
            "01" => "Jan",
            "02" => "Feb",
            "03" => "Mar",
            "04" => "Apr",
            "05" => "May",
            "06" => "Jun",
            "07" => "Jul",
            "08" => "Aug",
            "09" => "Sep",
            "10" => "Oct",
            "11" => "Nov",
            "12" => "Dec",
            _ => parts[1],
        };
        let day = parts[2].trim_start_matches('0');
        format!("{} {}, {}", month, day, parts[0])
    } else {
        published_at.to_string()
    }
}

#[component]
fn Hero() -> impl IntoView {
    let release = LocalResource::new(fetch_latest_release);

    let badge_text = move || {
        match release.get() {
            Some(Some(r)) => {
                let version = r.tag_name.clone();
                let name = r.name.as_deref().unwrap_or("").to_string();
                let date = r
                    .published_at
                    .as_deref()
                    .map(format_release_date)
                    .unwrap_or_default();

                // Use release name if available, otherwise just version + date
                if name.is_empty() || name == version {
                    format!("{} — {}", version, date)
                } else {
                    format!("{} — {} — {}", version, date, name)
                }
            }
            _ => "Loading latest release...".to_string(),
        }
    };

    view! {
        <section class="hero">
            <div class="container">
                <img class="hero-crab" src="public/opencrabs-logo.png" alt="OpenCrabs" />
                <h1>"OpenCrabs"</h1>
                <p class="hero-tagline">"THE ALL-IN-ONE AI AGENT LIVING IN YOUR TERMINAL."</p>
                <p class="hero-description">
                    "Build apps, backends, landing pages. Manages files, searches the web, runs deep research, schedules tasks and events. Set a goal with /goal and watch it loop autonomously until done. Self-improving, self-healing, fully autonomous. Connects from Terminal UI, CLI or your favorite channels."
                </p>
                <a href="https://github.com/adolfousier/opencrabs/releases/latest" class="hero-badge">
                    <span class="badge-new">"LATEST"</span>
                    {badge_text}
                    <span class="arrow">" →"</span>
                </a>
                // Xiaomi collab ended — MiMo is now a normal keyed provider
            </div>
        </section>
    }
}

// ── Quick Start ─────────────────────────────────────────────────────────────

#[component]
fn QuickStart(tag: Signal<String>) -> impl IntoView {
    let (active_tab, set_active_tab) = signal(0u8);

    view! {
        <section id="quickstart">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Quick Start"
                </h2>
                <div class="terminal">
                    <div class="terminal-header">
                        <div class="terminal-dots">
                            <span class="red"></span>
                            <span class="yellow"></span>
                            <span class="green"></span>
                        </div>
                        <div class="terminal-tabs">
                            <button
                                class:active=move || active_tab.get() == 0
                                on:click=move |_| set_active_tab.set(0)
                            >"Binary"</button>
                            <button
                                class:active=move || active_tab.get() == 1
                                on:click=move |_| set_active_tab.set(1)
                            >"Cargo"</button>
                            <button
                                class:active=move || active_tab.get() == 2
                                on:click=move |_| set_active_tab.set(2)
                            >"Source"</button>
                        </div>
                        <span class="terminal-platform">"macOS / Linux / Windows"</span>
                        <button class="terminal-copy-btn" on:click=move |_| copy_terminal_code()>
                            "📋"
                        </button>
                    </div>
                    <div class="terminal-body" style:display=move || if active_tab.get() == 0 { "block" } else { "none" }>
                        {move || {
                            let os = detect_os();
                            let t = tag.get();
                            let label = download_label(os);
                            let url = if t.is_empty() {
                                "https://github.com/adolfousier/opencrabs/releases/latest".to_string()
                            } else {
                                build_download_url_full(&t, os)
                            };
                            view! {
                                <div class="download-row">
                                    <a href=url class="download-btn">{label}</a>
                                    <a href="https://github.com/adolfousier/opencrabs/releases/latest" class="download-all">"All platforms →"</a>
                                </div>
                                <div class="download-divider">"or via terminal"</div>
                                {if os == "windows" {
                                    view! {
                                        <div>
                                            <span class="terminal-comment">"# PowerShell"</span>
                                        </div>
                                        <div>
                                            <span class="terminal-prompt">"PS> "</span>
                                            <span class="terminal-cmd">"Invoke-WebRequest -Uri https://github.com/adolfousier/opencrabs/releases/latest/download/opencrabs-windows-amd64.zip -OutFile opencrabs.zip"</span>
                                        </div>
                                        <div>
                                            <span class="terminal-prompt">"PS> "</span>
                                            <span class="terminal-cmd">"Expand-Archive opencrabs.zip -DestinationPath . && .\\opencrabs.exe"</span>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div>
                                            <span class="terminal-prompt">"$ "</span>
                                            <span class="terminal-cmd">"sudo apt install libgomp1"</span>
                                        </div>
                                        <div>
                                            <span class="terminal-prompt">"$ "</span>
                                            <span class="terminal-cmd">"TAG=$(curl -s https://api.github.com/repos/adolfousier/opencrabs/releases/latest | jq -r .tag_name)"</span>
                                        </div>
                                        <div>
                                            <span class="terminal-prompt">"$ "</span>
                                            <span class="terminal-cmd">"curl -fsSL https://github.com/adolfousier/opencrabs/releases/download/$TAG/opencrabs-$TAG-linux-amd64.tar.gz | tar xz"</span>
                                        </div>
                                        <div>
                                            <span class="terminal-prompt">"$ "</span>
                                            <span class="terminal-cmd">"./opencrabs"</span>
                                        </div>
                                    }.into_any()
                                }}
                            }
                        }}
                    </div>
                    <div class="terminal-body" style:display=move || if active_tab.get() == 1 { "block" } else { "none" }>
                        <div>
                            <span class="terminal-comment">"# Install via cargo (stable Rust)"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"cargo install opencrabs"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"opencrabs"</span>
                        </div>
                    </div>
                    <div class="terminal-body" style:display=move || if active_tab.get() == 2 { "block" } else { "none" }>
                        <div>
                            <span class="terminal-comment">"# Install all deps + Rust stable (macOS, Debian, Fedora, Arch)"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"curl -fsSL https://raw.githubusercontent.com/adolfousier/opencrabs/main/src/scripts/setup.sh | bash"</span>
                        </div>
                        <div>
                            <span class="terminal-comment">"# Then clone and build"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"git clone https://github.com/adolfousier/opencrabs.git && cd opencrabs"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"cargo build --release && ./target/release/opencrabs"</span>
                        </div>
                    </div>
                </div>
                <p class="terminal-note">
                    "Works on macOS, Linux & Windows. After install, type "
                    <strong>"/evolve"</strong>
                    " to auto-update."
                </p>
            </div>
        </section>
    }
}

// ── Features ────────────────────────────────────────────────────────────────

#[component]
fn Features() -> impl IntoView {
    let features = vec![
        (
            "🖥️",
            "~34-36 MB Single Binary",
            "Mac, Windows, or Linux. Anthropic, OpenAI, Gemini, GitHub Copilot, or any local model. Zero telemetry, not even opt-in: no analytics, no tracking, no phone-home. The only outbound traffic is what you explicitly initiate. Your data stays yours.",
        ),
        (
            "💬",
            "Any Chat App",
            "Talk to it on Telegram, Discord, Slack, WhatsApp, or Trello. Native rich message rendering with tables, lists, code blocks, math. Draft message streaming shows live \"typing...\" updates as tokens generate. Collapsible <details>/<summary> blocks for long outputs. Forum topic session isolation gives each topic its own context. Telegram reactions: reads inbound emoji reactions and replies with just a reaction when that fits. /cowork workspaces, /rename sessions, instant fast-cancel on /stop. /cd directory browser with auto project assignment. /new inherits the working directory from your most recent session. /profiles command for managing AI profiles. Owner impersonation detection in group chats. Session search across all channels. Works in DMs and group chats with persistent sessions. Or just use the TUI.",
        ),
        (
            "🧠",
            "Persistent Memory",
            "Remembers you across sessions. Your preferences, your context, your AI. Semantic search over everything.",
        ),
        (
            "⚡",
            "50+ Built-in Tools",
            "File ops, bash, web search, code execution, image gen, browser automation, local voice STT & TTS, PDF rendering, sub-agent orchestration. Proactive tool discovery — agent searches for tools before claiming inability. JIT activation for extended tools on-demand. Auto-download RTK for 10x token savings on 100+ commands. Define custom tools at runtime. Race-free multi-image pickup from any channel. Tool registry shared across all entry points so startup tools are available everywhere. Malformed tools.toml surfaces parse errors instead of silently dropping every tool. last_good .bak snapshot auto-creates on every write with fallback recovery.",
        ),
        (
            "🪟",
            "Split Panes",
            "Tmux-style horizontal and vertical pane splitting. Run 10 sessions side by side, each with its own provider and context. All processing in parallel. Native markdown rendering: emphasis, lists, links, and task items render directly in the terminal.",
        ),
        (
            "🔄",
            "/goal — Autonomous Goal Loop",
            "Set a goal with /goal <text> and the agent loops autonomously: executing, self-evaluating with an LLM judge, and continuing with a correction prompt until the goal is satisfied or the turn budget runs out. Supports /goal pause, /goal resume, /goal status, and /goal clear. Works across TUI, Telegram, Discord, Slack, and WhatsApp. Fully hands-off task execution.",
        ),
        (
            "🎯",
            "Mission Control",
            "Full-screen TUI dashboard with four panels: Analytics showing brain file sizes, tool usage bars, and failure rates, Inbox for reviewing RSI-proposed tools, commands, and skills (approve/reject inline with a/r keys), Activity log showing recent self-improvements, and Schedule queue for cron jobs. Rich schedule detail popup shows prompt, delivery target, run history with cost and duration. Cron delivery to Discord and Slack. Two-step delete safeguard with auto-backup. Race-free cron startup (no double-fire on first tick). Shared session per cron job with compaction isolation. Keyboard-driven: Tab/Shift-Tab cycle panels, j/k navigate, Enter for details, Esc to close.",
        ),
        (
            "🤖",
            "Multi-Agent & Teams",
            "Typed sub-agents (General, Explore, Plan, Code, Research) with filtered tool registries. Team orchestration spawns N agents in parallel and broadcasts to all. 20+ CLI subcommands including /mission-control dashboard, /skills picker, /btw parallel agent, /security-audit, /cost-estimate, and /repo-audit. Cross-harness skill system with auto-registered slash commands. Promoted most-used commands to top of /help. All slash commands render through rich AST pipeline with table formatting. Standardized bot menu on underscore form. Daemon mode with health endpoints.",
        ),
        (
            "🌐",
            "Browser Automation",
            "Native headless Chrome control via CDP. Navigate, click, type, screenshot, find elements by CSS/XPath/text/aria, extract content, run JS. System prompt rules enforce best practices. Per-session tab isolation. Smart detection of your default Chromium browser.",
        ),
        (
            "📐",
            "CODE.md — Built-in Coding Standards",
            "Ships with a brain template that enforces modular files, strict size limits, test coverage, and security-first patterns. Your agent writes production code, not prototypes.",
        ),
        (
            "🔒",
            "Security & Trust",
            "Confidential file protection: SSH keys, .env, credentials, and private configs are sacred. Never shared without explicit owner verification. Owner impersonation detection catches non-owners trying to act as the owner in Telegram group chats. bot_owner config field with is_owner() helper for identity checks. Per-chat ACL: each Telegram group gets its own allow list, closing the 'DM the bot privately to escape group oversight' bypass. WhatsApp response_policy (auto/owner_only/allowlist/open) controls who can interact. /cowork checks bot admin status before creating invite links. rm-blocklist prevents destructive commands with reversed-flag, quoted-path, and chained-flag defenses.",
        ),
        (
            "🛡️",
            "Self-Healing",
            "Auto-recovers corrupted config with auto-repair that never poisons last-good. Tracks per-provider health with auto-failover, 65% context budget management with async LLM compaction. Vision fallback chain: scans all enabled providers (Google, OpenRouter, OpenAI-compatible, Anthropic) before returning an error. Cross-channel crash recovery. Stuck stream detection, reasoning repetition loop detection, thinking/reasoning tag stripping from output so reasoning models never leak internal thoughts. 10-min CLI idle timeout, DB integrity checks. Append-only brain files with upstream template sync. System brain auto-rebuilds when brain files change. Expanded phantom detection, RSI escalation for repeat violations, partial JSON repair, TCP keepalive on all HTTP clients. Browser resilience: network idle wait, CDP health checks, lock release before await.",
        ),
        (
            "🔧",
            "Self-Improving",
            "Recursive Self-Improvement (RSI) — the agent analyzes its own performance via a persistent feedback ledger, identifies failure patterns, and autonomously updates its own brain files. Proposes new tools, commands, and skills via Mission Control inbox for human review. RSI efficiency gate requires proposals to state TOKEN SAVINGS, ERROR REDUCTION, or CAPABILITY ADDITION. Per-project brain overlay layers project-specific files on top of profile brain. AGENTS.md is always-loaded so hard rules are enforced every turn. Runtime commands and skills index injected so the agent sees live tools. Migrate from OpenClaw, Hermes, or any other tool with built-in CLI migration. Append-only protection, upstream template sync.",
        ),
        (
            "👤",
            "Multi-Profile",
            "Run multiple isolated instances from one binary. Each profile gets its own config, brain files, sessions, and daemon service. Token-lock isolation prevents two profiles from fighting over the same bot. All paths are profile-aware: RSI state, tmp purge, config edits all route through the active profile home. /profiles command to manage profiles from any channel. Native TUI profiles dialog with browse, create, delete, and migrate flows. Rich profile rendering on Discord, WhatsApp, and Slack. Export/import profiles as portable archives.",
        ),
        (
            "📁",
            "Projects",
            "Organize work into projects with dedicated sessions. Assign sessions with a single keypress. Per-project colours and badges for visual organization. Per-project brain overlay layers project-specific files on top of profile brain. Project file shares: symlink local copies, copy ephemeral ones into project files. Project-scoped file artifacts archived under projects/<name>/files/. Shared images from Telegram archived automatically. Full CRUD UI with SQLite-backed persistence.",
        ),
    ];

    view! {
        <section id="features">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "What It Does"
                </h2>
                <div class="features-grid">
                    {features.into_iter().map(|(icon, title, desc)| view! {
                        <div class="feature-card">
                            <span class="feature-icon">{icon}</span>
                            <h3>{title}</h3>
                            <p>{desc}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

// ── Integrations ────────────────────────────────────────────────────────────

#[component]
fn Integrations() -> impl IntoView {
    let channels = vec![
        "📱 Telegram",
        "💬 Discord",
        "🔔 Slack",
        "📞 WhatsApp",
        "📋 Trello",
    ];
    let providers = vec![
        "🤖 Anthropic",
        "🧠 OpenAI",
        "💎 Gemini",
        "🌐 OpenRouter",
        "🐙 GitHub Copilot",
        "🇨🇳 Xiaomi MiMo",
        "🇨🇳 z.ai GLM",
        "🏠 Ollama",
        "📡 LM Studio",
        "🔮 MiniMax",
        "💻 Claude CLI",
        "⌨️ OpenCode CLI",
        "🤖 Codex CLI",
    ];
    let tools = vec![
        "🐙 GitHub CLI",
        "📧 Gmail",
        "📅 Calendar",
        "🐦 Twitter/X",
        "📸 Instagram",
        "💼 LinkedIn",
        "🔍 Brave Search",
        "🎙️ Local STT/TTS",
        "🐳 Docker",
        "🌐 Browser (CDP)",
    ];

    view! {
        <section id="integrations">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Works With Everything"
                </h2>
                <div class="integrations-row">
                    {channels.into_iter().map(|name| view! {
                        <span class="integration-badge">{name}</span>
                    }).collect::<Vec<_>>()}
                </div>
                <div class="integrations-row">
                    {providers.into_iter().map(|name| view! {
                        <span class="integration-badge">{name}</span>
                    }).collect::<Vec<_>>()}
                </div>
                <div class="integrations-row">
                    {tools.into_iter().map(|name| view! {
                        <span class="integration-badge">{name}</span>
                    }).collect::<Vec<_>>()}
                </div>
                <div class="integrations-links">
                    <a href="https://docs.opencrabs.com/channels/overview.html" target="_blank">"View all integrations →"</a>
                </div>
            </div>
        </section>
    }
}

// ── Testimonials ────────────────────────────────────────────────────────────

#[component]
fn Testimonials() -> impl IntoView {
    let quotes: Vec<(&str, &str, &str)> = vec![
        (
            "Outperforms Hermes because it's in Rust and very tight use case wise. Reminds me of what I hoped OpenClaw would be.",
            "@5fakb48",
            "https://x.com/5fakb48/status/2069564004104561125",
        ),
        (
            "I tested OpenCrabs with OpenRouter and a random Nvidia free model, and right off the bat I like it more than the alternatives. Setup was a lot easier.",
            "@mariodian",
            "https://x.com/mariodian/status/2031545206559170847",
        ),
        (
            "Local STT + TTS in ~135MB is impressive. Feels like the voice layer for local agents is getting a lot more practical.",
            "@AlexBuildsCo",
            "https://x.com/AlexBuildsCo/status/2031037889094562160",
        ),
        (
            "Every day I tweak Crabs to help me do things faster. BTW Trello integration is amazing. My agent now has a board with tasks it manages on its own.",
            "@opryshok",
            "https://x.com/opryshok/status/2030282770912522288",
        ),
        (
            "After I was using OpenClaw and Hermes, OpenCrabs is the first agent I click with.",
            "@anschmieg",
            "https://github.com/adolfousier/opencrabs/pull/61#issuecomment-4201983933",
        ),
        (
            "I decided to check out Opencrabs. I installed it on Ubuntu Linux and it just works. Lot better than openclaw and if continued development it may even get enough traction among developers.",
            "@0xMaheshK",
            "https://x.com/0xMaheshK/status/2062763007529320646",
        ),
        (
            "I fired my assistant bc of OpenCrabs! The crab now monitors my Telegram group, OCRs invoices, puts them into the banking app, I approve manually, it downloads notification and puts it back to the Telegram group, and files transactions in my management accounting app.",
            "@leshchenko1979",
            "https://x.com/leshchenko1979/status/2068603163448353058",
        ),
        (
            "It's easy to become obsessed w/ crabs, I already feel it.",
            "Roman Gall · Telegram DM",
            "",
        ),
    ];

    view! {
        <section id="testimonials">
            <div class="container">
                <div class="testimonials-header">
                    <h2 class="section-title" style="margin-bottom: 0">
                        <span class="chevron">"›"</span>
                        "What People Say"
                    </h2>
                </div>
                <div class="testimonials-grid">
                    {quotes.into_iter().map(|(quote, author, link)| view! {
                        <div class="testimonial-card">
                            <p>"\""{ quote }"\""</p>
                            {if link.is_empty() {
                                view! { <span class="testimonial-author">{author}</span> }.into_any()
                            } else {
                                view! { <a class="testimonial-author" href={link} target="_blank" rel="noopener">{author}</a> }.into_any()
                            }}
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

// ── Community ───────────────────────────────────────────────────────────────

#[component]
fn Community() -> impl IntoView {
    view! {
        <section id="community">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Join the Colony"
                </h2>
                <div class="community-grid">
                    <a href="https://github.com/adolfousier/opencrabs" class="community-card" target="_blank">
                        <span class="icon">"🐙"</span>
                        <h3>"GitHub"</h3>
                        <p>"View the source"</p>
                    </a>
                    <a href="https://github.com/adolfousier/opencrabs/issues" class="community-card" target="_blank">
                        <span class="icon">"🐛"</span>
                        <h3>"Issues"</h3>
                        <p>"Report bugs & request features"</p>
                    </a>
                    <a href="https://github.com/adolfousier/opencrabs/blob/main/CHANGELOG.md" class="community-card" target="_blank">
                        <span class="icon">"📋"</span>
                        <h3>"Changelog"</h3>
                        <p>"See what's new"</p>
                    </a>
                    <a href="https://docs.opencrabs.com" class="community-card" target="_blank">
                        <span class="icon">"📖"</span>
                        <h3>"Documentation"</h3>
                        <p>"Learn the ropes"</p>
                    </a>
                </div>
            </div>
        </section>
    }
}

// ── Newsletter (coming soon — self-hosted on droplet with Resend) ───────────
//
// #[component]
// fn Newsletter() -> impl IntoView {
//     view! {
//         <section>
//             <div class="container">
//                 <div class="newsletter">
//                     <h2>
//                         <span class="chevron">"› "</span>
//                         "Stay in the Loop"
//                     </h2>
//                     <p>"Get updates on new features, integrations, and crab wisdom. No spam, unsubscribe anytime."</p>
//                     <div class="newsletter-form">
//                         <input type="email" placeholder="your@email.com" />
//                         <button>"Subscribe →"</button>
//                     </div>
//                 </div>
//             </div>
//         </section>
//     }
// }

// ── Footer ──────────────────────────────────────────────────────────────────

#[component]
fn Footer(stars: Signal<u32>) -> impl IntoView {
    let star_cta = move || {
        let count = stars.get();
        if count > 0 {
            format!("★ {} stars on GitHub — give us one more!", count)
        } else {
            "★ Star us on GitHub!".to_string()
        }
    };

    view! {
        <footer>
            <div class="container">
                <a href="https://github.com/adolfousier/opencrabs" class="footer-star-cta" target="_blank">
                    {star_cta}
                </a>
                <div class="footer-links">
                    <a href="https://docs.opencrabs.com" target="_blank">"Docs"</a>
                    <a href="https://github.com/adolfousier/opencrabs">"GitHub"</a>
                    <a href="https://github.com/adolfousier/opencrabs/blob/main/CHANGELOG.md">"Changelog"</a>
                    <a href="https://github.com/adolfousier/opencrabs/blob/main/LICENSE">"MIT License"</a>
                </div>
                <p class="footer-tagline">
                    "Built with 🦀 by "
                    <a href="https://github.com/adolfousier">"Adolfo Usier"</a>
                    " & the OpenCrabs community."
                </p>
            </div>
        </footer>
    }
}
