#[macro_use]
mod data;

use std::str;

use snapbox::{Assert, file};

use spiro::*;

const TRYCMD: &str = "TRYCMD";

#[test]
fn test() {
    let path = test_data!();

    let mut buffer = Vec::<u8>::new();
    {
        let mut writer = std::io::BufWriter::new(&mut buffer);
        let mut boxed_writer = Box::new(&mut writer);
        let mut ctx = bezctx_ps::PostScriptBezierContext::new(&mut boxed_writer);
        ctx.run_spiro(&path);
    }

    let bufstr: &str = str::from_utf8(&buffer).unwrap();
    Assert::new()
        .action_env(TRYCMD)
        .eq(bufstr.trim_matches('\0'), file!["data/spiro.ps"]);
}
