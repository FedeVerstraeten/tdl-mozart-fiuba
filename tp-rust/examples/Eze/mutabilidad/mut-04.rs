//Cuando se dice que algo es Inmutable en Rust
//no significa que no pueda ser cambiado,
//significa que tiene mutabilidad exterior
use std::sync::Arc;

fn main(){

	let x = Arc::new(5);
	let y = x.clone();
	
	println!("{}",x);
	println!("{}",y);


}

/*
Cuando	llamamos	a	 	clone()	 ,	el	 	Arc<T>	 	necesita	actualizar	el	contador	de	referencias.	A
pesar	de	que	no	hemos	usado	ningún	 	mut	 	aquí,	 	x	 	es	un	enlace	inmutable,	tampoco
tomamos	 	&mut	5	 	o	alguna	mas.	Entonces,	que	esta	pasando?
Para	entender	esto,	debemos	volver	al	núcleo	de	la	filosofía	que	guía	a	Rust,	seguridad	en
el	manejo	de	memoria,	y	el	mecanismo	a	través	del	cual	Rust	la	garantiza,	el	sistema	de
pertenencia,	y	mas	específicamente,	el	préstamo:
Puedes	tener	uno	u	otro	de	estos	dos	tipos	de	prestamo,	pero	no	los	dos	al	mismo
tiempo:
una	o	mas	referencias	( 	&T	 )	a	un	recurso,
exactamente	una	referencia	mutable	( 	&mut	T	 ).
Entonces,	esa	es	la	definición	real	de	‘inmutabilidad’:	es	seguro	tener	dos	apuntadores?	En
el	caso	de	 	Arc<T>	 ’s,	si:	la	mutación	esta	completamente	contenida	dentro	de	la	estructura
en	si	misma.	No	esta	disponible	al	usuario.	Por	esta	razón,	retorna	 	clone()	 	 	&T	 s.	Si
proporcionase	 	&mut	T	 s,	seria	un	problema.*/
