use museus::Denuncia;
use museus::LladreDAO;
use museus::Visitant;
use mysql as my;

use mysql::chrono::*;
use time::Timespec;

pub struct MyDatabase {
    pub connection: my::Pool,
}

impl MyDatabase {
    pub fn connecta(
        host: String,
        database: String,
        user: String,
        password: String,
    ) -> Result<MyDatabase, String> {
        let url = format!("mysql://{}:{}@{}/{}", user, password, host, database);
        match my::Pool::new(url) {
            Ok(x) => Ok(MyDatabase { connection: x }),
            Err(_) => Err(String::from("Unable to connect to database")),
        }
    }
}

impl LladreDAO for MyDatabase {
    fn obtenir_denuncies(&self) -> Vec<Denuncia> {
        let query = "SELECT m.museu_id, m.nom, d.hora FROM Denuncies d INNER JOIN Museus m ON d.museu_id = m.museu_id";
        let result: Vec<Denuncia> = self.connection
            .prep_exec(query, ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, nom, hora) = my::from_row(row);
                        let ts = my::from_value::<Timespec>(hora);
                        Denuncia {
                            museu_id: id,
                            museu_nom: nom,
                            hora: DateTime::from_utc(
                                NaiveDateTime::from_timestamp(ts.sec, ts.nsec as u32),
                                Utc,
                            ),
                        }
                    })
                    .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
            })
            .unwrap(); // Unwrap `Vec<Payment>`
        result
    }

    fn obtenir_visites(&self, museu_id: u32, hora: DateTime<Utc>) -> Vec<Visitant> {
        let query = "SELECT DISTINCT c.visitant_id, c.nom FROM Visites v INNER JOIN Visitants c ON v.visitant_id = c.visitant_id WHERE museu_id = :id AND hora = :hora";

        let nova_hora = hora.with_timezone(&Local).to_rfc3339();

        let clients: Vec<Visitant> = self.connection
            .prep_exec(
                query,
                params! {
                        "id" => museu_id,
                        "hora" => nova_hora,
                },
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, nom) = my::from_row(row);
                        Visitant {
                            visitant_id: id,
                            visitant_nom: nom,
                        }
                    })
                    .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
            })
            .unwrap(); // Unwrap `Vec<Payment>`
        clients
    }
}
