# Spezifikation OEX OFML Business Data Exchange
## (OFML Part VII)

# **GLOBAL**

Allgemeine Festlegungen und Definitionen

# Version 3.1.0


Editoren:
Markus Behrschmidt, Vitra Services GmbH

Thomas Gerth, EasternGraphicsGmbH


8.5.2023


Copyright © 2006 - 2023 Industrieverband Büro und Arbeitswelt e. V. (IBA)


# Inhalt

**1** **Einleitung .......................................................................................................... 3**

1.1 Übersicht OEX-Spezifikationen ........................................................................... 4

1.2 Versionierung ...................................................................................................... 4

1.3 Legende .............................................................................................................. 5


**2** **Definitionen ....................................................................................................... 6**

2.1 Elementtypen ...................................................................................................... 6
**2.1.1** **Grundelementtypen .................................................................................................................... 6**
2.1.1.1 `Frame` : Rahmenelement ............................................................................................................... 6
2.1.1.2 `Value` : Wertelement ..................................................................................................................... 6
2.1.1.3 `Empty` : Attributelement (leeres Element) ...................................................................................... 6
**2.1.2** **OEX-Rahmentypen (** **`Frame`** **) ....................................................................................................... 7**
2.1.2.1 `DocFrame` : OEX Dokumentenrahmen .......................................................................................... 7
2.1.2.2 `Applic` : Applikation, die das OEX-Dokument erstellt hat ............................................................ 7
2.1.2.3 `File` : Dokumentenmappe ............................................................................................................ 8
2.1.2.4 `Document` : Einzelnes Dokument .................................................................................................. 8
2.1.2.5 `Header` : Belegkopf ....................................................................................................................... 9
2.1.2.6 `Item` : Dokumentenposition ........................................................................................................... 9
2.1.2.7 `DateTime` : Datums- und Zeitangaben ......................................................................................... 9
2.1.2.8 `OrgData` : Organisationsdaten .................................................................................................... 11
2.1.2.9 `Address` : Adressen .................................................................................................................... 11
2.1.2.10 `Com` : Kommunikation ................................................................................................................... 12
2.1.2.11 `Contact` : Ansprechpartner ........................................................................................................ 13
2.1.2.12 `Text` : Texte ................................................................................................................................. 13
2.1.2.13 `Reference` : Verweise ................................................................................................................ 15
2.1.2.14 `Pricing` : Preiskalkulation .......................................................................................................... 16
2.1.2.15 `Config` : Konfigurationsdaten ..................................................................................................... 21
2.1.2.16 `ConfigText` : Konfigurationstexte .............................................................................................. 22
2.1.2.17 `Payment` : Zahlungsbedingungen ............................................................................................... 22
2.1.2.18 `DocNo` : Belegnummern ............................................................................................................... 23
2.1.2.19 `BankData` : Bankdaten ................................................................................................................ 24
**2.1.3** **OEX-Werttypen (** **`Value`** **) ............................................................................................................ 26**
**2.1.4** **OEX-Attributtypen (** **`Empty`** **) ....................................................................................................... 28**

2.2 Datendomänen ................................................................................................. 29

2.3 Datentypen ....................................................................................................... 44

2.4 Attribute ............................................................................................................ 47


**3** **OEX – Szenarien ............................................................................................. 54**

3.1 Bestellung mit anschließender Bestelländerung (Idealfall) ............................... 54

3.2 Bestellung mit Bestelländerungen (zeitversetzt zur Bestellbestätigung) ........... 54

3.3 Bestellung mit Änderungen ausgelöst durch den Lieferanten ........................... 55

3.4 Von Anfrage bis Rechnung (Idealfall) ............................................................... 55

3.5 Von Anfrage bis Rechnung inklusive Bestelländerung (Idealfall) ...................... 56


                                  - 1 

**4** **Anhang ............................................................................................................ 57**

4.1 Änderungshistorie ............................................................................................. 57


                                  - 2 

### **1 Einleitung**

Der Austausch von Geschäftsdaten erfolgt über Textdateien, die mittels **XML** (Extensible Markup Language)
formattiert sind. Für verschiedene Arten von Geschäftsdaten (z.B. Bestellung) werden im Rahmen von OEX
spezifische Dokumentenarten definiert und deren jeweilige Struktur spezifiziert.

Diese Spezifikation enthält allgemeine Bestimmungen für die Übermittlung von OEX-Dokumenten und
dokumentenartübergreifende Definitionen von Datentypen und -strukturen.

Der Austausch von OEX-Dokumenten findet typischerweise per Email-Anhang zwischen den von beiden
Partnern vereinbarten Email-Adressen statt. Dabei ist es erlaubt/möglich, mehrere OEX-Dokumente oder
auch andere Anhänge wie bspw. PDF-Dateien zu senden, die dann über den Elementtyp `Reference`
(Verweise) mit der Verweisart `ATT` (Attachment) im jeweiligen OEX-Dokument referenziert werden (siehe
2.1.2.13).

XML Version und Code Page

```
<?xml version="1.0" encoding="UTF-8"?>

```

Als Standard Code Page wird **UTF-8** (Unicode Transformation Format) verwendet. Am Anfang der Datei
kann optional das Byte Order Mark angegeben werden.

Alternativ können beide Partner für den Austausch der Daten folgende Code Pages vereinbaren:
ISO-8859-1 (International Standardization Organization) – Latin-1: u.a. westeuropäischer Zeichensatz
ISO-8859-2 (International Standardization Organization) – Latin-2: u.a. mitteleuropäischer Zeichensatz

Diese Angaben erfolgen am Anfang eines XML-Dokuments.


XML Schema (XS) Einbindung

Struktur und Datentypen der XML-Dateien werden über XML-Schemata definiert und verifiziert.
Pro Dokumentenart gibt es jeweils ein Schema. Der Name eines Schemas setzt sich zusammen aus dem
Präfix `oex`, der Dokumentenart (bspw. `orders` für Bestellung), der Versionsnummer sowie der
Dateierweiterung `xsd` . Desweiteren ist in jedem dokumentenartbezogenen Schema das übergeordnete
Schema ( `global` ) eingebunden.

`oex-<DocumentType>_<Major>.<Minor>.<Build>.xsd` dokumentenartbezogenes Schema
`oex-global_<Major>.<Minor>.<Build>.xsd` übergeordnetes Schema

Die Einbindung des dokumentenartbezogenen Schemas erfolgt über die für XML-Schemata festgelegten
Attribute im Rahmenelement `oexDocFrame` :

```
<oexDocFrame aMajor="3"
xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xsi:noNamespaceSchemaLocation="oex-<DocumentType>_<Major>.<Minor>.<Build>.xsd">

```

                                  - 3 

**1.1** **Übersicht OEX-Spezifikationen**

Die OEX-GLOBAL Spezifikation ist auch das führende Dokument hinsichtlich der gültigen Versionen der
dokumentartbezogenen Spezifikationen.

|Dokumentenart|Bezeichnung|Name der Spezifikation|XML-Schema|Version|
|---|---|---|---|---|
|`GLOBAL`|übergeordnet|`oex-global_3.0.0.pdf`|`oex-global_3.0.x.xsd`|3.1|
|`REQOTE`|Anfrage|`oex-reqote_3.0.x.pdf`|`oex-reqote_3.0.x.xsd`|3.1|
|`QUOTES`|Angebot|`oex-quotes_3.0.x.pdf`|`oex-quotes_3.0.x.xsd`|3.1|
|`ORDERS`|Bestellungen|`oex-orders_3.0.x.pdf`|`oex-orders_3.0.x.xsd`|3.1|
|`ORDRSP`|Bestellbestätigung|`oex-ordrsp_3.0.x.pdf`|`oex-ordrsp_3.0.x.xsd`|3.1|
|`ORDCHG`|Bestelländerung|`oex-ordchg_3.0.x.pdf`|`oex-ordchg_3.0.x.xsd`|3.1|
|`DESADV`|Lieferavis|`oex-desadv_3.0.x.pdf`|`oex-desadv_3.0.x.xsd`|3.1|
|`INVOIC`|Rechnung|`oex-invoic_3.0.x.pdf`|`oex-invoic_3.0.x.xsd`|3.1|



Das „x“ steht für die jeweils höchste Build-Versionsnummer der jeweiligen Spezifikation bzw. des jeweiligen XML-Schemas.


**1.2** **Versionierung**


Die Versionsnummer aller Spezifikationen, sowie XML-Schemata und Beispieldateien besteht aus 3
Komponenten und setzt sich wie folgt zusammen:
Major **2** .3.17
Minor 2. **3** .17
Build 2.3. **17**

**Major- und Minor-Versionsnummer** sind allen OEX-Spezifikationen **gemein** .
Dabei gelten bei den Spezifikationen jeweils die mit der höchsten Build-Versionsnummer.
Ist beispielsweise die Version **2** . **3** .2 die Version der Spezifikation ORDERS (Bestellung) mit der höchsten
Build-Nummer innerhalb der Minor-Nummer **2** . **3**, muss die Spezifikation GLOBAL **2** . **3** .17 herangezogen
werden, wenn dies die Version der Spezifikation GLOBAL mit der höchsten Build-Nummer innerhalb der
Minor-Nummer **2** . **3** ist (s.a. Beispielkonstellation unten).

Über die Build-Nummer werden die unterschiedlichen Änderungsstände der Spezifikationen gesteuert, die
nicht unmittelbar immer auch alle anderen Spezifikationen betreffen. Eine Änderung von GLOBAL, die zwar
Auswirkungen auf die dokumentenartbezogenen Spezifikationen hat, nicht aber auf die dokumentenspezifischen Strukturen oder abgeleitete Elemente, wird über die Build-Nummer abgebildet. Hiervon bleibt
die Versionsnummer der dokumentenartbezogenen Spezifikation unbeeinflusst.
Auch umgekehrt beeinflusst eine Änderung in einer dokumentartbezogenen Spezifikation nicht die Versionsnummer von GLOBAL, wenn sie nicht Auswirkung auf die dort definierten Elemente und Strukturen hat.

Sobald sich eine Änderung von GLOBAL auf Elemente und Strukturen mindestens einer bestehenden
Dokumentenart auswirkt, müssen **alle** Spezifikationen auf die nächst höhere Minor-Versionsnummer
gezogen werden. Die Build-Versionsnummer wird dabei für alle wieder auf Null („0“) gesetzt.

Je nach Schwere und Umfang können Änderungen darüber hinaus die nächste Major-Versionsnummer zur
Folge haben, hierbei werden Minor- und Build-Versionsnummer für alle Spezifikationen wieder auf Null („0“)
gesetzt.

Die Major- und Minor-Versionsnummern der XML-Schemata (XSD) und der Beispieldateien (XML) sind
ebenfalls gleich, um auch hier eine eindeutige Zuordnung zu einer Version der entsprechenden
Dokumentenart zu gewährleisten. Änderungen in diesen Dateien bedingen eine neue Build-Nummer.
Innerhalb der XML-Datei wird angegeben, auf welche Version der entsprechenden Dokumentenart und auf
welches dazugehörige XML-Schema sich diese bezieht. Innerhalb der XSD-Datei wird angegeben, auf
welches übergeordnete XML-Schema sie sich bezieht. Auch hier gelten die jeweils höchsten BuildVersionsnummern.


                                  - 4 

Eine Beispielkonstellation der Versionen für ORDERS (Bestellung):
Spezifikationen OEX-ORDERS **2.3.2** oex-orders_2.3.2.pdf
Spezifikationen OEX-GLOBAL **2.3.17** oex-global_2.3.17.pdf
Beispieldatei ORDERS **2.3.5** oex-orders-sample_2.3.5.xml
XML-Schema ORDERS **2.3.4** oex-orders_2.3.4.xsd
XML-Schema GLOBAL **2.3.8** oex-global_2.3.8.xsd


**1.3** **Legende**


Erläuterung spezieller Spalten, die in den Tabellen im Abschnitt 2 “Definitionen“ Verwendung finden.











|Spalte|Bezeichnung|Werte|Col4|Bedeutung|
|---|---|---|---|---|
|**`Wdh`**|Wiederholbarkeit|**1 **|**1 **|Element kann genau einmal vorkommen.|
|**`Wdh`**|Wiederholbarkeit|**#+**|**#+**|Element muss mehrfach bis zu der Zahl<br>vorkommen, die über den Platzhalter #<br>angegeben wird, darüber hinaus kann es<br>mehrfach vorkommen.<br>(Bsp.: 1+ = 1 mal muss, mehrmals kann)|
|**`Wdh`**|Wiederholbarkeit|**#***|**#***|Element kann keinmal bzw. mehrfach bis zu der<br>Zahl vorkommen, die über den Platzhalter #<br>angegeben wird. Wenn das Element ein<br>Pflichtelement ist, muss es mind. einmal<br>vorkommen. (Bsp.: 3* = 1 bis 3 mal)|
|**`Wdh`**|Wiederholbarkeit|*** **|*** **|Element kann keinmal bis mehrfach vorkommen.<br>Wenn das Element ein Pflichtelement ist, muss<br>es mind. einmal vorkommen.|
|**`Pfl`**<br>**`Pflicht`**|Pflichtelement|**<empty>**|**<empty>**|Element kann vorhanden sein, muss aber dann<br>auch einen Wert beinhalten.|
|**`Pfl`**<br>**`Pflicht`**|Pflichtelement|**X **|**X **|Element muss vorhanden sein und einen Wert<br>beinhalten.|
|**`Pfl`**<br>**`Pflicht`**|Pflichtelement|**# **|**# **|Element kann vorhanden sein, muss dann aber<br>auch einen Wert beinhalten, der Platzhalter**#** gibt<br>eine fortlaufende Nummer beginnend bei 1<br>innerhalb eines Rahmenelements für<br>Unterelemente an, die einander bedingen und<br>i.d.R. gemeinsam angegeben werden müssen.<br>(bspw. Menge und Mengeneinheit)|
|**`Lng`**|(maximale) Länge der<br>Datendomäne<br>(Inklusive Dezimal-<br>stellen und Trenn-<br>zeichen). Vorzeichen<br>sind nicht Bestandteil<br>der Länge bei numer-<br>ischen Werten. (`NUM`)|**1 – n**|**1 – n**|1 bis “unendlich“|
|**`Lng`**|(maximale) Länge der<br>Datendomäne<br>(Inklusive Dezimal-<br>stellen und Trenn-<br>zeichen). Vorzeichen<br>sind nicht Bestandteil<br>der Länge bei numer-<br>ischen Werten. (`NUM`)|*** **|*** **|Beliebig (üblich in Bezug auf die Datendomäne)|
|**`Lng`**|(maximale) Länge der<br>Datendomäne<br>(Inklusive Dezimal-<br>stellen und Trenn-<br>zeichen). Vorzeichen<br>sind nicht Bestandteil<br>der Länge bei numer-<br>ischen Werten. (`NUM`)|**<empty>**|**<empty>**|Bei bestimmten Datentypen|
|**`Dez`**|Dezimalstellen|**1 – n**|**1 – n**|1 bis “unendlich“|
|**`Dez`**|Dezimalstellen|**<empty>**|**<empty>**|Keine Dezimalstellen|
|**`Trz`**|Dezimaltrennzeichen||**. **|Standardmäßig Dezimalpunkt|
|**`Trz`**|Dezimaltrennzeichen|**<empty>**|**<empty>**|Kein Dezimaltrennzeichen|
|**`Restrikt.`**|Restriktionen bei<br>Wertetabellen|||Siehe Datendomänen|


Andere:

OCD OFML Commercial Data


                                  - 5 

### **2 Definitionen**

**2.1** **Elementtypen**
Typisierung der verwendeten Elemente, wobei die grundlegenden XML-Elemente in Grundelementtypen und
die darauf basierenden OEX-Elemente in OEX-Elementtypen eingeteilt werden.
Jeder Typ ist einer Datendomäne (kurz: Domäne) zugeordnet, die den Typ eindeutig beschreibt.

Namensgebung: Mit einem Großbuchstaben beginnend.


**2.1.1** **Grundelementtypen**
Stellen eine Gruppierung der XML-Elemente dar und bilden die Basis für OEX-Elementtypen.


**2.1.1.1** **`Frame`** **: Rahmenelement**

|Grundelementtyp|Bezeichnung/Beschreibung|Col3|
|---|---|---|
|**`Frame`**|Rahmenelement, kann Attribute und Unterelemente enthalten.<br>Basisdomäne: _`Frame` <br>Namensgebung dieser Elemente: beliebiges 3-stelliges Präfix**`abc`** <br>Bsp.:**`<oexFileaDocCount=`**`"`**`5`**`"`**`>`**`[Unterelemente]`**`</oexFile>` **<br>|Rahmenelement, kann Attribute und Unterelemente enthalten.<br>Basisdomäne: _`Frame` <br>Namensgebung dieser Elemente: beliebiges 3-stelliges Präfix**`abc`** <br>Bsp.:**`<oexFileaDocCount=`**`"`**`5`**`"`**`>`**`[Unterelemente]`**`</oexFile>` **<br>|
|**`Frame`**|**Unterelemente**|**Bezeichnung**|
|**`Frame`**|`Frame`|Rahmenelement|
|**`Frame`**|`Value`|Wertelement|
|**`Frame`**|`Empty`|Attributelement (leeres Element)|



**2.1.1.2** **`Value`** **: Wertelement**

|Grundelementtyp|Bezeichnung/Beschreibung|
|---|---|
|**`Value`**|Wertelement, kann Attribute enthalten.<br>Basisdomäne: _`Value` <br>Namensgebung dieser Elemente: Präfix**`v`** (value)<br>Bsp.:**`<vDocumentTypeaMajor=`**`"`**`3`**`" aMinor="`**`0`**`" `<br>`aBuild="`**`0`**`"`**`>ORDERS</vDocumentType>`**|



**2.1.1.3** **`Empty`** **: Attributelement (leeres Element)**

|Grundelementtyp|Bezeichnung/Beschreibung|
|---|---|
|**`Empty`**|Leeres Element, enthält nur Attribute.<br>Basisdomäne: _`Attribute` <br>Namensgebung dieser Elemente: Präfix**`e`** (empty)<br>Bsp.:**<****`eAppVersion aMajor=`**`"`**`2`**`"`**` aMinor=`**`"`**`0`**`"`**`/>` **|



                                  - 6 

**2.1.2** **OEX-Rahmentypen (** **`Frame`** **)**

Alle Rahmenelemente basieren auf dem Grundtyp `Frame` .
Hinweis: Elemente, die in spitzen Klammern benannt sind, haben eine variable Namensgebung (bspw.
<Document>) und können variable Unterelemente <*> besitzen. Sie werden dokumentenartbezogen
definiert.


**2.1.2.1** **`DocFrame`** **: OEX Dokumentenrahmen**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`DocFrame`**|**`_DocFrame`**|||||**OEX Dokumentenrahmen**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<Applic>`**|**`Applic`**|**1 **|**X **|||**Applikation, die das Dokument**<br>**erstellt hat**|
|~~**`<File>`**~~|~~**`File`**~~|**1 **|**X **|||**Dokumentenmappe**|



DocFrame stellt das Hauptrahmenelement jedes OEX-XML-Dokuments dar.
Über dessen Attribute wird beispielsweise auch das zur Dokumentenart passende XML-Schema (XSD)
eingebunden.

Beispiel:

```
<oexDocFrame aMajor="3" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xsi:noNamespaceSchemaLocation="oex-orders_3.0.0.xsd">
    <oexApplication>
       <vAppName> MeinAuftragserfassungsprogramm </vAppName>
       <eAppVersion aMajor="7" aMinor="3"/>
    </oexApplication>
    <oexFile aDocumentCount="1">
       <vDocumentType aMajor="3" aMinor="0" aBuild="0"> ORDERS </vDocumentType>
       <... 1 Dokument ...>
    </oexFile>
</oexDocFrame>

```

**2.1.2.2** **`Applic`** **: Applikation, die das OEX-Dokument erstellt hat**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Applic`**|**`_Frame`**|||||**Applikation, die das Dokument**<br>**erstellt hat**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<AppName>`**|**`Value`**|**1 **|**X **|||**Applikationsname**|
|**`<AppVersion>`**|**`Version`**|**1 **|**X **|||**Version der Applikation**|



Dient zur Identifikation der Applikation, die das OEX-Dokument erstellt.

Beispiel:

```
<oexApplication>
    <vAppName> MeinAuftragserfassungsprogramm </vAppName>
    <eAppVersion aMajor="7" aMinor="3"/>
</oexApplication>

```

                                  - 7 

**2.1.2.3** **`File`** **: Dokumentenmappe**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`File`**|**`_File`**|||||**Dokumentenmappe**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<DocumentType>`**|**`DocumentType`**|**1 **|**X **|||**Dokumentenart**|
|~~**`<Document>`**~~|~~**`Document`**~~|**1+**|**X **|||**Einzelnes Dokument**|



Enthält eine Dokumentenmappe mehrere Dokumente ( `Document` ), können diese nur von der gleichen
Dokumentenart und Version sein. Eine Mischung von Dokumenten aus Dokumentenarten wie beispielsweise
`ORDERS` (Bestellung) und `ORDCHG` (Bestelländerung) ist somit nicht zulässig.

Beispiel:


Dokumentenmappe mit 4 Dokumenten

```
<oexFile aDocumentCount="4">
    <vDocumentType aMajor="3" aMinor="0" aBuild="0"> ORDERS </vDocumentType>
    <oexDocument aDocNo="1" aItemCount="5">
       <... Dokumenteninhalt von Dokument 1 (dokumentenartbezogen) ...>
    </oexDocument>
    <oexDocument aDocNo="2" aItemCount="2">
       <... Dokumenteninhalt von Dokument 1 (dokumentenartbezogen) ...>
    </oexDocument>
    <oexDocument aDocNo="3" aItemCount="1">
       <... Dokumenteninhalt von Dokument 1 (dokumentenartbezogen) ...>
    </oexDocument>
    <oexDocument aDocNo="4" aItemCount="3">
       <... Dokumenteninhalt von Dokument 1 (dokumentenartbezogen) ...>
    </oexDocument>
</oexFile>

```

**2.1.2.4** **`Document`** **: Einzelnes Dokument**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Document`**|**`_Document`**|||||**Einzelnes Dokument**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<*>`**|**`* `**|*** **||||**Dokumentenartbezogen**|



Beispiel:


2 Dokumente jeweils mit Belegkopf und unterschiedlicher Anzahl Belegpositionen

```
<oexDocument aDocNo="1" aItemCount="3" aAction="C">
    <docHeader aAction="C">
       <... Inhalt der Belegkopfes (dokumentenartbezogen) ...>
    </docHeader>
    <docItem aItemNo="1" aAction="C">
       <... Inhalt der Belegposition (dokumentenartbezogen) ...>
    </docItem>
    <docItem aItemNo="2" aAction="C">
       <... Inhalt der Belegposition (dokumentenartbezogen) ...>
    </docItem>
    <docItem aItemNo="3" aAction="C">
       <... Inhalt der Belegposition (dokumentenartbezogen) ...>
    </docItem>
</oexDocument>
<oexDocument aDocNo="2" aItemCount="1">
    <docHeader aAction="C">
       <... Inhalt der Belegkopfes (dokumentenartbezogen) ...>
    </docHeader>
    <docItem aItemNo="1" aAction="C">
       <... Inhalt der Belegposition (dokumentenartbezogen) ...>
</oexDocument>

```

                                  - 8 

**2.1.2.5** **`Header`** **: Belegkopf**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Header`**|**`_Header`**|||||**Belegkopf**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<*>`**|**`* `**|*** **||||**Dokumentenartbezogen**|



Beispiel:

```
<docHeader aAction="C">
    <... Inhalt der Belegkopfes (dokumentenartbezogen) ...>
</docHeader>

```

**2.1.2.6** **`Item`** **: Dokumentenposition**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Item`**|**`_Item`**|||||**Belegposition**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<*>`**|**`* `**|*** **||||**Dokumentenartbezogen**|



Beispiel:


2 Belegpositionen

```
<docItem aItemNo="1" aAction="C">
    <... Inhalt der Belegposition (dokumentenartbezogen) ...>
</docItem>
<docItem aItemNo="2" aAction="C">
    <... Inhalt der Belegposition (dokumentenartbezogen) ...>
</docItem>

```

**2.1.2.7** **`DateTime`** **: Datums- und Zeitangaben**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`DateTime`**|**`_DateTime`**|||||**Datums- und Zeitangaben**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<DateTimeType>`**|**`DateTimeType`**|**1 **|**X **|||**Typ Datum/Zeit**|
|**`<TimeZone>`**|**`TimeZone`**|**1 **|**X **|||**Zeitzone**|
|**`<DateValue>`**|**`Date`**|**1 **|**X **|||**Datumsangabe**|
|**`<TimeValue>`**|**`Time`**|**1 **||||**Zeitangabe**|



Datums- und Zeitangaben werden mit Bezug auf die jeweilige Zeitzone (Zeitdifferenz) angegeben.

Beispiele:


Dokumentendatum am 09.08.2006 um 14.35 Uhr Mitteleuropäischer Sommerzeit (MESZ) im Belegkopf

```
<hdrDateTime>
    <vDateTimeType> DOC </vDateTimeType>
    <vTimeZone> +0200 </vTimeZone>
    <vDateValue aDateFormat="D"> 20060809 </vDateValue>
    <vTimeValue> 143500 </vTimeValue>
</hdrDateTime>

```

                                  - 9 

Dokumentendatum am 22.12.2006 um 07.43 Uhr Westeuropäischer Winterzeit (WEZ) im Belegkopf

```
<hdrDateTime>
    <vDateTimeType> DOC </vDateTimeType>
    <vTimeZone> +0000 </vTimeZone>
    <vDateValue aDateFormat="D"> 20061222 </vDateValue>
    <vTimeValue> 074300 </vTimeValue>
</hdrDateTime>

