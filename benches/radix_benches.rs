use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, BenchmarkGroup};


use radix::{ll_sort, vec_sort};
use criterion::measurement::WallTime;

fn sort_nums_with_vec(mut vec: Vec<u64>) {
    vec_sort(&mut vec);
}

fn sort_nums_with_linked_list(mut vec: Vec<u64>) {
    ll_sort(&mut vec);
}

fn add_bench(group: &mut BenchmarkGroup<WallTime>, n: usize) {
    let nums = radix::random_nums(n);
    group.bench_with_input(BenchmarkId::new("Vec", n), &nums,
                           |b, i| b.iter(|| sort_nums_with_vec(i.clone())));
    group.bench_with_input(BenchmarkId::new("Linked List", n), &nums,
                           |b, i| b.iter(|| sort_nums_with_linked_list(i.clone())));
}

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Radix Sort (Normal)");
    let sizes = [1000, 10_000, 100_000, 200_000, 300_000, 400_000, 500_000, 600_000, 700_000, 800_000, 900_000, 1_000_000];
    for &n in sizes.iter() {
        add_bench(&mut group, n);
    }
    group.finish();
    let mut group = c.benchmark_group("Radix Sort (Cache Defeated)");
    add_bench(&mut group, 2_000_000);
    add_bench(&mut group, 3_000_000);
    add_bench(&mut group, 4_000_000);
    group.finish();
}


criterion_group!(benches, bench_sorts);
criterion_main!(benches);