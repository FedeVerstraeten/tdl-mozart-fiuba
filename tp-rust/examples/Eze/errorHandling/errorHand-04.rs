use std::slice::Iter;

struct Profesor<'a> {
    nombre: &'a str,
    email: Option<&'a str>,
}

fn crear_email(unProfesor: &Profesor) -> Result<String, String> {
    match unProfesor.email {
        Some(email) => {
            let s = email.split('@').nth(0).unwrap();
            Ok(String::from(s) + "@fi.uba.ar")
        }, 
        None => Err(String::from("email no definido"))
                                                                            }
}

fn crear_los_emails(profes: Iter<&Profesor>) -> Result<(), String> {
    for prof in profes {
        //let new = crear_email(prof)?;
        let new = try!(crear_email(prof));
        println!("{} y su email: {}", prof.nombre, new);
    }

    Ok(())
}

fn main() {
    let mut profe1 = Profesor{ nombre: "Leandro", email: None };
    let profe2 = Profesor{ nombre: "Ariel", email: Some("ArielTDL@fi.uba.ar") };

    if let Err(e) = crear_los_emails(vec![&profe1, &profe2].iter())     {
        println!("Error: {}", e);
    }

    profe1.email = Some("LeandroTDL@fi.uba.ar");
    if let Err(e) = crear_los_emails(vec![&profe1, &profe2].iter()){
        println!("Error: {}", e);
    }
}
