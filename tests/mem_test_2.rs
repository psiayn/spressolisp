#![cfg(feature = "memory_test")]

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[macro_use]
extern crate assert_float_eq;

pub mod common;
pub mod mem_test_common;
use common::eval_expr_in_env;
use spressolisp::env::Env;

#[test]
fn test() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let mut env = Env::new();
    env.disable_gc();
    let res = eval_expr_in_env(mem_test_common::MANY_CLOSURES, &mut env);
    drop(res);

    let stats = dhat::HeapStats::get();

    // allocations done in total
    dhat::assert!(
        matches!(stats.total_blocks, 95_000..=100_000),
        "{} not in range",
        stats.total_blocks
    );
    dhat::assert!(
        matches!(stats.total_bytes, 7_650_000..=7_750_000),
        "{} not in range",
        stats.total_bytes
    );

    // peak of heap size
    dhat::assert!(
        matches!(stats.max_blocks, 21_000..=22_000),
        "{} not in range",
        stats.max_blocks
    );
    dhat::assert!(
        matches!(stats.max_bytes, 1_900_000..=2_000_000),
        "{} not in range",
        stats.max_bytes
    );

    // allocations remaining at this point
    dhat::assert!(
        matches!(stats.curr_blocks, 20_500..=21_500),
        "{} not in range",
        stats.curr_blocks
    );
    dhat::assert!(
        matches!(stats.curr_bytes, 1_850_000..=1_950_000),
        "{} not in range",
        stats.curr_bytes
    );
}
