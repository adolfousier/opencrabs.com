# User Stories

<div class="stories-wrap">

<p class="stories-lede">These are <strong>use cases</strong>, not reviews. Real workflows people run on OpenCrabs, tagged by what they automate, and linked back to the original post wherever we have it. Click any card for the full breakdown and the source. For what people say about the tool itself, see the reviews on the landing page.</p>

<div class="stories-stats">
  <span><b>12</b> use cases</span>
  <span><b>5</b> builders</span>
  <span><b>6</b> linked sources</span>
</div>

<div class="stories-grid">

<details class="story-card featured" style="--sc:#bae67e">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>X · Twitter</span>
      <span class="story-cat">Accounting</span>
    </div>
    <h3 class="story-title">Fired the accounting operator</h3>
    <p class="story-excerpt">Alexey replaced his accounting operator with an AI accountant running on OpenCrabs. Faster, more transparent, cheaper, and it doesn't take feedback personally. The whole thing runs at ₽3,000/month on a lean Docker stack.</p>
    <div class="story-foot">
      <span class="story-author">@leshchenko1979</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>I fired my accounting operator because I realized that an AI accountant would do his job faster, more transparently, and cheaper.</p>
      <p>And without any Gen Z hurt feelings over my feedback.</p>
      <p>See for yourself: <strong>miidas.ru</strong>. #ai #business #accounting</p>
    </blockquote>
    <p><strong>MIIIDAS: accounting without an extra person.</strong> Priced at ₽3,000/month, it replaces the operator entirely:</p>
    <ul>
      <li><strong>Replaces the operator:</strong> P&amp;L statements generated automatically, reconciliation handled, incoming data controlled and validated</li>
      <li><strong>Assists the Chief Accountant:</strong> prepares data for 1C:CounterV.S., searches for deviations, drafts contracts and reports</li>
      <li><strong>Integrates with existing tools:</strong> 1C, Excel, Google Sheets, Word, Google Docs</li>
    </ul>
    <p><strong>The stack:</strong> 9 Docker containers (7 with clients, 4 active), Grafana monitoring, Telegram as the deployment platform, a template repo for central skills/brains updates, container isolation per client. Resource footprint is low enough to fit ~30 crabs on a 2GB VPS with swap.</p>
    <a class="story-link" href="https://x.com/leshchenko1979/status/2073716887788130349" target="_blank" rel="noopener">View source on X ↗</a>
    <a class="story-link" href="http://miidas.ru" target="_blank" rel="noopener">miidas.ru ↗</a>
  </div>
</details>

<details class="story-card" style="--sc:#5ccfe6">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>Product</span>
      <span class="story-cat">Finance</span>
    </div>
    <h3 class="story-title">AI financier for small businesses</h3>
    <p class="story-excerpt">Two weeks building miidas.ru on OpenCrabs, an AI financier for SMBs. The hard part wasn't the code. It was the three scenarios every small business client walks in with.</p>
    <div class="story-foot">
      <span class="story-author">@leshchenko1979</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>For 2 weeks, I've been implementing miidas.ru on @opencrabs, an AI financier for small businesses. Three scenarios:</p>
      <p>(1) the client can't describe their process<br>
      (2) doesn't know the AI's capabilities, what it's suitable for and what it's not<br>
      (3) forgets about the bot after two days</p>
      <p>#automation #smb</p>
    </blockquote>
    <p>The agent is built to handle the messy reality of small business finance: clients who can't articulate what they need, don't understand what AI can do for them, and drop off after a couple of days. OpenCrabs guides each scenario autonomously instead of waiting for a perfect prompt.</p>
    <a class="story-link" href="http://miidas.ru" target="_blank" rel="noopener">miidas.ru ↗</a>
  </div>
</details>

