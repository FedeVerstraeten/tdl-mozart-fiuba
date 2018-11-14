//Use after free

//Las referencias no deben vivir por mas tiempo que el recurso al cual
//apuntan. Rust chequea los ámbitos de las referencias asegurandose
//que esto sea cierto

//Si Rust no lo verificara, se podría usar una referencia
//de manera invalida por accidente
fn main(){
	let y: &i32;

	//{//
		let x = 5;

		y = &x;
	//}//

	println!("{}", y);	
}

//y es válido solo para el ámbito donde x existe. Tan pronto como
//x se va, se hace inválido hacerle referencia.

//Es por ello que el error dice que el préstamo no vive lo suficiente
//ya que no es válido por la cantidad de tiempo correcta

//Lo mismo pasa cuando la ref es declarada antes de la var a la cual
//va a referenciar

//Esto se debe a que los recursos dentro del mismo ambito son
//liberados en orden opuesto al orden en que fueron declarados


//ESTO LLEVA A TIEMPOS DE VIDA
