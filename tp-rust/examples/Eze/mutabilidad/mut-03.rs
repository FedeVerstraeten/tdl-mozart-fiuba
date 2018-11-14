fn main(){
	let mut x = vec!["Hola", "Mundo"];

	x = vec!["Chau", "Mundo"]; 
	
	//let mut z = 1;

	let mut z = vec!["Team", "Rust"];

	println!("{}", x[0]);
	{
		let mut y = &mut x; // y es un enlace a var mutable a una referencia mutable, es decir, puedo asociar y a otra cosa puedo mutar lo asociado a y.

		*y = vec!["Rust", "Rules"];

		//y = &mut z;

		//*y= 0;

		y = &mut z;

		*y = vec!["Equipo","Rust"];
	}
	println!("{}", x[0]);

	//println!("{}", z);
	
	println!("{}", z[0]);
	


}