```

Wunschlieferdatum Woche 8/2006 Mitteleuropäischer Winterzeit (MEZ) im Belegkopf

```
<hdrDateTime>
    <vDateTimeType> CRD </vDateTimeType>
    <vTimeZone> +0100 </vTimeZone>
    <vDateValue aDateFormat="W"> 200608 </vDateValue>
</hdrDateTime>

```

Bestelldatum am 28.10.2006 um 11.27 Uhr Winterzeit New York (EST) im Belegkopf

```
<hdrDateTime>
    <vDateTimeType> ORD </vDateTimeType>
    <vTimeZone> -0500 </vTimeZone>
    <vDateValue aDateFormat="D"> 20061028 </vDateValue>
    <vTimeValue> 112700 </vTimeValue>
</hdrDateTime>

```

Ermittlung des Wunschlieferdatums mit Angabe von 10 _Kalendertagen_ bei Bestelleingang

```
<hdrDateTime>
    <vDateTimeType> DLD </vDateTimeType>
    <vTimeZone> +0200 </vTimeZone>
    <vDateValue aDateFormat="C" aDateCalcBase="*DIO" aDateCalcMode="+"> 0010 </vDateValue>
</hdrDateTime>

```

Bei einem Bestelleingang am 01.07.2009 wäre der Wunschliefertermin der **11.07.2009**

|Juli 2009|Col2|Col3|Col4|Col5|Col6|Col7|Col8|
|---|---|---|---|---|---|---|---|
|**KW**|** Mo**|**Di**|**Mi**|**Do**|**Fr**|**Sa**|**So**|
|**27**|||**1 **|**2 **|**3 **|**4 **|**5 **|
|**28**|**6 **|**7 **|**8 **|**9 **|**10**|**11**|**12**|
|**29**|**13**|**14**|**15**|**16**|**17**|**18**|**19**|
|**30**|**20**|**21**|**22**|**23**|**24**|**25**|**26**|
|**31**|**27**|**28**|**29**|**30**|**31**|||



Ermittlung des Lieferdatums mit Angabe von 14 _Kalendertagen_ auf das Bestellbestätigungsdatum

```
<hdrDateTime>
    <vDateTimeType> COD </vDateTimeType>
    <vTimeZone> +0200 </vTimeZone>
    <vDateValue aDateFormat="D"> 20090701 </vDateValue>
</hdrDateTime>
<hdrDateTime>
    <vDateTimeType> CRD </vDateTimeType>
    <vTimeZone> +0200 </vTimeZone>
    <vDateValue aDateFormat="C" aDateCalcBase="COD" aDateCalcMode="+"> 0014 </vDateValue>
</hdrDateTime>

```

Die Kalulationsbasis bildet das vorangegangene Rahmenelement mit dem Bestellbestätigungsdatum 01.07.2009, damit wäre der
Liefertermin der **15.07.2009**


                               - 10 

|Juli 2009|Col2|Col3|Col4|Col5|Col6|Col7|Col8|
|---|---|---|---|---|---|---|---|
|**KW**|** Mo**|**Di**|**Mi**|**Do**|**Fr**|**Sa**|**So**|
|**27**|||**1 **|**2 **|**3 **|**4 **|**5 **|
|**28**|**6 **|**7 **|**8 **|**9 **|**10**|**11**|**12**|
|**29**|**13**|**14**|**15**|**16**|**17**|**18**|**19**|
|**30**|**20**|**21**|**22**|**23**|**24**|**25**|**26**|
|**31**|**27**|**28**|**29**|**30**|**31**|||


**2.1.2.8** **`OrgData`** **: Organisationsdaten**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`OrgData`**|**`_OrgData`**|||||**Organisationsdaten**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<OrgDataType>`**|**`OrgDataType`**|**1 **|**X **|||**Arten Organisationsdaten**|
|**`<OrgDataValue>`**|**`Value`**|**1 **|**X **|||**Wert Organisationsdaten**|



Mögliche Organisationsdaten für den Datenaustausch siehe Domäne `_OrgDataType` .

Beispiele:


Angabe einer Kommission im Belegkopf

```
<hdrOrgData>
    <vOrgDataType> COM </vOrgDataType>
    <vOrgDataValue> Kommission Schmidt </vOrgDataValue>
</hdrOrgData>

```

Angabe einer Projektnummer im Belegkopf

```
<hdrOrgData>
    <vOrgDataType> PJN </vOrgDataType>
    <vOrgDataValue> 65789198789 </vOrgDataValue>
</hdrOrgData>

```

Angabe einer aufbereiteten Positionsnummer in der Belegposition

```
<itmOrgData>
    <vOrgDataType> POS </vOrgDataType>
    <vOrgDataValue> 100.A.10-1 </vOrgDataValue>
</itmOrgData>

```

**2.1.2.9** **`Address`** **: Adressen**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Address`**|**`_Address`**|||||**Adressen**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<AddressType>`**|**`AddressType`**|**1 **|**X **|||**Typ Adresse**|
|**`<AddressNumber>`**|**`Value`**|**1 **||||**Adress-Nummer**|
|**`<AddressID>`**|**`AddressID`**|*** **||||**Adress-ID**|
|**`<Title>`**|**`Value`**|**1 **||||**Anrede**|
|**`<Name1>`**|**`Name1`**|**1 **|**X **|||**Name 1**|
|**`<Name2>`**|**`Name2`**|**1 **||||**Name 2**|
|**`<Name3>`**|**`Name3`**|**1 **||||**Name 3**|
|**`<Name4>`**|**`Name4`**|**1 **||||**Name 4**|
|**`<Street>`**|**`Street`**|**1 **|**X **|||**Straße**|
|**`<StreetNo>`**|**`Value`**|**1 **||||**Straßennummer**|
|**`<Street2>`**|**`Street2`**|**1 **||||**Straße 2**|
|**`<CountryCode>`**|**`CountryCode`**|**1 **|**X **|||**Länderkennzeichen**|
|**`<PostalCode>`**|**`PostalCode`**|**1 **|**X **|||**Postleitzahl**|
|**`<Location>`**|**`Location`**|**1 **|**X **|||**Ort**|
|**`<District>`**|**`District`**|**1 **||||**Ortsteil**|
|**`<CountyCode>`**|**`CountyCode`**|**1 **||||**Region/Bundesland/-Staat**|
|**`<PostalCodePOBox>`**|**`PostalCodePOB`**|**1 **||||**Postleitzahl Postfach**|



                               - 11 

|<POBox>|Value|1|Col4|Col5|Col6|Postfachnummer|
|---|---|---|---|---|---|---|
|**`<TaxCode>`**|**`Value`**|**1 **||||**Steuernummer Finanzamt**|
|**`<TaxCodeEU>`**|**`Value`**|**1 **||||**Steuernummer EU / USt-IdNr.**|
|**`<TaxCodeUSA>`**|**`Value`**|**1 **||||**Steuernummer USA / Jurisdiction**|
|~~**`<Com>`**~~|~~**`Com`**~~|*** **||||**Kommunikation**|
|~~**`<Contact>`**~~|~~**`Contact`**~~|*** **||||**Ansprechpartner**|


Beispiel:


Auftraggeberadresse

```
<hdrAddress>
    <vAddressType> SO </vAddressType>
    <vAddressNumber> 2222222 </vAddressNumber>
    <vTitle> Firma </vTitle>
    <vName1> Einrichtungshaus Dorfer </vName1>
    <vName2> Wohn- und Bürodesign </vName2>
    <vStreet> Haupstr. </vStreet>
    <vStreetNo> 11 </vStreetNo>
    <vCountryCode> DE </vCountryCode>
    <vPostalCode> 80001 </vPostalCode>
    <vLocation> München </vLocation>
    <vCountyCode> BY </vCountyCode>
    <vPostalCodePOBox> 456789 </vPostalCodePOBox>
    <vPOBox> 131343654 </vPOBox>
    <vTaxCodeEU> DE123456789 </vTaxCodeEU>
    <hdrCom>
       <vComType aScopeInfo="B"> TEL </vComType>
       <vComValue> +49-89-123456 </vComValue>
    </hdrCom>
    <hdrCom>
       <vComType aScopeInfo="B"> FAX </vComType>
       <vComValue> +49-89-123457 </vComValue>
    </hdrCom>
    <hdrCom>
       <vComType aScopeInfo="B"> WWW< /vComType>
       <vComValue> http://www.dorfer.de </vComValue>
    </hdrCom>
    <hdrContact>
       <vContactType> SC </vContactType>
       <vContactNumber> 333333 </vContactNumber>
       <vTitle> Herr </vTitle>
       <vFirstName> Joseph </vFirstName>
       <vLastName> Mayer </vLastName>
       <hdrCom>
           <vComType aScopeInfo="B"> TEL </vComType>
           <vComValue> +49-89-123456 </vComValue>
       </hdrCom>
       <hdrCom>
           <vComType aScopeInfo="B"> EMA </vComType>
           <vComValue> Joseph.Mayer@dorfer.de </vComValue>
       </hdrCom>
    </hdrContact>
</hdrAddress>

```

**2.1.2.10** **`Com`** **: Kommunikation**

**OEX-Elementtyp** **Domäne** **Bezeichnung**
**`Com`** **`_Frame`** **Kommunikation**

|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<ComType>`**|**`ComType`**|**1 **|**X **|||**Art der Kommunikation**|
|**`<ComValue>`**|**`Value`**|**1 **|**X **|||**Wert Kommunikation**|



                               - 12 

Beispiel:


Geschäftliche Telefon-Nummer im Belegkopf

```
<hdrCom>
    <vComType aScopeInfo="B"> TEL </vComType>
    <vComValue> 01234-5678910 </vComValue>
</hdrCom>

```

**2.1.2.11** **`Contact`** **: Ansprechpartner**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Contact`**|**`_Frame`**|||||**Ansprechpartner**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<ContactType>`**|**`ContactType`**|**1 **|**X **|||**Typ Ansprechpartner**|
|**`<ContactNumber>`**|**`Value`**|**1 **||||**Nummer Ansprechpartner**|
|**`<Title>`**|**`Value`**|**1 **||||**Anrede**|
|**`<FirstName>`**|**`FirstName`**|**1 **||||**Vorname**|
|**`<LastName>`**|**`LastName`**|**1 **|**X **|||**Nachname**|
|~~**`<Com>`**~~|~~**`Com`**~~|*** **||||**Kommunikation**|



Mittels des Typs Ansprechpartner können diverse Personen übermittelt werden, die direkt (z.B. Sachbearbeiter) oder organisatorisch (z.B. Vertriebsmitarbeiter) an einem Geschäftsfall beteiligt sind.
Die Nummer des Ansprechpartners kann hierzu auch als Identifizierungsmerkmal verwendet werden, muss
dann aber beiden Geschäftspartnern bekannt sein.


Beispiel:


Ansprechpartner Vertriebsmitarbeiter mit geschäftlicher Telefon-Nummer und Email-Adresse im Belegkopf

```
<hdrContact>
    <vContactType> SC </vContactType>
    <vContactNumber> 333333 </vContactNumber>
    <vTitle> Herr </vTitle>
    <vFirstName> Joseph </vFirstName>
    <vLastName> Mayer </vLastName>
    <hdrCom>
       <vComType aScopeInfo="B"> TEL </vComType>
       <vComValue> +49-89-123456 </vComValue>
    </hdrCom>
    <hdrCom>
       <vComType aScopeInfo="B"> EMA </vComType>
       <vComValue> Joseph.Mayer@dorfer.de </vComValue>
    </hdrCom>
</hdrContact>

```

**2.1.2.12** **`Text`** **: Texte**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Text`**|**`_Text`**|||||**Texte**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<TextType>`**|**`TextType`**|**1 **|**X **|||**Textart**|
|**`<TextLanguage>`**|**`TextLanguage`**|**1 **|**X **|||**Textsprache**|
|**`<TextContent>`**|**`TextContent`**|**1+**|**X **|||**Textinhalt**|



Die Textstruktur lehnt sich an OCD ab Version 4 an.


                               - 13 

Texte werden unformatiert in eine bzw. mehrere Textzeilen gestellt. Steuerzeichen für Zeilenumbrüche,
Tabulatoren, Zeichenformatierungen etc. sind nicht zulässig.
Die jeweilige Applikation muss beim Erstellen der XML-Textelemente sicherstellen, dass der Text
entsprechend der Vorgabe geschrieben wird.
Sind für eine Textart mehrere Zeilen zulässig, wird das Element `TextContent` im Rahmentyp `Text`
entsprechend wiederholt und dabei das Attribut `aTextLineNo` für die Zeilennummer hochgezählt.
Für jede neue Textart bzw. jede neue Sprache innerhalb einer Textart fängt die Zeilennummerierung wieder
bei 1 an (siehe Attribut `aTextLineNo` im Typ `TextContent` ).
Hinweis: Eine neue Sprache kann sich auch nur durch das Attribut `aLocale` (Gebietsschema)
unterscheiden. (siehe Beispiel mit Langtext in einer Sprache, aber 2 Gebietsschemata)

Ob beim Lesen der Zeilen aus den XML-Textelementen der Text in einer verarbeitenden Applikation dann
als einzelne Zeilen oder als Fließtext zusammengefügt eingestellt wird, wird über das Attribut Zeilenformat
gesteuert (siehe Attribut `aLineFormat` im Typ `TextContent` ).


Beispiel mit einem Artikellangtext ( `ARTL` ) für Fließtextdarstellung:

```
<itmText>
    <vTextType> ARTL </vTextType>
    <vTextLanguage> de </vTextLanguage>
    <vTextContent aTextLineNo="1" aLineFormat="\"> Büroschreibtisch XYZ, </vTextContent>
    <vTextContent aTextLineNo="2" aLineFormat="~"> höhenverstellbar, Untergestell verchromt. </vTextContent>
</itmText>

```

Ziel-Darstellung im Editor einer Auftragserfassung:
Büroschreibtisch XYZ, höhenverstellbar, Untergestell verchromt.

Anmerkung:
Durch die Applikation kann abhängig von der Länge des Feldes für den Textinhalt ein Zeilenumbruch
eingefügt werden.


Beispiel mit Langtext ( `ARTL` ) mit erzwungenem Zeilenumbruch in 2 Sprachen und einem Kurztext ( `ARTS` ):

```
<itmText>
    <vTextType> ARTL </vTextType>
    <vTextLanguage> de </vTextLanguage>
    <vTextContent aTextLineNo="1" aLineFormat="\"> Büroschreibtisch XYZ, </vTextContent>
    <vTextContent aTextLineNo="2" aLineFormat="\"> höhenverstellbar, Untergestell verchromt. </vTextContent>
</itmText>
<itmText>
    <vTextType> ARTL </vTextType>
    <vTextLanguage> en </vTextLanguage>
    <vTextContent aTextLineNo="1" aLineFormat="\"> Office desk XYZ, </vTextContent>
    <vTextContent aTextLineNo="2" aLineFormat="\"> height-adjustable, base chromed. </vTextContent>
</itmText>
<itmText>
    <vTextType> ARTS </vTextType>
    <vTextLanguage> de </vTextLanguage>
    <vTextContent aTextLineNo="1" aLineFormat="\"> Büroschreibtisch XYZ </vTextContent>
</itmText>

```

Ziel-Darstellung (des Langtextes) im Editor einer Auftragserfassung:
Büroschreibtisch XYZ,
höhenverstellbar, Untergestell verchromt.

Anmerkung:
Durch die Applikation kann abhängig von der Länge des Feldes für den Textinhalt ein zusätzlicher
Zeilenumbruch eingefügt werden.


                               - 14 

Beispiel mit Langtext ( `ARTL` ) in einer Sprache, aber 2 Gebietsschemata (= 2 Sprachversionen):
American English (enUS) und British English (enGB)

```
<itmText>
    <vTextType> ARTL </vTextType>
    <vTextLanguage aLocale="US"> en </vTextLanguage>
    <vTextContent aTextLineNo="1" aLineFormat="\"> Tension Strip color black </vTextContent>
</itmText>
<itmText>
    <vTextType> ARTL </vTextType>
    <vTextLanguage aLocale="GB"> en </vTextLanguage>
    <vTextContent aTextLineNo="1" aLineFormat="\"> Closing Ledge colour black </vTextContent>
</itmText>

```

deutscher Text zum Vergleich:

```
<itmText>
    <vTextType> ARTL </vTextType>
    <vTextLanguage> de </vTextLanguage>
    <vTextContent aTextLineNo="1" aLineFormat="\"> Spannleiste Farbe schwarz </vTextContent>
</itmText>

```

Anmerkung:
Die Anzahl der Textzeilen einer Textart können je nach Sprache bzw. Sprachversion unterschiedlich sein.


**2.1.2.13** **`Reference`** **: Verweise**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Reference`**|**`_Reference`**|||||**Verweise**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<ReferenceType>`**|**`ReferenceType`**|**1 **|**X **|||**Verweisart**|
|**`<ReferenceValue>`**|**`Value`**|**1 **|**X **|||**Wert Verweis**|
|**`<ReferenceDesc>`**|**`Value`**|**1 **|**X **|||**Bezeichnung**<br>_(in Belegsprache)_|



Verweise auf Links, Dateianhänge (Attachments) oder andere Dokumente. Je nach Verweisart wird für den
Wert des Verweises entsprechend eine vollständige URL (Link) oder ein vollständiger Dateiname
(Attachment) angegeben. Eine Bezeichnung ist Pflicht, da diese durch Applikationen für die Darstellung der
Links oder DropDown-Listen verwendet werden.

Beispiele:


Link zur Beauskunftung des Auftragsstatus im Belegkopf:

```
<hdrReference>
    <vReferenceType aMIMEType="text/html"> LNK </vReferenceType>
    <vReferenceValue> http://www.dorfer-einrichtung.de/orderstatus.html?p=1213131 </vReferenceValue>
    <vReferenceDesc> Auftragsstatus </vReferenceDesc>
</hdrReference>

```

Eingebettetes Produktbild:

```
<hdrReference>
    <vReferenceType aMIMEType="image/jpeg"> EDS </vReferenceType>
    <vReferenceValue> /9j/4AAQSkZJRgABAgAAZABkAAD/7AARRHVja3kAAQAEAAAAKQAA/+4ADkFkb2JlAGTAAAAAAf/bA
IQADAgICAkIDAkJDBELCQsRFA8MDA8UFxISFBISFxYRFBMTFBEWFhobHRsaFiMjJiYjIzIyMjIyODg4ODg4ODg4OAEMCwsMDgwPD
Q0PFA4ODhQUDxAQDxQcExMUExMcIxoWFhYWGiMgIh0dHSIgJiYjIyYmMDAuMDA4ODg4ODg4ODg4/8AAEQgAeAClAwEiAAIRAQMRA
f/EAI4AAQABBQEBAAAAAAAAAAAAAAABAgMEBQYHCAEBAQEBAAAAAAAAAAAAAAAAAAECAxAAAQMCAwMHBgsGBwEAAAAAAQACAxEEI
RIFMUEGUWFxgSIyE5GhwVIjB7HRQmJygpKissIzQ3OjFEQV8OHSU4PD4xYRAQEBAAMBAQEAAAAAAAAAAAABEVECEjEhQf/aAAwDA
QACEQMRAD8A9URFKAiIgIiICIiAiIghSoUoCIoQanivVTpHDl/qDTSWKIiEjb4r/Zx/ecF4llEFoyPeTQnlEYyV63ZivQ/e3qOS1
0/S2HGaV11KPmW7ey08znvHkXnV32XNirXw2hnWBifKtdfiVhzz5Oy3vHzBYhNcVU8lzyTyqlS0FKhEEoiIPpxERRRERAREQEREB
ERBBGKlEQFClWL27isrOe8mwhto3yyH5rGl58wQeRccX41HjO5oaw2AjtW02ezBuJfvnKuWuXlxe7ealZEdxLO24vpzWe6e6R55X
zuMjvMAsSTtAjlW/wCMsJERZUREQSiIg+nERFFEREBERAREQERUvkZG0ue4NaN5NAgqRau64i0+3qAXSkeqKDyuotTc8bhlfDttm
9z/AImrU6duE2OpXG+9TVf5Lhl1kx1J9TkbABv8MduR3RQBp6Vjze8O7ZXLbR0+sfSuJ4x1254g1mFswaxlozwxGyuUOJzSnEnGt
G/VTzZ9NjUOo2BjBv7RHTg37oCxyr8rszid27o3K0QtIw5WZXcx2KhZj4w4UKxnxPZuqOULNiqERFBKKEQfTqLRahxtw3YPdC66F
xcN2w2zTM7r8OrR1lauT3iNc6lppc8jfWmkji8wMhSde1+Q2OxULincd6ue7pcLR866/wDIIOOtY36dbHouqf8AWr47cHqO2Rccz
jrUadrSGu+hdtPwxhX4+OZf2uj3IHLG+KT87U8duD1HVIubHHFj+0sL+PnMId+CRyP410V4oXzw8z7aYecMKeO3FNjeXF02JpAxc
tFfXEkxJc4lWH8SaDJ/XMH0mSt/FGsd+s6I7ZfRHozn8i31mM2rM0WZa+e0qs2bV9KY0v8AFe9oxJZFJT7T2sb51zGrcbVJh0qIN
```

                               - 15 

```
PyrmUh7h+7YKsrzklb3ExTrVxFprQAQb14rEz1B/uv/ACjft2LmoAQ10h7z8BXbRD4tzI6SZxe55zSSOJLnE8pOJVwiuzADABYt1
VJVJCrolFFW8qZFcopDUFkwMd3mgqk2MTtlW9B+NZQaqg1MNYP9uNe/2ejFFscuCKZBiDMwZGvc1vqgn0UCZJHbS8jncaK0+5DcI
hX559AVh73v77i7p2eTYmjJMVvXtFlftH0qnJabOyehnxhWWjHsivRismKCd5wi63dlBUyzjk7kdfqgelX26NeEVitZX/umhx8jH
VWTaWzm4vkYwbwwZj5XUC29nf6fDI2OommOxsjw7ZjhGyjcOcKyRNc1IJ7VxbKbm3c0VIc2ZhA5Spbqdwzu6jMzkBmlb8Ll6Jp+t
XEkjGRnIxzgHBoDQcd4G1efX0ty6/vKSuDjeSsDNoAMrhsdUK2YS6kaxqXydTlP/MT+JSdT1Zwx1GYjmmI/CsW/lDZpDHDE5jSQA
WCvlZlK9Fh9z2nT2UEkt1Na3j4mOnjaGvjZIQC5rcwrQHDas23lcedzF0pzXM5lPLLIX/jJVsywNHY9qRubsXd3XuXum42eqRyHk
mgLPOx7vgWqufdPxdAfZNtrkbjHKWn+KxqmrjkXXE+fP3ad0DAAcyus1B477A7nGBW2uOBuMrckP0ud1N8ZZIPuOK182ia7D+tp1
0z6UD/9KfqDL23d3iWdIw8oV5hY8VY4OHMarXutbtvft5W9Mbx+VUeFMDURvB5Q1wPwK7TG1opAWDE/UhQNjkkHI5jj56VWyhiuJ
GB0sJid6pNetWVFACraFeZaSHcsy20q4lIDWE9SYMPIclUXT/8Ayd//ACRm8J1Kjd0org4MWrdrnYeRU57OI+u7mx/yWM+SSU1ec
OTcqK8mCxrTMOoubhHGGj5x9AVt1/dO2PDPogemqx0U2mK3SyyH2sj3g7RX0bFsNE8Bt08xntlmUZgA7KaZqU/xRaxVRvfG8SxnL
Iw5mnnCS/uj0LTHZXtPIQtHxHZfyfEU4ApFcysvYeds3ad5JGuCz9Ju2TxRzswbIK05DscOorZ8T2DtR0AX8Dc17pGaXKNr7Z1PG
b9QgPH1l1vzWZ9xynD1m2/4j021cMzJbqMvG4sYfFcPssK99XiXu2yzcY6e4YhrZ3/wXt/Mvbly7NRCKUUVCFrTtClEFl9pbv70Y
KsP0fTn96FpWaiu3kyNW7hvR3bbcKg8KaIf6cLbonq8pkapnDGjMNRAOtZkOnWUH6ULG9SyUTbyuRFEUooPmR4DXFoxpvVKkmuJ2
qEQUIiApBooUoN1w7d+HI+1ccD7SP8AOPSu90O/MUrXbRsIOwjeDzFeVxSvikZLH+pGczfi612mlX7JGRysPYeARzcy69L+YzeWx
0LQf7B7y7NkDSNLv47iWwdjQNMZc+Gp+VERTb3aFeqrkdDubW5ltjcislrIZbaTYWPcx0LupzHkEfEutWO0y41LqURFlRERAREQE
REBERAREQfMSIiIKERAUoiBVbLRr4wTeC4+zkNWczt460RXru/iV2ulX5Y5tCvQtF1Jt3AGOPtGjDnCIunfPP6nX62alEXJsREQE
REBERAREQEREH//2Q== </vReferenceValue>
    <vReferenceDesc> Produktbild </vReferenceDesc>
</hdrReference>

