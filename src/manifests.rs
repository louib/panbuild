pub mod manifest;
pub mod arch;
pub mod debian;
pub mod flatpak;
pub mod snap;

pub fn run_build(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    if let Some(_) = ctx.manifest.flatpak_manifest {
        match crate::manifests::flatpak::run_build(&ctx) {
            Ok(r) => return 0,
            Err(e) => {
                return 1;
            }
        };
    }

    eprintln!("Could not run build with whatever we tried to run a build with.");
    return 0;
}
