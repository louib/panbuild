# Hack the planet!!

Make sure to run `rustfmt` on your code before commiting:
```
rustup component add rustfmt
```

## Building the flatpak
```
flatpak-builder --force-clean build build-aux/net.louib.panbuild.yml
```
