#[derive(Clone, Copy)]
pub struct Player {
    cooperation: f64,
    aggression: f64
}

impl Player {
    pub fn new(cooperation_: f64, aggression_: f64) -> Player {
        Player{cooperation: cooperation_, aggression: aggression_}
    }

    pub fn will_jump(&self) -> bool {
        if self.cooperation > 0.5 {
            return true;
        }
        return false;
    }
}