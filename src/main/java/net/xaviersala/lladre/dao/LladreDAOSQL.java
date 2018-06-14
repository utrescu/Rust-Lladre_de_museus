package net.xaviersala.lladre.dao;

import java.sql.Connection;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Timestamp;
import java.util.ArrayList;
import java.util.List;

import net.xaviersala.lladre.model.Visitant;
import net.xaviersala.lladre.model.Denuncia;
import net.xaviersala.lladre.model.Museu;

public abstract class LladreDAOSQL implements LladreDAO {

	Connection conn;

	public abstract void Connecta(String host, String jdbc, String user, String password) throws SQLException;

	private static final String OBTENIRDENUNCIES = "SELECT m.museu_id, m.nom, d.hora " + "FROM Denuncies d "
			+ "INNER JOIN Museus m ON d.museu_id = m.museu_id;";

	/**
	 * Recuperar totes les denúncies actives
	 */
	public List<Denuncia> obtenirDenuncies() throws SQLException {
		PreparedStatement ps = conn.prepareStatement(OBTENIRDENUNCIES);
		List<Denuncia> denuncies = new ArrayList<>();

		ResultSet rs = ps.executeQuery();

		while (rs.next()) {
			denuncies.add(new Denuncia(rs.getTimestamp(3), new Museu(rs.getInt(1), rs.getString(2))));
		}
		return denuncies;
	}

	private static final String OBTENIRVISITES = "SELECT DISTINCT c.visitant_id, c.nom " + "FROM Visites v "
			+ "INNER JOIN Visitants c ON v.visitant_id = c.visitant_id " + "WHERE museu_id = ? AND hora = ?";

	/**
	 * Recuperar totes les visites al museu en una hora determinada.
	 */
	public List<Visitant> obtenirVisites(Museu museu, Timestamp hora) throws SQLException {
		List<Visitant> clients = new ArrayList<Visitant>();

		PreparedStatement ps = conn.prepareStatement(OBTENIRVISITES);
		ps.setInt(1, museu.getId());
		ps.setTimestamp(2, hora);

		ResultSet rs = ps.executeQuery();

		while (rs.next()) {
			clients.add(new Visitant(rs.getString(1), rs.getString(2)));
		}

		return clients;
	}

}
