fn main(){
	let mut x = vec!["Hola", "Mundo"];

	x = vec!["Chau", "Mundo"]; 
	
	println!("{}", x[0]);
	{
		let y = &mut x; // y es un enlace a var inmutable a una referencia mutable, es decir, no puedo asociar y a otra cosa, pero puedo mutar lo asociado a y.

		*y = vec!["Rust", "Rules"];
	}
	println!("{}", x[0]);

}
