|Titel|OLAYERS – OFML-kompatible Layer1|
|---|---|
|**Autor/Editor**|Ekkehard Beier/Thomas Gerth, EasternGraphics<br>im Auftrag des Arbeitskreises_Industrielle Aspekte der OFML-Normung_<br>_(IAON)_.|
|**Referenz**|Titel:**OLAYERS Spezifikation**<br>Version:**1.3, 1. überarbeitete Fassung**<br>Datum:**2015-08-18**|
|**Historie**|Am Ende des Dokuments|

# **Inhaltsverzeichnis**

1. Einführung............................................................................................................................................................2
2. Generische Layer-Bezeichnung...........................................................................................................................2
3. Allgemeine Layer.................................................................................................................................................3
4. 3D-Layer..............................................................................................................................................................4
5. 2D-Layer..............................................................................................................................................................4
6. Snapping Layer.....................................................................................................................................................5
7. Allgemeine Empfehlungen für die AutoCAD-Datenanlage................................................................................6
8. OFML-Daten- und Anwendungsaspekte..............................................................................................................6

8.1. OFML-Daten-Aspekte.................................................................................................................................6
8.2. Anwendungsaspekte....................................................................................................................................6
Anhang......................................................................................................................................................................7

Historie................................................................................................................................................................7


_**Referenzen**_


[DSR] – Beschreibung _'Data Structure and Registration',_ Version 3.3, EasternGraphics, 2014.


[OFML] – _OFML – Standardisiertes Datenbeschreibungsformat der Büromöbelindustrie,_ Version 2.0, 2. überarbeitete Fassung, BSO e.V., 2002.


[TAGS] – _OLAYERS TAGS – Tag Names for OFML-compatible Layers_, Version 1.2, BSO e.V., 2013.


1 Copyright © 2006 – 2015 Arbeitskreis _Industrielle Aspekte der OFML-Normung_ . All rights reserved.


Seite 1 von 7


# **_1. Einführung_**

Die gegenüber FOS erweiterten Möglichkeiten von OFML – insbesondere im Bereich der Hersteller- und
Serienkennungen – erfordern zwangsläufig eine Anpassung des FOS-angelehnten Layer-Konzepts. Nachfolgend
wird eine Layer-Struktur beschrieben, welche OFML-kompatibel ist.


Dies ist ein 'lebendes' Dokument, d.h., die Menge der konkreten Layer-Modi und vordefinierten Bezeichner wird
sukzessive wachsen. Um die Anzahl der Releases dieses Dokuments zu beschränken, werden Vorschläge für
einheitliche Tags in einem separaten, mit geltenden Dokument aufgelistet, siehe [TAGS].

# **_2. Generische Layer-Bezeichnung_**


Die generische Layer-Bezeichnung ist wie folgt festgelegt:


_72_<MAN>_<SERIES>_<MOD>[_<TAG>]_


Dabei gelten die folgenden Festlegungen:


    - _72_     - ist eine von AutoDesk vorgegebene Kennzeichnung für Möbel. Dieser Kennung folgt ein
Unterstrich zur Unterscheidung von den FOS-Layer-Bezeichnern.

    - _<MAN>_     - ist die OFML-Kennung des Herstellers (DSR-Schlüssel _manufacturer_, s. [DSR]). Nach
<MAN> folgt ein Unterstrich zur Separierung vom Serienbezeichner (selbst wenn <MAN> bereits
einen Unterstrich am Ende beinhaltet).

    - _<SERIES>_     - ist die OFML-Kennung der Serie (DSR-Schlüssel _program_ ). Nach <SERIES> folgt
ein Unterstrich zur Separierung vom Modus-Bezeichner (selbst wenn <SERIES> bereits einen
Unterstrich am Ende beinhaltet).

    - _<MOD>_     - kennzeichnet einen bestimmten Modus. (Die möglichen Modi werden nachfolgend
festgelegt.) Sofern auf diesen Bezeichner der optionale Bezeichner <TAG> folgt, wird ein
Unterstrich nach <MOD> eingefügt.
In den nachfolgenden Beschreibungen sind optionale Modi (Layer) durch [*] gekennzeichnet.

    - _<TAG>_     - Dieser Kennzeichner ist datenspezifisch und es hängt vom jeweiligen <MOD> ab, ob
dieser vorhanden ist oder nicht.


Beispiel:

_72_EGR_OFFICE2_D3_ANY_    - 'Hersteller' EasternGraphics (EGR), Serie 'Büroelemente' (OFFICE2),
3D-Geometrie


Hinweis: Layer-Namen sind per Definition unabhängig von Groß- oder Kleinschreibung. In diesem Dokument
wird Großschreibung verwendet; in der Praxis sollte ebenfalls Großschreibung verwendet werden. Die
Verwendung von Layer-Namen, die sich nur in der Groß- und Kleinschreibung unterscheiden, ist unzulässig.


