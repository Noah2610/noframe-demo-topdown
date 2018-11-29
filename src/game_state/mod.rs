mod helpers;

use self::helpers::*;

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

use ::noframe::geo::prelude::*;
use ::noframe::entity::{
  Entity,
  Movement
};
use ::noframe::input_manager::InputManager;
use ::noframe::camera::Camera;

use ::player::Player;
use ::wall::Wall;

const FPS: f32 = 30.0;
const UPDATE_INTERVAL_MS: u64 = (1.0 / FPS * 1000.0) as u64;

pub struct GameState {
  window_rect:   Rect,
  player:        Player,
  walls:         Vec<Wall>,
  camera:        Camera,
  input_manager: InputManager,
  running:       bool,
  last_update:   Instant
}

impl GameState {
  pub fn new(window_size: [NumType; 2]) -> GameResult<Self> {
    Ok(Self {
      window_rect:   Rect::new(Point::new(0.0, 0.0), Size::from(window_size), Origin::TopLeft),
      player:        Player::new(0.0, 128.0),
      walls:         load_walls("./resources/walls.json")?,
      camera:        Camera::new(),
      input_manager: InputManager::new(),
      running:       true,
      last_update:   Instant::now()
    })
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

    let new_pos = self.player.get_move_while(
      |rect| !self.walls.iter().any( |wall| rect.intersects(wall) )
    );
    if &new_pos != self.player.point() {
      self.player.point_mut().set(&new_pos);
      self.camera.move_to(&Point::combine(
          vec![self.player.point(), &self.window_rect.center().inverted()]
      ));
    }

    self.player.update(_ctx)?;

    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    for wall in &mut self.walls {
      self.camera.draw(ctx, wall)?;
    }
    self.camera.draw(ctx, &self.player)?;

    graphics::present(ctx);
    ::ggez::timer::yield_now();
    return Ok(());
  }
}