```

**2.1.2.14** **`Pricing`** **: Preiskalkulation**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Pricing`**|**`_Pricing`**|||||**Preiskalkulation**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<ConditionType>`**|**`ConditionType`**|**1 **|**X **|||**Konditionsart**|
|**`<ConditionValue>`**|**`Condition`**|**1 **|**X **|||**Konditionswert**|
|**`<ConditionRate>`**|**`ConditionRate`**|**1 **||||**Konditionssatz**|
|**`<CondCurrency>`**|**`CondCurrency`**|**1 **||||**Konditionswährung**<br>_Wenn nicht anders angegeben, wird_<br>_die Belegwährung angenommen._|
|**`<ConditionText>`**|**`ConditionText`**|**1 **||||**Konditionsbezeichnung**<br>Bezogen auf ihre Konditionsart und<br>gfs. der Art des Zu- bzw. Abschlags.<br>_(in Belegsprache)_<br>_Eine erneute Angabe des Konditions-_<br>_satzes (_`ConditionRate`_) in der_<br>_Bezeichnung ist nicht erlaubt._|
|**`<PriceUnit>`**|**`PriceUnit`**|**1 **||||**Preiseinheit**<br>Einheit, auf die sich der Konditionswert<br>(z.B. Einzelpreis) bezieht.<br>Beispiele:**1** bei Preis pro Stück oder<br>**10** bei Preis pro 10 Stück)<br>_Wenn nicht anders angegeben, wird 1_<br>_angenommen._<br>_Gilt nicht bei Summenkonditionen oder_<br>_wenn eine Konditionsart auf eine_<br>_Summenkondition Bezug nimmt._|
|**`<QuantUnit>`**|**`QuantUnit`**|**1 **||||**Mengeneinheit**<br>zur Preiseinheit <br>_Wenn nicht anders angegeben, wird_<br>_die Bestellmengeneinheit angenom-_<br>_men._<br>_Gilt nicht bei Summenkonditionen oder_<br>_wenn eine Konditionsart auf eine_<br>_Summenkondition Bezug nimmt._|



Die Angabe einer anderen Mengeneinheit für den Preis als der Bestellmengeneinheit setzt voraus, dass dem
empfangenden System die Umrechnungsregeln bekannt sind.


                               - 16 

Ebenso verhält es sich mit der Angabe einer anderen Währung anstatt der Belegwährung. Auch hier muss
das empfangende System in der Lage sein, den Wert mit dem entsprechenden Kurs umrechnen zu können.
Verschiedene Warenwirtschafts- bzw. ERP-Systeme erlauben auf Kopfebene so genannte Kopfrabatte
`"DISH"` (Abschläge) bzw. Kopfzuschläge `"SURH"` ohne, dass diese auf die Positionen heruntergebrochen
werden und sich dort als Rabatte widerspiegeln. Das hat zur Folge, dass Summen, die zuvor aus den
Positionen (bspw. `TNET` ) errechnet wurden, nicht mit der Endsumme `"TNEH"` nach Kopfabschlägen
und/oder Kopfzuschlägen übereinstimmen (vgl. auch folgendes Beispiel 1).
Ebenso verhält es sich mit der Mehrwertsteuer. Das Steuernetto ( `TTNE` ) muss auf Kopfebene entsprechend
der Kopfzu- und Abschläge berechnet werden.

Beispiel 1 – Komplettes Szenario für den Einkaufspreis einer Bestellung mit 2 Positionen:

Bestell-Position 1: Bestellposition 2:
Bruttoeinzelpreis (Listenpreis) € 50,00 Bruttoeinzelpreis (Listenpreis) € 20,00
Bestellmenge 2 Bestellmenge 1
Volle Mehrwerststeuer 19 % Reduzierte Mehrwertsteuer 7 %
Rabatt 1 (als Grundrabatt) 20 % Absoluter Rabatt (als Sonderrabatt) € 2,00
Rabatt 2 (als sonst. Rabatt 1) 5 % auf rabattierten Wert

Bestell-Kopf:
Kopfrabatt (als sonst. Rabatt 2) 10 %

```
<!-- Header /-->
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="P"> TGRO </vConditionType>
    <vConditionValue> 120.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="2">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 20.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Grundrabatt <vConditionText>
</hdrPricing>
<hdrPricing aCondNo="3">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="D1" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 4.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Ausstellungsrabatt <vConditionText>
</hdrPricing>
<hdrPricing aCondNo="4">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="SD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Sonderrabatt <vConditionText>
</hdrPricing>
<hdrPricing aCondNo="5">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 94.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="6">
    <vConditionType aCondArea="P" aCondRef="5" aTypeDis="D2" aCondSign="-"> DISH </vConditionType>
    <vConditionValue> 9.40 </vConditionValue>
    <vConditionRate> 10.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Aktionsrabatt <vConditionText>
</hdrPricing>
<hdrPricing aCondNo="7">
    <vConditionType aCondArea="P"> TNEH </vConditionType>
    <vConditionValue> 84.60 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="8">
    <vConditionType aCondArea="P" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 68.40 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>

```

                               - 17 

```
<hdrPricing aCondNo="9">
    <vConditionType aCondArea="P" aCondRef="8" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue> 13.00 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="10">
    <vConditionType aCondArea="P" aTaxCode="2"> TTNE </vConditionType>
    <vConditionValue> 16.20 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="11">
    <vConditionType aCondArea="P" aCondRef="10" aTaxCode="2"> TTAX </vConditionType>
    <vConditionValue aCondValType="P"> 1.13 </vConditionValue>
    <vConditionRate> 7.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
<hdrPricing>
<hdrPricing aCondNo="12">
    <vConditionType aCondArea="P"> TOTL </vConditionType>
    <vConditionValue> 98.73 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>

<!-- Pos 1 /-->
<vOrderQuantity>2</vOrderQuantity>
<itmPricing aCondNo="1">
    <vConditionType aCondArea="P"> SGRO </vConditionType>
    <vConditionValue> 50.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="2">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 10.00 </vConditionValue>
    <vConditionRate> 20.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Grundrabatt <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="3">
    <vConditionType aCondArea="P" aCondRef="2" aTypeDis="D1" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vConditionRate> 5.00 </vConditionRate>
    <vConditionText> Ausstellungsrabatt <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="P"> SNET </vConditionType>
    <vConditionValue> 38.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="5">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 76.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="6">
    <vConditionType aCondArea="P" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 76.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="7">
    <vConditionType aCondArea="P" aCondRef="6" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue aCondValType="P"> 14.44 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

                               - 18 

```
<itmPricing aCondNo="8">
    <vConditionType aCondArea="P"> TOTL </vConditionType>
    <vConditionValue> 90.44 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<!-- Pos 2 /-->
<vOrderQuantity>1</vOrderQuantity>
<itmPricing aCondNo="1">
    <vConditionType aCondArea="P"> SGRO </vConditionType>
    <vConditionValue> 20.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="2">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="SD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vConditionText> Sonderrabatt <vConditionText>
    <vPriceUnit> 1 . 000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="3">
    <vConditionType aCondArea="P"> SNET </vConditionType>
    <vConditionValue> 18.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 18.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="5">
    <vConditionType aCondArea="P" aTaxCode="2"> TTNE </vConditionType>
    <vConditionValue> 18.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="6">
    <vConditionType aCondArea="P" aCondRef="5" aTaxCode="2"> TTAX </vConditionType>
    <vConditionValue aCondValType="P"> 1.26 </vConditionValue>
    <vConditionRate> 7.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="7">
    <vConditionType aCondArea="P"> TOTL </vConditionType>
    <vConditionValue> 19.26 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

Beispiel 2 – Szenario für komplexere Rabattangaben einer Rechnungsposition:
Bruttoeinzelpreis der Position beträgt € 50,00
Rabatt 1 (als Grundrabatt) von 20% auf den Bruttoeinzelpreis
Rabatt 2 (als sonstiger Rabatt 1) von 5% auf den bereits rabattierten Preis aus Rabatt 1
Rabatt 3 (als Sonderrabatt) von 10% auf den resultierenden Wert aus den Rabatten 1 und 2
Rechnungsmenge = 2
Rechnungsmengeneinheit = C62

```
<itmPricing aCondNo="1">
    <vConditionType aCondArea="S"> SGRO </vConditionType>
    <vConditionValue> 50.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1 . 000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>

```

                               - 19 

```
<itmPricing aCondNo="2">
    <vConditionType aCondArea="S" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 10.00 </vConditionValue>
    <vConditionRate> 20.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
    <vConditionText> Grundrabatt <vConditionText>
</itmPricing>
<itmPricing aCondNo="3">
    <vConditionType aCondArea="S" aCondRef="2" aTypeDis="D1" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vConditionRate> 5.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Ausstellungsrabatt <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="S"> SUBI </vConditionType>
    <vConditionValue> 38.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Zwischensumme <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="5">
    <vConditionType aCondArea="S" aCondRef="4" aTypeDis="SD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 3.80 </vConditionValue>
    <vConditionRate> 10.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Sonderrabatt <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="6">
    <vConditionType aCondArea="S"> SNET </vConditionType>
    <vConditionValue> 34.20 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="7">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 68.40 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

                               - 20 

**2.1.2.15** **`Config`** **: Konfigurationsdaten**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Config`**|**`_Configuration`**|<br>||||**Konfigurationsdaten**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<ClassID>`**|**`Value`**|**1 **||||**Merkmalsklasse**|
|**`<OptionID>`**|**`Value`**|**1 **|**X **|||**Merkmal**|
|**`<OptionEAN>`**|**`EAN_Option`**|**1 **||||**EAN des Merkmals**|
|**`<ValueID>`**|**`Value`**|**1 **|**X **|||**Merkmalswert**|
|**`<ValueEAN>`**|**`EAN_Value`**|**1 **||||**EAN des Merkmalswertes**|
|~~**`<ConfigText>`**~~|~~**`ConfigText`**~~|*** **||||**Konfigurationstexte**|



Beispiel:
Konfiguration mit 5 Merkmalen inkl. Texte (de), Merkmal Y-LENGTH mit freier Werteingabe.

```
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> 10 </vOptionID>
    <vValueID> 2 </vValueID>
    <itmConfigText>
       <vTextLanguage> de </vTextLanguage>
       <vOptionText> Tischoberfläche </vOptionText>
       <vValueText aTextLineNo="1" aLineFormat="\"> Buche </vValueText>
    </itmConfigText>
</itmConfiguration>
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> Y-LENGTH </vOptionID>
    <vValueID> 200.00 </vValueID>
    <itmConfigText>
       <vTextLanguage> de </vTextLanguage>
       <vOptionText> Tischbreite (cm) </vOptionText>
    </itmConfigText>
</itmConfiguration>
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> XYZ </vOptionID>
    <vValueID> A </vValueID>
    <itmConfigText>
       <vTextLanguage> de </vTextLanguage>
       <vOptionText> Tischuntergestell </vOptionText>
       <vValueText aTextLineNo="1" aLineFormat="\"> verchromt </vValueText>
    </itmConfigText>
</itmConfiguration>
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> 1M </vOptionID>
    <vValueID> C22 </vValueID>
    <itmConfigText>
       <vTextLanguage> de </vTextLanguage>
       <vOptionText> Tischhöhe </vOptionText>
       <vValueText aTextLineNo="1" aLineFormat="\"> 72 cm </vValueText>
    </itmConfigText>
</itmConfiguration>
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> ZB50 </vOptionID>
    <vValueID> 4D </vValueID>
    <itmConfigText>
       <vTextLanguage> de </vTextLanguage>
       <vOptionText> Ausstattung </vOptionText>
       <vValueText aTextLineNo="1" aLineFormat="\"> Auszugs-Container links </vValueText>
       <vValueText aTextLineNo="2" aLineFormat="\"> PC-Container rechts </vValueText>
    </itmConfigText>
</itmConfiguration>

```

                               - 21 

**2.1.2.16** **`ConfigText`** **: Konfigurationstexte**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`ConfigText`**|**`_Frame`**|||||**Konfigurationstexte**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<TextLanguage>`**|**`TextLanguage`**|**1 **|**X **|||**Textsprache**|
|**`<OptionText>`**|**`OptionText`**|**1 **|**X **|||**Merkmalstext**|
|**`<ValueText>`**|**`ValueText`**|*** **||||**Merkmalswertetext**<br>Hier fällt der Text unter Umständen<br>weg, wenn es sich um einen frei<br>bewertbaren Merkmalswert handelt.|



Dieses Rahmenelement bildet die Konfigurationstexte in einer oder mehr Sprachen der vorangegangenen
Konfigurationsdaten ab ( `Config` ).

(Beispiel siehe Konfigurationsdaten)


**2.1.2.17** **`Payment`** **: Zahlungsbedingungen**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`Payment`**|**`_Frame`**|**3***||**! **||**Zahlungsbedingungen**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<PaymentPart>`**|**`PaymentPart`**|**1 **|**X **|**! **||**Bestandteil der Zahlungsbedingung**|
|**`<PaymentRate>`**|**`PaymentRate`**|**1 **|**X **|||**Skonto-Satz (%)**<br>0,00 bedeutet ohne Abzug (netto).|
|**`<PaymentDays>`**|**`PaymentDays`**|**1 **|**X **|||**Anzahl Tage (Zahlungsziel)**<br>Tage bedeuten hier Wochentage,<br>0 Tage bedeutet sofort fällig.|



Die Zahlungsbedingungen dienen zur reinen Beschreibung von Skonto-Angaben und/oder Nettozahlung in
Verbindung mit einem Zahlungsziel. Anderslautende Zahlungsbedingungen können ansonsten textuell über
das Kopftextsegment `hdrText` ( `TextType="PAYC"` ) angegeben werden.
Diese Angaben sind nur erforderlich, wenn sie abweichend von vertraglichen Vereinbarungen oder nicht
vereinbart sind.
Maximal werden derzeit 3 Bestandteile für die Zahlungsbedingung unterstützt.
Für die einzelnen Fälligkeiten wird Folgendes angenommen: Rechnungsdatum + Anzahl Tage (Zahlungsziel)


Beispiel 1 – Zahlungsbedingung mit einem Bestandteil:
10 Tage ohne Abzug netto

```
<hdrPayment>
    <vPaymentPart> 1 </vPaymentPart>
    <vPaymentRate> 0.00 </vPaymentRate>
    <vPaymentDays> 10 </vPaymentDays>
</hdrPayment>

```

                               - 22 

Beispiel 2 – Zahlungsbedingung mit zwei Bestandteilen:
14 Tage 2% Skonto, 30 Tage netto

```
<hdrPayment>
    <vPaymentPart> 1 </vPaymentPart>
    <vPaymentRate> 2.00 </vPaymentRate>
    <vPaymentDays> 14 </vPaymentDays>
</hdrPayment>
<hdrPayment>
    <vPaymentPart> 2 </vPaymentPart>
    <vPaymentRate> 0.00 </vPaymentRate>
    <vPaymentDays> 30 </vPaymentDays>
</hdrPayment>

```

Beispiel 3 – Zahlungsbedingung mit drei Bestandteilen:
5 Tage 3% Skonto, 10 Tage 2%, 30 Tage netto

```
<hdrPayment>
    <vPaymentPart> 1 </vPaymentPart>
    <vPaymentRate> 3.00 </vPaymentRate>
    <vPaymentDays> 5 </vPaymentDays>
</hdrPayment>
<hdrPayment>
    <vPaymentPart> 2 </vPaymentPart>
    <vPaymentRate> 2.00 </vPaymentRate>
    <vPaymentDays> 10 </vPaymentDays>
</hdrPayment>
<hdrPayment>
    <vPaymentPart> 3 </vPaymentPart>
    <vPaymentRate> 0.00 </vPaymentRate>
    <vPaymentDays> 30 </vPaymentDays>
</hdrPayment>

```

Beispiel 4 – Zahlungsbedingung mit einem Bestandteil:
Sofort netto, ohne Abzug

```
<hdrPayment>
    <vPaymentPart> 1 </vPaymentPart>
    <vPaymentRate> 0.00 </vPaymentRate>
    <vPaymentDays> 0 </vPaymentDays>
</hdrPayment>

```

**2.1.2.18** **`DocNo`** **: Belegnummern**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`DocNo`**|**`_Frame`**|||||**Belegnummern**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<DocNoType>`**|**`DocNoType`**|**1 **|**X **|||**Belegnummernart**|
|**`<DocNo>`**|**`DocNo`**|**1 **|**X **|||**Belegnummer**|
|**`<DocLine>`**|**`DocItemNo`**|**1 **||||**Nummer der Belegposition**|



Im Laufe eines Geschäftsfalls häufen sich diverse damit verbundene Dokumente an. Dies können Verweise
auf Vorgängerbelege in der Abfolge eines Geschäftsfalls (bspw. Angebot  Bestellung  Auftrag) oder
zusätzlicher Belege als Referenz (bspw. ein Verweis auf einen anderen Auftrag) sein. Um diese Angaben
dynamisch zu halten, wird dieses Rahmenelement verwendet. Das Rahmenelement kann als Unterelement
im Belegkopf ( `Header` ) als auch auf Positionsebene ( `Item` ) verwendet werden, wobei im Belegkopf das
Unterelement `<DocLine>` in der Regel entfällt.


                               - 23 

Beispiele:


Vorgängerbelegnummern (Abfolge) einer Rechnungsposition des Lieferanten

```
<itmDocNo>
    <vDocNoType aDocContext=" S "> QUO </vDocNoType> !Angebotsposition
    <vDocNo> AN10040 </vDocNo>
    <vDocLine> 2 </vDocLine>
</itmDocNo>
<itmDocNo>
    <vDocNoType aDocContext=" S "> ORD </vDocNoType> !Bestellposition
    <vDocNo> OR552244 </vDocNo>
    <vDocLine> 7 </vDocLine>
</itmDocNo>
<itmDocNo>
    <vDocNoType aDocContext=" S "> CNF </vDocNoType> !Bestellbestätigungsposition
    <vDocNo> AB20050 </vDocNo>
    <vDocLine> 7 </vDocLine>
</itmDocNo>
<itmDocNo>
    <vDocNoType aDocContext=" S "> TSP </vDocNoType> !Transportschein
    <vDocNo> TP30060 </vDocNo>
</itmDocNo>
<itmDocNo>
    <vDocNoType aDocContext=" S "> DEL </vDocNoType> !Lieferscheinposition
    <vDocNo> LS40070 </vDocNo>
    <vDocLine> 2 </vDocLine>
</itmDocNo>

```

Referenz im Belegkopf auf eine Bestellbestätigung (Auftrag) als zusätzliche Information bei einer
Reklamationsabwicklung

```
<hdrDocNo>
    <vDocNoType aDocContext=" R "> CNF </vDocNoType> !Referenz-Bestellbestätigungsnummer
    <vDocNo> AB20011 </vDocNo>
</hdrDocNo>

```

**2.1.2.19** **`BankData`** **: Bankdaten**

|OEX-Elementtyp|Domäne|Col3|Col4|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`BankData`**|**`_Frame`**|||||**Bankdaten**|


|Unterelement|Typ|Wdh|Pfl|Col5|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`<BankName>`**|**`BankName`**|**1 **|**X **|||**Name der Bank**|
|**`<BankCountry>`**|**`BankCountry`**|**1 **|**X **|||**Land der Bank**|
|**`<BankLocation>`**|**`BankLocation`**|**1 **|**X **|||**Sitz der Bank**|
|**`<SwiftBic>`**|**`SwiftBic`**|**1 **|**1 **|||**SWIFT-BIC Int. Bankenschlüssel**|
|**`<Iban>`**|**`Iban`**|**1 **|**1 **|||**IBAN Internationale Kontonummer**|
|**`<BankKey>`**|**`BankKey`**|**1 **|**2 **|||**Bankenschlüssel (Bankleitzahl)**|
|**`<BankAccount>`**|**`BankAccount`**|**1 **|**2 **|||**Kontonummer**|
|**`<AccountHolder>`**|**`AccountHolder`**|**1 **|**X **|||**Kontoinhaber**|



Erläuterungen zu Pflichtangaben:
**1 + 2** Es werden immer paarweise SWIFT-BIC und IBAN angegeben _oder_ Bankenschlüssel und
Kontonummer _oder_ beide Paare.


                               - 24 

Beispiele:
SWIFT-BIC und IBAN (Internationaler Bankverkehr)

```
<hdrBankData>
    <vBankName> UBS </vBankName>
    <vBankCountry> CH </vBankCountry>
    <vBankLocation> Zürich </vBankLocation>
    <vSwiftBic> BSWCHZH80A </vSwiftBic>
    <vIban> CH0288880003586482168 </vIban>
    <vAccountHolder> Gruezi AG </vAccountHolder>
</hdrBankData>

```

Bankleitzahl (BLZ) und Kontonummer (Nationaler Bankverkehr)

```
<hdrBankData>
    <vBankName> Deutsche Bank </vBankName>
    <vBankCountry> DE </vBankCountry>
    <vBankLocation> Berlin </vBankLocation>
    <vBankKey> 10070024 </vBankKey>
    <vBankAccount> 09572423341 </vBankAccount>
    <vAccountHolder> Schmidt GmbH </vAccountHolder>
</hdrBankData>

