use std::io;
mod lib;
use lib::deck::Deck;

fn main() {
    let deck = Deck;
    let mut wins = 0;
    let mut loses = 0;

    loop {
        let card1 = deck.draw_card();
        println!("The first card is {}", card1.declare_card());
        println!("Will the next card be higher or lower?");
        println!("Enter 1 for lower, 2 for higher.");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u8 = choice.trim().parse().expect("Please type a number");
        let card2 = deck.draw_card();
        println!("The next card is the {}", card2.declare_card());
        let value1 = card1.get_value();
        let value2 = card2.get_value();
        let higher = value2 > value1;
        if value1 == value2 {
            println!("Card values were the same, no winner or loser this round.");
        } else if (higher && choice == 2) || (!higher && choice == 1) {
            wins += 1;
            println!("Winner!");
        } else {
            loses += 1;
            println!("Sorry, your guess was incorrect :(");
        }

        let mut decision = String::new();
        println!("Play again? [y]es or [n]o");
        io::stdin()
            .read_line(&mut decision)
            .expect("Failed to read line");
        let decision = decision.trim();
        if decision == "n" || decision == "N" {
            break;
        }
    }

    println!("Wins: {}", wins);
    println!("Loses: {}", loses);
}
