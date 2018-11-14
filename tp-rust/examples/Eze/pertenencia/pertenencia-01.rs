//La pertenencia es una de las características mas únicas e irresistibles de Rust. 
//A traves de la pertenencia es como Rust logra su objetivo mas importante, la seguridad

//Rust tiene foco en seguridad y velocidad.
//Rust logra esto a travez  de muchas "abstacciones de cero costo",
//lo que significa que  en Rust, las abstracciones cuestan tan poco
//como sea posible para hacerlo funcionar.

//Todo el analisis se hace en tiempo de compilación, por lo que no se
//tiene ningun costo en tiempo de ejecucion por ninguna de estas facilidades.

//El costo está en la curva de aprendizaje, se sufre lo que llaman
// "pelear con el comprobador de préstamo" (fighting with the borrow
//checker), que esto ocurre cuando el compilador de Rsut se rehusa a
//compilar un programa el cual el autor piensa que esta bien.

//Esto se debe a que la manera de pensarlo de como funciona la pertenencia no concuerda con las reglas actuales implementadas en Rust

fn main(){
	foo();
}

fn foo(){
	let v = vec![1,2,3];

	//let v2 = v; //Con esto y la siguiente linea tengo error
	//ya que quisiera usar un valor despues de haberlo movido a v2
	// y me va a sugerir usar v2

	println!("v[0] es : {}", v[0]);
}

//Los binding a variable poseen pertenencia sobre lo que están asociados
// Esto significa que cuand un binding a variable sale de ámbito, Rust
// libera los recursos asociados a este


//Cuando se transfiere la pertenencia a algo, en este caso a v2,
//se dice que se movio la cosa a la cual nos referiamos ([1,2,3])



//La linea let v = vec![1,2,3] asigna memoria para el objeto vector v 
//y para su data
//Al hacer let v2 = v, se mueve v a v2, creando una copia del puntero para v2
// Esto significa que hay dos punteros para el contenido del vector 
//Esto viola las garantias de seguridad de Rust, entonces, Rust
//prohibe el uso de v despues de haber hecho el movimiento