```

                               - 25 

**2.1.3** **OEX-Werttypen (** **`Value`** **)**


Alle Werteelemente basieren auf den Grundtyp `Value` .

|OEX-Elementtyp|Domäne|Bezeichnung|
|---|---|---|
|**`AccountHolder`**|**`_AccountHolder`**|**Kontoinhaber**|
|**`AddressID`**|**`_BusPartID`**|**Adress-ID**|
|**`AddressType`**|**`_AddressType`**|**Typ Adresse**|
|**`AddStateCode`**|**`_AddStateCode`**|**Zusätzliche Zustandsinformationen**|
|**`BankAccount`**|**`_BankAccount`**|**Kontonummer**|
|**`BankCountry`**|**`_Country`**|**Land der Bank**|
|**`BankKey`**|**`_BankKey`**|**Bankenschlüssel (Bankleitzahl)**|
|**`BankLocation`**|**`_Char35`**|**Sitz der Bank**|
|**`BankName`**|**`_Char35`**|**Name der Bank**|
|**`CatalogId`**|**`_CatalogId`**|**Katalog-ID**|
|**`Classification`**|**`_Classification`**|**Allgemeine Klassifizierung**|
|**`ClientClass`**|**`_BusPartClass`**|**Kunden-Klassifizierung**|
|**`ClientID`**|**`_BusPartID`**|**Kunden-ID**|
|**`CommodCode`**|**`_CommodCode`**|**Warennummer (INTRASTAT)**|
|**`CompSubArtId`**|**`_CompSubArtId`**|**Identifikation des Unterartikels**|
|**`ComType`**|**`_ComType`**|**Art der Kommunikation**|
|**`ConditionText`**|**`_Char35`**|**Konditionsbezeichnung**|
|**`ConditionType`**|**`_ConditionType`**|**Konditionsart**|
|**`ConditionRate`**|**`_ConditionRate`**|**Konditionssatz**|
|**`ConditionValue`**|**`_Condition`**|**Konditionswert**|
|**`CondCurrency`**|**`_Currency`**|**Konditionswährung**|
|**`ContactType`**|**`_ContactType`**|**Typ Ansprechpartner**|
|**`CountryCode`**|**`_CountryCode`**|**Länderkennzeichen**|
|**`CountryOrigin`**|**`_CountryCode`**|**Ursprungsland**|
|**`CountyCode`**|**`_CountyCode`**|**Region/Bundesland/-Staat**|
|**`CountyOrigin`**|**`_CountyCode`**|**Ursprungsregion**|
|**`CustomNumber`**|**`_Char35`**|**Zollnummer**|
|**`Date`**|**`_Date`**|**Datumsangabe**|
|**`DateTimeType`**|**`_DateTimeType`**|**Typ Datum/Zeit**|
|**`DelivComplet`**|**`_DelivComplet`**|**Vollständigkeit der Lieferung**|
|**`District`**|**`_Char35`**|**Ortsteil**|
|**`DocCurrency`**|**`_Currency`**|**Belegwährung**|
|**`DocLanguage`**|**`_Language`**|**Belegsprache**|
|**`DocNo`**|**`_Char35`**|**Belegnummer**|
|**`DocNoType`**|**`_DocNoType`**|**Belegnummernart**|
|**`DocItemNo`**|**`_PosNo`**|**Nummer der Belegposition**|
|**`DocumentType`**|**`_DocumentType`**|**Dokumentenart**|
|**`EAN_Article`**|**`_EAN`**|**EAN des Artikels**|
|**`EAN_Option`**|**`_EAN`**|**EAN des Merkmals**|
|**`EAN_Value`**|**`_EAN`**|**EAN des Merkmalswertes**|
|**`FirstName`**|**`_Char35`**|**Vorname**|
|**`FolderIsLOC`**|**`_YesNo`**|**Ist die Vezeichnung des Ordnerrs eine Orstangabe?**|
|**`GrossWeight`**|**`_Quantity`**|**Bruttogewicht**|
|**`Height`**|**`_Quantity`**|**Höhenangabe**|
|**`Iban`**|**`_Iban`**|**IBAN Internationale Kontonummer**|
|**`IncoTerm`**|**`_IncoTerm`**|**Inco Terms (Lieferbedingung)**|
|**`IncoTermLoc`**|**`_Char35`**|**Ortsangabe zu Inco Terms**|
|**`InvoiceType`**|**`_InvoiceType`**|**Rechnungsart**|



                               - 26 

|OEX-Elementtyp|Domäne|Bezeichnung|
|---|---|---|
|**`LastName`**|**`_Char35`**|**Nachname**|
|**`Length`**|**`_Quantity`**|**Längenangabe**|
|**`Location`**|**`_Char35`**|**Ort**|
|**`MeansTransp`**|**`_MeansTransp`**|**Förderhilfsmittel**|
|**`MeasureUnit`**|**`_Unit`**|**Maßeinheit**|
|**`Name1`**|**`_Char35`**|**Name 1**|
|**`Name2`**|**`_Char35`**|**Name 2**|
|**`Name3`**|**`_Char35`**|**Name 3**|
|**`Name4`**|**`_Char35`**|**Name 4**|
|**`NetWeight`**|**`_Quantity`**|**Nettogewicht**|
|**`NumPackages`**|**`_Integer`**|**Anzahl Packstücke**|
|**`NumArtPack`**|**`_Integer`**|**Anzahl Artikel pro Packstück**|
|**`OptionText`**|**`_Char80`**|**Merkmalstext**|
|**`OrderType`**|**`_OrderType`**|**Auftragsart**|
|**`OrgDataType`**|**`_OrgDataType`**|**Arten Organisationsdaten**|
|**`PackageNumber`**|**`_Char35`**|**Packstücknummer**|
|**`PackageType`**|**`_PackageType`**|**Verpackungsart**|
|**`PartDelivery`**|**`_YesNo`**|**Teillieferungen erlaubt?**|
|**`PaymentDays`**|**`_PaymentDays`**|**Anzahl Tage (Zahlungsziel)**|
|**`PaymentPart`**|**`_PaymentPart`**|**Bestandteil der Zahlungsbedingung**|
|**`PaymentRate`**|**`_PaymentRate`**|**Skonto-Satz (%)**|
|**`PostalCode`**|**`_PostalCode`**|**Postleitzahl**|
|**`PostalCodePOB`**|**`_PostalCode`**|**Postleitzahl Postfach**|
|**`PriceUnit`**|**`_Quantity`**|**Preiseinheit**|
|**`Quantity`**|**`_Quantity`**|**Mengenangabe**|
|**`QuantUnit`**|**`_Unit`**|**Mengeneinheit**|
|**`ReferenceType`**|**`_ReferenceType`**|**Verweisart**|
|**`ShipmentBase`**|**`_ShipmentBase`**|**Transportgrundlage**|
|**`Street`**|**`_Char35`**|**Straße**|
|**`Street2`**|**`_Char35`**|**Straße 2**|
|**`SupplierID`**|**`_BusPartID`**|**Lieferanten-ID**|
|**`SupplierClass`**|**`_BusPartClass`**|**Lieferanten-Klassifizierung**|
|**`SwiftBic`**|**`_SwiftBic`**|**SWIFT-BIC Int. Bankenschlüssel**|
|**`TextContent`**|**`_TextLine`**|**Textinhalt**|
|**`TextLanguage`**|**`_Language`**|**Textsprache**|
|**`TextLineNo`**|**`_LineNo`**|**Textzeilennummer**|
|**`TextType`**|**`_TextType`**|**Textart**|
|**`Time`**|**`_Time`**|**Zeitangabe**|
|**`TimeZone`**|**`_UTC`**|**Zeitzone**|
|**`TransportMode`**|**`_TransportMode`**|**Verkehrszweig**|
|**`UnitVolume`**|**`_Unit`**|**Volumeneinheit**|
|**`UnitWeight`**|**`_Unit`**|**Gewichtseinheit**|
|**`ValueText`**|**`_TextLine`**|**Merkmalswertetext**|
|**`VendorArtNo`**|**`_VendorArtNo`**|**Lieferantenartikelnummer**|
|**`VendorID`**|**`_VendorID`**|**Lieferantenkennung**|
|**`VendorSeries`**|**`_VendorSeries`**|**Lieferantenserie**|
|**`Volume`**|**`_Quantity`**|**Volumen**|
|**`Width`**|**`_Quantity`**|**Breitenangabe**|



                              - 27 

**2.1.4** **OEX-Attributtypen (** **`Empty`** **)**


Alle Attributelemente basieren auf den Grundtyp `Empty` .

|OEX-Elementtyp|Domäne|Bezeichnung|
|---|---|---|
|`AppVersion`|`_Version`|Version der Applikation|



                               - 28 

**2.2** **Datendomänen**

Namensgebung für Domänen: Präfix _ (Unterstrich) + Name beginnend mit einem Großbuchstaben.
Die Spalte “Restrikt.“ (Restriktion) unterscheidet innerhalb einer Wertetabelle, unter welchen Umständen
deren Werte zulässig sind. Der Datentyp wird unter 2.3 beschrieben, eventuelle Attribute unter 2.4.
Bei manchen Datendomänen wird ein Wert als gesetzt betrachtet, wenn der Wert “leer“ `<empty>` ist
und/oder das sich auf diese Datendomäne beziehende Element weggelassen `<skipped>` wird.

|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_AccountHolder`**|**`CHAR`**|**27**|||**Kontoinhaber**|
|**`_Address`**|**`FRAME`**<br>||<br>|<br>|**Adressen**|
|**`_Address`**|**Attribut** <br>|**Attribut** <br>|**Pflicht**|**Pflicht**||
|**`_Address`**|`aAction`|`aAction`|||Aktion|
|**`_AddressType`**|**`CHAR(UPPER)`**|**2 **|||**Adressarten**|
|**`_AddressType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_AddressType`**|`SO`|`SO`|||Auftraggeber|
|**`_AddressType`**|`SH`|`SH`|||Anlieferadresse|
|**`_AddressType`**|`IN`|`IN`|||Rechnungsempfänger|
|**`_AddressType`**|`PA`|`PA`|||Regulierer|
|**`_AddressType`**|`CA`|`CA`|||Spediteur|
|**`_AddressType`**|`SU`|`SU`|||Lieferant|
|**`_AddressType`**|`EU`|`EU`|||Endkunde|
|**`_AddressType`**|`IS`|`IS`|||Montagefirma|
|**`_AddressType`**|`IL`|`IL`|||Montageort|
|**`_AddressType`**|`BR`|`BR`|||Filiale (des Auftraggebers)|
|**`_AddStateCode`**|**`CHAR`**|*** **|||**Zusätzliche Zustandsinformationen**<br>Kodierung von Zuständen, die über den<br>kaufm. Variantencode hinaus zur<br>Wiederherstellung einer OFML-Instanz<br>benötigt wird (OFML Part III - spezifisch).|
|**`_AddStateCode`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_AddStateCode`**|`aAction`|`aAction`|||Aktion|
|**`_Attribute`**|**`ATTR`**||||**Attributelement**|
|**`_BankAccount`**|**`CHAR(NUPPER)`**|**20**|||**Bankkontonummer**<br>Nationale Kontonummer|
|**`_BankKey`**|**`CHAR(NUPPER)`**|**10**|||**Bankenschlüssel (Bankleitzahl)**<br>Nationaler Bankenschlüssel|
|**`_BusPartClass`**|**`CHAR`**|**20**|||**Geschäftspartner-Klassifizierung**|
|**`_BusPartClass`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_BusPartClass`**|`aBusPartClassType`|`aBusPartClassType`|X|X|Art der Geschäftspartner-Klassifizierung|
|**`_BusPartID`**|**`CHAR`**|**20**|||**Geschäftspartner-ID**|
|**`_BusPartID`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_BusPartID`**|`aBusPartIDType`|`aBusPartIDType`|X|X|Art der Geschäftspartner-ID|
|**`_CatalogId`**|**`CHAR(RX001)`**|*** **|||**Katalog-ID**<br>Eindeutiger Schlüssel eines Katalogprofils<br>Aufbau: <identifier>.<revision><br>(vgl. Spezifikation Katalogprofile)<br>Beispiel: de-2011.1|
|**`_CatalogId`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_CatalogId`**|`aAction`|`aAction`|||Aktion|
|**`_Char35`**|**`CHAR`**|**35**|||**Alphanumerischer Wert 35**|
|**`_Char80`**|**`CHAR`**|**80**|||**Alphanumerischer Wert 80**|



                               - 29 

|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_Classification`**|**`CHAR`**|*** **|||**Allgemeine Klassifizierung**|
|**`_Classification`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Classification`**|`aAction`|`aAction`|||Aktion|
|**`_Classification`**|`aClassSystem`|`aClassSystem`|X|X|Klassifikationssystem|
|**`_ClientArtNo`**|**`CHAR`**|*** **|||**Kundenartikelnummer**|
|**`_ClientArtNo`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_ClientArtNo`**|`aAction`|`aAction`|||Aktion|
|**`_CommodCode`**|**`NUM(NOSIGN)`**|**8 **|||**Warennummer (INTRASTAT)**<br>Statistische Warennummer definiert im<br>Warenverzeichnis für<br>Außenhandelsstatistik.|
|**`_CompSubArtId`**|**`CHAR`**|*** **|||**Identifikation des Unterartikels**<br>Die ID wird durch den übergeordneten<br>kompositen Artikel vergeben.<br>(OFML Part III - spezifisch)|
|**`_CompSubArtId`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_CompSubArtId`**|`aAction`|`aAction`|||Aktion|
|**`_ComType`**|**`CHAR(UPPER)`**|**3 **|||**Kommunikationsarten**|
|**`_ComType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_ComType`**|`TEL`|`TEL`|||Telefon-Nummer|
|**`_ComType`**|`FAX`|`FAX`|||Telefax-Nummer|
|**`_ComType`**|`MOB`|`MOB`|||Mobil-Nummer|
|**`_ComType`**|`WWW`|`WWW`|||Web-Seite|
|**`_ComType`**|`EMA`|`EMA`|||Email-Adresse|
|**`_ComType`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_ComType`**|`aScopeInfo`|`aScopeInfo`|X|X|Anwendungsbereich der Information|
|**`_Condition`**|**`NUM(NOSIGN)`**|*** **|**2 **|**. **|**Konditionswert** (absolut)<br>Preis, Rabattwert, Steuerwert etc.; wird<br>durch die Konditionsart (`_ConditionType`) <br>definiert.|
|**`_ConditionRate`**|**`NUM(NOSIGN)`**|*** **|**2 **|**. **|**Konditionssatz** (prozentual)<br>Rabattsatz, Steuersatz etc.; wird durch die<br>Konditionsart (`_ConditionType`) definiert.|
|**`_ConditionType`**<br>(Forts. auf nächster Seite)|**`CHAR(UPPER)`**|**4 **|||**Konditionsarten**<br>Legt die Art bzw. Verwendung eines<br>Konditionswerts (`_Condition`) bzw.<br>Konditionssatzes (`_ConditionRate`) fest.<br>Angaben wie Brutto und Netto beziehen<br>sich hier nicht auf die Mehrwertsteuer.|
|**`_ConditionType`**<br>(Forts. auf nächster Seite)|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_ConditionType`**<br>(Forts. auf nächster Seite)|`ECFR`|`ECFR`|`A `|`A `|ECO-Tax (Contribution) Frankreich|
|**`_ConditionType`**<br>(Forts. auf nächster Seite)|`SNET`|`SNET`|`ITM`<br>`A `|`ITM`<br>`A `|Nettoeinzelpreis<br>_Einzelpreise sind ggf. bereits eine Summe_<br>_aus mehreren Preisen, die sich auf Grund_<br>_eines konfigurierbaren Produktes ergeben_<br>_haben, jedoch nicht separat gespeichert_<br>_oder ausgewiesen werden._|
|**`_ConditionType`**<br>(Forts. auf nächster Seite)|`SGRO`|`SGRO`|`ITM`<br>`A `|`ITM`<br>`A `|Bruttoeinzelpreis<br>_(zu- und abschlagsfähig)_<br>_Einzelpreise sind ggf. bereits eine Summe_<br>_aus mehreren Preisen, die sich auf Grund_<br>_eines konfigurierbaren Produktes ergeben_<br>_haben, jedoch nicht separat gespeichert_<br>_oder ausgewiesen werden._|



- 30 



|Domäne|Datentyp|Lng|Dez|
|---|---|---|---|
|**`ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|**Wertetabelle**|**Restrikt.**||
|**`ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`TNEH`|`HDR`<br>`A `|Gesamtnetto Kopfebene<br>_Nach Abschlägen und/oder Zuschlägen auf_<br>_Kopfebene. (__`DISH`, __`SURH`) Sind diese nicht_<br>_angegeben, kann diese Konditionsart_<br>_entfallen, sie ist dann identisch mit der_<br>_Konditionsart Gesamtnetto (__`TNET`) auf_<br>_Kopfebene._|
|**`ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`TNET`|`A `|Gesamtnetto|
|**`ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`TGRO`|`A `|Gesamtbrutto|
|**`ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`TOTL`|`A `|Endbetrag<br>_Gesamtbetrag inkl. Mehrwertsteuer_|
|**`ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DISH`|`HDR`<br>`CR`<br>`TD`<br>`- `|Rabatt auf Kopfebene<br>_Wird vom Gesamtnettowert (__`TNET`) des_<br>_Kopfes gerechnet, danach sind weitere_<br>_Kombinationen oder Staffeln mit den_<br>_Konditionsarten__`DISH` and__`SURH` möglich._<br>_Soll ein Rabatt als Absolutrabatt angegeben_<br>_werden, so gilt Restriktion “A“._|
|**`ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DISI`|`CR`<br>`TD`<br>`- `|Rabatt auf Positionsebene<br>_Rabatte werden vom Bruttowert gerechnet._<br>_Die entsprechende Konditionsart des_<br>_Bruttowertes wird als Bezugskondition_<br>_angegeben. Weitere Rabatte können auch_<br>_vom bereits rabattierten Wert gerechnet_<br>_werden, hierbei wird die entsprechende_<br>_Konditionsart als Bezugskondition angeben._<br>_Auch eine Kombination mit Zuschlägen ist_<br>_möglich._<br>_Soll ein Rabatt als Absolutrabatt angeben_<br>_werden, so gilt Restriktion “A“._<br>_Auf Kopfebene stellt diese Konditionsart die_<br>_Summe aller Rabatte der Positionen dar_<br>_unter Berücksichtigung der Art des_<br>_Abschlags, hierbei wird kein Prozentsatz_<br>_angegeben. (Restriktion “A“)_|
|**`ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`SURH`|`HDR`<br>`CR`<br>`TS`<br>`+ `|Zuschlag auf Kopfebene<br>_Wird auf den Gesamtnettowert (__`TNET`) des_<br>_Kopfes gerechnet, danach sind weitere_<br>_Kombinationen oder Staffeln mit den_<br>_Konditionsarten__`DISH` and__`SURH` möglich._<br>_Soll ein Zuschlag als absoluter Zuschlag_<br>_angegeben werden, so gilt Restriktion “A“._|



- 31 



|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`SURI`|`SURI`|`CR`<br>`TS`<br>`+ `|`CR`<br>`TS`<br>`+ `|Zuschlag auf Positionsebene<br>_Zuschläge werden auf den Bruttowert_<br>_gerechnet. Weitere Zuschläge können aber_<br>_auch auf einen bereits zugeschlagenen_<br>_Wert gerechnet werden. In beiden Fällen_<br>_wird analog dem Rabatt jeweils die Bezugs-_<br>_kondition angegeben. Soll ein Zuschlag als_<br>_absoluter Zuschlag angeben werden, so gilt_<br>_Restriktion “A“._<br>_Auf Kopfebene stellt diese Konditionsart die_<br>_Summe aller Zuschläge der Positionen dar_<br>_unter Berücksichtigung der Art des_<br>_Zuschlags, hierbei wird kein Prozentsatz_<br>_angegeben. (Restriktion “A“)_|
|**`_ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`SUBH`|`SUBH`|`HDR`<br>`A `<br>|`HDR`<br>`A `<br>|Zwischensumme auf Kopfebene.<br>_Dient als Bezugskondition für darauf_<br>_folgende Rabatte bzw. Zuschläge (__`DISH`, _<br>_`SURH`). Alle vorhergehenden Rabatte bzw._<br>_Zuschläge werden mit ihren Bezugskondi-_<br>_tionen verrechnet und bilden die jeweilige_<br>_Zwischensumme._<br>_Alle nachfolgenden Rabatte oder Zuschläge_<br>_dürfen sich nicht auf Konditionen vor dieser_<br>_Zwischensumme beziehen. Die Angabe_<br>_mehrerer Zwischensummen vom Typ__`SUBH` _<br>_ist erlaubt, jedoch nicht direkt aufeinander_<br>_folgend._|
|**`_ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`SUBI`|`SUBI`|`A `<br>|`A `<br>|Zwischensumme auf Positions- und/oder<br>Kopfebene.<br>_Dient als Bezugskondition für darauf_<br>_folgende Rabatte bzw. Zuschläge (__`DISI`, _<br>_`SURI`). Alle vorhergehenden Rabatte bzw._<br>_Zuschläge werden mit ihren Bezugskondi-_<br>_tionen verrechnet und bilden die jeweilige_<br>_Zwischensumme._<br>_Alle nachfolgenden Rabatte oder Zuschläge_<br>_dürfen sich nicht auf Konditionen vor dieser_<br>_Zwischensumme beziehen. Die Angabe_<br>_mehrerer Zwischensummen vom Typ__`SUBI` _<br>_ist erlaubt, jedoch nicht direkt aufeinander_<br>_folgend._|
|**`_ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`TTNE`|`TTNE`|`A `<br>`TAX`|`A `<br>`TAX`|Steuernetto<br>_Unter Berücksichtigung des Steuerkenn-_<br>_zeichens wird diese Konditionsart auf_<br>_Kopfebene aufsummiert._|
|**`_ConditionType`**<br>(Forts., Forts. auf nächster<br>Seite)|`TTAX`|`TTAX`|`CR`<br>`P `<br>`TAX`|`CR`<br>`P `<br>`TAX`|Steuersatz<br>_Einem Steuerkennzeichen ist innerhalb_<br>_eines Belegs immer genau ein Steuersatz_<br>_zugeordnet._|



- 32 



|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`ConditionType`**<br>(Fortsetzung)|**Restriktionen**|**Restriktionen**|||**Verwendung**|
|**`ConditionType`**<br>(Fortsetzung)|`ITM`|`ITM`|||Nur bei Belegpositionen|
|**`ConditionType`**<br>(Fortsetzung)|`HDR`|`HDR`|||Nur bei Belegkopf|
|**`ConditionType`**<br>(Fortsetzung)|`A `|`A `|||Nur absolute Kondition<br>_`_Condition` enthält den Konditionswert._<br>_`_ConditionRate` entfällt._|
|**`ConditionType`**<br>(Fortsetzung)|`P `|`P `|||Nur prozentuale Kondition<br>_`_Condition` enthält den Wert auf Basis_<br>_des Prozentsatzes._<br>_`_ConditionRate` enthält den Prozentsatz._|
|**`ConditionType`**<br>(Fortsetzung)|`CR`|`CR`|||Angabe der Bezugskondition erforderlich|
|**`ConditionType`**<br>(Fortsetzung)|`TAX`|`TAX`|||Angabe Steuerkennzeichen erforderlich|
|**`ConditionType`**<br>(Fortsetzung)|`TS`|`TS`|||Art des Zuschlags erforderlich|
|**`ConditionType`**<br>(Fortsetzung)|`TD`|`TD`|||Art des Abschlags erforderlich|
|**`ConditionType`**<br>(Fortsetzung)|`+ `|`+ `|||Zuschlag (`aCondSign="+"`)|
|**`ConditionType`**<br>(Fortsetzung)|`- `|`- `|||Abschlag (`aCondSign="-"`)|
|**`ConditionType`**<br>(Fortsetzung)|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`ConditionType`**<br>(Fortsetzung)|`aCondArea`|`aCondArea`|X|X|Konditionsbereich|
|**`ConditionType`**<br>(Fortsetzung)|`aCondRef`|`aCondRef`|||Konditionsbezug (Berechnungsbasis)|
|**`ConditionType`**<br>(Fortsetzung)|`aTaxCode`|`aTaxCode`|||Steuerkennzeichen|
|**`ConditionType`**<br>(Fortsetzung)|`aTypeDis`|`aTypeDis`|||Art des Abschlags|
|**`ConditionType`**<br>(Fortsetzung)|`aTypeSur`|`aTypeSur`|||Art des Zuschlags|
|**`ConditionType`**<br>(Fortsetzung)|`aCondSign`|`aCondSign`|||Kennzeichen Zu- bzw. Abschlag|
|**`_Configuration`**|**`FRAME`**||||**Merkmal der Konfiguration**<br>_Ist das Attribut_`aMustCheck`_ nicht ange-_<br>_geben oder leer, wird der Wert_`Y`_ (ja)_<br>_angenommen._|
|**`_Configuration`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Configuration`**|`aIsVisible`|`aIsVisible`|X|X|Ist sichtbar?|
|**`_Configuration`**|`aMustCheck`|`aMustCheck`|||Ist prüfrelevant?|
|**`_Configuration`**|`aAction`|`aAction`|||Aktion|
|**`_ContactType`**|**`CHAR(UPPER)`**|**2 **|||**Kontaktarten**|
|**`_ContactType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_ContactType`**|`SC`|`SC`|||Ansprechpartner Vertrieb|
|**`_ContactType`**|`WC`|`WC`|||Ansprechpartner Lager|
|**`_ContactType`**|`IN`|`IN`|||Monteur|
|**`_ContactType`**|`EM`|`EM`|||Mitarbeiter|
|**`_ContactType`**|`CL`|`CL`|||Kunde|
|**`_ContactType`**|`SU`|`SU`|||Sachbearbeiter (Support)|
|**`_CountryCode`**|**`CHAR(UPPER)`**|**2 **|||**Länderschlüssel nach ISO 3166-1**<br>Beispiele:<br>`DE` <br>Deutschland<br>`ES` <br>Spanien<br>`GB` <br>Großbritannien<br>`FR` <br>Frankreich|
|**`_CountyCode`**|**`CHAR`**|**6 **|||**Bundesländer/-staaten nach ISO 3166-2**<br>Angegeben wird nur der 2. Teil. Der 1. Teil<br>entspricht dem Länderschlüssel nach ISO<br>3166-1 (`_CountryCode`).<br>Beispiele für das Land`DE` (Deutschland):<br>`BW` <br>Baden-Württemberg<br>`BY` <br>Bayern<br>`NW` <br>Nordrhein-Westfalen<br>`TH` <br>Thüringen|



                               - 33 

|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_Currency`**|**`CHAR(UPPER)`**|**3 **|||**Währungsschlüssel nach ISO 4217**<br>(für die derzeit gültigen Währungen)<br>Beispiele:<br>`EUR` Euro<br> <br>`GBP` Brit. Pfund<br>`CHF` Schweiz. Franken`USD` US Dollar|
|**`_Date`**|**`CHAR(DATE)`**|**8 **|||**Datum**|
|**`_Date`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Date`**|`aDateFormat`|`aDateFormat`|X|X|Datumsformat|
|**`_Date`**|`aDateCalcBase`|`aDateCalcBase`|1|1|Kalkulationsbasis bei Datumsermittlung|
|**`_Date`**|`aDateCalcMode`|`aDateCalcMode`|1|1|Kalkulationsverfahren bei Datumsermittlung|
|**`_DateTime`**|**`FRAME`**||||**Datums- und Zeitangaben**|
|**`_DateTime`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_DateTime`**|`aAction`|`aAction`|||Aktion|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|**`CHAR(UPPER)`**|**3 **|||**Typ Datum und Zeit**|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DOC`|`DOC`|`HDR`<br>`T `|`HDR`<br>`T `|Belegdatum<br>_Datum, wann der Beleg in die XML-Datei_<br>_geschrieben wurde._|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`CRD`|`CRD`|||Wunschliefertermin (Kunde)<br>_Für eine möglichst schnelle Lieferung kann_<br>_der Kunde bspw. einen kurzfristigen Termin_<br>_angeben, um dem Lieferanten zu signali-_<br>_sieren, seinen bestmöglichsten Termin zu_<br>_bestätigen. (unverbindlich)_|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DLD`|`DLD`|||Liefertermin (Lieferant)<br>_Unverbindlicher Liefertermin des_<br>_Lieferanten. Soll ein Fixtermin bestätigt_<br>_werden, wird stattdessen der Typ “__`FXD` –_<br>_Fixtermin“ verwendet._|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`EPD`|`EPD`|||Frühester Liefertermin<br>_Eine Lieferung vor diesem Termin ist nicht_<br>_zulässig. Kann zusammen mit__`LPD` _<br>_Spätester Liefertermin einen Lieferzeitraum_<br>_bilden._|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`LPD`|`LPD`|||Spätester Liefertermin<br>_Eine Lieferung nach diesem Termin ist nicht_<br>_zulässig. Kann zusammen mit__`EPD` _<br>_Frühester Liefertermin einen Lieferzeitraum_<br>_bilden._|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`FXD`|`FXD`|||Fixliefertermin|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`REQ`|`REQ`|`HDR`|`HDR`|Anfragedatum|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`QUO`|`QUO`|`HDR`|`HDR`|Angebotsdatum|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`QUV`|`QUV`|`HDR`|`HDR`|Angebotsgültigkeitsdatum<br>(Angebot gültig bis)|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`ORD`|`ORD`|`HDR`|`HDR`|Bestelldatum<br>_Datum, wann aus einem Bestellsystem_<br>_heraus bestellt wurde._|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`COD`|`COD`|`HDR`|`HDR`|Bestellbestätigungsdatum<br>_(Auftragsdatum)_|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DES`|`DES`|`HDR`|`HDR`|Lieferavisdatum|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DND`|`DND`|`HDR`|`HDR`|Lieferscheindatum|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`INV`|`INV`|`HDR`|`HDR`|Rechnungsdatum|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DUE`|`DUE`|||Fälligkeitsdatum|
|**`_DateTimeType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DSR`|`DSR`|`HDR`|`HDR`|Leistungserstellungsdatum|




- 34 

