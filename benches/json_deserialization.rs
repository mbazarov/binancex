#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use std::{fs::File, io::Read, time::Duration};

use binance_schemes::spot::{general::ExchangeInfo, market::OrderBook};
use criterion::{criterion_group, BatchSize, BenchmarkId, Criterion, Throughput};

macro_rules! bench_file {
    ($name:ident, $type_name:ident, $batch_size:ident) => {
        fn $name(c: &mut Criterion) {
            let mut f =
                File::open(concat!("benches/json_dataset/", stringify!($name), ".json")).unwrap();
            let mut buf = Vec::new();
            f.read_to_end(&mut buf).unwrap();
            let buf = buf;

            let mut group = c.benchmark_group(stringify!($name));
            group.measurement_time(Duration::from_secs(20));
            group.throughput(Throughput::Bytes(buf.len() as u64));

            group.bench_with_input(
                BenchmarkId::new("simd-json::from_slice", stringify!($type_name)),
                &buf,
                |b, input| {
                    b.iter_batched(
                        || input.clone(),
                        |mut bytes| {
                            let _json: $type_name = simd_json::from_slice(&mut bytes).unwrap();
                        },
                        BatchSize::$batch_size,
                    )
                },
            );

            group.bench_with_input(
                BenchmarkId::new("serde_json::from_slice", stringify!($type_name)),
                &buf,
                |b, input| {
                    b.iter_batched(
                        || input.clone(),
                        |bytes| {
                            let _json: $type_name = serde_json::from_slice(&bytes).unwrap();
                        },
                        BatchSize::$batch_size,
                    )
                },
            );

            group.finish();
        }
    };
}

bench_file!(spot_general_exchange_info, ExchangeInfo, SmallInput);
bench_file!(spot_market_depth_btcusdt_100, OrderBook, SmallInput);
bench_file!(spot_market_depth_btcusdt_5000, OrderBook, SmallInput);

criterion_group!(
    benches,
    spot_general_exchange_info,
    spot_market_depth_btcusdt_100,
    spot_market_depth_btcusdt_5000,
);

fn main() {
    let num: usize = match std::env::var("CPU_AFFINITY") {
        Ok(var) => var.parse().unwrap(),
        Err(e) => panic!("$CPU_AFFINITY is not set ({})", e),
    };

    let cores = vec![num];
    affinity::set_thread_affinity(&cores).unwrap();

    benches();
    Criterion::default().configure_from_args().final_summary();
}
