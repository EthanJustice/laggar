# laggar

site-to-markdown converter

![Build](https://github.com/EthanJustice/laggar/workflows/Rust/badge.svg)

## Roadmap

+ Fix CI not caching dependencies
+ Custom output file paths
+ Settings (content folder name, etc.)
+ URL normalization

## Build

To build, download the repository, then navigate to src/laggar and run `cargo build --release`.

## Usage

1. Navigate to the folder where you've installed the `laggar.exe` binary ([Build instructions](#build))
2. Run `laggar {command}` (see [Commands](#commands))

**Note: the default file name for a root domain (e.g. `https://github.com/`) is /content/{root}/ROOT.md**

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
