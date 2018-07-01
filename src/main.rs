#[macro_use]
extern crate mysql;
extern crate time;
// ...
use museus::mysql::MyDatabase;
use museus::LladreDAO;
use museus::Visitant;
use std::collections::HashSet;
use std::iter::FromIterator;

mod museus;

fn main() {
    let db = MyDatabase::connecta(
        String::from("172.99.0.2"),
        "lladres".to_string(),
        "root".to_string(),
        "ies2010".to_string(),
    ).unwrap();

    let denuncies = db.obtenir_denuncies();

    let mut repetits: HashSet<Visitant> = HashSet::new();
    let mut comptador = 0;

    for denuncia in denuncies {
        let noves = db.obtenir_visites(denuncia.museu_id, denuncia.hora);
        // println!("------------------");
        // println!("{} - {}", denuncia.museu_id, denuncia.museu_nom);
        // println!("------------------");
        // println!("{:?}", noves);

        let nous_visitants = genera_hashset(&noves);

        if comptador == 0 {
            repetits = nous_visitants;
        } else {
            repetits = interseccio(&repetits, &nous_visitants);
        }
        comptador += 1;
    }

    println!("Sospitosos");
    println!("----------------");
    for repetit in repetits.iter() {
        println!("{}", repetit.visitant_nom);
    }
}

fn interseccio(repes: &HashSet<Visitant>, nous: &HashSet<Visitant>) -> HashSet<Visitant> {
    let resultat = repes.intersection(nous).collect::<HashSet<&Visitant>>();
    referencies_to_value(&resultat)
}

fn referencies_to_value(nous: &HashSet<&Visitant>) -> HashSet<Visitant> {
    let mut resultat: HashSet<Visitant> = HashSet::new();
    for nou in nous.iter() {
        resultat.insert(Visitant {
            visitant_id: nou.visitant_id.clone(),
            visitant_nom: nou.visitant_nom.clone(),
        });
    }
    resultat
}

fn genera_hashset(data: &[Visitant]) -> HashSet<Visitant> {
    HashSet::from_iter(data.iter().cloned())
}
