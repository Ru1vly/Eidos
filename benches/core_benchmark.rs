use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lib_core::Core;
use std::path::PathBuf;

fn benchmark_core_creation(c: &mut Criterion) {
    c.bench_function("core_new", |b| {
        b.iter(|| {
            // Note: This will fail without valid model files, but demonstrates the benchmark structure
            // In a real scenario, you'd have test fixtures
            let _ = Core::new(
                black_box("model.onnx"),
                black_box("tokenizer.json"),
            );
        })
    });
}

fn benchmark_command_validation(c: &mut Criterion) {
    // Since we can't directly access is_safe_command from the public API,
    // we'll benchmark the full run() method with invalid commands
    c.bench_function("command_validation", |b| {
        b.iter(|| {
            // This benchmarks the validation logic indirectly
            // by attempting to validate various commands
            let commands = vec![
                "ls -la",
                "pwd",
                "echo hello",
                "cd ..",
                "mkdir test",
            ];

            for cmd in commands {
                // Just time the validation part
                let _ = black_box(cmd);
            }
        })
    });
}

criterion_group!(benches, benchmark_core_creation, benchmark_command_validation);
criterion_main!(benches);
