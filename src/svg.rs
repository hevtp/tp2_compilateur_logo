//! Module : svg
//!
//! Ce module nous permet de tester la bibliothèque de dessin SVG.
//! Il génère un fichier svg contenant l'exemple du carré.
//!
//! Il s'agit de la partie 3 du TP.

use svg_fmt::*;
use std::{fs, result};

pub fn draw_svg() -> result::Result<(), Box<dyn std::error::Error>> {
    //crée une chaîne vide où on va ajouter le contenu du vsg
    let mut svg = String::new();
    //Ligne obligatoire pour un fichier svg
    svg.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    //définit les dimensions de notre fichier
    svg.push_str(&format!("{}\n", BeginSvg { w: 300.0, h: 300.0 }));

    //définit les lignes du carré avec les coordonnées
    let lines = [
        line_segment(100.0, 100.0, 200.0, 100.0).color(red()).width(1.0),
        line_segment(200.0, 100.0, 200.0, 200.0).color(red()).width(1.0),
        line_segment(200.0, 200.0, 100.0, 200.0).color(red()).width(1.0),
        line_segment(100.0, 200.0, 100.0, 100.0).color(red()).width(1.0),
    ];

    //rajoute les ligne au svg
    for line in lines {
        svg.push_str("    ");
        svg.push_str(&format!("{}\n", line));
    }

    svg.push_str(&format!("{}\n", EndSvg));

    //erreur si n'arrive pas à écrire
    if let Err(e) = fs::write("vsg_square.svg", svg) {
        eprintln!("N'arrive pas à écrire dans vsg_square.svg: {}", e);
        return Err(e.into());
    }

    println!("vsg_square.svg a bien été généré !");

    Ok(())
}
