use std::env;
use std::fs::OpenOptions;
use stl_io::read_stl;

mod gcode;
mod math;
mod stage;
mod slice;

use gcode::Printer;
use math::{ X, Y, Z };
use slice::{Slice, IterSlices};
use stage::IterStages;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let mult: f32 = args.get(2).unwrap_or(&"1.0".to_string()).parse().unwrap_or(1.0);
    let layer_height = args.get(3).unwrap_or(&"0.1".to_string()).parse().unwrap_or(0.1);

    let mut file = OpenOptions::new().read(true).open(args[1].clone()).unwrap();
    let mut stl = read_stl(&mut file).unwrap();

    for vertice in stl.vertices.iter_mut() {
        vertice[X] = vertice[X] * mult;
        vertice[Y] = vertice[Y] * mult;
        vertice[Z] = vertice[Z] * mult;
    }

    let slices: Vec<Slice> = stl.iter_stages()
        .ok_or(())?
        .iter_slices(0.2)
        .ok_or(())?
        .collect();

    Printer::print(slices.into_iter(), layer_height);

    Ok(())
}
