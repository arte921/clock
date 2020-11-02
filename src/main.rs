use raylib::prelude::*;
use chrono::prelude::*;

const STRAAL: f32 = 100.0;
const MIDDEN: Vector2 = Vector2::new(200.0, 200.0);
const MILLISMIDDEN: Vector2 = Vector2::new(MIDDEN.x + STRAAL, MIDDEN.y - STRAAL - LETTERGROOTTE);
const LETTERGROOTTE: f32 = 30.0;

fn main () {
    let (mut rl, thread) = raylib::init()
        .size(400, 400)
        .title("haha rust go 🅱️rrrr")
        .build();
     
    while !rl.window_should_close() {
        let now = Local::now();

        let mut d = rl.begin_drawing(&thread); 
        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);

        // uren
        teken_wijzer(now.hour12().1, 12, STRAAL / 1.5, Color::WHITE, MIDDEN, &mut d);
        // minuten
        teken_wijzer(now.minute(), 60, STRAAL, Color::WHITE, MIDDEN, &mut d);
        // seconden
        teken_wijzer(now.second(), 60, STRAAL, Color::RED, MIDDEN, &mut d);
        // milliseconden
        teken_wijzer(now.timestamp_subsec_millis(), 1000, 20.0, Color::WHITE, MILLISMIDDEN, &mut d);

        // teken tekst en langere lijntjes voor uren
        for uur in 1..=12 {
            let target_coordinaat_tekst = tijdseenheid_naar_coordinaat(uur, 12, STRAAL + LETTERGROOTTE * 1.5, MIDDEN) - LETTERGROOTTE / 3.0;
            d.draw_text(&uur.to_string(), target_coordinaat_tekst.x as i32, target_coordinaat_tekst.y as i32, LETTERGROOTTE as i32, Color::WHITE);

            let binnenste_coordinaat_lijn = tijdseenheid_naar_coordinaat(uur, 12, STRAAL + 10.0, MIDDEN);
            let buitenste_coordinaat_lijn = tijdseenheid_naar_coordinaat(uur, 12, STRAAL + 30.0, MIDDEN);
            d.draw_line_v(binnenste_coordinaat_lijn, buitenste_coordinaat_lijn, Color::WHITE);

        }

        // teken bolletjes voor minuten/seconden
        for i in 1..=60 {
            // let target_coordinaat = tijdseenheid_naar_coordinaat(i, 60, STRAAL + 10.0, MIDDEN);
            // d.draw_circle(target_coordinaat.x as i32, target_coordinaat.y as i32, 4.0, Color::WHITE);

            let binnenste_coordinaat_lijn = tijdseenheid_naar_coordinaat(i, 60, STRAAL + 20.0, MIDDEN);
            let buitenste_coordinaat_lijn = tijdseenheid_naar_coordinaat(i, 60, STRAAL + 30.0, MIDDEN);
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