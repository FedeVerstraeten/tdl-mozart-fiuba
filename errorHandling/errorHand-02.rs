//ejemplo de falla
//
//
//La manera mas simple de indicar que una funcion puede fallar es
//usando el tipo Option<T>. Es util en casos simples,
//pero no da mucha info
//Para saber porque algo fallo, conviene usar Result<T, E>
//
#[derive(Debug)]
enum Version{Version1, Version2}

#[derive(Debug)]
enum ErrorParseo{LongInvalida, VersionTrucha}

fn parsear_version(cabecera: &[u8]) -> Result <Version, ErrorParseo>{
    if cabecera.len()<1{
        return Err(ErrorParseo::LongInvalida);
    }
    match cabecera[0]{
        1 => Ok (Version::Version1),
        2 => Ok (Version::Version2),
        _ => Err(ErrorParseo::LongInvalida)
    
    }
}

fn main(){
    //let version = parsear_version(&[1, 2, 3, 4]);//Ver1
    //let version = parsear_version(&[ 3]);//tira e
    //let version = parsear_version(&[2, 3, 4]);//Ver2
    let version = parsear_version(&[8, 3, 4]);//tira e
    match version{
        Ok (v) => {
            println!("Trabajando en la version: {:?}", v);
        }
        Err(e) =>{
            println!("error parseando: {:?}", e);
        }
    
    }


}
// Aca se hace uso de un enum, ErrorParseo, para enumerar los
// errores que pueden ocurrir
//
// el trait Debug es el que permite impimir el valor del enum
// usando la operacion de formato {:?}
