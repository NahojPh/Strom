use bevy::prelude::*;

use crate::enemy_util::Wave;

	
struct StatsText;

#[derive(Component)]
struct WaveText;


pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
		app
			.add_startup_system(UiPlugin::setup)	
			.add_system(UiPlugin::update_ui)
			;
    }
}

impl UiPlugin {
	fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	wave: Res<Wave>
	) {
		commands.spawn((TextBundle::from_section(
			format!("Wave: {}", wave.0),
			TextStyle {
			    font: asset_server.load("./fonts/Roboto-Black.ttf"),
			    font_size: 45.0,
			    color: Color::WHITE,
			},
		)
		.with_text_alignment(TextAlignment::CENTER)
		.with_style(Style {
			position_type: PositionType::Absolute,
			position: UiRect {
			    right: Val::Px(10.0),
			    bottom: Val::Px(15.0),
				..Default::default()
			},
			..Default::default()	
		}),
		WaveText,
		));
	
	}

	fn update_ui(
		mut wave_text_query: Query<&mut Text, With<WaveText>>,
		mut wave: ResMut<Wave>
	) {
		for mut text in wave_text_query.iter_mut() {
			text.sections[0].value = format!("Wave: {}", wave.0);
		}
		
		
	}
}







