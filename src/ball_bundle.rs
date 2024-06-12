use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::Position;

const BALL_SIZE: f32 = 5.;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    position: Position,
}

impl BallBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(x, y))
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
    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::rgb(1., 0., 0.));
    
    // `Assets::add` will load these into memory and return a
    // `Handle` (an ID) to these assets. When all references
    // to this `Handle` are cleaned up the asset is cleaned up.
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);
    
    // Here we are using `spawn` instead of `spawn_empty` 
    // followed by an `insert`. They mean the same thing, 
    // letting us spawn many components on a new entity at once.
    commands.spawn((
        BallBundle::new(0., 0.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        }
    ));
}
