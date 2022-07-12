//Base
use bevy::app::AppExit;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_config_cam::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
use std::time::Duration;
use rand::prelude::*;

mod app_state;
use app_state::transition;

mod camera;
use camera::main_cam;
use bevy_flycam::MovementSettings;
use bevy_flycam::PlayerPlugin;

fn main() {

    // stage for anything we want to do on a fixed timestep (ex. physics updates)
    let mut fixedupdate = SystemStage::parallel();
    fixedupdate.add_system(
        transition::spawn_sprite
            // only in-game!
            .run_in_state(transition::GameState::InGame)
            // only while the spacebar is pressed
            .run_if(transition::spacebar_pressed),
    );

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)

        .add_loopless_state(transition::GameState::MainMenu)
        // Add a FixedTimestep, cuz we can!
        .add_stage_before(
            CoreStage::Update,
            "FixedUpdate",
            FixedTimestepStage::from_stage(Duration::from_millis(125), fixedupdate),
        )
        // menu setup (state enter) systems
        .add_enter_system(transition::GameState::MainMenu, transition::setup_menu)
        // menu cleanup (state exit) systems
        .add_exit_system(transition::GameState::MainMenu, transition::despawn_with::<transition::MainMenu>)
        // game setup (state enter) systems
        .add_enter_system(transition::GameState::InGame, transition::setup_game_camera)

        //idk
        // .insert_resource(main_cam::InputState())
        // .insert_resource(main_cam::MovementSettings)
        // .add_enter_system(transition::GameState::InGame, main_cam::setup_player)
        // .add_enter_system(transition::GameState::InGame, main_cam::initial_grab_cursor)

        // game cleanup (state exit) systems
        .add_exit_system(transition::GameState::InGame, transition::despawn_with::<transition::MySprite>)
        .add_exit_system(transition::GameState::InGame, transition::despawn_with::<transition::GameCamera>)
        // menu stuff
        .add_system_set(
            ConditionSet::new()
                .run_in_state(transition::GameState::MainMenu)
                .with_system(exit_on_esc_system)
                .with_system(transition::butt_interact_visual)
                // our menu button handlers
                .with_system(transition::butt_exit.run_if(transition::on_butt_interact::<transition::ExitButt>))
                .with_system(transition::butt_game.run_if(transition::on_butt_interact::<transition::EnterButt>))
                .into()
        )
        // in-game stuff
        .add_system_set(
            ConditionSet::new()
                .run_in_state(transition::GameState::InGame)
                .with_system(transition::butt_interact_visual)
                .into()
        )

        // .add_system_set( //idk
        //     ConditionSet::new()
        //         .run_in_state(transition::GameState::InGame)
        //         .with_system(main_cam::player_move)
        //         .with_system(main_cam::player_look)
        //         .with_system(main_cam::cursor_grab)
        //         .into()
        // )

        // our other various systems:
        .add_system(transition::debug_current_state)

        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        
        //idk
        // .add_plugin(PlayerPlugin)
        // .insert_resource(MovementSettings {
        //     sensitivity: 0.00015, // default: 0.00012
        //     speed: 12.0,          // default: 12.0
        // })

        // setup our UI camera globally at startup and keep it alive at all times
        .add_startup_system(transition::setup_ui_camera)
        .run();
}

