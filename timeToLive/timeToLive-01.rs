//Tiempos de Vida

//prestar una referencia a otro recurso del que alguien mas es dueño
//es complicado

//Un ejemplo es el puntero colgante, aka use after free

//Para evitar esto, se asegura que nunca pase que se quiera usar
// un recurso luego de haber liberado el mismo

//Esto se hace incorporando en el prestamo el tiempo de vida, que
//describe el ámbito en el cual la referencia es válida

//Los tiempos de vida de las referencias se pueden hacer de manera
//implicita o explicita

fn main(){
	
	//Implícito
	fn foo1(x: &i32){

	}

	//Explicito
	fn foo2<'y>(x: &'y i32){
	
	}


}

//'y se lee como 'el tiempo de vida de y.
//Toda referencia posee un TTL asociado a ella,
//pero el compilador te permite omitirla en casos comunes

//Referente a sintaxis de funciones, los <> despues de un nombre de
//funcion, son parámetros fenericos, y los tiempos de vida son un
//parámetro generico.

//Se usa <> para declarar el tiempo de vida, entonces
//foo2 tiene un tiempo de vida: 'y.
//
