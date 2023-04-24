// https://users.rust-lang.org/t/usage-of-extern-crate/73619

use bracket_lib::prelude::*;

const SCREEN_WIDTH : i32 = 40;
const SCREEN_HEIGHT : i32 = 25;
const FRAME_DURATION : f32 = 75.0;

const DRAGON_FRAMES : [u16; 6] = [ 64, 1, 2, 3, 2, 1 ];

enum Dir {
    Left,
    Right,
    Up,
    Down
}

struct Player {
  x: i32,
  y: i32,
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
      ctx.cls();
      ctx.set(
          self.x,
          self.y,
          YELLOW,
          BLACK,
          to_cp437('@')
      );
      ctx.set_active_console(0);
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

  fn render(&mut self, ctx: &mut BTerm, player_x : i32) {
    ctx.set(
        self.x,
        self.y,
        WHITE,
        BLACK,
        to_cp437('@')
    );
    ctx.set_active_console(0);
  }
}

enum GameMode {
  Playing,
}

struct State {
  player: Player,
  frame_time: f32,
  food: Food,
  mode: GameMode,
  score: i32,
}

impl State {
  fn new() -> Self {
      State {
          player: Player::new(20, 30),
          frame_time: 0.0,
          food: Food::new(),
          mode: GameMode::Playing,
          score: 0,
      }
  }

  fn restart(&mut self) {
      self.player = Player::new(5, SCREEN_WIDTH/2);
      self.frame_time = 0.0;
      self.food = Food::new();
      self.mode = GameMode::Playing;
      self.score = 0;
  }

  fn play(&mut self, ctx: &mut BTerm) {
      ctx.cls_bg(NAVY);
      self.player.render(ctx);
      self.food.render(ctx, self.player.x);
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
      match self.mode {
          // GameMode::End => self.dead(ctx),
          GameMode::Playing => self.play(ctx),
      }
  }
}

fn main() -> BError {
  let context = BTermBuilder::simple80x50()
      .with_title("Flappy Dragon Enhanced")
      .build()?;

  main_loop(context, State::new())
}