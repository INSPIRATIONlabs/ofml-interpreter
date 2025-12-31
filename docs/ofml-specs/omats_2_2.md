# Spezifikation OMATS OFML–kompatible Materialien [*]

Version 2.2


Thomas Gerth, EasternGraphics GmbH (Editor)


22. April 2025


- Copyright © 2003–2025 Industrieverband B¨uro und Arbeitswelt e. V. (IBA)


## **Inhaltsverzeichnis**

**1** **Einleitung** **2**


**2** **Die Materialmodelle** **2**


2.1 ¨Ubersicht . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 2


2.2 Verwendete Datentypen . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 3


2.3 Materialtypen . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5


2.4 Die Materialparameter . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5


**3** **Textur–Mapping–Verfahren** **10**


3.1 Ebenen-Mapping . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 10


3.2 Quader–Mapping . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 11


3.3 Texturkoordinaten . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 11


**4** OFML **–Datenformat f¨ur Materialien** **12**


**A Einf¨uhrung in das physikalische Rendering (PBR)** **15**


**B Konvertierung von alten Materialien auf das neue Modell** **16**


**C Historie** **17**

## **Literatur**


[jfif] JPEG File Interchange Format, Version 1.02
World Wide Web Consortium (W3C)

( `www.w3.org/Graphics/JPEG/jfif3.pdf` )


[odb] ODB – OFML-Datenbank (OFML Part I), Version 2.4.
Industrieverband B¨uro und Arbeitswelt e.V. (IBA)


[ofml] OFML – Standardisiertes Datenbeschreibungsformat der B¨urom¨obelindustrie,
Version 2.0, 3. ¨uberarbeitete Auflage
Industrieverband B¨uro und Arbeitswelt e.V. (IBA)


[png] Portable Network Graphics (PNG) Specification, Version 1.2

PNG Development Group

( `www.libpng.org/pub/png/spec/1.2/png-1.2-pdg.html` )


1


## **1 Einleitung**

Diese Spezifikation definiert zwei Modelle zur Beschreibung von Materialien, welche in OFML–basierten
Applikationen zur Darstellung von Objektoberfl¨achen (Materialien) zur Anwendung kommen, sowohl im
Echtzeit- als auch im photorealistischen Bereich.


Desweiteren werden in dieser Spezifikation die unterst¨utzten Textur–Mapping–Verfahren sowie die Abbildung der abstrakten Modellparameter in OFML–Materialdefinitionsdateien beschrieben.


Die beiden Materialmodelle werden als OMATS1 und OMATS2 referenziert. OMATS1 ist dabei das ¨altere Modell. In neueren OFML–Applikationen wird es durch das neue Modell OMATS2 abgel¨ost, welches
auf dem Konzept des _physikalischen Renderings_ (PBR) basiert. Dieses erm¨oglicht bei einer kompakteren
Materialbeschreibung eine realistischere und ansprechendere Darstellung im Echtzeitmodus (mehr Informationen dazu s. Anh. A). Zudem bieten Materialeditoren, die auf diesem Modell basieren, eine bessere
Benutzerfreundlichkeit, da weniger Parameter eingestellt werden m¨ussen und weniger Abh¨angigkeiten
zwischen den Parametern existieren.


Zur Gew¨ahrleistung einer abw¨artskompatiblen Verarbeitung von Materialien gelten folgende Bestimmungen f¨ur Applikationen, die das neue Modell verwenden:


 Materialien, die auf der Basis von OMATS1 angelegt worden sind, werden automatisch auf das neue
Modell konvertiert (s.a. Anh. B).


 Beim Export einer OFML–Materialdefinitionsdatei werden die fehlenden Parameter f¨ur die Verarbeitung nach OMATS1 — umgekehrt zum vorigen Punkt — aus den Parametern f¨ur OMATS2
abgeleitet (und mit exportiert).

## **2 Die Materialmodelle**


**2.1** **¨Ubersicht**


Jedes Model definiert ein Set von Parametern zur Beschreibung von spezifischen Eigenschaften eines
Materials.


Eine besondere Rolle spielt dabei der Parameter _Materia_ ~~_l T_~~ _ype_ (s. 2.3): in Abh¨angigkeit vom gew¨ahlten
Materialtyp werden beim Rendering nur bestimmte Materialparameter verwendet.


Die folgende Tabelle gibt einen Uberblick ¨uber die definierten Materialparameter (in alphabetischer Rei- [¨]
henfolge ihrer Bezeichner) und deren Zuordnung zu den Materialtypen und den beiden Modellen.


Alle Parameter sind optional, d.h. m¨ussen in einer Materialbeschreibung nicht zwingend angef¨uhrt werden [1] .
Fehlt eine Parameterangabe, wird ein vordefinierter Wert verwendet. Dieser wird unten bei der Beschreibung der einzelnen Parameter spezifiziert.
F¨ur einige Parameter, die nur f¨ur das Modell OMATS2 definiert sind, ist kein Wert vordefiniert. Vielmehr
wird bei fehlender Parameterangabe der Wert aus OMATS1–Parametern (bzw. deren Default–Werten)
abgeleitet [2] .


1Theoretisch sind damit auch leere Materialbeschreibungen m¨oglich.
2Das Verfahren f¨ur diese Ableitung ist nicht definiert und kann also von Anwendung zu Anwendung variieren.


2