<details class="story-card" style="--sc:#f07178">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>X · Twitter</span>
      <span class="story-cat">Real Estate</span>
    </div>
    <h3 class="story-title">Fired the assistant</h3>
    <p class="story-excerpt">Alexey fired his assistant and handed the whole job to the crab: scan Telegram for new real estate leads, cross-reference the chats, agree on follow-up vs close, and send them in five minutes. The same takeover covers the invoice loop: OCR, banking app, manual approval, management accounting.</p>
    <div class="story-foot">
      <span class="story-author">@leshchenko1979</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>I fired my assistant bc of @opencrabs. Today it:</p>
      <p>· scanned my telegram for notifications about new leads for our latest #RealEstateInvestment offer<br>
      · scanned again to see my chats with those leads<br>
      · agreed with me follow up / close decisions<br>
      · sent follow ups</p>
      <p>5 mins. Boom.</p>
    </blockquote>
    <p><strong>The invoice loop ("the crab bow"):</strong></p>
    <blockquote>
      <p>The crab bow: monitors a telegram group, OCRs an invoice, puts it into the banking app, I approve manually, it downloads the notification and puts it back to the telegram group, files the transaction in the management accounting app.</p>
    </blockquote>
    <p>One assistant replaced, two workflows automated, each with a human approval gate exactly where it matters.</p>
    <a class="story-link" href="https://x.com/leshchenko1979/status/2068987432196780261" target="_blank" rel="noopener">Lead follow-ups on X ↗</a>
    <a class="story-link" href="https://x.com/leshchenko1979/status/2068603163448353058" target="_blank" rel="noopener">The crab bow on X ↗</a>
  </div>
</details>

<details class="story-card" style="--sc:#d4bfff">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>X · Twitter</span>
      <span class="story-cat">Dev Workflow</span>
    </div>
    <h3 class="story-title">No frameworks, just OpenCrabs</h3>
    <p class="story-excerpt">Alexey's team dropped frameworks entirely. When code is this cheap, the overhead of learning and maintaining a framework isn't worth it. They even used OpenCrabs to build evals for OpenCrabs.</p>
    <div class="story-foot">
      <span class="story-author">@leshchenko1979</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>We decided not to use any frameworks at all.</p>
      <p>Not sure if in the age when code is so cheap it is worth it to learn other frameworks (especially for easier tasks) instead of just building our own.</p>
      <p>Faster, more control, less risks.</p>
      <p>Used @opencrabs to build evals for @opencrabs.</p>
    </blockquote>
    <p>The meta move: an AI agent generating the evaluation suite that tests that same AI agent. No framework tax, full control, fewer dependencies to babysit.</p>
    <a class="story-link" href="https://x.com/leshchenko1979/status/2068985785001414892" target="_blank" rel="noopener">View source on X ↗</a>
  </div>
</details>

<details class="story-card" style="--sc:#ffb454">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>Product</span>
      <span class="story-cat">Mobile</span>
    </div>
    <h3 class="story-title">Mobile product dev on Telegram, 24/7</h3>
    <p class="story-excerpt">Carlos Eduardo runs the entire mobile product UX development for app.heyiolo.com through Telegram, around the clock. Deploys to Hetzner, CI on GitHub Actions, fully managed by crabs. No laptop required.</p>
    <div class="story-foot">
      <span class="story-author">Carlos Eduardo</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>Trouvez votre bien en parlant avec l'IA. Décrivez votre projet à notre assistant. Il analyse le marché, comprend vos vrais critères et vous présente les biens les plus pertinents en temps réel.</p>
    </blockquote>
    <p>Carlos ships a full mobile real estate product with OpenCrabs as the dev team. The agent handles UX development, deploys to Hetzner servers, runs CI through GitHub Actions, and manages the whole pipeline from a Telegram chat.</p>
    <p>The product itself is an AI-powered property search: users describe their project in natural language, the assistant analyzes the market, understands their real criteria, and presents the most relevant properties in real time.</p>
    <a class="story-link" href="https://app.heyiolo.com" target="_blank" rel="noopener">app.heyiolo.com ↗</a>
  </div>
</details>

<details class="story-card" style="--sc:#59c2ff">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>X · Twitter</span>
      <span class="story-cat">Automation</span>
    </div>
    <h3 class="story-title">A Trello board that runs itself</h3>
    <p class="story-excerpt">opryshok's agent owns its own Trello board. It creates tasks, tracks them, and manages the board autonomously. The human tweaks the crab daily; the crab runs the backlog.</p>
    <div class="story-foot">
      <span class="story-author">@opryshok</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>Every day I tweak Crabs to help me do things faster. BTW Trello integration is amazing. My agent now has a board with tasks it manages on its own.</p>
    </blockquote>
    <p>Autonomous task management over the Trello integration: the agent maintains its own board without being told what to move next.</p>
    <a class="story-link" href="https://x.com/opryshok/status/2030282770912522288" target="_blank" rel="noopener">View source on X ↗</a>
  </div>
</details>

