# Aidoku Gallery Sources

Unofficial Aidoku sources for photo-gallery websites. Each gallery is represented as a
completed manga with one webtoon-style chapter.

## Sources

| Source | ID | Language | Website |
| --- | --- | --- | --- |
| Buondua | `multi.buondua` | All | [buondua.com](https://buondua.com/) |
| 4KHD | `multi.4khd` | All | [4khd.com](https://www.4khd.com/) |
| Xiutaku | `zh.xiutaku` | Chinese | [xiutaku.com](https://xiutaku.com/) |

Both sources support browsing, search, list pagination, gallery metadata, multi-page
image galleries, and deep links.

## Layout

Each Aidoku source is a self-contained project under `sources/`:

```text
sources/
├── multi.buondua/
│   ├── Cargo.toml
│   ├── res/
│   └── src/
└── zh.xiutaku/
    ├── Cargo.toml
    ├── res/
    └── src/
```

## Build

The build requires Rust through `rustup` and GNU Make. From WSL, run:

```sh
cd /mnt/d/Git/buondua
make
```

The Makefile installs the `wasm32-unknown-unknown` target and Aidoku CLI when needed,
checks formatting and Clippy, and creates one package per source:

```text
release/multi.buondua.aix
release/zh.xiutaku.aix
```

Other available commands:

```sh
make check    # check formatting and run Clippy
make package  # build all source packages
make rebuild  # clean and rebuild all packages
make clean    # remove Cargo output and generated packages
```

## Source list

On pushes to the default branch, the GitHub Actions workflow builds every source,
generates the Aidoku source list, and replaces the contents of the `gh-pages` branch.
The installable source-list URL is:

```text
https://<owner>.github.io/<repository>/index.min.json
```

To enable it, open **Settings → Pages** and select:

- Source: **Deploy from a branch**
- Branch: `gh-pages`
- Folder: `/ (root)`

Add the resulting `index.min.json` URL from Aidoku's source-list settings. The published
branch also contains versioned icons and `.aix` downloads referenced by the JSON index.

## Content notice

These sources contain adult material and are marked NSFW.
