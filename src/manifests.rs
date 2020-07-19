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
        let lines = ctx.content.split("\n");
        // let mut paragraphs = Vec<Vec<String>>;
        let mut count = 0;
        for line in lines {
            print!("***** {}", line);
            let mut only_spaces = true;
            let mut indent_size = 0;
            let is_empty_line: bool = line.starts_with(|c: char| {
                if c == ' ' {
                    indent_size = indent_size + 1;
                    return true;
                }
                if c == '\t' {
                    return true;
                }
                return false;
            });
            count = count + 1;
        }
        return 0;
    }

    if ctx.source_type == "snap" {
        // let yml_load_result = YamlLoader::load_from_str(&ctx.content);

        // if yml_load_result.is_err() {
            // return;
        // }

        // let manifest_content = yml_load_result.unwrap();
        //
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
