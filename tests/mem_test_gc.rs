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
        matches!(stats.total_bytes, 7_630_000..=7_640_000),
        "{} not in range",
        stats.total_bytes
    );

    // peak of heap size
    dhat::assert!(
        matches!(stats.max_blocks, 705..=710),
        "{} not in range",
        stats.max_blocks
    );
    dhat::assert!(
        matches!(stats.max_bytes, 62_500..=64_500),
        "{} not in range",
        stats.max_bytes
    );

    // allocations remaining at this point
    dhat::assert!(
        matches!(stats.curr_blocks, 125..=130),
        "{} not in range",
        stats.curr_blocks
    );
    dhat::assert!(
        matches!(stats.curr_bytes, 11_500..=12_500),
        "{} not in range",
        stats.curr_bytes
    );
}
