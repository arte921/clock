use raylib::prelude::*;
use chrono::prelude::*;

struct Wizer {
    unit: u32,
    length: f32,
    color: Color,
    thiccness: f32
}

const RAY: f32 = 400.0;
const MARGIN: f32 = 0.05 * RAY;
const MIDDLE: Vector2 = Vector2::new(RAY + MARGIN, RAY + MARGIN);
const SCREENSIZE: i32 = 2 * (MARGIN + RAY) as i32;

// 86400000 per day
const WISERS: [Wizer; 3] = [
    // hours
    Wizer {
        unit: 43200000,
        length: 0.4,
        color: Color::WHITE,
        thiccness: 3.0
    },
    // minutes
    Wizer { 
        unit: 3600000,
        length: 0.7,
        color: Color::DARKBLUE,
        thiccness: 2.0
    },
    // seconds
    Wizer { 
        unit: 60000,
        length: 0.8,
        color: Color::RED,
        thiccness: 1.0
    },
];

fn main () {
    let (mut rl, thread) = raylib::init()
        .size(SCREENSIZE, SCREENSIZE)
        .title("haha rust go 🅱️rrrr")
        .build();
     
    while !rl.window_should_close() {
        let now = Local::now();
        let milliseconds = now.num_seconds_from_midnight() * 1000 + now.timestamp_subsec_millis();

        let mut d = rl.begin_drawing(&thread); 
        d.clear_background(Color::BLACK);

        for wiser in WISERS.iter() {
            d.draw_line_ex(
                MIDDLE,
                time_unit_to_coordinate(
                    milliseconds % wiser.unit,
                    wiser.unit,
                    wiser.length * RAY,
                    MIDDLE
                ),
                wiser.thiccness,
                wiser.color
            );
        }

        // draw short lines for minutes/seconds
        for i in 1..=60 {
            let inner_coordinate_line = time_unit_to_coordinate(i, 60, RAY * 0.9, MIDDLE);
            let outer_coordinate_line = time_unit_to_coordinate(i, 60, RAY, MIDDLE);
            d.draw_line_ex(inner_coordinate_line, outer_coordinate_line, 3.0, Color::BLUE);
        }

        // draw longer lines for hours
        for hour in 1..=12 {
            let inner_coordinate_line = time_unit_to_coordinate(hour, 12, RAY, MIDDLE);
            let outer_coordinate_line = time_unit_to_coordinate(hour, 12, RAY * 0.8, MIDDLE);
            d.draw_line_ex(inner_coordinate_line, outer_coordinate_line, 4.0, Color::GREEN);
        }
    }
}

// converts time units, such as hours / minutes / seconds to a vector with a rotation, scaled to wiser length
fn time_unit_to_coordinate (time: u32, time_unit: u32, ray: f32, addition: Vector2) -> Vector2  {
    let t = ((time as f32) / (time_unit as f32) - 0.25) * 2.0 * PI as f32;
    Vector2::new(t.cos(), t.sin()) * ray + addition
}
