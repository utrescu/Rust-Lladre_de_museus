table! {
    Denuncies (id_denuncia) {
        id_denuncia -> Varchar,
        museu_id -> Integer,
        hora -> Timestamp,
    }
}

table! {
    Visitants (visitant_id) {
        visitant_id -> Varchar,
        nom -> Varchar,
        sexe_id -> Integer,
    }
}

table! {
    Museus (museu_id) {
        museu_id -> Integer,
        nom -> Nullable<Varchar>,
        poblacio -> Nullable<Varchar>,
        capacitat -> Nullable<Integer>,
    }
}

table! {
    Visites(id_visita) {
        id_visita -> Varchar,
        museu_id -> Integer,
        visitant_id -> Varchar,
        hora -> Timestamp,
    }
}

joinable!(Visites -> Visitants (visitant_id));
joinable!(Visites -> Museus (museu_id));

allow_tables_to_appear_in_same_query!(Denuncies, Museus, Visitants, Visites,);
