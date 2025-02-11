#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Auto {
    pub marca: String,
    pub modelo: String,
    pub ano: i32,
    pub precio_bruto: f64,
    pub color: Color,
}

impl Auto {
    pub fn new(marca: String, modelo: String, ano: i32, precio_bruto: f64, color: Color) -> Self {
        Self { marca, modelo, ano, precio_bruto, color }
    }
    
    pub fn calcular_precio(&self) -> f64 {
        let mut precio = self.precio_bruto;
        
        if matches!(self.color, Color::Rojo | Color::Azul | Color::Amarillo) {
            precio *= 1.25;
        } else {
            precio *= 0.90;
        }
        
        if self.marca.to_lowercase() == "bmw" {
            precio *= 1.15;
        }
        
        if self.ano < 2000 {
            precio *= 0.95;
        }
        
        precio
    }
}