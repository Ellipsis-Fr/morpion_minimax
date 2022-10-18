use std::{io, cell::{RefCell, RefMut}, rc::Rc};
use rand::prelude::*;

enum Team {
    AI(char),
    Player(char)
}

fn main() {
    let mut grid = init_grid();
    
    println!("Bienvenue dans mon jeu de morpion !");
    let play_vs_ai = play_vs_ai();
    let (player, ai_or_other_player) = define_player_team(play_vs_ai);
    let player1_start = player1_start(play_vs_ai);

    let mut player1_turn = player1_start;
    
    println!("Pour jouer indiquez à chaque tour la ligne ('A', 'B' ou 'C') et la colonne ('1', '2' ou '3') que vous souhaitez jouer.\nBonne Chance !");
    
    loop {
        print_grid(&grid);

        if player1_turn || !play_vs_ai {
            if player1_turn {
                println!("Tour de joueur 1");
            } else {
                println!("Tour de joueur 2");
            }

            let input = get_input();
            
            let (row, column) = match convert_player_input(input) {
                Ok((row, column)) => (row, column),
                Err(e) => {
                    println!("{}Veuillez rejouer.", e);
                    continue;
                }
            };
    
            if !cell_is_empty(&grid, row, column) {
                println!("Cellule déjà jouée, veulliez en sélectionner une autre.");
                continue;
            }

            let team_symbol = get_team_symbol(&player, &ai_or_other_player, player1_turn);
            
            grid[row as usize][column as usize] = team_symbol;
        } else {
            println!("Ordi joue : ");
            let (row, column) = get_ai_play(&mut grid, &player, &ai_or_other_player);

            let team_symbol = get_team_symbol(&player, &ai_or_other_player, player1_turn);
            
            grid[row as usize][column as usize] = team_symbol;
        }

        
        match end_of_game(&grid) {
            Some(w) => {
                if w == 'd' {
                    println!("Egalité !");
                } else {  
                    if player1_turn {
                        if play_vs_ai {
                            println!("Félicitation joueur 1 tu as gagné contre la machine !!!! °O°'\nCe cas n'est pas censé arriver !");
                        } else {
                            println!("Félicitation joueur 1 tu as gagné !");
                            if player1_start {
                                println!("En commençant tu avais tout de même 51,5% de chance de gagner, seulement 30,5% de perdre et 18% de faire égalité.");
                            } else {
                                println!("En étant second tu n'avais que 30,5% de chance de gagner, contre 50,5% de perdre et 18% de faire égalité.");
                            }
                        }
                    } else {
                        if play_vs_ai {
                            println!("Tu as perdu... Mais c'était couru d'avance... au mieux vise l'égalité.");
                        } else {
                            println!("Félicitation joueur 2 tu as gagné !");
                                if !player1_start {
                                    println!("En commençant tu avais tout de même 51,5% de chance de gagner, seulement 30,5% de perdre et 18% de faire égalité.");
                                } else {
                                    println!("En étant second tu n'avais que 30,5% de chance de gagner, contre 50,5% de perdre et 18% de faire égalité.");
                                }
                        }
                    }                  
                };
                break;
            },
            None => ()
        }
        player1_turn = !player1_turn;
    }


    
}

fn init_grid() -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for y in 0..9 {
        let mut row = Vec::new();
        for x in 0..9 {
            row.push(' ');
        }
        grid.push(row);
    }

    grid
}

fn print_grid(grid: &Vec<Vec<char>>) {
    print_row_name(-1);
    print_column_name();
    println!();

    for y in 0..grid.len() {
        print_row_name(y as i32);
        if y % 3 == 0 {
            print_horizontal_line_separator(grid.len());
            print_row_name(y as i32);
        }
        for x in 0..=grid[y].len() {
            if x == 0 || x % 3 == 0 { print!("|"); } 
            if x != 9 {print!("{}", grid[y][x]);}
        }
        println!();
    }

    print_row_name(-1);
    print_horizontal_line_separator(grid.len());
}

fn print_horizontal_line_separator(length: usize) {
    for _ in 0..length + 5 {
        print!("_");
    }
    println!();
}

fn print_row_name(i: i32) {
    match i {
        1 => print!(" A "),
        4 => print!(" B "),
        7 => print!(" C "),
        _ => print!("   ")
    }
}

fn print_column_name() {
    print!("  1   2   3 ");
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Problème lors de la lecture de l'entrée joueur.");
    input.trim().to_uppercase().to_string()
}

fn play_vs_ai() -> bool {
    println!("Souhaitez-vous jouer Seul ou à 2 ? '1' ou '2'");

    loop {
        let input = get_input();
        let input_chars:Vec<char> = input.chars().collect();
    
        match check_input_format(&input_chars, "Player") {
            Err(e) => {
                println!("{}", format!("Format non respecté :\n{}", e));
                println!("Indiquez : '1' ou '2'");
            },
            Ok(_) => {
                if input == "1" { return true; }
                return false;
            }
        }
    }
    
}

