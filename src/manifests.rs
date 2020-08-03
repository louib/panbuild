pub mod snap;
pub mod debian;
pub mod flatpak;
pub mod arch;
pub mod manifest;

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
pub fn get_type(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    // FIXME the filename could match multiple build systems. How should we handle
    // that? If we try parsing a format and it fails, we might want to got back
    // and try parsing using other formats?
    if crate::manifests::debian::file_path_matches(&ctx.source_filename) {
        ctx.source_type = "debian".to_string();
    }

    if crate::manifests::snap::file_path_matches(&ctx.source_filename) {
        ctx.source_type = "snap".to_string();
    }

    if crate::manifests::flatpak::file_path_matches(&ctx.source_filename) {
        ctx.source_type = "flatpak".to_string();
    }

    return 0;
}

pub fn parse(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    if ctx.source_type == "debian" {
        ctx.manifest = crate::manifests::debian::parse(&ctx.content);
        return 0;
    }

    if ctx.source_type == "snap" {
        ctx.manifest = crate::manifests::snap::parse(&ctx.content);
        return 0;
    }

    if ctx.source_type == "flatpak" {
        ctx.manifest = crate::manifests::flatpak::parse(&ctx.content);
        return 0;
    }

    eprintln!("Invalid source type {}.", ctx.source_type);
    return 1;
}

pub fn dump(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    if ctx.destination_type == "debian" {
        let dump: String = crate::manifests::debian::dump(&ctx.manifest);
        println!("{}", dump);
        return 0;
    }

    if ctx.destination_type == "snap" {
        let dump: String = crate::manifests::snap::dump(&ctx.manifest);
        println!("{}", dump);
        return 0;
    }

    if ctx.destination_type == "flatpak" {
        let dump: String = crate::manifests::flatpak::dump(&ctx.manifest);
        println!("{}", dump);
        return 0;
    }

    eprintln!("Invalid destination type {}.", ctx.destination_type);
    return 1;
}
