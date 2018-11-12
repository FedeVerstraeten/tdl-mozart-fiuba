//try!
//
//
//Cuando hay que escribir muchas funciones que retronan el tipo Return
//el manejo de errores se vuelve denso
//La macro !try esconde algo de codigo reperitivo,
//
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::string::String;

struct Info{
    nombre: String,
    edad: i32,
    carrera: String,

}

fn escribir_info(info: &Info) -> io::Result<()>{
    let mut archivo = File::create("Rust_Team.txt").unwrap();

    try!(writeln!(&mut archivo, "nombre: {}", info.nombre));

    try!(writeln!(&mut archivo, "edad {}", info.edad));

    try!(writeln!(&mut archivo, "carrera {}", info.carrera));

    return Ok(());

}

fn main(){
    let nombre = String::from("Eze");
    let edad = 27;
    let carrera= String::from("Ing Electronica");

    let integrante = Info {nombre, edad, carrera};
    escribir_info(&integrante);


}
