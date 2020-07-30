# Hack the planet!!

## Building from sources

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
