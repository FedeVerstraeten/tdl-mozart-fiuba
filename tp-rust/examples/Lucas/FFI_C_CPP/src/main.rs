extern crate libc;
//extern crate c_types;

//FUNCIONES DE C Y C++
extern "C"{
    fn double_input_c(input: i32) -> i32;
    fn double_input_cpp(input: i32) -> i32;
    fn int_pointer_c(ptr: *mut  i32);
    fn rectangle_area_c(rect: *mut Rectangle) -> i32;
}

#[repr(C)]
struct Rectangle{
	width: i32,
	height: i32,
}

fn main() {
	let mut input = 3;    
    let mut output;
    
//Llamado a función de C
    output = unsafe{double_input_c(input)};
    println!("{} * 2 = {}", input,output);
    
//Llamado a función de C++
    output = unsafe{double_input_cpp(input)};
    println!("{} * 2 = {}", input,output);

    
//Pasaje de punteros a C    
    let rawptr = &mut input as *mut i32; // <----- uso un RAW POINTER
    unsafe{int_pointer_c(rawptr)};
    println!("Ahora input vale: {}", input);

//Pasaje de estructura a C
    let mut rect= Rectangle{width:3,
    						height:8};
    let area = unsafe{ rectangle_area_c(&mut rect)};
    println!("El área del rectángulo de {}x{} es: {}",
    		rect.width,
    		rect.height,
    		area)
}