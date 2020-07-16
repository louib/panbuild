# See https://docs.python.org/3/distutils/setupscript.html
# For documentation on the setup.py script.
import sys
import os.path
import platform
import warnings

from distutils import log
from distutils.core import setup, Command
from distutils.core import Distribution as _Distribution
from distutils.command.build_ext import build_ext as _build_ext
from distutils.command.bdist_rpm import bdist_rpm as _bdist_rpm
from distutils.errors import DistutilsError,\
    CompileError,\
    LinkError,\
    DistutilsPlatformError


# The name is 2flatpak even though the directory layer does not allow a number
# as the first character
NAME = '2flatpak'
VERSION = '0.0.1'
DESCRIPTION = """
flatpak is a repository of scripts to generate flatpak manifests from other build systems.
"""
LONG_DESCRIPTION = """
"""
AUTHOR = "louib"
AUTHOR_EMAIL = 'code@louib.net'
LICENSE = "BSD-3"
PLATFORMS = "Any"
KEYWORDS = [
    "flatpak",
    "snap",
    "package-manager",
    "package",
    "packages",
    "manifest",
    "snapcraft",
]
INSTALL_REQUIRES = [
    'PyYaml',
]
URL = "https://github.com/louib/2flatpak"
DOWNLOAD_URL = "https://github.com/louib/2flatpak/releases/"

# See https://pypi.org/classifiers/ for the list of classifiers accepted
# by PyPi
CLASSIFIERS = [
    "License :: OSI Approved :: BSD License",
    "Natural Language :: English",
    "Operating System :: POSIX :: Linux ",
    "Topic :: Software Development :: Libraries :: Python Modules",
]


if 'setuptools.extension' in sys.modules:
    _Extension = sys.modules['setuptools.extension']._Extension
    sys.modules['distutils.core'].Extension = _Extension
    sys.modules['distutils.extension'].Extension = _Extension
    sys.modules['distutils.command.build_ext'].Extension = _Extension

with_cython = False
if 'sdist' in sys.argv:
    # we need cython here
    with_cython = True
try:
    from Cython.Distutils.extension import Extension as _Extension
    from Cython.Distutils import build_ext as _build_ext  # noqa: F811
    with_cython = True
except ImportError:
    if with_cython:
        raise

try:
    from wheel.bdist_wheel import bdist_wheel
except ImportError:
    bdist_wheel = None


# on Windows, disable wheel generation warning noise
windows_ignore_warnings = [
    "Unknown distribution option: 'python_requires'",
    "Config variable 'Py_DEBUG' is unset",
    "Config variable 'WITH_PYMALLOC' is unset",
    "Config variable 'Py_UNICODE_SIZE' is unset",
    "Cython directive 'language_level' not set"
]

if platform.system() == 'Windows':
    for w in windows_ignore_warnings:
        warnings.filterwarnings('ignore', w)


class Distribution(_Distribution):

    def __init__(self, attrs=None):
        _Distribution.__init__(self, attrs)
        if not self.ext_modules:
            return
        for idx in range(len(self.ext_modules)-1, -1, -1):
            ext = self.ext_modules[idx]
            if not isinstance(ext, Extension):
                continue
            setattr(self, ext.attr_name, None)
            self.global_options = [
                    (ext.option_name, None,
                        "include %s (default if %s is available)"
                        % (ext.feature_description, ext.feature_name)),
                    (ext.neg_option_name, None,
                        "exclude %s" % ext.feature_description),
            ] + self.global_options
            self.negative_opt = self.negative_opt.copy()
            self.negative_opt[ext.neg_option_name] = ext.option_name

    def has_ext_modules(self):
        if not self.ext_modules:
            return False
        for ext in self.ext_modules:
            with_ext = self.ext_status(ext)
            if with_ext is None or with_ext:
                return True
        return False

    def ext_status(self, ext):
        implementation = platform.python_implementation()
        if implementation != 'CPython':
            return False
        if isinstance(ext, Extension):
            with_ext = getattr(self, ext.attr_name)
            return with_ext
        else:
            return True


class Extension(_Extension):

    def __init__(
        self, name, sources, feature_name, feature_description, feature_check, **kwds
    ):
        if not with_cython:
            for filename in sources[:]:
                base, ext = os.path.splitext(filename)
                if ext == '.pyx':
                    sources.remove(filename)
                    sources.append('%s.c' % base)
        _Extension.__init__(self, name, sources, **kwds)
        self.feature_name = feature_name
        self.feature_description = feature_description
        self.feature_check = feature_check
        self.attr_name = 'with_' + feature_name.replace('-', '_')
        self.option_name = 'with-' + feature_name
        self.neg_option_name = 'without-' + feature_name


