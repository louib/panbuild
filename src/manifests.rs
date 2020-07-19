pub mod snap;
pub mod npm;
pub mod debian;
pub mod flatpak;
pub mod pyproject;
pub mod manifest;

// FIXME should not be public.
pub mod abstract_manifest;

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
    return 0;
}

// Determines if the filename is a potential manifest
// of any supported build system. Empty string means the detection
// failed.
pub fn get_type(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    return 0;
}
