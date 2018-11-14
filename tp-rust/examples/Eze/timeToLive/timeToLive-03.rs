//Multiples tiempos de vida 

//Con Multiples referencias, se puede hacer uso 
//de el mismo tiempo de vida múltiples veces

fn main(){

}



//fin x_o_y <'a, 'b>(x: &'a str, y: &'a str) -> &'a str {
fn x_o_y<'a, 'b>(x: &'a	str, y:	&'b str) -> &'a	str{
	x
}

// EN la linea comentada, x e y vivirian por el mismo ambito, y
//el valor de retorno tambien está vivo en dicho ambito

// para tiempos de vidad distinto, es la otra sentencia
