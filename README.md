# Buondua Aidoku sources

An unofficial Aidoku source for [buondua.com](https://buondua.com/). Each photo gallery is
represented as a completed manga with one webtoon-style chapter.

## Layout

Each Aidoku source is a self-contained project under `sources/`:

```text
sources/
└── multi.buondua/
    ├── Cargo.toml
    ├── res/
    └── src/
```

## Build

From WSL, run `make`. The Makefile installs missing Aidoku build prerequisites and creates:

```text
sources/multi.buondua/package.aix
```

This source contains adult material and is marked NSFW.
