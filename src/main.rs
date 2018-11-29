#[macro_use]
extern crate json;
extern crate ggez;
extern crate noframe;

mod game_state;
mod player;
mod wall;

use std::{ env, path };

use ggez::{
  GameResult,
  graphics,
  event
};

use noframe::geo::NumType;

use self::game_state::GameState;

const WINDOW_SIZE: [NumType; 2] = [800.0, 600.0];

fn main() {
  if let Err(err) = run() {
    eprintln!("An error occured: {}", err);
    std::process::exit(1);
  }
}

fn run() -> GameResult<()> {
  let mut ctx = ggez::ContextBuilder::new(
    "noframe demo game: topdown", "Noah"
  ).window_setup(
    ggez::conf::WindowSetup::default().title("noframe Demo Game: Top-Down!")
  ).window_mode(
    ggez::conf::WindowMode::default().dimensions(
      WINDOW_SIZE[0] as u32,
      WINDOW_SIZE[1] as u32,
    )
  ).build().expect("Should build Context");

  if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    ctx.filesystem.mount(&path, true);
  }

  graphics::set_background_color(&mut ctx, [0.33, 0.33, 0.33, 1.0].into());
  let mut state = GameState::new(WINDOW_SIZE)?;
  event::run(&mut ctx, &mut state)?;
  return Ok(());
}
