# DevSlides

> **DevSlides is a fork of [OpenSlides](https://github.com/codewiththiha/OpenSlides) by [codewiththiha](https://github.com/codewiththiha).**
> This fork is maintained by [alyohara](https://github.com/alyohara).

**Beautiful code presentations for content creators, educators, and developers.**

DevSlides (a fork of OpenSlides) is a free, open-source, offline desktop app for turning code into polished slides with smooth, step-by-step transitions. It is a direct alternative to [codeslides.app](https://codeslides.app): create expressive code decks, keep your work on your own machine, and present without a subscription or internet connection.

---

### Slides

![slides demo](https://www.image2url.com/r2/default/gifs/1784959147106-cee92309-e79d-42f0-8dc0-1edf29556368.gif)

Turn source code into presentation-ready slides. Move between code states with smooth Magic Move transitions, and build a visual story around your code instead of showing a static editor.

### Highlights

![highlight demo](https://www.image2url.com/r2/default/gifs/1784959026570-b96f1a88-2500-464b-b34c-b551d382aab6.gif)

Reveal an idea line by line with stepped highlights. Control emphasis per step: dim amount, size-up scale, and custom transition timings — so you can guide viewers through a function, refactor, algorithm, or feature at a natural pace.

### Themes

![themes demo](https://www.image2url.com/r2/default/gifs/1784959215303-431de853-3e09-4b5f-8ebd-af19973168b4.gif)

Choose light or dark presentation themes to match your style and recording setup.

### Images (new in this fork)

Add images directly to your slides:

- **Background images** — fill the entire stage behind the code.
- **Element images** — freely position, resize, and stack layers on top of the code.
- **Image-only slides** — create slides with just an image (no code).
- **Paste from clipboard** — Ctrl/Cmd+V an image straight into a slide.
- **Native file picker** — choose any PNG, JPEG, GIF, WebP, BMP or AVIF file.

Images are embedded as base64 data URLs inside the project so slides stay fully portable — export, import, and share without worrying about external assets.

<p align="center">
  <img
    src="https://www.image2url.com/r2/default/images/1789139551529-19d70350-5be8-4aa4-9cde-56cb064ab5fb.gif"
    alt="add image demo"
    width="720"
  />
</p>

---

## Made for explaining code beautifully

Whether you are recording a tutorial, teaching a class, streaming a live build, giving a conference talk, or sharing a technical demo, DevSlides helps you focus attention on the exact part of the code that matters.

- Keep projects private, local, and ready to present anywhere.
- Search across slides, use thumbnails and hover previews, and navigate by keyboard.
- Present in full screen with optional autoplay and per-slide timing.
- Create, organize, rename, duplicate, import, and export slide projects.
- Arrange slides in stacks and reorder them with drag and drop.
- Add and arrange images on any slide.

## How it works

Create a project, add your code to slides, then select the lines you want to explain. DevSlides turns those selections into presentation steps, so you can guide viewers through a function, refactor, algorithm, or feature at a natural pace. When it is time to present, move through slides and highlight steps with the keyboard, use full-screen mode, or let the deck advance automatically.

Everything stays on your computer. That makes DevSlides useful for private client work, offline classrooms, live events, and any workflow where you want your code and slides under your control.

## Download

Prebuilt installers for macOS, Windows, and Linux are available from the [DevSlides Releases](https://github.com/alyohara/DevSlides/releases) page.

### Platform packages

- **macOS:** separate Apple Silicon (`aarch64`) and Intel (`x64`) builds
- **Windows:** x64 and ARM64 builds
- **Linux:** `.deb`, `.rpm`, and AppImage

### macOS installation

DevSlides macOS builds are ad-hoc signed. That is enough for Apple Silicon packages to launch after download; a paid Apple Developer certificate is still required for full notarization later.

1. Open the `.dmg` and drag **DevSlides** into **Applications**.
2. First launch: right-click the app → **Open**, then confirm.
3. If macOS still says the app is damaged, clear the download quarantine flag:

```bash
xattr -dr com.apple.quarantine /Applications/DevSlides.app
```

Then open the app again from Applications.

### Linux installation

DevSlides release builds include Linux packages in `.deb`, `.rpm`, and AppImage formats.

- **Debian / Ubuntu / Linux Mint / Pop!\_OS**

Install the required runtime libraries first, then install the package:

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-0 libgtk-3-0 libscrypt0 libayatana-appindicator3-1 librsvg2-common
sudo apt install ./DevSlides_<version>_amd64.deb
```

- **Fedora / RHEL / Rocky / AlmaLinux / other DNF-based distributions**

Install the required runtime libraries first, then install the package:

```bash
sudo dnf check-update
sudo dnf install -y webkit2gtk4.1 gtk3 libappindicator-gtk3 librsvg2
sudo dnf install -y ./DevSlides-<version>-1.x86_64.rpm
```

- **AppImage**

Needs FUSE. Without it: `./DevSlides_*.AppImage --appimage-extract-and-run`. On Wayland with rendering glitches, try `WEBKIT_DISABLE_DMABUF_RENDERER=1`. Otherwise the `.deb` / `.rpm` packages link against the system GTK stack and tend to be smoother.

## Tech stack

- **Desktop app:** Tauri 2 and Rust
- **Interface:** Svelte 5 (runes), TypeScript, Vite 7, and Tailwind CSS 4
- **Component toolkit:** bits-ui, paneforge, and @lucide/svelte
- **Data and flow:** @tanstack/svelte-query, svelte-spa-router, and persistent rune stores
- **Motion and DnD:** Svelte transitions/springs, svelte-dnd-action, and custom pointer drag-and-drop
- **Code rendering:** Shiki and shiki-magic-move
- **Local storage:** SQLite (via sqlx, Rust side)
- **Image handling:** arboard (clipboard), png/base64 encoding, native file dialog

## Run from source

### Requirements

- Bun or newer
- Rust stable toolchain and Cargo
- Platform requirements for Tauri 2

```bash
bun install
bun run tauri dev
```

To run the interface in a browser:

```bash
bun run dev
```

### Development checks

The same gates CI enforces on every push and pull request:

```bash
bun run check              # svelte-check: 0 errors, 0 warnings
bun run lint               # eslint
bun run format:check       # prettier (svelte + tailwind plugins)
bun run build              # production bundle
```

## Credits

DevSlides is a fork of [codewiththiha/OpenSlides](https://github.com/codewiththiha/OpenSlides) by [CodeWithThiha](https://github.com/codewiththiha). The image feature and related enhancements were added by [alyohara](https://github.com/alyohara).

## License

MIT — see [LICENSE](LICENSE).
