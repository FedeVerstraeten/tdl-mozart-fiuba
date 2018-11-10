struct Auto {
	marca: String,
	modelo: String,
	anio: i32,
	puertas: i32,
}

struct Bicicleta {
	marca: String,
	modelo: String,
	anio: i32,
	rodado: i32,
}

struct Moto {
	marca: String,
	modelo: String,
	anio: i32,
	cilindrada: i32,
}

trait Vehiculo {
	fn frenar(&self){
	}

	fn tocar_bocina(&self){
		println!("BIP");
	}

	fn to_string(&self)->String;
}

trait ACombustible{
	fn cargar_combustible(){

	}
}

impl Vehiculo for Moto {
	fn to_string(&self)->String{
		format!("{} {} año:{} cilindrada:{}",self.marca,self.modelo,self.anio,self.cilindrada)
	}	
}

impl Vehiculo for Auto {
	fn to_string(&self)->String{
		format!("{} {} año:{} puertas:{}",self.marca,self.modelo,self.anio,self.puertas)
	}	
}

impl Vehiculo for Bicicleta {
	fn to_string(&self)->String{
		format!("{} {} año:{} rodado:{}",self.marca,self.modelo,self.anio,self.rodado)
	}
	fn tocar_bocina(&self){
		println!("PIP PIP");
	}	
}

impl ACombustible for Auto{
}

impl ACombustible for Moto{
}


fn main() {
	let auto = Auto{
				marca: String::from("Peugeot"),
				modelo: String::from("106"),
				anio: 1997, 
				puertas: 3
			};
	
	let bici = Bicicleta{
				marca: String::from("Vairo"),
				modelo: String::from("5.0"),
				anio: 2012,
				rodado: 26
			};

	let moto = Moto{
				marca: String::from("Kawasaki"),
				modelo: String::from("Ninja"),
				anio: 2010,
				cilindrada: 650};

	let a :Vec<&Vehiculo> = vec![&auto,&bici,&moto];
	for d in a.iter(){
		d.tocar_bocina();
		println!("{}",d.to_string() );
	}
	
}