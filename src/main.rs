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
    caracter: char, //Carácter único
    visible: bool, // Si el carácter será visible para el usuario
}

//Enumeraciones
enum Progreso {
    Jugando, //El jugador sigue en el juego
    Victoria, //El jugador ha ganado
    Derrota //El jugador ha perdido
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


// La función para mostrar progreso revisará el vector de letras declarado en
// la parte superior del programa, si la letra es visible la asignará al
// vector, de no ser así en su lugar se asignará un _
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

// Es una funcion simple para leer la entrada del teclado del usuario, si el
// usuario no ingresa nada el carácter por defecto será un *
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

// Revisar progreso evalua los turnos y revisa que los booleanos que indican
// si todas las letras han sido descubiertas muestren el progreso del
// jugador
fn revisar_progreso(turnos_restantes: u8, letras: &Vec<Letra>) -> Progreso {
    let mut todos_revelados:bool = true;

    for letra in letras {
        if !letra.visible {
            todos_revelados = false;
        }
    }

    // Si todas las letras son reveladas el juego regresará un estado de
    // victoria
    if todos_revelados {
        return Progreso::Victoria;
    }

    // Si no se ha revelado todo el juego continuará
    if turnos_restantes > 0 {
        return Progreso::Jugando;
    }

    //Si no quedan turnos y todas las letras siguen sin ser descubiertas
    //el juego retornará un estado de derrota.
    return Progreso::Derrota;

}

fn main() {
    //Turnos restantes para el usuario
    let mut turnos_restantes = INTENTOS;

    //Asignamos la función usar palabra a una variable.
    let palabra_usada = usar_palabra();
    let mut letras = crear_letras(&palabra_usada);

    //Mensaje de bienvenida para el usuario
    println!("¡Bienvenido al ahorcado de Rust!");
    println!("Las palabras usadas son relacionadas al lenguaje de programacion\
             Rust");
    println!("(Ingrese un '*' para salir del programa)");

    // Loop para contar los intentos
    loop {
        println!("\nTienes {} turnos restantes", turnos_restantes);
        mostrar_progreso(&letras);

        println!("\nPor favor ingrese una letra:");
        let caracter_usuario = leer_entrada();

        if caracter_usuario == '*' {
            break;
        }

        // Actualizar el estado de cada letra si es que ésta ha sido revelada
        // mediante booleanos
        let mut minimo_revelado:bool = false;

        for letra in letras.iter_mut() {
            if letra.caracter == caracter_usuario {
                letra.visible = true;
                minimo_revelado = true;
            }
        }

        //Perder una vida si el usuario se equivoca
        if !minimo_revelado {
            turnos_restantes = turnos_restantes -1;
        }

        //Revisar el progreso del juego

        match revisar_progreso(turnos_restantes, &letras) {
            Progreso::Jugando => continue,
            Progreso::Victoria => {
                println!("\n¡Felicidades! Has ganado :D");
                break;
            }
            Progreso::Derrota => {
                println!("\nPerdiste :(");
            }
        }
    }

    //Mensaje de despedida
    println!("Adios :B");
}
