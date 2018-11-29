use ::ggez::{
  Context,
  GameResult,
  event::Keycode
};

use ::noframe::geo::{
  prelude::*,
  NumType
};
use ::noframe::entity::prelude::*;
use ::noframe::color::{
  self,
  Color
};
use ::noframe::deltatime::Deltatime;

const COLOR: Color = [0.4, 0.6, 0.7, 1.0];
const SIZE: Size = Size { w: 32.0, h: 32.0 };
const SPEED: ::noframe::geo::NumType = 20.0;
const SPEED_DECREASE: ::noframe::geo::NumType = 100.0;
const MAX_VELOCITY: ::noframe::geo::NumType = 300.0;

mod controls {
  use ::ggez::event::Keycode;
  pub const UP:    Keycode = Keycode::W;
  pub const DOWN:  Keycode = Keycode::S;
  pub const LEFT:  Keycode = Keycode::A;
  pub const RIGHT: Keycode = Keycode::D;
}

#[derive(PartialEq)]
enum Axis {
  X,
  Y
}

pub struct Player {
  point:        Point,
  size:         Size,
  origin:       Origin,
  color:        Color,
  velocity:     Point,
  max_velocity: Point,
  has_moved:    Vec<Axis>,
  dt:           Deltatime
}

impl Player {
  pub fn new(x: NumType, y: NumType) -> Self {
    Self {
      point:        Point::new(x, y),
      size:         SIZE,
      origin:       Origin::Center,
      color:        COLOR,
      velocity:     Point::new(0.0, 0.0),
      max_velocity: Point::new(MAX_VELOCITY, MAX_VELOCITY),
      has_moved:    Vec::new(),
      dt:           Deltatime::new()
    }
  }

  pub fn keys_pressed(&mut self, pressed_keys: &Vec<Keycode>) {
    for &keycode in pressed_keys {
      if let Some(point) = match keycode {
        controls::UP => {
          if !self.has_moved(Axis::Y) {
            self.moved_on_axis(Axis::Y);
            Some(Point::new( 0.0, -SPEED ))
          } else { None }
        }
        controls::DOWN => {
          if !self.has_moved(Axis::Y) {
            self.moved_on_axis(Axis::Y);
            Some(Point::new( 0.0, SPEED ))
          } else { None }
        }
        controls::LEFT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( -SPEED, 0.0 ))
          } else { None }
        }
        controls::RIGHT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( SPEED, 0.0 ))
          } else { None }
        }
        _ => None
      } {
        self.add_velocity(&point);
      }
    }
  }

  fn moved_on_axis(&mut self, axis: Axis) {
    if !self.has_moved.iter().any( |a| &axis == a ) {
      self.has_moved.push(axis);
    }
  }

  fn has_moved(&self, axis: Axis) -> bool {
    self.has_moved.iter().any( |a| &axis == a )
  }

  fn update_position(&mut self) {
    let vel = self.velocity().clone();
    self.handle_decrease_velocity();

    // self.point_mut().add(&vel);

    self.has_moved.clear();
  }

  fn handle_decrease_velocity(&mut self) {
    let decr = SPEED_DECREASE; //* self.dt.secs();
    let decr_vel = Point::new(
      if !self.has_moved(Axis::X) {
        decr
      } else { 0.0 },
      if !self.has_moved(Axis::Y) {
        decr
      } else { 0.0 }
    );
    self.decrease_velocity(&decr_vel);
  }
}

impl Mask for Player {
  fn point(&self) -> &Point {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point {
    &mut self.point
  }
  fn size(&self) -> &Size {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
  }
}

impl Entity for Player {
  fn color(&self) -> Color {
    self.color
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.update_position();
    self.dt.update();
    return Ok(());
  }
}

impl Velocity for Player {
  fn velocity(&self) -> &Point {
    &self.velocity
  }
  fn usable_velocity(&self) -> Point {
    self.velocity.mult_axes_by(self.dt.secs())
  }
  fn velocity_mut(&mut self) -> &mut Point {
    &mut self.velocity
  }
  fn max_velocity(&self) -> Point {
    self.max_velocity.clone()
  }
}

impl Movement for Player {}
