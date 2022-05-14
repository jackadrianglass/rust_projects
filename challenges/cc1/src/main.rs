use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct GreetTimer(Timer);

/// This is a start up system that inserts a bunch of people into
/// the ECS for later querying
fn add_folks(mut cmds: Commands) {
    cmds.spawn()
        .insert(Person)
        .insert(Name("Jack Glass".to_string()));
    cmds.spawn()
        .insert(Person)
        .insert(Name("Zach Sims".to_string()));
    cmds.spawn()
        .insert(Person)
        .insert(Name("Jake Parente".to_string()));
    cmds.spawn()
        .insert(Person)
        .insert(Name("John Sebastien Schroh".to_string()));
}

fn hello_system(
    time: Res<Time>,                   // This is a resource that is given to us by bevy
    mut timer: ResMut<GreetTimer>,     // This is the timer that we inserted into the engine
    query: Query<&Name, With<Person>>, // This is a query that asks for all entities with a Name component and a Person component
) {
    if timer.0.tick(time.delta()).just_finished() {
        for Name { 0: name } in query.iter() {
            println!("Hello {}!", name);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_folks)
            .add_system(hello_system);
    }
}

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn_bundle(OrthographicCameraBundle::new_2d());
    cmds.spawn_bundle(SpriteBundle {
        texture: asset_server.load("gear.png"),
        transform: Transform::from_xyz(20.0, 20.0, 0.0).with_scale(Vec3::new(0.2, 0.2, 1.0)),
        ..default()
    })
    .insert(Velocity(Vec2::new(150.0, 100.0)));
}

#[derive(Component)]
struct Velocity(Vec2);

fn move_rect(time: Res<Time>, mut query: Query<(&mut Velocity, &mut Transform)>) {
    for (mut vel, mut t) in query.iter_mut() {
        t.translation.x += vel.0.x * time.delta_seconds();
        t.translation.y += vel.0.y * time.delta_seconds();

        if t.translation.x < -800.0 {
            t.translation.x = -800.0;
            vel.0.x *= -1.0;
        } else if t.translation.x > 800.0 {
            t.translation.x = 800.0;
            vel.0.x *= -1.0;
        }

        if t.translation.y < -400.0 {
            t.translation.y = -400.0;
            vel.0.y *= -1.0;
        } else if t.translation.y > 400.0 {
            t.translation.y = 400.0;
            vel.0.y *= -1.0;
        }
    }
}

fn ui(mut egui_context: ResMut<EguiContext>, mut query: Query<&mut Velocity>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
        for mut vel in query.iter_mut() {
            let mut x = vel.0.x.abs();
            let mut y = vel.0.y.abs();

            ui.add(egui::Slider::new(&mut x, 5.0..=500.0).text("X speed"));
            ui.add(egui::Slider::new(&mut y, 5.0..=500.0).text("Y speed"));

            vel.0.x = x * vel.0.x.signum();
            vel.0.y = y * vel.0.y.signum();

            ui.label(format!("Velocity [{}, {}]", vel.0.x, vel.0.y));
        }
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(move_rect)
        .add_system(ui)
        .add_plugin(HelloPlugin)
        .run();
}
