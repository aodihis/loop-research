//! Case c: register-pressure variant.
//!
//! Hot kernel `sub` keeps 36 `f64` temporaries live at once, spilling beyond the
//! 16 XMM registers on x86-64. `fused` and `fission_impl` are the two loop
//! shapes (both left inline-able).
//! compute `b[i] = sub(a[i], 1.0001)` and `d[i] = sub(c[i], 0.9999)`.

/// Full 36-term polynomial — maximum register pressure (36 live f64 temps).
#[inline(always)]
fn sub(x: f64, k: f64) -> f64 {
    let p01 = x * (k * 8.0 + 5.0) + 1.0;
    let p02 = x * (k * 4.0 + 1.0) + 2.0;
    let p03 = x * (k * 11.0 + -3.0) + 3.0;
    let p04 = x * (k * 7.0 + -7.0) + 4.0;
    let p05 = x * (k * 3.0 + 6.0) + 5.0;
    let p06 = x * (k * 10.0 + 2.0) + 6.0;
    let p07 = x * (k * 6.0 + -2.0) + 7.0;
    let p08 = x * (k * 2.0 + -6.0) + 8.0;
    let p09 = x * (k * 9.0 + 7.0) + 9.0;
    let p10 = x * (k * 5.0 + 3.0) + 10.0;
    let p11 = x * (k * 1.0 + -1.0) + 11.0;
    let p12 = x * (k * 8.0 + -5.0) + 12.0;
    let p13 = x * (k * 4.0 + 8.0) + 13.0;
    let p14 = x * (k * 11.0 + 4.0) + 14.0;
    let p15 = x * (k * 7.0 + 0.0) + 15.0;
    let p16 = x * (k * 3.0 + -4.0) + 16.0;
    let p17 = x * (k * 10.0 + -8.0) + 17.0;
    let p18 = x * (k * 6.0 + 5.0) + 18.0;
    let p19 = x * (k * 2.0 + 1.0) + 19.0;
    let p20 = x * (k * 9.0 + -3.0) + 20.0;
    let p21 = x * (k * 5.0 + -7.0) + 21.0;
    let p22 = x * (k * 1.0 + 6.0) + 22.0;
    let p23 = x * (k * 8.0 + 2.0) + 23.0;
    let p24 = x * (k * 4.0 + -2.0) + 24.0;
    let p25 = x * (k * 11.0 + -6.0) + 25.0;
    let p26 = x * (k * 7.0 + 7.0) + 26.0;
    let p27 = x * (k * 3.0 + 3.0) + 27.0;
    let p28 = x * (k * 10.0 + -1.0) + 28.0;
    let p29 = x * (k * 6.0 + -5.0) + 29.0;
    let p30 = x * (k * 2.0 + 8.0) + 30.0;
    let p31 = x * (k * 9.0 + 4.0) + 31.0;
    let p32 = x * (k * 5.0 + 0.0) + 32.0;
    let p33 = x * (k * 1.0 + -4.0) + 33.0;
    let p34 = x * (k * 8.0 + -8.0) + 34.0;
    let p35 = x * (k * 4.0 + 5.0) + 35.0;
    let p36 = x * (k * 11.0 + 1.0) + 36.0;
    p01 + p02 + p03 + p04 + p05 + p06 + p07 + p08 + p09 + p10 + p11 + p12 + p13 + p14 + p15 + p16 + p17 + p18 + p19 + p20 + p21 + p22 + p23 + p24 + p25 + p26 + p27 + p28 + p29 + p30 + p31 + p32 + p33 + p34 + p35 + p36
}

/// Fused: one loop per element, computing both outputs.
fn fused(a: &[f64], b: &mut [f64], c: &[f64], d: &mut [f64]) -> f64 {
    for i in 0..a.len() {
        b[i] = sub(a[i], 1.0001);
        d[i] = sub(c[i], 0.9999);
    }
    b.iter().chain(d.iter()).sum()
}

/// Fission: one loop per computation.
fn fission_impl(a: &[f64], b: &mut [f64], c: &[f64], d: &mut [f64]) -> f64 {
    for i in 0..a.len() {
        b[i] = sub(a[i], 1.0001);
    }
    for i in 0..c.len() {
        d[i] = sub(c[i], 0.9999);
    }
    b.iter().chain(d.iter()).sum()
}

/// Fusion entry point: allocates the vecs and folds the f64 sum into a u64.
pub fn fusion(n: usize) -> u64 {
    let a: Vec<f64> = (0..n).map(|i| (i as f64 % 7.0) + 1.0).collect();
    let c: Vec<f64> = (0..n).map(|i| (i as f64 % 5.0) + 1.0).collect();
    let mut b = vec![0.0f64; n];
    let mut d = vec![0.0f64; n];

    let total = fused(&a, &mut b, &c, &mut d);
    total as u64
}

/// Fission entry point: allocates the vecs and folds the f64 sum into a u64.
pub fn fission(n: usize) -> u64 {
    let a: Vec<f64> = (0..n).map(|i| (i as f64 % 7.0) + 1.0).collect();
    let c: Vec<f64> = (0..n).map(|i| (i as f64 % 5.0) + 1.0).collect();
    let mut b = vec![0.0f64; n];
    let mut d = vec![0.0f64; n];

    let total = fission_impl(&a, &mut b, &c, &mut d);
    total as u64
}
