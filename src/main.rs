use raylib::prelude::*;
use chrono::prelude::*;

const STRAAL: f32 = 100.0;
const MIDDEN: Vector2 = Vector2::new(400.0, 400.0);
const LETTERGROOTTE: f32 = 30.0;

fn main () {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Hello, World")
        .build();
     
    while !rl.window_should_close() {
        let now = Local::now();


        let mut d = rl.begin_drawing(&thread); 
        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);


        d.draw_line_v(MIDDEN, MIDDEN + tijdseenheid_naar_coordinaat(now.hour12().1, 12, STRAAL / 1.5), Color::WHITE);
        d.draw_line_v(MIDDEN, MIDDEN + tijdseenheid_naar_coordinaat(now.minute(), 60, STRAAL), Color::WHITE);
        d.draw_line_v(MIDDEN, MIDDEN + tijdseenheid_naar_coordinaat(now.second() * 1000 + now.timestamp_subsec_millis(), 60 * 1000, STRAAL), Color::RED);



        for uur in 1..=12 {
            let target_coordinaat = MIDDEN + tijdseenheid_naar_coordinaat(uur, 12, STRAAL + LETTERGROOTTE);
            d.draw_text(&uur.to_string(), target_coordinaat.x as i32, target_coordinaat.y as i32, LETTERGROOTTE as i32, Color::WHITE);
        }

        for minuut in 1..=60 {
            let target_coordinaat = MIDDEN + tijdseenheid_naar_coordinaat(minuut, 60, STRAAL);
            d.draw_circle(target_coordinaat.x as i32, target_coordinaat.y as i32, 4.0, Color::WHITE);
        }
    }
}

fn tijdseenheid_naar_coordinaat (tijd: u32, tijdseenheid: u32, straal: f32) -> Vector2  {
    let t = ((tijd as f32) / (tijdseenheid as f32) - 0.25) * 2.0 * PI as f32;
    Vector2::new(t.cos(), t.sin()) * straal
}