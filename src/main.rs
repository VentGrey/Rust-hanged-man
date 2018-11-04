// Importar bibliotecas de archivos
use std::fs::File;
use std::io::prelude::*;


//La función usar palabra, no toma ningún argumento y retorna una cadena.
fn usar_palabra() -> String {
    //abrir archivo
    let mut archivo = File::open("palabras.txt").expect("No se pudo abrir el archivo");
}

fn main() {
    let palabra_usada = usar_palabra();
}


