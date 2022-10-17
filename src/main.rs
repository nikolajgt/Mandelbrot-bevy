use bevy::{
    prelude::*,
    render::{render_resource::TextureUsages},
    render::render_resource::{Extent3d, TextureFormat, TextureDescriptor},
    DefaultPlugins,
};
use bevy_inspector_egui::WorldInspectorPlugin;
mod mandelbrot;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1920f32,
            height: 1080f32,
            title: "Mandelbrot".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_system(exit_system_esc)
        .add_startup_system(spawn_scene)
        .add_startup_system(spawn_camera)
        .run();
}



fn spawn_scene(
    mut commands: Commands, 
    mut assets: ResMut<Assets<Image>>,

) {
    let output: &str = "test";
    let width = 1280 ;
    let height = 720;
    let iterations = 1000;
    let zoom: f64 = 1.0;  //STANDARD 1.0
    let xmove: f64 = 1.0;  //STANDARD 1.0
    let ymove: f64 = 1.0;  //STANDARD 1.0
    let save_image = true;

    // let mandelbrot_native = mandelbrot::libnative::MandlebrotNative::new(w, h, max_iterations);
    let mandelbrot_new = mandelbrot::libnew::RunMandelbrot::init(output, width, height, iterations, save_image, zoom, xmove, ymove);

    let image = mandelbrot_new.new_algo();
    

    let image_handle = assets.add(image);

    // plane
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { color: Color::BISQUE, flip_x: false, flip_y:false, custom_size: Some(Vec2::new(1280.0, 720.0)), anchor: bevy::sprite::Anchor::Center },
        texture: image_handle.into(),
        ..Default::default()
    });

    println!("Done with added to plane");
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }).insert(Name::new("Light"));

}



fn spawn_camera(
    mut commands: Commands, 
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform {
            translation: Vec3::from((0f32, 4f32, 0f32)),
            rotation: Quat::from_rotation_x(-1.5),
            scale: Vec3::new(1.0, 1.0, 1.0)
        },
        ..default()

    });
}




