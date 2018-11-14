//Invalidacion de iteradores

//ocurre cuando se trata de mutar una coleccion mientras se está
//iterando sobre ella

//el comprobador de prestamos de Rust previene esto

fn main(){
	let mut v1 = vec![1,2,3];
	
	for i in &v1{
	
		println!(" {}", i);
		
		//v1.push(4);	

	}
}

//Esto imprime de uno a tres. A medida que se itera en v1
//solo se tienen referencias a sus elementos, v1 es prestado
//de manera INMUTABLE, entonces querer cambiarlo mientras se
//lo itera llevaría a un error de compilación

