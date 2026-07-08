# Document Generation

OpenCrabs writes real documents, not just text files. The built-in `generate_document` tool creates XLSX, DOCX, and PDF natively inside the binary (no Python, no LibreOffice, no font installs), and PPTX through `python-pptx` when the host has it. On channels the agent sends the finished file back as a downloadable attachment in the same turn.

## Supported Formats

| Format | Engine | Content | Styling |
|--------|--------|---------|---------|
| **XLSX** | Native Rust (`rust_xlsxwriter`) | Multiple sheets, rows of typed cells; any cell starting with `=` becomes a **live Excel formula** that recalculates when the user edits the file | Colored header row, zebra striping, frozen header, autofilter dropdowns, colored sheet tabs, per-column number formats (`currency`, `percent`, `date`, `integer`, or raw Excel codes) |
| **DOCX** | Native Rust (`docx-rs`) | Headings (real Word styles, navigation pane works), paragraphs, bullet/numbered lists (real numbering), tables, **image blocks** (inline PNG/JPEG with optional caption) | Accent-colored headings, page header/footer on every page, shaded table headers, zebra rows |
| **PDF** | Native Rust (`printpdf` + bundled DejaVu Sans) | Same block model as DOCX; A4 flow with word wrap, page breaks, content-sized table columns with header separators and row rules; **image blocks** (inline PNG/JPEG with aspect-preserving sizing and optional caption); real Unicode text (accents, Cyrillic, arrows, checkmarks) | Brand accent + text colors, H1 underline bar, page header with **logo image** (local PNG/JPEG), footer with exact `Page N of M`, zebra tables |
| **PPTX** | Host `python-pptx` (clear install hint when missing) | Slides with title, bullets, speaker notes | **Brand template**: build slides into an existing `.pptx` so they inherit the company master (logos, fonts, backgrounds); accent-colored titles; per-slide layout choice |

## How It Works

Everything is driven by one structured tool call, so it works with any provider including local models. All styling defaults off; bad colors or a missing logo degrade to the plain look instead of failing the document.

The tool accepts:
- **Format** (xlsx, docx, pdf, pptx)
- **Content blocks** (headings, paragraphs, lists, tables, image blocks)
- **Style configuration** (brand colors, logos, page furniture)

## Image Blocks

PDF and DOCX support embedding PNG and JPEG images inline with optional captions. Images are sized to fit the page while preserving aspect ratio.

## Page Size and Orientation (v0.3.65)

All three document formats support custom page sizes and orientations:

| Format | Options |
|--------|---------|
| **PDF** | `landscape` mode, custom `page_width` and `page_height` (in points) |
| **DOCX** | `orientation` (`portrait` or `landscape`), custom `page_width` and `page_height` (in twips) |
| **PPTX** | `slide_width` and `slide_height` (in EMUs), aspect ratio control |

Default sizes: PDF/DOCX use A4 portrait, PPTX uses 16:9 widescreen.

## Brand Templates

For PPTX, you can point the tool at an existing `.pptx` file to inherit the company master (logos, fonts, backgrounds). Each slide gets a layout choice and accent-colored titles.

## Delivery

On Telegram, WhatsApp, Discord, and Slack, generated documents are sent as downloadable attachments in the same turn the tool was called. No separate "here's your file" message needed.

## Examples

Ask the agent:
- "Create a PDF report with these sales figures"
- "Generate an Excel spreadsheet with formulas for monthly totals"
- "Make a Word document with this content and our company logo"
- "Build a PowerPoint presentation from these bullet points"

The agent handles formatting, styling, and delivery automatically.
