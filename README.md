# laggar
 site-to-markdown converter

Direct dependencies: [reqwest](https://crates.io/crates/reqwest), [html2md](https://crates.io/crates/html2md), [crossterm](https://crates.io/crates/crossterm), and [clap](https://crates.io/crates/clap).

## Roadmap

+ Continuous integration (platform binaries)
+ HTTP status code handling (404, etc.)
+ ~~Subfolders of `/content/` based on domain (e.g. `/content/name.tld/`), for better legibility (pages are only path from root (e.g. `/content/name.tld/page.md`)~~
+ Clean error logging in TUI, full errors in `/logs/`
+ ~~Timing~~
+ Download logging (maybe timing for each individual piece)
+ Settings (maybe image downloading)

## Build

To build, download the repository, then navigate to src/laggar and run `cargo build --release`.

## Usage

1. Navigate to the folder where you've installed the `laggar.exe` binary ([Build instructions](#build))
2. Run `laggar {command}` (see [Commands](#commands))

### Commands

`-d, --download <URL>` - downloads the specified page

```plaintext
laggar --d https://example.com/
```

`-h, --help` - lists all commands

```plaintext
laggar -h
```

`-V` (note the capital) - lists the version

```plaintext
laggar -V
```
