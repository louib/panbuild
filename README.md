# [panbuild](https://github.com/louib/panbuild)
![Functional tests status](https://github.com/louib/panbuild/workflows/tests/badge.svg)
![Flatpak-build CI job status](https://github.com/louib/panbuild/workflows/flatpak-build/badge.svg)
![Flatpak-install CI job status](https://github.com/louib/panbuild/workflows/flatpak-install/badge.svg)
[![GitHub release](https://img.shields.io/github/license/louib/panbuild)](https://github.com/louib/panbuild/blob/master/LICENSE)

The universal build manifest converter.

The supported packaging systems are:
* flatpak;
* snap;
* debian packages (via debian `control` files);

Panbuild aims to make Unix system package managers inter-operable, whether they are distribution
agnostic (snap, flatpak) or distribution based (deb, rpm, pacman, Homebrew). The executable is
portable and comes with an internal database of projects that can be installed through
various build systems.

## Install

```
# Make sure you have flathub installed.
# This is not working yet.
flatpak install net.louib.panbuild
```

## Building the flatpak
```
flatpak-builder --force-clean build net.louib.panbuild.yml
```

## Other related tools
* https://github.com/flatpak/flatpak-builder-tools

## License

BSD-3
