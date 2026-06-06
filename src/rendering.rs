use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::grid::Grid;

const PIXEL_SCALE: f32 = 4.0;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_rendering);
        app.add_systems(Update, update_texture);
    }
}

#[derive(Resource)]
pub struct AutomataTexture(pub Handle<Image>);

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
        Transform::from_scale(Vec3::splat(PIXEL_SCALE)),
    ));
}

fn update_texture(
    grid: Res<Grid>,
    texture: Res<AutomataTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    if !grid.is_changed() {
        return;
    }

    let image = images
        .get_mut(&texture.0)
        .expect("Automata texture should exist");

    let data = image
        .data
        .as_mut()
        .expect("Image should have CPU-side data");

    for (i, cell) in grid.cells().iter().enumerate() {
        let pixel = cell.rgba();
        let offset = i * 4;

        data[offset] = pixel[0];
        data[offset + 1] = pixel[1];
        data[offset + 2] = pixel[2];
        data[offset + 3] = pixel[3];
    }
}
