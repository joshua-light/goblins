mod model;
mod random;

use time::Instant;

use rand::prelude::*;
use rand_xoshiro::SplitMix64;

use crate::model::{Model, ModelOptions};

fn main() {
    const STEPS: usize = 20_000;

    let mut model = Model::new(ModelOptions {
        max_steps: STEPS,

        initial_capital: 1,
        income: 1,

        rng: SplitMix64::seed_from_u64(1),

        p_income: 0.8,
        p_birth: 0.8,
        p_death: 0.21,
    });

    let now = Instant::now();

    model.init();

    for _ in 0..(STEPS - 1) {
        model.sim();
    }

    let report = model.finish();
    let elapsed = now.elapsed();

    println!("---");
    report.print();
    report.draw();
    println!("---");
    println!("Duration: {} ms.", elapsed.whole_milliseconds());
}
