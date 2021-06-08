use bevy::prelude::*;

struct Person;
struct Name(String);
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system())
            .add_system(hello_world.system())
            .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn hello_world() {
    println!("Hello World");
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("France Preseren".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Valentin Vodnik".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Joze Plecnik".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}
