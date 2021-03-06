# Hack the planet!!

## Install using cargo
```
cargo install panbuild
```

## Building from sources

### Dependencies
* libssl-dev
* pkg-config

### Building with Cargo
```
git clone git@github.com:louib/panbuild.git
cd panbuild/
cargo install --path .
```

You might need to adjust your `PATH` variable to find the binary:
```
export PATH="$PATH:~/.cargo/bin/"
```

### Building with flatpak
```
git clone git@github.com:louib/panbuild.git
cd panbuild/
flatpak-builder --force-clean build build-aux/net.louib.panbuild.yml
```

## Pull Requests
Make sure to run `rustfmt` on your code before commiting:
```
rustup component add rustfmt
```

## Logging
You can use the `PB_LOG_LEVEL` to tune the logging level, for example:
```
PB_LOG_LEVEL=debug panbuild lint Cargo.toml
```
