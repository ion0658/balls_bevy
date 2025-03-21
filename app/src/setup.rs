use bevy::prelude::*;

pub(crate) fn generate_window_settings() -> WindowPlugin {
    const PRESENT_MODE: bevy::window::PresentMode = if cfg!(target_os = "windows") {
        bevy::window::PresentMode::Mailbox
    } else if cfg!(target_os = "macos") {
        bevy::window::PresentMode::Immediate
    } else {
        bevy::window::PresentMode::FifoRelaxed
    };
    WindowPlugin {
        primary_window: Some(Window {
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            resizable: false,
            present_mode: PRESENT_MODE,
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub(crate) fn generate_rendere_settings() -> bevy::render::RenderPlugin {
    const RENDER: Option<bevy::render::settings::Backends> = if cfg!(target_os = "macos") {
        Some(bevy::render::settings::Backends::METAL)
    } else if cfg!(target_os = "windows") {
        Some(bevy::render::settings::Backends::DX12)
    } else {
        None
    };
    bevy::render::RenderPlugin {
        render_creation: bevy::render::settings::RenderCreation::Automatic(
            bevy::render::settings::WgpuSettings {
                power_preference: bevy::render::settings::PowerPreference::HighPerformance,
                backends: RENDER,
                ..Default::default()
            },
        ),
        synchronous_pipeline_compilation: false,
    }
}
