use std::time::Instant;

const N: usize = 100_000_000;

fn sum_only(n: usize) -> u64 {
    let mut result = 0u64;
    for i in 1..n {
        result = result.wrapping_add(i as u64 + (i - 1) as u64);
    }
    result
}

fn prod_only(n: usize) -> u64 {
    let mut vec: Vec<u64> = (0..n).map(|i| (i as u64 % 7) + 1).collect();
    for i in 1..n {
        vec[i] = vec[i].wrapping_mul(vec[i - 1]);
    }
    vec[n - 1]
}

fn fused(n: usize) -> u64 {
    let mut vec: Vec<u64> = (0..n).map(|i| (i as u64 % 7) + 1).collect();
    let mut result = 0u64;
    for i in 1..n {
        result = result.wrapping_add(i as u64 + (i - 1) as u64);
        vec[i] = vec[i].wrapping_mul(vec[i - 1]);
    }
    result.wrapping_add(vec[n - 1])
}

fn fission(n: usize) -> u64 {
    let mut vec: Vec<u64> = (0..n).map(|i| (i as u64 % 7) + 1).collect();
    let mut result = 0u64;
    for i in 1..n {
        result = result.wrapping_add(i as u64 + (i - 1) as u64);
    }
    for i in 1..n {
        vec[i] = vec[i].wrapping_mul(vec[i - 1]);
    }
    result.wrapping_add(vec[n - 1])
}

// Independent multiply (no recurrence): fusion should win here.
fn fused_indep(n: usize) -> u64 {
    let mut vec: Vec<u64> = (0..n).map(|i| (i as u64 % 7) + 1).collect();
    let mut result = 0u64;
    for i in 1..n {
        result = result.wrapping_add(i as u64 + (i - 1) as u64);
        vec[i] = vec[i].wrapping_mul(3);
    }
    result.wrapping_add(vec[n - 1])
}

fn fission_indep(n: usize) -> u64 {
    let mut vec: Vec<u64> = (0..n).map(|i| (i as u64 % 7) + 1).collect();
    let mut result = 0u64;
    for i in 1..n {
        result = result.wrapping_add(i as u64 + (i - 1) as u64);
    }
    for i in 1..n {
        vec[i] = vec[i].wrapping_mul(3);
    }
    result.wrapping_add(vec[n - 1])
}

fn median_of(f: fn(usize) -> u64, reps: usize) -> (u128, u64) {
    let mut times = Vec::with_capacity(reps);
    let mut checksum = 0u64;
    for _ in 0..reps {
        let t = Instant::now();
        checksum = f(N);
        times.push(t.elapsed().as_nanos());
    }
    times.sort_unstable();
    (times[reps / 2], checksum)
}

fn main() {
    for (name, f) in [
        ("sum_only  (index+index-1)", sum_only as fn(usize) -> u64),
        ("prod_only (serial multiply)", prod_only as fn(usize) -> u64),
        ("fused     (sum+prod one loop)", fused as fn(usize) -> u64),
        ("fission   (sum loop, then prod loop)", fission as fn(usize) -> u64),
        ("fused     (indep prod, one loop)", fused_indep as fn(usize) -> u64),
        ("fission   (indep prod, two loops)", fission_indep as fn(usize) -> u64),
    ] {
        let (med, checksum) = median_of(f, 5);
        println!("{name:<33} median = {:>12} us   (checksum {checksum})", med / 1000);
    }
}