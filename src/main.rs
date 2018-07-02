extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
// ...
mod museus;

use museus::Lladre;

use std::collections::HashSet;
// use std::iter::FromIterator;

fn main() {
    let connection = museus::establish_connection();
    let les_denuncies = museus::obtenir_denuncies(&connection);

    let mut repetits: HashSet<Lladre> = HashSet::new();
    let mut comptador = 0;

    for denuncia in les_denuncies {
        let possibles_lladres =
            museus::obtenir_visites(&connection, &denuncia.museu_id, &denuncia.hora);

        if comptador == 0 {
            repetits = possibles_lladres;
        } else {
            repetits = interseccio(&repetits, &possibles_lladres);
        }
        comptador += 1;
    }

    println!("Sospitosos");
    println!("----------------");
    for repetit in repetits.iter() {
        println!("{}", repetit.nom);
    }
}

fn interseccio(repes: &HashSet<Lladre>, nous: &HashSet<Lladre>) -> HashSet<Lladre> {
    let resultat = repes.intersection(nous).collect::<HashSet<&Lladre>>();
    referencies_to_value(&resultat)
}

fn referencies_to_value(nous: &HashSet<&Lladre>) -> HashSet<Lladre> {
    let mut resultat: HashSet<Lladre> = HashSet::new();
    for nou in nous.iter() {
        resultat.insert(Lladre {
            id: nou.id.clone(),
            nom: nou.nom.clone(),
        });
    }
    resultat
}
