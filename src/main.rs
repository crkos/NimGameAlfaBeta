use rand::Rng;

#[derive(Clone)]
struct Player {
    name: String,
    is_computer: bool,
}

impl Player {
    fn new(name: String, is_computer: bool) -> Self {
        Self { name, is_computer }
    }
}

#[derive(Clone)]
struct NimGame {
    current_player: Player,
    total_sticks: u32,
}

impl NimGame {
    fn new(player1: Player, player2: Player, total_sticks: u32) -> Self {
        Self {
            current_player: player1,
            total_sticks,
        }
    }

    fn is_game_over(&self) -> bool {
        self.total_sticks == 0
    }

    fn take_sticks(&mut self, num_sticks: u32) -> Result<(), String> {
        if num_sticks == 0 || num_sticks > 3 || num_sticks > self.total_sticks {
            return Err(String::from("Invalid number of sticks!"));
        }

        self.total_sticks -= num_sticks;
        self.switch_players();
        Ok(())
    }

    fn switch_players(&mut self) {
        if self.current_player.is_computer {
            self.current_player = Player::new(String::from("Human"), false);
        } else {
            self.current_player = Player::new(String::from("Computer"), true);
        }
    }

    fn get_successors(&self) -> Vec<NimGame> {
        let mut successors = vec![];

        for num_sticks in 1..=3 {
            if num_sticks <= self.total_sticks {
                let mut new_game = self.clone();
                new_game.take_sticks(num_sticks).unwrap();
                successors.push(new_game);
            }
        }

        successors
    }

    fn minimax(&self, depth: u32, is_maximizing: bool, alpha: i32, beta: i32) -> i32 {
        if depth == 0 || self.is_game_over() {
            return if self.current_player.is_computer { 1 } else { -1 };
        }

        let mut alpha = alpha;
        let mut beta = beta;

        if is_maximizing {
            let mut max_eval = i32::MIN;
            let successors = self.get_successors();

            for successor in successors {
                let eval = successor.minimax(depth - 1, false, alpha, beta);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }

            max_eval
        } else {
            let mut min_eval = i32::MAX;
            let successors = self.get_successors();

            for successor in successors {
                let eval = successor.minimax(depth - 1, true, alpha, beta);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }

            min_eval
        }
    }

    fn get_best_move(&self) -> u32 {
        let max_sticks = std::cmp::min(3, self.total_sticks);

        let mut best_value = i32::MIN;
        let mut best_move = 0;
        let mut alpha = i32::MIN;
        let beta = i32::MAX;

        for num_sticks in 1..=max_sticks {
            let mut new_game = self.clone();
            new_game.take_sticks(num_sticks).unwrap();
            let value = new_game.minimax(10, false, alpha, beta);

            if value > best_value {
                best_value = value;
                best_move = num_sticks;
            }
            alpha = alpha.max(value);
        }

        best_move
    }
}

fn main() {
    let player1 = Player::new(String::from("Human"), false);
    let player2 = Player::new(String::from("Computer"), true);
    let total_sticks = rand::thread_rng().gen_range(4..=20);
    let mut game = NimGame::new(player1, player2, total_sticks);

    while !game.is_game_over() {
        println!("Total sticks: {}", game.total_sticks);
        println!("Current player: {}", game.current_player.name);

        let num_sticks = if game.current_player.is_computer {
            let best_move = game.get_best_move();
            println!("Computer takes: {} sticks", best_move);
            best_move
        } else {
            println!("How many sticks do you want to take?");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            input.trim().parse().unwrap()
        };

        match game.take_sticks(num_sticks) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    println!("Game over!");
    println!("Winner: {}", game.current_player.name);
}

