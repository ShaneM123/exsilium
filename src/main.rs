use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{text::Text2dBounds};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(initial_setup)
        .run()


}

    fn initial_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
     commands.spawn(Camera2dBundle::default());

                //let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                let text_style = TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    font,
                };

            let home_tile =    commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::default().with_scale(Vec3::from_array([256.,256.,2.])),
                    material: materials.add(ColorMaterial::from(Color::NAVY)),
                    ..default()
                });

                let box_size = Vec2::new(300.0, 200.0);
                commands.spawn(Text2dBundle {
                    text: Text::from_section("Conscious City", text_style),
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: box_size,
                    },
                    visibility: Visibility::Visible,
                    transform: Transform::default().with_scale(Vec3::from_array([0.5,0.5,1.])).with_translation(Vec3::from_array([-50.0,100.,0.])),
                    ..default()
                });

    }
