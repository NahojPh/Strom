use bevy::prelude::*;

use crate::{enemy_util::Wave, AppState};

#[derive(Component)]
struct WaveText;

#[derive(Component)]
struct MainMenuText;



pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
		app
			.add_system(UiPlugin::setup.in_schedule(OnEnter(AppState::MainMenu)))	
			.add_system(UiPlugin::update_ingame_ui.in_set(OnUpdate(AppState::InGame)))
			.add_system(UiPlugin::render_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
			.add_system(UiPlugin::start_game_input_handler.in_set(OnUpdate(AppState::MainMenu)))
			.add_system(UiPlugin::clear_on_exit_main_menu.in_schedule(OnExit(AppState::MainMenu)))
			.add_system(UiPlugin::clear_on_exit_in_game.in_schedule(OnExit(AppState::InGame)))
			;
    }
}

impl UiPlugin {
	fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	) {
		commands.spawn((TextBundle::from_section(
			"Wave: 0",
			TextStyle {
			    font: asset_server.load("./fonts/Roboto-Black.ttf"),
			    font_size: 45.0,
			    color: Color::WHITE,
			},
		)
		.with_text_alignment(TextAlignment::Center)
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

	fn clear_on_exit_main_menu(
		mut commands: Commands,
		query: Query<Entity, With<MainMenuText>>,
		
	) {
		for entity in query.iter() {
			commands.entity(entity).despawn_recursive();
		}
		
	}
	
	fn clear_on_exit_in_game(
		mut commands: Commands,
		query: Query<Entity, With<WaveText>>,
		
	) {
		for entity in query.iter() {
			commands.entity(entity).despawn_recursive();
		}
		
	}

	fn update_ingame_ui(
		mut wave_text_query: Query<&mut Text, With<WaveText>>,
		wave: ResMut<Wave>
	) {
		for mut text in wave_text_query.iter_mut() {
			text.sections[0].value = format!("Wave: {}", wave.0);
		}
	}

	fn start_game_input_handler(
		mut next_state: ResMut<NextState<AppState>>,
        input: Res<Input<KeyCode>>,
	) {
		match input.get_just_released().next() {
		    Some(_) => next_state.set(AppState::InGame),
		    None => {},
		}
		
	}
	
	fn render_main_menu(
		mut commands: Commands,
		asset_server: Res<AssetServer>,	
		mut wave: ResMut<Wave>,
	) {
		commands.spawn((TextBundle::from_section(
			if wave.0 > 0 {format!("Last game ended on wave {}", wave.0)} else {format!("")},
			TextStyle {
			    font: asset_server.load("./fonts/Roboto-Black.ttf"),
			    font_size: 50.0,
			    color: Color::WHITE,
			},
		)
		.with_text_alignment(TextAlignment::Center)
		.with_style(Style {
			position_type: PositionType::Absolute,
			position: UiRect {
			    right: Val::Percent(20.0),
			    bottom: Val::Percent(60.0),
				..Default::default()
			},
			..Default::default()	
		}),
		MainMenuText,
		));

		commands.spawn((TextBundle::from_section(
			"Press any key to start the game",
			TextStyle {
			    font: asset_server.load("./fonts/Roboto-Black.ttf"),
			    font_size: 50.0,
			    color: Color::WHITE,
			},
		)
		.with_text_alignment(TextAlignment::Center)
		.with_style(Style {
			position_type: PositionType::Absolute,
			position: UiRect {
			    right: Val::Percent(20.0),
			    bottom: Val::Percent(40.0),
				..Default::default()
			},
			..Default::default()	
		}),
		MainMenuText,
		));
	

		wave.0 = 0;
	}
}







