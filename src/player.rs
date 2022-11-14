pub enum Player {
    X,
    O,
}

pub struct PlayerObj {}
impl PlayerObj {
    pub fn select_player(character: char) -> Player {
        return match character {
            'x' => Player::X,
            'X' => Player::X,
            'o' => Player::O,
            'O' => Player::O,
            _ => {
                println!("Я не розібрав що ви ввели. Тому, ви будете наказані грою за нолики!");
                Player::O
            }
        };
    }

    pub fn map_player_to_char(player: &Player) -> char {
        return match player {
            Player::X => 'X',
            Player::O => 'O',
        };
    }

    pub fn map_player_to_string(player: &Player) -> &str {
        return match player {
            Player::X => "хрестик",
            Player::O => "нолик",
        };
    }

    pub fn switch_move(player: Player) -> Player {
        return match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    pub fn print_next_player(player: &Player) {
        match player {
            Player::X => println!("===   Наступними ходять хрестики   ==="),
            Player::O => println!("===   Наступними ходять нолики   ==="),
        };
    }
}
