#[derive(Clone, Copy)]
pub struct Player {
    cooperation: f64,
    aggression: f64
}

enum CharacterCategory {
    VHi,
    Hi,
    Mid,
    Lo,
    VLo
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

    // https://stackoverflow.com/questions/49037111/alternatives-to-matching-floating-point-ranges
    fn get_category(character: f64) -> CharacterCategory {
        return match character {
            character if character>0.8 => CharacterCategory::VHi,
            character if character>0.6 => CharacterCategory::Hi,
            character if character>0.4 => CharacterCategory::Mid,
            character if character>0.2 => CharacterCategory::Lo,
            character if character>0.0 => CharacterCategory::VLo,
            _ => panic!("Character trait out-of-bounds!")
        };
    }

}