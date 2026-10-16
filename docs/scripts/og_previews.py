#!/usr/bin/env python3
"""Per-page Open Graph previews for the OpenCrabs docs.

Run from the docs/ directory after `mdbook build` (docs/build-all.sh calls it):

    python3 scripts/og_previews.py

Phase 1 renders every content page to a 1200x630 JPEG with headless Chrome.
Phase 2 rewrites the og:*/twitter:* tags and the canonical link of each page so
a shared link shows that page's own preview, title and description.

Pages that already ship a hand-made card keep it. print.html and 404.html are
skipped. Set SKIP_OG=1 to skip the whole step.
"""

from __future__ import annotations

import html.parser
import re
import socket
import subprocess
import sys
import time
from pathlib import Path

SITE = "https://docs.opencrabs.com"
DEFAULT_IMAGE = f"{SITE}/img/og-docs.png"
CHROME = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
SIPS = "/usr/bin/sips"
PORT = 8765
WIDTH, HEIGHT = 1200, 630
JPEG_QUALITY = "78"
SKIP_NAMES = {"print.html", "404.html"}

DOCS = Path(__file__).resolve().parent.parent
BOOK = DOCS / "book"
OG_DIR = BOOK / "og"
PNG_TMP = Path("/tmp/og-preview-render.png")
PROFILE = Path("/tmp/og-previews-chrome-profile")

CANONICAL_RE = re.compile(r'(<link rel="canonical" href=")[^"]*(")')


class FirstParagraph(html.parser.HTMLParser):
    """Collect the text of the first paragraph inside <main>."""

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self._in_main = False
        self._in_p = False
        self._done = False
        self.parts: list[str] = []

    def handle_starttag(self, tag, attrs):
        if tag == "main":
            self._in_main = True
        elif tag == "p" and self._in_main and not self._done:
            self._in_p = True

    def handle_endtag(self, tag):
        if tag == "p" and self._in_p:
            self._in_p = False
            if self.parts:
                self._done = True
        elif tag == "main":
            self._in_main = False

    def handle_data(self, data):
        if self._in_p and not self._done:
            self.parts.append(data)

    @property
    def text(self) -> str:
        return " ".join(" ".join(self.parts).split())


def shorten(text: str, limit: int = 200) -> str:
    if len(text) <= limit:
        return text
    cut = text[:limit].rsplit(" ", 1)[0].rstrip(",.;: ")
    return f"{cut}..."


def page_url(rel: Path) -> str:
    posix = rel.as_posix()
    if posix == "index.html":
        return f"{SITE}/"
    if posix.endswith("/index.html"):
        return f"{SITE}/{posix[: -len('index.html')]}"
    return f"{SITE}/{posix}"


def preview_rel(rel: Path) -> str:
    return rel.with_suffix(".jpg").as_posix()


def replace_meta(doc: str, attr: str, name: str, value: str) -> tuple[str, int]:
    pattern = re.compile(
        rf'(<meta\s+{attr}="{re.escape(name)}"\s+content=")[^"]*(")'
    )
    return pattern.subn(lambda m: m.group(1) + value + m.group(2), doc, count=1)


def wait_for_server(port: int, timeout: float = 10.0) -> bool:
    deadline = time.time() + timeout
    while time.time() < deadline:
        with socket.socket() as sock:
            sock.settimeout(0.25)
            if sock.connect_ex(("127.0.0.1", port)) == 0:
                return True
        time.sleep(0.1)
    return False


def render(page: Path) -> None:
    subprocess.run(
        [
            CHROME,
            "--headless=new",
            "--disable-gpu",
            "--no-first-run",
            "--hide-scrollbars",
            f"--user-data-dir={PROFILE}",
            f"--window-size={WIDTH},{HEIGHT}",
            f"--screenshot={PNG_TMP}",
            f"http://127.0.0.1:{PORT}/{page.relative_to(BOOK).as_posix()}",
        ],
        check=True,
        capture_output=True,
        timeout=60,
    )


def to_jpeg(jpg: Path) -> None:
    jpg.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        [
            SIPS,
            "-s",
            "format",
            "jpeg",
            "-s",
            "formatOptions",
            JPEG_QUALITY,
            str(PNG_TMP),
            "--out",
            str(jpg),
        ],
        check=True,
        capture_output=True,
    )


def main() -> int:
    if not BOOK.is_dir():
        print(f"book directory not found: {BOOK}", file=sys.stderr)
        return 1
    if not Path(CHROME).exists():
        print(f"headless Chrome not found: {CHROME}", file=sys.stderr)
        return 1

    pages = sorted(
        page
        for page in BOOK.rglob("*.html")
        if OG_DIR not in page.parents and page.name not in SKIP_NAMES
    )
    if not pages:
        print("no pages found", file=sys.stderr)
        return 1

    PROFILE.mkdir(parents=True, exist_ok=True)
    server = subprocess.Popen(
        [
            sys.executable,
            "-m",
            "http.server",
            str(PORT),
            "--bind",
            "127.0.0.1",
            "--directory",
            str(BOOK),
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

    rendered = 0
    kept = 0
    failed = 0
    failures: list[str] = []

    try:
        if not wait_for_server(PORT):
            print("local preview server did not start", file=sys.stderr)
            return 1

        for index, page in enumerate(pages, 1):
            rel = page.relative_to(BOOK)
            doc = page.read_text(encoding="utf-8")
            current = re.search(r'<meta property="og:image" content="([^"]*)"', doc)
            if not current:
                failed += 1
                failures.append(f"{rel}: no og:image tag")
                continue
            if current.group(1) != DEFAULT_IMAGE:
                kept += 1  # hand-made card, leave this page alone
                continue

            url = page_url(rel)
            try:
                render(page)
                to_jpeg(OG_DIR / rel.with_suffix(".jpg"))
            except Exception as exc:  # noqa: BLE001 - report, keep going
                failed += 1
                failures.append(f"{rel}: render failed: {exc}")
                continue

            title = re.search(r"<title>([^<]*)</title>", doc)
            title_text = title.group(1).strip() if title else "OpenCrabs Documentation"
            finder = FirstParagraph()
            finder.feed(doc)
            description = shorten(finder.text) or "OpenCrabs documentation."
            image_url = f"{SITE}/og/{preview_rel(rel)}"

            new = doc
            for attr, name, value in (
                ("property", "og:title", title_text),
                ("property", "og:description", description),
                ("property", "og:url", url),
                ("property", "og:image", image_url),
                ("property", "og:image:width", str(WIDTH)),
                ("property", "og:image:height", str(HEIGHT)),
                ("name", "twitter:title", title_text),
                ("name", "twitter:description", description),
                ("name", "twitter:image", image_url),
            ):
                new, hits = replace_meta(new, attr, name, value)
                if hits == 0:
                    failures.append(f"{rel}: {name} tag missing")
            new, hits = CANONICAL_RE.subn(
                lambda m: m.group(1) + url + m.group(2), new, count=1
            )
            if hits == 0:
                failures.append(f"{rel}: canonical link missing")
            page.write_text(new, encoding="utf-8")
            rendered += 1

            if index % 25 == 0:
                print(f"  {index}/{len(pages)} pages done", flush=True)
    finally:
        server.terminate()
        try:
            server.wait(timeout=5)
        except subprocess.TimeoutExpired:
            server.kill()
        PNG_TMP.unlink(missing_ok=True)

    print(
        f"previews written: {rendered}, hand-made cards kept: {kept}, failed: {failed}"
    )
    for line in failures[:20]:
        print(f"  ! {line}", file=sys.stderr)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
