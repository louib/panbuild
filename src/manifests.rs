pub mod snap;
pub mod debian;
pub mod flatpak;
pub mod abstract_manifest;

pub fn has_type(type_name: String) -> bool {
    if type_name == "debian" {
        return true;
    }
    if type_name == "flatpak" {
        return true;
    }
    if type_name == "snap" {
        return true;
    }
    return false;
}

// Determines if the filename is a potential manifest
// of any supported build system.
pub fn get_type(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    if crate::manifests::debian::is_type(ctx) {
        //ctx.source_type = "debian".to_string();
    }

    if crate::manifests::snap::is_type(ctx) {
        //ctx.source_type = "snap".to_string();
    }

    if crate::manifests::flatpak::is_type(ctx) {
        //ctx.source_type = "flatpak".to_string();
    }

    return 0;
}

// Get the top-level build system for the project.
pub fn get_build_system(
    ctx: &crate::execution_context::ExecutionContext
) -> crate::manifests::abstract_manifest::BuildSystem {
    if ctx.source_filename.ends_with("meson_options.txt") {
        return crate::manifests::abstract_manifest::BuildSystem::meson;
    }
    if ctx.source_filename.ends_with("control") {
        // return crate::manifests::abstract_manifest::BuildSystem::debian;
    }
    if ctx.source_filename.ends_with("Makefile") {
        return crate::manifests::abstract_manifest::BuildSystem::make;
    }
    return crate::manifests::abstract_manifest::DEFAULT_BUILD_SYSTEM;
}

pub fn parse(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    if ctx.source_type == "debian" {
        return crate::manifests::debian::parse(ctx);
    }

    if ctx.source_type == "snap" {
        return crate::manifests::snap::parse(ctx);
    }

    if ctx.source_type == "flatpak" {
        return crate::manifests::flatpak::parse(ctx);
    }

    return 1;
}

pub fn dump(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    if ctx.destination_type == "debian" {
        return crate::manifests::debian::dump(ctx);
    }

    if ctx.destination_type == "snap" {
        return crate::manifests::snap::dump(ctx);
    }

    if ctx.destination_type == "flatpak" {
        return crate::manifests::flatpak::dump(ctx);
    }

    return 1;
}
