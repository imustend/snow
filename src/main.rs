use std::path::Path;

use snowfall::Snowfall;

mod snowfall;

fn main() {
    let mut snow = Snowfall::new(Path::new("snow.txt"), 100);
    snow.simulate();
}
