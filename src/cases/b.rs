//! Case b: memory-pressure variant.
//!
//! Memory pressure comes from two sources:
//! 1. Three N-sized `u64` vecs live at once (24 bytes/element).
//! 2. Loop bodies read `input` through a pseudo-random index, so every
//!    iteration touches an unpredictable cache line (cache misses).

/// LCG step: cheap pseudo-random index over `[0, n)`.
#[inline]
fn next_idx(seed: &mut u64, n: usize) -> usize {
    *seed = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*seed % n as u64) as usize
}

/// Fusion: both pipelines in one loop, with random-access reads.
pub fn fusion(n: usize) -> u64 {
    let input: Vec<u64> = (0..n).map(|i| (i as u64 % 7) + 1).collect();
    let mut out_a = vec![0u64; n];
    let mut out_b = vec![0u64; n];
    let mut seed_a = 0x9E37_79B9_7F4A_7C15u64;
    let mut seed_b = 0x9E37_79B9_7F4A_7C15u64;

    for i in 0..n {
        // pipeline a uses seed_a, pipeline b uses seed_b — the same
        // sequences fission runs in its separate loops, just fused
        let j = next_idx(&mut seed_a, n);
        let x = input[j];
        let y = input[(j.wrapping_mul(31) + 7) % n];
        out_a[i] = x.wrapping_add(y).wrapping_mul(3);

        let j = next_idx(&mut seed_b, n);
        let x = input[j];
        let y = input[(j.wrapping_mul(31) + 7) % n];
        out_b[i] = y.wrapping_sub(x).wrapping_mul(5);
    }

    // re-touch both outputs so the writes are never dead
    let mut result = 0u64;
    for i in 0..n {
        result = result.wrapping_add(out_a[i]).wrapping_add(out_b[i]);
    }
    result
}

/// Fission: same random-access work split into separate loop passes.
pub fn fission(n: usize) -> u64 {
    let input: Vec<u64> = (0..n).map(|i| (i as u64 % 7) + 1).collect();
    let mut out_a = vec![0u64; n];
    let mut out_b = vec![0u64; n];

    // Fissioned loop 1: output a
    let mut seed_a = 0x9E37_79B9_7F4A_7C15u64;
    for i in 0..n {
        let j = next_idx(&mut seed_a, n);
        let x = input[j];
        let y = input[(j.wrapping_mul(31) + 7) % n];
        out_a[i] = x.wrapping_add(y).wrapping_mul(3);
    }

    // Fissioned loop 2: output b
    let mut seed_b = 0x9E37_79B9_7F4A_7C15u64;
    for i in 0..n {
        let j = next_idx(&mut seed_b, n);
        let x = input[j];
        let y = input[(j.wrapping_mul(31) + 7) % n];
        out_b[i] = y.wrapping_sub(x).wrapping_mul(5);
    }

    let mut result = 0u64;
    for i in 0..n {
        result = result.wrapping_add(out_a[i]).wrapping_add(out_b[i]);
    }
    result
}