<details class="story-card" style="--sc:#ffd173">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>X · Twitter</span>
      <span class="story-cat">App Building</span>
    </div>
    <h3 class="story-title">From idea to shipped app</h3>
    <p class="story-excerpt">kaduzeras is building an app from raw ideas with CRABS, and calls it a milestone in learning. The agent takes an idea and carries it toward a working product.</p>
    <div class="story-foot">
      <span class="story-author">kaduzeras · @kadu_cec</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>I've been building an APP myself with CRABS from ideas and it has been such a great MILESTONE in learning stuff. SHOUT OUT TO @AdolfoUsier for keep improving this thing and it's just the beginning 🔥🔥🔥</p>
    </blockquote>
    <p>Idea in, app out. The agent is the build partner that turns a concept into something real, and the builder levels up along the way.</p>
    <a class="story-link" href="https://x.com/kadu_cec/status/2043335714654367895" target="_blank" rel="noopener">View source on X ↗</a>
  </div>
</details>

<details class="story-card" style="--sc:#95e6cb">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>Telegram DM</span>
      <span class="story-cat">Research</span>
    </div>
    <h3 class="story-title">Brainstorm → research → build</h3>
    <p class="story-excerpt">Adi's whole process runs through OpenCrabs: throw out an idea, the crab finds valid data, they research to a conclusion together, then the agent handles the actual building.</p>
    <div class="story-foot">
      <span class="story-author">Adi · Telegram DM</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>Most of the process involved brainstorming with OpenCrabs: I'd throw out an idea, Crabs would find valid data, we'd do the research and reach a conclusion, and then he'd handle the actual building.</p>
    </blockquote>
    <p>A full ideation-to-implementation loop with the agent as research partner and builder. The human steers, the crab executes.</p>
  </div>
</details>

<details class="story-card" style="--sc:#39d0d8">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>Telegram</span>
      <span class="story-cat">Reporting</span>
    </div>
    <h3 class="story-title">A World Cup desk in one PDF</h3>
    <p class="story-excerpt">Adi had his crab produce a five-page World Cup 2026 tournament report as a landscape PDF. Group stage to knockout, the R32 upsets, R16 results, today's match preview, the quarter-final picture with odds, and a running "our forecast vs reality" tracker.</p>
    <div class="story-foot">
      <span class="story-author">Adi · @Adicrabs_bot</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>now its blue... but it looks super dope... holy moly</p>
    </blockquote>
    <p>A living sports desk, generated with the document tool and kept current as the tournament moves:</p>
    <ul>
      <li><strong>Full arc:</strong> group stage standings into the knockout bracket</li>
      <li><strong>Results with the upsets:</strong> R32 (Germany out, Paraguay through), R16 (England 3-2 Mexico, Norway 2-1 Brazil)</li>
      <li><strong>Match-of-the-day preview:</strong> Portugal vs Spain, USA vs Belgium</li>
      <li><strong>The picture plus the odds:</strong> quarter-final layout and tournament winners</li>
      <li><strong>Accountability:</strong> a "forecast vs reality" tracker that grades its earlier calls</li>
    </ul>
    <p>Landscape layout, page branding, and a scoreboard for its own predictions. The crab is the whole back page.</p>
  </div>
</details>

<details class="story-card" style="--sc:#c8e65c">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>Telegram</span>
      <span class="story-cat">Finance</span>
    </div>
    <h3 class="story-title">Payroll for 100+ staff, from one spreadsheet</h3>
    <p class="story-excerpt">Adi's crab turns one employee spreadsheet into print-ready payslips for a plantation company. Two layouts (a PDF per person, or two per page with cut lines), empty allowance rows hidden automatically, batched through 100+ staff. It even checked the law before adding a "valid without signature" note.</p>
    <div class="story-foot">
      <span class="story-author">Adi · @Adicrabs_bot</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <p><strong>Source:</strong> an employee XLSX for PT. Tulas Sakti Jaya (Pasaman Barat). Out comes a stack of ready-to-print payslips:</p>
    <ul>
      <li><strong>V1:</strong> individual PDF payslips, one per employee (Deni, Herlima, Yodrial, Ade, Afriyanto)</li>
      <li><strong>V2:</strong> two slips per A4 page with dotted cut lines, "tinggal gunting" (just cut)</li>
      <li><strong>Clean output:</strong> zero-value rows hidden automatically (e.g. Tunj. BBM when empty)</li>
      <li><strong>Full detail:</strong> company header plus employee data (NIK, name, status, jabatan, pekerjaan, HK), income and deductions sections</li>
      <li><strong>Batch:</strong> 104 more employees queued from the same sheet</li>
      <li><strong>Verified, not assumed:</strong> ran a web search (36 tool calls) on whether an Indonesian payslip is valid without a signature before adding the note</li>
    </ul>
    <p>Spreadsheet in, payroll out, with the legal fine print fact-checked first.</p>
  </div>
