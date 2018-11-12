//Ejemplo con Rc<T>
//Para compartir dada
//Como por ejemplo, haciendo una lista


enum Lista {
        Cons(i32, Rc<Lista>),
            Nil,
}

use Lista::{Cons, Nil};
use std::rc::Rc;

fn main() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        //let c = Cons(4, Rc::clone(&a));

        println!("{}",Rc::strong_count(&a));
}
