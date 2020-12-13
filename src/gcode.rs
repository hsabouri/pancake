pub const init: &'static str = "
M82 ;absolute extrusion mode
;Begin Start Gcode for Dagoma DiscoUltimate
;Sliced: 03-10-2020 12:30:37
;Initial extruder: 0

G90 ;Absolute positioning
M106 S255 ;Fan on full
G28 X Y ;Home stop X Y
G1 X100 ;Centre back during cooldown in case of oozing
M109 R90 ;Cooldown in case too hot
G28 ;Centre
G29 ;Auto-level
M104 S215 ;Pre-heat
M107 ;Fan off
G0 X100 Y5 Z0.5 ;Front centre for degunk
M109 S215 ;Wait for initial temp
M83 ;E Relative
G1 E10 F200 ;Degunk
G1 E-3 F5000 ;Retract
G0 Z3 ;Withdraw
M82 ;E absolute
G92 E0 ;E reset
G1 F6000 ;Set feedrate
";

pub const init2: &'static str = "
G92 E0
G92 E0
G1 F3000 E-3.5
;LAYER_COUNT:9
;LAYER:0
M107
G0 F956.2 X81.405 Y69.576 Z0.26
G1 F1020
";

pub const end: &'static str = "
M106 S255 ;Fan on full
M104 S0 ;Cool hotend
M140 S0 ;Cool heated bed
G91 ;Relative positioning
G1 E-3 F5000 ;Retract filament to stop oozing
G0 Z+3 ;Withdraw
G90 ;Absolute positioning
G28 X Y ;Home
M109 R90 ;Wait until head has cooled to standby temp
M107 ;Fan off
M18 ;Stepper motors off

;Finish End Gcode for Dagoma DiscoUltimate

M82 ;absolute extrusion mode
M104 S0
";

// TODO: run a test with these settings

pub const FLOW: f32 = 0.05;

use super::Slice;
use super::{X, Y, Z};

fn get_distance(a: Vec4, b: Vec4) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    let z = a.z - b.z;

    (x * x + y * y + z * z).sqrt()
}

#[derive(Debug, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub e: f32,
}

#[derive(Debug, Clone)]
pub struct Printer {
    pub cur_pos: Vec4,
    pub offset: Vec4,
}

impl Printer {
    fn move_to(&mut self, x: f32, y: f32, z: f32, e: f32) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;
        self.cur_pos.z = z;
        self.cur_pos.e = e;

        println!("G1 X{} Y{} Z{} E{}", x, y, z, e);
    }

    fn move_by(&mut self, x: f32, y: f32, z: f32, e: f32) {
        self.move_to(
            self.cur_pos.x + x,
            self.cur_pos.y + y,
            self.cur_pos.z + z,
            self.cur_pos.e + e,
        );
    }

    fn print_to(&mut self, x: f32, y: f32, z: f32) {
        let e = get_distance(self.cur_pos.clone(), Vec4 {x, y, z, e: 0.0}) * FLOW;

        self.move_to(
            x,
            y,
            z,
            self.cur_pos.e + e,
        );
    }

    fn print_by(&mut self, x: f32, y: f32, z: f32) {
        let e = get_distance(
            Vec4 {x: 0.0, y: 0.0, z: 0.0, e: 0.0},
            Vec4 {x, y, z, e: 0.0}
        ) * FLOW;

        self.move_by(x, y, z, e);
    }

    pub fn print<T>(input: T, layer_height: f32) -> Option<()> 
        where T: Iterator<Item = Slice>
    {
        println!("{}", init);
        println!("{}", init2);

        let first_layer_height: f32 = layer_height / 2.0;

        let mut state = Printer {
            cur_pos: Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                e: 0.0,
            },
            offset: Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                e: 0.0,
            }
        };

        state.move_by(0.0, 0.0, 10.0, 0.0);
        state.move_to(100.0, 100.0, first_layer_height, 0.0);

        let mut first_draw = true;

        for (i, slice) in input.enumerate() {
            println!(";LAYER:{}", i + 1);
            for polygon in slice.polygons.into_iter() {
                for segment in polygon.into_iter() {
                    let first_point = segment.vertices[0];
                    let second_point = segment.vertices[1];

                    if first_draw {
                        state.offset = Vec4 {
                            x: first_point[X],
                            y: first_point[Y],
                            z: first_point[Z],
                            e: 0.0,
                        };
                    } else {
                        // TODO: possible inversion of figure here
                        state.move_by(
                            first_point[X] - state.offset.x,
                            first_point[Y] - state.offset.y,
                            first_point[Z] - state.offset.z,
                            0.0,
                        );
                        state.offset = Vec4 {
                            x: first_point[X],
                            y: first_point[Y],
                            z: first_point[Z],
                            e: 0.0,
                        };
                    }

                    state.print_by(
                        second_point[X] - state.offset.x,
                        second_point[Y] - state.offset.y,
                        second_point[Z] - state.offset.z,
                    );
                    state.offset = Vec4 {
                        x: second_point[X],
                        y: second_point[Y],
                        z: second_point[Z],
                        e: 0.0,
                    };

                    first_draw = false;
                }
            }
        }

        println!("{}", end);
        Some(())
    }
}










