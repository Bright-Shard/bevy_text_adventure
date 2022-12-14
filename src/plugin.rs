use crate::input_output_manager::IOManager;
use crate::player::{Checkpoints, Inventory};
use crate::systems;
use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use bevy::prelude::MinimalPlugins;

/// Plugins needed by bevy_adventure.
pub struct AdventureDefaultPlugins;
impl PluginGroup for AdventureDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // Hierarchy for objects in the game
            .add(bevy::hierarchy::HierarchyPlugin::default())
            // Player input
            .add(bevy::input::InputPlugin::default())
    }

    fn name() -> String {
        String::from("BevyAdventurePlugins")
    }
}

/// The bevy_adventure plugin itself. This adds [AdventureDefaultPlugins] and [MinimalPlugins] to the app.
pub struct AdventurePlugin;
impl Plugin for AdventurePlugin {
    fn build(&self, app: &mut App) {
        let new_room_state = systems::NewRoomState::new(&mut app.world);
        // Add plugins
        app
            // Core Bevy plugins
            .add_plugins(MinimalPlugins)
            // Other plugins needed for adventure games
            .add_plugins(AdventureDefaultPlugins)
            // IOManager Resource
            .insert_resource(IOManager::new())
            // Inventory Resrouce
            .insert_resource(Inventory(Vec::new()))
            // Checkpoint resource
            .insert_resource(Checkpoints(Vec::new()))
            // Cached SystemStates
            .insert_resource(new_room_state);

        // Add systems
        systems::append_systems(app);
    }
}
