mod persona;
mod fecha;
mod auto;
mod concesionario_auto;

use persona::Persona;
use fecha::Fecha;
use auto::{Auto, Color};
use concesionario_auto::ConcesionarioAuto;

fn main() {
    let mut person = Persona::new("Pedro".to_string(), 28, None);
    person.imprimir();
    person.actualizar_direccion("Nueva Calle 456".to_string());
    person.imprimir();
    println!("person.obtener_edad() devuelve: {}", person.obtener_edad());

    println!("---------------------");

    let mut date = Fecha::new(12, 12, 2100);
    println!("¿Es año bisiesto?: {}", date.es_bisiesto());
    println!("¿Es fecha válida?: {}", date.es_fecha_valida());

    date.sumar_dias(3);
    println!("Nueva fecha después de sumar 3 días: {}/{}/{}", date.day, date.month, date.year);

    date.restar_dias(5);
    println!("Nueva fecha después de restar 5 días: {}/{}/{}", date.day, date.month, date.year);

    let other_date = Fecha::new(1, 3, 2020);
    println!(
        "¿La fecha actual es mayor que 1/3/2020?: {}",
        date.es_mayor(&other_date)
    );

    println!("---------------------");

    let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2015, 10000.0, Color::Rojo);
    println!("El precio final del auto es: ${}", auto1.calcular_precio());

    let auto2 = Auto::new("BMW".to_string(), "X7".to_string(), 2022, 10000.0, Color::Negro);
    println!("El precio final del auto es: ${}", auto2.calcular_precio());

    let auto3 = Auto::new("Fiat".to_string(), "Duna".to_string(), 1994, 10000.0, Color::Azul);
    println!("El precio final del auto es: ${}", auto3.calcular_precio());

    println!("---------------------");

    let mut concesionario1 = ConcesionarioAuto::new("Consecionario 1".to_string(), "Calle 13 y 54".to_string(), 180);
    println!("Agregando auto 1..");
    concesionario1.agregar_auto(&auto1);
    println!("Buscando auto 1..");
    match concesionario1.buscar_auto("Toyota", "Corolla", 2015) {
        Some(auto) => println!("Auto encontrado: {:?}", auto),
        None => println!("Auto no encontrado."),
    }

    println!("Agregando auto 2..");
    concesionario1.agregar_auto(&auto2);
    println!("Buscando auto 2..");
    match concesionario1.buscar_auto("BMW", "X7", 2022) {
        Some(auto) => println!("Auto encontrado: {:?}", auto),
        None => println!("Auto no encontrado."),
    }

    println!("Eliminando auto 2..");
    concesionario1.eliminar_auto(&auto2);
    println!("Buscando auto 2..");
    match concesionario1.buscar_auto("BMW", "X7", 2022) {
        Some(auto) => println!("Auto encontrado: {:?}", auto),
        None => println!("Auto no encontrado."),
    }
    
    println!("Buscando auto 3..");
    match concesionario1.buscar_auto("Fiat", "Duna", 1994) {
        Some(auto) => println!("Auto encontrado: {:?}", auto),
        None => println!("Auto no encontrado."),
    }
}
