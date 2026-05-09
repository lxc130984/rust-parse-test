use chumsky::Parser;
use criterion::{ criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use rust_parse_test::{parser,eval::eval};

const INPUT :&str = "fn factorial(n) {
    if n == 0 || n==1 {
        1
    } else {
        factorial(n-  2) + factorial(n - 1)
    }
}
factorial(29)";

fn bench_parser(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        b.iter(|| {
	    let result = parser().parse(black_box(INPUT));
	    black_box(result)
	    
	})
    });
}

// 组合多个基准函数
criterion_group!(benches, bench_parser);
criterion_main!(benches);