|Parameter|Materialtyp|Col3|Col4|Modell|Col6|
|---|---|---|---|---|---|
|Parameter|Common|Glass|Illuminant|OMATS1|OMATS2|
|Base~~ C~~olor|X|X||X|X|
|Base~~ C~~olo~~r M~~ap|X|X||X|X|
|Clearcoat|X||||X|
|Clearcoa~~t R~~oughness|X||||X|
|Clearcoa~~t N~~orma~~l M~~ap|X||||X|
|Emissive~~ C~~olor|||X|X|X|
|Emissive~~ C~~olo~~r M~~ap|||X||X|
|Luminance|||X|X|X|
|Metallness|X||||X|
|Metallnes~~s M~~ap|X||||X|
|Norma~~l M~~ap|X|X||X|X|
|Opacit~~y M~~ap|X|X|X||X|
|Refractiv~~e I~~ndex||X||X|X|
|Roughness|X|X|||X|
|Roughness~~ M~~ap|X|X|||X|
|Sheen|X||||X|
|Shee~~n C~~olor|X||||X|
|Shee~~n R~~oughness|X||||X|
|Shininess|X|||X||
|Soun~~d A~~bsorption|X|X|X|X|X|
|Specula~~r C~~olor|X|||X||
|Specula~~r F~~actor|X|||X||
|Transparency|X|X|X|X|X|


**2.2** **Verwendete Datentypen**


Bei der Beschreibung der Parameter werden folgende Datentypen verwendet:


`PI` positive Ganzzahl


`FP` allgemeine Gleitkommazahl


`RGB` Vektor aus drei Farbwerten zur Repr¨asentation der Basisfarben Rot, Gr¨un und Blau


Jeder Farbwert _C_ muss im Bereich 0 _._ 0 _≤_ _C ≤_ 1 _._ 0 liegen.


`RGB-IMAGE` Bilddatei


Dieser Datentyp beschreibt zweidimensionale Bilddateien, bestehend aus RGB–Farbwerten.


Erlaubt sind folgende Formate: PNG, JPEG


Die Dimensionen der Bilddateien sollen Zweierpotenzen sein.


Die maximal erlaubte Gr¨oße ist 4.096 x 4.096, generell sollten die Texturen jedoch nur so
hoch wie n¨otig aufgel¨ost werden.


Je nach Beschaffenheit des Materials gelten dabei die folgenden Empfehlungen:


 1.024 x 1.024 – sehr feine, hoch-strukturierte Materialien
 512 x 512 – ”normale“ Materialien
 256 x 256 – einfache, gering-strukturierte Materialien


3


Die Bilddateien sind normalerweise so anzulegen, dass eine Wiederholung in beiden Dimensionen optisch ansprechend m¨oglich ist.


Die Namensvergabe der Bilddateien ist beliebig.


`RGBA-IMAGE` Bilddatei mit Transparenz


Dieser Datentyp stellt eine Erweiterung des Typs `RGB-IMAGE` dar und beinhaltet einen zus¨atzlichen Transparenzwert. Dieser kann entweder ein skalarer Wert sein oder ein expliziter
Farbwert, welcher digital die Transparenz steuert, d.h. Texel [3] mit diesem Farbwert werden
transparent dargestellt.


Erlaubt sind folgende Formate: PNG

```
GRAYSCALE-IMAGE
```

Graustufen–Bilddatei


Im Gegensatz zu `RGB-IMAGE` umfasst dieser Typ nur einen Wert pro Pixel [4] .


Erlaubt sind folgende Formate: PNG, JPEG


`SYMBOL` symbolischer Bezeichner


Anmerkung zu den Datentypen `RGB`, `RGB-IMAGE` und `RGBA-IMAGE` :
F¨ur RGB–Farbwerte wird der sRGB–Farbraum angenommen.


Anmerkungen zu den Bilddatei–Formaten PNG und JPEG:


Bilder im PNG–Format m¨ussen gem¨aß der ”PNG (Portable Network Graphics) Specification“ [png] angelegt werden:


 m¨ussen sequenziell (nicht interlaced/progressive) strukturiert sein


 m¨ussen im Fall von `RGB-IMAGE` das RGB–Farbmodell nutzen


 m¨ussen 8 Bit pro (Farb-)Kanal nutzen


 d¨urfen nicht animiert sein


Bilder im JPEG–Format m¨ussen gem¨aß der Spezifikation f¨ur das ”JPEG File Interchange Format“ [jfif]
angelegt werden:


 m¨ussen sequenziell (nicht interlaced/progressive) strukturiert sein


 m¨ussen _Huffman_ -Kodierung (nicht arithmetische Kodierung) nutzen

 m¨ussen das YCbCr–Farbmodell nutzen [5]


 m¨ussen 8 Bit pro Farbkanal nutzen


Bilder in beiden Formaten sollten keine eingebetteten Metadaten (Thumbnails, EXIF, IPTC, ICC-Profile
etc.) enthalten [6] . Falls Metadaten enthalten sind, d¨urfen diese keine Rotation enthalten [7] .


3in der 3D-Computergrafik ein Pixel einer Textur
4Liegt eine Bilddatei mit 3 Farbkan¨alen (RGB) vor, wird der Grauwert der Farbe verwendet.
5Beim Import findet eine Konvertierung in das RGB–Farbmodell statt.
6Diese bringen bei Texturen keinen Nutzen und erh¨ohen nur unn¨otig die Dateigr¨oße sowie die Download-Zeit.
7Ansonsten kann es zu einer fehlerhaften Darstellung kommen.


4


**2.3** **Materialtypen**


Der Materialtyp (Parameter _Materia_ ~~_l T_~~ _ype_ vom Typ `SYMBOL` ) dient der Auswahl eines passenden Shaders [8] .
Weiterhin k¨onnen auf Basis des Materialtyps die verf¨ugbaren Parameter in einem Materialeditor eingeschr¨ankt werden.


Folgende Typen sind definiert:


 _Common_


Dies ist der empfohlene Default-Typ, wenn dem Material keiner der u.g. spezifischen Materialtypen
zugeordnet werden kann oder soll.


 _Glass_


Dieser Typ sollte allen Glas–Materialien zugeordnet werden. Wird stattdessen der Typ _Common_
verwendet, kann es z.B. dazu kommen, dass das Material wie transparenter Kunststoff wirkt.


 _Illuminant_


Materialtyp f¨ur selbstleuchtende Objekte. Sinnvoll in Verbindung mit dem Parameter _Luminance_ .


