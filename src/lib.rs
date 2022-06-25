#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub const SCALE: f32 = 2.0 / 3.0;

pub mod assets;
pub mod audio;
pub mod bullet;
pub mod debug;
pub mod enemy;
pub mod external;
pub mod gameover;
pub mod map;
pub mod menus;
pub mod minion;
pub mod particles;
pub mod player;
pub mod prelude;
pub mod spawner;
pub mod world_ui;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum GameState {
    Splash,
    MainMenu,
    Tutorial,
    GamePlay,
    GameOver { won: bool },
}
