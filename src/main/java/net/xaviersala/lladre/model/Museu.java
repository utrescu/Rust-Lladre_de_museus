package net.xaviersala.lladre.model;

public class Museu {
	int id;
	String nom;
	
	public Museu() {
	}
	
	public Museu(int id, String nom) {
		this.id = id;
		this.nom = nom;
	}
	
	public int getId() {
		return id;
	}
	public void setId(int id) {
		this.id = id;
	}
	public String getNom() {
		return nom;
	}
	public void setNom(String nom) {
		this.nom = nom;
	}
	@Override
	public String toString() {
		return id + " " + nom;
	}
	
	
}
