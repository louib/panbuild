#!/usr/bin/env sh

if ! command -v flatpak; then
    exit 1
fi

if ! command -v flatpak-builder; then
    exit 1
fi

# TODO verify also the versions?

flatpak remote-add --user --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install --user -y flathub org.gnome.Platform/x86_64/3.36 org.gnome.Sdk/x86_64/3.36
flatpak-builder --force-clean --user build net.louib.panbuild.json
flatpak-builder --user --run build net.louib.panbuild.json python3 setup.py sdist

echo "📦 Published the new flatpak package."

# TODO publish on flathub.
