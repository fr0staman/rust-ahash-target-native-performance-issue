# Rust Ahash target-cpu=native performance issue

This bench show performance issue with crate ahash on my setup.

Maybe, I'm not alone, so you can try too.

## My setup
Rust:
```
rustc 1.74.1 (a28077b28 2023-12-04)
binary: rustc
commit-hash: a28077b28a02b92985b3a3faecf92813155f1ea1
commit-date: 2023-12-04
host: x86_64-unknown-linux-gnu
release: 1.74.1
LLVM version: 17.0.4
```
```
CPU: AMD Ryzen 5 4500U
OS: Ubuntu 22.04.3 LTS
```

## Results

Standard target
```bash
fr0staman@kotobook:~/source/repos/rust/rust-ahash-target-native-performance-issue$ cargo bench
    Finished bench [optimized] target(s) in 36.18s
     Running unittests src/main.rs (target/release/deps/rust_ahash_target_native_performance_issue-4df22a78d1110619)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/issue.rs (target/release/deps/ahash-3b7ae86a7bc7bacb)
Gnuplot not found, using plotters backend
Performance/ahash/(32, 128)
                        time:   [21.672 µs 21.698 µs 21.727 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
Performance/ahash/(256, 1024)
                        time:   [983.01 µs 983.94 µs 984.92 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
Performance/ahash/(1024, 4096)
                        time:   [15.256 ms 15.298 ms 15.341 ms]
```

target-cpu=native
```bash
fr0staman@kotobook:~/source/repos/rust/rust-ahash-target-native-performance-issue$ RUSTFLAGS='-C target-cpu=native' cargo bench
    Finished bench [optimized] target(s) in 46.42s
     Running unittests src/main.rs (target/release/deps/rust_ahash_target_native_performance_issue-4df22a78d1110619)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/issue.rs (target/release/deps/ahash-3b7ae86a7bc7bacb)
Gnuplot not found, using plotters backend
Performance/ahash/(32, 128)
                        time:   [37.734 µs 37.761 µs 37.789 µs]
                        change: [+73.336% +73.657% +73.980%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high severe
Performance/ahash/(256, 1024)
                        time:   [2.4681 ms 2.4698 ms 2.4717 ms]
                        change: [+150.51% +150.90% +151.29%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Performance/ahash/(1024, 4096)
                        time:   [38.308 ms 38.369 ms 38.433 ms]
                        change: [+149.98% +150.82% +151.60%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```