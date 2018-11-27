use ::std::time::{ Instant, Duration };

use ::ggez::{
  Context,
  GameResult,
  graphics,
  event::{
    self,
    Keycode
  }
};

use ::noframe::geo::mask::Mask;
use ::noframe::entity::{
  Entity,
  Movement
};
use ::noframe::input_manager::InputManager;

use ::player::Player;
use ::wall::Wall;

const FPS: f32 = 30.0;
const UPDATE_INTERVAL_MS: u64 = (1.0 / FPS * 1000.0) as u64;

pub struct GameState {
  player:        Player,
  walls:         Vec<Wall>,
  input_manager: InputManager,
  running:       bool,
  last_update:   Instant
}

impl GameState {
  pub fn new() -> Self {
    Self {
      player: Player::new(256.0, 256.0),
      walls:  vec![
        Wall::new_default_size(0.0,   0.0),
        Wall::new_default_size(64.0,  0.0),
        Wall::new_default_size(128.0, 0.0),
        Wall::new_default_size(192.0, 0.0),
      ],
      input_manager: InputManager::new(),
      running:       true,
      last_update:   Instant::now()
    }
  }
}

impl event::EventHandler for GameState {
  fn key_down_event(&mut self,
                    ctx:     &mut Context,
                    keycode: Keycode,
                    _keymod: event::Mod,
                    repeat:  bool) {
    self.input_manager.key_down(keycode, _keymod, repeat);
    if let Keycode::Escape = keycode {
      ctx.quit().expect("Should quit Context");
    }
  }

  fn key_up_event(&mut self,
                  _ctx:    &mut Context,
                  keycode: Keycode,
                  _keymod: event::Mod,
                  repeat:  bool) {
    self.input_manager.key_up(keycode, _keymod, repeat);
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    if !self.running || Instant::now() - self.last_update < Duration::from_millis(UPDATE_INTERVAL_MS) {
      return Ok(());
    }

    for wall in &mut self.walls {
      wall.update(_ctx)?;
    }
    self.player.keys_pressed(self.input_manager.keys());

    // TODO
    // self.player.move_while( |rect| self.walls.iter().any( |wall| rect.intersects(wall) ) );

    self.player.update(_ctx)?;

    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    for wall in &mut self.walls {
      wall.draw(ctx)?;
    }
    self.player.draw(ctx)?;

    graphics::present(ctx);
    ::ggez::timer::yield_now();
    return Ok(());
  }
}