|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_DateTimeType`**<br>(Fortsetzung)|`PRD`|`PRD`|||Preisdatum<br>_Datum, mit dem die Preise aus einer zu_<br>_diesem Datum gültigen Preisliste kalkuliert_<br>_wurden. Hierzu korrespondiert die Angabe_<br>_der Preisliste in den Organisationsdaten._|
|**`_DateTimeType`**<br>(Fortsetzung)|**Restriktionen**|**Restriktionen**|||**Verwendung**|
|**`_DateTimeType`**<br>(Fortsetzung)|`ITM`|`ITM`|||Nur bei Belegpositionen|
|**`_DateTimeType`**<br>(Fortsetzung)|`HDR`|`HDR`|||Nur bei Belegkopf|
|**`_DateTimeType`**<br>(Fortsetzung)|`T `|`T `|||Angabe der Uhrzeit erforderlich|
|**`_DelivComplet`**|**`CHAR(UPPER)`**|**1 **|||**Vollständigkeit der Lieferung**<br>(bezogen auf eine Bestellung oder Bestell-<br>position)|
|**`_DelivComplet`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_DelivComplet`**|`E `|`E `|||Volllieferung|
|**`_DelivComplet`**|`P `|`P `|||Teillieferung|
|**`_Document`**|**`FRAME`**||||**Einzelnes Dokument**|
|**`_Document`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Document`**|`aDocNo`|`aDocNo`|X|X|Laufende Nummer des Dokuments|
|**`_Document`**|`aItemCount`|`aItemCount`|X|X|Anzahl Positionen im Dokument|
|**`_Document`**|`aAction`|`aAction`|X|X|Aktion|
|**`_DocFrame`**|**`FRAME`**||||**OEX Dokumentenrahmen**|
|**`_DocFrame`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_DocFrame`**|`aMajor`|`aMajor`|X|X|Major Versionsnummer|
|**`_DocFrame`**|`aTransferMode`|`aTransferMode`|||Transfer Modus der XML-Datei|
|**`_DocFrame`**|`<XSD>`|`<XSD>`|X|X|XML-Schema Einbindung (s. Abschn. 1)|
|**`_DocNoType`**<br>|**`CHAR(UPPER)`**|**3 **|||**Belegnummernart**|
|**`_DocNoType`**<br>|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_DocNoType`**<br>|`REQ`|`REQ`|||Anfragenummer|
|**`_DocNoType`**<br>|`QUO`|`QUO`|||Angebotsnummer|
|**`_DocNoType`**<br>|`ORD`|`ORD`|||Bestellnummer|
|**`_DocNoType`**<br>|`CHG`|`CHG`|||Bestelländerungsnummer|
|**`_DocNoType`**<br>|`CNF`|`CNF`|||Bestellbestätigungsnummer<br>_(Entspricht Auftragsnummer oder_<br>_Verkaufsbelegnummer aus Sicht des_<br>_Lieferanten)_|
|**`_DocNoType`**<br>|`DEL`|`DEL`|||Liefernummer<br>_(Lieferscheinnummer)_|
|**`_DocNoType`**<br>|`LOL`|`LOL`|||Ladelistennummer|
|**`_DocNoType`**<br>|`SHP`|`SHP`|||Transportnummer<br>_Ein Transport setzt sich aus einer bis_<br>_mehreren Lieferungen (__`DEL`) und/oder_<br>_Aufträgen (__`CNF`) zusammen._<br>_(siehe auch Lieferavis (__`DESADV`))_|
|**`_DocNoType`**<br>|`INV`|`INV`|||Rechnungsnummer|
|**`_DocNoType`**<br>|`TAN`|`TAN`|||Vorgangsnummer|
|**`_DocNoType`**<br>|`CON`|`CON`|||Rahmenvertragsnummer|
|**`_DocNoType`**<br>|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_DocNoType`**<br>|`aDocContext`|`aDocContext`|X|X|Dokumentenzusammenhang|




- 35 

|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_DocumentType`**|**`CHAR(UPPER)`**|**6 **|||**Dokumentenart**|
|**`_DocumentType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_DocumentType`**|`REQOTE`|`REQOTE`|||Anfrage|
|**`_DocumentType`**|`QUOTES`|`QUOTES`|||Angebot|
|**`_DocumentType`**|`ORDERS`|`ORDERS`|||Bestellung|
|**`_DocumentType`**|`ORDCHG`|`ORDCHG`|||Bestelländerung|
|**`_DocumentType`**|`ORDRSP`|`ORDRSP`|||Bestellbestätigung|
|**`_DocumentType`**|`DESADV`|`DESADV`|||Lieferavis|
|**`_DocumentType`**|`INVOIC`|`INVOIC`|||Rechnung|
|**`_DocumentType`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_DocumentType`**|`aMajor`|`aMajor`|X|X|Major Versionsnummer|
|**`_DocumentType`**|`aMinor`|`aMinor`|X|X|Minor Versionsnummer|
|**`_DocumentType`**|`aBuild`|`aBuild`|X|X|Build Versionsnummer|
|**`_EAN`**|**`CHAR`**|*** **|||**EAN-Nummer**<br>International (**E**uropean)**A**rticle**N**umber|
|**`_EAN`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_EAN`**|`aAction`|`aAction`|||Aktion|
|**`_EAN`**|`aEANType`|`aEANType`|X|X|EAN Typ|
|**`_File`**|**`FRAME`**||||**Dokumentenmappe**|
|**`_File`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_File`**|`aDocumentCount`|`aDocumentCount`|X|X|Anzahl der Dokumente in der Mappe.|
|**`_Frame`**|**`FRAME`**||||**Rahmenelement**|
|**`_Header`**|**`FRAME`**||||**Dokumentenkopf**|
|**`_Header`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Header`**|`aAction`|`aAction`|||Aktion|
|**`_Iban`**|**`CHAR(NUPPER)`**|**34**|||**IBAN Internationale Bankkontonummer**<br>**I**nternational**B**ank**A**ccount**N**umber<br>Nach ISO 13616:2003|
|**`_IncoTerm`**|**`CHAR(UPPER)`**|**3 **|||**Inco Terms nach Inco Terms 2000**<br>(International gültige Lieferbedingungen)|
|**`_IncoTerm`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_IncoTerm`**|`CFR`|`CFR`|`POD`|`POD`|Kosten und Fracht|
|**`_IncoTerm`**|`CIF`|`CIF`|`POD`|`POD`|Kosten, Versicherung und Fracht|
|**`_IncoTerm`**|`CIP`|`CIP`|`DST`|`DST`|Frachtfrei, versichert|
|**`_IncoTerm`**|`CPT`|`CPT`|`DST`|`DST`|Frachtfrei|
|**`_IncoTerm`**|`DAF`|`DAF`|`PLA`|`PLA`|Geliefert Grenze|
|**`_IncoTerm`**|`DDP`|`DDP`|`PLA`|`PLA`|Geliefert verzollt|
|**`_IncoTerm`**|`DDU`|`DDU`|`PLA`|`PLA`|Geliefert unverzollt|
|**`_IncoTerm`**|`DEQ`|`DEQ`|`POD`|`POD`|Geliefert ab Kai|
|**`_IncoTerm`**|`DES`|`DES`|`POD`|`POD`|Geliefert ab Schiff|
|**`_IncoTerm`**|`EXW`|`EXW`|`PLA`|`PLA`|Ab Werk|
|**`_IncoTerm`**|`FAS`|`FAS`|`POS`|`POS`|Frei Längsseite Seeschiff|
|**`_IncoTerm`**|`FCA`|`FCA`|`PLA`|`PLA`|Frei Frachtführer|
|**`_IncoTerm`**|`FOB`|`FOB`|`POS`|`POS`|Frei an Bord|
|**`_IncoTerm`**|**Restriktionen**|**Restriktionen**|||**Ortsangaben**|
|**`_IncoTerm`**|`PLA`|`PLA`|||Genannter Ort|
|**`_IncoTerm`**|`POS`|`POS`|||Genannter Verschiffungshafen|
|**`_IncoTerm`**|`POD`|`POD`|||Genannter Bestimmungshafen|
|**`_IncoTerm`**|`DST`|`DST`|||Genannter Bestimmungsort|



                               - 36 

|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_InvoiceType`**|**`CHAR(UPPER)`**|**2 **|||**Rechnungsart**|
|**`_InvoiceType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_InvoiceType`**|`IN`|`IN`|||Rechnung|
|**`_InvoiceType`**|`CN`|`CN`|||Gutschrift|
|**`_InvoiceType`**|`PI`|`PI`|||Proforma Rechnung|
|**`_Integer`**|**`NUM`**|*** **|||**Integerwert**|
|**`_Item`**|**`FRAME`**||||**Belegposition**<br>_Im Gegensatz zum Positionszähler im_<br>_Beleg (_`aItemNo`_), der Nummer der_<br>_Belegposition (_`DocItemNo`_) und sonstiger_<br>_Positionsnummernangaben_<br>`(_OrgDataType POS`_) darf sich die_<br>`aUUID`_ einer Belegposition in der Abfolge_<br>_der Belege zum Geschäftsfall nicht ändern!_|
|**`_Item`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Item`**|`aItemNo`|`aItemNo`|X|X|Positionszähler|
|**`_Item`**|`aAction`|`aAction`|||Aktion|
|**`_Item`**|`aUUID`|`aUUID`|||Global eindeutiger Identifikator|
|**`_Item`**|`aItemType`|`aItemType`|||Positionstyp|
|**`_Language`**|**`CHAR(LOWER)`**|**2 **|||**Sprachenschlüssel nach ISO 639-1**<br>Beispiele:<br>`de` <br>deutsch<br> <br>`en` <br>englisch<br>`fr` <br>französisch<br>`es` <br>spanisch<br> <br>Bei einer Unterscheidung der Sprache<br>nach einem Land (Sprachversion) wird das<br>Attribut für das Gebietsschema<br>angegeben.<br>Beispiel:<br>Sprache`en` (englisch) und Gebietsschema<br>`aLocale="US"` für American English.|
|**`_Language`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Language`**|`aLocale`|`aLocale`|||Gebietsschema|
|**`_LineNo`**|**`NUM(LIST1)`**|*** **|||**Zeilennummer**|
|**`_MeansTransp`**|**`CHAR(UPPER)`**|**3 **|||**Förderhilfsmittel**|
|**`_MeansTransp`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_MeansTransp`**|`PAL`|`PAL`|||Palette|
|**`_MeansTransp`**|`ROL`|`ROL`|||Rolle|
|**`_MeansTransp`**|`SAC`|`SAC`|||Sack|
|**`_MeansTransp`**|`LAB`|`LAB`|||Gitterbox|
|**`_MeansTransp`**|`CON`|`CON`|||Container|
|**`_MeansTransp`**|`BAR`|`BAR`|||Fass|



                               - 37 

|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_OrderType`**|**`CHAR(UPPER)`**|**2 **|||**Auftragsart**<br>Kann zur weiteren Bestimmung des<br>Auftrages herangezogen werden,<br>insbesondere bei der Bestellung<br>(ORDERS).|
|**`_OrderType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_OrderType`**|`SO`|`SO`|||Standardauftrag|
|**`_OrderType`**|`XO`|`XO`|||Expressauftrag<br>_Wird in aller Regel für vom Lieferanten_<br>_vorgegebene Artikel (bspw. im Rahmen_<br>_einer Verkaufsaktion oder vertraglicher_<br>_Vereinbarungen angeboten), die eine_<br>_schnellere Lieferzeit/Verfügbarkeit haben_<br>_als im Normalfall. Dabei wird das Auftrags-_<br>_volumen üblicherweise auch auf eine_<br>_bestimmte Stückzahl beschränkt._<br>_Ob und wie diese Form des Auftrages_<br>_abgwickelt wird, hängt schlussendlich vom_<br>_jeweiligen Hersteller ab._|
|**`_OrderType`**|`MU`|`MU`|||Bemusterung|
|**`_OrderType`**|`CP`|`CP`|||Reklamationsabwicklung<br>_Hierbei ist vorab eine entsprechende_<br>_Reklamationsanzeige beim Lieferanten_<br>_eingegangen und evtl. eine_<br>_Vorgangsnummer vergeben worden, die in_<br>_der Bestellung mitgegeben werden kann._<br>_(__`DocNo`)_|
|**`_OrderType`**|`SP`|`SP`|||Ersatzteilbestellung|
|**`_OrderType`**|`EO`|`EO`|||Bestellung für eigenen Mitarbeiter|
|**`_OrderType`**|`SR`|`SR`|||Bestellung für die eigene Ausstellung<br>(Showroom)|
|**`_OrgData`**|**`FRAME`**<br>||<br>|<br>|**Organisationsdaten**|
|**`_OrgData`**|**Attribut** <br>|**Attribut** <br>|**Pflicht**|**Pflicht**||
|**`_OrgData`**|`aAction`|`aAction`|||Aktion|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|**`CHAR(UPPER)`**|**3 **|||**Arten von Organisationsdaten**<br>_Hinweis: Die Möglichkeit diverse Daten in_<br>_ein Dokument stellen zu können, bedeutet_<br>_nicht zwingend, dass der Empfänger diese_<br>_Daten auch verarbeiten oder zurückliefern_<br>_kann._|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|`CCC`|`CCC`|||Kundenkostenstelle|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|`CNF`|`CNF`|`ITM`|`ITM`|Konfigurations-ID<br>_Bspw. als Hilfe für einen Konfigurator zur_<br>_Erkennung einer von ihm generierten_<br>_Konfiguration (Variantencode)._|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|`COG`|`COG`|`ITM`|`ITM`|Warengruppe|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|`COM`|`COM`|||Kommission|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DIC`|`DIC`|||Vertriebsweg|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DIV`|`DIV`|||Sparte|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DLO`|`DLO`|||Versandstelle|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|`DPL`|`DPL`|||Lieferndes Werk|
|**`_OrgDataType`**<br>(Forts., Forts. auf nächster<br>Seite)|`ITM`|`ITM`|`ITM`|`ITM`|Artikel-ID<br>_Bspw. für spezielle Endartikelnummern_|



                               - 38 

|Domäne|Datentyp|Lng|Dez|Trz|Col6|Bezeichnung|
|---|---|---|---|---|---|---|
|**`_OrgDataType`**<br>(Fortsetzung)|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Restrikt.**||
|**`_OrgDataType`**<br>(Fortsetzung)|`LOC`|`LOC`||||Ortsangabe, z.B. Abladestelle u.ä.<br>(Gebäude/Stockwerk/Raum)|
|**`_OrgDataType`**<br>(Fortsetzung)|`OVC`|`OVC`|`ITM`|`ITM`|`ITM`|OFML-Variantencode<br>_s.a. CNF_|
|**`_OrgDataType`**<br>(Fortsetzung)|`PGR`|`PGR`||||Einkäufergruppe|
|**`_OrgDataType`**<br>(Fortsetzung)|`PJN`|`PJN`||||Projektnummer|
|**`_OrgDataType`**<br>(Fortsetzung)|`PLO`|`PLO`||||Ladestelle|
|**`_OrgDataType`**<br>(Fortsetzung)|`POR`|`POR`||||Einkaufsorganisation|
|**`_OrgDataType`**<br>(Fortsetzung)|`POS`|`POS`|`ITM`|`ITM`|`ITM`|Positions-ID<br>_Zur Angabe einer speziell aufbereiteten_<br>_oder abweichenden Positionsnummer_<br>_gegenüber der eindeutigen_<br>_Positionsnummer._<br>_Beispiel: “100.A.10-1“._|
|**`_OrgDataType`**<br>(Fortsetzung)|`PRI`|`PRI`||||Verarbeitungskennzeichen<br>_Zur Abgrenzung interner Prozesse einer_<br>_Organisation von externen Prozessen._|
|**`_OrgDataType`**<br>(Fortsetzung)|`SGR`|`SGR`||||Verkäufergruppe|
|**`_OrgDataType`**<br>(Fortsetzung)|`SOF`|`SOF`||||Verkaufsbüro|
|**`_OrgDataType`**<br>(Fortsetzung)|`SOR`|`SOR`||||Verkaufsorganisation|
|**`_OrgDataType`**<br>(Fortsetzung)|`TOU`|`TOU`||||Tour|
|**`_OrgDataType`**<br>(Fortsetzung)|`TRZ`|`TRZ`||||Transportzone|
|**`_OrgDataType`**<br>(Fortsetzung)|**Restriktionen**|**Restriktionen**||||**Verwendung**|
|**`_OrgDataType`**<br>(Fortsetzung)|`ITM`|`ITM`||||Nur bei Belegpositionen|
|**`_OrgDataType`**<br>(Fortsetzung)|`HDR`|`HDR`||||Nur bei Belegkopf|
|**`_PackageType`**|**`CHAR(UPPER)`**|**3 **||||**Verpackungsart**|
|**`_PackageType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Restrikt.**||
|**`_PackageType`**|`CBB`|`CBB`||||Karton|
|**`_PackageType`**|`PAP`|`PAP`||||Papier|
|**`_PackageType`**|`FOI`|`FOI`||||Folie|
|**`_PackageType`**|`BOT`|`BOT`||||Flasche|
|**`_PackageType`**|`TIN`|`TIN`||||Dose|
|**`_PackageType`**|`CAN`|`CAN`||||Kanister|
|**`_PackageType`**|`BOX`|`BOX`||||Kiste|
|**`_PackageType`**|`BAG`|`BAG`||||Beutel|
|**`_PaymentDays`**|**`NUM(NOSIGN)`**|**3 **||||**Anzahl Tage**|
|**`_PaymentPart`**|**`NUM(LIST1)`**|**1 **||||**Bestandteil der Zahlungsbedingung**|
|**`_PaymentRate`**|**`NUM(NOSIGN)`**|**5 **|**2 **||**. **|**Skonto-Satz (%)**|
|**`_PosNo`**|**`CHAR(POS)`**|**6 **||||**Positionsnummer**|
|**`_PostalCode`**|**`CHAR(POSTAL)`**|**10**||||**Postleitzahl**|
|**`_Pricing`**|**`Frame`**|||||**Preiskalkulation**|
|**`_Pricing`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**|**Pflicht**||
|**`_Pricing`**|`aCondNo`|`aCondNo`|X|X|X|Laufende Nummer der Kondition|
|**`_Pricing`**|`aAction`|`aAction`||||Aktion|
|**`_Quantity`**|**`NUM(NOSIGN)`**|*** **|**3 **||**. **|**Mengenangaben**|
|**`_Quantity`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**|**Pflicht**||
|**`_Quantity`**|`aAction`|`aAction`||||Aktion|
|**`_Reference`**|**`Frame`**|||||**Verweis**|
|**`_Reference`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**|**Pflicht**||
|**`_Reference`**|`aAction`|`aAction`||||Aktion|



                               - 39 

|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_ReferenceType`**|**`CHAR(UPPER)`**|**3 **|||**Verweisarten**|
|**`_ReferenceType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_ReferenceType`**|`ATT`|`ATT`|||Anhang (Attachment)<br>_Vollständiger Name einer Datei, die mit_<br>_dem OEX-Dokument in einer Email_<br>_geschickt wird. (z.B._<br>_Produktinformationen.pdf)_|
|**`_ReferenceType`**|`DOC`|`DOC`|||Dokumentenangabe<br>_Name oder Dateiname eines Dokuments_|
|**`_ReferenceType`**|`EDS`|`EDS`|||Eingebetteter Datenstrom, kodiert mit<br>Base64|
|**`_ReferenceType`**|`LNK`|`LNK`|||Ausführbarer Link (vollständige URL)<br>_Z.B. um über den Internet-Browser direkt_<br>_auf eine HTML-Seite zu springen._|
|**`_ReferenceType`**|`XML`|`XML`|||Eingebettetes CDATA-Element<br>_Mit einem CDATA-Element können in XML_<br>_vorliegende strukturierte Daten aus einem_<br>_Drittsystem eingebettet werden. Voraus-_<br>_setzung dafür ist, dass die eingebetteten_<br>_Daten selber keine CDATA-Elemente_<br>_verwenden und als Zeichenkodierung_<br>_(encoding) UTF-8 benutzen. (Als MIME-_<br>_Type ist_„text/plain“_ anzugeben.)_|
|**`_ReferenceType`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_ReferenceType`**|`aMIMEType`|`aMIMEType`|X|X|MIME-Type<br>Typ der Daten (nach RFC 2046)|
|**`_ShipmentBase`**|**`CHAR(UPPER)`**|**1 **|||**Transportgrundlage**<br>Definiert, ob sich ein Transport entweder<br>aus Bestellungen oder aus Lieferungen<br>zusammensetzt.|
|**`_ShipmentBase`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_ShipmentBase`**|`O `|`O `|||Bestellungen (Aufträge)|
|**`_ShipmentBase`**|`D `|`D `|||Lieferungen (Lieferscheine)|
|**`_SwiftBic`**|**`CHAR(NUPPER)`**|**11**|||**SWIFT-BIC Int. Bankenschlüssel**<br>**S**ociety for**W**orldwide**I**nterbank**F**inancial<br>**T**elecommunication<br>**B**ank**I**dentifier**C**ode<br>Nach ISO 9362|
|**`_TextLine`**|**`CHAR`**|**80**|||**Textzeile**|
|**`_TextLine`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_TextLine`**|`aTextLineNo`|`aTextLineNo`|X|X|Textzeilennummer|
|**`_TextLine`**|`aLineFormat`|`aLineFormat`|X|X|Zeilenformat|
|**`_Text`**|**`Frame`**||||**Texte**|
|**`_Text`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Text`**|`aAction`|`aAction`|||Aktion|
|**`_TextType`**<br>(Forts. auf nächster Seite)|**`CHAR(UPPER)`**|**4 **|||**Textarten**|
|**`_TextType`**<br>(Forts. auf nächster Seite)|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_TextType`**<br>(Forts. auf nächster Seite)|`HEAD`|`HEAD`|`HDR`|`HDR`|Allgemeiner Kopftext<br>_Texte, die nicht durch Textarten für den_<br>_Belegkopf abgedeckt sind._|
|**`_TextType`**<br>(Forts. auf nächster Seite)|`ITEM`|`ITEM`|`ITM`|`ITM`|Sonstiger Positionstext<br>_Texte, die nicht durch die unten definierten_<br>_Textarten für die Belegposition abgedeckt_<br>_sind._|



                               - 40 

|Domäne|Datentyp|Lng|Dez|
|---|---|---|---|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|**Wertetabelle**|**Restrikt.**||
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`ARTS`|`ITM`|Artikelkurztext<br>_Dient als Kurzzeichnung des Artikels und_<br>_wird nur mit einer Zeile angegeben._<br>(`aTextLineNo = 1`) <br>_Neben der Artikelnummer wird durch den_<br>_Artikelkurztext der Artikel innerhalb der_<br>_XML-Datei zusätzlich identifiziert und sorgt_<br>_für eine bessere Lesbarkeit der XML-Datei_<br>_beispiels-weise direkt in einem Browser_<br>_mittels einem Style-Sheet._|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`ARTL`|`ITM`|Artikellangtext<br>_Wird zur detaillierten Beschreibung des_<br>_Artikels verwendet. Wird ein beiden_<br>_Geschäftspartnern bekannter_<br>_Standardartikel (Originalartikel vom_<br>_Hersteller/Lieferant) nicht verändert, muss_<br>_kein Artikellangtext über-tragen werden._|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`ARTV`|`ITM`|Variantentext<br>_Beschreibt die vom Anwender gewünschte_<br>_Konfiguration. (Aufgrund der im OCD_<br>_vorgesehenen Steuermöglichkeiten für den_<br>_Variantentext kann sich dieser von dem_<br>_Text unterscheiden, der sich aus der_<br>_Menge/Summe der Texte für die einzelnen_<br>_konfigurierbaren Merkmale ergibt.)_|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`ARTM`|`ITM`|Modifizierter Artikeltext<br>_Handelt es sich um einen geänderten_<br>_Standardartikel (Originalartikel vom_<br>_Hersteller/Lieferant), wird dessen_<br>_modifizierter Text hiermit übertragen, sowie_<br>_der Artikel entsprechend gekennzeichnet._<br>_Systeme, die keine Unterscheidungen von_<br>_Textarten besitzen und alle Texte in einem_<br>_Textblock verwalten, müssen dann auch_<br>_den gesamten Textblock hier einstellen._<br>(`_VendorArtNo`  `aStatus = M`)|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`ARTU`|`ITM`|Nutzerdefinierter Zusatztext<br>_Ein vom Anwender des Bestellsystems_<br>_eingegebener Zusatztext, z.B. zur Angabe_<br>_von Informationen zur Aufstellung o.ä._<br>_Diese Textart ist inhaltlich vom modifizier-_<br>_ten Artikeltext (ARTM) abgegrenzt und_<br>_verlangt im Gegensatz zu diesem auch_<br>nicht_ die Angabe von_ `aStatus = M`.|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`PAYC`|`HDR`|Zahlungsbedingungen<br>_Wenn abweichend von vertraglichen_<br>_Vereinbarungen oder nicht vereinbart._|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`GRTM`||Warenannahmezeiten|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`DNOT`||Versandhinweise|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`DCON`|`HDR`|Lieferbedingungen<br>_Wenn abweichend von vertraglichen_<br>_Vereinbarungen oder nicht vereinbart._|
|**`_TextType`** <br>(Forts., Forts. auf nächster<br>Seite)|`INOT`||Montagehinweise|



                               - 41 

|Domäne|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`_TextType`** <br>(Fortsetzung)|`PRMD`|`PRMD`|||Abwicklungsmodalitäten<br>_Hinweise zur Unterstützung der_<br>_Ausführung eines Geschäftsfalls._<br>_Bsp.: „Bitte stellen Sie bei Anlieferung_<br>_Personal zum Entladen und Verteilen zur_<br>_Verfügung.“_|
|**`_TextType`** <br>(Fortsetzung)|`ADAG`|`ADAG`|||Zusatzvereinbarungen<br>_(mit vertraglicher Relevanz)_|
|**`_TextType`** <br>(Fortsetzung)|**Restriktionen**|**Restriktionen**|||**Verwendung**|
|**`_TextType`** <br>(Fortsetzung)|`ITM`|`ITM`|||Nur bei Belegpositionen|
|**`_TextType`** <br>(Fortsetzung)|`HDR`|`HDR`|||Nur bei Belegkopf|
|**`_Time`**|**`CHAR(TIME)`**|**6 **|||**Zeit**|
|**`_TransportMode`**|**`CHAR(UPPER)`**|**3 **|||**Verkehrszweig**|
|**`_TransportMode`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_TransportMode`**|`SNA`|`SNA`|||Seeschifffahrt|
|**`_TransportMode`**|`SIN`|`SIN`|||Binnenschifffahrt|
|**`_TransportMode`**|`SCO`|`SCO`|||Küstenschifffahrt|
|**`_TransportMode`**|`LRO`|`LRO`|||Straßenverkehr|
|**`_TransportMode`**|`LRR`|`LRR`|||Eisenbahnverkehr|
|**`_TransportMode`**|`AAV`|`AAV`|||Luftfahrt|
|**`_TransportMode`**|`MMT`|`MMT`|||Multimodaler Transport<br>_(mehrere Verkehrszweige)_|
|**`_Unit`**|**`CHAR(NUPPER)`**|<br>**3 **|||**Maßeinheitenschlüssel**<br>Gemäß Common Code der UN/ECE<br>Recommendation 20<br>Beispiele:<br>`C62` Eins (Stück)<br>`MTR` Meter<br> <br> <br>`MTK` Quadratmeter|
|**`_Unit`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Unit`**|`aAction`|`aAction`|||Aktion|
|**`_UTC`**|**`CHAR(UTC)`**|**5 **|||**Zeitzone nach UTC**<br>Koordinierte Weltzeit<br>(UTC = Universal Time Coordinated)|
|**`_Value`**|**`CHAR`**|*** **|||**Beliebiger Wert**|
|**`_VendorArtNo`**|**`CHAR`**|*** **|||**Lieferantenartikelnummer**<br>_Hierbei handelt es sich um die_<br>_Grundartikelnummer des Lieferanten._<br>_(analog OCD Artikeltabelle__ ArticleID)_<br>_Bei konfigurierbaren Artikel wird das_<br>_Konfigurationsergebnis über den_<br>_Rahmentyp “_`Config` –<br>_Konfigurationsdaten“ beschrieben._<br>_Optional steht für weitere Informationen_<br>_zum Artikel bzw. zur Konfiguration der_<br>_Rahmentyp_` OrgData`_zur Verfügung._<br>_(bspw._`_OrgDataType "CNF"`_ oder_<br>`"ITM"`_)_|
|**`_VendorArtNo`**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_VendorArtNo`**|`aAction`|`aAction`|||Aktion|
|**`_VendorArtNo`**|`aStatus`|`aStatus`|X|X|Artikelstatus|



                               - 42 

|Domäne|Datentyp|Lng|Col4|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|---|
|**`_VendorID`**|**`CHAR(NUPPER)`**|<br>**4 **|<br>**4 **|||**Lieferantenkennung**<br>_Herstellerkürzel aus OCD-Spezifikation_|
|**`_VendorID`**|**Attribut**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_VendorID`**|`aAction`|`aAction`|`aAction`|||Aktion|
|**`_VendorSeries`**|**`CHAR(NUPPER)`**|<br>**4 **|<br>**4 **|||**Lieferantenserie**<br>_Herstellerserie aus OCD-Spezifikation_|
|**`_VendorSeries`**|**Attribut**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_VendorSeries`**|`aAction`|`aAction`|`aAction`|||Aktion|
|**`_Version`**|**`ATTR`**|||||**Versionierung**|
|**`_Version`**|**Attribut**|**Attribut**|**Attribut**|**Pflicht**|**Pflicht**||
|**`_Version`**|`aMajor`|`aMajor`|`aMajor`|X|X|Major Versionsnummer|
|**`_Version`**|`aMinor`|`aMinor`|`aMinor`|X|X|Minor Versionsnummer|
|**`_Version`**|`aBuild`|`aBuild`|`aBuild`|||Build Versionsnummer|
|**`_YesNo`**|**`BOOL`**|**`BOOL`**|**1 **|<br>|<br>|**Ja/Nein**|
|**`_YesNo`**|**Wertetabelle**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**||
|**`_YesNo`**|`Y `|`Y `|`Y `|||Ja|
|**`_YesNo`**|`N `|`N `|`N `|||Nein|



                               - 43 

**2.3** **Datentypen**

Namensgebung für Datentypen: komplett in Großbuchstaben

|Datentyp|Optionen|Bezeichnung/Beschreibung|
|---|---|---|
|**`ATTR`**|**Attributelement**|**Attributelement**|
|**`BOOL`**|**Boolescher Wert**|**Boolescher Wert**|
|**`BOOL`**|~~**`Y `**~~|Ja|
|**`BOOL`**|~~**`N `**~~|Nein|
|**`CHAR`**<br>(Forts. auf nächster Seite)|**Alle Zeichen der zugrunde gelegten Code-Page des OEX-Dokuments**|**Alle Zeichen der zugrunde gelegten Code-Page des OEX-Dokuments**|
|**`CHAR`**<br>(Forts. auf nächster Seite)|~~**`UPPER`**~~|**Nur Großbuchstaben**<br>Zulässige Zeichen:<br>**`ABCDEFGHIJKLMNOPQRSTUVWXYZ`**|
|**`CHAR`**<br>(Forts. auf nächster Seite)|~~**`NUPPER`**~~|**Großbuchstaben und Zahlen**<br>Zulässige Zeichen:<br>**`ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 `**<br>Beispiele:<br>`DE456271567`<br> <br>(USt-IdNr.) <br>`UBSWCHZH80A`<br> <br>(SWIFT-BIC) <br>`DE68210501700012345678`(IBAN)|
|**`CHAR`**<br>(Forts. auf nächster Seite)|~~**`XUPPER`**~~|**Großbuchstaben und andere**<br>Zulässige Zeichen:<br>**`ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789`**<br>**`+-*=_\/.,:;()!?#&%`**`"` <br>Leerzeichen innerhalb|
|**`CHAR`**<br>(Forts. auf nächster Seite)|~~**`LOWER`**~~|**Kleinbuchstaben**<br>Zulässige Zeichen:<br>**`abcdefghijklmnopqrstuvwxyz`**|
|**`CHAR`**<br>(Forts. auf nächster Seite)|~~**`NLOWER`**~~|**Kleinbuchstaben und Zahlen**<br>Zulässige Zeichen:<br>**`abcdefghijklmnopqrstuvwxyz0123456789`**|
|**`CHAR`**<br>(Forts. auf nächster Seite)|~~**`XLOWER`**~~|**Kleinbuchstaben und andere**<br>Zulässige Zeichen:<br>**`abcdefghijklmnopqrstuvwxyz0123456789`**<br>**`+-*=_\/.,:;()!?#&%`**`"` <br>Leerzeichen innerhalb|
|**`CHAR`**<br>(Forts. auf nächster Seite)|~~**`RX001`**~~|**Regulärer Ausdruck 001**<br>`[a-z][a-z0-9_-]*.[0-9]* `<br>Beispiel: de-2011.1|
|**`CHAR`**<br>(Forts. auf nächster Seite)|~~**`NUMB`**~~|**Nummerierung, Aufzählung, Reihe**<br>Zulässige Zeichen: <br>**`0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-.`** <br>Leerzeichen innerhalb<br>Beispiele:<br>`1, 1.1, 1.2 etc.`<br>`1, 1-1, 1-2 etc.`<br>`A, B, C etc.`<br>`I, II, III, IV etc.`<br>`I.1, I.2, etc.`|



                               - 44 

|Datentyp|Optionen|Bezeichnung/Beschreibung|
|---|---|---|
|**`CHAR`**<br>(Fortsetzung)|~~**`POS`**~~|**Positionsnummerierung**<br>Zulässige Zeichen: <br>**`0123456789 `**<br>_I.d.R eine aufsteigende Nummerierung unter_<br>_Verwendung einer Schrittweite (Inkrement)._<br>_Die Nummer kann mit führenden Nullen angegeben_<br>_werden._<br>Beispiel (6-stellige Nummer, Inkrement 1): <br>`000001, 000002, 000003 etc.`|
|**`CHAR`**<br>(Fortsetzung)|~~**`POSTAL`**~~|**Postleitzahlen**<br>Zulässige Zeichen: <br>**`0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ`** <br>Leerzeichen und**`-`** innerhalb<br>Beispiele:<br>`07743`<br>(Jena) <br>`170 00`<br>(Prag) <br>`ECM1 5PG`(London) <br>`00-023`<br>(Warschau)|
|**`CHAR`**<br>(Fortsetzung)|~~**`DATE`**~~|**Datumsangabe**<br>Zulässige Zeichen:<br>**`0123456789`** <br>_Tagesdatum (JJJJMMTT)) oder Wochendatum_<br>_(JJJJWW) siehe Attribut_ `aDateFormat`|
|**`CHAR`**<br>(Fortsetzung)|~~**`TIME`**~~|**Zeitangabe**<br>Zeitformat: 24 Stunden`HHMMSS` <br>`HH` <br> <br>Stunden (**`00`** –**`23`**) <br>`MM` <br> <br>Minuten (**`00`** –**`59`**) <br>`SS` <br> <br>Sekunden (**`00`** –**`59`**)|
|**`CHAR`**<br>(Fortsetzung)|~~**`UTC`**~~|**UTC Koordinierte Weltzeit**<br>(UTC = Universal Time Coordinated)<br>_Die Zeitzonen werden als positive oder negative_<br>_Abweichung (Zeitdifferenz) von UTC angegeben._<br>Format:`VSSMM` <br>`V` <br>= Vorzeichen (**`+`** od.**`-`** )<br>`SS` <br>= Stunden (**`00`** –**`23`**) <br>`MM` <br>= Minuten (**`00`** –**`59`**) <br>Beispiele:<br>Westeuropäische Zeit (WEZ)<br>`+0000` (+0 Stunden)<br>UTC<br>(Großbritannien, Portugal, Island, etc.)<br>Mitteleuropäische Zeit (MEZ)<br>`+0100` (+1 Stunde)<br>UTC+1<br>(Deutschland, Frankreich, Schweiz, etc.)<br>Mitteleurop. Sommerzeit (MESZ)<br>`+0200` (+2 Stunden)<br>UTC+2<br>Eastern Standard Time (EST)<br>`-0500` (-5 Stunden)<br>UTC-5<br>(USA-New York, Kuba, Peru, etc.)|
|**`CHAR`**<br>(Fortsetzung)|~~**`UUID`**~~|**Global eindeutiger Identifikator**<br>(UUID = Universally Unique Identifier)<br>_36-stellige Zeichenkettendarstellung nach RFC 4122_<br>Zulässige Zeichen:<br>**`0123456789-abcdefABCDEF`** <br>Beispiel: <br>`bbb5a714-27c6-416c-ad47-e4df02b6a93c`|



                              - 45 

