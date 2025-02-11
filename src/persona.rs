// src/persona.rs
#[derive(Debug, PartialEq)]
pub struct Persona {
    nombre: String,
    edad: u8,
    direccion: Option<String>,
}

impl Persona {
    // Método `new`: crea y retorna una nueva instancia de `Persona`.
    pub fn new(nombre: String, edad: u8, direccion: Option<String>) -> Self {
        Self { nombre, edad, direccion }
    }

    // Método `imprimir`: imprime los datos de la persona.
    pub fn imprimir(&self) {
        println!(
            "Nombre: {}, Edad: {}, Dirección: {}",
            self.nombre,
            self.edad,
            self.direccion.as_deref().unwrap_or("No especificada")
        );
    }

    // Método `obtener_edad`: retorna la edad de la persona.
    pub fn obtener_edad(&self) -> u8 {
        self.edad
    }

    // Método `actualizar_direccion`: actualiza la dirección de la persona.
    pub fn actualizar_direccion(&mut self, nueva_direccion: String) {
        self.direccion = Some(nueva_direccion);
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_persona() {
        let persona = Persona::new(
            "Juan".to_string(),
            30,
            Some("Calle Falsa 123".to_string()),
        );
        assert_eq!(
            persona,
            Persona {
                nombre: "Juan".to_string(),
                edad: 30,
                direccion: Some("Calle Falsa 123".to_string()),
            }
        );
    }

    #[test]
    fn test_imprimir() {
        let persona = Persona::new("Ana".to_string(), 25, None);
        persona.imprimir(); // Visualmente debería mostrar "Nombre: Ana, Edad: 25, Dirección: No especificada".
    }

    #[test]
    fn test_obtener_edad() {
        let persona = Persona::new("Carlos".to_string(), 40, None);
        assert_eq!(persona.obtener_edad(), 40);
    }

    #[test]
    fn test_actualizar_direccion() {
        let mut persona = Persona::new("Laura".to_string(), 22, None);
        persona.actualizar_direccion("Avenida Siempre Viva 742".to_string());
        assert_eq!(
            persona.direccion,
            Some("Avenida Siempre Viva 742".to_string())
        );
    }
}
