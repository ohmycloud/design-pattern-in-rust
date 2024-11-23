use criterion::{black_box, criterion_group, criterion_main, Criterion};
use examples::log_parser::Parser as FsmParser;
use examples::nom_parser::parse_log;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn benchmark_fsm(c: &mut Criterion) {
    let log = "2024-05-05 23:59:58.846 [223.104.43.11:11686#10.0.1.88:5003] R:6822eee05c460d03030001001940000080c843001a40003373c843001b400033b3c84300";
    let count = Arc::new(AtomicUsize::new(0));

    c.bench_function("FSM Parser", |b| {
        b.iter(|| {
            let count = Arc::clone(&count);
            let mut parser = FsmParser::new();
            parser.parse(log).unwrap();
            count.fetch_add(1, Ordering::SeqCst);
        });
    });

    println!("FSM Parser total iterations: {}", count.load(Ordering::SeqCst));
}

fn benchmark_nom(c: &mut Criterion) {
    let log = "2024-05-05 23:59:58.846 [223.104.43.11:11686#10.0.1.88:5003] R:6822eee05c460d03030001001940000080c843001a40003373c843001b400033b3c84300";
    let count = Arc::new(AtomicUsize::new(0));

    c.bench_function("nom Parser", |b| {
        b.iter(|| {
            let count = Arc::clone(&count);
            parse_log(black_box(log)).unwrap();
            count.fetch_add(1, Ordering::SeqCst);
        });
    });

    println!("nom Parser total iterations: {}", count.load(Ordering::SeqCst));
}

criterion_group!(benches, benchmark_fsm, benchmark_nom);
criterion_main!(benches);
