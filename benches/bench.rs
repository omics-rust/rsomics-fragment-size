use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_fragment_size(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-fragment-size");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bam = manifest.join("tests/golden/paired_pe.bam");

    c.bench_function("rsomics-fragment-size golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([bam.to_str().unwrap()])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_fragment_size);
criterion_main!(benches);
