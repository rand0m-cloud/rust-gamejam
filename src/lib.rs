#![allow(clippy::type_complexity)]

pub mod assets;
pub mod bullet;
pub mod debug;
pub mod enemy;
pub mod external;
pub mod map;
pub mod menus;
pub mod minion;
pub mod player;
pub mod prelude;
pub mod spawner;
pub mod world_ui;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum GameState {
    Splash,
    MainMenu,
    GamePlay,
}
