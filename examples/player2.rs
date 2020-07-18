use nocturne::game::{Game, Status};

fn main() {
    let mut env = Game::new();

    loop {
        match env.player_move() {
            Ok(Status::OnGoing) => {},
            Ok(s) => { match s {
                Status::WhiteWins => { println!("White Wins!"); }
                Status::BlackWins => { println!("Black Wins!"); }
                _ => { println!("Draw..."); }
                };
                return;
            }
            Err(_) => { println!("Invalid Move!"); }
        }
    };
}