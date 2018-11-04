// Importar bibliotecas de archivos
use std::fs::File;
use std::io::prelude::*;


//La función usar palabra, no toma ningún argumento y retorna una cadena.
fn usar_palabra() -> String {
    //abrir archivo
    let mut archivo = File::open("palabras.txt").expect("No se pudo abrir el archivo");

    // Leer el archivo
    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)
        .expect("Fallo al leer el archivo");

    // Recortar las palabras del archivo
    let palabras:  Vec<&str> = contenido.trim().split(',').collect();
}

fn main() {
    let palabra_usada = usar_palabra();
}


