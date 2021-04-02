// See https://wiki.archlinux.org/index.php/PKGBUILD
// And
// https://jlk.fjfi.cvut.cz/arch/manpages/man/BUILDINFO.5
// And
// https://wiki.archlinux.org/index.php/Arch_package_guidelines
// for details on the pkgbuild format.
// TODO use the list of all arch packages at
// https://www.archlinux.org/packages/
// and all the packages in a git access are here
// https://github.com/archlinux/svntogit-packages

pub struct ArchManifest {}

pub fn file_path_matches(path: &str) -> bool {
    if path.to_uppercase().ends_with("PKGBUILD") {
        return true;
    }
    // TODO not sure about this, but I think the source infos
    // are easier to parse than the pkg build, which is a script.
    // See https://wiki.archlinux.org/index.php/.SRCINFO
    if path.to_uppercase().ends_with(".SRCINFO") {
        return true;
    }
    return false;
}
