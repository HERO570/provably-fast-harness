// Diagnostic only: compare witness lookup combination with the already-used
// deferred dot accumulator. Not a prover patch or a whole-proof benchmark.
use std::hint::black_box;
use std::time::{Duration, Instant};
use stwo::core::fields::m31::{M31, P};
use stwo::core::fields::qm31::QM31;
use stwo::prover::backend::simd::dot::VeryPackedM31Dot;
use stwo::prover::backend::simd::m31::{N_LANES, PackedM31};
use stwo::prover::backend::simd::qm31::PackedQM31;
use stwo::prover::backend::simd::very_packed_m31::VeryPackedM31;
use stwo_constraint_framework::logup::LookupElements;

fn zero() -> PackedM31 { PackedM31::broadcast(M31(0)) }

#[inline(always)]
fn deferred(elements: &LookupElements<32>, values: &[PackedM31]) -> PackedQM31 {
    assert!(values.len() <= elements.alpha_powers.len());
    let start = usize::from(!values.is_empty()
        && elements.alpha_powers[0] == QM31::from_u32_unchecked(1, 0, 0, 0));
    let z = elements.z.to_m31_array();
    PackedQM31::from_packed_m31s(std::array::from_fn(|j| {
        let init = if start == 1 && j == 0 { values[0] } else { zero() };
        if values.len() <= start { return init - PackedM31::broadcast(z[j]); }
        let mut acc = VeryPackedM31Dot::new(VeryPackedM31::from_fn(|i| {
            if i == 0 { init } else { zero() }
        }));
        acc.add_products(values[start..].iter().zip(&elements.alpha_powers[start..]).map(|(v, c)| {
            (VeryPackedM31::from_fn(|i| if i == 0 { *v } else { zero() }), c.to_m31_array()[j])
        }));
        acc.reduce().0[0] - PackedM31::broadcast(z[j])
    }))
}

fn raw(value: PackedQM31) -> [[u32; 4]; N_LANES] {
    value.to_array().map(|v| v.to_m31_array().map(|x| x.0))
}

fn values(seed: u32) -> [PackedM31; 32] {
    std::array::from_fn(|col| PackedM31::from_array(std::array::from_fn(|lane| {
        let value = seed.wrapping_mul(1664525).wrapping_add(1013904223)
            .wrapping_add(col as u32 * 7919).wrapping_add(lane as u32 * 101);
        match (seed as usize + col + lane) % 13 {
            0 => M31(0), 1 => M31(P), 2 => M31(1), 3 => M31(P - 1),
            _ => M31(value % P),
        }
    })))
}

#[inline(never)]
fn kernel(rows: &[[PackedM31; 4]], elements: &LookupElements<32>, use_dot: bool) -> PackedQM31 {
    let elements = black_box(elements);
    let mut sum = PackedQM31::broadcast(QM31::from_u32_unchecked(0, 0, 0, 0));
    for row in rows {
        let value = if use_dot { deferred(elements, row) } else { elements.combine(row) };
        sum += value;
    }
    black_box(sum)
}

fn main() {
    let mut elements = LookupElements::<32>::dummy();
    let original = elements.clone();
    let mut checked = 0;
    for challenge in 0..8 {
        elements = original.clone();
        if challenge & 1 != 0 { elements.alpha_powers[0] = QM31::from_u32_unchecked(2, 3, 4, 5); }
        if challenge & 2 != 0 { elements.z = QM31::from_u32_unchecked(0, 0, 0, 0); }
        if challenge & 4 != 0 { elements.alpha_powers = [QM31::from_u32_unchecked(0, 0, 0, 0); 32]; }
        for seed in 0..128 {
            let row = values(seed);
            for len in [0, 1, 2, 4, 8, 16, 32] {
                let old: PackedQM31 = elements.combine(&row[..len]);
                let new = deferred(&elements, &row[..len]);
                assert_eq!(raw(old), raw(new), "challenge={challenge} seed={seed} len={len}");
                checked += 1;
            }
        }
    }
    println!("raw coordinate differential checks: PASS; {checked} cases, lengths0..32, zero/P/wrap/custom coefficients");
    if std::env::args().any(|arg| arg == "--check-only") { return; }
    let rows: Vec<_> = (0..65536).map(|seed| { let v = values(seed); [v[0], v[1], v[2], v[3]] }).collect();
    let elements = black_box(original);
    let mut baseline = Duration::ZERO;
    let mut dot = Duration::ZERO;
    for pair in 0..20 {
        for pass in 0..2 {
            let use_dot = (pair + pass) % 2 == 1;
            let start = Instant::now();
            black_box(kernel(&rows, &elements, use_dot));
            let elapsed = start.elapsed();
            if use_dot { dot += elapsed; } else { baseline += elapsed; }
        }
    }
    let old_ns = baseline.as_nanos() as f64 / 20.0 / rows.len() as f64;
    let new_ns = dot.as_nanos() as f64 / 20.0 / rows.len() as f64;
    println!("20 alternating kernel pairs, four-term lookups; baseline={old_ns:.3}ns/row deferred={new_ns:.3}ns/row ratio={:.6}", new_ns / old_ns);
    println!("Kernel screen only: no complete-proof or evaluator claim.");
}
