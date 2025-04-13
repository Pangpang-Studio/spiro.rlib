#[macro_use]
mod data;

use std::fmt;

use snapbox::{Assert, file};

use spiro::{SpiroCP, SpiroCpTy::*, bezctx_oplist::Operation};

const TRYCMD: &str = "TRYCMD";

#[test]
fn test() {
    let mut path = test_data!();
    let oplist = run_spiro(&mut path);
    eprintln!("{oplist:?}");
}

#[test]
fn compute_pderivs_no_index_out_of_bounds() {
    let mut path = vec![
        SpiroCP { x: 0., y: 0.5, ty: G4 },
        SpiroCP { x: 0.5, y: 0., ty: G4 },
        SpiroCP { x: 1., y: 0.5, ty: G4 },
        SpiroCP { x: 0.5, y: 1., ty: G4 },
    ];
    let oplist = run_spiro(&mut path);
    Assert::new()
        .action_env(TRYCMD)
        .eq(format!("{oplist:#?}"), file!["data/pseudo_circle.oplist"]);
}

#[test]
fn add_mat_line_no_underflow() {
    let mut path = test_data!();
    path.drain(..2);
    let oplist = run_spiro(&mut path);
    Assert::new()
        .action_env(TRYCMD)
        .eq(format!("{oplist:#?}"), file!["data/broken_s.oplist"]);
}

fn run_spiro(path: &mut [SpiroCP]) -> Vec<RoundedOperation> {
    // SAFETY: `transmute` is safe here because `RoundedOperation` is a
    // `repr(transparent)` newtype around `Operation`.
    unsafe { std::mem::transmute(spiro::run_spiro(path)) }
}

#[repr(transparent)]
struct RoundedOperation(Operation);

impl RoundedOperation {
    // The number of decimal places to round our floats with for snapshot testing.
    const PRECISION: usize = 12;
}

impl fmt::Debug for RoundedOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = Self::PRECISION;
        match self.0 {
            Operation::MoveTo(x, y, b) => f
                .debug_tuple("MoveTo")
                .field(&format_args!("{x:.p$}"))
                .field(&format_args!("{y:.p$}"))
                .field(&b)
                .finish(),
            Operation::LineTo(x, y) => f
                .debug_tuple("LineTo")
                .field(&format_args!("{x:.p$}"))
                .field(&format_args!("{y:.p$}"))
                .finish(),
            Operation::CurveTo(x, y, z, w, u, v) => f
                .debug_tuple("CurveTo")
                .field(&format_args!("{x:.p$}"))
                .field(&format_args!("{y:.p$}"))
                .field(&format_args!("{z:.p$}"))
                .field(&format_args!("{w:.p$}"))
                .field(&format_args!("{u:.p$}"))
                .field(&format_args!("{v:.p$}"))
                .finish(),
            Operation::MarkKnot(i) => f.debug_tuple("MarkKnot").field(&i).finish(),
        }
    }
}
