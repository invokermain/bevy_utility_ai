use crate::logic::ai::HunterAI;
use crate::logic::components::Energy;
use crate::logic::food::Hunger;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::hierarchy::BuildChildren;
use bevy::prelude::*;
use bevy::ui::FlexDirection;
use bevy_utility_ai::AIMeta;

#[derive(Component)]
pub struct FpsRoot;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ActionText;

#[derive(Component)]
pub struct HungerText;

#[derive(Component)]
pub struct EnergyText;

pub fn setup_fps_counter(mut commands: Commands) {
    let default_text_style = TextStyle {
        font_size: 16.0,
        color: Color::WHITE,
        ..default()
    };
    // create our UI root node
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    width: Val::Px(200.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let text_fps = commands
        .spawn((
            FpsText,
            TextBundle {
                text: Text::from_section("FPS: N/A", default_text_style.clone()),
                ..default()
            },
        ))
        .id();

    let text_action = commands
        .spawn((
            ActionText,
            TextBundle {
                text: Text::from_section("Action: N/A", default_text_style.clone()),
                ..default()
            },
        ))
        .id();

    let text_hunger = commands
        .spawn((
            HungerText,
            TextBundle {
                text: Text::from_section("Hunger: N/A", default_text_style.clone()),
                ..default()
            },
        ))
        .id();

    let text_energy = commands
        .spawn((
            EnergyText,
            TextBundle {
                text: Text::from_section("Energy: N/A", default_text_style),
                ..default()
            },
        ))
        .id();

    commands.entity(root).push_children(&[
        text_fps,
        text_action,
        text_hunger,
        text_energy,
    ]);
}

pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections[0].value = format!("FPS: {value:>4.0}");
        } else {
            text.sections[0].value = "FPS: N/A".into();
        }
    }
}

pub fn action_text_update_system(
    q_hunter: Query<&AIMeta, With<HunterAI>>,
    mut q_text: Query<&mut Text, With<ActionText>>,
) {
    if let Ok(hunter_ai) = q_hunter.get_single() {
        for mut text in &mut q_text {
            let action_name = &hunter_ai.current_action_name;
            if !action_name.is_empty() {
                let action_text = match action_name.as_str() {
                    "ActionHunt" => "Hunting",
                    "ActionRest" => "Resting",
                    "ActionEat" => "Eating",
                    "ActionIdle" => "Idling",
                    _ => action_name,
                };
                text.sections[0].value = format!("Action: {action_text}");
            } else {
                text.sections[0].value = "Action: N/A".to_string();
            }
        }
    }
}

pub fn hunger_text_update_system(
    q_hunter: Query<&Hunger, With<HunterAI>>,
    mut q_text: Query<&mut Text, With<HungerText>>,
) {
    if let Ok(hunger) = q_hunter.get_single() {
        for mut text in &mut q_text {
            let value = 100.0 * hunger.value / hunger.max;
            text.sections[0].value = format!("Hunger: {value:.0}%");
        }
    }
}

pub fn energy_text_update_system(
    q_hunter: Query<&Energy, With<HunterAI>>,
    mut q_text: Query<&mut Text, With<EnergyText>>,
) {
    if let Ok(energy) = q_hunter.get_single() {
        for mut text in &mut q_text {
            let value = 100.0 * energy.value / energy.max;
            text.sections[0].value = format!("Energy: {value:.0}%");
        }
    }
}
