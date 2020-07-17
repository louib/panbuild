# -*- coding: utf-8 -*-

# See `man flatpak-manifest` for the flatpak manifest specs.
class FlatpakManifest:
    def __init__(self, path):
        self.path = path

    def is_valid(self):
        return True
