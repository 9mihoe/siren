// https://users.rust-lang.org/t/usage-of-extern-crate/73619

use bracket_lib::prelude::*;

const SCREEN_WIDTH : i32 = 40;
const SCREEN_HEIGHT : i32 = 25;
const FRAME_DURATION : f32 = 75.0;

enum Dir {
    Left,
    Right,
    Up,
    Down
}

struct Player {
  pub x: i32,
  pub y: i32,
  length: i32
}

impl Player {
  fn new(x: i32, y: i32) -> Self {
      Player {
          x: x,
          y: y,
          length: 0
      }
  }

  fn render(&mut self, ctx: &mut BTerm) {
      ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'));
      ctx.set_active_console(0);
  }

  pub fn update_position(&mut self, ctx: &mut BTerm) {
    // Left right up down no work for some reason.
    if let Some(key) = ctx.key {
      let delta = match key {
          VirtualKeyCode::D => self.x += 10,
          VirtualKeyCode::A => self.x -= 10,
          VirtualKeyCode::W => self.y -= 10,
          VirtualKeyCode::S => self.y += 10,
          _ => self.x += 0,
      };
    }
  }
}

struct Food {
  x: i32,
  y: i32,
}

impl Food {
  fn new() -> Self {
      let mut random = RandomNumberGenerator::new();
      Food {
          x: 20,
          y: 10,
      }
  }

  fn render(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    ctx.set(self.x, self.y, WHITE, BLACK, to_cp437('@'));
    ctx.set_active_console(0);
  }
}

struct State {
  player: Player,
  frame_time: f32,
  food: Food,
  score: i32,
}

impl State {
  fn new() -> Self {
      State {
          player: Player::new(20, 20),
          frame_time: 0.0,
          food: Food::new(),
          score: 0,
      }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    self.food.render(ctx);
    self.player.update_position(ctx);
    self.player.render(ctx);
  }
}

fn main() -> BError {
  let context = BTermBuilder::simple80x50()
      .with_title("Flappy Dragon Enhanced")
      .build()?;

  main_loop(context, State::new())
}