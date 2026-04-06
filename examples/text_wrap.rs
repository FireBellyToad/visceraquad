use macroquad::prelude::*;

static LOREM: &str = "Lorem ipsum odor amet, consectetuer adipiscing elit. Ultrices nostra volutpat facilisis magna mus. Rhoncus tempor feugiat netus maecenas pretium leo vitae. Eros aliquet maecenas eu diam aliquet varius hac elementum. Sociosqu platea per ultricies vitae praesent mauris nostra ridiculus. Est cursus pulvinar efficitur mus vel leo. Integer et nec eleifend non leo. Lorem rutrum ultrices potenti facilisis hendrerit facilisi metus sit. AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA

Intentional newlines
are preserved.";

#[macroquad::main("Text Wrap")]
async fn main() {
    let font_size = 24;
    let mut alignment = TextAlignment::Left;
    loop {
        clear_background(BLACK);

        let maximum_line_length = f32::max(20.0, mouse_position().0 - 20.0);
        let text = wrap_text(LOREM, None, font_size, 1.0, maximum_line_length);
        let dimensions = measure_multiline_text(&text, None, font_size, 1.0, Some(1.0));

        draw_text(
            "Change text alignment with arrow keys",
            20.0,
            20.0,
            font_size as f32,
            RED,
        );
        draw_multiline_text_ex(
            &text,
            20.0,
            40.0 + dimensions.offset_y,
            Some(1.0),
            TextParams {
                font_size: font_size as u16,
                font_scale: 1.0,
                color: WHITE,
                alignment,
                ..Default::default()
            },
        );
        draw_rectangle_lines(20.0, 40.0, dimensions.width, dimensions.height, 2.0, BLUE);
        draw_line(
            20.0 + maximum_line_length,
            0.0,
            20.0 + maximum_line_length,
            screen_height(),
            1.0,
            RED,
        );

        match get_last_key_pressed() {
            Some(key) => {
                if key == KeyCode::Right {
                    alignment = TextAlignment::Right;
                } else if key == KeyCode::Up || key == KeyCode::Down {
                    alignment = TextAlignment::Center;
                } else if key == KeyCode::Left {
                    alignment = TextAlignment::Left;
                }
            }
            None => {}
        }

        next_frame().await
    }
}
