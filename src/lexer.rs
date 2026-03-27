//! module : lexer
//!
//! Ce module effectue une analyse lexicale du programme. Autrement dit, il prend le texte brut du programme et le transforme en une séquence de tokens ou lexemes (unités de base du langage)
//! Il est capable, pour l'instant, de reconnaîte les élements simples du language :
//!- Les commandes de direction : forward, backward, left, right
//!- les nombres
//! - éviter les espaces et annoncer une erreur en cas de caractère inattendu

use santiago::lexer::LexerRules;
santiago::def!(ANY, r"(?:.|\n)"); // tout les caractères


///Construction du lexique des commandes en associant l'état correspondant si l'état est défaut
pub fn lexer_rules() -> LexerRules{
    santiago::lexer_rules!(

        // Les directions :

        // Forward
        "DEFAULT" | "FORWARD" = string "forward";
        // Backward
        "DEFAULT" | "BACKWARD" = string "backward";
        // Left
        "DEFAULT" | "LEFT" = string "left";
        // Right
        "DEFAULT" | "RIGHT" = string "right";

        // Number
        "DEFAULT" | "NUMBER" = pattern r"[0-9]+";

        // Les espaces " " sont ignorés
        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();

        // Erreur
        "DEFAULT" | "ERROR" = pattern ANY!() => |lexer| {
            lexer.error("Caractère inattendu")
        };
    )
}