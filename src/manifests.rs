pub mod snap;
pub mod npm;
pub mod debian;
pub mod flatpak;
pub mod pyproject;
pub mod manifest;

// FIXME should not be public.
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

    return 1;
}
