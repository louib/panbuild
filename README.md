# [panbuild](https://github.com/louib/panbuild)
![Functional tests status](https://github.com/louib/panbuild/workflows/tests/badge.svg)
![Flatpak-build CI job status](https://github.com/louib/panbuild/workflows/flatpak-build/badge.svg)
![Flatpak-install CI job status](https://github.com/louib/panbuild/workflows/flatpak-install/badge.svg)
[![Crates.io version](https://img.shields.io/crates/v/panbuild?style=flat-square)](https://crates.io/crates/panbuild)
[![License file](https://img.shields.io/github/license/louib/panbuild)](https://github.com/louib/panbuild/blob/master/LICENSE)
<!-- uncomment when there is a release available -->
<!-- [![GitHub release](https://img.shields.io/github/release/louib/panbuild)](https://github.com/louib/panbuild/releases/) -->

The universal builder.

> **This repo is a work-in-progress and is not ready for general use.
  The command-line options, command names and file formats might change
  at any time until the project reaches version 1.0.0.**

Panbuild lets you manage build workspaces when developing desktop applications.
Workspaces are sandboxed and dependencies are installed on a per-project basis.
Manifests from the following packaging systems are supported:
* flatpak;
* snap;
* debian packages (via debian `control` files);

## Install

See [HACKING.md](./HACKING.md)

## License
BSD-3