Fehlt der Parameter, so ermittelt die Applikation anhand von Heuristiken aus den anderen angegebenen
Parametern selbst¨andig einen passenden Shader [9] (welcher dann u.U. nicht die gew¨unschten Ergebnisse
bringt).


**2.4** **Die Materialparameter**


Vorbemerkungen:
Die Parameter sind in alphabetischer Reihenfolge angef¨uhrt. Hinter dem Bezeichner des Parameters
werden in eckigen Klammern jeweils der Datentyp sowie in geschweiften Klammern die zugeh¨origen
Materialtypen und Modelle angegeben.


 _Bas_ ~~_e C_~~ _olor_ `[RGB]` _{Common_, _Glass_, OMATS1, OMATS2 _}_


Die Basisfarbe dient zur Simulation der diffusen Reflexionseigenschaften der Objektoberfl¨ache. Im
Modell OMATS2 dient der Parameter bei Metallen auch zur Bestimmung von Farbe und Intensit¨at
der spiegelnden Reflexion.


Der vordefinierte Wert ist `1.0,1.0,1.0` (weiss).


 _Bas_ ~~_e C_~~ _olo_ ~~_r M_~~ _ap_ `[RGB-IMAGE, RGBA-IMAGE, GRAYSCALE-IMAGE]` _{Common_, _Glass_, OMATS1, OMATS2 _}_


Die hierdurch referenzierte Bilddatei dient zur ersetzenden Beschreibung des Parameters
_Bas_ ~~_e C_~~ _olor_ .


Im Fall eines `RGBA-IMAGE` dienen die Transparenzwerte, welche sich aus dem Alpha–Kanal ergeben,
zus¨atzlich zur ersetzenden Beschreibung des Parameters _Transparency_ .


Im Fall eines `GRAYSCALE-IMAGE` wird f¨ur alle 3 Farbkan¨ale der gleiche Wert verwendet.


Es ist keine Bilddatei diesbez¨uglich vordefiniert.


Zu den Textur–Mapping–Verfahren siehe Abschn. 3.


8 _Shader_ sind Programme zur Berechnung von Rendering–Effekten, z.B. f¨ur die r¨aumliche Wahrnehmung von 3D–
Modellen.
9Wenn z.B. der Luminance–Wert des Materials ¨uber `0` liegt und das Material nicht texturiert ist, wird der Typ _Illuminant_
angenommen.


5


 _Clearcoat_ `[FP]` _{Common_, OMATS2 _}_


Mit diesem Parameter l¨asst sich eine Klarlackschicht auf der darunter liegenden Oberfl¨ache simulieren.


Die Werte liegen im Bereich von `0.0` bis `1.0` und steuern die St¨arke der Klarlackschicht.


Der vordefinierte Wert ist `0.0` .


 _Clearcoa_ ~~_t N_~~ _orma_ ~~_l M_~~ _ap_ `[RGB-IMAGE]` _{Common_, OMATS2 _}_


Erm¨oglicht die Anderung der Oberfl¨achennormalen f¨ur die Klarlackschicht. Wenn diese Map nicht [¨]
vorhanden ist, werden stattdessen die Oberfl¨achennormalen verwendet. Das bedeutet, dass die Klarlackebene nicht von der regul¨aren Normal-Map betroffen ist (Parameter _Norma_ ~~_l_~~ _Map_ ).


Die Werte in der referenzierten Bilddatei werden als normierte Normalenvektoren interpretiert.


Es ist keine Bilddatei diesbez¨uglich vordefiniert.


Zu den Textur–Mapping–Verfahren siehe Abschn. 3.


 _Clearcoa_ ~~_t R_~~ _oughness_ `[FP]` _{Common_, OMATS2 _}_


Der Grad der Rauheit des Klarlacks bestimmt, wie glatt oder rau die Oberfl¨ache der Klarlackschicht
ist.


Die Werte liegen im Bereich von `0.0` bis `1.0` .


Der vordefinierte Wert ist `0.0` .


 _Emissiv_ ~~_e C_~~ _olor_ `[RGB]` _{Illuminant_, OMATS1, OMATS2 _}_


Definiert die Farbe des emittierten Lichts einer geometrie-basierten Lichtquelle.


Der vordefinierte Wert ist `0.0,0.0,0.0` (schwarz).


 _Emissiv_ ~~_e C_~~ _olo_ ~~_r M_~~ _ap_ `[RGB-IMAGE, GRAYSCALE-IMAGE]` _{Illuminant_, OMATS2 _}_


Die hierdurch referenzierte Bilddatei dient zur ersetzenden Beschreibung des Parameters
_Emissiv_ ~~_e C_~~ _olor_ .


Im Fall eines `GRAYSCALE-IMAGE` wird f¨ur alle 3 Farbkan¨ale der gleiche Wert verwendet.


Es ist keine Bilddatei diesbez¨uglich vordefiniert.


Zu den Textur–Mapping–Verfahren siehe Abschn. 3.


 _Luminance_ `[FP]` _{Illuminant_, OMATS1, OMATS2 _}_


Spezifiziert die Leuchtdichte einer geometrie-basierten Lichtquelle in _cd/m_ [2] .
( _Candela_  - _cd_  - ist die SI–Einheit der Basisgr¨oße _Lichtst¨arke_ .)


Der vordefinierte Wert ist `0.0` .


 _Metallness_ `[FP]` _{Common_, OMATS2 _}_


In der realen Umwelt sind Materialien in Metalle und Nicht-Metalle einteilbar. Deshalb sollte dieser
Wert bei den meisten Materialien `0.0` oder `1.0` sein. Zwischenwerte dienen zur Darstellung von
Halbmetallen oder verunreinigten Metallen.


6


 _Metallnes_ ~~_s M_~~ _ap_ `[GRAYSCALE-IMAGE]` _{Common_, OMATS2 _}_


Die hierdurch referenzierte Bilddatei dient zur ersetzenden Beschreibung des Parameters _Metallness_ :
Helle Bildbereiche erhalten Metalleigenschaften, dunkle werden als Nicht–Metall interpretiert.


