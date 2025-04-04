#![doc = include_str!("../README.md")]

pub mod inner;
use inner::*;

pub mod bezctx_oplist;
pub mod bezctx_ps;
use bezctx_oplist::Operation;

#[cfg(feature = "glifparser")]
pub mod bezctx_oplist_glifparser;
#[cfg(feature = "glifparser")]
pub use bezctx_oplist_glifparser::BezCtxGpPenOpsData;

/// A “Bézier context”, for doing curve conversions Spiro→cubic Bézier.
#[derive(Copy, Clone)]
pub struct BezierContext<T, A> {
    /// Callback on "move to" operation.
    /// (x, y, is_open)
    pub move_fn: fn(&mut Self, f64, f64, bool) -> A,
    /// (x, y)
    pub line_fn: fn(&mut Self, f64, f64) -> A,
    /// (x1, y1, x2, y2, x3, y3)
    pub curve_fn: fn(&mut Self, f64, f64, f64, f64, f64, f64) -> A,
    /// (knot_idx)
    pub mark_knot_fn: fn(&mut Self, usize) -> A,
    /// Called before any points emitted.
    pub start: fn(&mut Self) -> A,
    /// Called after all points emitted.
    pub end: fn(&mut Self) -> A,
    /// User-provided data available to the callbacks.
    pub data: T,
}

impl<T, A> BezierContext<T, A> {
    pub fn move_to(&mut self, x: f64, y: f64, is_open: bool) -> A {
        (self.move_fn)(self, x, y, is_open)
    }
    pub fn line_to(&mut self, x: f64, y: f64) -> A {
        (self.line_fn)(self, x, y)
    }
    /// SVG semantics
    pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> A {
        (self.curve_fn)(self, x1, y1, x2, y2, x3, y3)
    }
    pub fn mark_knot(&mut self, knot_idx: usize) -> A {
        (self.mark_knot_fn)(self, knot_idx)
    }
    /// Start / end of run
    pub fn start(&mut self) -> A {
        (self.start)(self)
    }
    pub fn end(&mut self) -> A {
        (self.end)(self)
    }
}

impl<T, A> BezierContext<T, A> {
    /// Sets up the path, solves (integrates) the Spiro, makes it into a cubic Bézier, then returns
    /// a mutable reference to the path data.
    pub fn run_spiro(&mut self, path: &[SpiroCP]) -> &mut T {
        let mut segs = setup_path(path);
        solve_spiro(&mut segs);
        self.start();
        spiro_to_bpath(&segs, path.len(), self);
        self.end();
        &mut self.data
    }
}

impl<T, A> Default for BezierContext<T, A>
where
    T: Default,
    A: Default,
{
    fn default() -> Self {
        Self {
            start: |_| A::default(),
            move_fn: |_, _, _, _| A::default(),
            line_fn: |_, _, _| A::default(),
            curve_fn: |_, _, _, _, _, _, _| A::default(),
            mark_knot_fn: |_, _| A::default(),
            end: |_| A::default(),
            data: T::default(),
        }
    }
}

/// A Spiro control point.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SpiroCP {
    pub x: f64,
    pub y: f64,
    pub ty: SpiroCpTy,
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SpiroCpTy {
    Corner = b'v',
    G4 = b'o',
    G2 = b'c',
    /// Also known as "flat".
    Left = b'[',
    /// Also known as "curl".
    Right = b']',
    Anchor = b'a',
    Handle = b'h',
    #[default]
    End = b'z',
    Open = b'{',
    EndOpen = b'}',
}

impl std::cmp::PartialEq<u8> for SpiroCpTy {
    fn eq(&self, other: &u8) -> bool {
        &(*self as u8) == other
    }
}

/// C(2Rust-derived) implementation of third-order polynomial spirals.
///
/// C is from:
/// ```text
/// ppedit - A pattern plate editor for Spiro splines.
/// Copyright (C) 2007 Raph Levien
///
/// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
/// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
/// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
/// option. This file may not be copied, modified, or distributed
/// except according to those terms.
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct SpiroSegment {
    x: f64,
    y: f64,
    ty: SpiroCpTy,
    bend_th: f64,
    ks: [f64; 4],
    seg_ch: f64,
    seg_th: f64,
}

