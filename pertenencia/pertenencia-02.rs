//El trait Copy
//Un trait es basicamente una anotación hecha a un tipo en particular,
//el cual agrega comportamiento extra

fn main(){
	foo();
}

fn foo(){
	let a = 2;
	
	let x = a;	
		
	let b = doble(a);	

	println!("{}", a);

	println!("{}", x);	

	println!("{}", b);

	let c = true;

	let d = mentir(c);

	println!("{}",c);
	println!("{}",d);


}

fn doble(x: i32) -> i32 {// el --> indica que se devuelve la pertenencia
	x*2
}

fn mentir(x: bool) -> bool {
	!x
}

//Todos los tipos primitidos implementan el trait Copy y su pertenencia
//no es movida como se podria esperar. Que implementen este trait 
//significa que que cuando asigno a b, una copia de la data es hecha
//Entonces, puedo usar a despues, ya que tipos primitivos como i32 y bool
// no poseen punteros a data en ningun otro lado, copiarlos es literal
//copiarlos por completo


//Si se tuviera que hacer esto con cada funcion se volvería muy tedioso
// para esto, Rust ofrece el Prestamo, que soluciona esto
