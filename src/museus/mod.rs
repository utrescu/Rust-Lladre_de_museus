use mysql::chrono::*;

pub trait LladreDAO {
    fn obtenir_denuncies(&self) -> Vec<Denuncia>;
    fn obtenir_visites(&self, museu_id: u32, hora: DateTime<Utc>) -> Vec<Visitant>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Denuncia {
    pub museu_id: u32,
    pub museu_nom: String,
    pub hora: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Visitant {
    pub visitant_id: String,
    pub visitant_nom: String,
}

pub mod mysql;
