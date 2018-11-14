
fn main(){
	let v1 = vec![1,2,3];
	let v2 = vec![1,2,3];
	let (v1,v2,respuesta) =	fooFeo(v1,v2);

	println!("La respuesta del FooFeo es: {}", respuesta);
	
	let respuesta = fooLindo(&v1, &v2);
	println!("La respuesta del FooLindo es {}", respuesta);	
	
	println!("vean que aca puedo usar v1 sin drama {}",v1[0]);
	
}

fn fooFeo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>,Vec<i32>,i32){
	(v1, v2, 654321)
}

fn fooLindo (v1: &Vec<i32>, v2: &Vec <i32>) -> i32 {
	123456
}
// el -> indica que se devuelve la pertenencia

//En fooLindo, en vez de tomar Vec<i32> como argumentos, tomo
// una referencia: &Vec<i32>. y no paso directamente v1 y v2,
//sino &v1 y &v2, que son referencias. En vez de tomar pertenencia
//sobre el recurso, se toma prestada.
//Un enlace a variable que hace un prestamo de algo no libera el
//recurso cuando sale de ambito.
// Esto significa que despues de llamar a fooLindo(), puedo
// volver a hacer uso de los enlaces a variable originales.
