use evongin::components::camera::Camera;
use evongin::components::colliders::circlecollider::CircleCollider;
use evongin::components::colliders::polycollider::PolyCollider;
use evongin::components::rigidbody::Rigidbody;
use evongin::components::scene::Scene;
use evongin::components::shapes::circle::Circle;
use evongin::components::shapes::rect::Rect;
use evongin::components::simulation::Simulation;
use evongin::components::transform::Transform;

use nalgebra::{Rotation2, Vector2};
use sdl2::pixels::Color;
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

    let player_rigidbody = Rigidbody::new(
        Vector2::new(0.0, 100.0),
        Vector2::new(0.0, 0.0),
        Box::new(CircleCollider::new(Box::new(player))),
    );

    let mut scene = Scene::new(Camera::new(
        sdl2::surface::Surface::new(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            sdl2::pixels::PixelFormatEnum::RGB24,
        )
        .unwrap(),
        Transform::default(),
    ));

    scene.add_object(Box::new(player_rigidbody));
    scene.add_collider(Box::new(ground_collider));
    scene.add_object(Box::new(ground));

    let mut simulation = Simulation::new(window, vec![scene], 0);

    simulation.run();
}
