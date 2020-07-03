# laggar
 site-to-markdown converter

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
