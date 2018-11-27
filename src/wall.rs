use ::noframe::geo::{
  prelude::*,
  NumType
};
use ::noframe::entity::Entity;
use ::noframe::color::{
  self,
  Color
};

const COLOR: Color = color::BLACK;
const SIZE:  Size  = Size { w: 64.0, h: 64.0 };

pub struct Wall {
  point: Point,
  size: Size,
  origin: Origin,
  color: Color
}

impl Wall {
  pub fn new(x: NumType, y: NumType, w: NumType, h: NumType) -> Self {
    Self {
      point:  Point::new(x, y),
      size:   Size::new(w, h),
      origin: Origin::TopLeft,
      color:  COLOR
    }
  }

  pub fn new_default_size(x: NumType, y: NumType) -> Self {
    Self::new(x, y, SIZE.w, SIZE.h)
  }
}

impl Mask for Wall {
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

impl Entity for Wall {
  fn color(&self) -> Color {
    self.color
  }
}
