# claude-usage-tracker

### _Portable meters for the Claude limits:_
  **Week — all models** - Overall % of Weekly limit across all models
  
  **Week — model-scoped** - Labelled % with Fable weekly limit currently
  
  **Session (5h)** - 5 hour session window

### _Features_

- Extra-usage credits line
- Fixed dark
- One small window
- 📌 pin for always-on-top
- Refresh on launch + button with an honest "updated N min ago" ticker.
- Dismissible error banners; a rate limit (429) arms a 3-minute
  countdown, then retries automatically.

Reads the OAuth token from `~/.claude/.credentials.json` (fresh on every
refresh, never stored, never shown) and asks the same endpoint Claude
Code's own `/usage` screen uses. Every failure is an explicit state —
the app shows errors, never an assumed metric.


<img width="442" height="331" alt="image" src="https://github.com/user-attachments/assets/a4cbada3-c9e3-44fd-b9aa-efe3fb5c1cf8" />


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

Tag a version and CI does the rest — Windows portable exe + installer
and a universal macOS .dmg land on the GitHub release:

```sh
git tag v0.1.x && git push --tags
```

The Mac .dmg is unsigned: first open is right-click → Open.

## Changelog

- **v0.1.3** (2026-08-17) — rate limits heal themselves: a 429 shows a
  dismissible banner with a 3-minute countdown, then retries on its
  own until the numbers are back. Other HTTP errors now read as
  sentences instead of raw "http-NNN" codes, and every error banner
  gets an × to close it.
- **v0.1.2** (2026-08-15) — weekly meters now sit on top (they're the
  numbers that govern a week), and the progress fills actually draw:
  the CSP had been silently blocking their inline widths since day
  one. Dropped the MSI (setup.exe + portable exe cover Windows), added
  the MIT license, and bundle filenames finally match the tag.
- **v0.1.1** (2026-08-13) — first release that shipped: the three
  meters, pin, credits line, and CI-built artifacts — portable exe +
  installers on Windows, universal .dmg on macOS. Bundles were named
  0.1.0; the app version hadn't been bumped with the tag.
- **v0.1.0** (2026-08-12) — initial tag. The release job couldn't
  upload artifacts (missing write permission), so it never got a
  release page. Superseded by v0.1.1.

## License

[MIT](LICENSE).