Seite 2 von 7


# **_3. Allgemeine Layer_**

Diese Layer sind unabhängig von der speziellen 2D- oder 3D-Ansicht. Einige der Layer werden vermutlich nur
in einer bestimmten Ansicht (primär 2D-Ansicht) verwendet. Dennoch sind sie per Definition ansichtenunabhängig und aus diesem Grund diesem Bereich zugeordnet. Insofern sich (aktuell) eine eindeutige
Zuordnung zu einer Ansichtsart ergibt, dann ist diese in eckigen Klammern nach dem Layer-Modus benannt.


Die folgenden Modi sind definiert:


  - _*_DIMENSIONS_MM [2D]_


Auf diesem Layer können die geometrischen Bemaßungstexte des Objekts hinterlegt werden.


Die generelle Empfehlung zum Aufbau dieses Textes lautet: _Breite x Tiefe x Höhe_, wobei die Maße in
Millimeter angegeben werden und als Tausender-Separator der Punkt (.) verwendet wird. Allerdings sind
beliebige Abweichungen möglich, wie z.B. andere Maße ( _Radius_ ) bzw. eine Teilmenge der genannten Maße
( _Breite x Tiefe_ ).


Die Layer-Farbe ist grün (ACAD-Farbindex: 3).


  - _*_TEXT_<LANG> [2D][*]_


Auf diesem Layer kann ein zusätzlicher (über die geometrischen Bemaßungstexte hinausgehender) Text zu
dem Objekt abgelegt werden, wie z.B. die Anzahl der Ordnerhöhen bei Schränken.


Dabei gibt <LANG> die Sprache gemäß ISO 639-1 an, z.B. _DE_ für Deutsch, _EN_ für Englisch, usw.


Die Layer-Farbe ist grün (ACAD-Farbindex: 3).


  - _*_ARTICLE_INFO [2D][*]_


Auf diesem Layer können Artikelinformationen hinterlegt werden. Dies ist üblicherweise die kfm.
Artikelnummer; es sind allerdings auch strukturelle Informationen wie Haupt- und Unterpositionen möglich.


Die Layer-Farbe ist weiss/schwarz (ACAD-Farbindex: 7).


  - _*_ARTICLE_INFO_DPOS [2D][*]_


Auf diesem Layer kann die sog. Zeichnungsposition abgelegt werden. Die Zeichnungsposition ist eine
anwendungsspezifisch vergebene Nummer, die einen Bezug zwischen dem CAD-Artikel und
Bestellliste/Warenkorb repräsentiert.


Die Layer-Farbe ist weiss/schwarz (ACAD-Farbindex: 7).


  - _*_SPECIAL [2D][*]_


Auf diesem Layer können im Fall von Sonderartikeln erweiterte Informationen abgelegt werden.


Die Layer-Farbe ist magenta (ACAD-Farbindex: 6).


  - _*_MISC [2D][*]_


Auf diesem Layer werden optional weitere Informationen abgelegt, deren Sinn und Zweck nicht näher
beschrieben ist. Dies kann beispielsweise für Zubehörartikel verwendet werden, für die keine 3D-Grafik
vorhanden ist.


Die Layer-Farbe ist blau (ACAD-Farbindex: 5).


Seite 3 von 7


# **_4. 3D-Layer_**

Hierbei handelt es sich um Layer, welche nur in 3D-Ansichten relevant sind. Die folgenden Modi sind definiert:


  - _*_D3_<TAG> [*]_


Auf diesen Layern werden explizite 3D-Informationen abgelegt.


Die Angabe eines <TAG> ist zwingend erforderlich. Allerdings sind die Werte hierfür nicht genormt, da
sich diese prinzipiell durch die Herstellerdatenanlage ergeben und primär von der Art und Weise der
Materialzuordnung ergeben. Im mit geltenden Dokument [TAGS] werden im Abschnitt ‚Geometrie-Tags’
Vorschläge für die Bezeichnung dieser <TAG>’s gemacht.


Die Layer-Farbe wird individuell festgelegt.


  - _*_D3FRONT_<TAG> [*]_


Auf diesen Layern werden explizite 3D-Informationen für Frontelemente abgelegt. Objekte, die auf diesem
Layer liegen, können durch einen entsprechenden Filter des Planungssystems aus- und eingeblendet werden.


Die Angabe eines <TAG> ist zwingend erforderlich. Allerdings sind die Werte hierfür nicht genormt, da
sich diese prinzipiell durch die Herstellerdatenanlage ergeben und primär von der Art und Weise der
Materialzuordnung ergeben. Im mit geltenden Dokument [TAGS] werden im Abschnitt ‚Geometrie-Tags’
Vorschläge für die Bezeichnung dieser <TAG>’s gemacht.


