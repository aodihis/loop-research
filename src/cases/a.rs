/// Fusion: both simple calculations in a single loop:
///   sum:  index + (index - 1)
///   prod: vec[index] = vec[index] * vec[index - 1]
pub fn fusion(n: usize) -> u64 {
    let mut vec: Vec<u64> = (0..n).map(|i| (i as u64 % 7) + 1).collect();
    let mut result = 0u64;

    for i in 1..n {
        // index + index-1
        result = result.wrapping_add(i as u64 + (i - 1) as u64);

        // vec[index] = vec[index] * vec[index-1]
        vec[i] = vec[i].wrapping_mul(vec[i - 1]);
    }

    // fold the last product value in so the multiply work is never dead
    result = result.wrapping_add(vec[n.saturating_sub(1)]);
    result
}

/// Fission: the same two calculations split into separate loop passes.
pub fn fission(n: usize) -> u64 {
    let mut vec: Vec<u64> = (0..n).map(|i| (i as u64 % 7) + 1).collect();
    let mut result = 0u64;

    // Fissioned loop 1: index + index-1
    for i in 1..n {
        result = result.wrapping_add(i as u64 + (i - 1) as u64);
    }

    // Fissioned loop 2: vec[index] = vec[index] * vec[index-1]
    for i in 1..n {
        vec[i] = vec[i].wrapping_mul(vec[i - 1]);
    }

    result = result.wrapping_add(vec[n.saturating_sub(1)]);
    result
}