Es ist keine Bilddatei diesbez¨uglich vordefiniert.


Zu den Textur–Mapping–Verfahren siehe Abschn. 3.


 _Norma_ ~~_l M_~~ _ap_ `[RGB-IMAGE]` _{Common_, _Glass_, OMATS1, OMATS2 _}_


Eine Normal-Map modifiziert die Normalenvektoren der Oberfl¨ache, so dass damit die Beleuchtung
von Erhebungen und Vertiefungen simuliert werden kann, welche in der Objektgeometrie nicht
vorhanden sind.


Die Werte in der referenzierten Bilddatei werden als normierte Normalenvektoren interpretiert.


Es ist keine Bilddatei diesbez¨uglich vordefiniert.


Zu den Textur–Mapping–Verfahren siehe Abschn. 3.


 _Opacit_ ~~_y M_~~ _ap_ `[GRAYSCALE-IMAGE]` _{Common_, _Glass_, _Illuminant_, OMATS2 _}_


Die hierdurch referenzierte Bilddatei dient zur ersetzenden Beschreibung des Parameters
_Transparency_ .


Die Werte in der Bilddatei werden als `alpha` interpretiert, mit `transparency = 1.0 - alpha` .


Wenn der Parameter _Bas_ ~~_e C_~~ _olo_ ~~_r M_~~ _ap_ ein RGBA-IMAGE referenziert, wird dessen Alphakanal
ignoriert, wenn es eine _Opacit_ ~~_y M_~~ _ap_ gibt.


Es ist keine Bilddatei diesbez¨uglich vordefiniert.


Zu den Textur–Mapping–Verfahren siehe Abschn. 3.


 _Refractiv_ ~~_e I_~~ _ndex_ `[FP]` _{Glass_, OMATS1, OMATS2 _}_


Der skalare Wert gibt im Fall transparenter Materialien die Brechung des Lichts an. Unter dem
Brechungsindex versteht man das Verh¨altnis der Phasengeschwindigkeit des Lichts im Vakuum zu
der im jeweiligen Material.


Ausgew¨ahlte Werte sind:


**–** Wasser: `1.33`

**–** Glas: `1.5` ... `1.9`


Der vordefinierte Wert ist `1.0` und entspricht dem Brechungsindex von Vakuum.


 _Roughness_ `[FP]` _{Common_, _Glass_, OMATS2 _}_


Der Grad der Rauheit bestimmt, wie glatt oder rau eine Oberfl¨ache ist. Je nach Rauheitsgrad wird
das reflektierte Licht an der Oberfl¨ache schw¨acher oder st¨arker gestreut.


Die Werte liegen im Bereich von `0.0` bis `1.0` .


 _Roughnes_ ~~_s M_~~ _ap_ `[GRAYSCALE-IMAGE]` _{Common_, _Glass_, OMATS2 _}_


Die hierdurch referenzierte Bilddatei dient zur ersetzenden Beschreibung des Parameters _Roughness_ :
Helle Bereiche des Bildes erscheinen matt, dunkle Bildbereiche gl¨anzend.


Es ist keine Bilddatei diesbez¨uglich vordefiniert.


Zu den Textur–Mapping–Verfahren siehe Abschn. 3.


7


 _Sheen_ `[FP]` _{Common_, OMATS2 _}_


_Sheen_ [10] ist eine zus¨atzliche Reflexionsschicht, die Mikrofasern auf der darunter liegenden Oberfl¨ache
simuliert. Sie kann verwendet werden, um Samt-Materialien zu erzeugen.


Der Parameter steuert die St¨arke des Effekts mit Werten im Bereich von `0.0` bis `1.0` .


Der vordefinierte Wert ist `0.0` .


 _Shee_ ~~_n C_~~ _olor_ `[RGB]` _{Common_, OMATS2 _}_


Die Farbe von Mikrofasern (s. Parameter _Sheen_ ). T¨ont die _Sheen_ -Reflexion.


Der vordefinierte Wert ist `1.0,1.0,1.0` (weiß).


 _Shee_ ~~_n R_~~ _oughness_ `[FP]` _{Common_, OMATS2 _}_


Steuert, wie sich die _Sheen_ -Reflexion ¨uber die Oberfl¨ache verteilt.


Kleinere Werte f¨uhren zu scharfen Reflexionen bei Streifwinkeln, w¨ahrend gr¨oßere Werte weichere
Reflexionen auf der gesamten Oberfl¨ache verursachen.


Der Parameter modelliert, wie stark die Ausrichtung der Mikrofasern von der Oberfl¨achennormalen
abweicht.


Die Werte liegen im Bereich von `0.0` bis `1.0` .


Der vordefinierte Wert ist `0.5` .


 _Shininess_ `[FP]` _{Common_, OMATS1 _}_


Der skalare Wert gibt den Glanz f¨ur gl¨anzende Oberfl¨achen an. Hierbei handelt es sich um den
ganzzahligen Exponenten des _cos-Terms_ entsprechend dem Beleuchtungsmodell nach _Phong_ .


Als Faustregel gilt: Je gr¨oßer dieser Wert, desto kleiner der die Spiegelung der Lichtquelle simulierende Glanzeffekt.


Der vordefinierte Wert ist `30` .


 _Soun_ ~~_d A_~~ _bsorption_ `[PI {PI FP}*]` _{Common_, _Glass_, _Illuminant_, OMATS1, OMATS2 _}_


Der Parameter wird als eine Menge von Wertepaaren dargestellt, welche f¨ur unterschiedliche Frequenzen (1. Wert) den Schallabsorptionsgrad (2. Wert) angeben. Vor den Wertepaaren wird die
Anzahl der Wertepaare spezifiziert.


¨Ublicherweise wird der Schallabsorptionsgrad f¨ur folgende Frequenzen angegeben:
125Hz, 250Hz, 500Hz, 1000Hz, 2000Hz, 4000Hz.


