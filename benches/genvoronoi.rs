use rand::prelude::*;
use voronoi::{voronoi, Cell, Point};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const BOX_SIZE: f64 = 800.;

fn generate_points(count: usize) -> Vec<Point> {
    let mut rng = thread_rng();

    (0..count)
        .map(|_| Point::rand(&mut rng) * BOX_SIZE)
        .collect()
}

fn bench_voronoi(c: &mut Criterion) {
    let cell = Cell::new(BOX_SIZE);
    let mut group = c.benchmark_group("points");

    for &num_points in [1, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_points),
            &generate_points(num_points),
            |b, points| b.iter(|| voronoi(points.clone(), &cell)),
        );
    }
}

criterion_group!(benches, bench_voronoi);
criterion_main!(benches);
