use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::{PrimaryWindow, WindowResized},
};

use crate::automata::Grid;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_rendering);
        app.add_systems(Update, (resize_automata, update_texture));
    }
}

#[derive(Resource)]
struct AutomataTexture(pub Handle<Image>);

#[derive(Component)]
struct AutomataSprite;

fn setup_rendering(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    grid: Res<Grid>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let window = windows.single().unwrap();

    let (grid_width, grid_height) = grid.dimensions();
    let pixel_scale = f32::min(
        window.width() / grid_width as f32,
        window.height() / grid_height as f32,
    );

    let image = Image::new_fill(
        Extent3d {
            width: grid_width as u32,
            height: grid_height as u32,
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
        Transform::from_scale(Vec3::splat(pixel_scale)),
        AutomataSprite,
    ));
}

fn resize_automata(
    mut resize_events: MessageReader<WindowResized>,
    grid: Res<Grid>,
    mut sprite: Query<&mut Transform, With<AutomataSprite>>,
) {
    let Some(event) = resize_events.read().last() else {
        return;
    };

    let (grid_width, grid_height) = grid.dimensions();

    let pixel_scale = f32::min(
        event.width / grid_width as f32,
        event.height / grid_height as f32,
    );

    sprite.single_mut().unwrap().scale = Vec3::splat(pixel_scale);
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

    for (i, cell) in grid.cells().enumerate() {
        let pixel = cell.rgba();
        let offset = i * 4;

        data[offset] = pixel[0];
        data[offset + 1] = pixel[1];
        data[offset + 2] = pixel[2];
        data[offset + 3] = pixel[3];
    }
}