Der Schallabsorptionsgrad ist eine nicht-negative Gleitkommazahl. Normalerweise liegt der Wert
zwischen `0.0` (keine Absorption) und `1.0` (vollst¨andige Absorption). Es k¨onnen aber auch Werte
vorkommen, die leicht ¨uber `1.0` liegen. Das ist dann m¨oglich, wenn die tats¨achlich wirksame Fl¨ache
eines schallabsorbierenden Objekts gr¨oßer ist als die geometrische Fl¨ache, die f¨ur die AkustikBerechnung herangezogen wird.


Fehlt die Angabe in der Materialbeschreibung, wird das entsprechende Objekt bei der Akustikberechnung nicht ber¨ucksichtigt.


Beispiel: `6 125 0.1 250 0.3 500 0.2 1000 0.1 2000 0.5 4000 0.4`


10der Glanz, Schimmer eines Stoffes


8


 _Specula_ ~~_r C_~~ _olor_ `[RGB]` _{Common_, OMATS1 _}_


Die spekulare Farbe dient zur Simulation der spekularen [11] Reflexionseigenschaften der Objektoberfl¨ache und legt gleichzeitig die Farbe f¨ur Glanzlichter (Phong-Modell) fest.


Der vordefinierte Wert ist `0.0,0.0,0.0` (schwarz).


 _Specula_ ~~_r F_~~ _actor_ `[FP]` _{Common_, OMATS1 _}_


Die Gewichtung der spekularen Farbe dient zur Steuerung der Intensit¨at der spekularen Reflektion
der Objektoberfl¨ache. Glanzlichter (Phong-Modell) werden davon nicht beeinflusst.


Die Werte liegen ¨ublicherweise im Bereich von `0.0` bis `1.0` .


Der vordefinierte Wert ist `1.0` .


 _Transparency_ `[FP]` _{Common_, _Glass_, _Illuminant_, OMATS1, OMATS2 _}_


Die Transparenz dient zur Simulation transparenter Eigenschaften des Materials. Es handelt sich
um einen frequenzunabh¨angigen skalaren Wert.


Die Werte liegen im Bereich von `0.0` bis `1.0` .


Der vordefinierte Wert ist `0.0`, d.h. es liegt keine Transparenz vor.


11spiegelnden


9


## **3 Textur–Mapping–Verfahren**

Im Rahmen von OMATS werden die in diesem Abschnitt beschriebenen Textur–Mapping–Verfahren
unterst¨utzt.
Diese beziehen sich auf die Materialparameter _Bas_ ~~_e_~~ _Colo_ ~~_r_~~ _Map_, _Clearcoa_ ~~_t N_~~ _orma_ ~~_l M_~~ _ap_,
_Emissiv_ ~~_e C_~~ _olo_ ~~_r M_~~ _ap_, _Metallnes_ ~~_s M_~~ _ap_, _Norma_ ~~_l M_~~ _ap_, _Opacit_ ~~_y M_~~ _ap_ und _Roughnes_ ~~_s_~~ _Map_ .


Ausgangspunkt sind dabei die Datentypen `RGB-IMAGE`, `RGBA-IMAGE` bzw. `GRAYSCALE-IMAGE` wie in 2.2
definiert. Diese werden, wie in Abb. 1 dargestellt, auf den normierten U-V–Koordinatenraum projiziert,
auf den alle weiteren Ausf¨uhrungen in diesem Abschnitt Bezug nehmen.


Abbildung 1: U-V–Koordinatenraum


Die folgenden Textur-Transformationen werden unterst¨utzt (in dieser Reihenfolge):


1. Rotation um den Winkel W


2. Verschiebung um einen U-V–Offset


3. Skalierung im U-V–Raum


F¨ur Normal–Maps k¨onnen separate Transformationsparameter angegeben werden (in Bezug auf die anderen Maps). Ist kein spezifischer Transformationsparameter f¨ur die Normal–Map angegeben, wird der
entsprechende Parameter f¨ur die anderen Maps verwendet (wenn vorhanden).


**3.1** **Ebenen-Mapping**


Hierbei handelt es sich um eine planare Abbildung in eine vorgegebene Projektionsebene. Diese beschreibt
die Lage des U-V–Raums und kann folgendermaßen gew¨ahlt werden:


 YZ-Ebene


 XZ-Ebene


 XY-Ebene


 Definition durch normierten Normalenvektor


Zus¨atzlich kann die Projektionsebene beliebig um alle Koordinatenachsen rotiert werden.


Verschiebung und Skalierung im U-V–Raum wird ebenfalls unterst¨utzt.


10


**3.2** **Quader–Mapping**


Beim Quader-Mapping handelt es sich um die automatische Abbildung der Modellkoordinaten in die Begrenzungsfl¨achen eines achsenparallel ausgerichteten Quaders. Entlang jeder Koordinatenachse definiert
der Quader einen eigenen U-V–Raum. Entsprechen die Basisvektoren _U_ und _V_ jeweils einem kanonischen
Basisvektor, lassen sich 8 Basisvarianten angeben, wie in Abb. 2 dargestellt.


Abbildung 2: Quader–Mapping


Im Tripel hinter dem Schl¨usselwort `auto` ist f¨ur jede Seite des Quaders die Richtung des
U–Vektors kodiert, und zwar in der Reihenfolge vorne, rechts, oben.

(Die Ausrichtungen hinten, links und unten sind analog.)


Spiegelungen der Quader an den Koordinatenebenen f¨uhren zu entsprechenden Spiegelungen im U-V–
Raum, und damit zu weiteren Varianten. Translation, Skalierung und Rotation im U-V–Raum werden
ebenfalls unterst¨utzt. Die Zuordnung eines Vertices zu einer Seitenfl¨ache des Quaders erfolgt anhand der
betragsgr¨oßten Koordinate des Normalenvektors.


**3.3** **Texturkoordinaten**


