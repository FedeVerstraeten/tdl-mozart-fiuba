//ejemplo de panico


use Evento::Lanzamiento;

enum Evento{
    Lanzamiento,
}

fn probabilidad(_: &Evento) -> f64{
    //simplificando un toque
    0.95

}

fn probabilidad_con_onda(evento: Evento)-> &'static str{
    match probabilidad(&evento){
    1.00 => "de una",
    0.00 => "ni a palos",
    0.00 ... 0.25 => "muy poca chance",
    0.25 ... 0.50 => "poca chance",
    0.50 ... 0.75 => "hay chances",
    0.75 ... 1.00 => "dale que va",   
    _ => unreachable!()
    }

}

fn main(){
    println!("{}", probabilidad_con_onda(Lanzamiento));

}

//falla es un error del cual se puede recuperar de alguna manera
//
//con esto se quiere decir que sos casos donde se puede esperar un
//error
//
//
//
//
//Un pańico es un error irrecuperable
//un ejemplo de es el uso de la macro unreachable
//Un ejemplo es la macro unreachable!()
//Si no se usara la macro esa, tendriamos un error
//ya que Rust no podria saber que hemos cubierto todos los casos
//ya que no sabe que la prob va entre cero y uno
//
//nunca deberiamos alcanzar el caso _, y por esto hacemos uso de la
//macro unreachable!() para indicarlo
