package net.xaviersala.lladre.model;

import java.sql.Timestamp;

public class Denuncia {
	Museu museu;
	Timestamp data;
	
	public Denuncia() {
		
	}
	
	public Denuncia(Timestamp data, Museu museu) {
		this.data = data;
		this.museu = museu;
	}
	
	public Museu getMuseu() {
		return museu;
	}
	
	public void setMuseu(Museu museu) {
		this.museu = museu;
	}
	
	public Timestamp getData() {
		return data;
	}
	
	public void setData(Timestamp data) {
		this.data = data;
	}
	
	@Override
	public String toString() {
		return "Denuncia: " + museu + "(" + data + ")";
	}
	
}
