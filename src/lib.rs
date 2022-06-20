#![allow(clippy::type_complexity)]

pub mod assets;
pub mod bullet;
pub mod enemy;
pub mod external;
pub mod map;
pub mod minion;
pub mod player;
pub mod prelude;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum GameState {
    Splash,
    GamePlay,
}
