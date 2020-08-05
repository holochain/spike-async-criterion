use async_criterion::*;
use criterion::*;
use once_cell::sync::Lazy;

// let's register a lazy static tokio runtime
static TOKIO: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new()
        .threaded_scheduler()
        .build()
        .unwrap()
});

// execute `count` count parallel tokio tasks
fn exec_count(count: usize) {
    // we need to enter our tokio runtime to spawn tasks in it
    TOKIO.enter(move || {
        // collect our join handles in this vec
        let mut all = Vec::new();

        // spawn `count` tasks to execute in the tokio runtime
        for _ in 0..count {
            all.push(tokio::task::spawn(async move {
                assert_eq!(
                    43,
                    my_async_fn_that_does_work(black_box(42))
                        .await
                        .map_err(|_| ())?
                );
                let out: Result<(), ()> = Ok(());
                out
            }));
        }

        // synchronously wait on all our tasks to complete
        // this blocks criterion's exec thread, not any of tokio's task threads
        futures::executor::block_on(async move {
            futures::future::try_join_all(all)
                .await
                .map(|_| ())
                .map_err(|_| ())
        })
        .unwrap(); // unwrap here so we have no thread panics
    });
}

fn bench(c: &mut Criterion) {
    let cpu_count = num_cpus::get();
    let mut group = c.benchmark_group("async_criterion");
    group.bench_function("exec_one", |b| b.iter(|| exec_count(1)));
    group.bench_function("exec_cpu_count", |b| b.iter(|| exec_count(cpu_count)));
    group.bench_function("exec_twice_cpu_count", |b| {
        b.iter(|| exec_count(cpu_count * 2))
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
