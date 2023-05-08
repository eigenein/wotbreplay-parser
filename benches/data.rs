use std::fs::File;

use criterion::{criterion_group, criterion_main, Criterion};
use wotbreplay_parser::replay::Replay;

fn bench_read_data(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("read_data");
    for path in [
        "replays/20230429_0126__helaas_pindakaas.wotbreplay",
        "replays/20230503_0027__helaas_pindakaas.wotbreplay",
        "replays/20230429_1682194799_malinovka.wotbreplay",
    ] {
        let mut replay = Replay::open(File::open(path).unwrap()).unwrap();
        group.bench_function(path, |bencher| bencher.iter(|| replay.read_data().unwrap()));
    }
    group.finish();
}

criterion_group!(benches, bench_read_data);
criterion_main!(benches);
