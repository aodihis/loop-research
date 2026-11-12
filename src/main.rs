use std::time::Instant;

use loop_scan_bench::cases::{a, b, c};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <N> <case> <method>", args[0]);
        eprintln!("  N      - size of the vec (usize)");
        eprintln!("  case   - a, b, c, ...");
        eprintln!("  method - fusion | fission");
        std::process::exit(1);
    }

    let n: usize = args[1].parse().expect("N must be a valid usize");
    let case = args[2].as_str();
    let method = args[3].as_str();

    let start = Instant::now();
    let result = match case {
        "a" => match method {
            "fusion" => a::fusion(n),
            "fission" => a::fission(n),
            other => panic!("method '{other}' not found (expected `fusion` or `fission`)"),
        },
        "b" | "2" => match method {
            "fusion" => b::fusion(n),
            "fission" => b::fission(n),
            other => panic!("method '{other}' not found (expected `fusion` or `fission`)"),
        },
        "c" | "3" => match method {
            "fusion" => c::fusion(n),
            "fission" => c::fission(n),
            other => panic!("method '{other}' not found (expected `fusion` or `fission`)"),
        },
        other => panic!("case '{other}' not found (expected `a`, `b`, `c`, ...)"),
    };
    let elapsed = start.elapsed();

    println!("case: {case}, method: {method}, N: {n}, result: {result}, time: {elapsed:?}, ns: {}", elapsed.as_nanos());
}