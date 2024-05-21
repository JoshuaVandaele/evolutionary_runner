use std::time::{Duration, Instant};

use evongin::components::colliders::circlecollider::CircleCollider;
use evongin::components::colliders::polycollider::PolyCollider;
use evongin::components::rigidbody::Rigidbody;
use evongin::components::shapes::circle::Circle;
use evongin::components::shapes::rect::Rect;
use evongin::components::transform::Transform;

use evongin::traits::collider::Collider;
use evongin::traits::physics::Physics;
use nalgebra::{Rotation2, Vector2};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem
        .window("Life", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let transform = Transform::new(
        Vector2::new(0.0, WINDOW_HEIGHT as f32 - 33.0),
        Vector2::new(1.0, 1.0),
        Rotation2::new(0.0),
    );

    let ground = Rect::new(
        transform,
        (WINDOW_WIDTH - 1) as f32,
        32.0,
        Color::RGB(0, 255, 0),
    );
    let ground_collider = PolyCollider::new(Box::new(ground));

    let player = Circle::new(
        Transform::new(
            Vector2::new(100.0, 100.0),
            Vector2::new(1.0, 1.0),
            Rotation2::new(0.0),
        ),
        32.0,
        Color::RGB(255, 0, 0),
    );

    let mut player_rigidbody = Rigidbody::new(
        Vector2::new(0.0, 100.0),
        Vector2::new(0.0, 0.0),
        Box::new(CircleCollider::new(Box::new(player))),
    );

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_instant = Instant::now();

    'running: loop {
        let delta_time = last_instant.elapsed().as_secs_f32();
        last_instant = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        ground_collider.get_shape().draw(&mut canvas);
        player_rigidbody.collider.get_shape().draw(&mut canvas);
        player_rigidbody.update(delta_time, &vec![Box::new(ground_collider.clone())]);

        canvas.present();
        ::std::thread::sleep(
            Duration::new(0, 1_000_000_000u32 / 60).saturating_sub(last_instant.elapsed()),
        );
    }
}
