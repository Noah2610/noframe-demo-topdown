use ::std::fs::File;
use ::std::io::prelude::*;

use ::ggez::GameResult;

use ::wall::Wall;

pub fn load_walls(json_filepath: &str) -> GameResult<Vec<Wall>> {
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
