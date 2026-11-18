// Isolated arithmetic kernel probe for the 69645ee0 fixed-table pair formulas.
// Source references: crates/constraint-framework/src/logup.rs::LookupElements::combine;
// crates/prover/src/witness/components/range_check_20.rs and range_check_18.rs.
use std::hint::black_box;
use std::time::{Duration, Instant};

use stwo::core::fields::m31::{M31, P};
use stwo::core::fields::qm31::{QM31, SecureField};
use stwo::prover::backend::simd::m31::{PackedM31, N_LANES};
use stwo::prover::backend::simd::qm31::PackedQM31;
use stwo_constraint_framework::logup::LookupElements;

#[derive(Clone, Copy)]
struct Row {
    seq: PackedM31,
    mult0: PackedM31,
    mult1: PackedM31,
    id0: PackedM31,
    id1: PackedM31,
}

#[inline(never)]
fn baseline(rows: &[Row], lookup: &LookupElements<2>) -> (PackedQM31, PackedQM31) {
    let lookup = black_box(lookup);
    let mut sum_n = PackedQM31::broadcast(QM31::from_m31(M31(0), M31(0), M31(0), M31(0)));
    let mut sum_d = sum_n;
    for chunk in rows.chunks(65_536) {
      let (id0, id1) = (chunk[0].id0, chunk[0].id1);
      for row in chunk {
        let d0: PackedQM31 = lookup.combine(&[id0, row.seq]);
        let d1: PackedQM31 = lookup.combine(&[id1, row.seq]);
        let n = -(d0 * row.mult1 + d1 * row.mult0);
        let d = d0 * d1;
        sum_n += n;
        sum_d += d;
      }
    }
    black_box((sum_n, sum_d))
}

#[inline(never)]
fn delta_variant(rows: &[Row], lookup: &LookupElements<2>) -> (PackedQM31, PackedQM31) {
    let lookup = black_box(lookup);
    let mut sum_n = PackedQM31::broadcast(QM31::from_m31(M31(0), M31(0), M31(0), M31(0)));
    let mut sum_d = sum_n;
    for chunk in rows.chunks(65_536) {
      let (id0, id1) = (chunk[0].id0, chunk[0].id1);
      let zero = PackedM31::broadcast(M31(0));
      let right: PackedQM31 = lookup.combine(&[id1, zero]);
      let left: PackedQM31 = lookup.combine(&[id0, zero]);
      let delta = right - left;
      for row in chunk {
        let d0: PackedQM31 = lookup.combine(&[id0, row.seq]);
        let d1 = d0 + delta;
        let n = -(d0 * row.mult1 + d1 * row.mult0);
        let d = d0 * d1;
        sum_n += n;
        sum_d += d;
      }
    }
    black_box((sum_n, sum_d))
}

fn packed(seed: u32) -> PackedM31 {
    PackedM31::from_array(std::array::from_fn(|lane| {
        let v = (seed as u64 + lane as u64 * 7919) % P as u64;
        M31::from_u32_unchecked(v as u32)
    }))
}

fn challenge(seed: u32) -> LookupElements<2> {
    let f = |x: u32| M31::from_u32_unchecked(x % P);
    let alpha = QM31::from_m31(f(seed.wrapping_mul(17) + 3), f(seed + 5), f(seed + 7), f(seed + 11));
    let z = QM31::from_m31(f(seed + 13), f(seed + 19), f(seed + 23), f(seed + 29));
    LookupElements { z, alpha, alpha_powers: [SecureField::from_u32_unchecked(1, 0, 0, 0), alpha] }
}

fn rows() -> Vec<Row> {
    let rc20: [u32; 8] = [1410849886, 514232941, 531010560, 480677703,
        497455322, 447122465, 463900084, 682009131];
    let rc18: [u32; 2] = [1109051422, 1424798916];
    let mut out = Vec::with_capacity(278_528);
    for pair in rc20.chunks_exact(2) {
        for i in 0..65_536u32 {
            let seq = packed(i.wrapping_mul(16));
            let mult0 = packed(i.wrapping_mul(101).wrapping_add(7));
            let mult1 = packed(i.wrapping_mul(137).wrapping_add(13));
            let id0 = PackedM31::broadcast(M31::from_u32_unchecked(pair[0]));
            let id1_m31 = M31::from_u32_unchecked(pair[1]);
            let id1 = PackedM31::broadcast(id1_m31);
            out.push(Row { seq, mult0, mult1, id0, id1 });
        }
    }
    for i in 0..16_384u32 {
        let seq = packed(i.wrapping_mul(16));
        let mult0 = packed(i.wrapping_mul(211).wrapping_add(17));
        let mult1 = packed(i.wrapping_mul(251).wrapping_add(31));
        let id0 = PackedM31::broadcast(M31::from_u32_unchecked(rc18[0]));
        let id1_m31 = M31::from_u32_unchecked(rc18[1]);
        let id1 = PackedM31::broadcast(id1_m31);
        out.push(Row { seq, mult0, mult1, id0, id1 });
    }
    assert_eq!(out.len(), 278_528);
    out
}

fn main() {
    let rows = rows();
    let lookups: Vec<_> = (1..=4).map(challenge).collect();

    for lookup in &lookups {
        for row in &rows {
            let (n0, d0) = baseline(std::slice::from_ref(row), lookup);
            let (n1, d1) = delta_variant(std::slice::from_ref(row), lookup);
            assert_eq!(n0.to_array(), n1.to_array());
            assert_eq!(d0.to_array(), d1.to_array());
        }
    }
    println!("exact packed numerator and denominator pairs: PASS; rows={} challenges={}", rows.len(), lookups.len());
    if std::env::args().any(|arg| arg == "--check-only") { return; }

    let mut base_time = Duration::ZERO;
    let mut delta_time = Duration::ZERO;
    let mut base_out = PackedQM31::broadcast(QM31::from_m31(M31(0), M31(0), M31(0), M31(0)));
    let mut delta_out = base_out;
    const PAIRS: usize = 20;
    for round in 0..PAIRS {
      let lookup = &lookups[round % lookups.len()];
      for pass in 0..2 {
        let is_base = (round + pass) % 2 == 0;
        let start = Instant::now();
        let result = if is_base { baseline(&rows, lookup) } else { delta_variant(&rows, lookup) };
        let elapsed = start.elapsed();
        if is_base { base_time += elapsed; base_out = black_box(result.0 + result.1); }
        else { delta_time += elapsed; delta_out = black_box(result.0 + result.1); }
      }
    }
    black_box((base_out, delta_out, N_LANES));
    let base_ns = base_time.as_nanos() as f64 / PAIRS as f64 / rows.len() as f64;
    let delta_ns = delta_time.as_nanos() as f64 / PAIRS as f64 / rows.len() as f64;
    println!("averaged {} paired rounds, alternating order, same challenge within each pair; {} pair-rows/round", PAIRS, rows.len());
    println!("baseline two combines: total {:?}, {:.3} ns/pair", base_time, base_ns);
    println!("delta one combine:     total {:?}, {:.3} ns/pair", delta_time, delta_ns);
    println!("absolute kernel delta: {:.3} ns/pair; savings {:.3} ms per 278528 pairs", base_ns - delta_ns, (base_ns - delta_ns) * rows.len() as f64 / 1.0e6);
}
