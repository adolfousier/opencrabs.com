use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Nav />
        <Hero />
        <Testimonials />
        <QuickStart />
        <Features />
        <Integrations />
        <Community />
        <Newsletter />
        <Footer />
    }
}

// ── Navigation ──────────────────────────────────────────────────────────────

#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav>
            <div class="container">
                <a href="/" class="nav-logo">
                    <span class="crab-icon">"🦀"</span>
                    "OpenCrabs"
                </a>
                <ul class="nav-links">
                    <li><a href="#features">"Features"</a></li>
                    <li><a href="#integrations">"Integrations"</a></li>
                    <li><a href="#community">"Community"</a></li>
                    <li>
                        <a href="https://github.com/adolfousier/opencrabs" class="btn-github" target="_blank">
                            "GitHub"
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

// ── Hero ────────────────────────────────────────────────────────────────────

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="container">
                <span class="hero-crab">"🦀"</span>
                <h1>"OpenCrabs"</h1>
                <p class="hero-tagline">"THE AI AGENT THAT LIVES IN YOUR TERMINAL."</p>
                <p class="hero-description">
                    "Runs code, manages files, searches the web, schedules tasks, "
                    "and connects to Telegram, Discord, Slack, and WhatsApp. "
                    "All from your terminal or any chat app."
                </p>
                <a href="https://github.com/adolfousier/opencrabs/releases/latest" class="hero-badge">
                    <span class="badge-new">"NEW"</span>
                    "v0.2.54 — /evolve self-update, smarter compaction"
                    <span class="arrow">" →"</span>
                </a>
            </div>
        </section>
    }
}

// ── Quick Start ─────────────────────────────────────────────────────────────

#[component]
fn QuickStart() -> impl IntoView {
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
                            <button class="active">"Binary"</button>
                            <button>"Cargo"</button>
                            <button>"Source"</button>
                        </div>
                        <span class="terminal-platform">"macOS / Linux"</span>
                    </div>
                    <div class="terminal-body">
                        <div>
                            <span class="terminal-comment">"# Download latest release for your platform"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">
                                "curl -fsSL https://github.com/adolfousier/opencrabs/releases/latest/download/opencrabs-$(uname -m)-$(uname -s | tr A-Z a-z).tar.gz | tar xz"
                            </span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"./opencrabs chat"</span>
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
            "Runs on Your Machine",
            "Mac, Windows, or Linux. Anthropic, OpenAI, Gemini, or any local model. Private by default — your data stays yours.",
        ),
        (
            "💬",
            "Any Chat App",
            "Talk to it on Telegram, Discord, Slack, or WhatsApp. Works in DMs and group chats. Or just use the TUI.",
        ),
        (
            "🧠",
            "Persistent Memory",
            "Remembers you across sessions. Your preferences, your context, your AI. Semantic search over everything.",
        ),
        (
            "⚡",
            "40+ Built-in Tools",
            "File ops, bash, web search, code execution, image gen, document parsing. No plugins needed for the basics.",
        ),
        (
            ">_",
            "Full System Access",
            "Read and write files, run shell commands, execute scripts. Full access or sandboxed — your choice.",
        ),
        (
            "🔧",
            "Self-Evolving",
            "Type /evolve to download the latest version. Type /rebuild to build from source. The crab upgrades itself.",
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
    let channels = vec!["📱 Telegram", "💬 Discord", "🔔 Slack", "📞 WhatsApp"];
    let providers = vec![
        "🤖 Anthropic",
        "🧠 OpenAI",
        "💎 Gemini",
        "🌐 OpenRouter",
        "🏠 Ollama",
        "📡 LM Studio",
    ];
    let tools = vec![
        "🐙 GitHub",
        "📋 Trello",
        "📧 Gmail",
        "📅 Calendar",
        "🐦 Twitter",
        "🔍 Brave Search",
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
                    <a href="https://github.com/adolfousier/opencrabs#features">"View all integrations →"</a>
                </div>
            </div>
        </section>
    }
}

// ── Testimonials ────────────────────────────────────────────────────────────

#[component]
fn Testimonials() -> impl IntoView {
    let quotes = vec![
        (
            "One thing Crabs does better — when I change config and send messages while restarting, it always picks up and replies as soon as it wakes up. Hot reload at runtime. Other agents just hang and leave you waiting forever.",
            "@adolfousier",
        ),
        (
            "Built a full cron scheduler, channel message capture, and 700 tests in one session. The crab doesn't sleep.",
            "@meetneuraai",
        ),
        (
            "/evolve is genius. The agent literally updates itself mid-conversation and picks up where it left off.",
            "@ItsYourTimeDev",
        ),
        (
            "Every day I tweak Crabs to help me do things faster. BTW Trello integration is amazing. My agent now has a board with tasks it manages on its own.",
            "@opryshok",
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
                    {quotes.into_iter().map(|(quote, author)| view! {
                        <div class="testimonial-card">
                            <p>"\""{ quote }"\""</p>
                            <span class="testimonial-author">{author}</span>
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
                    <a href="https://github.com/adolfousier/opencrabs#readme" class="community-card" target="_blank">
                        <span class="icon">"📖"</span>
                        <h3>"Documentation"</h3>
                        <p>"Learn the ropes"</p>
                    </a>
                </div>
            </div>
        </section>
    }
}

// ── Newsletter ──────────────────────────────────────────────────────────────

#[component]
fn Newsletter() -> impl IntoView {
    view! {
        <section>
            <div class="container">
                <div class="newsletter">
                    <h2>
                        <span class="chevron">"› "</span>
                        "Stay in the Loop"
                    </h2>
                    <p>"Get updates on new features, integrations, and crab wisdom. No spam, unsubscribe anytime."</p>
                    <div class="newsletter-form">
                        <input type="email" placeholder="your@email.com" />
                        <button>"Subscribe →"</button>
                    </div>
                </div>
            </div>
        </section>
    }
}

// ── Footer ──────────────────────────────────────────────────────────────────

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="container">
                <div class="footer-links">
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
