//! module : parser
//! 
//! Ce module effectue une analyse syntaxique du programme. Il prend en entrée la séquence de tokens produite par le lexer et construit un arbre de syntaxe abstraite qui représente la structure du programme. 

use santiago::grammar::Grammar;

/// Énumération représentant l'arbre de syntaxe abstraite (AST)
#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Program(Vec<AST>),
    Command(Vec<AST>),
    Order(Vec<AST>),
    Forward,
    Backward,
    Left,
    Right,
    Number(i32),
    Empty,
}

/// Définition de la grammaire du langage Logo avec actions AST
pub fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "program" => rules "command" "program" => AST::Program;
        "program" => empty => |_| AST::Empty;

        "command" => rules "order" "number" => AST::Command;

        "order" => rules "forward" => AST::Order;
        "order" => rules "backward" => AST::Order;
        "order" => rules "left" => AST::Order;
        "order" => rules "right" => AST::Order;

        "forward" => lexemes "FORWARD" => |_| AST::Forward;
        "backward" => lexemes "BACKWARD" => |_| AST::Backward;
        "left" => lexemes "LEFT" => |_| AST::Left;
        "right" => lexemes "RIGHT" => |_| AST::Right;

        "number" => lexemes "NUMBER" => |lexemes| {
            let value = str::parse::<i32>(&lexemes[0].raw).unwrap();
            AST::Number(value)
        };
    )
}

/// Fonction d'évaluation de l'AST
pub fn eval(ast: &AST) {
    match ast { 
        AST::Program(commands) => {
            for comm in commands {
                eval(comm); 
            }
        }
        AST::Command(commands) => { 
            if commands.len() == 2 { 
                let order = &commands[0];
                let number = &commands[1];

                let value = if let AST::Number(n) = number { *n } else { 0 }; 

                match extract_order(order) {
                    Some(AST::Forward) => println!("Avance de {} unités", value),
                    Some(AST::Backward) => println!("Recule de {} unités", value),
                    Some(AST::Left) => println!("Tourne à gauche de {} degrés", value),
                    Some(AST::Right) => println!("Tourne à droite de {} degrés", value),
                    _ => {}
                }
            }
        }
        AST::Empty => println!("Stop"),
        _ => {}
    }
}

/// Fonction d'extraction de l'ordre à partir de l'AST
fn extract_order(ast: &AST) -> Option<&AST> {
    match ast {
        AST::Order(commands) if !commands.is_empty() => Some(&commands[0]), 
        AST::Forward | AST::Backward | AST::Left | AST::Right => Some(ast), 
        _ => None, 
    }
}