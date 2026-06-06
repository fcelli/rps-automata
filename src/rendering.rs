use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::simulation::Grid;

#[derive(Resource)]
pub struct AutomataTexture(pub Handle<Image>);

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_rendering);
    }
}

fn setup_rendering(mut commands: Commands, mut images: ResMut<Assets<Image>>, grid: Res<Grid>) {
    commands.spawn(Camera2d);

    let image = Image::new_fill(
        Extent3d {
            width: grid.width as u32,
            height: grid.height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    let image_handle = images.add(image);

    commands.insert_resource(AutomataTexture(image_handle.clone()));

    commands.spawn((
        Sprite::from_image(image_handle),
        Transform::from_scale(Vec3::splat(4.0)),
    ));
}
