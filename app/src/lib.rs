#![windows_subsystem = "windows"]

mod setup;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PIXELS_PER_METER: f32 = 100f32;

#[derive(Component)]
pub struct Ball {
    pub life: usize,
}

#[derive(Component)]
pub struct Wall;

#[derive(Resource)]
struct WindowSize {
    pub width: f32,
    pub height: f32,
}

pub fn run() -> anyhow::Result<()> {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(setup::generate_window_settings())
                .set(setup::generate_rendere_settings()),
        )
        .add_plugins(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER)
                .in_schedule(FixedPreUpdate),
        )
        .add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<properties::AppProperties>()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_window)
        .add_systems(Startup, setup_rapier_gravity)
        .add_systems(PostStartup, setup_world)
        .add_systems(Update, handle_click)
        .add_observer(spawn_ball_handler)
        .run();
    Ok(())
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_window(
    mut commands: Commands,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Some(primary_window) = windows.iter().next() else {
        return;
    };
    let window_width = primary_window.width();
    let window_height = primary_window.height();
    commands.insert_resource(WindowSize {
        width: window_width,
        height: window_height,
    });
}

fn setup_rapier_gravity(
    mut rapier_config: Query<&mut RapierConfiguration>,
    properties: Res<properties::AppProperties>,
) {
    rapier_config.par_iter_mut().for_each(|mut conf| {
        conf.gravity = properties.get_physics_properties().get_gravity() * PIXELS_PER_METER;
    });
}

fn setup_world(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    properties: Res<properties::AppProperties>,
) {
    let world_width =
        (window_size.width - properties.get_ui_properties().get_window_margin()) / 2f32;
    let world_height =
        (window_size.height - properties.get_ui_properties().get_window_margin()) / 2f32;
    /* Create the walls. */
    commands.spawn(Wall).insert((
        RigidBody::Fixed,
        Collider::cuboid(
            world_width,
            properties.get_ui_properties().get_border_width(),
        ),
        Friction::coefficient(
            properties
                .get_physics_properties()
                .get_wall_properties()
                .get_friction(),
        ),
        Restitution::coefficient(
            properties
                .get_physics_properties()
                .get_wall_properties()
                .get_restitution(),
        ),
        Transform::from_xyz(0f32, -world_height, 0f32),
    ));
    commands.spawn(Wall).insert((
        RigidBody::Fixed,
        Collider::cuboid(
            world_width,
            properties.get_ui_properties().get_border_width(),
        ),
        Sensor,
        Friction::coefficient(
            properties
                .get_physics_properties()
                .get_wall_properties()
                .get_friction(),
        ),
        Restitution::coefficient(
            properties
                .get_physics_properties()
                .get_wall_properties()
                .get_restitution(),
        ),
        Transform::from_xyz(0f32, world_height * 0.6, 0f32),
    ));
    commands.spawn(Wall).insert((
        RigidBody::Fixed,
        Collider::cuboid(
            properties.get_ui_properties().get_border_width(),
            world_height * 0.8,
        ),
        Friction::coefficient(
            properties
                .get_physics_properties()
                .get_wall_properties()
                .get_friction(),
        ),
        Restitution::coefficient(
            properties
                .get_physics_properties()
                .get_wall_properties()
                .get_restitution(),
        ),
        Transform::from_xyz(-world_width, -world_height * 0.2, 0f32),
    ));
    commands.spawn(Wall).insert((
        RigidBody::Fixed,
        Collider::cuboid(
            properties.get_ui_properties().get_border_width(),
            world_height * 0.8,
        ),
        Friction::coefficient(
            properties
                .get_physics_properties()
                .get_wall_properties()
                .get_friction(),
        ),
        Restitution::coefficient(
            properties
                .get_physics_properties()
                .get_wall_properties()
                .get_restitution(),
        ),
        Transform::from_xyz(world_width, -world_height * 0.2, 0f32),
    ));
}

#[derive(Event)]
struct BallSpawnEvent {
    pos: Vec2,
    r: f32,
}

fn handle_click(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Single<&Window>,
    window_size: Res<WindowSize>,
    properties: Res<properties::AppProperties>,
) {
    let (camera, camera_transform) = *camera;
    let world_width =
        (window_size.width - properties.get_ui_properties().get_window_margin()) / 2f32;
    let world_height =
        (window_size.height - properties.get_ui_properties().get_window_margin()) / 2f32;
    let min_r = world_width.min(world_height) / 40f32;
    let max_r = world_width.min(world_height) / 20f32;
    let r = rand::random_range(min_r..max_r);

    let Some(pos) = windows
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world(camera_transform, cursor_pos).ok())
        .map(|ray| ray.origin.truncate())
        .filter(|p| p.x.abs() < (world_width - r))
        .map(|mut p| {
            p.y = world_height * 0.6 + r * 2f32;
            p
        })
    else {
        return;
    };
    if mouse_button_input.just_released(MouseButton::Left) {
        commands.trigger(BallSpawnEvent { pos, r });
    }
}

fn spawn_ball_handler(
    ball_spawn_events: Trigger<BallSpawnEvent>,
    mut commands: Commands,
    properties: Res<properties::AppProperties>,
) {
    /* Create the bouncing ball. */
    let friction = Friction {
        coefficient: properties
            .get_physics_properties()
            .get_ball_properties()
            .get_friction(),
        combine_rule: CoefficientCombineRule::Min,
    };
    let restitution = Restitution {
        coefficient: properties
            .get_physics_properties()
            .get_ball_properties()
            .get_restitution(),
        combine_rule: CoefficientCombineRule::Max,
    };
    commands.spawn(Ball { life: 10 }).insert((
        RigidBody::Dynamic,
        Collider::ball(ball_spawn_events.r),
        Ccd::enabled(),
        ActiveEvents::COLLISION_EVENTS,
        friction,
        restitution,
        bevy_rapier2d::dynamics::Velocity {
            linvel: Vec2::ZERO,
            angvel: 0f32,
        },
        Transform::from_xyz(ball_spawn_events.pos.x, ball_spawn_events.pos.y, 0f32),
    ));
}
