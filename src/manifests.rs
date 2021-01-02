pub mod arch;
pub mod debian;
pub mod flatpak;
pub mod manifest;
pub mod snap;

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
// of any supported build system, using the path of the input file.
pub fn detect_type(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    // FIXME the filename could match multiple build systems. How should we handle
    // that? If we try parsing a format and it fails, we might want to got back
    // and try parsing using other formats?
    if crate::manifests::debian::file_path_matches(&ctx.source_filename) {
        ctx.source_type = "debian".to_string();
        return 0;
    }

    if crate::manifests::snap::file_path_matches(&ctx.source_filename) {
        ctx.source_type = "snap".to_string();
        return 0;
    }

    if crate::manifests::flatpak::file_path_matches(&ctx.source_filename) {
        ctx.source_type = "flatpak".to_string();
        return 0;
    }

    return -1;
}

pub fn parse(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    if ctx.source_type == "debian" {
        crate::manifests::debian::parse(ctx);
        return 0;
    }

    if ctx.source_type == "snap" {
        crate::manifests::snap::parse(ctx);
        return 0;
    }

    if ctx.source_type == "flatpak" {
        crate::manifests::flatpak::parse(ctx);
        return 0;
    }

    eprintln!("Invalid source type {}.", ctx.source_type);
    return 1;
}

pub fn dump(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    if let Some(_) = ctx.manifest.flatpak_manifest {
        ctx.content = crate::manifests::flatpak::dump_native(&ctx.manifest);
        return 0;
    }

    eprintln!("💩 Could not dump whatever we tried to dump.");
    return 1;
}

pub fn get_modules(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    if let Some(_) = ctx.manifest.flatpak_manifest {
        ctx.manifest.depends_on = crate::manifests::flatpak::get_modules(&ctx.manifest);
        return 0;
    }

    eprintln!("Could not get modules for whatever we tried to get modules for.");
    return 0;
}

pub fn add_module(ctx: &mut crate::execution_context::ExecutionContext, module: &crate::manifests::manifest::AbstractModule) -> i32 {
    if let Some(_) = ctx.manifest.flatpak_manifest {
        crate::manifests::flatpak::add_module(&mut ctx.manifest, module);
        return 0;
    }

    eprintln!("Could not get modules for whatever we tried to get modules for.");
    return 0;
}

pub fn run_build(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    if let Some(_) = ctx.manifest.flatpak_manifest {
        match crate::manifests::flatpak::run_build(&ctx) {
            Ok(r) => return 0,
            Err(e) => {
                return 1;
            },
        };
    }

    eprintln!("Could not run build with whatever we tried to run a build with.");
    return 0;
}
