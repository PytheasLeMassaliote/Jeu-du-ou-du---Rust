use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devinez le nombre");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);
    let mut essais = 0;

    loop {
        println!("Veuillez entrer un nombre ou tapez 'quit' pour quitter le jeu.");
        

        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        if supposition.trim().to_lowercase() == "quit" {
            println!("Vous avez quitté le jeu, voici le nombre secret : {}", nombre_secret);
            break;
        }

        if !supposition.trim().chars().all(|c| c.is_digit(10)) {
            println!("Veuiller entrer uniquement des caratères numériques.");
            continue;
        }

        let supposition: i32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => {
                println!("Veuillez entrer un nombre valide.");
                continue;
            }
        };

        if supposition < 1 || supposition > 100 {
            println!("Le numéro secret est entre 1 et 100.");
            continue;
        }

        println!("Votre nombre : {}", supposition);

        essais += 1;

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                println!("Voulez-vous continuer ? (Tapez 'Y' pour continuer ou 'N' pour quitter");

                let mut reponse = String::new();
                io::stdin()
                    .read_line(&mut reponse)
                    .expect("Echec de la lecture de la réponse");

                match reponse.trim().to_lowercase().as_str() {
                    "y" => continue,
                    "n" => {
                        println!("Vous avez décidé de quitter. Au revoir :D !");
                        return;
                    }
                    _ => {
                        println!("Réponse invalide. Veuillez entrer 'Y' pour continuer ou 'N' pour quitter");
                        continue;
                    }
                }
            }
        }

        if essais >= 5 {
            println!("Vous avez essayé plus de 5 fois sans trouver.");

            println!("Voulez vous connaître le nombre mystère ? (Tapez Y pour oui et N pour non");

            let mut reponse = String::new();
            io::stdin()
                .read_line(&mut reponse)
                .expect("Echec lors de la lecture de la réponse");

            match reponse.trim().to_lowercase().as_str() {
                "y" => {
                    println!("Le nombre mystère était : {}", nombre_secret);
                    println!("Voulez-vous continuer ?");

                    let mut reponse = String::new();
                    io::stdin()
                        .read_line(&mut reponse)
                        .expect("Echec lors de la lecture de la réponse");

                    match reponse.trim().to_lowercase().as_str() {
                        "y" => continue,
                        "n" => {
                            println!("Vous avez choisi de quitter");
                            return;
                        }
                        _ => println!("Réponse invalide, Veuillez entre Y pour oui et N pour non"),
                    }
                }

                    "n" => {
                        println!("Vous avez choisi de ne pas connaître le nombre secret");
                        println!("Voulez-vous continuer ?");

                        let mut reponse = String::new();
                        io::stdin()
                        .read_line(&mut reponse)
                        .expect("Echec lors de la lecteur de la réponse");

                    match reponse.trim().to_lowercase().as_str() {
                        "y" => continue,
                        "n" => {
                            println!("Vous avez choisi de quitter");
                            return;
                        }
                        _ => println!("Réponse invalide. Veuillez entrer Y pour oui ou N pour non"),
                    }
                }
                _ => println!("Réponse invalide. Veuillez entrer Y pour oui et N pour non"),
            }
            essais = 0;
        }
    }
}