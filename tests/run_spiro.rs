#[macro_use]
mod data;

use spiro::{SpiroCpTy::*, *};

#[test]
fn test() {
    let mut path = test_data!();
    let oplist = run_spiro(&mut path);
    eprintln!("{oplist:?}");
}

#[test]
fn add_mat_line_no_index_out_of_bounds() {
    let mut path = test_data!();
    path.insert(path.len() - 2, SpiroCP { ty: End, ..path[0] });
    let oplist = run_spiro(&mut path);
    eprintln!("{oplist:?}");
}

#[test]
fn add_mat_line_no_underflow() {
    let mut path = test_data!();
    path.drain(..2);
    let oplist = run_spiro(&mut path);
    eprintln!("{oplist:?}");
}

#[test]
fn add_mat_line_no_underflow_abrupt_end() {
    let mut path = test_data!();
    path.reverse();
    path.insert(1, SpiroCP { ty: End, ..path[0] });
    let oplist = run_spiro(&mut path);
    eprintln!("{oplist:?}");
}
