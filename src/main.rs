extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow;
use graphics::{
    clear, color, rectangle as rect, rectangle::square as sqr, text::Text, DrawState, Transformed,
};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings as Settings};
use piston::{
    event_loop::{EventLoop, EventSettings, Events},
    input::{Button, Key, PressEvent, RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use rand::{thread_rng as rgn, Rng};
use std::collections::VecDeque;

const OPENGL_VERSION: OpenGL = OpenGL::V4_5;
const TILE_SIZE: f64 = 20.0;
const COLS: i32 = 32;
const ROWS: i32 = 32;

fn main() {
    let black = color::hex("222222");
    let white = color::hex("ffffff");
    let red = color::hex("ff0000");

    let p = |n: i32| f64::from(n) * TILE_SIZE;
    let new_snake = || VecDeque::from(vec![(ROWS / 2, COLS / 2)]);
    let new_apple = || (rgn().gen_range(0, ROWS), rgn().gen_range(0, COLS));
    let new_game = || (false, (1, 0), (1, 0), new_snake(), new_apple(), 0);

    let mut window: GlutinWindow =
        WindowSettings::new("Cobra.rs", [p(ROWS) as u32, p(COLS) as u32])
            .opengl(OPENGL_VERSION)
            .exit_on_esc(true)
            .build()
            .expect("Unable create the window");

    let mut gc = GlyphCache::new("./assets/PxPlus_IBM_VGA8.ttf", (), Settings::new()).expect("Unable to load font");
    let mut gl = GlGraphics::new(OPENGL_VERSION);
    let mut events = Events::new(EventSettings::new()).ups(8);
    let (mut game_over, mut direction, mut last_direction, mut snake, mut apple, mut score) = new_game();

    while let Some(event) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            direction = match key {
                Key::Up if last_direction != (0, 1) => (0, -1),
                Key::Down if last_direction != (0, -1) => (0, 1),
                Key::Left if last_direction != (1, 0) => (-1, 0),
                Key::Right if last_direction != (-1, 0) => (1, 0),
                _ => last_direction,
            };

            if game_over && key == Key::Space {
                apple = new_apple();
                snake = new_snake();
                direction = (1, 0);
                game_over = false;
                score = 0;
            }
        }

        if let Some(_u) = event.update_args() {
            if !game_over {
                let mut head = *snake.front().expect("Unable to get the snake head");
                let prev_head = head.clone();

                head.0 += direction.0;
                head.1 += direction.1;
                last_direction = direction;

                if apple.0 == head.0 && apple.1 == head.1 {
                    snake.push_back(prev_head);
                    apple = new_apple();
                    score += 1;
                }

                if head.0 == -1
                    || head.0 == COLS
                    || head.1 == -1
                    || head.1 == ROWS
                    || snake.iter().any(|b| head.0 == b.0 && head.1 == b.1)
                {
                    game_over = true;
                } else {
                    snake.pop_back();
                    snake.push_front(head);
                }
            }
        }

        if let Some(r) = event.render_args() {
            clear(black, &mut gl);

            gl.draw(r.viewport(), |c, gl| {
                if game_over {
                    Text::new_color(white, 32)
                        .draw(
                            "YOU DIED!",
                            &mut gc,
                            &DrawState::default(),
                            c.transform.trans(230.0, 320.0),
                            gl,
                        )
                        .expect("Unable to draw the game over text");
                }

                Text::new_color(white, 12)
                    .draw(
                        &format!("Score: {}", &score),
                        &mut gc,
                        &DrawState::default(),
                        c.transform.trans(8.0, 20.0),
                        gl,
                    )
                    .expect("Unable to draw the score text");

                snake
                    .iter()
                    .map(|b| sqr(p(b.0), p(b.1), TILE_SIZE))
                    .for_each(|s| rect(white, s, c.transform, gl));

                rect(red, sqr(p(apple.0), p(apple.1), TILE_SIZE), c.transform, gl);
            })
        }
    }
}
