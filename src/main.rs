// https://users.rust-lang.org/t/usage-of-extern-crate/73619

use bracket_lib::prelude::*;

const SCREEN_WIDTH : i32 = 50;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 75.0;

enum Dir {
  Static, // Only at the beginning.
  Left,
  Right,
  Up,
  Down
}

struct Player {
  pub x: i32,
  pub y: i32,
  pub dir: Dir,
  pub len: i32
}

impl Player {
  fn new(x: i32, y: i32) -> Self {
      Player {
          x: x,
          y: y,
          len: 1,
          dir: Dir::Static
      }
  }

  fn render(&mut self, ctx: &mut BTerm) {
      ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'));
      ctx.set_active_console(0);
  }

  fn update_direction(&mut self, ctx: &mut BTerm) {
    if let Some(key) = ctx.key {
      match key {
        VirtualKeyCode::D => self.dir = Dir::Left,
        VirtualKeyCode::A => self.dir = Dir::Right,
        VirtualKeyCode::W => self.dir = Dir::Up,
        VirtualKeyCode::S => self.dir = Dir::Down,
        _ => (),
      };
    }
  }

  fn update_position(&mut self) {
  // Left right up down no work for some reason.
    match self.dir {
      Dir::Left => self.x += 1,
      Dir::Right => self.x -= 1,
      Dir::Up => self.y -= 1,
      Dir::Down => self.y += 1,
      Dir::Static => ()
    }
  }

  fn is_out_of_bounds(&mut self) -> bool {
    return (
      self.x <= 0 
      || self.x >= SCREEN_WIDTH 
      || self.y <= 0 
      || self.y >= SCREEN_HEIGHT
    );
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

enum GameMode {
  Playing,
  Dead
}

struct State {
  mode: GameMode,
  player: Player,
  ticks: u64,
  food: Food,
  score: i32,
}

impl State {
  fn new() -> Self {
      State {
        mode: GameMode::Playing,
        player: Player::new(20, 20),
        ticks: 0,
        food: Food::new(),
        score: 0,
      }
  }

  fn restart(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    self.player = Player::new(20, 20);
    self.ticks = 0;
    self.food = Food::new();
    self.score = 0;
  }

  fn play(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    self.food.render(ctx);
    self.player.update_direction(ctx);
    if self.ticks % 3 == 0 {
      self.player.update_position();
    }
    self.player.render(ctx);
    if self.player.is_out_of_bounds() {
      self.mode = GameMode::Dead;
    }
  }

  fn dead(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "You are dead!");
    ctx.print_centered(8, "(P) Play Again");
    ctx.print_centered(9, "(Q) Quit Game");

    if let Some(key) = ctx.key {
      match key {
          VirtualKeyCode::P => {
            self.mode = GameMode::Playing;
            self.restart(ctx);
          },
          _ => {}
      }
    }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    match self.mode {
      GameMode::Playing => self.play(ctx),
      GameMode::Dead => self.dead(ctx),
    }
  }
}

fn main() -> BError {
  let context = (
    BTermBuilder::simple(50, 50)
    .unwrap()
    .with_title("Snek")
    .build()?
  );
  main_loop(context, State::new())
}