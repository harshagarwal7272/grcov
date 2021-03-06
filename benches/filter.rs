#![feature(test)]
extern crate grcov;
extern crate test;

use grcov::{CovResult, Function};
use std::collections::HashMap;
use test::{black_box, Bencher};

#[bench]
fn bench_filter_covered(b: &mut Bencher) {
    let mut functions: HashMap<String, Function> = HashMap::new();
    functions.insert(
        "f1".to_string(),
        Function {
            start: 1,
            executed: true,
        },
    );
    functions.insert(
        "f2".to_string(),
        Function {
            start: 2,
            executed: false,
        },
    );
    let result = CovResult {
        lines: [(1, 21), (2, 7), (7, 0)].iter().cloned().collect(),
        branches: [].iter().cloned().collect(),
        functions: functions,
    };
    b.iter(|| black_box(grcov::is_covered(&result)));
}

#[bench]
fn bench_filter_covered_no_functions(b: &mut Bencher) {
    let result = CovResult {
        lines: [(1, 21), (2, 7), (7, 0)].iter().cloned().collect(),
        branches: [].iter().cloned().collect(),
        functions: HashMap::new(),
    };
    b.iter(|| black_box(grcov::is_covered(&result)));
}

#[bench]
fn bench_filter_uncovered_no_lines_executed(b: &mut Bencher) {
    let mut functions: HashMap<String, Function> = HashMap::new();
    functions.insert(
        "f1".to_string(),
        Function {
            start: 1,
            executed: true,
        },
    );
    functions.insert(
        "f2".to_string(),
        Function {
            start: 2,
            executed: false,
        },
    );
    let result = CovResult {
        lines: [(1, 0), (2, 0), (7, 0)].iter().cloned().collect(),
        branches: [].iter().cloned().collect(),
        functions: HashMap::new(),
    };
    b.iter(|| black_box(grcov::is_covered(&result)));
}

#[bench]
fn bench_filter_covered_functions_executed(b: &mut Bencher) {
    let mut functions: HashMap<String, Function> = HashMap::new();
    functions.insert(
        "top-level".to_string(),
        Function {
            start: 1,
            executed: true,
        },
    );
    functions.insert(
        "f".to_string(),
        Function {
            start: 2,
            executed: true,
        },
    );
    let result = CovResult {
        lines: [(1, 21), (2, 7), (7, 0)].iter().cloned().collect(),
        branches: [].iter().cloned().collect(),
        functions: functions,
    };
    b.iter(|| black_box(grcov::is_covered(&result)));
}

#[bench]
fn bench_filter_covered_toplevel_executed(b: &mut Bencher) {
    let mut functions: HashMap<String, Function> = HashMap::new();
    functions.insert(
        "top-level".to_string(),
        Function {
            start: 1,
            executed: true,
        },
    );
    let result = CovResult {
        lines: [(1, 21), (2, 7), (7, 0)].iter().cloned().collect(),
        branches: [].iter().cloned().collect(),
        functions: functions,
    };
    b.iter(|| black_box(grcov::is_covered(&result)));
}

#[bench]
fn bench_filter_uncovered_functions_not_executed(b: &mut Bencher) {
    let mut functions: HashMap<String, Function> = HashMap::new();
    functions.insert(
        "top-level".to_string(),
        Function {
            start: 1,
            executed: true,
        },
    );
    functions.insert(
        "f".to_string(),
        Function {
            start: 7,
            executed: false,
        },
    );
    let result = CovResult {
        lines: [(1, 21), (2, 7), (7, 0)].iter().cloned().collect(),
        branches: [].iter().cloned().collect(),
        functions: functions,
    };
    b.iter(|| black_box(grcov::is_covered(&result)));
}
