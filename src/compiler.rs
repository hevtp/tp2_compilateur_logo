//! module : compiler
//!
//! Ce module permet de compiler le programme Logo. Il prend le programme et le transforme en un fichier SVG.

use crate::parser::{AST, extract_order};
use std::f32::consts::PI;
use svg_fmt::*;

/// Structure Logo pour stocker l'état de la tortue
pub struct Logo {
    pub position_x: f32,
    pub position_y: f32,
    pub angle_orientation: f32,
    pub etat_stylo: EtatStylo,
    pub fichier_svg: String,
}

#[derive(PartialEq)] //permet de faire les opération == entre les états du stylo
pub enum EtatStylo {
    Up,
    Down,
}

impl Logo {

    /// Crée une nouvelle tortue et initialise le SVG
    pub fn new(x: f32, y: f32) -> Self {
        let mut svg = String::new();
        svg.push_str(&format!("{}\n", BeginSvg { w: 300.0, h: 300.0 }));
        svg.push('\n');

        Logo {
            position_x: x,
            position_y: y,
            angle_orientation: 0.0,
            etat_stylo: EtatStylo::Down,
            fichier_svg: svg,
        }
    }

    /// Compile l'AST en mettant à jour la position et en générant les lignes SVG
    pub fn compile(&mut self, ast: &AST) {
        match ast {
            AST::Program(commands) => {
                for comm in commands {
                    self.compile(comm);
                }
            }

            AST::Command(commands) => {
                if commands.len() == 2 {
                    let order = &commands[0];
                    let number = &commands[1];

                    let value = if let AST::Number(n) = number { *n as f32 } else { 0.0 };
                    let angle_rad = self.angle_orientation * PI / 180.0; //car cos() utilise des radian

                    // Sauvegarde les anciennes positions
                    let ancien_x = self.position_x;
                    let ancien_y = self.position_y;

                    match extract_order(order) {
                        Some(AST::Forward) => {
                            self.position_x += angle_rad.cos() * value; // cos(x) = adj/hyp
                            self.position_y += angle_rad.sin() * value; //sin(x) = opp/hyp
                        }
                        Some(AST::Backward) => {
                            self.position_x -= angle_rad.cos() * value;
                            self.position_y -= angle_rad.sin() * value;
                        }
                        Some(AST::Left) => self.angle_orientation -= value,
                        Some(AST::Right) => self.angle_orientation += value,
                        _ => {}
                    }

                    // Ajoute une ligne au SVG si le stylo est baissé

                    let violet = Color { r: 128, g: 0, b: 128 }; //Joli violet

                    if self.etat_stylo == EtatStylo::Down
                        && (ancien_x != self.position_x || ancien_y != self.position_y)
                    {
                        let ligne = line_segment(ancien_x, ancien_y, self.position_x, self.position_y)
                            .color(violet)
                            .width(1.0);

                        self.fichier_svg.push_str(&format!("{}\n", ligne));
                    }
                }
            }

            AST::Empty => {}
            _ => {}
        }
    }

    /// Termine le SVG
    pub fn finish(mut self) -> String {
        self.fichier_svg.push_str("</svg>\n");
        self.fichier_svg
    }
}