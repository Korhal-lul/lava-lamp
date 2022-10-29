use rustbox::{Color, Event, InitOptions, Key, RustBox};
use std::fmt::format;

use glam::IVec2;
use rand::seq::SliceRandom;
use rand::Rng;

struct Ball {
    pos: IVec2,
    dir: IVec2,
}
struct Parameters {
    n_balls: usize,
    speed: i32,
    rim: i32,
    contained: i32,
    radius: f32,
    color: rustbox::Color,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            n_balls: 1,
            speed: 1,
            rim: 0,
            contained: 0,
            radius: 1.0,
            color: Color::Default,
        }
    }
}
fn main() {
    let parameters = Parameters::default();
    let rustbox = RustBox::init(InitOptions::default()).expect("Rust box failed to init!!");

    let max_x = rustbox.height() as i32;
    let max_y = rustbox.width() as i32 * 2;
    let radius = ((parameters.radius * parameters.radius) + (max_x * max_y) as f32) / 15000 as f32;
    let margin = if parameters.contained > 0 {
        (parameters.radius * 10.0).round() as i32
    } else {
        0
    };

    let mut balls = Vec::with_capacity(parameters.n_balls);

    let mut rng = rand::thread_rng();
    let possible_dir = [-1, 1];
    for _ in 0..parameters.n_balls {
        let pos = IVec2::new(
            rng.gen_range(0..max_x - margin),
            rng.gen_range(0..max_y - margin),
        );
        let dir = IVec2::new(
            *possible_dir.choose(&mut rng).unwrap(),
            *possible_dir.choose(&mut rng).unwrap(),
        );
        balls.push(Ball { pos, dir })
    }

    loop {
        //move balls

        for i in 0..parameters.n_balls {
            if balls[i].pos.x + balls[i].dir.x >= max_x - margin
                || balls[i].pos.x + balls[i].dir.x < margin
            {
                balls[i].dir.x *= -1;
            }
            if balls[i].pos.y + balls[i].dir.y >= max_y - margin
                || balls[i].pos.y + balls[i].dir.y < margin
            {
                balls[i].dir.y *= -1;
            }
            let dir = balls[i].dir;
            balls[i].pos += IVec2::new(1, 1);
            rustbox.print(
                balls[i].pos.x as usize,
                balls[i].pos.y as usize,
                rustbox::RB_BOLD,
                Color::Blue,
                Color::Default,
                &*format!("{:?}", balls[i].pos),
            );
        }
        let sum_const = 0.0225;
        let sum_const2 = sum_const * (1.0 + (0.25 * parameters.rim as f32));
        let color = parameters.color;
        for i in 0..max_x {
            for j in 0..max_y / 2 {
                let mut sum = [0.0, 0.0];

                for jj in 0..2usize {
                    for k in 0..parameters.n_balls {
                        let y = j * 2 + jj as i32;
                        sum[jj] += (radius * radius)
                            / ((i - balls[k].pos.x) * (i - balls[k].pos.x)
                                + (y - balls[k].pos.y) * (y - balls[k].pos.y))
                                as f32;
                    }
                }
                let x = i as usize;
                let y = j as usize;
                if sum[0] > sum_const {
                    if sum[1] > sum_const {
                        rustbox.print(
                            x,
                            y,
                            rustbox::RB_BOLD,
                            Color::White,
                            Color::Black,
                            &*format!("{:?}", sum),
                        );
                    } else {
                        rustbox.print(
                            x,
                            y,
                            rustbox::RB_BOLD,
                            Color::White,
                            Color::Black,
                            &*format!("X={x} Y={y}"),
                        );
                    }
                } else if sum[1] > sum_const {
                    rustbox.print(
                        x,
                        y,
                        rustbox::RB_BOLD,
                        Color::White,
                        Color::Black,
                        &*format!("X={x} Y={y}"),
                    );
                }
            }
        }
        rustbox.present();
        rustbox.clear();

        match rustbox.poll_event(false) {
            Ok(evt) => match evt {
                Event::KeyEvent(key) => match key {
                    Key::Esc => break,
                    Key::Char('q') => break,
                    Key::Ctrl('q') => break,
                    Key::Ctrl('c') => break,
                    _ => (),
                },
                _ => {}
            },
            Err(err) => panic!("Deu ruim no key event {err}"),
        };
    }
}
