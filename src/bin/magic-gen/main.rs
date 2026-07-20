#![allow(warnings)]
mod magic;

use flying_goose::{
    movement::sliders::defs::{BISHOP_SLIDER, ROOK_SLIDER, Slider}, types::{BitBoard, EMPTY_BITBOARD, print_bb}
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

use crate::magic::{RANDOM_SEED, get_slider_magics, print_magic_entries};
use flying_goose::movement::sliders::magics::MagicEntry;
use magic::print_magics;

fn main() {
    generate_magics(&ROOK_SLIDER, "rook");
    generate_magics(&BISHOP_SLIDER, "bishop");
}

fn generate_magics(slider: &Slider, slider_name: &str) -> () {
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    println!(
        "Generating {slider_name} magics across {num_threads} threads (one seed each), keeping the smallest table..."
    );
    println!("(only thread 0 prints per-square detail; the rest run silently)");
    let overall_start = std::time::Instant::now();

    // Draw one seed per thread up front, sequentially, from a single master RNG, so each
    // thread explores an independent (but reproducible) random stream.
    let mut master_rng = Pcg64::seed_from_u64(RANDOM_SEED);
    let thread_seeds: Vec<u64> = (0..num_threads).map(|_| master_rng.random()).collect();

    let best: Option<(Vec<MagicEntry>, Vec<BitBoard>)> = std::thread::scope(|scope| {
        let handles: Vec<_> = thread_seeds
            .into_iter()
            .enumerate()
            .map(|(thread_idx, seed)| {
                std::thread::Builder::new()
                    .stack_size(16 * 1024 * 1024)
                    .spawn_scoped(scope, move || {
                        let verbose = thread_idx == 0;
                        let thread_start = std::time::Instant::now();
                        let (magic_entries, slider_global_table) =
                            get_slider_magics(slider, seed, verbose);
                        let size_kb = (slider_global_table.len() * std::mem::size_of::<BitBoard>())
                            as f64
                            / 1024.0;
                        println!(
                            "[thread {thread_idx}] seed {seed}: table size {:.2} Kb (in {:.2?})",
                            size_kb,
                            thread_start.elapsed()
                        );
                        (magic_entries, slider_global_table)
                    })
                    .expect("failed to spawn worker thread")
            })
            .collect();

        let mut best: Option<(Vec<MagicEntry>, Vec<BitBoard>)> = None;
        for handle in handles {
            let candidate = handle.join().expect("worker thread panicked");
            let is_better = match &best {
                None => true,
                Some((_, best_table)) => candidate.1.len() < best_table.len(),
            };
            if is_better {
                best = Some(candidate);
            }
        }
        best
    });

    let (best_entries, best_table) = best.expect("num_threads is > 0, so best is always set");
    let size_kb = (best_table.len() * std::mem::size_of::<BitBoard>()) as f64 / 1024.0;
    println!(
        "Best {slider_name} global table: {:.2} Kb out of {num_threads} threads (total time {:.2?})",
        size_kb,
        overall_start.elapsed()
    );
    print_magic_entries(&best_entries, slider_name);
}
