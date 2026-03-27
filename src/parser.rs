//! module : parser
//! 
//! Ce module effectue une analyse syntaxique du programme. Il prend en entrée la séquence de tokens produite par le lexer et construit un arbre de syntaxe abstraite qui représente la structure du programme. 

use santiago::grammar::Grammar; 

/// Définition de la grammaire du langage Logo
pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "program" => rules "command" "program"; 
        "program" => empty;

        "command" => rules "order" "number";

        "order" => lexemes "FORWARD";
        "order" => lexemes "BACKWARD";
        "order" => lexemes "LEFT";
        "order" => lexemes "RIGHT";

        "number" => lexemes "NUMBER";
    )
}