class build_ext(_build_ext):

    def run(self):
        optional = True
        disabled = True
        for ext in self.extensions:
            with_ext = self.distribution.ext_status(ext)
            if with_ext is None:
                disabled = False
            elif with_ext:
                optional = False
                disabled = False
                break
        if disabled:
            return
        try:
            _build_ext.run(self)
        except DistutilsPlatformError:
            exc = sys.exc_info()[1]
            if optional:
                log.warn(str(exc))
                log.warn("skipping build_ext")
            else:
                raise

    def get_source_files(self):
        self.check_extensions_list(self.extensions)
        filenames = []
        for ext in self.extensions:
            if with_cython:
                self.cython_sources(ext.sources, ext)
            for filename in ext.sources:
                filenames.append(filename)
                base = os.path.splitext(filename)[0]
                for ext in ['c', 'h', 'pyx', 'pxd']:
                    filename = '%s.%s' % (base, ext)
                    if filename not in filenames and os.path.isfile(filename):
                        filenames.append(filename)
        return filenames

    def get_outputs(self):
        self.check_extensions_list(self.extensions)
        outputs = []
        for ext in self.extensions:
            fullname = self.get_ext_fullname(ext.name)
            filename = os.path.join(self.build_lib,
                                    self.get_ext_filename(fullname))
            if os.path.isfile(filename):
                outputs.append(filename)
        return outputs

    def build_extensions(self):
        self.check_extensions_list(self.extensions)
        for ext in self.extensions:
            with_ext = self.distribution.ext_status(ext)
            if with_ext is not None and not with_ext:
                continue
            if with_cython:
                ext.sources = self.cython_sources(ext.sources, ext)
            try:
                self.build_extension(ext)
            except (CompileError, LinkError):
                if with_ext is not None:
                    raise
                log.warn("Error compiling module, falling back to pure Python")


class bdist_rpm(_bdist_rpm):

    def _make_spec_file(self):
        argv0 = sys.argv[0]
        features = []
        for ext in self.distribution.ext_modules:
            if not isinstance(ext, Extension):
                continue
            with_ext = getattr(self.distribution, ext.attr_name)
            if with_ext is None:
                continue
            if with_ext:
                features.append('--'+ext.option_name)
            else:
                features.append('--'+ext.neg_option_name)
        sys.argv[0] = ' '.join([argv0]+features)
        spec_file = _bdist_rpm._make_spec_file(self)
        sys.argv[0] = argv0
        return spec_file


class test(Command):

    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        build_cmd = self.get_finalized_command('build')
        build_cmd.run()
        sys.path.insert(0, build_cmd.build_lib)
        if sys.version_info[0] < 3:
            sys.path.insert(0, 'tests/lib')
        else:
            sys.path.insert(0, 'tests/lib3')
        import test_all
        if not test_all.main([]):
            raise DistutilsError("Tests failed")


cmdclass = {
    'build_ext': build_ext,
    'bdist_rpm': bdist_rpm,
    'test': test,
}
if bdist_wheel:
    cmdclass['bdist_wheel'] = bdist_wheel


if __name__ == '__main__':
    setup(
        name=NAME,
        version=VERSION,
        description=DESCRIPTION,
        long_description=LONG_DESCRIPTION,
        author=AUTHOR,
        author_email=AUTHOR_EMAIL,
        license=LICENSE,
        platforms=PLATFORMS,
        url=URL,
        download_url=DOWNLOAD_URL,
        classifiers=CLASSIFIERS,
        keywords=KEYWORDS,
        install_requires=INSTALL_REQUIRES,

        # This is the only way I see to install a manpage from the setup.py
        # script.
        # See https://github.com/pypa/packaging-problems/issues/72 for
        # details
        # FIXME I'm not even sure this is working.
        data_files=[('man/man1', ['manpages/2flatpak.1'])],

        # FIXME we should only have one script here: 2flatpak
        scripts=[
            'scripts/yml2json',
            'toflatpak/toflatpak',
        ],
        # package_dir={'': {2: 'toflatpak', 3: 'toflatpak'}[sys.version_info[0]]},
        packages=['toflatpak'],

        distclass=Distribution,
        cmdclass=cmdclass,
        python_requires='>=2.7, !=3.0.*, !=3.1.*, !=3.2.*, !=3.3.*, !=3.4.*',
    )