Nicht immer l¨aßt sich das gew¨unschte Ergebnis mittels allgemeiner Mapping–Verfahren beschreiben.
Manchmal ist eine explizite Angabe der U-V–Koordinaten erforderlich. Diese werden dann nicht am
Material, sondern an der Geometrie selbst hinterlegt. Damit entf¨allt die Berechnung der Projektion aus
dem Modellbereich in den U-V–Raum. Skalierung, Offset und Rotation werden weiterhin auf die U-V–
Koordinaten angewendet.


Dieses Verfahren wird auf alle Arten von Maps angewendet.


Wie die Texturkoordinaten hinterlegt sind, richtet sich nach dem Geometrieformat, d.h. die Angabe von
Texturkoordinaten muss dort vorgesehen sein. Daf¨ur k¨onnen die Formate 3DS und OBJ genutzt werden.


11


## 4 OFML –Datenformat f¨ur Materialien

Vorbemerkung:
Die Ausf¨uhrungen in diesem Abschnitt ersetzen bzw. aktualisieren die Ausf¨uhrungen in Anhang D.2

” [Materialien“ aus [][ofml][]!]


Die Definition eines OFML–Materials beinhaltet ein Set von _Parametern_ . Ein Parameter besteht aus
einem Schl¨ussel, der die Bedeutung des Parameters definiert, gefolgt von durch Leerzeichen getrennten
Argumenten [12] . In den Tabellen unten sind die aktuell unterst¨utzten Schl¨ussel und zugeh¨origen Argumente
definiert.


Eine Materialdefinition kann in in zwei Formaten repr¨asentiert werden, die sich im Wesentlichen durch
die Form der Trennung der Parameter unterscheiden:


 **Materialdefinitionsdatei**


Die Parameter werden durch ein Zeilenende getrennt.


Der Name einer Materialdefinitionsdatei (Erweiterung `.mat` ) ergibt sich aus der letzten Komponente
des vollqualifizierten Bezeichners des Materials, unter dem es in den OFML–Daten (z.B. ODB,
OFML–Part I [odb]) referenziert wird, wobei f¨ur den Dateinamen Kleinschreibweise vorgeschrieben
ist [13] .


 **Inline–Deklaration**


Die Parameter werden durch ein Semikolon (’ `;` ’) getrennt.


Inline–Deklarationen k¨onnen bei der OFML–Programmierung gem¨aß Part III der OFML–
Spezifikation [ofml] oder bei der ODB–Datenanlage [odb] verwendet werden.


Inline-Deklarationen sind in zwei Formen m¨oglich:


**–** _Reine Inline–Deklarationen_ beginnen mit dem Dollar-Zeichen (’ `$` ’) und enthalten eine vollst¨andige Materialdefinition.

**–** _Material–Modifikatoren_ beginnen mit einem voll qualifizierten Materialnamen, der auf ein Material in der OFML–Datenbasis verweist (Basis–Material). Danach folgen, durch Semikolon
getrennt, einzelne Parameter, die den entsprechenden Parameter des Basis–Materials ¨uberschreiben.


Die bei der Beschreibung der Argumente in den folgenden Tabellen verwendeten syntaktischen und lexikalischen Elemente sind in der Legende am Ende dieses Abschnitts beschrieben.


12Es gibt auch Parameter ohne Argumente.
13Der Name eines Materials – ohne den vorangestellten Paket-Namensraum – sollte den Regeln f¨ur OFML–Bezeichner
folgen, also nur alphanumerische Zeichen (inklusive dem Unterstrich) enthalten und nicht mit einer Ziffer beginnen.


12


Die folgende Tabelle definiert f¨ur alle aktuell unterst¨utzten Modell–Parameter (s. Abschn. 2) die zugeh¨origen Schl¨ussel und Argumente:

|Parameter|Modell|Schlu¨ssel|Argument(e)|
|---|---|---|---|
|Materia~~l T~~ype|1, 2|`type`|`(common|glass|illuminant)`|
|Base Color|1, 2|`dif`|`R[F1] G[F1] B[F1]`|
|Base Colo~~r M~~ap|1, 2|`tex image`|`FT[FT] FN[FN]`|
|Clearcoat|2|`clearcoat`|`S[F1]`|
|Clearcoa~~t N~~orma~~l M~~ap|2|`clearcoat_bumps`|`FT[FT] FN[FN]`|
|Clearcoa~~t R~~oughness|2|`clearcoat_roughness`|`S[F1]`|
|Emissiv~~e ~~Color|1, 2|`emission`|`R[F1] G[F1] B[F1]`|
|Emissiv~~e ~~Colo~~r M~~ap|2|`emission image`|`FT[FT] FN[FN]`|
|Luminance|1, 2|`luminance`|`S[F]`|
|Metallness|2|`metallic`|`S[F1]`|
|Metallnes~~s M~~ap|2|`metallic image`|`FT[FT] FN[FN]`|
|Norma~~l M~~ap|1, 2|`bumps`|`FT[FT] FN[FN]`|
|Opacit~~y M~~ap|2|`opacity image`|`FT[FT] FN[FN]`|
|Refractiv~~e I~~ndex|1, 2|`refraction`|`S[F]`|
|Roughness|2|`roughness`|`S[F1]`|
|Roughnes~~s M~~ap|2|`roughness image`|`FT[FT] FN[FN]`|
|Sheen|2|`sheen`|`S[F1]`|
|Shee~~n C~~olor|2|`sheen_color`|`R[F1] G[F1] B[F1]`|
|Shee~~n R~~oughness|2|`sheen_roughness`|`S[F1]`|
|Shininess|1|`shi`|`S[F]`|
|Soun~~d A~~bsorption|1, 2|`sndabsorb`|`N[I] {F[I] C[F]}*`|
|Specula~~r C~~olor|1|`spe`|`R[F1] G[F1] B[F1]`|
|Specula~~r F~~actor|1|`reflection`|`S[F]`|
|Transparency|1, 2|`tra`|`S[F1]`|



(Legende siehe unter der n¨achsten Tabellle.)


13


Die folgende Tabelle definiert die Schl¨ussel und zugeh¨origen Argumente, die f¨ur die Textur–Mapping–

