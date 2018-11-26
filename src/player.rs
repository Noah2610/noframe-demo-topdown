use ::ggez::{
  Context,
  GameResult,
  event::Keycode
};

use ::noframe::geo::prelude::*;
use ::noframe::entity::prelude::*;

const SIZE: Size = Size { w: 32.0, h: 32.0 };
const SPEED: ::noframe::geo::NumType = 4.0;
const SPEED_DECREASE: ::noframe::geo::NumType = 4.0;
const MAX_VELOCITY: ::noframe::geo::NumType = 16.0;

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
  velocity:     Point,
  max_velocity: Point,
  has_moved:    Vec<Axis>
}

impl Player {
  pub fn new() -> Self {
    Self {
      point:        Point::new(64.0, 64.0),
      size:         SIZE,
      origin:       Origin::TopLeft,
      velocity:     Point::new(0.0, 0.0),
      max_velocity: Point::new(MAX_VELOCITY, MAX_VELOCITY),
      has_moved:    Vec::new()
    }
  }

  pub fn keys_pressed(&mut self, pressed_keys: &Vec<Keycode>) {
    for &keycode in pressed_keys {
      if let Some(point) = match keycode {
        controls::UP    => {
          self.moved_on_axis(Axis::Y);
          Some(Point::new( 0.0,   -SPEED))
        }
        controls::DOWN  => {
          self.moved_on_axis(Axis::Y);
          Some(Point::new( 0.0,    SPEED))
        }
        controls::LEFT  => {
          self.moved_on_axis(Axis::X);
          Some(Point::new(-SPEED,  0.0))
        }
        controls::RIGHT => {
          self.moved_on_axis(Axis::X);
          Some(Point::new( SPEED,  0.0 ))
        }
        _               => None
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
    self.point_mut().add(&vel);
    self.has_moved.clear();
  }

  fn handle_decrease_velocity(&mut self) {
    let decr_vel = Point::new(
      if !self.has_moved(Axis::X) {
        SPEED_DECREASE
      } else { 0.0 },
      if !self.has_moved(Axis::Y) {
        SPEED_DECREASE
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
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.update_position();
    return Ok(());
  }
}

impl Velocity for Player {
  fn velocity(&self) -> &Point {
    &self.velocity
  }
  fn velocity_mut(&mut self) -> &mut Point {
    &mut self.velocity
  }
  fn max_velocity(&self) -> &Point {
    &self.max_velocity
  }
}