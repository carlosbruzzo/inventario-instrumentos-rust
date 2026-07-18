use polodb_core::{
    CollectionT, Database,
    bson::{self, Document, doc, from_document, to_document},
};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
enum TipoInstrumento {
    Cuerdas { cantidad_cuerdas: u8 },
    Teclado { digital: bool },
}

#[derive(Debug, Serialize, Deserialize)]
struct Instrumento {
    nombre: String,
    marca: String,
    tipo: TipoInstrumento,
}

fn leer(mensaje: &str) -> String {
    print!("{}", mensaje);
    io::stdout().flush().unwrap();
    let mut texto = String::new();
    io::stdin().read_line(&mut texto).unwrap();
    texto.trim().to_string()
}

fn leer_numero(mensaje: &str, min: u8, max: u8) -> u8 {
    loop {
        let texto = leer(mensaje);
        match texto.parse::<u8>() {
            Ok(numero) if numero >= min && numero <= max => return numero,
            Ok(_) => println!("El número debe estar entre {} y {}.", min, max),
            Err(_) => println!("Eso no es un número válido. Intenta de nuevo."),
        }
    }
}

fn leer_si_no(mensaje: &str) -> bool {
    loop {
        let texto = leer(mensaje).to_lowercase();
        match texto.as_str() {
            "s" => return true,
            "n" => return false,
            _ => println!("Por favor responde 's' o 'n'."),
        }
    }
}

fn main() {
    let db = Database::open_path("inventario.db").expect("No se pudo abrir la base de datos");
    println!("=== INVENTARIO DE INSTRUMENTOS ===");
    loop {
        println!();
        println!("1 - Agregar instrumento");
        println!("2 - Ver instrumentos");
        println!("3 - Buscar por nombre o marca");
        println!("4 - Borrar instrumento");
        println!("5 - Salir");
        let opcion = leer("Opción: ");
        match opcion.as_str() {
            "1" => agregar_instrumento(&db),
            "2" => ver_instrumentos(&db),
            "3" => buscar_instrumento(&db),
            "4" => borrar_instrumento(&db),
            "5" => {
                println!("Programa finalizado.");
                break;
            }
            _ => println!("Opción incorrecta"),
        }
    }
}

fn agregar_instrumento(db: &Database) {
    let coleccion = db.collection::<Document>("instrumentos");
    let nombre = leer("Nombre: ");
    let marca = leer("Marca: ");
    println!("Tipo:");
    println!("1 - Cuerdas");
    println!("2 - Teclado");
    let opcion = leer("Seleccione: ");
    let tipo = match opcion.as_str() {
        "1" => {
            let cantidad = leer_numero("Cantidad de cuerdas: ", 1, 12);
            TipoInstrumento::Cuerdas {
                cantidad_cuerdas: cantidad,
            }
        }
        "2" => {
            let digital = leer_si_no("¿Es digital? (s/n): ");
            TipoInstrumento::Teclado { digital }
        }
        _ => {
            println!("Tipo inválido.");
            return;
        }
    };
    let instrumento = Instrumento {
        nombre,
        marca,
        tipo,
    };
    let documento = to_document(&instrumento).expect("Error convirtiendo a BSON");
    coleccion.insert_one(documento).expect("Error insertando");
    println!("Instrumento guardado.");
}

fn ver_instrumentos(db: &Database) {
    let coleccion = db.collection::<Document>("instrumentos");
    println!();
    println!("=== INVENTARIO ===");
    let mut cursor = coleccion.find(doc! {}).run().expect("Error al consultar");
    let mut encontrados = 0;
    while let Some(resultado) = cursor.next() {
        let documento = resultado.expect("Error leyendo documento");
        let instrumento: Instrumento =
            from_document(documento).expect("Error convirtiendo documento");
        encontrados += 1;
        let detalle = match instrumento.tipo {
            TipoInstrumento::Cuerdas { cantidad_cuerdas } => {
                format!("Cuerdas ({} cuerdas)", cantidad_cuerdas)
            }
            TipoInstrumento::Teclado { digital } => {
                if digital {
                    "Teclado Digital".to_string()
                } else {
                    "Teclado Acústico".to_string()
                }
            }
        };
        println!(
            "{} | Marca: {} | {}",
            instrumento.nombre, instrumento.marca, detalle
        );
    }
    if encontrados == 0 {
        println!("No hay instrumentos registrados.");
    }
}

fn buscar_instrumento(db: &Database) {
    let coleccion = db.collection::<Document>("instrumentos");
    let texto = leer("Buscar (nombre o marca): ");

    let patron = bson::Regex {
        pattern: texto,
        options: "i".to_string(), // "i" = case-insensitive
    };

    let filtro = doc! {
        "$or": [
            { "nombre": { "$regex": patron.clone() } },
            { "marca": { "$regex": patron } },
        ]
    };

    println!();
    println!("=== RESULTADOS ===");
    let mut cursor = coleccion.find(filtro).run().expect("Error al consultar");
    let mut encontrados = 0;

    while let Some(resultado) = cursor.next() {
        let documento = resultado.expect("Error leyendo documento");
        let instrumento: Instrumento =
            from_document(documento).expect("Error convirtiendo documento");
        encontrados += 1;

        let detalle = match instrumento.tipo {
            TipoInstrumento::Cuerdas { cantidad_cuerdas } => {
                format!("Cuerdas ({} cuerdas)", cantidad_cuerdas)
            }
            TipoInstrumento::Teclado { digital } => {
                if digital {
                    "Teclado Digital".to_string()
                } else {
                    "Teclado Acústico".to_string()
                }
            }
        };

        println!(
            "{} | Marca: {} | {}",
            instrumento.nombre, instrumento.marca, detalle
        );
    }

    if encontrados == 0 {
        println!("No se encontraron coincidencias.");
    }
}
fn borrar_instrumento(db: &Database) {
    let coleccion = db.collection::<Document>("instrumentos");
    let nombre = leer("Nombre exacto del instrumento a borrar: ");

    let filtro = doc! { "nombre": nombre.clone() };

    // Primero mostramos qué se va a borrar, para confirmar
    let encontrado = coleccion.find_one(filtro.clone()).expect("Error al buscar");

    match encontrado {
        Some(documento) => {
            let instrumento: Instrumento =
                from_document(documento).expect("Error convirtiendo documento");
            println!(
                "Encontrado: {} | Marca: {}",
                instrumento.nombre, instrumento.marca
            );
            let confirmar = leer_si_no("¿Confirmás que querés borrarlo? (s/n): ");
            if confirmar {
                coleccion.delete_one(filtro).expect("Error al borrar");
                println!("Instrumento borrado.");
            } else {
                println!("Operación cancelada.");
            }
        }
        None => {
            println!("No se encontró ningún instrumento con ese nombre exacto.");
        }
    }
}