|Verfahren (s. Abschn.|3) ben¨otigt werden:|Col3|
|---|---|---|
|Parameter|Schl¨ussel|Argument(e)|
|_Transformationena_<br>Rotation<br>Verschiebung<br>Skalierung|`rotate`,` nrotate`,` clearcoat_rotate`<br>`offset`,` noffset`,` clearcoat_offset`<br>`scale`,` nscale`,` clearcoat_scale`|`0 0 A[F]`<br>`U[F] V[F] 0`<br>`U[F] V[F] 0`|
|_Ebenen–Mapping_<br>YZ–Ebene<br>XZ–Ebene<br>XY–Ebene<br>Normalenvektor|`prjx`<br>`prjy`<br>`prjz`<br>`prj`|`X[F1] Y[F1] Z[F1]`|
|_Quader–Mapping_|`auto`|`(xyx|xzx|xzz|yyz|xyz|yyx|yzx|yzz)`|
|_Texturkoordinatenb_|`import`||



_a_ Die Parameter, deren Schl¨ussel mit dem Buchstaben ’n’ beginnen, wirken sich nur auf die Normal–Map aus. Ist einer
dieser Parameter nicht angegeben, greift der entsprechende Parameter ohne den Buchstaben ’n’ am Anfang.
Die Parameter, deren Schl¨ussel mit ”clearcoat ~~“~~ beginnen, wirken sich nur auf die Clearcoat-Normal-Map aus. Ist einer
dieser Parameter nicht angegeben, greift der entsprechende Parameter ohne ”clearcoat _b_ In der Objekt–Geometrie hinterlegte Texturkoordinaten werden nur ausgewertet, wenn der ~~“~~ am Anfang. `import` –Parameter vorhanden ist. Wenn der Schl¨ussel angegeben ist, in der Geometrie aber keine Texturkoordinaten hinterlegt sind, ist das Verhalten
undefiniert.


Legende:


 Ein Argument wird entweder durch explizite Auflistung der m¨oglichen (alternativen) Werte in der
Form `(Wert1|Wert2|...)` beschrieben [14], oder in der Form `Name[Typ]`, wobei der Name die Semantik des Arguments bezeichnet.


 Eine sich wiederholende Menge von Argumenten wird in der Form `{Arg1 ...}*` dargestellt.


 Folgende Bezeichner (Abk¨urzungen) werden f¨ur benannte Argumente verwendet:


**–** `S`    - skalarer Wert

**–** `FT`, `FN`    - Dateityp, Dateiname

**–** `R`, `G`, `B`    - Rot, Gr¨un, Blau

**–** `U`, `V`, `A`    - U-V–Koordinaten bzw. -Skalierung, Winkel

**–** `X`, `Y`, `Z`    - X-Y-Z–Koordinaten

**–** `N`, `F`, `C`    - Anzahl, Frequenz, Absorptionsgrad


 Folgende Bezeichner werden f¨ur die Typen der Argumente verwendet:


**–** `F`    - Gleitkommazahl

**–** `F1`    - Gleitkommazahl im Bereich `0.0` bis `1.0`

**–** `I`    - Ganzzahl

**–** `FT`    - Dateityp: `(png|jpg)`

**–** `FN`    - Dateiname: (ggf. voll qualifizierter) OFML–Name, der auf eine Bilddatei in der OFML–
Datenbasis verweist [15]


14Bei einem einzigen m¨oglichen Wert entf¨allt die umschließende Klammer.
15Eine Qualifizierung ist notwendig, wenn sich die Bilddatei nicht im Datenverzeichnis der OFML–Serie befindet, in
der auch die Materialdefinitionsdatei hinterlegt ist bzw. zu der die OFML–Instanz geh¨ort, auf die eine Inline–Deklaration
angewendet wird.


14


## **A Einf¨uhrung in das physikalische Rendering (PBR)**

PBR ( **P** hysically **B** ased **R** endering) simuliert, was geschieht, wenn Licht auf die Oberfl¨ache eines Objektes
trifft. Ein physikalisch korrekt beschriebenes Material interagiert mit Licht auf unterschiedliche Arten:
Es wirft Licht zur¨uck ( _Reflexion_ ), bricht es ( _Refraktion_ ) oder schluckt es ( _Absorption_ ). Unter nat¨urlichen
Bedingungen wird Licht nicht zu hundert Prozent absorbiert, reflektiert oder gebrochen – alle Materialien
bewegen sich in dem Spektrum zwischen diesen Extremen.


Ein Material ist sichtbar, weil es einfallendes Licht reflektiert. Alternativ nehmen wir auch Materialien
wahr, die selbst Licht aussenden.


Die drei oben genannten grunds¨atzlichen M¨oglichkeiten der Interaktion von Licht und Material werden
u.a. durch die **Materialeigenschaften** beeinflußt:


 Die Materialeigenschaften bestimmen die Art der Reflexion:
Bei der _spekularen_ [16] _Reflexion_ wird das Licht direkt an der Oberfl¨ache zur¨uckgeworfen.
_Diffuse Reflexion_ entsteht durch Streuung innerhalb des Materials (Lichtstrahlen dringen ein St¨uck
weit in das Material ein und werden in verschiedene Richtungen abgelenkt.)
Metalle reflektieren ausschließlich spekular, Nichtmetalle ¨uberwiegend diffus.


 Je nach Materialbeschaffenheit dringen die Lichtstrahlen tiefer in das Material ein. Sie werden entweder durch das Material hindurchgeleitet ( _Transparenz_ ), innerhalb des Materials zur¨uckgeworfen
( _Transluzenz_ ) oder vom Material geschluckt ( _Absorption_ ).



Spekulare
Reflexion



Diffuse Reflexion Transparenz Transluzenz Absorption



Neben den Materialeigenschaften (s.o.) werden beim PBR auch die physikalischen **Eigenschaften des**
**Lichts** ber¨ucksichtigt:


 Nach dem Grundsatz der Energieerhaltung wird nie mehr Licht reflektiert, als einstrahlt.
