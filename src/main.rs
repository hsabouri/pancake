use clap::{App, Arg};
use std::fs::OpenOptions;
use stl_io::read_stl;

pub mod ast;
mod gcode;
mod math;
mod slice;
mod stage;

use ast::{Axis, Transform};
use gcode::Printer;
use math::{Center, Displace, Homothety, RotateX, RotateY, RotateZ, Scale, X, Y, Z};
use slice::{IterSlices, Slice};
use stage::IterStages;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub transform);

fn transformations(mut stl: stl_io::IndexedMesh, raw: &str) -> stl_io::IndexedMesh {
    let transformations: Vec<Transform> = transform::TransformsParser::new().parse(raw).unwrap();

    for transform in transformations.into_iter() {
        stl = match transform {
            Transform::Rotate(Axis::X, theta) => stl.rotate_x(theta),
            Transform::Rotate(Axis::Y, theta) => stl.rotate_y(theta),
            Transform::Rotate(Axis::Z, theta) => stl.rotate_z(theta),
            Transform::Move(x, y, z) => stl.displace(x, y, z),
            Transform::Scale(x, y, z) => stl.scale(x, y, z),
            Transform::Homothety(v) => stl.homothety(v),
            Transform::Center => stl.center(),
            _ => stl,
        }
    }

    stl
}

fn main() -> anyhow::Result<()> {
    let matches = App::new("Pancake")
        .version("1.0")
        .author("Hugo S. <hsabouri@student.42.fr>")
        .about("Simple and fast 3D printing Slicer made in Rust")
        .arg(
            Arg::new("model")
                .required(true)
                .help(".stl file to slice")
                .value_name("MODEL"),
        )
        .arg(
            Arg::new("layer_height")
                .takes_value(true)
                .short('l')
                .long("layer-height")
                .default_value("0.1")
                .help("Layer height in millimeters"),
        )
        .arg(
            Arg::new("transform")
                .short('t')
                .long("transform")
                .takes_value(true)
                .help("Transform the model before slicing"),
        )
        .get_matches();

    let file_path = matches
        .value_of("model")
        .expect("Error: No .stl file. Expected: String");

    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();

    let layer_height: f64 = matches
        .value_of("layer_height")
        .unwrap_or(&"0.1".to_string())
        .parse()
        .expect("Error: Invalid layer_height. Expected: float");

    let mut stl = read_stl(&mut file).unwrap();

    if let Some(raw) = matches.value_of("transform") {
        stl = transformations(stl, raw);
    }

    let slices: Vec<Slice> = stl
        .iter_stages()
        .unwrap()
        .iter_slices(0.2)
        .unwrap()
        .collect();

    Printer::print(slices.into_iter(), layer_height);

    Ok(())
}
