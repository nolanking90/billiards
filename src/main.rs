mod billiard;
use billiard::Billiard;

mod util;
use util::*;

use bevy::{color::palettes::css::*, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_prototype_lyon::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use std::f64::consts::PI;

#[derive(Component)]
struct Collision;

#[derive(Default, Resource)]
struct UiState {
    s_initial: f32,
    s_final: f32,
    delta: f32,
    num_collisions: i32,
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .init_resource::<UiState>()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_plugins(EguiPlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_ui)
        .run();
}

fn build_collisions(billiard: &Billiard) -> (ShapeBundle, Collision) {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::from_array(coord_to_pixel(f64to32(
        billiard.z_inside(&billiard.s_initial()),
    ))));

    let points: Vec<Vec2> = billiard
        .minimize_length()
        .iter()
        .map(|&x| f64to32(x))
        .map(coord_to_pixel)
        .map(Vec2::from_array)
        .collect();

    for pt in points {
        path_builder.line_to(pt);
        path_builder.move_to(pt);
    }

    if billiard.num_collisions() % 2 == 1 {
        path_builder.line_to(Vec2::from_array(coord_to_pixel(f64to32(
            billiard.z_inside(&billiard.s_final()),
        ))));
    } else {
        path_builder.line_to(Vec2::from_array(coord_to_pixel(f64to32(
            billiard.z_outside(&billiard.s_final()),
        ))));

    }

    path_builder.close();
    let path = path_builder.build();

    (
        ShapeBundle {
            path,
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
            ..default()
        },
        Collision,
    )
}

fn build_inside(billiard: &Billiard) -> ShapeBundle {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::from_array(coord_to_pixel(f64to32(
        billiard.z_inside(&0.0),
    ))));

    for i in 0..=1000 {
        let s = (i as f64) * (2.0 * PI / 1000.0);
        let point = Vec2::from_array(coord_to_pixel(f64to32(billiard.z_inside(&s))));
        path_builder.line_to(point);
        path_builder.move_to(point);
    }

    path_builder.close();
    let path = path_builder.build();

    ShapeBundle {
        path,
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        ..default()
    }
}

fn build_outside(billiard: &Billiard) -> ShapeBundle {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::from_array(coord_to_pixel(f64to32(
        billiard.z_outside(&0.0),
    ))));

    for i in 0..=1000 {
        let s = (i as f64) * (2.0 * PI / 1000.0);
        let point = Vec2::from_array(coord_to_pixel(f64to32(billiard.z_outside(&s))));
        path_builder.line_to(point);
        path_builder.move_to(point);
    }

    path_builder.close();
    let path = path_builder.build();

    ShapeBundle {
        path,
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        ..default()
    }
}

fn setup(mut commands: Commands, mut ui_state: ResMut<UiState>) {
    commands.spawn(Camera2dBundle::default());

    ui_state.s_initial = 1.0;
    ui_state.s_final = 2.0;
    ui_state.num_collisions = 10;
    ui_state.delta = 0.1;

    let s_initial = ui_state.s_initial as f64;
    let s_final = ui_state.s_final as f64;
    let num_collisions = ui_state.num_collisions as i64;
    let delta = ui_state.delta as f64;

    let billiard = Billiard::new(s_initial, s_final, num_collisions, delta);

    commands.spawn((build_outside(&billiard), Stroke::new(BLACK, 5.)));
    commands.spawn((build_inside(&billiard), Stroke::new(BLACK, 5.)));
    commands.spawn((build_collisions(&billiard), Stroke::new(RED, 2.5)));
}

fn update_ui(
    mut ui_state: ResMut<UiState>,
    mut query: Query<&mut Path, With<Collision>>,
    mut contexts: EguiContexts,
) {
    let s_initial = ui_state.s_initial as f64;
    let s_final = ui_state.s_final as f64;
    let num_collisions = ui_state.num_collisions as i64;
    let delta = ui_state.delta as f64;

    egui::SidePanel::left("my_left_panel").show(contexts.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut ui_state.s_initial, 0.5..=6.0).text("s initial"));
        ui.add(egui::Slider::new(&mut ui_state.s_final, 1.0..=6.0).text("s final"));
        ui.add(egui::Slider::new(&mut ui_state.num_collisions, 4..=100).text("Collisions"));
    });

    let billiard = Billiard::new(s_initial, s_final, num_collisions, delta);

    let mut path = query.single_mut();
    *path = build_collisions(&billiard).0.path;
}
