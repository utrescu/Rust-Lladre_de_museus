# Aprenent Rust

Aquest és el meu primer programa en Rust. Fins ara no n'havia fet mai cap i per tant segurament hi faig algunes coses que poden ser fetes d'una forma més òptima.

## Compilar

1.  Primer s'ha d'instal·lar Rust. La forma més senzilla sol ser seguir les instruccions de [la seva web](https://www.rust-lang.org/es-ES/) ;-). també estan en castellà però en resum consisteixen en executar:

    curl https://sh.rustup.rs -sSf | sh

2.  clonar el repositori

3.  compilar i executar el programa (per defecte agafa in.txt com a paràmetre)

    cargo run --release

Un cop compilat l'executable es trobarà disponible a **target/release/** i es pot executar sense fer servir cargo.

## Configuració

Aquest programa accedeix a una base de dades MySQL que prèviament s'ha d'importar.

He generat diverses bases de dades per fer diferents proves:

> Podeu descarregar una còpia de la base de dades d'[Aquí](https://drive.google.com/file/d/1lF2grKTyYZJqAh9F6DuK61gscZhJ9ufg/view?usp=sharing)
>
> I hi ha una altra Base de dades diferent [Aquí](https://drive.google.com/file/d/1E27wssmMpxNk4vdGAhTNZrSObXxxBgg_/view?usp=sharing)
>
> I una de gegant [Aquí]()

En aquest cas els paràmetres de la base de dades estan en el fitxer .env que és carregat automàticament al arrancar

    DATABASE_URL=mysql://root:ies2010@172.99.0.2/lladres


# Exercici: L’amenaça als museus

Es va rebre un anònim a l’Ajuntament on hi havia l’amenaça de robar en diversos museus de la ciutat. Els museus de la ciutat van rebre una amenaça de robatori però no s’ho van prendre seriosament perquè:

- Tots tenen una alarma “Direct Secure” connectada amb la policia.
- Tots els visitants que entren en algun dels museus són obligats a identificar-se.

![Volen robar els museus](imatges/museus0.png)

Però tot i les precaucions i les mesures de seguretat, l’amenaça s’ha complert! Hi ha hagut robatoris en uns quants dels museus de la ciutat!

![Han robat igualment](imatges/museus1.png)

El gran detectiu **Sherlock “Informatic” Holmes** ha vingut a intentar ajudar-nos i ràpidament ha deduït que tots els robatoris han estat fets per la mateixa gent.

![Sherlock "informàtic" Holmes](imatges/museus2.png)

Però la quantitat de dades és molt gran i per tant ha decidit entrar totes les dades que arriben de cada museu en una base de dades relacional que té aquesta estructura:

![Estructura de dades](imatges/museus3.png)

S’ha creat una taula _“Visites”_ conté cada un dels clients que hi havia en el museu en una hora determinada i s’ha entrat en la taula “Denúncies” en quin moment es van produir els robatoris en cada museu.

## Tasca

1.  Sherlock “Informàtic” Holmes no sap res d’informàtica (ups!) i per això us demana a vosaltres que li feu un programa que determini quines persones eren en els museus en el moment en que es van produir els robatoris per determinar qui van ser els lladres.

Recordeu que ha deduït que els lladres sempre anaven junts a tots els robatoris.
