use macroquad::color::colors;
use macroquad::prelude::*;

enum Places {
    Start,
    AtFork,
    AttackedByWolves,
    AttackedByBandits,
    DefeatedBandits,
    ShotByBandit,
    Destination,
}

const FONT_SIZE: u16 = 40;
fn word_wrap(text: &str, font: Option<&Font>, max_width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0;
    for word in text.split_whitespace() {
        let word_width = measure_text(word, font, FONT_SIZE, 1.0).width;
        if current_width + word_width > max_width {
            lines.push(current_line.clone());
            current_line.clear();
            current_width = 0.0;
        }
        current_line.push_str(word);
        current_line.push(' ');
        current_width += word_width;
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}

#[macroquad::main("RPG")]
async fn main() {
    let font = load_ttf_font("./assets/fonts/avenixel.ttf").await.unwrap();
    let mut text = "You are a Ranger, on your way to the annual Gathering. ".to_string();
    let mut place = Places::Start;
    let height = measure_text(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        Some(&font),
        FONT_SIZE,
        1.0,
    )
    .height;
    let mut show = 1;
    let speed = 0.08;
    let mut last_update = get_time();
    loop {
        if show == text.len() {
            match place {
                Places::Start => {
                    if is_key_down(KeyCode::Up) {
                        place = Places::AtFork;
                        text += "You start your journey. You are at a fork in the road. Do you go right or left? ";
                    }
                }
                Places::AtFork => {
                    if is_key_down(KeyCode::Right) {
                        place = Places::AttackedByWolves;
                        text+= "You take the right fork. You are attacked by a pack of ravenous wolves! You try to fight, but are overwhelmed and quickly devoured. Game Over. ";
                    }
                    if is_key_down(KeyCode::Left) {
                        place = Places::AttackedByBandits;
                        text +=
                        "You take the left fork. You are attacked by bandits! Do you run or fight? ";
                    }
                }
                Places::AttackedByWolves => {}
                Places::AttackedByBandits => {
                    if is_key_down(KeyCode::Right) {
                        place = Places::DefeatedBandits;
                        text +=
                        "You fight. The bandits are terrible fighters. You dispatch them easily. ";
                    }
                    if is_key_down(KeyCode::Left) {
                        place = Places::ShotByBandit;
                        text +=
                        "You try to run, but one of the bandits have a bow. One lucky shot and it's Game Over. ";
                    }
                }
                Places::DefeatedBandits => {
                    if is_key_down(KeyCode::Up) {
                        place = Places::Destination;
                        text += "You have arrived at your destination. You Win. ";
                    }
                }
                _ => {}
            }
        }
        if get_time() - last_update > speed {
            last_update = get_time();
            if show < text.len() {
                show += 1
            }
        }
        clear_background(colors::BLACK);
        draw_rectangle(50.0, 50.0, 700.0, 300.0, colors::DARKBLUE);
        let lines = word_wrap(&text[..show], Some(&font), 700.0);
        for (linenum, line) in lines.iter().enumerate() {
            draw_text_ex(
                line.as_str(),
                60.0,
                80.0 + ((height + 3.0) * linenum as f32),
                TextParams {
                    font: Some(&font),
                    font_size: 32,
                    ..Default::default()
                },
            );
        }
        next_frame().await
    }
}
