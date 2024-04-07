use macroquad::color::colors;
use macroquad::prelude::*;

const font_size: u16 = 40;
fn word_wrap(text: &str, font: Option<&Font>, max_width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0;
    for word in text.split_whitespace() {
        let word_width = measure_text(word, font, font_size, 1.0).width;
        if current_width + word_width > max_width {
            lines.push(current_line.clone());
            current_line.clear();
            current_width = 0.0;
        }
        current_line.push_str(word);
        current_line.push(' ');
        current_width += word_width;
        println!("{}\t{}", word, current_width);
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}

#[macroquad::main("RPG")]
async fn main() {
    let font = load_ttf_font("./assets/fonts/avenixel.ttf").await.unwrap();
    let lines = word_wrap(
        "You are a Ranger, on your way to the annual Gathering. You are at a fork in the road. Do you go right or left?",
        Some(&font),
        700.0,
    );
    let height = measure_text(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        Some(&font),
        font_size,
        1.0,
    )
    .height;
    println!("{:#?}", lines);
    loop {
        clear_background(colors::BLACK);
        draw_rectangle(50.0, 50.0, 700.0, 300.0, colors::DARKBLUE);
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
