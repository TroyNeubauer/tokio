use std::io::Read;

use bytes::BytesMut;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokio_util::codec::{Decoder, LinesCodec};

fn chuck_lines(mut text: BytesMut) {
    let mut codec = LinesCodec::new();
    while let Some(line) = codec.decode(&mut text).unwrap() {
        black_box(line);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut f = std::fs::File::open("shakespeare.txt").unwrap();
    let mut vec = Vec::new();
    f.read_to_end(&mut vec).unwrap();
    c.bench_function("chunk lines in shakespeare", |b| {
        b.iter(|| chuck_lines(vec.as_slice().into()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
