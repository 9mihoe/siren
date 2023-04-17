mod player;
// use rand::Rng; // 0.8.5

fn main() {
    let num_players: u32 = 100;
    let players: Vec<player::Player> = {
        let mut tmp: Vec<player::Player> = Vec::new();
        for _ in 0..num_players {
            tmp.push(player::Player::new(0.1, 0.1));
        }
        tmp
    };
    println!("{}", players.len());
}