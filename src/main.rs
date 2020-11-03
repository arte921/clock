use raylib::prelude::*;
use chrono::prelude::*;

struct Wijzer {
    eenheid: u32,
    lengte: f32,
    kleur: Color
}

const STRAAL: f32 = 400.0;
const MARGE: f32 = 0.05 * STRAAL;
const MIDDEN: Vector2 = Vector2::new(STRAAL + MARGE, STRAAL + MARGE);

// 86400000 per dag
const WIJZERS: [Wijzer; 3] = [
    Wijzer {eenheid: 43200000,  lengte: 0.4, kleur: Color::WHITE}, // uren
    Wijzer {eenheid: 3600000,   lengte: 0.7, kleur: Color::BLUE }, // minuten
    Wijzer {eenheid: 60000,     lengte: 0.8, kleur: Color::RED  }, // seconden
];

fn main () {
    let (mut rl, thread) = raylib::init()
        .size(2 * (MARGE + STRAAL) as i32, 2 * (MARGE + STRAAL) as i32)
        .title("haha rust go 🅱️rrrr")
        .build();
     
    while !rl.window_should_close() {
        let now = Local::now();
        let milliseconden = now.num_seconds_from_midnight() * 1000 + now.timestamp_subsec_millis();

        let mut d = rl.begin_drawing(&thread); 
        d.clear_background(Color::BLACK);
        d.draw_fps(0, 0);

        for wijzer in WIJZERS.iter() {
            teken_wijzer(milliseconden % wijzer.eenheid, wijzer.eenheid, wijzer.lengte * STRAAL, wijzer.kleur, MIDDEN, &mut d);
        }

        // teken tekst en langere lijntjes voor uren
        for uur in 1..=12 {
            let binnenste_coordinaat_lijn = tijdseenheid_naar_coordinaat(uur, 12, STRAAL, MIDDEN);
            let buitenste_coordinaat_lijn = tijdseenheid_naar_coordinaat(uur, 12, STRAAL * 0.8, MIDDEN);
            d.draw_line_v(binnenste_coordinaat_lijn, buitenste_coordinaat_lijn, Color::WHITE);
        }

        // teken bolletjes voor minuten/seconden
        for i in 1..=60 {
            let binnenste_coordinaat_lijn = tijdseenheid_naar_coordinaat(i, 60, STRAAL, MIDDEN);
            let buitenste_coordinaat_lijn = tijdseenheid_naar_coordinaat(i, 60, STRAAL * 0.9, MIDDEN);
            d.draw_line_v(binnenste_coordinaat_lijn, buitenste_coordinaat_lijn, Color::WHITE);
        }
    }
}

fn teken_wijzer (tijd: u32, tijdseenheid: u32, straal: f32, kleur: Color, midden: Vector2, d: &mut RaylibDrawHandle) {
    d.draw_line_v(midden, tijdseenheid_naar_coordinaat(tijd, tijdseenheid, straal, midden), kleur);
}

// zet een tijdseenheid, zoals uren/minuten/seconden om naar een vector met een rotatie, geschaald naar wijzerlengte
fn tijdseenheid_naar_coordinaat (tijd: u32, tijdseenheid: u32, straal: f32, toevoeging: Vector2) -> Vector2  {
    let t = ((tijd as f32) / (tijdseenheid as f32) - 0.25) * 2.0 * PI as f32;
    Vector2::new(t.cos(), t.sin()) * straal + toevoeging
}