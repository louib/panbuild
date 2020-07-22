// See https://wiki.archlinux.org/index.php/PKGBUILD
// And
// https://jlk.fjfi.cvut.cz/arch/manpages/man/BUILDINFO.5
// for details on the pkgbuild format.

pub fn dump(manifest: &crate::manifests::manifest::AbstractManifest) -> String {
    let mut response = String::from("");
    return response;

}

pub fn parse(content: &str) -> crate::manifests::manifest::AbstractManifest {
    let mut response = crate::manifests::manifest::AbstractManifest::default();
    return response;
}

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

pub fn file_content_matches(content: &str) -> bool {
    return false;
}
