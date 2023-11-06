use bevy::prelude::*;
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use player::PlayerPlugin;
use setup_game::SetupGamePlugin;

mod player;
mod setup_game;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_string(), // ToDo
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // The canvas size is constrained in index.html and build/web/styles.css
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::new(FixedUpdate))
        .add_plugins((SetupGamePlugin, PlayerPlugin))
        .run();
}
