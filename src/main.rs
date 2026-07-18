use polodb_core::{
    CollectionT, Database,
    bson::{Document, doc, from_document, to_document},
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

fn main() {
    let db = Database::open_path("inventario.db").expect("No se pudo abrir la base de datos");
    println!("=== INVENTARIO DE INSTRUMENTOS ===");
    loop {
        println!();
        println!("1 - Agregar instrumento");
        println!("2 - Ver instrumentos");
        println!("3 - Salir");
        let opcion = leer("Opción: ");
        match opcion.as_str() {
            "1" => agregar_instrumento(&db),
            "2" => ver_instrumentos(&db),
            "3" => {
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
            let cantidad = leer("Cantidad de cuerdas: ");
            let cantidad = cantidad.parse::<u8>().unwrap_or(6);
            TipoInstrumento::Cuerdas {
                cantidad_cuerdas: cantidad,
            }
        }
        "2" => {
            let respuesta = leer("¿Es digital? (s/n): ");
            TipoInstrumento::Teclado {
                digital: respuesta.to_lowercase() == "s",
            }
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

