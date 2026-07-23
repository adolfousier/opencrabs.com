# User Stories

Real stories from the OpenCrabs community. What people are actually building, automating, and shipping with their agents.

---

## AI Financier for Small Businesses

> For 2 weeks, I've been implementing [miidas.ru](http://miidas.ru) on @opencrabs, an AI financier for small businesses. Three scenarios: (1) the client can't describe their process (2) doesn't know the AI's capabilities, what it's suitable for and what it's not (3) forgets about the bot after two days.
>
> **Alexey** · [#automation](https://x.com/search?q=%23automation) [#smb](https://x.com/search?q=%23smb)

Alexey built a full AI financial assistant for Russian SMBs on OpenCrabs. The agent handles the messy reality of small business finance: clients who can't articulate what they need, don't understand what AI can do for them, and drop off after a couple days. The agent guides them through all three scenarios autonomously.

### What miidas does

**MIIIDAS: accounting without an extra person.** Priced at ₽3,000/month, it replaces the accounting operator entirely:

- **Replaces the operator:** P&L statements generated automatically, reconciliation handled, incoming data controlled and validated
- **Assists the Chief Accountant:** prepares data for 1C:CounterV.S., searches for deviations, drafts contracts and reports
- **Integrates with existing tools:** 1C, Excel, Google Sheets, Word, Google Docs

### The stack

Alexey runs the whole thing on a lean Docker setup:

- **9 containers** (7 with clients, 4 active at any time)
- **Grafana** for monitoring
- **Telegram** as the deployment platform ("when you decide to deploy a school of crabs via Telegram, you'll have the infra ready")
- **Template repo** for central skills/brains updates across all containers
- **Container isolation** per client
- **Resource footprint:** low enough to fit ~30 crabs on a 2GB VPS with swap

---

## Fired the Accounting Operator

> I fired my accounting operator because I realized that an AI accountant would do his job faster, more transparently, and cheaper. And without any Gen Z hurt feelings over my feedback. See for yourself: [miidas.ru](http://miidas.ru).
>
> **Alexey** · [#ai](https://x.com/search?q=%23ai) [#business](https://x.com/search?q=%23business) [#accounting](https://x.com/search?q=%23accounting)

Same agent, different angle. The accounting operator was replaced entirely. The AI accountant processes transactions, generates P&L statements, handles reconciliation, and drafts reports without the overhead, and without taking feedback personally. At ₽3,000/month it costs a fraction of a human operator.

---

## Fired the Assistant

> I fired my assistant bc of @opencrabs. Today it:
> - scanned my Telegram for notifications about new leads for our latest #RealEstateInvestment offer
> - scanned again to see my chats with those leads
> - agreed with me follow up / close decisions
> - sent follow ups
>
> 5 mins. Boom.
>
> **Alexey**

The agent scanned Telegram for real estate leads, cross-referenced existing conversations, checked in with Alexey on follow-up vs close decisions, then sent the follow-ups. The whole pipeline took 5 minutes.

---

## No Frameworks, Just OpenCrabs

> We decided not to use any frameworks at all. Not sure if in the age when code is so cheap it is worth it to learn other frameworks (especially for easier tasks) instead of just building our own. Faster, more control, less risks. Used @opencrabs to build evals for @opencrabs.
>
> **Alexey**

Alexey's team ditched frameworks entirely. When code generation is this cheap, the overhead of learning and maintaining a framework isn't worth it for most tasks. They even used OpenCrabs to build evaluation suites for OpenCrabs itself.

---

## Mobile Product Dev on Telegram, 24/7

> Carlos Eduardo runs the entire mobile product UX development for [app.heyiolo.com](https://app.heyiolo.com) through Telegram, 24/7. Deploying to Hetzner, CI on GitHub Actions, fully managed by crabs.
>
> **Carlos Eduardo** · [app.heyiolo.com](https://app.heyiolo.com)

Carlos ships a full mobile real estate product ("Trouvez votre bien en parlant avec l'IA") with OpenCrabs as the dev team. The agent handles UX development, deploys to Hetzner servers, runs CI through GitHub Actions, and manages the whole pipeline from a Telegram chat. No laptop required.

The product itself is an AI-powered property search: users describe their project in natural language, the assistant analyzes the market, understands their real criteria, and presents the most relevant properties in real time.

---

## Share Your Story

Using OpenCrabs for something interesting? We want to hear about it.

- **X/Twitter:** Tag [@opencrabs](https://x.com/opencrabs) or use #opencrabs
- **Telegram:** DM [@usieradolfo](https://t.me/usieradolfo)
- **GitHub:** [github.com/adolfousier/opencrabs](https://github.com/adolfousier/opencrabs)
