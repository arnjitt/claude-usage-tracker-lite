# claude-usage-tracker

Portable meters for the Claude limits that actually gate a working day:
**session (5h)** · **week — all models** · **week — Fable**, plus the
extra-usage credits line. Fixed dark, one small window, a 📌 pin for
always-on-top, refresh on launch + button with an honest
"updated N min ago" ticker.

Reads the OAuth token from `~/.claude/.credentials.json` (fresh on every
refresh, never stored, never shown) and asks the same endpoint Claude
Code's own `/usage` screen uses. Every failure is an explicit state —
the app shows errors, never an assumed metric.


<img width="439" height="330" alt="image" src="https://github.com/user-attachments/assets/b19025ad-cf55-4502-a8f5-0fc3618a199b" />

## Requirements

A machine where **Claude Code is signed in** (that's where the
credentials file comes from). Windows 11 ships WebView2 already; macOS
uses the system webview.

## Run / build

```sh
bun install
bun run dev      # live window against your real usage
bun run build    # standalone exe / .app + installer bundles
```

Toolchain: Bun + Rust (rustup) + the platform's C toolchain
(MSVC Build Tools on Windows, Xcode CLT on macOS).

## Distribution

Tag a version and CI does the rest — Windows portable exe + installers
and a universal macOS .dmg land on the GitHub release:

```sh
git tag v0.1.x && git push --tags
```

The Mac .dmg is unsigned: first open is right-click → Open.
![Uploading image.png…]()
