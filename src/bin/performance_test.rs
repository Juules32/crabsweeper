use crabsweeper::{Generator, NaiveSolvableGenerator, OptimizedSolvableGenerator, Position};
use rand::Rng;
use rand_pcg::{Pcg64, rand_core::SeedableRng};
use std::{time::Instant, error::Error, fs};
use csv::Writer;
use uuid::Uuid;

const GRID_SIZES: [(usize, usize); 3] = [(9, 9), (16, 16), (30, 16)];
const GRID_NAMES: [&'static str; 3] = ["small", "medium", "large"];
const MAX_DURATION: u128 = 1000;
const GENERATORS: [&dyn Generator; 2] = [&NaiveSolvableGenerator, &OptimizedSolvableGenerator];

fn main() -> Result<(), Box<dyn Error>> {
    let id = Uuid::new_v4().to_string();
    let mut rng = Pcg64::seed_from_u64(12345);

    for i in 0..GRID_SIZES.len() {
        let grid_size = GRID_SIZES[i];
        let grid_name = GRID_NAMES[i];

        fs::create_dir_all(format!("performance_tests/{}", id))?;

        let mut wtr = Writer::from_path(&format!("performance_tests/{}/{}.csv", id, grid_name))?;

        wtr.write_record(["Number of Mines", "Time (ms)", "Generator Type"])?;

        for generator in GENERATORS {
            let mut stop = false;
            for n in 0..=99 {
                for _ in 0..10 {
                    let start = Instant::now();

                    generator.generate(
                        grid_size.0,
                        grid_size.1,
                        Position { x: grid_size.0 / 2, y: grid_size.1 / 2},
                        n,
                        rng.next_u64(),
                    );

                    let duration = start.elapsed().as_millis();

                    if duration >= MAX_DURATION {
                        stop = true;
                    }

                    wtr.write_record([n.to_string(), duration.to_string(), generator.name().into()])?;
                }

                if stop {
                    break;
                }
            }
        }
    }

    Ok(())
}
