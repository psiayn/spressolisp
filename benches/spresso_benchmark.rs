use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use spressolisp::{evaluate_expression, env::Env};

fn criterion_benchmark(c: &mut Criterion) {
    let mut env = Env::new();

    let mut group = c.benchmark_group("add many 1s");
    for num in [10, 100, 1000, 10_000, 20_000].iter() {
        let mut input = String::new();
        input.push_str("(+");
        for _ in 0..*num {
            input.push_str(" 1");
        }
        input.push_str(")");

        group.bench_with_input(BenchmarkId::from_parameter(num), num, |b, _num| {
            b.iter(|| evaluate_expression(input.clone(), &mut env));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
