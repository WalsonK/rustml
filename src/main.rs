use rustml::environment::{monty_hall, lineworld, gridworld};
use std::io;
use rand::Rng;

fn main() {
    let mut monty_hall = monty_hall::MontyHall::new(3);
    println!("Bienvenue au jeu de Monty Hall!");
    println!("Il y a {} portes. Une porte cache un prix.", monty_hall.nb_portes);

    // Choix initial du joueur
    loop {
        println!("Choisissez une porte (0, 1 ou 2):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Échec de la lecture de la ligne");
        let choice: usize = match input.trim().parse() {
            Ok(num) if num < monty_hall.nb_portes => num,
            _ => {
                println!("Entrée invalide, veuillez entrer un nombre entre 0 et {}.", monty_hall.nb_portes - 1);
                continue;
            }
        };

        // Applique le choix initial
        if monty_hall.next_state(choice).is_ok() {
            break;
        } else {
            println!("Action invalide, essayez à nouveau.");
        }
    }

    // Monty ouvre une porte
    let mut rng = rand::thread_rng();
    let opened_door = (0..monty_hall.nb_portes)
        .filter(|&x| x != monty_hall.winning_door && x != monty_hall.chosen_door.unwrap())
        .nth(rng.gen_range(0..monty_hall.nb_portes - 2))
        .unwrap();
    monty_hall.opened_door = Some(opened_door);

    println!("Monty ouvre la porte {}.", opened_door);
    println!("Voulez-vous changer de porte ? (oui/non):");

    // Le joueur décide de changer ou non
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Échec de la lecture de la ligne");
        let input = input.trim().to_lowercase();
        match input.as_str() {
            "oui" => {
                let new_choice = (0..monty_hall.nb_portes)
                    .filter(|&x| x != monty_hall.chosen_door.unwrap() && x != monty_hall.opened_door.unwrap())
                    .next()
                    .unwrap();
                monty_hall.chosen_door = Some(new_choice);
                break;
            }
            "non" => break,
            _ => println!("Entrée invalide, veuillez répondre par 'oui' ou 'non'."),
        }
    }

    // Fin du jeu
    let (reward, _) = monty_hall.step(monty_hall.chosen_door.unwrap());
    if reward > 0.0 {
        println!("Félicitations! Vous avez gagné!");
    } else {
        println!("Désolé, vous avez perdu. La porte gagnante était la porte {}.", monty_hall.winning_door);
    }
    println!("{}", monty_hall);
}





/*mod two_round_rock_paper_scissors;
use crate::two_round_rock_paper_scissors::{Action, Agent, Adversary, Environment};
fn main() {
    let mut env = Environment::new();
    let agent_action_round_1 = env.agent.choose_action();
    let (result_round_1, _) = env.step(agent_action_round_1);
    println!("Round 1: Agent chose {:?}, result: {}", agent_action_round_1, result_round_1);

    let agent_action_round_2 = env.agent.choose_action(); // Agent choisit une action aléatoire pour le deuxième round
    let (result_round_2, done) = env.step(agent_action_round_2); // Passer l'action du deuxième round
    println!("Round 2: Agent chose {:?}, result: {}", agent_action_round_2, result_round_2); // Agent joue la même action que le deuxième round

    if done {
        println!("Game over. Total score: {}", env.agent_score);
    }
}*/