Ein – nicht selbstleuchtendes – Material wird der Beleuchtung der Umgebung entsprechend dargestellt.


 Die Menge der reflektierten Lichtstrahlen ist vom Blickwinkel abh¨angig. Dieser sogenannte _Fresnel–_
_Effekt_ bewirkt, dass Oberfl¨achen bei einem flachen Blickwinkel [17] st¨arker spiegeln als wenn man
senkrecht auf die Oberfl¨ache schaut.


Die Eigenschaften des Lichts werden ¨uber den Shader der Applikation simuliert und sind nicht direkt
durch den Benutzer (Materialdatenanleger) beeinflussbar.


Die Materialeigenschaften hingegen sind die Hebel f¨ur die Erstellung physikalisch stimmiger Materialien.
Das am PBR orientierte Materialmodell OMATS2 definiert entsprechende Parameter, wobei die Menge
der relevanten Materialparameter vom festgelegten _Materialtyp_ abh¨angig ist (s. Abschn. 2).


16spiegelnden
17englisch: grazing angle


15


## **B Konvertierung von alten Materialien auf das neue Modell**

Materialien, die nach dem alten Model OMATS1 angelegt worden sind, werden von einer OFML–
Applikation, die das neue Modell verwendet, automatisch auf das neue Modell konvertiert.


Im Normalfall liefert diese Konvertierung eine zufriedenstellende Darstellung. In seltenen F¨allen k¨onnen
dennoch Anpassungen erforderlich werden:


 In einigen F¨allen gl¨anzen Materialien st¨arker.
In diesem Fall muss mittels der Angabe des (neuen) Parameters _Roughness_ die Rauheit nachjustiert
werden.


 Eventuell werden Metalle nicht als solche erkannt (dies kann zum Beispiel bei Chrom der Fall sein).
In diesem Fall muss explizit der neue Parameter _Metallness_ (mit dem Wert `1.0` ) angegeben werden.


16


## **C Historie**

Die ersten Versionen dieser Spezifikation wurden von Ekkehard Beier (EasternGraphics GmbH) im Auftrag
des Arbeitskreises _Industrielle Aspekte der OFML–Normung (IAON)_ in Zusammenarbeit mit wegscheider
office solution gmbh (Deutschland) und weber office solution gmbh (Schweiz) erstellt. Beginnend mit Version
1.4 ist die Spezifikation Gegenstand der Normierung durch das OFML–Normungsgremium des IBA.


**Version 2.2 (2025-04-22)**


 Neue Parameter f¨ur das Modell OMATS2 sowie deren entsprechenden Schl¨ussel in OFML–
Materialdefinitionen hinzugef¨ugt:
_Clearcoat_, _Clearcoa_ ~~_t N_~~ _orma_ ~~_l M_~~ _ap_, _Clearcoa_ ~~_t_~~ _Roughness_, _Emissiv_ ~~_e_~~ _Colo_ ~~_r_~~ _Map_, _Opacit_ ~~_y M_~~ _ap_, _Sheen_,
_Shee_ ~~_n C_~~ _olor_ und _Sheen_ ~~_R_~~ _oughness_ .


 Der Schl¨ussel f¨ur den Materialparameter _Refractiv_ ~~_e I_~~ _ndex_ in OFML–Materialdefinitionen wurde in
`refraction` umbenannt.
Der bisherige Schl¨ussel `ref` wird als veraltet deklariert.


 Der unklare und nicht verwendete Dateityp `any` f¨ur Bilddateien in OFML–Materialdefinitionen wurde entfernt.


 Pr¨azisierung zu Verwendung von Metadaten in Bilddateien f¨ur Texturen.


**Version 2.1 (2023-06-27)**


 F¨ur Bilddateien (Datentypen _*-IMAGE_ ) wurde die maximal erlaubte Gr¨oße von 4.096 x 4.096
festgelegt.


**Version 2.0 (2019-06-19)**


 Neues Materialmodell OMATS2


 Neuer Datentyp _GRAYSCALE-IMAGE_


 Datentyp _FP3-IMAGE_ entfernt, daf¨ur Beschreibung des Parameters _Norma_ ~~_l_~~ _Map_ angepasst


 Parameter _Diffus_ ~~_e_~~ _Color_ in _Bas_ ~~_e C_~~ _olor_ und _Diffus_ ~~_e_~~ _Map_ in _Bas_ ~~_e_~~ _Colo_ ~~_r_~~ _Map_ sowie _Refraction_ in
_Refractiv_ ~~_e I_~~ _ndex_ umbenannt


 Materialtypen _Glas_ ~~_s T_~~ _ranslucent_ und _Meta_ ~~_l P_~~ _olished_ sowie Parameter _Ambien_ ~~_t C_~~ _olor_ wegen geringer praktischer Relevanz entfernt


 Das Bilddateiformat TGA ist obsolet


 Entfernung der Bez¨uge auf AutoCAD


 Umstrukturierung des Dokuments


**Version 1.5 (2015-02-27)**


 Neue, explizite Transformationsparameter _nrotate_, _noffset_ und _nscale_ f¨ur Normal-Maps


**Version 1.4, 1. ¨uberarbeitete Fassung (2014-01-08)**


 Korrektur der Typdeklaration (F anstatt F1) bei einigen Materialparametern in refsec:Datenformat


**Version 1.4 (2013-07-17)**


 Neue Materialtypen _Glas_ ~~_s T_~~ _ranslucent_ und _Meta_ ~~_l P_~~ _olished_


 Restrukturierung, Entfernung nicht (mehr) relevanter Abschnitte


17


**Version 1.3 (2011-12-06)**


 Materialtypen


 Selbstleuchtende Materialien


 Schallabsorption


**Version 1.2 (2011-12-14)**


 Phong-Level


 OFML-Datenformat f¨ur Materialien


**Version 1.1 (2007-11-07)**


 Textur-Transformationen und -Mappings


**Version 1.0 (2006-09-25)**


 initiale Version



18


