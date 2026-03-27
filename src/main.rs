//! TP2 - Compilateur et Interpréteur Logo par Agathe Julien et Hevisinda Top
//! Objectif : faire revivre le language logo en créeant un compliateur logo vers svg, puis un intérprateur logo.
//!
use std::fs;
use tp2_compilateur_logo::{lexer, parser};

fn main() {
    println!("=== Compilation Logo ===\n");

    let file_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "examples/carre.lg".to_string());

    // Lecture du fichier
    let programme = fs::read_to_string(&file_path)
        .expect("N'arrive pas à lire le fichier");

    println!("Fichier lu : {}", file_path);

    println!("Programme Logo :\n{}\n", programme);

    // Analyse lexicale
    println!("--- Phase Lexicale ---");
    let rules = lexer::lexer_rules();
    let tokens = santiago::lexer::lex(&rules, &programme).unwrap();
    println!("Tokens : {:?}\n", tokens);

    // Analyse syntaxique
    println!("--- Phase Syntaxique ---");
    let grammar = parser::grammar(); 
    match santiago::parser::parse(&grammar, &tokens) { //parse les tokens selon la grammaire
        Ok(parse_trees) => { 
            println!("Parse tree :\n{}", parse_trees[0]);
            
            println!("--- Arbre de syntaxe abstraite (AST) ---");
            let ast = parse_trees[0].as_abstract_syntax_tree();
            println!("{:?}\n", ast);

            println!("--- Evaluation ---");
            parser::eval(&ast);
        }
        Err(e) => {
            println!("syntax error: {}", e);
        }
    }
}