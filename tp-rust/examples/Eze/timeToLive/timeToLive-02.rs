//Tiempos de Vida
// en struts 

struct Mistruct<'a> {
	x: &'a i32,
}


fn main(){

	let y = &10; //esto es lo mismo que let _y = 10 y let y = &_y
	let una_struct = Mistruct {x:y};

	println!("{}", una_struct.x); 	
}

//Porque se necesita un ttl en el struct?
// porque hay que asegurarse que cualquier referencia a Mistruct
// no pueda vivir mas que la referencia a un i32 que este contiene