|Datentyp|Optionen|Bezeichnung/Beschreibung|
|---|---|---|
|**`FRAME`**|**Rahmenelement**|**Rahmenelement**|
|**`NUM`**|**Numerischer Wert**<br>ggf. mit Angaben über Dezimalstellen und Trennzeichen.<br>Als Dezimaltrennzeichen wird der Punkt “**.**“ (Dezimalpunkt) verwendet.<br>Vorzeichen (`+` und`-`) werden vorangestellt. Ist kein Vorzeichen angegeben, wird<br>“`+`“ angenommen.|**Numerischer Wert**<br>ggf. mit Angaben über Dezimalstellen und Trennzeichen.<br>Als Dezimaltrennzeichen wird der Punkt “**.**“ (Dezimalpunkt) verwendet.<br>Vorzeichen (`+` und`-`) werden vorangestellt. Ist kein Vorzeichen angegeben, wird<br>“`+`“ angenommen.|
|**`NUM`**|~~**`LIST1`**~~|**Aufzählung 1**<br>_Verwendung bei bestimmten wiederkehrenden_<br>_Elementen. (z.B. Textzeilen)_<br>**Inkrement 1, Startwert 1, kein Vorzeichen,**<br>Beispiel:`1, 2, 3, 4` etc.|
|**`NUM`**|~~**`COUNT`**~~|**Anzahl Listelemente**<br>_Anzahl der Elemente die mit Datentyp NUM und Option_<br>_LIST1 aufgezählt werden und immer mindestens 1_<br>_Element beinhalten müssen._<br>**Mindestwert 1, kein Vorzeichen**|
|**`NUM`**|~~**`NOSIGN`**~~|**Ohne Vorzeichen**|
|**`NUM`**|~~**`VERSION`**~~|**Versionsnummer**<br>0 – 65535 (Integer), kein Vorzeichen|



                              - 46 

**2.4** **Attribute**

Namensgebung für Attribute: Präfix **`a`**
Bei manchen Attributen wird ein Wert als gesetzt betrachtet, wenn der Wert “leer“ `<empty>` ist und/oder das
Attribut weggelassen `<skipped>` wird.

|Attribut|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`aAction`**|**`CHAR(UPPER)`**|**1 **|||**Aktion**<br>_Verarbeitungsmethode für die das_<br>_Dokument empfangende Applikation._<br>_Wenn das Attribut in einem Element nicht_<br>_Pflicht ist, wird bei keiner Angabe der Wert_<br>`N`_ angenommen._|
|**`aAction`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aAction`**|`C `|`C `|||Anlegen|
|**`aAction`**|`D `|`D `|||Löschen|
|**`aAction`**|`M `|`M `|||Modifizieren|
|**`aAction`**|`N `|`N `|||Keine Aktion / Ohne Änderung|
|**`aBuild`**|**`NUM(VERSION)`**|*** **|||**Build Versionsnummer**|
|**`aBusPartClassType`**|**`CHAR(UPPER)`**|*** **|||**Art der Geschäftspartner-Klassifizierung**<br>_Legt fest, nach welcher Art (Norm,_<br>_Standard, Regel) die Klassifizierung_<br>_angegeben ist._|
|**`aBusPartClassType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aBusPartClassType`**|`SIC`|`SIC`|||Standard Industrial Classification|
|**`aBusPartClassType`**|`ISIC`|`ISIC`|||International Standard Industrial<br>Classification|
|**`aBusPartClassType`**|`NACE`|`NACE`|||Statistische Systematik der<br>Wirtschaftszweige in der Europäischen<br>Gemeinschaft|
|**`aBusPartClassType`**|`ICS`|`ICS`|||Individuelles Klassifizierungs-System<br>(keiner offiziellen Norm entsprechend)|
|**`aBusPartIDType`**<br>**`CHAR(UPPER)`**<br>*** **<br> <br> <br>**Wertetabelle**<br>**Restrikt.**<br>`GLN`<br> <br>`DUNS`<br> <br>`IIS`<br>|**`CHAR(UPPER)`**|*** **|||**Art der Geschäftspartner-ID**<br>_Legt fest, nach welcher Art (Norm,_<br>_Standard, Regel) die ID angegeben ist._|
|**`aBusPartIDType`**<br>**`CHAR(UPPER)`**<br>*** **<br> <br> <br>**Wertetabelle**<br>**Restrikt.**<br>`GLN`<br> <br>`DUNS`<br> <br>`IIS`<br>|**`CHAR(UPPER)`**|*** **|||**Bezeichnung**|
|**`aBusPartIDType`**<br>**`CHAR(UPPER)`**<br>*** **<br> <br> <br>**Wertetabelle**<br>**Restrikt.**<br>`GLN`<br> <br>`DUNS`<br> <br>`IIS`<br>|`GLN`|`GLN`|||Global Location Number|
|**`aBusPartIDType`**<br>**`CHAR(UPPER)`**<br>*** **<br> <br> <br>**Wertetabelle**<br>**Restrikt.**<br>`GLN`<br> <br>`DUNS`<br> <br>`IIS`<br>|`DUNS`|`DUNS`|||Data Universal Numbering System|
|**`aBusPartIDType`**<br>**`CHAR(UPPER)`**<br>*** **<br> <br> <br>**Wertetabelle**<br>**Restrikt.**<br>`GLN`<br> <br>`DUNS`<br> <br>`IIS`<br>|`IIS`|`IIS`|||Individuelles ID-System<br>(keiner offiziellen Norm entsprechend)|
|**`aClassSystem`**|**`CHAR(XUPPER)`**|*** **|||**Klassifikationssystem**<br>_Legt fest, nach welcher Art (Norm,_<br>_Standard) eine Klasse bzw. Kategorie_<br>_angegeben ist._<br>_Folgende Systeme sind vordefiniert_<br>_(reserviert):_|
|**`aClassSystem`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aClassSystem`**|`ECO_FR`|`ECO_FR`|||Artikelkategorie für ECO-Tax Frankreich<br>(gemäß Anhang der OCD-Spezifikation)|
|**`aClassSystem`**|`ECLASS-x.y`|`ECLASS-x.y`|||Klassizierung nach dem eCl@ss-Modell in<br>der Version x.y|
|**`aClassSystem`**|`UNSPSC`|`UNSPSC`|||Klassizierung nach dem Standard<br>UN/SPSC|



                               - 47 

|Attribut|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`aCondArea`**|**`CHAR(UPPER)`**|**2 **|||**Konditionsbereich**|
|**`aCondArea`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aCondArea`**|`P `|`P `|||Einkauf|
|**`aCondArea`**|`S `|`S `|||Verkauf|
|**`aCondArea`**|`OP`|`OP`|||originaler EK des Herstellers (OCD)|
|**`aCondArea`**|`OS`|`OS`|||originaler VK des Herstellers (OCD)|
|**`aCondNo`**|**`NUM(LIST1)`**|**6 **|||**Laufende Nummer der Kondition**|
|**`aCondRef`**|**`NUM(NOSIGN)`**|**6 **|||**Konditionsbezug**<br>auf laufende Nummer der Kondition<br>(`aCondNo`).<br>_Für die Berechnungsbasis wird als Bezug_<br>_die_**_laufende Nummer_**_ der jeweiligen_<br>_Kondition angegeben. Die folgenden_<br>_Restriktionen enthalten die Konditionsarten,_<br>_auf die Bezug genommen werden kann._|
|**`aCondRef`**|**Restriktionen**|**Restriktionen**|||**Verwendung**|
|**`aCondRef`**|`SGRO`|`SGRO`|||Einzelbruttopreis|
|**`aCondRef`**|`TGRO`|`TGRO`|||Gesamtbrutto|
|**`aCondRef`**|`DISH`|`DISH`|||Rabatt auf Kopfebene|
|**`aCondRef`**|`DISI`|`DISI`|||Rabatt auf Positionsebene|
|**`aCondRef`**|`SURH`|`SURH`|||Zuschlag auf Kopfebene|
|**`aCondRef`**|`SURI`|`SURI`|||Zuschlag auf Positionsebene|
|**`aCondRef`**|`SUBH`|`SUBH`|||Zwischensumme auf Kopfebene|
|**`aCondRef`**|`SUBI`|`SUBI`|||Zwischensumme auf Positions-/Kopfebene|
|**`aCondRef`**|`TTNE`|`TTNE`|||Steuernetto|
|**`aCondSign`**|**`CHAR`**|**1 **|||**Kennzeichen Zu- bzw. Abschlag**|
|**`aCondSign`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aCondSign`**|`+ `|`+ `|||Zuschlag|
|**`aCondSign`**|`- `|`- `|||Abschlag|
|**`aDateCalcBase`**|**`CHAR(XUPPER)`**|**4 **|||**Kalkulationsbasis bei Datumsermittlung**<br>_Wird bei Feldern des Datentyps_<br>_`CHAR(DATE)` verwendet im_<br>_Zusammenhang mit dem Attribut_<br>_`aDateFormat` und dessen Angabe für eine_<br>_Anzahl von Tagen, die dann auf die_<br>_gewählte Kalkulationsbasis gemäß des_<br>_Attributs__`aDateCalcMode` gerechnet_<br>_werden, um im Empfängersystem das_<br>_entsprechende Datum zu ermitteln._|
|**`aDateCalcBase`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aDateCalcBase`**|`*DIO`|`*DIO`|||Datum Bestelleingang<br>_Dynamische Datum, ist erst zum Zeitpunkt_<br>_der Verarbeitung bekannt._|
|**`aDateCalcBase`**|`<_DateTimeType>`|`<_DateTimeType>`|||Angabe eines Typs “Datum und Zeit“<br>_Bezug auf ein Datum, das im Vorgänger-_<br>_element gleichen Typs angegeben wurde._|
|**`aDateCalcMode`**|**`CHAR`**|**1 **|||**Kalkulationsverfahren bei**<br>**Datumsermittlung**<br>_Setzt das Vorhandensein des Attributs_<br>_`aDateCalcBase` voraus._|
|**`aDateCalcMode`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aDateCalcMode`**|`+ `|`+ `|||Addition|
|**`aDateCalcMode`**|`- `|`- `|||Subtraktion|



                               - 48 

|Attribut|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`aDateFormat`**|**`CHAR(UPPER)`**|**1 **|||**Datumsformat**<br>`JJJJ` <br>Jahr (4 Stellen) bspw.`2006` <br>`MM` <br> <br>Monat (2 Stellen) bspw.`02` für<br> <br> <br> <br>Februar<br>`TT` <br> <br>Tag (2 Stellen) bspw.`03` <br>`WW` <br> <br>Woche (2 Stellen) bspw.`05` <br>`CCCC` <br>Anzahl Kalendertage (4<br>Stellen)<br> <br>bspw.`0014` <br>Beispiele:<br>`20060203` 3. Februar 2006<br> <br> <br>`200605` <br>Woche 5 in 2006|
|**`aDateFormat`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aDateFormat`**|`D `|`D `|||Tagesformat JJJJMMTT (YYYYMMDD)|
|**`aDateFormat`**|`W `|`W `|||Wochenformat JJJJWW (YYYYWW)|
|**`aDateFormat`**|`C `|`C `|||Anzahl Kalendertage KKKK (CCCC)|
|**`aDocContext`**|**`CHAR(XUPPER)`**|**1 **|||**Dokumentenzusammenhang**|
|**`aDocContext`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aDocContext`**|`S `|`S `|||**Abfolge**<br>Abfolge der Belege eines Geschäftsfalls.<br>_Bei einer Rechnungsposition_<br>_beispielsweise:_<br>_Belegnummer (und Position) des Angebots_<br>_(__`QUO`), des Auftrages (__`ORD` )(Bestellung),_<br>_der Lieferung (__`DEL`)._|
|**`aDocContext`**|`R `|`R `|||**Referenz**<br>Über die Referenz ist es möglich auf<br>Belege zu referenzieren, die keine Belege<br>im Sinne der Abfolge eines Geschäftsfalls<br>sind, aber als zusätzliche Information<br>dienen, um diesen abzuwickeln.<br>_Bspw. kann bei einer Reklamationsab-_<br>_wicklung auf den Auftrag verwiesen_<br>_werden, bei dem die Reklamation auftrat._|
|**`aDocNo`**|**`NUM(LIST1)`**|**6 **|||**Laufende Nummer des Dokuments**|
|**`aDocumentCount`**|**`NUM(COUNT)`**|**6 **|||**Anzahl der Dokumente in der Mappe**|
|**`aEANType`**|**`CHAR(XUPPER)`**|**6 **|||**EAN-Typ**|
|**`aEANType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aEANType`**|`EAN-8`|`EAN-8`|||EAN-Code mit 8 Ziffern|
|**`aEANType`**|`EAN-13`|`EAN-13`|||EAN-Code mit 13 Ziffern|
|**`aIsPseudo`**|**`BOOL`**|**1 **|||**Repräsentiert die Position einen**<br>**Pseudo-Artikel?**<br>Falls ja (Wert`Y`), muss die Position im<br>verarbeitenden System ggf. speziell<br>behandelt werden.<br>_Die Artikelnummer eines Pseudo-Artikels_<br>_ist im ERP-System des Lieferanten_<br>_typischerweise nicht vorhanden, sondern_<br>_wurde im Rahmen der OFML-Datenanlage_<br>_aus technischen Gründen künstlich_<br>_angelegt._<br>_Ist das Attribut nicht angegeben oder leer,_<br>_wird der Wert_`N`_ (nein) angenommen._|
|**`aIsVisible`**|**`BOOL`**|**1 **|||**Ist das Merkmal für den Anwender**<br>**sichtbar?**<br>_Ist das Attribut nicht angegeben oder leer,_<br>_wird der Wert_`Y`_ (ja) angenommen._|



                               - 49 

|Attribut|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`aItemCount`**|**`NUM(COUNT)`**|**6 **|||**Anzahl der Belegpositionen im**<br>**Dokument**<br>_Unabhängig davon, ob es sich um eine_<br>_Hauptposition oder um eine Unterposition_<br>_handelt._|
|**`aItemNo`**|**`NUM(LIST1)`**|**6 **|||**Laufende Nummer der Belegposition**|
|**`aItemType`**|**`CHAR(UPPER)`**|**1 **|||**Typ der Belegposition**<br>_Zur differenzierten Verarbeitung der_<br>_Position._|
|**`aItemType`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aItemType`**|`<empty>/<skipped>`|`<empty>/<skipped>`|||Standard|
|**`aItemType`**|`O `|`O `|`1 `|`1 `|Optional|
|**`aItemType`**|`A `|`A `|`1 `|`1 `|Alternativ|
|**`aItemType`**|**Restriktionen**|**Restriktionen**|||**Verwendung**|
|**`aItemType`**|`1 `|`1 `|||für Anfrage und Angebot|
|**`aLineFormat`**|**`CHAR`**|**1 **|||**Zeilenformat**<br>Angelehnt an OCD ab Version 4|
|**`aLineFormat`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aLineFormat`**|~~**`\ `**~~|~~**`\ `**~~|||**Zeilenvorschub**<br>_Textzeile wird in einer neuen Zeile_<br>_ausgegeben._|
|**`aLineFormat`**|~~**`~ `**~~|~~**`~ `**~~|||**Fließtext**<br>_Die Textzeile wird als Fließtext an den_<br>_vorherigen Text gehängt. Beginnt die_<br>_Textzeile selber nicht mit einem_<br>_Leerzeichen, muss dieses von der_<br>_verarbeitenden Applikation eingefügt_<br>_werden._|
|**`aLocale`**|**`CHAR(UPPER)`**|**2 **|||**Gebietsschema**<br>Zur Feinsteuerung von Länderspezifika<br>bspw. Sprache, Maßeinheiten etc.<br>Angaben auf Basis des Länderschlüssels<br>nach ISO 3166-1<br>Beispiele:<br>`DE` <br>Deutschland<br>`ES` <br>Spanien<br>`GB` <br>Großbritannien<br>`FR` <br>Frankreich|
|**`aMajor`**|**`NUM(VERSION)`**|*** **|||**Major Versionsnummer**|
|**`aMIMEType`**|**`CHAR`**|*** **|||**MIME-Type**<br>(Multipurpose Internet Mail Extensions)<br>Format gemäß RFC 2046:<br><Media Type>/<Subtype><br>Beispiele:<br>text/html<br>text/plain<br>image/jpeg<br>application/pdf<br>application/msword|
|**`aMinor`**|**`NUM(VERSION)`**|*** **|||**Minor Versionsnummer**|
|**`aMustCheck`**|**`BOOL`**|**1 **|||**Ist prüfrelevant?**<br>_Legt fest, ob die Entität (z.B. Merkmal der_<br>_Konfiguration, Domäne__`_Configuration`) _<br>_bei Änderungen im Vergleich zum Vor-_<br>_gängerdokument (s.a.__`aAction`) während_<br>_der Verarbeitung des Dokuments zwingend_<br>_geprüft werden muss. (Bei__`N` kann/soll eine_<br>_Änderung ignoriert werden.)_|



                               - 50 

