use crate::components::*;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_ui_button);
    }
}

fn setup_ui_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buttons_query: Query<(Entity, &UiButton, &UiText, &Transform), Without<Children>>,
) {
    // Node の座標系への変換が難しすぎるので、text を付加するだけに留める
    for (entity, button, text, _transform) in &buttons_query {
        let font = asset_server.load("fonts/PeaberryMono.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 20.0,
            color: Color::WHITE,
        };
        let button_style = TextStyle {
            font: font.clone(),
            font_size: 24.0,
            color: Color::rgb_u8(0x24, 0x22, 0x34),
        };
        let text_alignment = TextAlignment::Center;

        commands.entity(entity).with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(&text.0, text_style).with_alignment(text_alignment),
                transform: Transform::from_translation(Vec3::new(0.0, -36.0, 1.0)),
                ..Default::default()
            });
            builder.spawn(Text2dBundle {
                text: Text::from_section(&button.0, button_style).with_alignment(text_alignment),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..Default::default()
            });
        });
    }
}
