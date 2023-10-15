use bevy::window::PrimaryWindow;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
const PLAYER_SIZE: f32 = 100.0;
const PLAYER_SPEED: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Juego de Prueba".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, confine_player))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::TEAL),
        },
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });

    let textura = asset_server.load("imagenes\\zorrito.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..default()
            },
            texture: textura,
            ..default()
        },
        Player {},
    ));
}

#[derive(Component)]
pub struct Player {}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        //Normalizo el vector para que no se mueva más rápido en diagonal
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

//Sistema que evita que el jugador se salga de la pantalla
pub fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        // Creo un vector con las coordenadas minimas y máximas para x e y
        let min_y_max = vec![
            0.0 + half_player_size,
            window.width() - (half_player_size),
            0.0 + half_player_size,
            window.height() - half_player_size,
        ];

        let mut translation = player_transform.translation;

        if translation.x < min_y_max[0] {
            translation.x = min_y_max[0];
        } else if translation.x > min_y_max[1] {
            translation.x = min_y_max[1];
        }

        if translation.y < min_y_max[2] {
            translation.y = min_y_max[2];
        } else if translation.y > min_y_max[3] {
            translation.y = min_y_max[3];
        }

        player_transform.translation = translation;
    }
}
