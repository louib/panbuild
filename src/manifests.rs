pub mod snap;
pub mod npm;
pub mod debian;
pub mod flatpak;
pub mod pyproject;
pub mod manifest;

// FIXME should not be public.
pub mod abstract_manifest;

pub fn parse(ctx: &manifest::ConversionContext) -> i32 {
    if ctx.source_type == "debian" {
        debian::parse();

    }
    return 0;
}

pub fn dump(ctx: &manifest::ConversionContext) -> i32 {
    return 0;
}

pub fn get_type(ctx: &manifest::ConversionContext) -> i32 {
    return 0;
}
