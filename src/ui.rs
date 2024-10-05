use bevy::render::view::RenderLayers;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_vello::VelloPlugin;
use bevy_vello_graphics::VelloGraphicsPlugin;
use velyst::{prelude::*, typst_element::prelude::*, VelystPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VelloPlugin {
                canvas_render_layers: RenderLayers::layer(1),
                ..default()
            },
            VelloGraphicsPlugin,
            VelystPlugin::default(),
        ))
        .add_systems(Startup, spawn_ui_camera);
    }
}

/// Spawn camera specifically only for Ui rendering (render layer 1).
fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Ui Camera"),
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::NONE.into(),
                order: 1,
                ..default()
            },
            ..default()
        },
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
        RenderLayers::layer(1),
    ));
}

pub type InteractionQuery<'a, 'w, 's> =
    Query<'w, 's, (&'a Interaction, &'a TypstLabel), Changed<Interaction>>;

pub fn pressed<'a>(
    mut q_interactions: impl Iterator<Item = (&'a Interaction, &'a TypstLabel)>,
    label_str: &str,
) -> bool {
    q_interactions.any(|(interaction, label)| {
        label.as_str() == label_str && *interaction == Interaction::Pressed
    })
}

// pub fn hovered<'a>(
//     mut q_interactions: impl Iterator<Item = (&'a Interaction, &'a TypstLabel)>,
//     label_str: &str,
// ) -> bool {
//     q_interactions.any(|(interaction, label)| {
//         label.as_str() == label_str && *interaction == Interaction::Hovered
//     })
// }

pub fn windowed_func<F: WindowedFunc>(
    q_window: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
    mut func: ResMut<F>,
) {
    let Ok(window) = q_window.get_single() else {
        return;
    };

    func.set_width_height(window.width() as f64, window.height() as f64);
}

pub fn interactable_func<F: InteractableFunc>(
    q_interactions: Query<(&Interaction, &TypstLabel)>,
    mut func: ResMut<F>,
    time: Res<Time>,
    mut last_hovered: Local<Option<TypLabel>>,
    mut hovered_animation: Local<f64>,
) {
    let mut hovered_button = None;
    for (interaction, label) in q_interactions.iter() {
        if *interaction == Interaction::Hovered {
            hovered_button = Some(**label);
        }
    }

    if hovered_button != *last_hovered {
        *hovered_animation = 0.0;
        *last_hovered = hovered_button;
    }

    const SPEED: f64 = 6.0;
    // Clamp at 1.0
    *hovered_animation = f64::min(*hovered_animation + time.delta_seconds_f64() * SPEED, 1.0);

    func.hovered_button(hovered_button, *hovered_animation);
}

pub trait WindowedFunc: TypstFunc {
    fn set_width_height(&mut self, width: f64, height: f64);
}

pub trait InteractableFunc: TypstFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64);
}
