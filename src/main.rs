mod player;

fn main() {
    // Try: https://stackoverflow.com/questions/67656408/how-to-properly-initialize-a-vector-in-rust-given-that-the-vector-will-be-immut
    let mut players: Vec<player::Player> = Vec::new();
    let NUM_PLAYERS: u32 = 100;
    for i in 0..NUM_PLAYERS {
        players.push(player::Player::new(0.1, 0.1));
    }
    // let p = player::Player::new(0.1, 0.1);
    // let will_jump = p.will_jump();
    println!("hello world");
}