mod board;
mod game;
mod rules;
mod solver;
mod userinput;

fn main() {
    println!("If you want to load game from file enter filename, otherwise press enter");
    let mut filename = String::new();
    std::io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");
    let filename = filename.trim();

    let mut game = game::Game::new(4, 2, 2);
    if !filename.is_empty() {
        game = game::Game::load_from_file(filename);
    }

    dbg!("Game created");
    game.display();

    loop {
        let x = userinput::await_rule();
        if x.1 == 0 {
            break;
        } else if x.1 == 2 {
            game.add_rule(x.0.unwrap());
        }
    }

    loop {
        game.display();
        let (position, command) = userinput::await_input();

        match position {
            Some((pos, value)) => {
                game.set_value(pos, value);
            }
            None => {
                if command == "quit" {
                    break;
                } else if command == "finish" {
                    let mut solver = solver::Solver::new(game);
                    solver.solve();
                    solver.display_solutions();
                    break;
                } else if !command.is_empty() {
                    game.save_to_file(&command);
                } else {
                    continue;
                }
            }
        }

        let (violations, pending) = game.check_rules();

        if violations.is_none() && pending.is_none() {
            println!("All rules satisfied!");
            println!("Game won");
            break;
        }

        if violations.is_some() {
            println!("Violations:");
            for violation in violations.unwrap() {
                println!("{:?}", violation);
            }
        }

        if pending.is_some() {
            println!("Pending:");
            for pending_rule in pending.unwrap() {
                println!("{:?}", pending_rule);
            }
        }
    }
}
