//crates externas
extern crate rand;
use rand::Rng;

// Importar bibliotecas de archivos
use std::fs::File;
use std::io::prelude::*;

// Entrada/Salida del usuario
use std::io;

//Limitar al usuario (Si no, no sería un ahorcado)
const INTENTOS: u8 = 5;

// Estructuras
struct Letra {
    caracter: char,
    visible: bool,
}

//Enumeraciones
enum Progreso {
    Jugando,
    Victoria,
    Derrota
}

//La función usar palabra, no toma ningún argumento y retorna una cadena.
fn usar_palabra() -> String {
    //abrir archivo
    let mut archivo = File::open("palabras.txt").expect("No se pudo abrir el archivo");

    // Leer el archivo
    let mut contenido = String::new();
    archivo
        .read_to_string(&mut contenido)
        .expect("Fallo al leer el archivo"); //Excepción en caso de no poder leer el archivo

    // Recortar las palabras del archivo
    let palabras: Vec<&str> = contenido.trim().split(',').collect();

    // Generar un indice aleatorio
    let indice_rand = rand::thread_rng().gen_range(0, palabras.len());

    /* La funcion retornara una cadena literal tomada de la cadea de palabras
     * que cargamos del archivo*/
    return String::from(palabras[indice_rand]);
}

fn crear_letras(palabra: &String) -> Vec<Letra> {
    //Crear un vector vacío
    let mut letras: Vec<Letra> = Vec::new();

    // Enlazar cada letra en una estructura
    for i in palabra.chars() {
        letras.push(Letra {
            caracter: i,
            visible: false,
        });
    }

    return letras;
}

fn mostrar_progreso(letras: &Vec<Letra>) {
    let mut mostrar_cadena = String::from("Progreso:");

    for letra in letras {
        mostrar_cadena.push(' ');

        if letra.visible {
            mostrar_cadena.push(letra.caracter);
        } else {
            mostrar_cadena.push('_');
        }

        println!("{}", mostrar_cadena);
    }
}

fn leer_entrada() -> char {
    let mut entrada = String::new();

    match io::stdin().read_line(&mut entrada) {
        Ok(_) => {
            match entrada.chars().next() {
                Some(i) => {return i;}
                None => {return '*';}
            }
        }
        Err(_) => {return '*';}
    }
}

fn main() {
    //Turnos restantes para el usuario
    let mut turnos_restantes = INTENTOS;

    //Asignamos la función usar palabra a una variable.
    let palabra_usada = usar_palabra();
    let letras = crear_letras(&palabra_usada);


    mostrar_progreso(&letras);
    println!("La palabra seleccionada fué {}", palabra_usada);


}