/// Convert Spiro points into Spiro segments.
pub fn setup_path(src: &[SpiroCP]) -> Vec<SpiroSegment> {
    let mut n = src.len();

    if n == 0 {
        return Vec::new();
    }

    if (src[0]).ty == b'{' {
        n -= 1;
    };

    let mut r = vec![SpiroSegment::default(); n + 1];

    let mut i = 0;
    let mut ilast;
    while i < n {
        r[i].x = src[i].x;
        r[i].y = src[i].y;
        r[i].ty = src[i].ty;
        r[i].ks[0] = 0.0;
        r[i].ks[1] = 0.0;
        r[i].ks[2] = 0.0;
        r[i].ks[3] = 0.0;
        i += 1
    }
    r[n].x = src[n % src.len()].x;
    r[n].y = src[n % src.len()].y;
    r[n].ty = src[n % src.len()].ty;
    i = 0;
    while i < n {
        let dx: f64 = r[i + 1].x - r[i].x;
        let dy: f64 = r[i + 1].y - r[i].y;
        r[i].seg_ch = dx.hypot(dy);
        r[i].seg_th = dy.atan2(dx);
        i += 1
    }
    ilast = n - 1;
    i = 0;
    while i < n {
        if (r[i].ty == b'{') || (r[i].ty == b'}') || (r[i].ty == b'v') {
            r[i].bend_th = 0.0;
        } else {
            r[i].bend_th = mod2pi(r[i].seg_th - r[ilast].seg_th);
        }
        ilast = i;
        i += 1
    }
    r
}

/// Solve (integrate) Spiro.
pub fn solve_spiro(s: &mut [SpiroSegment]) {
    const INTEGRATION_STEPS: usize = 10;

    let nmat: usize = count_vec(s);
    let mut n_alloc: usize = nmat;
    let mut norm: f64;

    if nmat == 0 {
        return;
    }
    if s[0].ty != b'{' && s[0].ty != b'v' {
        n_alloc *= 3
    }
    if n_alloc < 5 {
        n_alloc = 5
    }

    let mut m = vec![BandMath::default(); n_alloc];
    let mut v = vec![0.0; n_alloc];
    let mut perm = vec![0; n_alloc];

    for _ in 0..INTEGRATION_STEPS {
        norm = spiro_iter(s, &mut m, &mut perm, &mut v);
        if norm < 1e-12 {
            break;
        }
    }
}

/// Convert Spiro segments to cubic Bézier splines.
pub fn spiro_to_bpath<T, A>(s: &[SpiroSegment], n: usize, bc: &mut BezierContext<T, A>) {
    if n == 0 {
        return;
    }
    let mut i = 0;
    let nsegs: usize = if s[n - 1].ty == b'}' { (n) - 1 } else { n };
    while i < nsegs {
        let x0: f64 = s[i].x;
        let y0: f64 = s[i].y;
        let x1: f64 = s[i + 1].x;
        let y1: f64 = s[i + 1].y;
        if i == 0 {
            bc.move_to(x0, y0, s[0_usize].ty == b'{');
        }
        bc.mark_knot(i);
        spiro_seg_to_bpath(s[i].ks, x0, y0, x1, y1, bc, 0);
        i += 1
    }
}

/// Run Spiro and yield a [`Vec`] of [`Operation`]’s
pub fn run_spiro(path: &mut [SpiroCP]) -> Vec<Operation> {
    let path_len = path.len();
    let mut ctx = BezierContext::<Vec<Operation>, ()>::new();
    let mut segs = setup_path(path);
    ctx.start();
    solve_spiro(&mut segs);
    spiro_to_bpath(&segs, path_len, &mut ctx);
    ctx.end();
    ctx.data
}
