#[allow(unused)]
macro_rules! test_data {
    () => {{
        use spiro::SpiroCpTy::*;
        vec![
            SpiroCP {
                x: 334.,
                y: 117.,
                ty: Corner,
            },
            SpiroCP {
                x: 305.,
                y: 176.,
                ty: Corner,
            },
            SpiroCP { x: 212., y: 142., ty: G2 },
            SpiroCP { x: 159., y: 171., ty: G2 },
            SpiroCP { x: 224., y: 237., ty: G2 },
            SpiroCP { x: 347., y: 335., ty: G2 },
            SpiroCP { x: 202., y: 467., ty: G2 },
            SpiroCP {
                x: 81.,
                y: 429.,
                ty: Corner,
            },
            SpiroCP {
                x: 114.,
                y: 368.,
                ty: Corner,
            },
            SpiroCP { x: 201., y: 402., ty: G2 },
            SpiroCP { x: 276., y: 369., ty: G2 },
            SpiroCP { x: 218., y: 308., ty: G2 },
            SpiroCP { x: 91., y: 211., ty: G2 },
            SpiroCP { x: 124., y: 111., ty: G2 },
            SpiroCP { x: 229., y: 82., ty: G2 },
        ]
    }};
}
