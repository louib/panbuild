use std::env;

pub fn get_all() -> Vec<crate::projects::project::Project> {
    let core_projects = self::get_core_projects();

    let projects_dir = env::var("PANBUILD_PROJECTS_DIR").unwrap_or(String::from("")).to_string();
    if projects_dir.is_empty() {
        return core_projects;
    }

    // TODO validate the directory!
    return core_projects;
}

pub fn get_core_projects() -> Vec<crate::projects::project::Project> { vec![
    crate::projects::project::Project {
        id: "linux".to_string(),
        name: "linux".to_string(),
        summary: "The Linux Kernel".to_string(),
        description: "
        ".to_string(),
        homepage: "https://github.com/torvalds/linux".to_string(),
        url: "https://github.com/torvalds/linux.git".to_string(),
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 0,
        is_core: true,
    },
    crate::projects::project::Project {
        id: "flatpak-builder".to_string(),
        name: "flatpak-builder".to_string(),
        summary: "Tool to build flatpaks from source".to_string(),
        description: "
            Flatpak-builder is a tool for building flatpaks from sources.
            See http://flatpak.org/ for more information.
        ".to_string(),
        homepage: "http://flatpak.org/".to_string(),
        url: "".to_string(),
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 0,
        is_core: true,
    },
    crate::projects::project::Project {
        id: "flatpak".to_string(),
        name: "flatpak".to_string(),
        summary: "Linux application sandboxing and distribution framework".to_string(),
        description: "
            Flatpak is a system for building, distributing, and running sandboxed desktop applications on Linux.
        ".to_string(),
        homepage: "http://flatpak.org/".to_string(),
        url: "".to_string(),
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 0,
        is_core: true,
    },
    crate::projects::project::Project {
        id: "gcc".to_string(),
        name: "gcc".to_string(),
        summary: "GCC, the GNU Compiler Collection".to_string(),
        description: "
            The GNU Compiler Collection includes front ends for C, C++, Objective-C, Fortran, Ada, Go,
            and D, as well as libraries for these languages (libstdc++,...). GCC was originally written
            as the compiler for the GNU operating system. The GNU system was developed to be 100% free software,
            free in the sense that it respects the user's freedom.
        ".to_string(),
        homepage: "https://gcc.gnu.org/".to_string(),
        url: "https://gcc.gnu.org/git/gcc.git".to_string(),
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 0,
        is_core: true,
    },
    crate::projects::project::Project {
        id: "phosh".to_string(),
        name: "phosh".to_string(),
        summary: "a trivial wayland shell for prototyping things.".to_string(),
        description: "
            A pure Wayland shell prototype for GNOME on mobile devices.
            For a matching compositor see https://source.puri.sm/Librem5/phoc but others implementing wlr-layer-shell should work as well.
        ".to_string(),
        homepage: "https://source.puri.sm/Librem5/phosh".to_string(),
        url: "https://source.puri.sm/Librem5/phosh.git".to_string(),
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 3,
        is_core: true,
    },
    crate::projects::project::Project {
        id: "phoc".to_string(),
        name: "phoc".to_string(),
        summary: "Wayland compositor for mobile phones like the Librem 5".to_string(),
        description: "
            wlroots based Phone compositor as used on the Librem5.
            Phoc is pronounced like the English word fog.
        ".to_string(),
        homepage: "https://source.puri.sm/Librem5/phoc".to_string(),
        url: "https://source.puri.sm/Librem5/phoc.git".to_string(),
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 3,
        is_core: true,
    },
    crate::projects::project::Project {
        id: "curl".to_string(),
        name: "curl".to_string(),
        summary: "Curl is a command-line tool for transferring data specified with URL syntax.".to_string(),
        description: "
            A command line tool and library for transferring data with URL syntax, supporting HTTP, HTTPS, FTP, FTPS,
            GOPHER, TFTP, SCP, SFTP, SMB, TELNET, DICT, LDAP, LDAPS, MQTT, FILE, IMAP, SMTP, POP3, RTSP and RTMP.
            libcurl offers a myriad of powerful features.
        ".to_string(),
        homepage: "https://github.com/curl/curl.git".to_string(),
        url: "https://github.com/curl/curl".to_string(),
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 1,
        is_core: true,
    },
    crate::projects::project::Project {
        id: "glibc".to_string(),
        name: "glibc".to_string(),
        summary: "The GNU C Library (glibc)".to_string(),
        description: "
            The GNU C Library project provides the core libraries for the GNU system and GNU/Linux systems,
            as well as many other systems that use Linux as the kernel. These libraries provide critical APIs including ISO C11,
            POSIX.1-2008, BSD, OS-specific APIs and more. These APIs include such foundational facilities as open, read, write,
            malloc, printf, getaddrinfo, dlopen, pthread_create, crypt, login, exit and more.
        ".to_string(),
        homepage: "https://www.gnu.org/software/libc/".to_string(),
        url: "git://sourceware.org/git/glibc.git".to_string(),
        maintainers: vec![
            "maintainers <libc-maintainers@gnu.org>".to_string(),
        ],
        keywords: vec![],
        dependencies: vec![],
        layer: 1,
        is_core: true,
    },
    crate::projects::project::Project {
        id: "wget".to_string(),
        name: "wget".to_string(),
        summary: "GNU Wget is a free utility for non-interactive download of files from the Web.".to_string(),
        description: "
            It can follow links in HTML pages and create local versions of remote
            web sites, fully recreating the directory structure of the original
            site.  This is sometimes referred to as \"recursive downloading.\"
            While doing that, Wget respects the Robot Exclusion Standard
            (/robots.txt).  Wget can be instructed to convert the links in
            downloaded HTML files to the local files for offline viewing.

            Recursive downloading also works with FTP, where Wget can retrieve a
            hierarchy of directories and files.

            With both HTTP and FTP, Wget can check whether a remote file has
            changed on the server since the previous run, and only download the
            newer files.

            Wget has been designed for robustness over slow or unstable network
            connections; if a download fails due to a network problem, it will
            keep retrying until the whole file has been retrieved.  If the server
            supports regetting, it will instruct the server to continue the
            download from where it left off.

            If you are behind a firewall that requires the use of a socks style
            gateway, you can get the socks library and compile wget with support
            for socks.

            Most of the features are configurable, either through command-line
            options, or via initialization file .wgetrc.  Wget allows you to
            install a global startup file (/usr/local/etc/wgetrc by default) for
            site settings.

            Wget works under almost all Unix variants in use today and, unlike
            many of its historical predecessors, is written entirely in C, thus
            requiring no additional software, such as Perl.  The external software
            it does work with, such as OpenSSL, is optional.  As Wget uses the GNU
            Autoconf, it is easily built on and ported to new Unix-like systems.
            The installation procedure is described in the INSTALL file.

            As with other GNU software, the latest version of Wget can be found at
            the master GNU archive site ftp.gnu.org, and its mirrors.  Wget
            resides at <ftp://ftp.gnu.org/pub/gnu/wget/>.

            Please report bugs in Wget to <bug-wget@gnu.org>.
        ".to_string(),
        homepage: "http://git.savannah.gnu.org/cgit/wget.git".to_string(),
        url: "https://git.savannah.gnu.org/git/wget.git".to_string(),
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 1,
        is_core: true,
    },
]}
