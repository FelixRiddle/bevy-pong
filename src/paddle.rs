use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;

use crate::{Position, Velocity, Shape};

pub const PADDLE_SPEED: f32 = 1.;
pub const PADDLE_WIDTH: f32 = 10.;
pub const PADDLE_HEIGHT: f32 = 50.;

#[derive(Component)]
pub struct Paddle;

#[derive(Bundle)]
pub struct PaddleBundle {
    paddle: Paddle,
    shape: Shape,
    position: Position,
    velocity: Velocity
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::new(0., 0.)),
        }
    }
}

/// Spawn paddles
/// 
/// 
pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;
        
        let mesh = Mesh::from(Rectangle::new(
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ));
        
        let mesh_handle = meshes.add(mesh);
        
        commands.spawn((
            // Player,
            PaddleBundle::new(right_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: materials.add(
                    ColorMaterial::from(Color::rgb(0., 0., 1.))
                ),
                ..default()
            }
        ));
        
        commands.spawn((
            // Ai,
            PaddleBundle::new(left_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: materials.add(
                    ColorMaterial::from(Color::rgb(0., 0., 1.))
                ),
                ..default()
            }
        ));
    }
    
    // let mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    // let material = ColorMaterial::from(Color::rgb(0., 1., 0.));
    
    // let mesh_handle = meshes.add(mesh);
    // let material_handle = materials.add(material);
    
    // commands.spawn((
    //     PaddleBundle::new(20., -25.),
    //     MaterialMesh2dBundle {
    //         mesh: mesh_handle.into(),
    //         material: material_handle,
    //         ..default()
    //     }
    // ));
}