fn define_player_team(play_vs_ai: bool) -> (Team, Team) {
    if play_vs_ai {
        println!("Quelle équipe souhaitez-vous jouer ? 'X', 'O' ou '?' ?");
    } else {
        println!("Quelle équipe Joueur 1 joue-t-il ? 'X', 'O' ou '?' ?");
    }
    
    loop {
        let input = get_input();
        let input_chars:Vec<char> = input.chars().collect();
    
        match check_input_format(&input_chars, "Team") {
            Err(e) => {
                println!("{}", format!("Format non respecté :\n{}", e));
                println!("Indiquez : 'X', 'O' ou '?'");
            },
            Ok(_) => {
                match input_chars[0] {
                    'X' => {
                        return (Team::Player('X'), Team::AI('O'));
                    },
                    'O' => {
                        return (Team::Player('O'), Team::AI('X'));
                    },
                    '?' => {
                        let mut rng = thread_rng();
                        let x: f32 = rng.gen();
                        if x > 0.5 { return (Team::Player('X'), Team::AI('O')); }
                        else { return (Team::Player('O'), Team::AI('X'));}
                    },
                    _ => ()
                }
            }
        }

    }
}

fn player1_start(play_vs_ai: bool) -> bool {
    if play_vs_ai {
        println!("Souhaitez-vous commencer ? 'Y' ou 'N' ?");
    } else {
        println!("Joueur 1 souhaitez-vous commencer ? 'Y' ou 'N' ?");
    }

    loop {
        let input = get_input();
        let input_chars:Vec<char> = input.chars().collect();
    
        match check_input_format(&input_chars, "Start") {
            Err(e) => {
                println!("{}", format!("Format non respecté :\n{}", e));
                println!("Indiquez : 'Y' ou 'N'");
            },
            Ok(_) => {
                if input == "Y" { return true; }
                return false;
            }
        }
    }
}

fn convert_player_input(player_input: String) -> Result<(u32, u32), String> {
    let player_input = player_input.split_whitespace().collect::<Vec<&str>>().iter().map(|c| c.to_string()).collect::<String>();
    let row_and_column_played:Vec<char> = player_input.chars().collect();

    match check_input_format(&row_and_column_played, "Game") {
        Err(e) => return Err(format!("Format non respecté :\n{}", e)),
        Ok(_) => ()
    }
    
    let mut row:u32 = match row_and_column_played[0] {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0
    };

    let mut column = row_and_column_played[1].to_digit(10).unwrap();

    row = edit_position(row);
    column = edit_position(column);

    Ok((row, column))
}

fn check_input_format(chars: &Vec<char>, category: &str) -> Result<(), String> {
    match category {
        "Player" => return check_input_number_of_players(&chars),
        "Team" => return check_input_selected_team(&chars),
        "Start" => return check_input_start(&chars),
        "Game" => return check_input_played(&chars),
        _ => return Ok(())
    }
}

fn check_input_number_of_players(chars: &Vec<char>) -> Result<(), String> {
    if chars.is_empty() || chars.len() > 1 {
        return Err(format!("La saisie ne doit comporter qu'un caractère.\n"));
    }

    match chars[0] {
        '1' | '2' => (),
        _ => {
            return Err(format!("{} n'est pas un caractère valide pour défnir le nombre de joeur.\n", chars[0]));
        }
    }

    Ok(())
}

fn check_input_selected_team(chars: &Vec<char>) -> Result<(), String> {
    if chars.is_empty() || chars.len() > 1 {
        return Err(format!("La saisie ne doit comporter qu'un caractère.\n"));
    }

    match chars[0] {
        'X' | 'O' | '?' => (),
        _ => {
            return Err(format!("{} n'est pas un caractère valide pour sélectionner une équipe.\n", chars[0]));
        }
    }

    Ok(())
}

fn check_input_start(chars: &Vec<char>) -> Result<(), String> {
    if chars.is_empty() || chars.len() > 1 {
        return Err(format!("La saisie ne doit comporter qu'un caractère.\n"));
    }

    match chars[0] {
        'Y' | 'N' => (),
        _ => {
            return Err(format!("{} n'est pas un caractère valide pour indiquer si vous souhaitez commencer.\n", chars[0]));
        }
    }

    Ok(())
}

fn check_input_played(chars: &Vec<char>) -> Result<(), String> {
    if chars.is_empty() || chars.len() > 2 {
        return Err(format!("La saisie ne doit comporter que 2 caractères.\n"));
    }

    let mut input_error = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i as u32 == 0 {
            let decimal = *c as u32;
            if decimal < 65 || decimal > 67 { input_error.push_str(&format!("{} n'est pas un caractère valide pour désigner une ligne (Renseignez plutôt 'A', 'B' ou 'C').\n", *c).to_string()); }
        } else {
            let digit = *c as u32 - '0' as u32;
            if digit > 3 { input_error.push_str(&format!("{} n'est pas un chiffre valide pour désigner une colone (Renseignez plutôt '1', '2' ou '3').\n", *c).to_string()); }
        }
    }

    if !input_error.is_empty() {
        return Err(input_error);
    }

    Ok(())
}