|Attribut|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`aScopeInfo`**|**`CHAR(UPPER)`**|**1 **|||**Anwendungsbereich der Information**|
|**`aScopeInfo`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aScopeInfo`**|`B `|`B `|||geschäftlich|
|**`aScopeInfo`**|`P `|`P `|||privat|
|**`aStatus`**<br>(Forts. auf nächster Seite)|**`CHAR(UPPER)`**|**1 **|||**Artikelstatus**<br>Der Artikelstatus legt fest, welchen<br>Ursprung der Artikel hat und ob dessen<br>Stammdaten (Aufbau/Texte/Konfiguration)<br>verändert wurden. <br>_Hiermit soll die automatische Verarbeitung_<br>_unterstützt werden. Der Artikelstatus_<br>_bezieht sich nicht auf Bestellmengen,_<br>_Preise oder andere die Position_<br>_betreffenden Daten._|
|**`aStatus`**<br>(Forts. auf nächster Seite)|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aStatus`**<br>(Forts. auf nächster Seite)|~~**`M `**~~|~~**`M `**~~|`REQOTE`<br>`QUOTES`<br>`ORDERS`<br>`ORDRSP`<br>`ORDCHG`|`REQOTE`<br>`QUOTES`<br>`ORDERS`<br>`ORDRSP`<br>`ORDCHG`|**Modifikation des Artikels**<br>_Die vom Hersteller/Lieferanten bereit-_<br>_gestellten Originaldaten des Artikels (O)_<br>_oder ein Sonderartikel (S) wurden_<br>_geändert._<br>_(bspw. Artikelnummer, Serienkürzel, Texte)_<br>_Modifizierte Texte werden über den_<br>_Positionstext mit der Textart__`"ARTM"` _<br>_(Modifizierter Artikeltext) bereitgestellt._<br>_Modifikationen sind vom Besteller beim_<br>_Hersteller/Lieferant vorher angefragt_<br>_worden, anderenfalls wird ein solcher_<br>_Artikel ggf. vom Hersteller/Lieferant_<br>_abgelehnt bzw. nicht bestätigt._<br>_Ersetzt der Hersteller/Lieferant diesen_<br>_Artikel nicht durch einen Sonderartikel (S),_<br>_liefert er ihn ebenfalls mit Status (M) und_<br>_dem modifizierten Text__`"ARTM"` zurück._|
|**`aStatus`**<br>(Forts. auf nächster Seite)|~~**`S `**~~|~~**`S `**~~|||**Sonderartikel des**<br>**Herstellers/Lieferanten**<br>_Artikeldaten, die durch den Hersteller/_<br>_Lieferanten für einen modifizierten Artikel_<br>_(M) oder Kundenartikel (C) zurückgeliefert_<br>_werden, wenn er den Artikel durch einen_<br>_eigenen ersetzt._<br>_Artikeltexte werden hierbei wie folgt_<br>_zurückgegeben:_<br>_Artikellangtext über Textart_`"ARTL"`_, _<br>_Artikelkurztext über Textart_`"ARTS"`_, _<br>_Mögliche Konfiguration über die_<br>_Konfigurationsrahmenelemente_<br>`"itmConfiguration"`_ und_<br>`"itmConfigText"`_. _<br>_Dieses Kennzeichen wird bei einer_<br>_Bestelländerung (ORDCHG) ebenfalls_<br>_durch den Besteller angegeben, wenn er_<br>_diesen Artikel nicht modifiziert hat._|




- 51 

|Attribut|Datentyp|Lng|Col4|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|---|
|**`aStatus`**<br>(Fortsetzung)|**Wertetabelle**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aStatus`**<br>(Fortsetzung)|~~**`O `**~~|~~**`O `**~~|~~**`O `**~~|||**Originalartikel**<br>_Artikel entspricht den Originaldaten, wie_<br>_durch den Hersteller/Lieferanten über_<br>_elektronischer Preisliste bereitgestellt._|
|**`aStatus`**<br>(Fortsetzung)|~~**`C `**~~|~~**`C `**~~|~~**`C `**~~|`REQOTE`<br>`QUOTES`<br>`ORDERS`<br>`ORDRSP`<br>`ORDCHG`<br>|`REQOTE`<br>`QUOTES`<br>`ORDERS`<br>`ORDRSP`<br>`ORDCHG`<br>|**Kundenartikel**<br>_Ein vom Besteller selbst in seinen_<br>_Stammdaten oder direkt im Auftrag_<br>_(Einmalverwendung) erstellter Artikel für_<br>_einen Sonderartikel des_<br>_Herstellers/Lieferanten._<br>_Ein solcher Artikel ist vom Besteller beim_<br>_Hersteller/Lieferant vorher angefragt,_<br>_anderenfalls wird ein solcher Artikel ggf._<br>_vom Hersteller/Lieferant abgelehnt bzw._<br>_nicht bestätigt. Ggf. hat der_<br>_Hersteller/Lieferant dem Besteller bereits_<br>_eine Artikelnummer genannt, mit der dieser_<br>_den Artikel selbst anlegen kann._<br>_Artikeltexte werden hierbei wie folgt_<br>_übertragen:_<br>_Artikellangtext über Textart_`"ARTL"`_, _<br>_Artikelkurztext über Textart_`"ARTS"`_, _<br>_Ersetzt der Hersteller/Lieferant diesen_<br>_Artikel nicht durch einen Sonderartikel (S),_<br>_liefert er ihn ebenfalls mit Status (C) zurück._|
|**`aStatus`**<br>(Fortsetzung)|**Restriktionen**|**Restriktionen**|**Restriktionen**|||**Verwendung**|
|**`aStatus`**<br>(Fortsetzung)|`REQOTE`|`REQOTE`|`REQOTE`|||Anfrage|
|**`aStatus`**<br>(Fortsetzung)|`QUOTES`|`QUOTES`|`QUOTES`|||Angebot|
|**`aStatus`**<br>(Fortsetzung)|`ORDERS`|`ORDERS`|`ORDERS`|||Bestellung|
|**`aStatus`**<br>(Fortsetzung)|`ORDRSP`|`ORDRSP`|`ORDRSP`|||Bestellbestätigung|
|**`aStatus`**<br>(Fortsetzung)|`ORDCHG`|`ORDCHG`|`ORDCHG`|||Bestelländerung|
|**`aStatus`**<br>(Fortsetzung)|`DESADV`|`DESADV`|`DESADV`|||Lieferavis|
|**`aStatus`**<br>(Fortsetzung)|`INVOIC`|`INVOIC`|`INVOIC`|||Rechnung|
|**`aTaxCode`**|**`NUM(NOSIGN)`**|**`NUM(NOSIGN)`**|**3 **|||**Steuerkennzeichen**|
|**`aTaxCode`**|**Wertetabelle**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aTaxCode`**|`0 `|`0 `|`0 `|||steuerfrei|
|**`aTaxCode`**|`1 - 6`|`1 - 6`|`1 - 6`|||Steuerkategorie für Mehrwertsteuer laut<br>Anhang der OCD-Spezifikation:<br>1 = Normalsatz (volle Steuer)<br>2 = ermäßigter Satz (reduzierte Steuer)<br>3 = stark ermäßigter Satz<br>4 = Zwischensatz<br>5 = Dienstleistungen<br>6 = Nullsatz|
|**`aTaxCode`**|`7 - 99`|`7 - 99`|`7 - 99`|||reserviert (für zukünftige Standardisierung)|
|**`aTaxCode`**|`100 - 999`|`100 - 999`|`100 - 999`|||zur freien Verwendung|
|**`aTextLineNo`**|**`NUM(LIST1)`**|**`NUM(LIST1)`**|**6 **|||**Textzeilennummer**|
|**`aTransferMode`**|**`CHAR(UPPER)`**|**`CHAR(UPPER)`**|**1 **|||**Transfermodus der XML-Datei**|
|**`aTransferMode`**|**Wertetabelle**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aTransferMode`**|`<empty>/<skipped>`|`<empty>/<skipped>`|`<empty>/<skipped>`|||Echtdatentransfer|
|**`aTransferMode`**|`R `|`R `|`R `|||Wiederholtransfer|
|**`aTransferMode`**|`T `|`T `|`T `|||Testtransfer|




- 52 

|Attribut|Datentyp|Lng|Dez|Trz|Bezeichnung|
|---|---|---|---|---|---|
|**`aTypeDis`**|**`CHAR(NUPPER)`**|**2 **|||**Art des Abschlags**|
|**`aTypeDis`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aTypeDis`**|`BD`|`BD`|||Grundrabatt|
|**`aTypeDis`**|`VO`|`VO`|||Volumenrabatt|
|**`aTypeDis`**|`SD`|`SD`|||Sonderrabatt|
|**`aTypeDis`**|`RD`|`RD`|||Wiederverkäuferrabatt|
|**`aTypeDis`**|`AR`|`AR`|||Treuerabatt|
|**`aTypeDis`**|`D1 – D9`|`D1 – D9`|||Sonstige Abschläge/Rabatte (1 – 9)|
|**`aTypeSur`**|**`CHAR(NUPPER)`**|**2 **|||**Art des Zuschlags**|
|**`aTypeSur`**|**Wertetabelle**|**Wertetabelle**|**Restrikt.**|**Restrikt.**|**Bezeichnung**|
|**`aTypeSur`**|`PC`|`PC`|||Verpackungszuschlag|
|**`aTypeSur`**|`TP`|`TP`|||Transportzuschlag|
|**`aTypeSur`**|`MO`|`MO`|||Montagezuschlag|
|**`aTypeSur`**|`SQ`|`SQ`|||Mindermengenzuschlag|
|**`aTypeSur`**|`PS`|`PS`|||Abwicklungszuschlag|
|**`aTypeSur`**|`S1 – S9`|`S1 – S9`|||Sonstige Zuschläge (1 – 9)|
|**`aUUID`**|~~**`CHAR (UUID)`**~~|**36**|||**Global eindeutiger Identifikator**|



                               - 53 

### **3 OEX – Szenarien**

**3.1** **Bestellung mit anschließender Bestelländerung (Idealfall)**


`ORDERS`  `ORDRSP`  `ORDCHG`  `ORDRSP`
Szenario: (1) Besteller bestellt, (2) Lieferant bestätigt Bestellung, (3) Besteller ändert Bestellung,
(4) Lieferant bestätigt Änderung


**Zeitstrahl** **Besteller** **Lieferant**


**3.2** **Bestellung mit Bestelländerungen (zeitversetzt zur Bestellbestätigung)**


`ORDERS`  `ORDCHG`  `ORDRSP`  `ORDCHG`  `ORDRSP`
Szenario: (1) Besteller bestellt, (3) Besteller ändert Bestellung vor Bestellbestätigung, (3) Lieferant
bestätigt inkl. Änderungen, (4) Besteller ändert Bestellung ein weiteres mal, (5) Lieferant bestätigt


**Zeitstrahl** **Besteller** **Lieferant**


                               - 54 

**3.3** **Bestellung mit Änderungen ausgelöst durch den Lieferanten**


`ORDERS`  `ORDRSP`  `ORDRSP`
Szenario: (1) Besteller bestellt, (2) Lieferant bestätigt, (3) Lieferant ändert (bspw. Lieferdatum)


**Zeitstrahl** **Besteller** **Lieferant**


**3.4** **Von Anfrage bis Rechnung (Idealfall)**


`REQOTE`  `QUOTES`  `ORDERS`  `ORDRSP`  `DESADV`  `INVOIC`
Szenario: (1) Besteller stellt Anfrage, (2) Lieferant schickt Angebot, (3) Besteller bestellt,
(4) Lieferant bestätigt, (5) Lieferant avisiert die Lieferung, (6) Lieferant stellt Rechnung


**Zeitstrahl** **Besteller** **Lieferant**


                               - 55 

**3.5** **Von Anfrage bis Rechnung inklusive Bestelländerung (Idealfall)**


`REQOTE`  `QUOTES`  `ORDERS`  `ORDRSP`  `ORDCHG`  `ORDRSP`  `DESADV`  `INVOIC`
Szenario: (1) Besteller stellt Anfrage, (2) Lieferant schickt Angebot, (3) Besteller bestellt,
(4) Lieferant bestätigt, (5) Besteller schickt Bestelländerung, (6) Lieferant bestätigt Bestelländerung,
(7) Lieferant avisiert die Lieferung, (8) Lieferant stellt Rechnung


**Zeitstrahl** **Besteller** **Lieferant**


                               - 56 

### **4 Anhang**

**4.1** **Änderungshistorie**

|Version|Änderungen|
|---|---|
|3.1.0 – 8.5.2023|1 Einleitung <br>Erweiterte Erläuterungen<br> <br>2.1.3 OEX-Werttypen<br>Neu:<br> `FolderIsLOC`<br>– Ist die Bezeichnung des Ordners eine Ortsangabe?<br> <br>2.2 Datendomänen <br>Entfernt (nicht verwendet):<br> `_Phone`<br> <br>– Nummer für Telefon etc (s. dafür allgemeinen Rahmentyp`Com`) <br>Korrektur:<br> _TextType<br> <br>- Textarten<br>Bei Textart`ARTM` muss das Attribut`aStatus` von`_VendorArtNo` den Wert`M` haben.<br>Erweiterung:<br> _Configuration<br>– Merkmal der Konfiguration<br>Neues Attribut<br>`aIsVisible` <br>– ist sichtbar?<br> <br>2.3 Datentypen<br>`CHAR`– Alle Zeichen der zugrunde gelegten Code-Page des OEX-Dokuments<br> <br>Entfernte Option (nicht verwendet):`PHONE` – Telefonnummern<br>Entfernte Option (nicht verwendet):`NUMCHAR` – Nummern und Buchstaben<br> <br>2.4 Attribute<br>Neu:<br> `aIsPseudo`<br>– repräsentiert die Position einen Pseudo-Artikel?<br> `aIsVisible`<br>– ist das Merkmal für den Anwender sichtbar?<br>|
|3.0.0 – 30.11.2017<br>(Forts. auf nächster<br>Seite)|1.1 Übersicht OEX-Spezifikationen<br>Neue Major-Versionen der Dokumentenarten:<br> `REQOTE` – Anfrage<br> `QUOTES` – Angebot<br> `ORDERS` – Bestellung<br> `ORDRSP` – Bestellbestätigung<br> `ORDCHG` – Bestelländerung<br> `DESADV` – Lieferavis<br> `INVOIC` – Rechnung<br> <br>2.1.2 OEX-Rahmentypen <br>Änderung:<br> 2.1.2.7`DateTime`: Datums- und Zeitangaben<br>Domäne`_DateTime` ► ersetzt`_Frame` <br> 2.1.2.8`OrgData`: Organisationsdaten<br>Domäne`_OrgData` ► ersetzt`_Frame` <br> 2.1.2.9`Address`: Adressen<br>Domäne`_Address` ► ersetzt`_Frame` <br> 2.1.2.12`Text`: Texte<br>Domäne`_Text` ► ersetzt`_Frame` <br> 2.1.2.13`Reference`: Verweise<br>Domäne`_Reference` ► ersetzt`_Frame` <br> 2.1.2.14`Pricing`: Preiskalkulation<br>`<QuantUnit>`<br>– Mengeneinheit ► ersetzt`<MeasureUnit>` <br> 2.1.2.15`Config`: Konfigurationsdaten<br>Domäne`_Configuration` ► ersetzt`_Frame` <br> <br>2.1.3 OEX-Werttypen<br>Neu:<br> `Quantity`<br>– Mengenangabe<br> `QuantUnit`<br>– Mengeneinheit<br>|



                               - 57 

|Version|Änderungen|
|---|---|
|3.0.0 – 30.11.2017<br>(Fortsetzg.)|Entfernt (stattdessen wird nun der allgemeinere Typ`Quantity` verwendet):<br> `ChgOrdQuant`<br>– geänderte Bestellmenge<br> `ConfOrdQuant` <br>– bestätigte Bestellmenge<br> <br> `DelivQuantity` <br>– Liefermenge<br> `InvoiQuantity` <br>– Rechnungsmenge<br> `OrderQuantity` <br>– Bestellmenge<br> `QuoteQuantity` <br>– Angebotsmenge<br> `RequQuantity` <br>– Anfragemenge<br>Entfernt (stattdessen wird nun der allgemeinere Typ`QuantUnit` verwendet):<br> `ChgOrdUnit`<br>– geänderte Bestellmengeneinheit<br> `ConfOrdUnit` <br>– bestätigte Bestellmengeneinheit<br> `DelivUnit` <br>– Liefermengeneinheit<br> `InvoiUnit` <br>– Rechnungsmengeneinheit<br> `OrderUnit` <br>– Bestellmengeneinheit<br> `QuoteUnit` <br>– Angebotsmengeneinheit<br> `RequUnit` <br>– Anfragemengeneinheit<br>Entfernt (stattdessen wird nun der allgemeinere Typ`DocNo` verwendet):<br> `DeliveryNumber`– Liefernummer<br> `InvoiceNumber` <br>– Rechnungsnummer<br> `OrdChangeNo` <br>– Bestelländerungsnummer<br> `OrdConfirmNo` <br>– Bestellbestätigungsnummer<br> `OrderNumber` <br>– Bestellnummer<br> `QuoteNumber` <br>– Angebotsnummer<br> `RequestNumber` <br>– Anfragenummer<br> `ShipmentNumber` <br>– Transportnummer<br>Umbenannt:<br> `DocLine`►`DocItemNo`– Nummer der Belegposition<br>Entfernt:<br>Alle Werttypen der Domäne`_PosNo` außer`DocItemNo` (stattdessen wird nun der allgemeinere Typ<br>`DocItemNo` verwendet).<br> <br>2.2 Datendomänen<br>Neu:<br> `_Address`<br>– Adressen<br> `_ClientArtNo`<br>– Kundenartikelnummer<br> `_Configuration`– Merkmal der Konfiguration<br> `_DateTime`<br>– Datums- und Zeitangaben<br> `_OrgData`<br>– Organisationsdaten<br> `_Reference`<br>– Verweise <br> `_Text`<br> <br>– Texte<br> <br>Erweiterung:<br> Neues Attribut`aAction` in:<br>_`AddStateCode`, _`CatalogId`, _`Classification`, _`CompSubArtId`, `_Pricing`, _`Quantity`, <br>_`Unit`, _`VendorArtNo`, _`VendorID`, _`VendorSeries` <br> _Item<br> <br>– Dokumentenposition<br>Neues Attribut<br>`aUUID` – global eindeutiger Identifikator<br> `_OrgDataType`<br>– Arten von Organisationsdaten<br>Neuer Wert<br>`COG` <br>– Warengruppe (class of goods)<br>Neuer Wert<br>`OVC` <br>– OFML-Variantencode<br> `_ReferenceType`– Verweisarten<br>Neuer Wert<br>`XML` <br>– Eingebettetes CDATA-Element<br> `_TextType`<br>– Textarten<br>Neuer Wert<br>`ARTV` – Variantentext<br>Neuer Wert<br>`ARTU` – Nutzerdefinierter Zusatztext<br>Änderung:<br> `_Header`<br> <br>– Dokumentenkopf<br>Attribut`aAction` ist nicht mehr Plicht<br> `_Item`<br> <br>– Belegposition<br>Attribut`aAction` ist nicht mehr Plicht<br>Attribute:<br> <br>`aItemTypeClient` <br>– Positionstyp der Kundenposition<br> <br> <br> <br>`aItemTypeVendor` <br>– Positionstyp der Lieferantenposition<br>ersetzt durch<br>`aItemType`<br> <br>– Typ der Belegposition|



                              - 58 

|Version|Änderungen|
|---|---|
|3.0.0 – 30.11.2017<br>(Fortsetzg.)|2.3 Datentypen<br>`CHAR`– Alle Zeichen der zugrunde gelegten Code-Page des OEX-Dokuments<br> <br>Neue Option:`UUID` – global eindeutiger Identifikator<br> <br>2.4 Attribute<br>Neu:<br> `aMustCheck`<br>– ist prüfrelevant?<br> `aUUID`<br> <br>– global eindeutiger Identifikator<br>Erweiterung:<br> `aAction` <br> <br>– Aktion<br>Präzisierung zum Defaultwert, wenn das Attribut in einem Element nicht Pflicht ist.<br>Korrektur:<br> `aBuild`,` aMajor`,` aMinor` – Versionsnummern<br>Die Anzahl der Ziffern ist nun nicht mehr auf 2 begrenzt.|
|2.3.1 – 13.1.2017|2.1.3 OEX-Werttypen<br>Fehlende Typen ergänzt:<br> `ChgOrdQuant`<br>– geänderte Bestellmenge<br> `ChgOrdUnit`<br>– geänderte Bestellmengeneinheit<br> `OrdChangeNo`<br>– Bestelländerungsnummer<br> `OrdChgCompNo`<br>– Nummer der Bestelländerungsposition des kompositen Artikels<br> `OrdChgItemNo`<br>– Nummer der Bestelländerungsposition<br> `OrdChgTopLevl`<br>– Übergeordnete Bestelländerungsnummer<br> `QuoteItemNo`<br>– Nummer der Angebotsposition<br> `RequestItemNo`<br>– Nummer der Anfrageposition<br>Entfernt (da nicht mehr verwendet):<br> `QuoteItemType`<br>– Typ der Angebotsposition<br> `RequItemType`<br>– Typ der Anfrageposition<br>2.2 Datendomänen <br>Erweiterung:<br> `_DocNoType`<br>– Belegnummernarten<br>Fehlenden Wert ergänzt:`CHG` – Bestelländerungsnummer<br>|
|2.3.0 – 1.7.2015|1.1 Übersicht OEX-Spezifikationen<br>Neue Minor-Versionen der Dokumentenarten:<br> `REQOTE` – Anfrage<br> `QUOTES` – Angebot<br> `ORDERS` – Bestellung<br> `ORDRSP` – Bestellbestätigung<br> `ORDCHG` – Bestelländerung<br> `DESADV` – Lieferavis<br> `INVOIC` – Rechnung<br> <br>2.1.3 OEX-Werttypen<br>Neu:<br> `Classification`– Allgemeine Klassifizierung<br> <br>2.2 Datendomänen <br>Neu:<br> `_Classification`– Allgemeine Klassifizierung<br> <br>2.4 Attribute<br>Neu:<br> `aClassSystem`<br>– Klassifizierungssystem (3 vordefinierte Systeme/Werte)<br> <br>Erweiterung:<br> `aCondArea` <br>– Konditionsbereich<br>Neuer Wert:<br>`OP` <br>– originaler EK des Herstellers<br>Neuer Wert:<br>`OS` <br>– originaler VK des Herstellers<br>Änderung:<br> `aTaxCode` <br>– Steuerkennzeichen<br>wurde auf 3 Stellen erweitert, Nummern 1-6 sind nun vordefiniert (für standardisierte Mwst-Sätze) und<br>Nummern 7-99 sind reserviert (für mögliche zukünftige Standardisierungen).<br>|



                              - 59 

|Version|Änderungen|
|---|---|
|2.2.0 – 11.10.2013<br>(Forts. auf nächster<br>Seite)<br>|1.1 Übersicht OEX-Spezifikationen<br>Neue Minor-Versionen der Dokumentenarten:<br> `REQOTE` – Anfrage<br> `QUOTES` – Angebot<br> `ORDERS` – Bestellung<br> `ORDRSP` – Bestellbestätigung<br> `ORDCHG` – Bestelländerung<br> `DESADV` – Lieferavis<br> `INVOIC` – Rechnung<br> <br>2.1.2 OEX-Rahmentypen <br>Erweiterung:<br>2.1.2.9`Address`: Adresse<br>neue optionale Elemente<br> `<AddressID>`<br>– Adress-ID ► ersetzt`<ILN_AddressID>` <br> `<Street2>`<br>– Straße 2<br> `<District>`<br>– Ortsteil<br>Gelöscht:<br> `<ILN_AddressID>`– ILN-Nummer Adresse<br> <br>Beispiele:<br>2.1.2.13`Reference`: Beispiel geändert und ein neues hinzugefügt<br> <br>2.1.3 OEX-Werttypen<br>Neu:<br> `Street2`<br> <br>– Straße 2<br> `District`<br>– Ortsteil<br> `CatalogId`<br>– Katalog-ID<br> `CompSubArtId`<br>– Identifikation des Unterartikels<br> `AddStateCode`<br>– Zusätzliche Zustandsinformationen<br> `ClientID`<br>– Kunden-ID ► ersetzt`ILN_Client` <br> `ClientClass`<br>– Kunden-Klassifizierung<br> `SupplierID`<br>– Lieferanten-ID ► ersetzt`ILN_Vendor` <br> `SupplierClass`<br>– Lieferanten-Klassifizierung<br> `AddressID`<br>– Adress-ID ► ersetzt`ILN_Address` <br> <br>Gelöscht:<br> `ILN_Address`<br>– ILN-Nummer Adresse<br> `ILN_Client`<br>– ILN-Nummer Kunde<br> `ILN_Vendor`<br>– ILN-Nummer Lieferant<br>Korrigiert (Schreibfehler):<br> `OrderCompNo ► OrderComposNo`– Nummer der Bestellposition des kompositen Artikels<br> <br>2.2 Datendomänen <br>Neu:<br> `_CatalogId`<br>– Katalog-ID<br> `_CompSubArtId`<br>– Identifikation des Unterartikels<br> `_AddStateCode`<br>– Zusätzliche Zustandsinformationen<br> `_BusPartID`<br>– Geschäftspartner-ID ► ersetzt`_ILN` <br> `_BusPartClass`<br>– Geschäftspartner-Klassifizierung<br> <br>Erweiterung:<br> `_ReferenceType`– Verweisart<br>Neuer Wert “`EDS` – Embedded Data Stream as Base64”<br>Neues Attribut`aMIMEType` – MIME-Typ<br> `_AddressType`<br>– Adressarten<br>Wertetabelle: Wert „`IL` – Ort der Montage“<br>Änderung:<br> `_OrgDataType`<br>– Arten der Organisationsdaten<br>Wert „`PRL` – Preisliste“ entfällt und findet Ersatz durch die Katalog-ID<br> <br>Gelöscht:<br> `_ILN`<br> <br>– ILN-Nummer<br>|



                              - 60 

