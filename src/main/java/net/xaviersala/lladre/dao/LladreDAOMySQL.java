package net.xaviersala.lladre.dao;

import java.sql.DriverManager;
import java.sql.SQLException;

public class LladreDAOMySQL extends LladreDAOSQL {

	@Override
	public void Connecta(String database, String user, String password) throws SQLException {
		conn = DriverManager.getConnection("jdbc:mysql://localhost/" + database, user, password);
	}

}
