# -*- coding: utf-8 -*-

# See `man flatpak-manifest` for the flatpak manifest specs.
class FlatpakManifest:
    # A string defining the application id.
    # string
    _app_id = None

    # The branch to use when exporting the application. If this is unset the defaults come from the default-branch option.

    # This key overrides both the default-branch key, and the --default-branch commandline option.
    # Unless you need a very specific branchname (like for a runtime or an extension) it is recommended
    # to use the default-branch key instead, because you can then override the default using
    # --default-branch when building for instance a test build.
    # string
    _branch = ""

    # The default branch to use when exporting the application. Defaults to master.
    # This key can be overridden by the --default-branch commandline option.
    # string
    _default_branch = ""

    # The collection ID of the repository, defaults to being unset. Setting a globally unique collection
    # ID allows the apps in the repository to be shared over
    # peer to peer systems without needing further configuration. If building in an existing repository,
    # the collection ID must match the existing configured collection ID for that repository.
    # string
    _collection_id = ""

    # The name of the runtime that the application uses.
    # string
    _runtime = ""

    # The version of the runtime that the application uses, defaults to master.
    # string
    _runtime_version = ""

    # The name of the development runtime that the application builds with.
    # string
    _sdk = ""

    # Initialize the (otherwise empty) writable /var in the build with a copy of this runtime.
    # string
    _var = ""

    # Use this file as the base metadata file when finishing.
    # string
    _metadata = ""

    # Build a new runtime instead of an application.
    # boolean
    _build_runtime = False

    # Build an extension.
    # boolean
    _build_extension = False

    # base (string)
    # Start with the files from the specified application.
    # This can be used to create applications that extend another application.
    _base = ""

    # base-version (string)
    # Use this specific version of the application specified in base. If unspecified, this uses the value specified in branch
    _base_version = ""

    # array of string
    _base_extensions = []
    # Install these extra extensions from the base application when
    # initializing the application directory.

    # array of strings
    # Inherit these extra extensions points from the base application or sdk when finishing the build.
    _inherit_extensions = []

    # array of strings
    # Inherit these extra extensions points from the base application or sdk when finishing the build, but do not inherit them into the platform.
    _inherit_sdk_extensions = []

    # array of strings
    # Inherit these extra extensions points from the base application or sdk when finishing the build, but do not inherit them into the platform.
    _build_options = []

    # Add these tags to the metadata file.
    # tags (array of strings)

    _modules = []

    def __init__(self, path):
        self.path = path

    def is_valid(self):
        return True