|Version|Änderungen|
|---|---|
|2.2.0 – 11.10.2013<br>(Fortsetzg.)|2.3 Datentypen<br>`CHAR` – Alle Zeichen der zugrunde gelegten Code-Page des OEX-Dokuments<br>Neue Option:<br> `RX001` – Regulärer Ausdruck 001<br>betroffene Domäne:`_CatalogId` <br> <br>2.4 Attribute<br>Neu:<br> `aMIMEType` <br> <br>– MIME-Typ<br> `aBusPartClassType`<br>– Art der Geschäftspartner-Klassifizierung<br> `aBusPartIDType`– Art der Geschäftspartner-ID<br>|
|2.1.0 – 06.11.2009<br>(Forts. auf nächster<br>Seite)|1.1 Übersicht OEX-Spezifikationen<br>Neue Dokumentenarten:<br> `REQOTE` – Anfrage<br> `QUOTES` – Angebot<br>Neue Minor-Versionen der Dokumentenarten:<br> `ORDERS` – Bestellung<br> `ORDRSP` – Bestellbestätigung<br> `ORDCHG` – Bestelländerung<br> `DESADV` – Lieferavis<br> `INVOIC` – Rechnung<br> <br>1.2 Versionierung <br>Konkretisierungen/Ergänzungen<br> <br>2.1.2 OEX-Rahmentypen <br>Berichtigung:<br> 2.1.2.5`Header`: Belegkopf und 2.1.2.6`Item`: Dokumentenposition<br>bei den Beispielen fehlte das Attribut`aAction` <br>Änderung:<br> 2.1.2.15`itmPricing`: Preiskalkulation<br>das optionale Element`ConditionText` (gültig auch für`hdrPricing`) wurde nach dem Element<br>`CondCurrency` neu positioniert, danach folgen nun die nur in`itmPricing` gültigen optionalen Elemente<br>`PriceUnit` und`MeasureUnit.` <br> <br>2.1.3 OEX-Werttypen<br>Neu:<br> `QuoteAlterNo`<br>– Alternativposition zur Angebotsposition<br> `QuoteComposNo`<br>– Nummer der Angebotsposition des kompositen Artikels<br> `QuoteItemType`<br>– Positionstyp der Angebotsposition<br> `QuoteNumber`<br>– Angebotsnummer<br> `QuoteQuantity`<br>– Angebotsmenge<br> `QuoteTopLevel`<br>– Übergeordnete Nummer der Angebotsposition<br> `QuoteUnit`<br>– Angebotsmengeneinheit<br> `RequAlterNo`<br>– Alternativposition zur Anfrageposition<br> `RequComposNo`<br>– Nummer der Anfrageposition des kompositen Artikels<br> `RequestNumber`<br>– Anfragenummer<br> `RequItemType`<br>– Positionstyp der Anfrageposition<br> `RequQuantity`<br>– Anfragemenge<br> `RequTopLevel`<br>– Übergeordnete Nummer der Anfrageposition<br> `RequUnit`<br>– Anfragemengeneinheit|



                              - 61 

|Version|Änderungen|
|---|---|
|2.1.0 – 06.11.2009<br>(Fortsetzung)|2.2 Datendomänen <br>Neu:<br> `_ItemType`<br>– Positionstyp<br>Erweiterung:<br> `_Date`<br> <br>– Datum<br>Neue Attribute:<br>`aDateCalcBase` <br>– Kalkulationsbasis bei Datumsermittlung<br> <br> <br> <br>`aDateCalcMode` <br>– Kalkulationsverfahren bei Datumsermittlung<br> `_DateTimeType`<br>– Type Datum und Zeit<br>Neuer Wert:<br>`QUV` <br> <br> <br>– Angebotsgültigkeitsdatum<br> `_Language`<br>– Sprachenschlüssel<br>Neues Attribut:<br>`aLocale` <br> <br>– Gebietsschema<br> `_Item`<br> <br>– Dokumentenposition<br>Neue Attribute:<br>`aItemTypeClient` <br>– Positionstyp der Kundenposition<br> <br> <br> <br>`aItemTypeVendor` <br>– Positionstyp der Lieferantenposition<br>Löschung:<br> `_ValueText` <br>– Merkmalswertetext<br>ersetzt durch`_TextLine` – Textzeile, betrifft Typ`ValueText`, wegen Redundanz (Domäne identisch)<br>Hinzugefügt:<br> `_Pricing` <br>– Preiskalkulation<br>fehlte bisher, muss auf Grund des Attributes`aCondNo` aber definiert sein, da sie damit von der<br>Basisdomäne`_Frame`abweicht. (Erläuterungen und Beispiele waren bereits korrekt)<br>Berichtigung:<br> `_VendorArtNo`<br>– Lieferantenartikelnummer<br>im Gegensatz zu Beispielen und Erläuterungen war hier noch das alte Attribut`aSpecial` angegeben,<br>richtig:`aStatus` <br> `aDocNoType` <br>– Belegnummernart<br>Längenangabe berichtigt von 6 auf 3. (durch Attribut`aDocContext` nicht mehr erforderlich und stammte<br>noch aus dem alternativen Ansatz)<br> <br>2.3 Datentypen<br>`CHAR` – Alle Zeichen der zugrunde gelegten Code-Page des OEX-Dokuments<br>Neue Optionen:<br> `NUPPER` – Großbuchstaben und Zahlen<br>betroffen:`aTypeDis`, `aTypeSur`, `_Unit`, `_VendorID`, `_VendorSeries`<br>schränkt den Datentyp gemäß der jeweiligen Wertemenge genauer ein<br> `NLOWER` – Kleinbuchstaben und Zahlen<br>schränkt den Datentyp gemäß der jeweiligen Wertemenge genauer ein<br> `COUNT` – Anzahl Listelemente<br>folgende Attribute zwecks Präzisierung umgestellt:`aDocumentCount`, `aItemCount` <br>Geänderte Option:<br> `NUMCHAR` – Nummern und Buchstaben<br>Da hier auch Kleinbuchstaben erlaubt sind, wurden folgende Domänen zwecks Präzisierung auf die<br>Option`NUPPER` umgestellt:`_BankAccount`, `_BankKey`, `_Iban`, `_SwiftBic` <br>Präzisierte Option:<br> `POS` – Positionsnummerierung<br>Angabe der zulässigen Werte und weiterführende Beschreibung<br> <br>2.4 Attribute<br>Neu:<br> `aDateCalcBase` <br>– Kalkulationsbasis bei Datumsermittlung<br> `aDateCalcMode` <br>– Kalkulationsverfahren bei Datumsermittlung<br> `aLocale` <br> <br>– Gebietsschema<br> `aItemTypeClient` – Positionstyp der Kundenposition<br> `aItemTypeVendor` – Positionstyp der Lieferantenposition<br>Erweiterung:<br> `aDateFormat`<br>– Datumsformat<br>Neuer Wert:<br>`C` <br>– Kalendertage<br>Berichtigung:<br> `aEANType` <br>– EAN-Typ<br>Längenangabe berichtigt von 1 auf 6. (hat nicht mit der tatsächlichen Länge der Werte übereingestimmt)<br>Präzisierung:<br> `aStatus` <br> <br>– Artikelstatus<br>wurde präzisiert bezüglich wie sich ein Artikel zusammensetzt und wann ein Artikel als geändert gilt.<br>Außerdem wurde die Restriktion aus den Stati S und O entfernt, weil diese auf keine der bisher<br>vorhandenen Dokumententarten eingeschränkt sind, somit eine Aufzählung aller unnötig ist.<br> <br>3 OEX-Szenarien<br>Fallbeispiele zu Anfrage und Angebot|



                              - 62 

|Version|Änderungen|
|---|---|
|2.0.0 – 21.11.2008<br>(Forts. auf nächster<br>Seite)|1.1 Übersicht OEX-Spezifikationen<br>Neue Dokumentenarten:<br>`DESADV` – Lieferavis<br> <br> <br> <br> <br>`INVOIC` – Rechnung<br>Neue Minor-Versionen der Dokumentenarten:`ORDERS` – Bestellung<br> <br> <br> <br> <br> <br> <br>`ORDRSP` – Bestellbestätigung<br> <br> <br> <br> <br> <br> <br>`ORDCHG` – Bestelländerung<br> <br>1.3 Legende<br>Präzisierung der Legende bezüglich Wiederholbarkeit, Schlüsselelemente und Pflichtelemente<br> <br>2.1.2 OEX-Rahmentypen<br>Neu:<br> <br>2.1.2.18`DocNo`: Belegnummern<br> <br> <br>2.1.2.19`BankData`: Bankdaten<br> <br> <br>2.1.3 OEX-Werttypen<br>Neu:<br>`CommodCode` – Warennummer (INTRASTAT)<br> <br>`CountryOrigin` – Ursprungsland<br> <br>`CountyOrigin` – Ursprungsregion<br> <br>`CustomNumber` – Zollnummer<br> <br>`ShipmentNumber` – Transportnummer<br> <br>`ShipmentBase` – Transportgrundlage<br> <br>`DelivComplet` – Vollständigkeit der Lieferung<br> <br>`DeliveryNumber` – Liefernummer<br> <br>`DelivItemNo` – Nummer der Lieferposition<br> <br>`DelivQuantity` – Liefermenge<br> <br>`DelivTopLevel` – Übergeordnete Nummer der Lieferposition<br> <br>`DelivUnit` – Liefermengeneinheit<br> <br>`GrossWeight` – Bruttogewicht<br> <br>`Height` – Höhenangabe<br> <br>`Length` – Längenangabe<br> <br>`MeansTransp` – Förderhilfsmittel<br> <br>`TransportMode` – Verkehrszweig<br> <br>`MeasureUnit` – Maßeinheit<br> <br>`NetWeight` – Nettogewicht<br> <br>`NumPackage` – Anzahl Packstücke<br> <br>`NumArtPack` – Anzahl Artikel pro Packstück<br> <br>`PackageNumber` –Packstücknummer<br> <br>`PackedWithItem` – Verpackt mit Lieferposition<br> <br>`PackageType` – Verpackungsart<br> <br>`UnitVolume` – Volumeneinheit<br> <br>`UnitWeight` – Gewichtseinheit<br> <br>`Volumen` – Volumen<br> <br>`Width` – Breitenangabe<br> <br>`AccountHolder` – Kontoinhaber<br> <br>`BankAccount` – Kontonummer<br> <br>`BankCountry` – Land der Bank<br> <br>`BankKey` – Bankenschlüssel (Bankleitzahl)<br> <br>`BankLocation` – Sitz der Bank<br> <br>`BankName` – Name der Bank<br> <br>`Iban` – IBAN Internationale Kontonummer<br> <br>`SwiftBic` – SWIFT-BIC Int. Bankenschlüssel<br> <br>`DocNo` – Belegnummer<br> <br>`DocNoType` – Belegnummernart<br> <br>`DocLine` – Belegposition<br> <br>`OrdConfCompNo` – Nummer der Bestellbestätigungsposition des kompositen Artikels<br> <br>`OrderCompNo` – Nummer der Bestellposition des kompositen Artikels<br> <br>`OrderType` – Auftragsart<br> <br>`InvoiceNumber` – Rechnungsnummer<br> <br>`InvoiItemNo` – Nummer der Rechnungsposition<br> <br>`InvoiTopLevel` – Übergeordnete Nummer der Rechnungsposition<br> <br>`InvoiceType` – Rechnungsart<br> <br>`InvoiQuantity` – Rechnungsmenge<br> <br>`InvoiUnit` – Rechnungsmengeneinheit<br> <br>`ConditionText` – Konditionsbezeichnung<br>|



                              - 63 

|Version|Änderungen|
|---|---|
|2.0.0 – 21.11.2008<br>(Fortsetzung)|2.2 Datendomänen<br>Neu:<br>`_CommodCode` – Warennummer (INTRASTAT)<br> <br>`_DelivComplet` – Vollständigkeit der Lieferung<br> <br>`_Integer` – Integerwert<br> <br>`_MeansTransp` – Förderhilfsmittel<br> <br>`_TransportMode` – Verkehrszweig<br> <br>`_PackageType` – Verpackungsart<br> <br>`_ShipmentBase` – Transportgrundlage<br> <br>`_AccountHolder` – Kontoinhaber<br> <br>`_BankAccount` – Bankkontonummer<br> <br>`_BankKey` – Bankenschlüssel (Bankleitzahl)<br> <br>`_Iban` – IBAN Internationale Bankkontonummer<br> <br>`_SwiftBic` – SWIFT-BIC Int. Bankenschlüssel<br> <br>`_DocNoType` – Belegnummernart<br> <br>`_OrderType` – Auftragsart<br> <br>`_InvoiceType` – Rechnungsart<br>Erweiterung:`_OrgDataType` – Arten Organisationsdaten<br> <br> <br>Neue Werte:<br>`PRI` – Verarbeitungskennzeichen<br> <br> <br> <br>`PLO` – Ladestelle<br> <br> <br> <br>`TOU` – Tour<br> <br> <br>Gelöschte Werte:<br>`CON` – Rahmenvernummer neu in`_DocNoType (CON)` <br> <br> <br> <br>`TAN` – Vorgangsnummer neu in`_DocNoType (TAN)` <br> <br> <br> <br>`RQN` – Anfragenummer neu in`_DocNoType (REQ)` <br> <br> <br> <br>`QTN` – Angebotsnummer neu in`_DocNoType (QUO)` <br> <br>`_ConditionType` – Konditionarten<br> <br> <br>Neue Werte:<br>`SUBH` – Zwischensumme auf Kopfebene <br> <br> <br> <br>`SUBI` – Zwischensumme auf Positions- und/oder Kopfebene<br> <br> <br> <br>`TOTL` – Endbetrag<br> <br> <br> <br>`TTNE` – Steuernetto<br> <br> <br>Neue Attribute:<br>`aTypeDis` – Art des Abschlags<br> <br> <br> <br>`aTypeSur` – Art des Zuschlags<br> <br> <br>Neue Restriktionen:`TS` – Art des Zuschlags erforderlich<br> <br> <br> <br>`TD` – Art des Abschlags erforderlich<br> <br>`_TextLine` – Textzeile<br> <br> <br>Neues Attribut:<br>`aLineFormat` – Zeilenformat<br> <br>`_ComType` – Kommunikationsarten<br> <br> <br>Neues Attribut:<br>`aScopeInfo` – Anwendungsbereich der Information<br> <br>`_TextType` – Textarten<br> <br> <br>Neue Werte:<br>`PRMD` – Abwicklungsmodalitäten <br> <br> <br> <br>`INOT` – Montagehinweise<br> <br>`_DateTimeType` – Typ Datum und Zeit<br> <br> <br>Neue Werte:<br>`REQ` – Anfragedatum <br> <br> <br> <br> <br> <br> <br>`QUO` – Angebotsdatum <br> <br> <br> <br>`DES` – Lieferavisdatum<br> <br> <br> <br>`DND` – Lieferscheindatum<br> <br> <br> <br>`INV` – Rechnungsdatum<br> <br> <br> <br>`DUE` – Fälligkeitsdatum<br> <br> <br> <br>`DSR` – Leistungserstellungsdatum<br> <br>`_AddressType` – Adressarten<br> <br> <br>Neuer Wert:<br>`IS` – Montagefirma (Montage vor Ort) <br> <br> <br>`_ContactType` – Kontaktarten<br> <br> <br> <br>Neuer Wert:<br>`IN` – Monteur <br> <br> <br>2.3 Datentypen<br>Erweiterung:`CHAR` – Alle Zeichen der zugrunde gelegten Code-Page des OEX-Dokuments<br> <br> <br>Neue Optionen:<br>`XUPPER` – Großbuchstaben und andere<br> <br> <br> <br>`XLOWER` – Kleinbuchstaben und andere<br> <br> <br> <br>`NUMCHAR` – Nummern und Zeichen in Großbuchstaben<br> <br>|



                              - 64 

|Version|Änderungen|
|---|---|
|2.0.0 – 21.11.2008<br>(Fortsetzung)|2.4 Attribute<br>Neu:<br>`aTypeDis` – Art des Abschlags<br> <br>`aTypeSur` – Art des Zuschlags<br> <br>`aLineFormat` – Zeilenformat<br> <br>`aScopeInfo` – Anwendungsbereich der Information<br> <br>`aDocContext` – Dokumentenzusammenhang<br>Erweiterung:`aCondRef` – Konditionsbezug<br> <br> <br>Neue Restriktionen:<br>`TTNE` – Steuernetto<br> <br> <br> <br>`SUBH` – Zwischensumme auf Kopfebene<br> <br> <br> <br>`SUBI` – Zwischensumme auf Positions- und/oder Kopfebene<br> <br>Beispiele für OEX-Rahmentypen<br>Neu:<br>2.1.2.1`DocFrame`: OEX-Dokumentenrahmen<br> <br>2.1.2.2`Applic`: Applikation, die das OEX-Dokument erstellt hat<br> <br>2.1.2.3`File`: Dokumentenmappe<br> <br>2.1.2.4`Document`: Einzelnes Dokument<br> <br>2.1.2.5`Header`: Belegkopf<br> <br>2.1.2.6`Item`: Dokumentenposition<br> <br>2.1.2.8`OrgData`: Organisationsdaten<br> <br>2.1.2.9`Address`: Adressen<br> <br>2.1.2.10`Com`: Kommunikation<br> <br>2.1.2.11`Contact`: Ansprechpartner<br> <br>2.1.2.15`Config`: Konfigurationsdaten<br> <br>2.1.2.18`DocNo`: Belegnummern<br> <br>2.1.2.19`BankData`: Bankdaten<br>Erweiterung: 2.1.2.7`DateTime`: Datums- und Zeitangaben<br> <br>2.1.2.12`Text`: Texte<br> <br>2.1.2.13`Reference`: Verweise<br> <br>2.1.2.14`Pricing`: Preiskalkulation<br>|
|1.1.1 – 24.04.2007|1.1 Übersicht OEX-Spezifikationen<br>Neue Dokumentenart`ORDCHG` – Bestelländerung<br>Neue Build-Versionen der Dokumentarten`ORDERS` – Bestellung und`ORDRSP` – Bestellbestätigung<br> <br>2.1.2.11`Contact` – Ansprechpartner <br>Allgemeine Beschreibung zur Verwendung<br> <br>2.2 Datendomänen <br>Erweiterung:`_OrgDataType` – Arten Organisationsdaten<br> <br> <br>Neuer Wert:<br>`TRZ` – Transportzone<br> <br>`_ReferenceType` – Verweisarten<br> <br> <br>Neuer Wert:<br>`ATT` – Dateianhänge<br> <br>3 OEX-Szenarien<br>Fallbeispiele|
|1.1.0 – 18.09.2006<br>(Forts. auf nächster<br>Seite)<br>|Preiskalkulation: <br>Das bestehende Rahmenelement`Pricing` wurde strukturell noch mal überarbeitet, um mehr Optionen<br>abzudecken. U.a. sind nun beliebig viele Rabatte, Zuschläge und Steuern möglich.<br>Das Rahmenelement`Pricing` erhält nun als Attribut eine laufende Nummer`aCondNo` auf die andere<br>Konditionsarten referenzieren können (`aCondRef`). Es wird also nicht mehr wie vormals direkt auf eine<br>Konditionsart referenziert. Die Konditionsart`ConditionType` erhält zusätzlich zwei neue Attribute<br>`aCondSign` Zu- bzw. Abschlag und`aTaxCode` Steuerkennzeichen.<br>In diesem Zusammenhang gelten für den Typ`ConditionValue` nur noch absolute Werte und dessen<br>Attribut`aCondValType` (Wertetyp) entfällt. Dafür kommt der Typ`ConditionRate` Konditionssatz hinzu,<br>der prozentuale Werte beinhaltet. Somit ist es beispielsweise bei Rabatten und Steuern innerhalb der<br>Konditionsart möglich, beide Werte anzugeben. Die betroffenen Typen bzw. Domänen wurden angepasst.<br> <br>Konfigurationstexte:<br>Im bestehenden Rahmenelement`Configuration` wurde das Rahmenelement`ConfigText` <br>(Konfigurationstexte) eingefügt. Hiermit kann nach Bedarf und Fähigkeit der jeweiligen Applikation, die<br>komplette Konfiguration in ihren Elementen übermittelt werden.<br> <br>Texte:<br>Im bestehenden Rahmenelement`Text` wurde der Typ`TextLineNo` (Zeilennummer) entfernt und als<br>Attribut`aTextLineNo` von Typ`TextContent` (Zeileninhalt) hinzugefügt. Damit wird eine kompaktere<br>Darstellung der Texte erreicht.|




- 65 

|Version|Änderungen|
|---|---|
|1.1.0 – 18.09.2006<br>(Fortsetzung)|Datums- und Zeitangaben:<br>In das bestehende Rahmenelement`DateTime` wurde der Typ`TimeZone` (Zeitzone) hinzugefügt.<br>Das Attribut Zeitformat`aTimeFormat` ist entfallen, es gilt grundsätzlich das 24-Stunden-Format.<br> <br>Zahlungsbedingungen:<br>Neues Rahmenelement`Payment` für Zahlungsbedingungen.<br> <br>Neue Werte bei Domänen:<br>Arten Organisationsdaten (`_OrgDataType`) <br>`TAN` <br>Vorgangsnummer<br>`QTN` <br>Angebotsnummer<br>`RQN` <br>Anfragenummer<br> <br>Adressarten (_AddressType)<br>`BR` <br>Filiale (des Auftraggebers)<br> <br>Textarten (`_TextType`) <br>`ARTM` <br>Modifizierter Artikeltext im Zusammenhang mit Artikelstatus`aStatus`<br> <br>Artikelstatus:<br>Bisher wurde ein Sonderartikel über das Attribut`aSpecial`=“`Y`“ des Typs Lieferantenartikelnummer<br>`VendorArtNo` gekennzeichnet. Dieses Attribut wurde durch das Attribut Artikelstatus`aStatus` ersetzt und<br>dokumentiert zusätzlich die Herkunft und Modifikation eines Lieferantenartikels.<br> <br>Spezifikationsneuerungen:<br>GLOBAL als führendes Dokument bei Versionen aller Spezifikationen.<br>Attribute verweisen nun auch konsequenterweise auf einen Datentyp.<br>Character-Datentypen DATE und TIME laufen nun unter Datentyp CHAR(DATE) und CHAR(TIME).<br>Einige Elemente wurden detaillierte spezifiziert.|
|1.0.3 – 01.06.2006|XML-Schema:<br>Änderung: Pro Dokumentenart genau 1 Schema, es wird somit kein globales Schema implementiert.<br> <br>Versionierung: Build-Nummer für XSD-Dateien, ORDERS- und ORDRSP-Spezifikation eingeführt<br>`_DocumentType`  `aBuild`<br> <br>Legende anlog der anderen Spezifikationen eingefügt<br> <br>Maßeinheiten (`_Unit`) (bedingt durch Tests/Implementierung):<br>Analog OFML-OCD werden diese durch den Common Code der UN/ECE Recommendation 20 dargestellt.<br>Nicht wie ursprünglich angegeben durch ISO 31-0.<br> <br>Rahmensegment:`Pricing`: Preiskalkulation (bedingt durch Tests/Implementierung):<br>Neue Felder`<CondCurrency>` (Konditionswährung),`<PriceUnit>` (Preiseinheit) und`<MeasureUnit>` <br>(Mengeneinheit zur Preiseinheit) alle Optional<br> <br>Definitionsbeschreibung mit Beispiel ergänzt zwecks Klarstellung (bedingt durch Tests/Implementierung):<br>Rahmensegment:`Text`: Texte<br> <br>Rahmensegment:`Config`: Konfigurationsdaten<br>Änderung: Feld`<ClassID>` (Merkmalsklasse) von Pflicht nach Optional<br> <br>Definitionsbeschreibung ergänzt zwecks Klarstellung:<br>Datendomäne`_VendorArtNo` – Lieferantenartikelnummer<br> <br>Kleinere Textkorrekturen und Ergänzungen (u.a. Beispiele bei ISO Datendomänen)|
|1.0.2 – 10.03.2006|Änderungshistorie hinzugefügt<br> <br>Kleinere textliche Korrekturen<br> <br>Erweiterungen bei`_OrgDataType` – Arten Organisationsdaten|



                              - 66 

|Version|Änderungen|
|---|---|
|1.0.1 – 27.01.2006|Hinzugefügte Werttypen für Dokumentenart ORDRSP:<br>`OrdConfirmNo` <br>Bestellbestätigungsnummer<br>`OrdConfItemNo` <br>Bestellbestätigungsposition<br>`OrdConfTopLevl` <br>Übergeordnete Bestellbestätigungsposition<br>`ConfOrdQuant` <br>Bestätigte Bestellmenge<br>`ConfOrdUnit` <br>Bestätigte Bestellmengeneinheit<br> <br>Erweiterte Domäne für Dokumentenart ORDRSP:<br>`DateTimeType` <br>DLD – Liefertermin des Lieferanten|
|1.0.0 – 18.01.2006|Initialversion|



                              - 67 

