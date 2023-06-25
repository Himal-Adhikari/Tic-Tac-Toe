use std::{
    fmt::Display,
    io::{self, Write},
};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Player1,
    Player2,
    Empty,
}

impl State {
    fn toggle(self) -> State {
        match self {
            State::Player1 => State::Player2,
            State::Player2 => State::Player1,
            State::Empty => unreachable!(),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Player1 => write!(f, "Player1"),
            State::Player2 => write!(f, "Player2"),
            State::Empty => write!(f, ""),
        }
    }
}

const WINNING_STATE: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn main() {
    let mut state = [State::Empty; 9];
    let mut letters = [' '; 9];
    let mut turn = State::Player1;
    let mut print_first = String::new();

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clears the terminal and puts the cursor at the top
        io::stdout().flush().unwrap();
        if !print_first.is_empty() {
            println!("{}", print_first);
            print_first.clear();
        }
        println!("-------------        -------------");
        println!(
            "| {} | {} | {} |        | 1 | 2 | 3 |",
            letters[0], letters[1], letters[2]
        );
        println!("-------------        -------------");
        println!(
            "| {} | {} | {} |        | 4 | 5 | 6 |",
            letters[3], letters[4], letters[5]
        );
        println!("-------------        -------------");
        println!(
            "| {} | {} | {} |        | 7 | 8 | 9 |",
            letters[6], letters[7], letters[8]
        );
        println!("-------------        -------------");

        let previous_player = turn.toggle();
        if won(previous_player, &state) {
            state = [State::Empty; 9];
            letters = [' '; 9];
            print_first = format!("CONGRATULATIONS!!! {} wins", previous_player.to_string());
            turn = State::Player1;
            continue;
        }
        if game_finished(&state) {
            state = [State::Empty; 9];
            letters = [' '; 9];
            turn = State::Player1;
            print_first.push_str("It's a draw");
            continue;
        }

        let mut input = String::new();
        print!("Type any number between 1 and 9 to change that value : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = match input.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                print_first.push_str("Please enter a number");
                continue;
            }
        };
        if !(input <= 9 && input >= 1) {
            print_first.push_str("Please enter a number between 1 and 9");
            continue;
        }
        match state[input - 1] {
            State::Player1 | State::Player2 => {
                print_first.push_str("You cannot change that");
                continue;
            }
            State::Empty => {
                match turn {
                    State::Player1 => {
                        state[input - 1] = State::Player1;
                        letters[input - 1] = 'X';
                    }
                    State::Player2 => {
                        state[input - 1] = State::Player2;
                        letters[input - 1] = 'O';
                    }
                    State::Empty => unreachable!(),
                }
                turn = turn.toggle();
            }
        }
    }
}

fn won(player: State, state: &[State; 9]) -> bool {
    'outer: for vec in WINNING_STATE.iter() {
        for &index in vec.iter() {
            if !(state[index] == player) {
                continue 'outer;
            }
        }
        return true;
    }
    false
}

fn game_finished(state: &[State; 9]) -> bool {
    for player in state.iter() {
        match player {
            State::Empty => return false,
            _ => continue,
        }
    }
    true
}
