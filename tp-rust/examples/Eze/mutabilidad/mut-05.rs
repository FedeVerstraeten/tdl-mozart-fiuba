//este modulo, std::cell, tiene mutabilidad interior


//RefCell Proporciona referencias &mut de lo que contiene,
//mediante el metodo borrow_mut()

use std::cell::RefCell;

fn main(){

	let x = RefCell::new(42);
	let y = x.borrow_mut();
	
	//let z = x.borroew_mut(); //si lo junto con lo otro, pincha

	//println!("{}",x);
	println!("{}",y);

}


//Lo que RefCell hace es aplicar las reglas de préstamo de Rust
//en tiempo de ejecución, y hace panic! si se violan las reglas.

//Esto lleva a ------> Mutabilidad a nivel campos
