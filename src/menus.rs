//Explicit paths because kayak uses all the same names
use bevy::{
    core::Name,
    prelude::{default, AssetServer, Commands, Handle, Plugin, Res, ResMut, State, SystemSet},
};
use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, ImageManager, UICameraBundle},
    core::{
        render, rsx,
        styles::{Corner, Edge, LayoutType, Style, StyleProp, Units},
        use_state, widget, Color, EventType, Index, OnEvent, OnLayout, WidgetProps,
    },
    widgets,
};

use crate::{GameState, SCALE};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .init_resource::<f32>()
            .add_startup_system(setup_kayak)
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy_ui))
            .add_system_set(SystemSet::on_enter(GameState::Tutorial).with_system(spawn_tutorial))
            .add_system_set(SystemSet::on_exit(GameState::Tutorial).with_system(destroy_ui));
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

fn spawn_main_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut image_manager: ResMut<ImageManager>,
) {
    let ooga_booga_text_doesnt_render_in_wasm: Handle<bevy::render::texture::Image> =
        assets.load("text_workaround.png");
    let image_handle = image_manager.get(&ooga_booga_text_doesnt_render_in_wasm);

    let ooga_booga_text_doesnt_render_in_wasm: Handle<bevy::render::texture::Image> =
        assets.load("volume_workaround.png");
    let volume_handle = image_manager.get(&ooga_booga_text_doesnt_render_in_wasm);

    let ooga_booga_text_doesnt_render_in_wasm: Handle<bevy::render::texture::Image> =
        assets.load("button_workaround.png");
    let button_handle = image_manager.get(&ooga_booga_text_doesnt_render_in_wasm);

    let context = BevyContext::new(|context| {
        let container_style = Style {
            layout_type: StyleProp::Value(LayoutType::Column),
            width: StyleProp::Value(Units::Percentage(35.0)),
            height: StyleProp::Value(Units::Percentage(45.0)),

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

        let _title_style = Style {
            bottom: StyleProp::Value(Units::Stretch(1.0)),
            ..default()
        };

        let start_button = OnEvent::new(|context, event| {
            if let EventType::Click(..) = event.event_type {
                context.query_world::<ResMut<State<GameState>>, _, _>(|mut state| {
                    state
                        .set(GameState::Tutorial)
                        .expect("Failed to change state");
                });
            }
        });

        let button_style = Style {
            width: StyleProp::Value(Units::Percentage(80.0 * SCALE)),
            height: StyleProp::Value(Units::Percentage(10.0 * SCALE)),
            top: StyleProp::Value(Units::Pixels(5.0)),
            //Centers children
            padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
            ..default()
        };

        let element_style = Style {
            layout_type: StyleProp::Value(LayoutType::Row),
            width: StyleProp::Value(Units::Percentage(100.0)),
            height: StyleProp::Value(Units::Percentage(10.0)),
            padding: StyleProp::Value(Edge {
                left: Units::Stretch(1.0),
                right: Units::Stretch(1.0),
                top: Units::Pixels(30.0),
                bottom: Units::Stretch(1.0),
            }),
            ..default()
        };

        //Sizes could be loaded from image desc but that's hell
        let image_title_style = Style {
            top: StyleProp::Value(Units::Pixels(10.0)),
            width: StyleProp::Value(Units::Pixels(460.0 * SCALE)),
            height: StyleProp::Value(Units::Pixels(235.0 * SCALE)),
            ..default()
        };

        let image_button_style = Style {
            width: StyleProp::Value(Units::Pixels(73.0 * SCALE)),
            height: StyleProp::Value(Units::Pixels(25.0 * SCALE)),
            ..default()
        };

        let image_volume_style = Style {
            width: StyleProp::Value(Units::Pixels(113.0 * SCALE)),
            height: StyleProp::Value(Units::Pixels(32.0 * SCALE)),
            ..default()
        };

        let box_color = Color::WHITE;
        let button_color = Color::new(0.9, 0.1, 0.1, 1.0);

        render! {
            <widgets::App>
                <widgets::Background styles={Some(container_style)}>
                    //<widgets::Text content={"Fight for the Frontier".to_string()} size={32.0} styles={Some(_title_style)}/>
                    //<widgets::Text content={"Game By: rand0m-cloud & LogicProjects".to_string()} size={24.0} />
                    //<widgets::Text content={"Art By: Madeline Hunt".to_string()} size={24.0} styles={Some(_title_style)}/>
                    //<widgets::Text content={"Made with Bevy!".to_string()} size={24.0} styles={Some(_title_style)}/>
                    <widgets::Image handle={image_handle} styles={Some(image_title_style)}/>
                    <widgets::Button styles={Some(button_style)} on_event={Some(start_button)}>
                        //<widgets::Text content={"Start".to_string()} size={24.0} />
                    <widgets::Image handle={button_handle} styles={Some(image_button_style)}/>
                    </widgets::Button>
                    //<widgets::Button styles={Some(button_style)}>
                        //<widgets::Text content={"Options".to_string()} size={24.0} />
                    //</widgets::Button>
                    <widgets::Element styles={Some(element_style)}>
                        //<widgets::Text content={"Volume: ".to_string()} size={24.0} />
                    <widgets::Image handle={volume_handle} styles={Some(image_volume_style)}/>
                    <SliderBox size={(200.0, 30.0)} box_color={box_color} button_color={button_color}/>
                    </widgets::Element>
                </widgets::Background>
            </widgets::App>
        }
    });

    commands.insert_resource(context);
}

#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct SliderBoxProps {
    size: (f32, f32),
    box_color: Color,
    button_color: Color,
}

