use bevy::prelude::*;

#[derive(Resource)]
pub struct AppProperties {
    ui_properties: UIProperties,
    physics_properties: PhysicsProperties,
}

impl Default for AppProperties {
    fn default() -> Self {
        Self {
            ui_properties: UIProperties::new(50f32, 0.1f32),
            physics_properties: PhysicsProperties::new(
                Vec2::new(0f32, -9.81f32),
                CollisionProperties::new(0f32, 0.1f32),
                CollisionProperties::new(1f32, 0.1f32),
            ),
        }
    }
}

impl AppProperties {
    pub fn new(ui_properties: UIProperties, physics_properties: PhysicsProperties) -> Self {
        Self {
            ui_properties,
            physics_properties,
        }
    }
    pub const fn get_ui_properties(&self) -> &UIProperties {
        &self.ui_properties
    }
    pub const fn get_physics_properties(&self) -> &PhysicsProperties {
        &self.physics_properties
    }
}

pub struct UIProperties {
    window_margin: f32,
    border_width: f32,
}

impl UIProperties {
    pub fn new(window_margin: f32, border_width: f32) -> Self {
        Self {
            window_margin,
            border_width,
        }
    }

    pub const fn get_window_margin(&self) -> f32 {
        self.window_margin
    }

    pub const fn get_border_width(&self) -> f32 {
        self.border_width
    }
}

pub struct PhysicsProperties {
    gravity: Vec2,
    wall_properties: CollisionProperties,
    ball_properties: CollisionProperties,
}

impl PhysicsProperties {
    pub fn new(
        gravity: Vec2,
        wall_properties: CollisionProperties,
        ball_properties: CollisionProperties,
    ) -> Self {
        Self {
            gravity,
            wall_properties,
            ball_properties,
        }
    }
    pub const fn get_gravity(&self) -> Vec2 {
        self.gravity
    }
    pub const fn get_wall_properties(&self) -> &CollisionProperties {
        &self.wall_properties
    }
    pub const fn get_ball_properties(&self) -> &CollisionProperties {
        &self.ball_properties
    }
}

pub struct CollisionProperties {
    friction: f32,
    restitution: f32,
}

impl CollisionProperties {
    pub fn new(friction: f32, restitution: f32) -> Self {
        Self {
            friction,
            restitution,
        }
    }

    pub const fn get_friction(&self) -> f32 {
        self.friction
    }

    pub const fn get_restitution(&self) -> f32 {
        self.restitution
    }
}
