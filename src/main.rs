extern crate ggez;
extern crate noframe;

use ggez::{
  Context,
  graphics,
  event
};

use noframe::geo::prelude::*;

mod game_state;
mod player;
use self::game_state::GameState;

fn main() {
  let mut ctx = ggez::ContextBuilder::new(
    "demo game: top-down", "Noah"
  ).window_setup(
    ggez::conf::WindowSetup::default().title("Demo Game: Top-Down!")
  ).window_mode(
    ggez::conf::WindowMode::default().dimensions(
      800u32,
      600u32
    )
  ).build().expect("Should build Context");

  graphics::set_background_color(&mut ctx, [0.33, 0.33, 0.33, 1.0].into());
  let mut state = GameState::new();
  if let Err(e) = event::run(&mut ctx, &mut state) {
    eprintln!("An error occured: {}", e);
  }
}
