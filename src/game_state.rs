use ::std::time::{ Instant, Duration };
use ::std::fs::File;
use ::std::io::prelude::*;

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

fn load_walls(json_filepath: &str) -> GameResult<Vec<Wall>> {
  let mut file = File::open(json_filepath)?;
  let mut json = String::new();
  file.read_to_string(&mut json)?;
  let data = match json::parse(&json) {
    Ok(d)  => d,
    Err(e) => return Err(ggez::GameError::from(e.to_string()))
  };  // I don't think this is idomatic rust...

  return Ok(
    data["instances"].members().map( |d| {
      let pos  = &d["position"];
      let size = &d["size"];
      Wall::new(
        pos["x"].as_f32().expect(
          &format!("Couldn't convert a Wall's position JsonValue 'x' to a f32: {:?}", pos["x"])
        ),
        pos["y"].as_f32().expect(
          &format!("Couldn't convert a Wall's position JsonValue 'y' to a f32: {:?}", pos["y"])
        ),
        size["w"].as_f32().expect(
          &format!("Couldn't convert a Wall's size JsonValue 'w' to a f32: {:?}", size["w"])
        ),
        size["h"].as_f32().expect(
          &format!("Couldn't convert a Wall's size JsonValue 'h' to a f32: {:?}", size["h"])
        )
      )
    }).collect()
  );
}

pub struct GameState {
  player:        Player,
  walls:         Vec<Wall>,
  input_manager: InputManager,
  running:       bool,
  last_update:   Instant
}

impl GameState {
  pub fn new() -> GameResult<Self> {
    Ok(Self {
      player:        Player::new(0.0, 128.0),
      walls:         load_walls("./resources/walls.json")?,
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
    }

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
