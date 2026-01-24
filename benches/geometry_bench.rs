//! Benchmark suite for geometric operations
//!
//! This module benchmarks the performance of various geometric operations
//! using the criterion library.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use projgeom_rs::*;

fn bench_dot_product(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_product");

    group.bench_function("basic", |b| {
        b.iter(|| {
            projgeom_rs::pg_object::dot_product(
                black_box(&[1, 2, 3]),
                black_box(&[4, 5, 6])
            )
        })
    });

    group.bench_function("large_values", |b| {
        b.iter(|| {
            projgeom_rs::pg_object::dot_product(
                black_box(&[1000, 2000, 3000]),
                black_box(&[4000, 5000, 6000])
            )
        })
    });

    group.finish();
}

fn bench_cross_product(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_product");

    group.bench_function("basic", |b| {
        b.iter(|| {
            projgeom_rs::pg_object::cross_product(
                black_box(&[1, 2, 3]),
                black_box(&[4, 5, 6])
            )
        })
    });

    group.bench_function("large_values", |b| {
        b.iter(|| {
            projgeom_rs::pg_object::cross_product(
                black_box(&[1000, 2000, 3000]),
                black_box(&[4000, 5000, 6000])
            )
        })
    });

    group.finish();
}

fn bench_point_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("point_creation");

    group.bench_function("pg_point", |b| {
        b.iter(|| PgPoint::new(black_box([1, 2, 3])))
    });

    group.bench_function("elliptic_point", |b| {
        b.iter(|| EllipticPoint::new(black_box([1, 2, 3])))
    });

    group.bench_function("hyperbolic_point", |b| {
        b.iter(|| HyperbolicPoint::new(black_box([1, 2, 3])))
    });

    group.bench_function("euclid_point", |b| {
        b.iter(|| EuclidPoint::new(black_box([1, 2, 3])))
    });

    group.finish();
}

fn bench_meet(c: &mut Criterion) {
    let mut group = c.benchmark_group("meet");

    let p1 = PgPoint::new([1, 2, 3]);
    let p2 = PgPoint::new([4, 5, 6]);

    group.bench_function("pg_points", |b| {
        b.iter(|| black_box(p1).meet(black_box(&p2)))
    });

    let l1 = PgLine::new([1, 0, 0]);
    let l2 = PgLine::new([0, 1, 0]);

    group.bench_function("pg_lines", |b| {
        b.iter(|| black_box(l1).meet(black_box(&l2)))
    });

    group.finish();
}

fn bench_incident(c: &mut Criterion) {
    let mut group = c.benchmark_group("incident");

    let p = PgPoint::new([1, 2, 3]);
    let l = PgLine::new([4, 5, 6]);

    group.bench_function("point_line", |b| {
        b.iter(|| black_box(p).incident(black_box(&l)))
    });

    group.finish();
}

fn bench_parametrize(c: &mut Criterion) {
    let mut group = c.benchmark_group("parametrize");

    let p1 = PgPoint::new([1, 2, 3]);
    let p2 = PgPoint::new([4, 5, 6]);

    group.bench_function("basic", |b| {
        b.iter(|| black_box(p1).parametrize(black_box(2), black_box(&p2), black_box(3)))
    });

    group.finish();
}

fn bench_perp(c: &mut Criterion) {
    let mut group = c.benchmark_group("perpendicular");

    let p_ell = EllipticPoint::new([1, 2, 3]);
    let p_hyp = HyperbolicPoint::new([1, 2, 3]);
    let p_euc = EuclidPoint::new([1, 2, 1]);

    group.bench_function("elliptic", |b| {
        b.iter(|| black_box(p_ell).perp())
    });

    group.bench_function("hyperbolic", |b| {
        b.iter(|| black_box(p_hyp).perp())
    });

    group.bench_function("euclidean", |b| {
        b.iter(|| black_box(p_euc).perp())
    });

    group.finish();
}

fn bench_harmonic_conjugate(c: &mut Criterion) {
    let mut group = c.benchmark_group("harmonic_conjugate");

    let a = PgPoint::new([1, 0, 0]);
    let b = PgPoint::new([0, 1, 0]);
    let c = PgPoint::new([1, 1, 0]);

    group.bench_function("basic", |b| {
        b.iter(|| harm_conj(black_box(&a), black_box(&b), black_box(&c)))
    });

    group.finish();
}

fn bench_normalization(c: &mut Criterion) {
    let mut group = c.benchmark_group("normalization");

    group.bench_function("basic", |b| {
        let mut coord = [2, 4, 6];
        b.iter(|| normalize_homogeneous(black_box(&mut coord)))
    });

    group.bench_function("large_values", |b| {
        let mut coord = [1000, 2000, 3000];
        b.iter(|| normalize_homogeneous(black_box(&mut coord)))
    });

    group.finish();
}

fn bench_orientation(c: &mut Criterion) {
    let mut group = c.benchmark_group("orientation");

    let p1 = PgPoint::new([0, 0, 1]);
    let p2 = PgPoint::new([1, 0, 1]);
    let p3 = PgPoint::new([0, 1, 1]);

    group.bench_function("basic", |b| {
        b.iter(|| orientation(black_box(&p1), black_box(&p2), black_box(&p3)))
    });

    group.finish();
}

fn bench_triangle_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("triangle_operations");

    let triangle = [
        EllipticPoint::new([1, 0, 0]),
        EllipticPoint::new([0, 1, 0]),
        EllipticPoint::new([0, 0, 1]),
    ];

    group.bench_function("orthocenter", |b| {
        b.iter(|| orthocenter(black_box(&triangle)))
    });

    group.bench_function("tri_altitude", |b| {
        b.iter(|| tri_altitude(black_box(&triangle)))
    });

    group.bench_function("tri_dual", |b| {
        b.iter(|| tri_dual(black_box(&triangle)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_dot_product,
    bench_cross_product,
    bench_point_creation,
    bench_meet,
    bench_incident,
    bench_parametrize,
    bench_perp,
    bench_harmonic_conjugate,
    bench_normalization,
    bench_orientation,
    bench_triangle_operations
);
criterion_main!(benches);