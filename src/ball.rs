use bevy::math::bounding::{
    BoundingCircle,
    Aabb2d,
};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::circle_collision::{
    self,
    Collision,
};
use crate::{
    Position, Velocity, Shape
};

const BALL_SIZE: f32 = 5.;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    shape: Shape,
    velocity: Velocity,
    position: Position,
}

impl BallBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0., 0.)),
        }
    }
}

/// Spawn ball
/// 
/// 
pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::rgb(1., 0., 0.));
    
    // `Assets::add` will load these into memory and return a
    // `Handle` (an ID) to these assets. When all references
    // to this `Handle` are cleaned up the asset is cleaned up.
    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(color);
    
    // Here we are using `spawn` instead of `spawn_empty` 
    // followed by an `insert`. They mean the same thing, 
    // letting us spawn many components on a new entity at once.
    commands.spawn((
        BallBundle::new(1., 1.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        }
    ));
}

/// Move ball
/// 
/// 
pub fn move_ball(
    // Give me all positions that also contain a `Ball` component
    mut ball: Query<(&mut Position, &Velocity),
    With<Ball>>
) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0;
    }
}

/// Handle collisions
/// 
/// 
pub fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((
        mut ball_velocity,
        ball_position,
        ball_shape,
    )) = ball.get_single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = circle_collision::collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(
                    position.0,
                    shape.0 / 2.,
                )
            ) {
                match collision {
                    Collision::Left | Collision::Right => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Top | Collision::Bottom => {
                        ball_velocity.0.y *= -1.;
                    }
                }
            }
        }
    }
}
