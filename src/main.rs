//! TP2 - Compilateur et Interpréteur Logo par Agathe Julien et Hevisinda Top
//! Objectif : faire revivre le language logo en créeant un compliateur logo vers svg, puis un intérprateur logo.


use std::fs;

//Import des modules de lib
use tp2_compilateur_logo::lexer;
use tp2_compilateur_logo::svg::draw_svg;
fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Lecture du programme Logo");

    let programme = fs::read_to_string("examples/commandes_simples.lg")
        .expect("N'arrive pas à lire le fichier");

    let lexer_rules  = lexer::lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &programme).unwrap();
    println!("{:?}", lexemes);

    println!("Génération du carré");
    draw_svg()?;
    open::that("vsg_square.svg")?;

    Ok(())
}
