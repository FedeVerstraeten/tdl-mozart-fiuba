//las referencias a secas son inmutables, como los enlaces a variables
//Esto quiere decir que en el fooLindo de antes
//no puedo cambiar los vectores


fn main(){
	let mut v1 = vec![1,2,3];
	
	//fooMut(&mut v1);//le paso una ref mutable, entonces puedo
	//cambiar v1

	
	let mut v2 = &mut v1;
	fooMutMut(&mut v2);
	
	//println!("La respuesta del FooMut es {}", v1[3]);	
	
	
	println!("La respuesta del FooMutMut es {}", v2[3]);	
}

//fn fooLindo (v: Vec<i32){
fn fooMut (v: &mut Vec<i32>){
	v.push(4);
}

fn fooMutMut (v: &mut Vec<i32>){
	v.push(5);
}

//Reglas de prestamo
//1-cualquier prestamo debe vivir en un ambito no mayor al del dueño
//2-se puede tener uno u otro de los dos tipos de prestamos,
//pero no a la vez:
//	-una o mas referencias (&T) a un recurso
//	-exactamente una referencia mutable(&mut T)

//Con las referencias comunes, se puede tener cuantas uno quiera
//ya que ninguna de ellas modifica la variable
//SI uno esta escribiendo, y necesita dos o mas punteros a la misma
//memoria, puede tener solo UN &mut a la vez


//Esto previene:
//	Invalidación de iteradores
//	Use after free

