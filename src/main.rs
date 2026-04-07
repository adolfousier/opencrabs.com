use leptos::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

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
    let stars = LocalResource::new(|| fetch_star_count());
    let stars_signal = Signal::derive(move || {
        stars.get().flatten().unwrap_or(0)
    });

    let release = LocalResource::new(|| fetch_latest_release());
    let tag_signal = Signal::derive(move || {
        release.get().flatten().map(|r| r.tag_name.clone()).unwrap_or_default()
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
    let parts: Vec<&str> = published_at.split('T').next().unwrap_or("").split('-').collect();
    if parts.len() == 3 {
        let month = match parts[1] {
            "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
            "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
            "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
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
    let release = LocalResource::new(|| fetch_latest_release());

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
                <p class="hero-tagline">"THE AI AGENT THAT LIVES IN YOUR TERMINAL."</p>
                <p class="hero-description">
                    "Open-source AI agent built in Rust. 50+ tools, 11+ providers, multi-agent teams, "
                    "self-healing, and every channel. 1,827 tests."
                </p>
                <a href="https://github.com/adolfousier/opencrabs/releases/latest" class="hero-badge">
                    <span class="badge-new">"LATEST"</span>
                    {badge_text}
                    <span class="arrow">" →"</span>
                </a>
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
                                            <span class="terminal-comment">"# Download latest release for your platform"</span>
                                        </div>
                                        <div>
                                            <span class="terminal-prompt">"$ "</span>
                                            <span class="terminal-cmd">
                                                "ARCH=$(uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/') OS=$(uname -s | tr A-Z a-z) TAG=$(curl -s https://api.github.com/repos/adolfousier/opencrabs/releases/latest | grep -o '\"tag_name\":\"[^\"]*\"' | cut -d'\"' -f4) && curl -fsSL \"https://github.com/adolfousier/opencrabs/releases/download/$TAG/opencrabs-$TAG-$OS-$ARCH.tar.gz\" | tar xz"
                                            </span>
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
                            <span class="terminal-cmd">"curl -fsSL https://raw.githubusercontent.com/adolfousier/opencrabs/main/scripts/setup.sh | bash"</span>
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
            "~23 MB Single Binary",
            "Mac, Windows, or Linux. Anthropic, OpenAI, Gemini, GitHub Copilot, or any local model. Private by default — your data stays yours.",
        ),
        (
            "💬",
            "Any Chat App",
            "Talk to it on Telegram, Discord, Slack, WhatsApp, or Trello. Works in DMs and group chats with persistent sessions. Or just use the TUI.",
        ),
        (
            "🧠",
            "Persistent Memory",
            "Remembers you across sessions. Your preferences, your context, your AI. Semantic search over everything.",
        ),
        (
            "⚡",
            "50+ Built-in Tools",
            "File ops, bash, web search, code execution, image gen, browser automation, local voice STT & TTS, sub-agent orchestration. Define custom tools at runtime.",
        ),
        (
            "🪟",
            "Split Panes",
            "Tmux-style horizontal and vertical pane splitting. Run 10 sessions side by side, each with its own provider and context. All processing in parallel.",
        ),
        (
            "🤖",
            "Multi-Agent & Teams",
            "Typed sub-agents (General, Explore, Plan, Code, Research) with filtered tool registries. Team orchestration spawns N agents in parallel and broadcasts to all. 20+ CLI subcommands, daemon mode with health endpoints.",
        ),
        (
            "🌐",
            "Browser Automation",
            "Native headless Chrome control via CDP. Navigate, click, type, screenshot, extract content, run JS. Smart detection of your default Chromium browser.",
        ),
        (
            "📐",
            "CODE.md — Built-in Coding Standards",
            "Ships with a brain template that enforces modular files, strict size limits, test coverage, and security-first patterns. Your agent writes production code, not prototypes.",
        ),
        (
            "🛡️",
            "Self-Healing",
            "Auto-recovers corrupted config, tracks per-provider health with auto-failover, 65% context budget management with LLM compaction. Cross-channel crash recovery routes pending requests back to the originating Telegram/Discord/Slack chat on restart. Stuck stream detection, 10-min CLI idle timeout, DB integrity checks.",
        ),
        (
            "👤",
            "Multi-Profile",
            "Run multiple isolated instances from one binary. Each profile gets its own config, brain files, sessions, and daemon service. Token-lock isolation prevents two profiles from fighting over the same bot. Export/import profiles as portable archives.",
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
    let channels = vec!["📱 Telegram", "💬 Discord", "🔔 Slack", "📞 WhatsApp", "📋 Trello"];
    let providers = vec![
        "🤖 Anthropic",
        "🧠 OpenAI",
        "💎 Gemini",
        "🌐 OpenRouter",
        "🐙 GitHub Copilot",
        "🇨🇳 z.ai GLM",
        "🏠 Ollama",
        "📡 LM Studio",
        "🔮 MiniMax",
        "💻 Claude CLI",
        "⌨️ OpenCode CLI",
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
            "That 130MB all-in path is a big deal for folks who want voice agents without API burn.",
            "@CurrentlyKI",
            "https://x.com/CurrentlyKI/status/2031188774483538125",
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