fn edit_position(i: u32) -> u32 {
    let j = i - 1;
    j * 3 + 1
}

fn cell_is_empty(grid: &Vec<Vec<char>>, row: u32, column: u32) -> bool {
    if grid[row as usize][column as usize] == ' ' { return true; }
    false
}

fn get_team_symbol(player: &Team, ai_or_other_player: &Team, player1_turn: bool) -> char {
    if player1_turn {
        match player {
            Team::Player(c) => return *c,
            _ => panic!("Erreur de récupération du symbole du premier joueur !")
        }
    } else {
        match ai_or_other_player {
            Team::AI(c) => return *c,
            _ => panic!("Erreur de récupération du symbole du second joueur !")
        }
    }
}

fn end_of_game(grid: &Vec<Vec<char>>) -> Option<char> {
    if grid[1][1] != ' ' {
        let symbol_to_look = grid[1][1];
        if 
            (symbol_to_look == grid[1][4] && symbol_to_look == grid[1][7])
            ||
            (symbol_to_look == grid[4][4] && symbol_to_look == grid[7][7])
            ||
            (symbol_to_look == grid[4][1] && symbol_to_look == grid[7][1])
        { return Some(symbol_to_look); }
    }
    if grid[4][1] != ' ' {
        let symbol_to_look = grid[4][1];
        if symbol_to_look == grid[4][4] && symbol_to_look == grid[4][7] { return Some(symbol_to_look); }
    }
    if grid[7][1] != ' ' {
        let symbol_to_look = grid[7][1];
        if 
            (symbol_to_look == grid[7][4] && symbol_to_look == grid[7][7])
            ||
            (symbol_to_look == grid[4][4] && symbol_to_look == grid[1][7])
        { return Some(symbol_to_look); }
    }
    if grid[1][4] != ' ' {
        let symbol_to_look = grid[1][4];
        if symbol_to_look == grid[4][4] && symbol_to_look == grid[7][4] { return Some(symbol_to_look); }
    }
    if grid[1][7] != ' ' {
        let symbol_to_look = grid[1][7];
        if symbol_to_look == grid[4][7] && symbol_to_look == grid[7][7] { return Some(symbol_to_look); }
    }

    for row in (1..grid.len()).step_by(3) {
        for column in (1..grid[row].len()).step_by(3) {
            if grid[row][column] == ' ' { return None; }
        }
    }

    Some('d')
}

fn get_ai_play(grid: &mut Vec<Vec<char>>, player: &Team, ai_or_other_player: &Team) -> (u32, u32) {
    let player_symbol = get_team_symbol(&player, &ai_or_other_player, true);
    let ai_symbol = get_team_symbol(&player, &ai_or_other_player, false);

    let (mut row, mut column) = (1, 1);
    let mut score_max = i32::MIN;

    for y in (1..grid.len()).step_by(3) {
        for x in (1..grid[y].len()).step_by(3) {
            if cell_is_empty(&grid, y as u32, x as u32) {
                grid[y][x] = ai_symbol;
                let score = get_minimax(grid, 0, false, player_symbol, ai_symbol);

                if score > score_max {
                    score_max = score;
                    row = y as u32;
                    column = x as u32;
                }
                grid[y][x] = ' ';
            }
        }    
    }

    (row, column)
}

fn get_minimax(grid: &mut Vec<Vec<char>>, depth: u32, minimax: bool, player_symbol: char, ai_symbol: char) -> i32 {
    match end_of_game(&grid) {
        Some(w) => {
            if w == 'd' {
                return 0;
            } else {
                if minimax {
                    if w == ai_symbol {
                        return 10;
                    } else {
                        return -10;
                    }
                } else {
                    if w == ai_symbol {
                        return 10;
                    } else {
                        return -10;
                    }
                }
            }
        },
        None => {
            if minimax {
                let mut score_max = i32::MIN;

                for y in (1..grid.len()).step_by(3) {
                    for x in (1..grid[y].len()).step_by(3) {
                        if cell_is_empty(&grid, y as u32, x as u32) {
                            grid[y][x] = ai_symbol;
                            let score = get_minimax(grid, 0, false, player_symbol, ai_symbol);

                            if score > score_max {
                                score_max = score;
                            }
                            grid[y][x] = ' ';
                        }
                    }    
                }

                return score_max;
            } else {
                let mut score_min = i32::MAX;

                for y in (1..grid.len()).step_by(3) {
                    for x in (1..grid[y].len()).step_by(3) {
                        if cell_is_empty(&grid, y as u32, x as u32) {
                            grid[y][x] = player_symbol;
                            let score = get_minimax(grid, 0, true, player_symbol, ai_symbol);

                            if score < score_min {
                                score_min = score;
                            }
                            grid[y][x] = ' ';
                        }
                    }    
                }

                return score_min;
            }
        }
    }
}