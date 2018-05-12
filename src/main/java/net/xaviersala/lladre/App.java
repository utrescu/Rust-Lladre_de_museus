package net.xaviersala.lladre;

import java.sql.SQLException;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

import net.xaviersala.lladre.dao.LladreDAO;
import net.xaviersala.lladre.dao.LladreDAOMySQL;
import net.xaviersala.lladre.model.Visitant;
import net.xaviersala.lladre.model.Denuncia;

/**
 * Cerca en la base de dades quin és el famós lladre del món.
 *
 */
public class App 
{
    public static void main( String[] args )
    {
        LladreDAO dades = new LladreDAOMySQL();
        try {
			
        	dades.Connecta("lladres", "root", "ies2010");
        	List<Denuncia> denuncies = dades.obtenirDenuncies();
        	
        	Set<Visitant> sospitosos = new HashSet<>();
        	boolean primeraVegada = true;
        	
        	for (Denuncia denuncia: denuncies) {
        	
        		// Recuperar tots els visitants del museu en la hora de la denuncia
        		Set<Visitant> nousClients = new HashSet<>();
        		nousClients.addAll(dades.obtenirVisites(denuncia.getMuseu(), denuncia.getData()));   
        		
        		if (primeraVegada == true) {
        			sospitosos.addAll(nousClients);
        			primeraVegada = false;
        		} else {
        			// Intersecció entre els conjunts
        			sospitosos.retainAll(nousClients);
        		}
        	}
			
        	System.out.println("Els culpables són:\n");
        	sospitosos.forEach(it -> System.out.println("   " + it.getNom()));			
			
		} catch (SQLException e) {
			System.out.println("Fallada al connectar " + e.getMessage());
		}
    }
}
