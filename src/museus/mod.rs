pub mod models;
pub mod schema;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenv::dotenv;
use std::collections::HashSet;
use std::env;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Lladre {
    pub id: String,
    pub nom: String,
}

use self::models::Denuncia;
use self::models::Visita;
use self::models::Visitant;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn obtenir_denuncies(connection: &MysqlConnection) -> Vec<Denuncia> {
    use museus::schema::Denuncies::dsl::*;

    let les_denuncies = Denuncies
        .load::<Denuncia>(connection)
        .expect("Error loading denuncies");
    les_denuncies
}

pub fn obtenir_visites(
    connection: &MysqlConnection,
    num_museu: &i32,
    hora_robatori: &NaiveDateTime,
) -> HashSet<Lladre> {
    use museus::schema::Visitants::dsl::*;
    use museus::schema::Visites::dsl::hora;
    use museus::schema::Visites::dsl::museu_id;
    use museus::schema::Visites::dsl::*;

    let visites: Vec<(Visita, Visitant)> = Visites
        .inner_join(Visitants)
        .filter(museu_id.eq(num_museu).and(hora.eq(hora_robatori)))
        .load(connection)
        .expect("Error loading Visites");

    let mut possibles_lladres: HashSet<Lladre> = HashSet::new();

    for (visita, persona) in visites {
        possibles_lladres.insert(Lladre {
            id: visita.visitant_id,
            nom: persona.nom,
        });
    }
    possibles_lladres
}