#[widget]
fn SliderBox(props: SliderBoxProps) {
    //Set up slider internal state
    let (is_dragging, set_is_dragging, ..) = use_state!(false);
    let (offset, set_offset, ..) = use_state!(props.size.0 / 7.3);
    let (pos, set_pos, ..) = use_state!(props.size.0 / 7.3);
    let (percent, set_percent, ..) = use_state!(10.0);
    let (layout, set_layout, ..) = use_state!(10000.0);

    //Handle dragging
    let drag_handler = Some(OnEvent::new(move |ctx, event| match event.event_type {
        EventType::MouseDown(data) => {
            println!("Draging");
            ctx.capture_cursor(event.current_target);
            set_is_dragging(true);
            set_offset(pos - data.position.0);
        }
        EventType::MouseUp(..) => {
            ctx.release_cursor(event.current_target);
            set_is_dragging(false);
        }
        EventType::Hover(data) => {
            if is_dragging {
                set_pos(offset + data.position.0);
            }
        }
        _ => {}
    }));

    //Get width and height on every layout
    let on_layout = OnLayout::new(move |_, event| {
        let layout = event.layout;
        set_layout(layout.width);
    });

    let (width, height) = props.size;
    let button_width = 15.0;
    let round_amount = 10.0;

    //Calculate max allowed percent
    //(position is set at top left corner of button so max percent is less than 100)
    let max_percent = (1.0 - button_width / layout) * 100.0;

    //Update percent
    set_percent((pos / layout * 100.0).clamp(0.0, max_percent));

    //Report setting back to game ECS world
    let true_percent = percent / max_percent;
    context.query_world::<Commands, _, _>(|mut commands| {
        commands.insert_resource(true_percent);
    });

    let background = Style {
        border_radius: StyleProp::Value(Corner::all(round_amount)),
        left: StyleProp::Value(Units::Pixels(20.0)),
        width: StyleProp::Value(Units::Pixels(width * SCALE)),
        height: StyleProp::Value(Units::Pixels(height * SCALE)),
        background_color: StyleProp::Value(props.box_color),
        //padding: StyleProp::Value(Edge::all(Units::Percentage(10.0))),
        ..default()
    };

    let button = Style {
        border_radius: StyleProp::Value(Corner::all(round_amount)),
        background_color: StyleProp::Value(props.button_color),
        width: StyleProp::Value(Units::Percentage(button_width)),
        height: StyleProp::Value(Units::Percentage(100.0)),
        left: StyleProp::Value(Units::Percentage(percent)),
        ..default()
    };

    rsx! {
        <widgets::Background styles={Some(background)} on_layout={Some(on_layout)} >
            <widgets::Background on_event={drag_handler} styles={Some(button) } />
        </widgets::Background>
    }
}

fn spawn_tutorial(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut image_manager: ResMut<ImageManager>,
) {
    let tutorial_screen: Handle<bevy::render::texture::Image> = assets.load("tutorial.png");
    let image_handle = image_manager.get(&tutorial_screen);

    let start_button: Handle<bevy::render::texture::Image> = assets.load("button_workaround.png");
    let button_handle = image_manager.get(&start_button);

    let context = BevyContext::new(|context| {
        let start_button = OnEvent::new(|context, event| {
            if let EventType::Click(..) = event.event_type {
                context.query_world::<ResMut<State<GameState>>, _, _>(|mut state| {
                    state
                        .set(GameState::GamePlay)
                        .expect("Failed to change state");
                });
            }
        });

        let bg_image_style = Style {
            width: StyleProp::Value(Units::Percentage(100.0)),
            height: StyleProp::Value(Units::Percentage(101.0)),
            ..default()
        };

        let button_style = Style {
            left: StyleProp::Value(Units::Percentage(80.0)),
            width: StyleProp::Value(Units::Pixels(146.0)),
            height: StyleProp::Value(Units::Pixels(50.0)),
            top: StyleProp::Value(Units::Percentage(75.0)),
            ..default()
        };

        let image_style = Style {
            width: StyleProp::Value(Units::Pixels(146.0)),
            height: StyleProp::Value(Units::Pixels(50.0)),
            ..default()
        };

        render! {
            <widgets::App>
                <widgets::Image handle={image_handle} styles={Some(bg_image_style)}>
                  <widgets::Button styles={Some(button_style)} on_event={Some(start_button)}>
                    <widgets::Image handle={button_handle} styles={Some(image_style)}/>
                  </widgets::Button>
                </widgets::Image>
            </widgets::App>
        }
    });

    commands.insert_resource(context);
}
