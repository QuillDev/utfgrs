use bevy::prelude::App;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Query;
use bevy::prelude::With;

#[derive(Component)]
struct Entity;

#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_startup_system(populate_entites)
        .add_system(print_entity_positions)
        .add_system(print_entity_names)
        .run()
}

fn populate_entites(mut commands: Commands) {
    commands
        .spawn()
        .insert(Entity)
        .insert(Position { x: 0f32, y: 0f32 })
        .insert(Name("TestEntity".to_string()));
    commands
        .spawn()
        .insert(Entity)
        .insert(Position { x: 16f32, y: 0f32 });
    commands
        .spawn()
        .insert(Entity)
        .insert(Position { x: 32f32, y: 0f32 });
    commands
        .spawn()
        .insert(Entity)
        .insert(Position { x: 48f32, y: 0f32 });
}

fn print_entity_positions(query: Query<&Position, With<Entity>>) {
    for position in query.iter() {
        println!("x: {}, y: {:}", position.x, position.y)
    }
}

fn print_entity_names(query: Query<&Name, With<Entity>>) {
    for name in query.iter() {
        println!("name: {}", name.0)
    }
}
