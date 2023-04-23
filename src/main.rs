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

//   fn move(&mut self, dir: Dir) {
//     match dir {
//         Left => self.x + 1,
//         Right => self.x - 1,
//         Up => self.y + 1,
//         Down => self.y -1
//       }
//   }

  fn render(&mut self, ctx: &mut BTerm) {
    // https://github.com/amethyst/bracket-lib/blob/f75d0419db3c636986d9829a1d4ff4708002f84d/bracket-terminal/src/bterm.rs
    //   ctx.set_active_console(0);
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
          x: random.range(5, 20),
          y: random.range(5, 20),
      }
  }

  fn render(&mut self, ctx: &mut BTerm, player_x : i32) {
    ctx.cls();
    ctx.set(
        self.x,
        self.y,
        WHITE,
        BLACK,
        to_cp437('@')
    );
    ctx.set_active_console(0);
  }

  fn food_eaten(&self, player: &Player) -> bool {
    //   let half_size = self.size / 2;
    //   player.x == self.x
    //       && ((player.y as i32) < self.gap_y - half_size || player.y as i32 > 
    //       self.gap_y + half_size)
    return false;
  }
}

enum GameMode {
  Playing,
  End,
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
          player: Player::new(5, 25),
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

  fn dead(&mut self, ctx: &mut BTerm) {
      ctx.cls();
      ctx.print_color_centered(5, RED, BLACK, "You are dead!");
      ctx.print_centered(6, &format!("You earned {} points", self.score));
      ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Again");
      ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

      if let Some(key) = ctx.key {
          match key {
              VirtualKeyCode::P => self.restart(),
              VirtualKeyCode::Q => ctx.quitting = true,
              _ => {}
          }
      }
  }

  fn play(&mut self, ctx: &mut BTerm) {
      ctx.cls_bg(NAVY);
      self.frame_time += ctx.frame_time_ms;
      if self.frame_time > FRAME_DURATION {
          self.frame_time = 0.0;
      }
    //   if let Some(VirtualKeyCode::Space) = ctx.key {
    //       self.player.flap();
    //   }
      self.player.render(ctx);
      ctx.print(0, 0, "Press SPACE to flap.");
      ctx.print(0, 1, &format!("Score: {}", self.score));

      self.food.render(ctx, self.player.x);
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
      match self.mode {
          GameMode::End => self.dead(ctx),
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