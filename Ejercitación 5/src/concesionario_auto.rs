use crate::auto::Auto;

#[derive(Debug)]
pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_maxima: usize,
    pub autos: Vec<Auto>,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad_maxima: usize) -> Self {
        Self {
            nombre,
            direccion,
            capacidad_maxima,
            autos: Vec::new(),
        }
    }

    pub fn agregar_auto(&mut self, auto: &Auto) -> bool {
        if self.autos.len() < self.capacidad_maxima {
            self.autos.push(auto.clone()); // Clonamos para no mover la propiedad
            true
        } else {
            false
        }
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) -> bool {
        if let Some(index) = self.autos.iter().position(|a| a == auto) {
            self.autos.remove(index);
            true
        } else {
            false
        }
    }

    pub fn buscar_auto(&self, marca: &str, modelo: &str, ano: i32) -> Option<&Auto> {
        self.autos.iter().find(|a| a.marca == marca && a.modelo == modelo && a.ano == ano)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agregar_auto() {
        let mut concesionario = ConcesionarioAuto::new("MiConcesionario".to_string(), "Calle 123".to_string(), 2);
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2015, 10000.0, Color::Rojo);
        let auto2 = Auto::new("BMW".to_string(), "X7".to_string(), 2022, 20000.0, Color::Negro);
        let auto3 = Auto::new("Fiat".to_string(), "Duna".to_string(), 1994, 5000.0, Color::Azul);

        assert!(concesionario.agregar_auto(auto1.clone()));
        assert!(concesionario.agregar_auto(auto2.clone()));
        assert!(!concesionario.agregar_auto(auto3)); // No debe agregarse porque excede la capacidad
    }

    #[test]
    fn test_eliminar_auto() {
        let mut concesionario = ConcesionarioAuto::new("MiConcesionario".to_string(), "Calle 123".to_string(), 2);
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2015, 10000.0, Color::Rojo);
        let auto2 = Auto::new("BMW".to_string(), "X7".to_string(), 2022, 20000.0, Color::Negro);
        
        concesionario.agregar_auto(auto1.clone());
        concesionario.agregar_auto(auto2.clone());
        
        assert!(concesionario.eliminar_auto(&auto1));
        assert!(!concesionario.eliminar_auto(&auto1)); // No debe eliminar un auto que ya no está
    }

    #[test]
    fn test_buscar_auto() {
        let mut concesionario = ConcesionarioAuto::new("MiConcesionario".to_string(), "Calle 123".to_string(), 2);
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2015, 10000.0, Color::Rojo);
        let auto2 = Auto::new("BMW".to_string(), "X7".to_string(), 2022, 20000.0, Color::Negro);
        
        concesionario.agregar_auto(auto1.clone());
        concesionario.agregar_auto(auto2.clone());
        
        let encontrado = concesionario.buscar_auto("Toyota", "Corolla", 2015);
        assert!(encontrado.is_some());
        assert_eq!(encontrado.unwrap(), &auto1);
        
        let no_encontrado = concesionario.buscar_auto("Fiat", "Duna", 1994);
        assert!(no_encontrado.is_none());
    }
}
