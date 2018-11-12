//Ejemplo de lo que puedo hacer con Box<T>
//
//
//

use std::ops::Deref;

impl<T> Deref for MiBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }


}


struct MiBox<T>(T);//defino mi propio smart pointer

impl<T> MiBox<T>{
    fn new(x: T) -> MiBox<T> {
        MiBox(x)
    }


}

fn main(){
    let b = Box::new(5);//simplemente para almacenar data
    println!("b = {}",b);



    let x =5;
    let y = Box::new(x);//usandolo como referencia
    
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = MiBox::new(x);
    assert_eq!(5,*z);

}
