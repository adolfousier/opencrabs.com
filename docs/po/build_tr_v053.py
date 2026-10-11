import json

DELTA_KEYS = [
    '**9,032 tests**',
    '**Epistemic memory (v0.5.3)**',
    '**Finished theme engine (v0.5.3)**',
    '**First-class Windows (v0.5.3)**',
    '**Trigger-gated cron (v0.5.3)**',
    '**WhatsApp suggestion polls (v0.5.3)**',
    '**Windows is a first-class platform (v0.5.3):**',
    'Memory management (list, get, stats, prune)',
]

T = {
 'pt-PT': {
  'tests': '**9,032 testes** em providers, ferramentas, channels, TUI, self-healing e automação de browser.',
  'epi': '**Memória epistémica (v0.5.3)** — os factos do MEMORY.md ganham confidence e decay (#1643), as secções frias arquivam-se sozinhas e o `opencrabs memory prune` mostra o que ficou frio antes de tocar em nada (#1657)',
  'theme': '**Motor de temas terminado (v0.5.3)** — todos os widgets do TUI resolvem as cores através do tema, com um guard de suite contra novos sítios de ANSI cru, e os temas podem declarar um canvas background (#1634)',
  'win': '**Windows de primeira classe (v0.5.3)** — shell handling por plataforma, leitura de ficheiros BOM/UTF-16, fallbacks de tilde e cwd, com a CI a compilar Windows e a fazer gate aos PRs (#627)',
  'cron': '**Cron com triggers (v0.5.3)** — um job corre um pre-flight check e só executa quando a condição é satisfeita (output não vazio, exit code, regex), os runs saltados custam 0 tokens e um trigger disparado pode despachar um goal para uma session (#233)',
  'polls': '**Polls de sugestões no WhatsApp (v0.5.3)** — conjuntos de follow-ups além do limite nativo de botões renderizam como poll, e um voto seleciona a opção (#1616)',
  'wininst': '**O Windows é uma plataforma de primeira classe (v0.5.3):** shell handling consciente da plataforma, leitura BOM/UTF-16 e fallbacks de tilde/cwd incluídos, e a CI compila Windows em cada PR (#627).',
  'cli': 'Gestão de memória (list, get, stats, prune)',
 },
 'es': {
  'tests': '**9,032 tests** en providers, herramientas, channels, TUI, self-healing y automatización de browser.',
  'epi': '**Memoria epistémica (v0.5.3)** — los hechos de MEMORY.md llevan confidence y decay (#1643), las secciones frías se archivan solas y `opencrabs memory prune` muestra qué quedó frío antes de tocar nada (#1657)',
  'theme': '**Motor de temas terminado (v0.5.3)** — cada widget del TUI resuelve sus colores a través del tema, con un guard de suite contra nuevos sitios de ANSI crudo, y los temas pueden declarar un canvas background (#1634)',
  'win': '**Windows de primera clase (v0.5.3)** — manejo de shell por plataforma, lectura BOM/UTF-16, fallbacks de tilde y cwd, con CI compilando Windows y aplicando gate a los PRs (#627)',
  'cron': '**Cron con triggers (v0.5.3)** — un job corre un pre-flight check y solo ejecuta cuando tu condición se cumple (output no vacío, exit code, regex), los runs omitidos cuestan 0 tokens y un trigger disparado puede despachar un goal a una session (#233)',
  'polls': '**Encuestas de sugerencias en WhatsApp (v0.5.3)** — los conjuntos de follow-ups que superan el límite nativo de botones se renderizan como encuesta, y un voto selecciona la opción (#1616)',
  'wininst': '**Windows es una plataforma de primera clase (v0.5.3):** manejo de shell consciente de la plataforma, lectura BOM/UTF-16 y fallbacks de tilde/cwd integrados, y CI compila Windows en cada PR (#627).',
  'cli': 'Gestión de memoria (list, get, stats, prune)',
 },
 'fr': {
  'tests': "**9,032 tests** sur les providers, les outils, les channels, le TUI, le self-healing et l'automatisation de browser.",
  'epi': "**Mémoire épistémique (v0.5.3)** — les faits de MEMORY.md portent une confidence et un decay (#1643), les sections froides s'archivent d'elles-mêmes et `opencrabs memory prune` montre ce qui est devenu froid avant d'y toucher (#1657)",
  'theme': "**Moteur de thèmes achevé (v0.5.3)** — chaque widget du TUI résout ses couleurs via le thème, avec un guard de suite contre tout nouveau site ANSI brut, et les thèmes peuvent déclarer un canvas background (#1634)",
  'win': "**Windows natif (v0.5.3)** — gestion du shell par plateforme, lecture BOM/UTF-16, fallbacks tilde et cwd, avec une CI qui compile Windows et gate les PR (#627)",
  'cron': "**Cron déclenché sur condition (v0.5.3)** — un job exécute un pre-flight check et ne s'exécute que si votre condition est remplie (output non vide, exit code, regex), les runs ignorés coûtent 0 token et un trigger déclenché peut dispatcher un goal vers une session (#233)",
  'polls': "**Sondages de suggestions WhatsApp (v0.5.3)** — les ensembles de follow-ups au-delà de la limite native de boutons s'affichent en sondage, et un vote sélectionne l'option (#1616)",
  'wininst': "**Windows est une plateforme native (v0.5.3) :** gestion du shell consciente de la plateforme, lecture BOM/UTF-16 et fallbacks tilde/cwd intégrés, et la CI compile Windows à chaque PR (#627).",
  'cli': 'Gestion de la mémoire (list, get, stats, prune)',
 },
 'ru': {
  'tests': '**9,032 теста** по providers, инструментам, channels, TUI, self-healing и браузерной автоматизации.',
  'epi': '**Эпистемическая память (v0.5.3)** — факты в MEMORY.md несут confidence и decay (#1643), холодные секции архивируются сами, а `opencrabs memory prune` показывает, что остыло, прежде чем что-то менять (#1657)',
  'theme': '**Завершённый движок тем (v0.5.3)** — каждый виджет TUI берёт цвета из темы, suite-guard срабатывает на любой новый сырой ANSI-сайт, и темы могут объявлять canvas background (#1634)',
  'win': '**Windows как первая платформа (v0.5.3)** — платформенная обработка shell, чтение BOM/UTF-16, fallback для tilde и cwd, CI собирает Windows и блокирует PR (#627)',
  'cron': '**Cron с триггерами (v0.5.3)** — job выполняет pre-flight проверку и запускается только при выполнении вашего условия (непустой output, exit code, regex), пропущенные runs стоят 0 токенов, а сработавший trigger может отправить goal в session (#233)',
  'polls': '**Опросы предложений в WhatsApp (v0.5.3)** — наборы follow-up, не влезающие в лимит нативных кнопок, рендерятся опросом, и голос выбирает опцию (#1616)',
  'wininst': '**Windows — платформа первого класса (v0.5.3):** платформенная обработка shell, чтение BOM/UTF-16 и fallback tilde/cwd встроены, CI собирает Windows на каждый PR (#627).',
  'cli': 'Управление памятью (list, get, stats, prune)',
 },
 'id': {
  'tests': '**9,032 test** untuk providers, tools, channels, TUI, self-healing, dan otomasi browser.',
  'epi': '**Memori epistemik (v0.5.3)** — fakta di MEMORY.md membawa confidence dan decay (#1643), section yang dingin mengarsipkan diri sendiri, dan `opencrabs memory prune` menampilkan apa yang sudah dingin sebelum menyentuhnya (#1657)',
  'theme': '**Mesin tema selesai (v0.5.3)** — setiap widget TUI menyelesaikan warnanya melalui tema, dengan suite guard terhadap situs ANSI mentah baru, dan tema bisa mendeklarasikan canvas background (#1634)',
  'win': '**Windows kelas satu (v0.5.3)** — penanganan shell per-platform, pembacaan file BOM/UTF-16, fallback tilde dan cwd, dengan CI membangun Windows dan meng-gate PR (#627)',
  'cron': '**Cron dengan trigger (v0.5.3)** — job menjalankan pre-flight check dan hanya mengeksekusi saat kondisi Anda terpenuhi (output tidak kosong, exit code, regex), run yang dilewati biayanya 0 token, dan trigger yang terpicu bisa mengirim goal ke sebuah session (#233)',
  'polls': '**Poll saran WhatsApp (v0.5.3)** — kumpulan follow-up yang melebihi batas tombol native dirender sebagai poll, dan satu suara memilih opsinya (#1616)',
  'wininst': '**Windows adalah platform kelas satu (v0.5.3):** penanganan shell yang sadar platform, pembacaan BOM/UTF-16 dan fallback tilde/cwd bawaan, dan CI membangun Windows di setiap PR (#627).',
  'cli': 'Manajemen memori (list, get, stats, prune)',
 },
}

ORDER = ['tests', 'epi', 'theme', 'win', 'cron', 'polls', 'wininst', 'cli']

for loc in T:
    ids = json.load(open(f'po/{loc}_ids_release.json'))
    key_to_kind = {}
    for msgid in ids:
        for i, k in enumerate(DELTA_KEYS):
            if msgid.startswith(k) or msgid == k:
                key_to_kind[msgid] = ORDER[i]
    out = {msgid: T[loc][key_to_kind[msgid]] for msgid in key_to_kind}
    missing = [k for k in ORDER if k not in key_to_kind.values()]
    assert not missing, f'{loc} missing kinds: {missing}'
    with open(f'po/tr_{loc}_release.json', 'w', encoding='utf-8') as f:
        json.dump(out, f, ensure_ascii=False, indent=1)
    print(f'{loc}: {len(out)} translations written')
