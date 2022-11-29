use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("display all the nines", |b| {
        b.iter(|| {
            let pyramid_of_nines = vec![
                9,
                99,
                999,
                9_999,
                99_999,
                9_99_999,
                99_999_99,
                999_999_99,
                9_999_999_99,
                99_999_999_99,
                999_999_999_99,
                9_999_999_999_99,
                99_999_999_999_999,
                999_999_999_999_999
            ];

            pyramid_of_nines   
                .iter()
                .for_each(|n| {
                    format!("{}", numbers_to_words::NumberToWords::new(n.to_owned()));
                })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);