Die Layer-Farbe wird individuell festgelegt.


  - _*_ACOUSTICS_ __<TAG> [*]_


Auf diesem Layer werden zusätzliche 3D-Geometrien als Akustik-Repräsentationen der Objekte abgelegt.
Diese werden im Rahmen von akustischen Auswertungen der Planung berücksichtigt.


Die Angabe eines <TAG> ist optional möglich und ergibt sich durch die Notwendigkeit unterschiedlicher
Materialzuordnungen. Die Werte hierfür sind nicht genormt.


Die Layer-Farbe wird individuell festgelegt.

# **_5. 2D-Layer_**


Hierbei handelt es sich um Layer, welche nur in 2D-Ansichten relevant sind. Die folgenden Modi sind definiert:


  - _*_D2_<TAG>_


Auf diesem Layer werden explizite 2D-Information abgelegt.


Die Angabe eines <TAG> ist zwingend erforderlich. Allerdings sind die Werte hierfür nicht genormt, da
sich diese prinzipiell durch die Herstellerdatenanlage ergeben und primär von der Art und Weise der
Materialzuordnung ergeben. In [TAGS] werden im Abschnitt ‚Geometrie-Tags’ Vorschläge für die
Bezeichnung dieser <TAG>’s gemacht.


Die Layer-Farbe ist gelb (ACAD-Farbindex: 2).


  - _*_D2DETAIL_<TAG> [*]_


Auf diesem Layer werden zusätzliche 2D-Information abgelegt, wie z.B. Fußgestelle und Kabelkanäle.


Für den <TAG> gelten die Regelungen wie oben.


Die Layer-Farbe ist gelb (ACAD-Farbindex: 2).


Seite 4 von 7


  - _*_D2SNAP [*]_


Auf diesem Layer können 2D-Fangpunkte abgelegt werden. Diese werden vom Snapping-Mechanismus des
OFML-Planungssystem bevorzugt verwendet gegenüber Fangpunkten, die sich aus der Artikel-Geometrie
ableiten.
Dies bezieht sich sowohl auf Objekte, die an andere Objekte snappen können [2], als auch auf Objekte, an die
gesnappt werden kann.


Die Layer-Farbe ist rot (ACAD-Farbindex: 1).

# **_6. Snapping Layer_**


Diese Layer können in einem OFML-Planungssystem verwendet werden, um mittels des SnappingMechanismus eine einfache Anfügelogik zwischen Artikeln auf Geschwisterebene zu realisieren [3] .


  - _*_ATTACH_<TAG> [*]_


Auf diesem Layer werden die Zielobjekte abgelegt, d.h., die Objekte, an die gesnappt werden kann.


Aktuell sind folgende Objektarten zulässig: Punkte, Linien und Flächen.


Die Layer-Farbe wird individuell festgelegt.


  - _*_ORIGIN_<TAG> [*]_


Auf diesem Layer werden die Snapobjekte abgelegt, d.h., die Objekte, die an Zielobjekte snappen können.


Aktuell sind folgende Objektarten zulässig: Punkte.


Die Layer-Farbe wird individuell festgelegt.


Das Snapping-Verhalten wird über die Buchstabenkombinationen in den <TAG>-Bezeichnern gesteuert: Ist ein
Buchstabe sowohl bei ATTACH als auch bei ORIGIN hinterlegt, so lassen sich die Geometrien aneinander
fügen [4] .


Beispiel:

    - Fläche auf dem Layer *_ATTACH_AB

    - Objekt 1 mit Punkt auf dem Layer *_ORIGIN_A

    - Objekt 2 mit Punkt auf dem Layer *_ORIGIN_C

    - Objekt 3 mit Punkt auf dem Layer *_ORIGIN_BCG


Das Fangverhalten zwischen der Fläche und den Punkten wird bei Objekt 1 und Objekt 3 aktiviert, da
deren <TAG>-Bezeichner einen deckungsgleichen Buchstaben wie im <TAG> des ATTACH-Layers
der Fläche besitzen.


Sind auch 2D-Fangpunkte auf dem Layer *_D2SNAP abgelegt, gilt:


   - Bei Zielobjekten werden Punkte auf *_ATTACH_<TAG> gegenüber Punkten auf *_D2SNAP
bevorzugt, sofern sich der Mauszeiger an einem passenden Punkt aus dem *_ORIGIN_<TAG> Layer
des Snap-Objektes befindet.


   - Bei Snap-Objekten werden Punkte aus den beiden Layern *_D2SNAP und *_ORIGIN_<TAG>
