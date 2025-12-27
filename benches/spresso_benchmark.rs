use criterion::{criterion_group, criterion_main, Criterion};
use spressolisp::{env::Env, evaluate_expression};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let mut env = Env::new();
    c.bench_function("simple", |b| {
        b.iter(|| {
            evaluate_expression(
                "bench".to_owned(),
                black_box("(+ 1 1)".to_owned()),
                &mut env,
            )
        })
    });

    c.bench_function("loop_1m", |b| {
        b.iter(|| {
            evaluate_expression(
                "bench".to_owned(),
                black_box(
                    r"
                    (define i 0)
                    (define n 10000)
                    (define sum 0)
                    (loop (< i n) (list (
                        (define sum (+ sum i))
                        (define i (+ i 1))
                    )))
                    "
                    .to_owned(),
                ),
                &mut env,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
