# TP2 - Compilateur et Interpréteur Logo

Ce projet réimplémente le langage **Logo** en Rust et permet de compiler des programmes Logo en fichiers **SVG**.

## Usage

- Lit et affiche le contenu du fichier Logo.
- Effectue l'analyse lexicale et syntaxique.
- Génère un arbre AST
- Compile le programme en SVG et l’ouvre.

Note : par défaut, l’exemple est le carré (`examples/carre.lg`), mais un autre code Logo peut être utilisé.


```bash
cargo run -- <nom_projet>
