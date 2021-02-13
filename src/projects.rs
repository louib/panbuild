pub mod project;

pub const CORE_PROJECTS: [&'static str; 20] = [
    "https://git.savannah.gnu.org/cgit/bash.git",
    "https://git.savannah.gnu.org/cgit/make.git",
    "https://git.savannah.gnu.org/cgit/diffutils.git",
    "https://git.savannah.gnu.org/cgit/findutils.git",
    "https://git.savannah.gnu.org/cgit/gzip.git",
    "https://git.savannah.gnu.org/git/grep.git",
    "https://git.savannah.gnu.org/cgit/tar.git",
    "https://git.savannah.gnu.org/git/libtool.git",
    "https://git.lysator.liu.se/lsh/lsh.git",
    "https://git.savannah.gnu.org/cgit/gawk.git",
    "https://github.com/gwsw/less.git",
    "https://github.com/openbsd/src.git",
    "https://gcc.gnu.org/git/gcc.git",
    "https://git.sv.gnu.org/cgit/coreutils.git",
    "https://sourceware.org/git/binutils-gdb.git",
    "https://sourceware.org/git/glibc.git",
    "https://gitlab.gnome.org/GNOME/gtk.git",
    "https://gitlab.gnome.org/GNOME/glib.git",
    "https://dev.gnupg.org/source/gnupg.git",
    "https://gitlab.com/gnutls/gnutls.git",
];

// Get the potential modules that are inferable from the
// projects.
pub fn get_modules() -> Vec<crate::modules::SoftwareModule> {
    let mut modules = vec![];
    for project in crate::db::Database::get_all_projects() {
        for project_version in &project.versions {
            for artifact_name in &project.artifact_names {
                let mut module = crate::modules::SoftwareModule::default();
                module.name = artifact_name.to_string();
                module.version = project_version.to_string();
                module.tag = project_version.to_string();
                if project.vcs_urls.len() != 0 {
                    module.url = project.vcs_urls[0].to_string();
                }
                modules.push(module);
                // println!("Project {} could install {} version {}.", project.name, artifact_name, project_version);
            }
        }
    }
    modules
}

pub fn get_project_tag_names() -> Vec<String> {
    // call tag_names(&self, pattern: Option<&str>) -> Result<StringArray, Error>
    // on the repository.
    // https://docs.rs/git2/0.13.8/git2/struct.Repository.html#method.tag_names
    return vec![];
}

pub fn get_project_commit_signature() -> String {
    // Here we want to asses a project signature by traversing the Revision graph.
    // https://docs.rs/git2/0.13.8/git2/struct.Revwalk.html
    return String::from("");
}
