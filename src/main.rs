use std::io::Write;

mod game;
fn main() -> std::io::Result<()> {
    let solution = "apple";
    let mut g = game::Game::new(solution);
    let mut buffer = String::new();
    while !g.is_game_over() {
        buffer.clear();
        print!("Guess {} of 6: ", 6 - g.guesses_left() + 1);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buffer)?;
        let guess = buffer.trim();

        if let Ok(correctness) = g.guess(guess) {
            println!("{}", correctness);
        } else {
            println!("Invalid Guess!");
            continue;
        }
    }

    if g.game_won() {
        println!("You win!");
    } else {
        println!("You lose!\nThe solution was `{}`", solution);
    }

    Ok(())
}
