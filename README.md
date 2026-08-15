# claude-usage-tracker

Portable meters for the Claude limits that actually gate a working day:
**week — all models** · **week — model-scoped** (labelled with whatever
model the API reports) · **session (5h)**, plus the
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

Tag a version and CI does the rest — Windows portable exe + installer
and a universal macOS .dmg land on the GitHub release:

```sh
git tag v0.1.x && git push --tags
```

The Mac .dmg is unsigned: first open is right-click → Open.

## Changelog

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