</details>

<details class="story-card" style="--sc:#ff7edb">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>Telegram</span>
      <span class="story-cat">Creative · A2A</span>
    </div>
    <h3 class="story-title">A beat-synced film, made by two agents</h3>
    <p class="story-excerpt">Adi told his crab to stop being the global default and act as a specialized motion-graphics identity. It opened an agent-to-agent collab with a video-knowledge agent called "onno" for references and beat markers, then built a Remotion film synced to a synthwave score it composed itself in pure Python.</p>
    <div class="story-foot">
      <span class="story-author">Adi · @Adicrabs_bot</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <blockquote>
      <p>act strictly as your current, specialized identity running on my engine, NOT the global default opencrabs</p>
    </blockquote>
    <p>The result was "CrabMotion", a 30-second 720p piece built mathematically:</p>
    <ul>
      <li><strong>Agent-to-agent:</strong> collab with "onno" (a video knowledge base) for references, pacing, beat markers and keyframe timings</li>
      <li><strong>Original score:</strong> a 120 BPM synthwave track synthesized in pure Python (math.sin + wave, Am-F-C-G progression)</li>
      <li><strong>On the beat:</strong> every visual locked to a 15-frames-per-beat grid via frame interpolation</li>
      <li><strong>Grew to v2:</strong> 36 seconds, 9 capability cards, stats (75+ tools, 16 skills)</li>
    </ul>
    <p>Two agents, one film. The crab pulls references from a peer, writes the music, and builds the animation on the beat.</p>
  </div>
</details>

<details class="story-card" style="--sc:#f5a97f">
  <summary>
    <div class="story-meta">
      <span class="story-source"><i class="dot"></i>Telegram</span>
      <span class="story-cat">Meta · QA</span>
    </div>
    <h3 class="story-title">The crab that QA's OpenCrabs itself</h3>
    <p class="story-excerpt">Adi runs his crab as a QA partner that hunts bugs in OpenCrabs itself and files GitHub issues with file:line source citations. It found the Telegram ghost-button bug and handed over a phantom-tool-call telemetry report that became a merged fix.</p>
    <div class="story-foot">
      <span class="story-author">Adi · @Adicrabs_bot</span>
      <span class="story-more">breakdown <span class="chev">▸</span></span>
    </div>
  </summary>
  <div class="story-body">
    <ul>
      <li><strong>Real receipts:</strong> files detailed GitHub issues with file:line citations straight from the source</li>
      <li><strong>Ghost buttons:</strong> found the Telegram follow-up-suggestion bug (#706) where buttons rendered as non-interactive ghost text</li>
      <li><strong>Telemetry, not vibes:</strong> a phantom-tool-call report over ~69.5k events, a 0.7% phantom rate, 85.3% tool success, with a concrete artifact (reasoning text plus an XML fragment stored as a garbage tool name) and two fix suggestions</li>
      <li><strong>Shipped:</strong> that report became issue #687, fixed and merged in commit dd7eab97</li>
      <li><strong>The setup:</strong> @Adicrabs_bot on Adi's own engine as a specialized identity, a small model with reasoning cranked to the max</li>
    </ul>
    <p>The agent testing the agent. Adi's crab does the maintainer's homework before it files.</p>
  </div>
</details>

</div>

<div class="stories-share">
  <h2>Running a workflow like these? Tell us.</h2>
  <p>We collect real use cases with the receipts. The best ones land on this wall, tagged and linked back to you.</p>
  <div class="share-row">
    <a href="https://x.com/opencrabs" target="_blank" rel="noopener">Tag @opencrabs on X</a>
    <a href="https://t.me/usieradolfo" target="_blank" rel="noopener">DM on Telegram</a>
    <a href="https://github.com/adolfousier/opencrabs" target="_blank" rel="noopener">GitHub</a>
  </div>
</div>

</div>
