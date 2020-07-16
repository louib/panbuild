# 2flatpak
![Functional tests status](https://github.com/louib/2flatpak/workflows/tests/badge.svg)
![PEP8 CI job status](https://github.com/louib/2flatpak/workflows/pep8/badge.svg)
![Flatpak-build CI job status](https://github.com/louib/2flatpak/workflows/flatpak-build/badge.svg)
![Flatpak-install CI job status](https://github.com/louib/2flatpak/workflows/flatpak-install/badge.svg)
[![GitHub release](https://img.shields.io/github/license/louib/2flatpak)](https://github.com/louib/2flatpak/blob/master/LICENSE)

[2flatpak](https://github.com/louib/2flatpak) generates flatpak manifests from other packaging systems.

The currently supported packaging systems are:
* snap;
* debian packages (via debian `control` files);
* pip (via requirements.txt and pyproject.toml).

## Install

```
# Make sure you have flathub installed.
# This is not working yet.
flatpak install net.louib.2flatpak
```

## Building the flatpak
```
flatpak-builder --force-clean build net.louib.2flatpak.yml
```

## Other related tools
* https://github.com/flatpak/flatpak-builder-tools

## License

BSD-3
