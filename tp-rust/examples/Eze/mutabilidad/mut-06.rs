//Mutabilidad a nivel de campos

//La mutabilidad es una propiedad de un préstamo (&mut) o un
// enlace a variable (let mut). Esto quiere decir que, por ejemplo,
// no se puede tener un struct con algunos campos mutables
// y otros inmutables.

use std::cell::Cell;

fn main(){

	struct Punto {
		x: i32,
		//mut y: i32,//esto rompe
		y: i32,
	}

	struct PuntoMut{
		x: i32,
		y: Cell<i32>,
	}

	

	let mut a= Punto {x:5, y:6};
	a.x=10;

	let b= Punto {x:5, y:6};
	//b.x=10;//Esto tira error, no se puede asignar esto a un campo inmutable

	println!("{}",a.x);//La mutabilidad del struct esta en su enlace a variable 

	//Por otro lado, si se usa Cell<T>, se puede emular mutabilidad a niver de campos	
	let c = PuntoMut {x:5, y: Cell::new(6)};

	c.y.set(7);

	println!("y: {:?}", c.y);

}



