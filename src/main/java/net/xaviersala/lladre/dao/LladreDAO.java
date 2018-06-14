package net.xaviersala.lladre.dao;

import java.sql.SQLException;
import java.sql.Timestamp;
import java.util.List;

import net.xaviersala.lladre.model.Visitant;
import net.xaviersala.lladre.model.Denuncia;
import net.xaviersala.lladre.model.Museu;

public interface LladreDAO {
	void Connecta(String host, String database, String user, String password) throws SQLException;
	List<Denuncia> obtenirDenuncies() throws SQLException;
	List<Visitant> obtenirVisites(Museu museu, Timestamp hora) throws SQLException;
}
