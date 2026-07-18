# 🎸 Inventario de Instrumentos

Ejercicio en Rust para practicar bases de datos NoSQL, usando [PoloDB](https://github.com/PoloDB/PoloDB) como motor embebido y [Serde](https://serde.rs/) para serializar/deserializar datos.

## 📋 Funcionalidades

- **Agregar instrumentos** con validación de datos (rangos numéricos, respuestas sí/no)
- **Ver inventario completo**
- **Buscar** por nombre o marca (case-insensitive, coincidencia parcial)
- **Borrar instrumentos** con confirmación previa
- Soporta 4 tipos de instrumento, cada uno con sus propios atributos:
  - 🎸 **Cuerdas** (cantidad de cuerdas)
  - 🎹 **Teclado** (digital o acústico)
  - 🎺 **Viento** (material)
  - 🥁 **Percusión** (con o sin parche)

## 🛠️ Tecnologías

- **Rust**
- **PoloDB** — base de datos NoSQL embebida (documental)
- **Serde** — serialización/deserialización automática de structs y enums a BSON

## 🚀 Cómo correrlo

```bash
git clone https://github.com/carlosbruzzo/inventario-instrumentos-rust.git
cd inventario-instrumentos-rust
cargo run
```

La base de datos (`inventario.db/`) se crea automáticamente en el directorio del proyecto la primera vez que se ejecuta.

## 📖 Uso

Al ejecutar el programa vas a ver un menú:
1 - Agregar instrumento
2 - Ver instrumentos
3 - Buscar por nombre o marca
4 - Borrar instrumento
5 - Salir
Simplemente elegí el número de la opción y seguí las instrucciones en pantalla.

## 📚 Aprendizajes

Este proyecto fue un ejercicio práctico para entender:

- Cómo modelar datos variados con `enum` + variantes con campos (útil para NoSQL, a diferencia de tablas rígidas de SQL)
- Serialización automática de Rust ↔ BSON con Serde
- Manejo de inputs de usuario con validación y reintentos
- Consultas con filtros y expresiones regulares en una base documental
