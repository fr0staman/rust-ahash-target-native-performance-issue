use ahash::AHashMap;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{distributions::Alphanumeric, Rng};

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn count_symbols(string: &str, symbol_counts: &mut AHashMap<char, usize>) {
    for ch in string.chars() {
        symbol_counts
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(0);
    }
}

fn start_count_symbols(strings: &Vec<String>) {
    let mut result_map = AHashMap::with_capacity(64);

    let result = {
        let mut symbol_counts = AHashMap::with_capacity(64);
        for string in strings {
            count_symbols(string, &mut symbol_counts)
        }

        symbol_counts
    };

    for (symbol, count) in result {
        result_map
            .entry(symbol)
            .and_modify(|v| *v += count)
            .or_insert(count);
    }
}

pub fn _get_strings(vector_size: usize, string_length: usize) -> Vec<String> {
    let mut strings = Vec::with_capacity(vector_size);

    for _ in 0..vector_size {
        let random_string = generate_random_string(string_length);
        strings.push(random_string);
    }

    strings
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Performance");

    for param in [(32, 128), (256, 1024), (1024, 4096)] {
        let bench = BenchmarkId::new("ahash", format!("{:?}", param));
        let strings = _get_strings(param.0, param.1);

        group.bench_with_input(bench, &strings, |b, i| b.iter(|| start_count_symbols(i)));
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
