use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
use rand::prelude::*;
use dolly::prelude::*;


// mod crate::camera;
// use crate::camera::main_cam; 

// Our Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
}

/// Marker for our in-game sprites
#[derive(Component)]
pub struct MySprite;

/// Marker for the main menu entity
#[derive(Component)]
pub struct MainMenu;

/// Marker for the main game camera entity
#[derive(Component)]
pub struct GameCamera;

/// Marker for the "Exit App" button
#[derive(Component)]
pub struct ExitButt;

/// Marker for the "Enter Game" button
#[derive(Component)]
pub struct EnterButt;

/// Transition back to menu on pressing Escape
pub fn back_to_menu_on_esc(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}

/// We can just access the `CurrentState`, and even use change detection!
pub fn debug_current_state(state: Res<CurrentState<GameState>>) {
    if state.is_changed() {
        println!("Detected state change to {:?}!", state);
    }
}

/// Condition system for holding the space bar
pub fn spacebar_pressed(kbd: Res<Input<KeyCode>>) -> bool {
    kbd.pressed(KeyCode::Space)
}

/// Despawn all entities with a given component type
pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// /// Spawn a MySprite entity
pub fn spawn_sprite(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("calling this ");
    let mut rng = thread_rng();
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgba(rng.gen(), rng.gen(), rng.gen(), 0.5).into()),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.25, 0.25, 0.25)) //half the cube size
        .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)))
        //.id()
        .insert(MySprite);
}

/// Spawn the UI camera
pub fn setup_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

/// Spawn the game camera
pub fn setup_game_camera(
    // mut app: App,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_3d())
        .insert(GameCamera);


    // plane
    //collider doesn't move as cube rotates when falling bc it hit smth
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(2.5, 0., 2.5));
}

/// Change button color on interaction
pub fn butt_interact_visual(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = UiColor(Color::rgb(0.75, 0.75, 0.75));
            }
            Interaction::Hovered => {
                *color = UiColor(Color::rgb(0.8, 0.8, 0.8));
            }
            Interaction::None => {
                *color = UiColor(Color::rgb(1.0, 1.0, 1.0));
            }
        }
    }
}

/// Condition to help with handling multiple buttons
///
/// Returns true when a button identified by a given component is clicked.
pub fn on_butt_interact<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

/// Handler for the Exit Game button
pub fn butt_exit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}

/// Handler for the Enter Game button
pub fn butt_game(mut commands: Commands) {
    // queue state transition
    commands.insert_resource(NextState(GameState::InGame));
}

/// Construct the main menu UI
pub fn setup_menu(mut commands: Commands, ass: Res<AssetServer>) {
    let butt_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: Rect::all(Val::Px(8.0)),
        margin: Rect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        ..Default::default()
    };

    let menu = commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::rgb(0.5, 0.5, 0.5)),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: Rect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                //align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenu)
        .id();

    let butt_enter = commands
        .spawn_bundle(ButtonBundle {
            style: butt_style.clone(),
            ..Default::default()
        })
        .insert(EnterButt)
        .id();

    let butt_exit = commands
        .spawn_bundle(ButtonBundle {
            style: butt_style.clone(),
            ..Default::default()
        })
        .insert(ExitButt)
        .id();

    commands
        .entity(menu)
        .push_children(&[butt_enter, butt_exit]);
}
