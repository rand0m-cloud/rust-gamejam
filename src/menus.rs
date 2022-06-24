//Explicit paths because kayak uses all the same names
use bevy::{
    core::Name,
    prelude::{default, AssetServer, Commands, Plugin, Res, ResMut, State, SystemSet},
};
use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{
        render,
        styles::{Corner, Edge, LayoutType, Style, StyleProp, Units},
        Color, EventType, Index, OnEvent,
    },
    widgets,
};

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_startup_system(setup_kayak)
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy_ui));
    }
}

fn destroy_ui(mut commands: Commands) {
    commands.remove_resource::<BevyContext>();
}

fn setup_kayak(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(UICameraBundle::new())
        .insert(Name::new("UI Camera"));
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));
}

fn spawn_main_menu(mut commands: Commands) {
    let context = BevyContext::new(|context| {
        let container_style = Style {
            layout_type: StyleProp::Value(LayoutType::Column),
            width: StyleProp::Value(Units::Percentage(30.0)),
            height: StyleProp::Value(Units::Percentage(40.0)),

            border_radius: StyleProp::Value(Corner::all(10.0)),
            background_color: StyleProp::Value(Color::new(0.6, 0.4, 0.3, 1.0)),
            //Centers
            left: StyleProp::Value(Units::Stretch(1.0)),
            right: StyleProp::Value(Units::Stretch(1.0)),
            top: StyleProp::Value(Units::Stretch(1.0)),
            bottom: StyleProp::Value(Units::Stretch(1.0)),
            //Centers children
            padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
            ..default()
        };

        let title_style = Style {
            bottom: StyleProp::Value(Units::Stretch(1.0)),
            ..default()
        };

        let start_button = OnEvent::new(|context, event| {
            if let EventType::Click(..) = event.event_type {
                context.query_world::<ResMut<State<GameState>>, _, _>(|mut state| {
                    state
                        .set(GameState::GamePlay)
                        .expect("Failed to change state");
                });
            }
        });

        let button_style = Style {
            width: StyleProp::Value(Units::Percentage(80.0)),
            height: StyleProp::Value(Units::Percentage(10.0)),
            top: StyleProp::Value(Units::Pixels(10.0)),
            //Centers children
            padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
            ..default()
        };

        render! {
            <widgets::App>
                <widgets::Background styles={Some(container_style)}>
                    <widgets::Text content={"Video Game Title".to_string()} size={32.0} styles={Some(title_style)}/>
                    <widgets::Text content={"Game By: rand0m-cloud & LogicProjects".to_string()} size={24.0} styles={Some(title_style)}/>
                    <widgets::Text content={"Made with Bevy!".to_string()} size={24.0} styles={Some(title_style)}/>
                    <widgets::Button styles={Some(button_style)} on_event={Some(start_button)}>
                        <widgets::Text content={"Start".to_string()} size={24.0} />
                    </widgets::Button>
                    <widgets::Button styles={Some(button_style)}>
                        <widgets::Text content={"Options".to_string()} size={24.0} />
                    </widgets::Button>
                </widgets::Background>
            </widgets::App>
        }
    });

    commands.insert_resource(context);
}
