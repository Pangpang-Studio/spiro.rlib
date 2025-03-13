#![cfg(feature = "glifparser")]

#[macro_use]
mod data;

use glifparser::outline::ToOutline as _;
use snapbox::{file, Assert};

use spiro::*;

const TRYCMD: &str = "TRYCMD";

#[test]
fn test() {
    let mut ctx = BezierContext::<BezCtxGpPenOpsData, ()>::new();
    let path = test_data!();
    ctx.run_spiro(&path);
    let outline: glifparser::Outline<()> = ctx.data.ops_path.to_outline();
    let mut glif = glifparser::Glif::<()>::new();
    glif.outline = Some(outline);
    let out_glif = glifparser::glif::write(&glif).unwrap();
    Assert::new().action_env(TRYCMD).eq(out_glif, file!["data/spiro.glif"]);
}
