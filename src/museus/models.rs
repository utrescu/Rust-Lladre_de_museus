use chrono::NaiveDateTime;
use museus::schema::*;

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct Denuncia {
    pub id_denuncia: String,
    pub museu_id: i32,
    pub hora: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Eq, Debug)]
#[primary_key(id_visita)]
#[belongs_to(Visitant)]
#[table_name = "Visites"]
pub struct Visita {
    pub id_visita: String,
    pub museu_id: i32,
    pub visitant_id: String,
    pub hora: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Eq, Debug)]
// #[has_many(Visita)]
#[primary_key(visitant_id)]
#[table_name = "Visitants"]
pub struct Visitant {
    pub visitant_id: String,
    pub nom: String,
    pub sexe_id: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[primary_key(museu_id)]
#[table_name = "Museus"]
pub struct Museu {
    pub museu_id: i32,
    pub museu_nom: String,
    pub poblacio: String,
    pub capacitat: i32,
}