berücksichtigt. Mit welchem der Punkte das Objekt als erstes am Mauszeiger hängt, ist von der Position
der Punkte abhängig.


2 In diesem Fall hängt der zu platzierende Artikel mit einem der möglichen Fangpunkte am Mauszeiger und der Anwender kann die
Punkte dann ggf. (je nach Applikation) durchschalten.
3 Aktuell wird dies z.B. im pCon.planner ab Version 6.6 unterstützt. Metatypen oder spezifisch programmierte OFML-Klassen müssen
dazu nicht angelegt werden.
4 Die zueinander passenden Anfügepunkte können dann im Planungssystem farblich hervorgehoben werden, um die Planung für den
Anwender zu erleichtern.


Seite 5 von 7


# **_7. Allgemeine Empfehlungen für die AutoCAD-Datenanlage_**


  - Für die Einfügepunkte der Artikel gelten die folgenden Empfehlungen:


     - Normalerweise ist der linke, untere, hintere Punkt des Begrenzungsvolumens zu verwenden.

     - Bei frei platzierbaren, symmetrischen Artikeln (Stühle, Rundtische) sollte ein entsprechend
zentrierter Punkt angegeben werden.


  - Für Textprimitiven sollte der Textstil _txt.shx_ verwendet werden.

# **_8. OFML-Daten- und Anwendungsaspekte_**


_**8.1. OFML-Daten-Aspekte**_


Die externen Grafikdaten werden aus der ODB (siehe [OFML]) über voll qualifizierte Bezeichner referenziert:


_::<man>::<series>::<geometry>_


(In ODB muss komplett Kleinschreibung angewendet werden.)


Die externen Grafikdaten befinden sich im Verzeichnis <data>/<man>/<series>/1 [5] und verwenden die folgende
Namenskonvention <geometry>.{geo|egms|dwg|3ds}.


Die Layer-Zuweisung erfolgt auf ODB-Ebene über das Attribut _layer_ [6] . Die dabei verwendeten Layer-Bezeichner
entsprechen der OLAYERS-Konvention.


Die externen Grafikdaten selber enthalten keine Layer-Zuweisung [7] . Für 2D-Daten ist aus Gründen der
Abwärtskompatibilität eine Layer-Zuweisung in den Daten möglich. In diesem Fall gilt die Layer-Konvention
wie in diesem Dokument beschrieben.


Wenn innerhalb der DWG-Daten Blockreferenzen und Sub-Block-Strukturen verwendet werden, so müssen
diese die <MAN>-<SERIES>-Qualifizierung verwenden, um Konflikte mit den Daten anderer Hersteller zu
vermeiden!


_**8.2. Anwendungsaspekte**_


Wenn die OFML-Anwendung die Strukturen der OFML-Daten in das AutoCAD-System überträgt, müssen die
entsprechenden Blöcke voll qualifiziert angelegt werden und dazu die entsprechenden Informationen in der
Blockbezeichnung führen.


Für die eingefügten OFML-Objekte soll folgender Layer verwendet werden:


_72_<MAN2>_INSERT_


Hierbei ist _<MAN2>_ entweder _ANY_, falls keine eindeutige Herstellerzuordnung möglich ist (etwa im Fall von
Konfigurationen) oder entspricht <MAN> wie oben definiert.


Die Bezeichnung eventueller Blöcke zur Repräsentation von OFML-Artikeln ist anwendungsspezifisch.


5 Hier sei die Versionsnummer 1 angenommen.
6 Details siehe [OFML] und begleitende Dokumente zur ODB-Datenanlage.
7   Sie werden im AutoCAD also auf dem speziellen Layer 0 gezeichnet.


Seite 6 von 7


# **_Anhang_**

_**Historie**_


**Version 1.3, 1. überarbeitete Fassung - 2015-08-18**

  - Abschnitt zur Migration von FOS entfernt

  - Kleinere Korrekturen und verbesserte Beschreibungen


**Version 1.3 - 2014-03-21**

  - Neuer Layer *_D3FRONT_<TAG>


**Version 1.2 - 2013-07-17**

  - Neue Layer *_ATTACH_<TAG> und *_ORIGIN_<TAG>

  - Neuer Layer *_ACOUSTICS_<TAG>

  - Geringfügige Umstrukturierung des Dokuments


**Version 1.1 - 2007-10-04**

  - Neuer Layer *_ARTICLE_INFO_DPOS

  - Neue Farbe für *_ARTICLE_INFO

  - OFML-kompatible Layer-Bezeichner

  - OFML-Daten- und -Anwendungsaspekte

  - Auslagerung der Tag-Namen

  - Überarbeitung der generischen Layer-Bezeichnung


**Version 1.0 – 2006-05-03**

  - initiale Version


Seite 7 von 7


