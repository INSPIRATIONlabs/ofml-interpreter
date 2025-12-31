# Spezifikation OEX OFML Business Data Exchange
## (OFML Part VII)

# **DESADV**
## Lieferavis

# Version 3.1.0

Editoren:
Markus Behrschmidt, Vitra Services GmbH

Thomas Gerth, EasternGraphics GmbH


8.5.2023


Copyright © 2008 – 2023 Industrieverband Büro und Arbeitswelt e. V. (IBA)


# Inhalt

**1** **Einleitung .............................................................................................................. 3**

1.1 Verwendung dieser Spezifikation ............................................................................ 3

1.2 Dateinamenkonventionen ....................................................................................... 3

1.3 XML-Deklaration ..................................................................................................... 4

1.4 Prüfmechanismen ................................................................................................... 4

1.5 Vollständigkeit des Dokumentes ............................................................................. 4

1.6 Legende .................................................................................................................. 5


**2** **Definitionen ........................................................................................................... 6**

2.1 Übergeordnete Spezifikation .................................................................................. 6

2.2 Dokumentenartbezogene Spezifikationen .............................................................. 6


**3** **Struktur .................................................................................................................. 7**

3.1 Übersicht Dokumentenstruktur ............................................................................... 7

3.2 Rahmenelement `oexDocFrame` - OEX Dokumentenrahmen ................................ 8

3.3 Rahmenelement `oexApplication` - Applikation, die das Dokument erstellt ....... 8

3.4 Rahmenelement `oexFile` - Dokumentenmappe .................................................. 8

3.5 Rahmenelement `oexDocument` - Einzelnes Dokument ........................................ 9

3.6 Rahmenelement `docHeader` - Belegkopf Lieferavis ............................................. 9

3.7 Rahmenelement `hdrDateTime` - Kopf Lieferavis: Datums- und Zeitangaben .... 10

3.8 Rahmenelement `hdrOrgData` - Kopf Lieferavis: Organisationsdaten ................ 10

3.9 Rahmenelement `hdrAddress` - Kopf Lieferavis: Adressen ................................ 10

3.10 Rahmenelement `hdrCom` - Kopf Lieferavis: Kommunikation ............................... 11

3.11 Rahmenelement `hdrContact` - Kopf Lieferavis: Ansprechpartner ..................... 12

3.12 Rahmenelement `hdrText` - Kopf Lieferavis: Texte ............................................. 12

3.13 Rahmenelement `hdrReference` - Kopf Lieferavis: Verweise ............................ 12

3.14 Rahmenelement `hdrPricing` - Kopf Lieferavis: Preiskalkulation ...................... 13

3.15 Rahmenelement `docItem` - Belegposition Lieferavis .......................................... 14

3.16 Rahmenelement `itmDocument` - Einzelnes Dokument ...................................... 14

3.17 Rahmenelement `itmHeader` - Belegkopf Lieferung ........................................... 14

3.18 Rahmenelement `hdrDocNo` - Kopf Lieferung: Belegnummern ............................ 15

3.19 Rahmenelement `hdrDateTime` - Kopf Lieferung: Datums- und Zeitangaben .... 16

3.20 Rahmenelement `hdrOrgData` - Kopf Lieferung: Organisationsdaten ................. 16


                                  - 1 

3.21 Rahmenelement `hdrAddress` - Kopf Lieferung: Adressen ................................. 17

3.22 Rahmenelement `hdrCom` - Kopf Lieferung: Kommunikation ................................ 17

3.23 Rahmenelement `hdrContact` - Kopf Lieferung: Ansprechpartner ..................... 18

3.24 Rahmenelement `hdrText` - Kopf Lieferung: Texte ............................................. 18

3.25 Rahmenelement `hdrReference` - Kopf Lieferung: Verweise ............................. 18

3.26 Rahmenelement `hdrPricing` - Kopf Lieferung: Preiskalkulation ....................... 19

3.27 Rahmenelement `hdrPayment` - Kopf Lieferung: Zahlungsbedingungen ............ 20

3.28 Rahmenelement `docItem` - Belegposition Lieferung .......................................... 20

3.29 Rahmenelement `itmConfiguration` - Position Lieferung: Konfiguration ........ 21

3.30 Rahmenelement `itmConfigText` - Position Lieferung: Konfigurationstexte ..... 22

3.31 Rahmenelement `itmDocNo` - Position Lieferung: Belegnummern ...................... 22

3.32 Rahmenelement `itmDateTime` - Position Lieferung: Datums- u. Zeitangaben .. 22

3.33 Rahmenelement `itmOrgData` - Position Lieferung: Organisationsdaten ........... 23

3.34 Rahmenelement `itmAddress` - Position Lieferung: Adressen ........................... 23

3.35 Rahmenelement `itmCom` - Position Lieferung: Kommunikation .......................... 24

3.36 Rahmenelement `itmContact` - Position Lieferung: Ansprechpartner ................ 24

3.37 Rahmenelement `itmText` - Position Lieferung: Texte ........................................ 24

3.38 Rahmenelement `itmReference` - Position Lieferung: Verweise ....................... 25

3.39 Rahmenelement `itmPricing` - Position Lieferung: Preiskalkulation ................. 25


**4** **Anhang ................................................................................................................ 27**

4.1 Änderungshistorie ................................................................................................. 27


                                  - 2 

### **1 Einleitung**

Diese Spezifikation definiert alle Elemente, die für die Beschreibung eines Lieferavis verwendet werden.
Der Lieferavis definiert einen Transport, der sich entweder aus Bestellungen (Aufträgen) mit lieferrelevanten
Positionen oder aus bereits erstellten Lieferungen zu einem Kunden zusammensetzt.
Der Zeitpunkt des Versandes des Lieferavis erfolgt vor der physischen Lieferung der Ware und soll dem
Warenempfänger die Möglichkeit geben, den Warenempfang (Wareneingang) entsprechend zu organisieren
und zu koordinieren.

Mitgeltende Spezifikationen (in der jeweils gültigen Version, siehe 2.1):


OEX-GLOBAL – dokumentenartübergreifende Spezifikation

Verwandte Dokumentenarten/Spezifikationen:


Anfrage (OEX-REQOTE), Angebot (OEX-QUOTES), Bestellung (OEX-ORDERS), Bestellbestätigung (OEXORDRSP), Bestelländerung (OEX-ORDCHG) und Rechnung (OEX-INVOIC).


**1.1** **Verwendung dieser Spezifikation**


In dieser Spezifikation werden speziell die Strukturen und Elemente für die Dokumentenart “DESADV Lieferavis“ beschrieben. Globale Strukturen und Elemente, die auch in anderen Dokumentenarten
Verwendung finden, werden im Detail in der übergeordneten Spezifikation “OEX-GLOBAL“ in der
korrespondierenden Version beschrieben. Nur von dort abgeleitete sowie dokumentenartbezogene
Strukturen und Elemente werden hier in dieser Spezifikation beschrieben (siehe auch 0 und 0).


**1.2** **Dateinamenkonventionen**
Als Dateinamenkonvention für die Dokumentenart “DESADV“ gilt:

```
oex-desadv_<sender-id>_jjjjmmtt-hhmmss.xml

```

Grundlage bilden hier also die Dokumentenart sowie Datum und Zeit (24-Stunden-Format) der Erstellung der
Datei. Die Dateierweiterung lautet “ `xml` “.
`<sender-id>` ist der variable Teil des Dateinamens, der vom Sender der Datei vergeben werden muss. Die
max. Länge beträgt 20 Zeichen. Hierbei kann es sich beispielsweise um eine fortlaufende Nummerierung
des Senders handeln, der Kundennummer oder der Lieferantennummer.
Nur Ziffern, Buchstaben und Bindestriche sind hierbei erlaubt.
Mit diesen Angaben können dann auch gerade in einem Fehlerfall Rückschlüsse gezogen werden, um was
für eine Art der Datei es sich handelt, von wem sie kommt und wann sie erstellt wurde.

Beispiele: `oex-desadv_A000034956_20080124-132702.xml`
```
      oex-desadv_TP-9564154_20080517-083527.xml

```

                                  - 3 

**1.3** **XML-Deklaration**

XML Version und Code Page

Siehe dokumentenartübergreifende Spezifikation OEX-GLOBAL.


XML Schema (XS) Einbindung

Die Einbindung des dokumentenartbezogenen Schemas erfolgt über die für XML-Schemata festgelegten
Attribute im Rahmenelement `oexDocFrame` :

```
<oexDocFrame aMajor="3"
xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xsi:noNamespaceSchemaLocation="oex-desadv_<Major>.<Minor>.<Build>.xsd">

```

Die Einbindung des übergeordneten Schemas ( `oex-global` ) ist bereits im dokumentenartbezogenen
Schema definiert.


Gültige Version des XML Schemas


Für diese Spezifikation gilt das dokumentenartbezogene Schema in der Version 3.1.0
**`oex-desadv_3.0.0.xsd`** bzw. bei Änderungen des Schemas ohne Auswirkung auf diese Spezifikation,
das Schema mit der höchsten Build-Nummer.


**1.4** **Prüfmechanismen**


Bei der Verwendung entsprechender XML-Parser, kann zur Prüfung eines OEX-DESADV-Dokuments (XMLDatei) das jeweils gültige XML Schema (XS) verwendet werden.
Das Schema wird von den entsprechenden Spezifikationen abgeleitet und als Prüftool bezüglich der
Elementstruktur und Datendefinition bereitgestellt. Darüber hinausgehende Prüfungen auf logische Inhalte
und Abhängigkeiten, sowie ein Mapping der Daten unterliegen der jeweils verwendeten Applikation.


**1.5** **Vollständigkeit des Dokumentes**
Das Dokument wird grundsätzlich vollständig übertragen, d.h. auch mit Positionen (oder Daten), die keine
Änderungen beinhalten ( `aAction = N` ), insbesondere, wenn das Dokument wegen Änderungen nochmal
übermittelt wird.


                                  - 4 

**1.6** **Legende**


Erläuterung spezieller Spalten, die in den Tabellen im Abschnitt “Struktur“ Verwendung finden.

|Spalte|Bezeichnung|Werte|Bedeutung|
|---|---|---|---|
|**`Wdh`**|Wiederholbarkeit|**1 **|Element kann genau einmal vorkommen.|
|**`Wdh`**|Wiederholbarkeit|**#+**|Element muss mehrfach bis zu der Zahl<br>vorkommen, die über den Platzhalter #<br>angegeben wird, darüber hinaus kann es<br>mehrfach vorkommen.<br>(Bsp.: 1+ = 1 mal muss, mehrmals kann)|
|**`Wdh`**|Wiederholbarkeit|**#***|Element kann keinmal bzw. mehrfach bis zu der<br>Zahl vorkommen, die über den Platzhalter #<br>angegeben wird. Wenn das Element ein<br>Pflichtelement ist, muss es mind. einmal<br>vorkommen. (Bsp.: 3* = 1 bis 3 mal)|
|**`Wdh`**|Wiederholbarkeit|*** **|Element kann keinmal bis mehrfach vorkommen.<br>Wenn das Element ein Pflichtelement ist, muss<br>es mind. einmal vorkommen.|
|**`Pfl`**|Pflichtelement|**<empty>**|Element kann vorhanden sein, muss aber dann<br>auch einen Wert beinhalten.|
|**`Pfl`**|Pflichtelement|**X **|Element muss vorhanden sein und einen Wert<br>beinhalten.|
|**`Pfl`**|Pflichtelement|**# **|Element kann vorhanden sein, muss dann aber<br>auch einen Wert beinhalten, der Platzhalter**#** gibt<br>eine fortlaufende Nummer beginnend bei 1<br>innerhalb eines Rahmenelements für<br>Unterelemente an, die einander bedingen und<br>i.d.R. gemeinsam angegeben werden müssen.<br>(bspw. Menge und Mengeneinheit)|
|**`Sch`**|Schlüsselelement|**! **|Element muss vorhanden sein und einen Wert<br>beinhalten, außerdem muss das Element<br>zusammen mit Wert und ggfs. speziell ange-<br>gebenen Pflichtattributen eindeutig bei Wieder-<br>holungen innerhalb eines Rahmenelements sein.<br>Sind mehrere Elemente so gekennzeichnet,<br>bilden sie zusammen einen eindeutigen Wert.<br>(Wirkung wie bei einem Primärschlüssel)|
|**`Mod`**|Modifikation|**<empty>**|Element ist dokumentartbezogen und/oder<br>verweist auf den angegebenen Typ aus der<br>übergeordneten Spezifikation.|
|**`Mod`**|Modifikation|**D **|Element leitet sich vom angegebenen Typ aus<br>der übergeordneten Spezifikation ab und ist<br>dokumentenartbezogen angepasst<br>(abgeleiteter Typ)|



Elemente, die als optional gekennzeichnet sind (Pfl=<empty>), müssen auf Grund gesetzlicher
Gegebenheiten bezüglich des Geschäftsfalls ggfs. bestückt werden (abhängig von Land und/oder
Region/Bundesland/Bundesstaat).


                                  - 5 

### **2 Definitionen**

**2.1** **Übergeordnete Spezifikation**


Die dokumentenartübergreifenden Spezifikationen sind dem Dokument **OEX-GLOBAL** in der jeweiligen
gültigen Version 3.1.x zu entnehmen, wobei „x“ für die höchste Build-Versionsnummer steht.


**2.2** **Dokumentenartbezogene Spezifikationen**
Spezifikation des Dokuments “DESADV“ – Lieferavis

**Versionierung**


Diese Spezifikation liegt in der Version 3.1.0 vor:


Major **3** .1.0
Minor 3. **1** .0
Build 3.1. **0**


Detaillierte Erläuterungen zur Versionierung sind in der übergeordneten Spezifikation (OEX-GLOBAL)
ersichtlich.

**Wiederholbarkeit, Pflicht- und Schlüsselelemente**


Eigenschaften der Elemente wie Wiederholbarkeit, Pflicht- und Schlüsselelement können dokumentenartbezogen gesetzt werden und bedeuten keine Ableitung auf die verwiesenen Typen bzw. Domänen aus
der übergeordneten Spezifikation (OEX-GLOBAL).

**Abgeleitete Elementtypen**


Als “abgeleitet“ wird ein Elementtyp bezeichnet, wenn er sich entgegen seiner übergeordneten Spezifikation
(OEX-GLOBAL) auf bestimmte Werte, Attribute und/oder Unterelemente einschränkt.


                                  - 6 

### **3 Struktur**

**3.1** **Übersicht Dokumentenstruktur**


Struktur der Rahmenelemente

```
<XML-Deklaration>
```

`oexDocFrame` OEX Dokumentenrahmen
`├─── oexApplication` Applikation, die das Dokument erstellt hat
`└─── oexFile` Dokumentenmappe
`└─── oexDocument` Einzelnes Dokument
`├─── docHeader` Belegkopf Lieferavis
`│` `├─── hdrDateTime` Kopf Lieferavis: Datums- und Zeitangaben
`│` `├─── hdrOrgData` Kopf Lieferavis: Organisationsdaten
`│` `├─── hdrAddress` Kopf Lieferavis: Adressen
`│` `│` `├─── hdrCom` Kopf Lieferavis: Kommunikation
`│` `│` `└─── hdrContact` Kopf Lieferavis: Ansprechpartner
`│` `│` `└─── hdrCom` Kopf Lieferavis: Kommunikation
`│` `├─── hdrText` Kopf Lieferavis: Texte
`│` `├─── hdrReference` Kopf Lieferavis: Verweise
`│` `└─── hdrPricing` Kopf Lieferavis: Preiskalkulation
`└─── docItem` Belegposition Lieferavis
`└─── itmDocument` Einzelnes Dokument
`├─── itmHeader` Belegkopf Lieferung
`│` `├─── hdrDocNo` Kopf Lieferung: Belegnummern
`│` `├─── hdrDateTime` Kopf Lieferung: Datums- und Zeitangaben
`│` `├─── hdrOrgData` Kopf Lieferung: Organisationsdaten
`│` `├─── hdrAddress` Kopf Lieferung: Adressen
`│` `│` `├─── hdrCom` Kopf Lieferung: Kommunikation
`│` `│` `└─── hdrContact` Kopf Lieferung: Ansprechpartner
`│` `│` `└─── hdrCom` Kopf Lieferung: Kommunikation
`│` `├─── hdrText` Kopf Lieferung: Texte
`│` `├─── hdrReference` Kopf Lieferung: Verweise
`│` `├─── hdrPricing` Kopf Lieferung: Preiskalkulation
`│` `└─── hdrPayment` Kopf Lieferung: Zahlungsbedingungen
`└─── docItem` Belegposition Lieferung
`├─── itmConfiguration` Position Lieferung: Konfigurationsdaten
`│` `└─── itmConfigText` Position Lieferung: Konfigurationstexte
`├─── itmDocNo` Position Lieferung: Belegnummern
`├─── itmDateTime` Position Lieferung: Datums- und Zeitangabe
`├─── itmOrgData` Position Lieferung: Organisationsdaten
`├─── itmAddress` Position Lieferung: Adressen
`│` `├─── itmCom` Position Lieferung: Kommunikation
`│` `└─── itmContact` Position Lieferung: Ansprechpartner
`│` `└─── itmCom` Position Lieferung: Kommunikation
`├─── itmText` Position Lieferung: Texte
`├─── itmReference` Position Lieferung: Verweise
`└─── itmPricing` Position Lieferung: Preiskalkulation


                                  - 7 

**3.2** **Rahmenelement** **`oexDocFrame`** **– OEX Dokumentenrahmen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`oexDocFrame`**|**`DocFrame`**|**1 **|**X **|||**OEX Dokumentenrahmen**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`oexApplication`**|**`Applic`**|**1 **|**X **|||**Applikation, die das Dokument**<br>**erstellt hat**|
|~~**`oexFile`**~~|~~**`File`**~~|**1 **|**X **|||**Dokumentenmappe**|



**3.3** **Rahmenelement** **`oexApplication`** **– Applikation, die das Dokument erstellt**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`oexApplication`**|**`Applic`**|**1 **|**X **|||**Applikation, die das Dokument**<br>**erstellt hat**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vAppName`**|**`Value`**|**1 **|**X **|||**Applikationsname**|
|**`eAppVersion`**|**`AppVersion`**|**1 **|**X **|||**Version der Applikation**|



**3.4** **Rahmenelement** **`oexFile`** **– Dokumentenmappe**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`oexFile`**|**`File`**|**1 **|**X **|||**Dokumentenmappe**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDocumentType`**|**`DocumentType`**|**1 **|**X **||**D **|**Dokumentenart**|
|**`vDocumentType`**|**Attribut**|**Attribut**|||||
|**`vDocumentType`**|`aMajor`|`aMajor`|X|||Major Versionsnummer|
|**`vDocumentType`**|`aMinor`|`aMinor`|X|||Minor Versionsnummer|
|**`vDocumentType`**|`aBuild`|`aBuild`|X|||Build Versionsnummer|
|**`vDocumentType`**|**Wertetabelle**|**Wertetabelle**|||**D **||
|**`vDocumentType`**|`DESADV`|`DESADV`||||Lieferavis|
|~~**`oexDocument`**~~|~~**`Document`**~~|**1+**|**X **|||**Einzelnes Dokument**|



                                  - 8 

**3.5** **Rahmenelement** **`oexDocument`** **– Einzelnes Dokument**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`oexDocument`**~~|~~**`Document`**~~|**1+**|**X **|**! **||**Einzelnes Dokument**<br>_(in Bezug auf die Dokumentenart)_|
|~~**`oexDocument`**~~|**Attribut**|**Attribut**|||||
|~~**`oexDocument`**~~|`aDocNo`|`aDocNo`|X|!||Laufende Nummer des Dokuments|
|~~**`oexDocument`**~~|`aItemCount`|`aItemCount`|X|||Anzahl der Positionen im Dokument|
|~~**`oexDocument`**~~|`aAction`|`aAction`|X|||Aktion|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`docHeader`**|**`Header`**|**1 **|**X **|||**Belegkopf Lieferavis**|
|~~**`docItem`**~~|~~**`Item`**~~|**1+**|**X **|||**Belegposition Lieferavis**|



**3.6** **Rahmenelement** **`docHeader`** **– Belegkopf Lieferavis**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`docHeader`**|**`Header`**|**1 **|**X **|||**Belegkopf**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vShipmentNumber`**|**`DocNo`**|**1 **|**X **|||**Transportnummer**<br>_(auch Nummer des Lieferavis)_<br>_Eindeutige Nummer des Lieferavis des_<br>_Lieferanten._|
|**`vClientNumber`**|**`Value`**|**1 **|**X **|||**Kundennummer**<br>_Nummer, unter der der Lieferant den_<br>_Besteller (Kunde) führt._|
|**`vClientID`**|**`ClientID`**|*** **||||**Kunden-ID**|
|**`vClientClass`**|**`ClientClass`**|*** **||||**Kunden-Klassifizierung**|
|**`vVendorNumber`**|**`Value`**|**1 **|**X **|||**Lieferantennummer**<br>_Nummer, unter der der Besteller_<br>_(Kunde) den Lieferanten führt._|
|**`vSupplierID`**|**`SupplierID`**|*** **||||**Lieferanten-ID**|
|**`vSupplierClass`**|**`SupplierClass`**|*** **||||**Lieferanten-Klassifizierung**|
|**`vDocCurrency`**|**`DocCurrency`**|**1 **|**X **|||**Belegwährung**|
|**`vIncoTerm`**|**`IncoTerm`**|**1 **|**1 **|||**Inco Terms (Lieferbedingung)**<br>_Anders lautende Lieferbedingungen_<br>_können über den Kopftext “Liefer-_<br>_bedingungen“ übergeben werden._|
|**`vIncoTermLocation`**|**`IncoTermLoc`**|**1 **|**1 **|||**Ortsangabe zu Inco Terms**|
|**`vDocLanguage`**|**`DocLanguage`**|**1 **|**X **|||**Belegsprache**|
|**`vShipmentBase`**|**`ShipmentBase`**|**1 **|**X **|||**Transportgrundlage**|
|**`vNumPackages`**|**`NumPackages`**|**1 **||||**Anzahl Packstücke (Colli)**|
|**`vGrossWeight`**|**`GrossWeight`**|**1 **|**2 **|||**Bruttogewicht (gesamt)**|
|**`vNetWeight`**|**`NetWeight`**|**1 **|**2 **|||**Nettogewicht (gesamt)**|
|**`vUnitWeight`**|**`UnitWeight`**|**1 **|**2 **|||**Gewichtseinheit**|
|**`vVolume`**|**`Volume`**|**1 **|**3 **|||**Volumen (gesamt)**|
|**`vUnitVolume`**|**`UnitVolume`**|**1 **|**3 **|||**Volumeneinheit**|
|**`vTransportMode`**|**`TransportMode`**|**1 **||||**Verkehrszweig**|
|**`vCustomNumber`**|**`CustomNumber`**|**1 **||||**Zollnummer**|
|~~**`hdrDateTime`**~~|~~**`DateTime`**~~|**3+**|**X **|||**Kopf Lieferavis: Datums- u. Zeitang.**|
|~~**`hdrOrgData`**~~|~~**`OrgData`**~~|*** **||||**Kopf Lieferavis: Organisationsdaten**|
|~~**`hdrAddress`**~~|~~**`Address`**~~|*** **||||**Kopf Lieferavis: Adressen**|
|~~**`hdrText`**~~|~~**`Text`**~~|*** **||||**Kopf Lieferavis: Texte**|



                                  - 9 

|hdrReference|Reference|*|Col4|Col5|Col6|Kopf Lieferavis: Verweise|
|---|---|---|---|---|---|---|
|~~**`hdrPricing`**~~|~~**`Pricing`**~~|*** **|||**D **|**Kopf Lieferavis: Preiskalkulation**|


Der Belegkopf enthält alle wichtigen Referenzen des Dokuments.

Erläuterungen zu Pflichtangaben:


**1** Die Ortsangabe zu Inco Terms muss angegeben werden, sobald die Lieferbedingung dies fordert.
**2** Die Gewichtseinheit muss angegeben werden, sobald Bruttogewicht und/oder Nettogewicht
angegeben wird.
**3** Die Volumeneinheit muss angegeben werden, sobald das Volumen angegeben wird.


**3.7** **Rahmenelement** **`hdrDateTime`** **– Kopf Lieferavis: Datums- und Zeitangaben**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrDateTime`**|**`DateTime`**|**3+**|**X **|**! **||**Kopf Lieferavis: Datum und Zeit**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDateTimeType`**|**`DateTimeType`**|**1 **|**X **|**! **||**Typ Datum/Zeit**|
|**`vTimeZone`**|**`TimeZone`**|**1 **|**X **|||**Zeitzone**|
|**`vDateValue`**|**`Date`**|**1 **|**X **|||**Datumsangabe**|
|**`vTimeValue`**|**`Time`**|**1 **||||**Zeitangabe**|



Dieses Rahmenelement dient zur Übergabe von Datums- und Zeitangaben eines Lieferavis.
Als Pflichtangaben sind hier das Belegdatum ( `DOC` ) und Lieferavisdatum ( `DES` ), sowie Lieferdatum ( `DLD` )
oder Fixlieferdatum ( `FXD` ) erforderlich.


**3.8** **Rahmenelement** **`hdrOrgData`** **– Kopf Lieferavis: Organisationsdaten**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrOrgData`**|**`OrgData`**|*** **||**! **||**Kopf Lieferavis: Organisationsdaten**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vOrgDataType`**|**`OrgDataType`**|**1 **|**X **|**! **||**Arten Organisationsdaten**|
|**`vOrgDataValue`**|**`Value`**|**1 **|**X **|||**Wert Organisationsdaten**|



Verwendungsbeispiele: Tour ( `TOU` ) `"T21"`
Lieferndes Werk ( `DPL` ) `"W0005"`

**3.9** **Rahmenelement** **`hdrAddress`** **– Kopf Lieferavis: Adressen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrAddress`**|**`Address`**|**1+**|**X **|**! **||**Kopf Lieferavis: Adressen**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vAddressType`**|**`AddressType`**|**1 **|**X **|**! **||**Typ Adresse**|
|**`vAddressNumber`**|**`Value`**|**1 **||||**Adress-Nummer**|
|**`vAddressID`**|**`AddressID`**|*** **||||**Adress-ID**|
|**`vTitle`**|**`Value`**|**1 **||||**Anrede**|
|**`vName1`**|**`Name1`**|**1 **|**X **|||**Name 1**|
|**`vName2`**|**`Name2`**|**1 **||||**Name 2**|



                               - 10 

|vName3|Name3|1|Col4|Col5|Col6|Name 3|
|---|---|---|---|---|---|---|
|**`vName4`**|**`Name4`**|**1 **||||**Name 4**|
|**`vStreet`**|**`Street`**|**1 **|**X **|||**Straße**|
|**`vStreetNo`**|**`Value`**|**1 **||||**Straßennummer**|
|**`vStreet2`**|**`Street2`**|**1 **||||**Straße 2**|
|**`vCountryCode`**|**`CountryCode`**|**1 **|**X **|||**Länderkennzeichen**|
|**`vPostalCode`**|**`PostalCode`**|**1 **|**X **|||**Postleitzahl**|
|**`vLocation`**|**`Location`**|**1 **|**X **|||**Ort**|
|**`vDistrict`**|**`District`**|**1 **||||**Ortsteil**|
|**`vCountyCode`**|**`CountyCode`**|**1 **||||**Region/Bundesland/-Staat**|
|**`vPostalCodePOBox`**|**`PostalCodePOB`**|**1 **||||**Postleitzahl Postfach**|
|**`vPOBox`**|**`Value`**|**1 **||||**Postfachnummer**|
|**`vTaxCode`**|**`Value`**|**1 **||||**Steuernummer Finanzamt**|
|**`vTaxCodeEU`**|**`Value`**|**1 **||||**Steuernummer EU / USt-IdNr.**|
|**`vTaxCodeUS`**|**`Value`**|**1 **||||**Steuernummer US / Jurisdiction**|
|~~**`hdrCom`**~~|~~**`Com`**~~|*** **||||**Kopf Lieferavis: Kommunikation**|
|~~**`hdrContact`**~~|~~**`Contact`**~~|*** **||||**Kopf Lieferavis: Ansprechpartner**|


Die Anlieferadresse (SH) kann hier angegeben werden, sofern der gesamte Transport dort hin geliefert wird.
In der Regel sind die Adressen insbesondere Auftraggeber (SO) und Lieferant (SU) den beiden
Geschäftspartnern bekannt und als Stammdaten hinterlegt und bedürfen nicht unbedingt einer Übertragung,
sie korrespondieren auch zur Kundennummer bzw. Lieferantennummer des Belegkopfes ( `docHeader` ).
Ggf. wird mit einer Adresse aber auch eine für die Transportabwicklung zuständige Kontaktperson
übermittelt.


**3.10** **Rahmenelement** **`hdrCom`** **– Kopf Lieferavis: Kommunikation**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrCom`**|**`Com`**|*** **||**! **||**Kopf: Kommunikation**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vComType`**|**`ComType`**|**1 **|**X **|**! **||**Art der Kommunikation**|
|**`vComType`**|**Attribut**|**Attribut**|||||
|**`vComType`**|`aScopeInfo`|`aScopeInfo`|**X **|**! **||Anwendungsbereich der Information|
|**`vComValue`**|**`Value`**|**1 **|**X **|||**Wert Kommunikation**|



Zur Angabe von Telefon, Telefax, Email etc. zur Adresse und/oder zum Ansprechpartner.


                               - 11 

**3.11** **Rahmenelement** **`hdrContact`** **– Kopf Lieferavis: Ansprechpartner**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrContact`**|**`Contact`**|*** **||||**Kopf Lieferavis: Ansprechpartner**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vContactType`**|**`ContactType`**|**1 **|**X **|||**Typ Ansprechpartner**|
|**`vContactNumber`**|**`Value`**|**1 **||||**Nummer Ansprechpartner**|
|**`vTitle`**|**`Value`**|**1 **||||**Anrede**|
|**`vFirstName`**|**`FirstName`**|**1 **||||**Vorname**|
|**`vLastName`**|**`LastName`**|**1 **|**X **|||**Nachname**|
|~~**`hdrCom`**~~|~~**`Com`**~~|*** **||||**Kopf Lieferavis: Kommunikation**|



Zur Angabe der Ansprechpartner, die für die Abwicklung des Geschäftsfalls erforderlich sind oder
organisatorisch zugeordnet werden (z.B. einen Ansprechpartner im Lager).


**3.12** **Rahmenelement** **`hdrText`** **– Kopf Lieferavis: Texte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrText`**|**`Text`**|*** **||**! **||**Kopf Lieferavis: Texte**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vTextType`**|**`TextType`**|**1 **|**X **|**! **||**Textart**|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|**! **||**Textsprache**|
|**`vTextContent`**|**`TextContent`**|**1+**|**X **|||**Textinhalt**|



**3.13** **Rahmenelement** **`hdrReference`** **– Kopf Lieferavis: Verweise**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrReference`**|**`Reference`**|*** **||||**Kopf Lieferavis: Verweise**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vReferenceType`**|**`ReferenceType`**|**1 **|**X **|||**Verweisart**|
|**`vReferenceValue`**|**`Value`**|**1 **|**X **|||**Wert Verweis**|
|**`vReferenceDesc`**|**`Value`**|**1 **|**X **|||**Bezeichnung**<br>_(in Belegsprache)_|



Werden Dateianhänge zu einem OEX-Dokument in einer Email geschickt, müssen diese hier entsprechend
angegeben werden. Dies ermöglicht einer Applikation verschiedene Dateianhänge dem entsprechenden
OEX-Dokument zuzuordnen und ggfs. weiter zu verarbeiten.

Verwendungsbeispiel: Internetlink (LNK) zu einem Trackingsystem
```
           "http://www.moebel-huber.de/lieferstatus.html?p=987654321"

```

                               - 12 

**3.14** **Rahmenelement** **`hdrPricing`** **– Kopf Lieferavis: Preiskalkulation**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`hdrPricing`**~~|~~**`Pricing`**~~|*** **||**! **|**D **|**Kopf Lieferavis: Preiskalkulation**|
|~~**`hdrPricing`**~~|**Attribut**|**Attribut**|||||
|~~**`hdrPricing`**~~|`aCondNo`|`aCondNo`|X|!||Laufende Nummer der Belegposition|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vConditionType`**|**`ConditionType`**|**1 **|**X **|||**Konditionsart**|
|**`vConditionValue`**|**`ConditionValue`**|**1 **|**X **|||**Konditionswert**|
|**`vConditionRate`**|**`ConditionRate`**|**1 **||||**Konditionssatz**|
|**`vCondCurrency`**|**`CondCurrency`**|**1 **||||**Konditionswährung**|
|**`vConditionText`**|**`ConditionText`**|**1 **||||**Konditionsbezeichnung**<br>_(in Belegsprache)_|



In diesem Rahmenelement wird die Netto-Summe ( `TNET` ) aller Lieferungen des Transports angegeben
(Pflichtangabe). Sie dient als Kontrollsumme bei der Verarbeitung des Dokuments und als Warenwert für
den Versand bzw. die Verzollung.
Andere Angaben, wie z.B. Steuern, sind optional.
Die Währung wird, wenn hier nicht anders angegeben, durch die Belegwährung vorgegeben.
Der Konditionsbereich ist auf den Verkauf beschränkt ( `aCondArea="S"` ).

Hinweis: Die Unterelemente für Preiseinheit und Mengeneinheit des Rahmenelements entfallen, da es sich
hier immer um Summenkonditionen handelt.

Beispiel 1 – Angabe des Nettowertes (Warenwert) des Transports:
Nettowert der Lieferung aus Lieferavis Position 1 beträgt € 250,00
Nettowert der Lieferung aus Lieferavis Position 2 beträgt € 150,00

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 400.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>

```

Beispiel 2 – Angabe des Nettowertes (Warenwert), der Steuerkonditionen und dem Endbetrag des
Transports:
Nettowert der Lieferung aus Lieferavis Position 1 beträgt € 250,00 mit Steuerkennzeichen 1, 19%
Nettowert der Lieferung aus Lieferavis Position 2 beträgt € 150,00 mit Steuerkennzeichen 1, 19%

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 400.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="2">
    <vConditionType aCondArea="S" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 400.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="3">
    <vConditionType aCondArea="S" aCondRef="2" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue> 76.00 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="4">
    <vConditionType aCondArea="S"> TOTL </vConditionType>
    <vConditionValue> 476.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>

```

                               - 13 

**3.15** **Rahmenelement** **`docItem`** **– Belegposition Lieferavis**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`docItem`**~~|~~**`Item`**~~|**1+**|**X **|**! **||**Belegposition Lieferavis**|
|~~**`docItem`**~~|**Attribut**|**Attribut**|||||
|~~**`docItem`**~~|`aItemNo`|`aItemNo`|X|!||Laufende Nummer der Belegposition|
|~~**`docItem`**~~|`aAction`|`aAction`||||Aktion|
|~~**`docItem`**~~|`aUUID`|`aUUID`||||Global eindeutiger Identifikator|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmDocument`**|**`Document`**|**1 **|**X **||**D **|**Einzelnes Dokument**|



Die Belegpositionen des Lieferavis (Transports) enthalten je nach Transportgrundlage ( `vShipmentBase` )
die einzelnen Bestellungen (Aufträge) oder die bereits erstellten Lieferungen (Lieferscheine).


**3.16** **Rahmenelement** **`itmDocument`** **– Einzelnes Dokument**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`itmDocument`**~~|~~**`Document`**~~|**1 **|**X **|**! **|**D **|**Einzelnes Dokument**|
|~~**`itmDocument`**~~|**Attribut**|**Attribut**|||**D **||
|~~**`itmDocument`**~~|`aItemCount`|`aItemCount`|X|||Anzahl der Positionen im Dokument|
|~~**`itmDocument`**~~|`aAction`|`aAction`|X|||Aktion|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`docHeader`**|**`Header`**|**1 **|**X **|||**Belegkopf Lieferung**|
|~~**`docItem`**~~|~~**`Item`**~~|**1+**|**X **|||**Belegposition Lieferung**|



Dieses Rahmenelement beinhaltet alle weiteren Elemente, die für die Beschreibung der Lieferung verwendet
werden.


**3.17** **Rahmenelement** **`itmHeader`** **– Belegkopf Lieferung**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmHeader`**|**`Header`**|**1 **|**X **|||**Belegkopf Lieferung**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDeliveryNumber`**|**`DocNo`**|**1 **|**X **|||**Liefernummer**<br>_Eindeutige Nummer der Lieferung des_<br>_Lieferanten. Bilden Bestellungen_<br>_(Aufträge) die Transportgrundlage wird_<br>_hier die Bestellbestätigungsnummer_<br>_des Lieferanten eingetragen._|
|**`vClientNumber`**|**`Value`**|**1 **|**X **|||**Kundennummer**<br>_Nummer, unter der der Lieferant den_<br>_Besteller (Kunde) führt._|
|**`vClientID`**|**`ClientID`**|*** **||||**Kunden-ID**|
|**`vClientClass`**|**`ClientClass`**|*** **||||**Kunden-Klassifizierung**|
|**`vVendorNumber`**|**`Value`**|**1 **|**X **|||**Lieferantennummer**<br>_Nummer, unter der der Besteller_<br>_(Kunde) den Lieferanten führt._|
|**`vSupplierID`**|**`SupplierID`**|*** **||||**Lieferanten-ID**|



                               - 14 

|vSupplierClass|SupplierClass|*|Col4|Col5|Col6|Lieferanten-Klassifizierung|
|---|---|---|---|---|---|---|
|**`vDocCurrency`**|**`DocCurrency`**|**1 **|**X **|||**Belegwährung**|
|**`vIncoTerm`**|**`IncoTerm`**|**1 **|**1 **|||**Inco Terms (Lieferbedingung)**<br>_Anders lautende Lieferbedingungen_<br>_können über den Kopftext “Liefer-_<br>_bedingungen“ übergeben werden._|
|**`vIncoTermLocation`**|**`IncoTermLoc`**|**1 **|**1 **|||**Ortsangabe zu Inco Terms**|
|**`vDocLanguage`**|**`DocLanguage`**|**1 **|**X **|||**Belegsprache**|
|**`vDelivComplet`**|**`DelivComplet`**|**1 **||||**Vollständigkeit der Lieferung**|
|**`vNumPackages`**|**`NumPackages`**|**1 **||||**Anzahl Packstücke (Colli)**|
|**`vGrossWeight`**|**`GrossWeight`**|**1 **|**2 **|||**Bruttogewicht (gesamt)**|
|**`vNetWeight`**|**`NetWeight`**|**1 **|**2 **|||**Nettogewicht (gesamt)**|
|**`vUnitWeight`**|**`UnitWeight`**|**1 **|**2 **|||**Gewichtseinheit**|
|**`vVolume`**|**`Volume`**|**1 **|**3 **|||**Volumen (gesamt)**|
|**`vUnitVolume`**|**`UnitVolume`**|**1 **|**3 **|||**Volumeneinheit**|
|**`vTransportMode`**|**`TransportMode`**|**1 **||||**Verkehrszweig**|
|**`vCustomNumber`**|**`CustomNumber`**|**1 **||||**Zollnummer**|
|~~**`hdrDocNo`**~~|~~**`DocNo`**~~|*** **|||**D **|**Kopf Lieferung: Belegnummern**|
|~~**`hdrDateTime`**~~|~~**`DateTime`**~~|**3+**|**X **|||**Kopf Lieferung: Datums- u. Zeitang.**|
|~~**`hdrOrgData`**~~|~~**`OrgData`**~~|*** **||||**Kopf Lieferung: Organisationsdaten**|
|~~**`hdrAddress`**~~|~~**`Address`**~~|**1+**|**X **|||**Kopf Lieferung: Adressen**|
|~~**`hdrText`**~~|~~**`Text`**~~|*** **||||**Kopf Lieferung: Texte**|
|~~**`hdrReference`**~~|~~**`Reference`**~~|*** **||||**Kopf Lieferung: Verweise**|
|~~**`hdrPricing`**~~|~~**`Pricing`**~~|*** **|||**D **|**Kopf Lieferung: Preiskalkulation**|
|~~**`hdrPayment`**~~|~~**`Payment`**~~|**3***||||**Kopf Lieferung: Zahlungsbedingung**|


Der Belegkopf enthält alle wichtigen Referenzen der Lieferung.

Erläuterungen zu Pflichtangaben:


**1** Die Ortsangabe zu Inco Terms muss angegeben werden, sobald die Lieferbedingung dies fordert.
**2** Die Gewichtseinheit muss angegeben werden, sobald Bruttogewicht und/oder Nettogewicht
angegeben wird.
**3** Die Volumeneinheit muss angegeben werden, sobald das Volumen angegeben wird.


**3.18** **Rahmenelement** **`hdrDocNo`** **– Kopf Lieferung: Belegnummern**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrDocNo`**|**`DocNo`**|*** **||||**Kopf Lieferung: Belegnummern**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDocNoType`**|**`DocNoType`**|**1 **|**X **|||**Belegnummernart**|
|**`vDocNo`**|**`DocNo`**|**1 **|**X **|||**Belegnummer**|



Dieses Rahmenelement enthält die Belegnummern der Vorgängerbelege in der Abfolge des Geschäftsfalls
und/oder zusätzliche Belege als Referenz für die Lieferung. Die Angabe der Belegposition entfällt hier auf
Kopfebene. Es kann somit nicht auf Positionen andere Belege referenziert werden. Setzt sich eine Lieferung
bspw. aus Positionen verschiedener Aufträge (Bestellungen) zusammen, darf keine Angabe hierüber im
Kopf erfolgen, sondern muss auf Positionsebene ( `itmDocNo` ) mit Angabe der Belegposition erfolgen.

Die Angabe der Liefernummer selbst ist hier nicht erlaubt, da diese eindeutig für den Beleg ist und bereits im
Rahmenelement Belegkopf ( `docHeader` ) im Element `vDeliveryNumber` angegeben wird.


                               - 15 

Pflichtangaben bei bereits erstellten Lieferungen (Transportgrundlage), sofern vorhanden und eindeutig
(nicht aus verschiedenen Belegen):
Bestellnummer (entfällt bspw. bei telefonischer Bestellung/-Änderung ohne Angabe einer eindeutigen
Bestellnummer seitens des Kunden)
```
    <vDocNoType aDocContext="S"> ORD </vDocNoType>
```

Bestellbestätigungsnummer (entfällt bspw. falls es sich um eine Lieferung ohne Auftragsbezug handelt)
```
    <vDocNoType aDocContext="S"> CNF </vDocNoType>

```

**3.19** **Rahmenelement** **`hdrDateTime`** **– Kopf Lieferung: Datums- und Zeitangaben**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrDateTime`**|**`DateTime`**|**3+**|**X **|**! **||**Kopf Lieferung: Datum und Zeit**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDateTimeType`**|**`DateTimeType`**|**1 **|**X **|**! **||**Typ Datum/Zeit**|
|**`vTimeZone`**|**`TimeZone`**|**1 **|**X **|||**Zeitzone**|
|**`vDateValue`**|**`Date`**|**1 **|**X **|||**Datumsangabe**|
|**`vTimeValue`**|**`Time`**|**1 **||||**Zeitangabe**|



Dieses Rahmenelement dient zur Übergabe von Datums- und Zeitangaben eines Lieferkopfes.
Als Pflichtangaben sind hier das Belegdatum ( `DOC` ), sowie Lieferdatum ( `DLD` ) oder Fixlieferdatum ( `FXD` )
erforderlich. Des Weiteren je nach Transportgrundlage das Bestellbestätigungsdatum ( `COD` ) oder das
Lieferscheindatum ( `DND` ).


**3.20** **Rahmenelement** **`hdrOrgData`** **– Kopf Lieferung: Organisationsdaten**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrOrgData`**|**`OrgData`**|*** **||**! **||**Kopf Lieferung: Organisationsdaten**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vOrgDataType`**|**`OrgDataType`**|**1 **|**X **|**! **||**Arten Organisationsdaten**|
|**`vOrgDataValue`**|**`Value`**|**1 **|**X **|||**Wert Organisationsdaten**|



Verwendungsbeispiele: Kommissionsangabe ( `COM` ) `"Kommission Schmidt"`
Projektnummer ( `PJN` ) `"576134"`
Versandstelle ( `DLO` ) `"V5050"`


                               - 16 

**3.21** **Rahmenelement** **`hdrAddress`** **– Kopf Lieferung: Adressen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrAddress`**|**`Address`**|**1+**|**X **|**! **||**Kopf Lieferung: Adressen**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vAddressType`**|**`AddressType`**|**1 **|**X **|**! **||**Typ Adresse**|
|**`vAddressNumber`**|**`Value`**|**1 **||||**Adress-Nummer**|
|**`vAddressID`**|**`AddressID`**|*** **||||**Adress-ID**|
|**`vTitle`**|**`Value`**|**1 **||||**Anrede**|
|**`vName1`**|**`Name1`**|**1 **|**X **|||**Name 1**|
|**`vName2`**|**`Name2`**|**1 **||||**Name 2**|
|**`vName3`**|**`Name3`**|**1 **||||**Name 3**|
|**`vName4`**|**`Name4`**|**1 **||||**Name 4**|
|**`vStreet`**|**`Street`**|**1 **|**X **|||**Straße**|
|**`vStreetNo`**|**`Value`**|**1 **||||**Straßennummer**|
|**`vStreet2`**|**`Street2`**|**1 **||||**Straße 2**|
|**`vCountryCode`**|**`CountryCode`**|**1 **|**X **|||**Länderkennzeichen**|
|**`vPostalCode`**|**`PostalCode`**|**1 **|**X **|||**Postleitzahl**|
|**`vLocation`**|**`Location`**|**1 **|**X **|||**Ort**|
|**`vDistrict`**|**`District`**|**1 **||||**Ortsteil**|
|**`vCountyCode`**|**`CountyCode`**|**1 **||||**Region/Bundesland/-Staat**|
|**`vPostalCodePOBox`**|**`PostalCodePOB`**|**1 **||||**Postleitzahl Postfach**|
|**`vPOBox`**|**`Value`**|**1 **||||**Postfachnummer**|
|**`vTaxCode`**|**`Value`**|**1 **||||**Steuernummer Finanzamt**|
|**`vTaxCodeEU`**|**`Value`**|**1 **||||**Steuernummer EU / USt-IdNr.**|
|**`vTaxCodeUS`**|**`Value`**|**1 **||||**Steuernummer US / Jurisdiction**|
|~~**`hdrCom`**~~|~~**`Com`**~~|*** **||||**Kopf Lieferung: Kommunikation**|
|~~**`hdrContact`**~~|~~**`Contact`**~~|*** **||||**Kopf Lieferung: Ansprechpartner**|



Als Pflichtangabe wird vom Lieferanten hier die Lieferadresse (SH) angegeben. Sind gewisse Konditionen
(Transportkosten) für die Lieferung zwischen beiden Geschäftspartner vereinbart, können diese auch mittels
ebenfalls vereinbarten Indikatoren wir Adress-Nummer, Transportzone (siehe Organisationsdaten) und Zubzw. Abschlägen in der Preiskalkulation übermittelt werden.
Es empfiehlt sich auch alle von den Stammdaten abweichenden Adressen anzugeben.
In der Regel sind die Adressen insbesondere Auftraggeber (SO) und Lieferant (SU) den beiden
Geschäftspartnern bekannt und als Stammdaten hinterlegt und bedürfen nicht unbedingt einer Übertragung,
sie korrespondieren auch zur Kundennummer bzw. Lieferantennummer des Belegkopfes ( `docHeader` ).
Ggf. wird mit einer Adresse aber auch eine für diese Bestellung zuständige Kontaktperson übermittelt.


**3.22** **Rahmenelement** **`hdrCom`** **– Kopf Lieferung: Kommunikation**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrCom`**|**`Com`**|*** **||**! **||**Kopf: Kommunikation**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vComType`**|**`ComType`**|**1 **|**X **|**! **||**Art der Kommunikation**|
|**`vComType`**|**Attribut**|**Attribut**|||||
|**`vComType`**|`aScopeInfo`|`aScopeInfo`|**X **|**! **||Anwendungsbereich der Information|
|**`vComValue`**|**`Value`**|**1 **|**X **|||**Wert Kommunikation**|



Zur Angabe von Telefon, Telefax, Email etc. zur Adresse und/oder zum Ansprechpartner.


                               - 17 

**3.23** **Rahmenelement** **`hdrContact`** **– Kopf Lieferung: Ansprechpartner**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrContact`**|**`Contact`**|*** **||||**Kopf Lieferung: Ansprechpartner**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vContactType`**|**`ContactType`**|**1 **|**X **|||**Typ Ansprechpartner**|
|**`vContactNumber`**|**`Value`**|**1 **||||**Nummer Ansprechpartner**|
|**`vTitle`**|**`Value`**|**1 **||||**Anrede**|
|**`vFirstName`**|**`FirstName`**|**1 **||||**Vorname**|
|**`vLastName`**|**`LastName`**|**1 **|**X **|||**Nachname**|
|~~**`hdrCom`**~~|~~**`Com`**~~|*** **||||**Kopf Lieferung: Kommunikation**|



Zur Angabe der Ansprechpartner, die für die Abwicklung des Geschäftsfalls erforderlich sind oder
organisatorisch zugeordnet werden (z.B. einen Ansprechpartner im Lager).


**3.24** **Rahmenelement** **`hdrText`** **– Kopf Lieferung: Texte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrText`**|**`Text`**|*** **||**! **||**Kopf Lieferung: Texte**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vTextType`**|**`TextType`**|**1 **|**X **|**! **||**Textart**|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|**! **||**Textsprache**|
|**`vTextContent`**|**`TextContent`**|**1+**|**X **|||**Textinhalt**|



**3.25** **Rahmenelement** **`hdrReference`** **– Kopf Lieferung: Verweise**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrReference`**|**`Reference`**|*** **||**! **||**Kopf Lieferung: Verweise**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vReferenceType`**|**`ReferenceType`**|**1 **|**X **|**! **||**Verweisart**|
|**`vReferenceValue`**|**`Value`**|**1 **|**X **|**! **||**Wert Verweis**|
|**`vReferenceDesc`**|**`Value`**|**1 **|**X **|||**Bezeichnung**<br>_(in Belegsprache)_|



Werden Dateianhänge zu einem OEX-Dokument in einer Email geschickt, müssen diese hier entsprechend
angegeben werden. Dies ermöglicht einer Applikation verschiedene Dateianhänge dem entsprechenden
OEX-Dokument zuzuordnen und ggfs. weiter zu verarbeiten.

Verwendungsbeispiel: Internetlink (LNK) zu einem Trackingsystem
```
           "http://www.moebel-huber.de/lieferstatus.html?p=987654321"

```

                               - 18 

**3.26** **Rahmenelement** **`hdrPricing`** **– Kopf Lieferung: Preiskalkulation**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`hdrPricing`**~~|~~**`Pricing`**~~|*** **||**! **|**D **|**Kopf Lieferung: Preiskalkulation**|
|~~**`hdrPricing`**~~|**Attribut**|**Attribut**|||||
|~~**`hdrPricing`**~~|`aCondNo`|`aCondNo`|X|!||Laufende Nummer der Belegposition|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vConditionType`**|**`ConditionType`**|**1 **|**X **|||**Konditionsart**|
|**`vConditionValue`**|**`ConditionValue`**|**1 **|**X **|||**Konditionswert**|
|**`vConditionRate`**|**`ConditionRate`**|**1 **||||**Konditionssatz**|
|**`vCondCurrency`**|**`CondCurrency`**|**1 **||||**Konditionswährung**|
|**`vConditionText`**|**`ConditionText`**|**1 **||||**Konditionsbezeichnung**<br>_(in Belegsprache)_|



In diesem Rahmenelement wird die Netto-Summe ( `TNET`, ggfs. `TNEH` ) der Positionen einer Lieferung
angegeben (Pflichtangabe). Sie dient als Kontrollsumme bei der Verarbeitung des Dokuments und als
Warenwert für den Versand bzw. die Verzollung.
Andere Angaben, wie z.B. Steuern. sind optional oder werden, wenn erforderlich, angegeben.
Die Währung wird, wenn hier nicht anders angegeben, durch die Belegwährung vorgegeben.
Der Konditionsbereich ist auf den Verkauf beschränkt ( `aCondArea="S"` ).

Hinweis: Im Gegensatz zu den Preisangaben auf Positionsebene entfallen in diesem Rahmenelement die
Unterelemente für Preiseinheit und Mengeneinheit, da es sich hier immer um Summenkonditionen handelt.

Beispiel 1 – Angabe des Nettowertes (Warenwert) der Lieferung:
Nettowert der Position 1 beträgt € 100,00
Nettowert der Position 2 beträgt € 150,00

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>

```

Beispiel 2 – Angabe des Nettowertes (Warenwert), der Steuerkonditionen und dem Endbetrag der Lieferung:
Nettowert der Position 1 beträgt € 100,00 mit Steuerkennzeichen 1, 19%
Nettowert der Position 2 beträgt € 150,00 mit Steuerkennzeichen 1, 19%

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="2">
    <vConditionType aCondArea="S" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="3">
    <vConditionType aCondArea="S" aCondRef="2" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue> 47.50 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="4">
    <vConditionType aCondArea="S"> TOTL </vConditionType>
    <vConditionValue> 297.50 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>

```

                               - 19 

**3.27** **Rahmenelement** **`hdrPayment`** **– Kopf Lieferung: Zahlungsbedingungen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrPayment`**|**`Payment`**|**3***||**! **||**Lieferung: Zahlungsbedingungen**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vPaymentPart`**|**`PaymentPart`**|**1 **|**X **|**! **||**Bestandteil der Zahlungsbedingung**|
|**`vPaymentRate`**|**`PaymentRate`**|**1 **|**X **|||**Skonto-Satz (%)**|
|**`vPaymentDays`**|**`PaymentDays`**|**1 **|**X **|||**Anzahl Tage (Zahlungsziel)**|



**3.28** **Rahmenelement** **`docItem`** **– Belegposition Lieferung**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`docItem`**~~|~~**`Item`**~~|**1+**|**X **|**! **||**Belegposition Lieferung**|
|~~**`docItem`**~~|**Attribut**|**Attribut**|||||
|~~**`docItem`**~~|`aItemNo`|`aItemNo`|X|!||Laufende Nummer der Belegposition|
|~~**`docItem`**~~|`aAction`|`aAction`||||Aktion|
|~~**`docItem`**~~|`aUUID`|`aUUID`||||Global eindeutiger Identifikator|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDelivItemNumber`**|**`ItemPosNo`**|**1 **|**X **|**! **||**Nummer der Lieferposition**<br>_Eindeutige Positionsnummer innerhalb_<br>_der Lieferung. Bilden Bestellungen_<br>_(Aufträge) die Transportgrundlage wird_<br>_hier die Positionsnummer der Bestell-_<br>_bestätigung des Lieferanten_<br>_eingetragen._|
|**`vDelivTopLevelNo`**|**`ItemPosNo`**|**1 **||||**Übergeordnete Nummer der**<br>**Lieferposition**<br>_Verweis auf die übergeordnete_<br>_Positionsnummer bei Verwendung von_<br>_Unterpositionen oder Stücklisten._|
|**`vClientArticleNo`**|**`ClientArtNo`**|**1 **||||**Kundenartikelnummer**|
|**`vVendorArticleNo`**|**`VendorArtNo`**|**1 **|**X **|||**Lieferantenartikelnummer**|
|**`vVendorID`**|**`VendorID`**|**1 **|**X **|||**Lieferantenkennung**|
|**`vVendorSeries`**|**`VendorSeries`**|**1 **|**X **|||**Lieferantenserie**|
|**`vCatalogId`**|**`CatalogId`**|**1 **||||**Katalog-ID**|
|**`vArticleEAN`**|**`EAN_Article`**|**1 **||||**EAN des Artikels**|
|**`vDelivQuantity`**|**`Quantity`**|**1 **|**X **|||**Liefermenge**|
|**`vDelivUnit`**|**`QuantUnit`**|**1 **|**X **|||**Liefermengeneinheit**|
|**`vDelivComplet`**|**`DelivComplet`**|**1 **||||**Vollständigkeit der Lieferung**|
|**`vPackedWithItemNo`**|**`DocItemNo`**|**1 **|**1 **|||**Verpackt mit Lieferposition**|
|**`vPackageNumber`**|**`PackageNumber`**|**1 **||||**Packstücknummer**|
|**`vNumPackages`**|**`NumPackages`**|**1 **|**1 **|||**Anzahl Packstücke (Colli)**|
|**`vNumArtPack`**|**`NumArtPack`**|**1 **|**1 **|||**Anzahl Artikel pro Packstück**|
|**`vGrossWeight`**|**`GrossWeight`**|**1 **|**1, 2**|||**Bruttogewicht (gesamt)**|
|**`vNetWeight`**|**`NetWeight`**|**1 **|**1, 2**|||**Nettogewicht (gesamt)**|
|**`vUnitWeight`**|**`UnitWeight`**|**1 **|**1, 2**|||**Gewichtseinheit**|
|**`vVolume`**|**`Volume`**|**1 **|**1, 3**|||**Volumen (gesamt)**|
|**`vUnitVolume`**|**`UnitVolume`**|**1 **|**1, 3**|||**Volumeneinheit**|
|**`vPackLength`**|**`Length`**|**1 **|**4 **|||**Länge der Verpackung**|
|**`vPackWidth`**|**`Width`**|**1 **|**4 **|||**Breite der Verpackung**|
|**`vPackHeight`**|**`Height`**|**1 **|**4 **|||**Höhe der Verpackung**|



                               - 20 

|vMeasureUnit|MeasureUnit|1|4|Col5|Col6|Maßeinheit Verpackungsabmessung|
|---|---|---|---|---|---|---|
|**`vCommodCode`**|**`CommodCode`**|**1 **||||**Warennummer (INTRASTAT)**|
|**`vCountryOrigin`**|**`CountryOrigin`**|**1 **||||**Ursprungsland**|
|**`vCountyOrigin`**|**`CountyOrigin`**|**1 **||||**Ursprungsregion**|
|**`vPackageType`**|**`PackageType`**|**1 **||||**Verpackungsart**|
|**`vMeansTransp`**|**`MeansTransp`**|**1 **||||**Förderhilfsmittel**|
|**`vClassification`**|**`Classification`**|<br>*** **||||**Klasse/Kategorie der Bestellposition**|
|~~**`itmConfiguration`**~~|~~**`Config`**~~|*** **||||**Position Lieferung: Konfiguration**|
|~~**`itmDocNo`**~~|~~**`DocNo`**~~|*** **||||**Position Lieferung: Belegnummern**|
|~~**`itmDateTime`**~~|~~**`DateTime`**~~|*** **||||**Position Lieferung: Datum und Zeit**|
|~~**`itmOrgData`**~~|~~**`OrgData`**~~|*** **||||**Position Lieferung: Organisation**|
|~~**`itmAddress`**~~|~~**`Address`**~~|*** **||||**Position Lieferung: Adressen**|
|~~**`itmText`**~~|~~**`Text`**~~|**1+**|**X **|||**Position Lieferung: Texte**|
|~~**`itmReference`**~~|~~**`Reference`**~~|*** **||||**Position Lieferung: Verweise**|
|~~**`itmPricing`**~~|~~**`Pricing`**~~|*** **||||**Position Lieferung: Preiskalkulation**|


Basisdaten der Belegposition der Lieferung.

Durch das Zusammenspiel von Positionsnummer mit der übergeordneten Positionsnummer lässt sich eine
„beliebig“ tiefe Hierarchiestruktur abbilden. Spezielle Strukturen der Positionsnummer lassen sich hierdurch
aber nicht abbilden, schon gar nicht Verzeichnisstrukturen. Positionsnummernangaben wie bspw. “100.A.101“ können über die Organisationsdaten Typ `POS` übermittelt werden, in wie weit eine andere Applikation
diese verarbeiten, zurückliefern kann oder gar für sich selbst verwendet, bleibt jedoch offen.

Erläuterungen zu Pflichtangaben:


**1** Wird angegeben, dass die Position mit einer anderen Lieferposition verpackt worden ist, dürfen
weitere Angaben unter 1 (Anzahl, Gewichte und Volumen) nicht noch einmal zum Gesamtwert des
Lieferkopfes addiert werden. Die Lieferposition, auf die über das Element ( `vPackedWithItemNo` )
referenziert wird, ist die „führende“ Lieferposition.
**2** Die Gewichtseinheit muss angegeben werden, sobald Bruttogewicht und/oder Nettogewicht
angegeben wird.
**3** Die Volumeneinheit muss angegeben werden, sobald das Volumen angegeben wird.
**4** Die Maßeinheit für die Verpackungsabmessung muss angegeben werden, sobald mindestens eine
der Angaben für Länge, Breite oder Höhe der Verpackung angegeben wird.


**3.29** **Rahmenelement** **`itmConfiguration`** **– Position Lieferung: Konfiguration**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmConfiguration`**|**`Config`**|*** **||||**Position Lieferung: Konfiguration**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vClassID`**|**`Value`**|**1 **||||**Merkmalsklasse**|
|**`vOptionID`**|**`Value`**|**1 **|**X **|||**Merkmal**|
|**`vOptionEAN`**|**`EAN_Option`**|**1 **||||**EAN des Merkmals**|
|**`vValueID`**|**`Value`**|**1 **|**X **|||**Merkmalswert**|
|**`vValueEAN`**|**`EAN_Value`**|**1 **||||**EAN des Merkmalswertes**|
|~~**`itmConfigText`**~~|~~**`ConfigText`**~~|*** **||||**Position Lieferung: Konfig.-Texte**|



                               - 21 

**3.30** **Rahmenelement** **`itmConfigText`** **– Position Lieferung: Konfigurationstexte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmConfigText`**|**`ConfigText`**|*** **||||**Lieferung Konfigurationstexte**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|||**Textsprache**|
|**`vOptionText`**|**`OptionText`**|**1 **|**X **|||**Merkmalstext**|
|**`vValueText`**|**`ValueText`**|*** **||||**Merkmalswertetext**<br>Hier fällt der Text unter Umständen<br>weg, wenn es sich um einen frei<br>bewertbaren Merkmalswert handelt.|



**3.31** **Rahmenelement** **`itmDocNo`** **– Position Lieferung: Belegnummern**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmDocNo`**|**`DocNo`**|*** **||||**Position Lieferung: Belegnummern**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDocNoType`**|**`DocNoType`**|**1 **|**X **|||**Belegnummernart**|
|**`vDocNo`**|**`DocNo`**|**1 **|**X **|||**Belegnummer**|
|**`vDocLine`**|**`DocItemNo`**|**1 **||||**Nummer der Belegposition**|



Dieses Rahmenelement enthält die Positionsnummern der Vorgängerbelege in der Abfolge des Geschäftsfalls und/oder zusätzliche Belege als Referenz für die Lieferung. Die Angabe der Positionsnummer ist immer
erforderlich, solange es sich nicht um einen Beleg ohne Positionsangaben handelt.


**3.32** **Rahmenelement** **`itmDateTime`** **– Position Lieferung: Datums- u. Zeitangaben**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmDateTime`**|**`DateTime`**|*** **||**! **||**Position Lieferung: Datum und Zeit**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDateTimeType`**|**`DateTimeType`**|**1 **|**X **|**! **||**Typ Datum/Zeit**|
|**`vTimeZone`**|**`TimeZone`**|**1 **|**X **|||**Zeitzone**|
|**`vDateValue`**|**`Date`**|**1 **|**X **|||**Datumsangabe**|
|**`vTimeValue`**|**`Time`**|**1 **||||**Zeitangabe**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrDateTime` abweicht oder zusätzliche positionsrelevante Informationen enthält.


                               - 22 

**3.33** **Rahmenelement** **`itmOrgData`** **– Position Lieferung: Organisationsdaten**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmOrgData`**|**`OrgData`**|*** **||**! **||**Position Lieferung: Organisat.-daten**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vOrgDataType`**|**`OrgDataType`**|**1 **|**X **|**! **||**Art der Organisationsdaten**|
|**`vOrgDataValue`**|**`Value`**|**1 **|**X **|||**Wert Organisationsdaten**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrOrgData` abweicht oder zusätzliche positionsrelevante Informationen enthält.


**3.34** **Rahmenelement** **`itmAddress`** **– Position Lieferung: Adressen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmAddress`**|**`Address`**|*** **||**! **||**Position Lieferung: Adressen**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vAddressType`**|**`AddressType`**|**1 **|**X **|**! **||**Typ Adresse**|
|**`vAddressNumber`**|**`Value`**|**1 **||||**Adress-Nummer**|
|**`vAddressID`**|**`AddressID`**|*** **||||**Adress-ID**|
|**`vTitle`**|**`Value`**|**1 **||||**Anrede**|
|**`vName1`**|**`Name1`**|**1 **|**X **|||**Name 1**|
|**`vName2`**|**`Name2`**|**1 **||||**Name 2**|
|**`vName3`**|**`Name3`**|**1 **||||**Name 3**|
|**`vName4`**|**`Name4`**|**1 **||||**Name 4**|
|**`vStreet`**|**`Street`**|**1 **|**X **|||**Straße**|
|**`vStreetNo`**|**`Value`**|**1 **||||**Straßennummer**|
|**`vStreet2`**|**`Street2`**|**1 **||||**Straße 2**|
|**`vCountryCode`**|**`CountryCode`**|**1 **|**X **|||**Länderkennzeichen**|
|**`vPostalCode`**|**`PostalCode`**|**1 **|**X **|||**Postleitzahl**|
|**`vLocation`**|**`Location`**|**1 **|**X **|||**Ort**|
|**`vDistrict`**|**`District`**|**1 **||||**Ortsteil**|
|**`vCountyCode`**|**`CountyCode`**|**1 **||||**Region/Bundesland/-Staat**|
|**`vPostalCodePOBox`**|**`PostalCodePOB`**|**1 **||||**Postleitzahl Postfach**|
|**`vPOBox`**|**`Value`**|**1 **||||**Postfachnummer**|
|**`vTaxCode`**|**`Value`**|**1 **||||**Steuernummer Finanzamt**|
|**`vTaxCodeEU`**|**`Value`**|**1 **||||**Steuernummer EU / USt-IdNr.**|
|**`vTaxCodeUS`**|**`Value`**|**1 **||||**Steuernummer US / Jurisdiction**|
|~~**`itmCom`**~~|~~**`Com`**~~|*** **||||**Position Lieferung: Kommunikation**|
|~~**`itmContact`**~~|~~**`Contact`**~~|*** **||||**Position Lieferung: Ansprechpartn.**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrAddress` abweicht oder zusätzliche positionsrelevante Informationen enthält.


                               - 23 

**3.35** **Rahmenelement** **`itmCom`** **– Position Lieferung: Kommunikation**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmCom`**|**`Com`**|*** **||**! **||**Position: Kommunikation**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vComType`**|**`ComType`**|**1 **|**X **|**! **||**Art der Kommunikation**|
|**`vComType`**|**Attribut**|**Attribut**|||||
|**`vComType`**|`aScopeInfo`|`aScopeInfo`|**X **|**! **||Anwendungsbereich der Information|
|**`vComValue`**|**`Value`**|**1 **|**X **|||**Wert Kommunikation**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrAddress` abweicht oder zusätzliche positionsrelevante Informationen enthält.


**3.36** **Rahmenelement** **`itmContact`** **– Position Lieferung: Ansprechpartner**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmContact`**|**`Contact`**|*** **||||**Position Lieferung: Ansprechpartn.**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vContactType`**|**`ContactType`**|**1 **|**X **|||**Typ Ansprechpartner**|
|**`vContactNumber`**|**`Value`**|**1 **||||**Nummer Ansprechpartner**|
|**`vTitle`**|**`Value`**|**1 **||||**Anrede**|
|**`vFirstName`**|**`FirstName`**|**1 **||||**Vorname**|
|**`vLastName`**|**`LastName`**|**1 **|**X **|||**Nachname**|
|~~**`itmCom`**~~|~~**`Com`**~~|*** **||||**Position Lieferung: Kommunikation**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrAddress` abweicht oder zusätzliche positionsrelevante Informationen enthält.


**3.37** **Rahmenelement** **`itmText`** **– Position Lieferung: Texte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmText`**|**`Text`**|**1+**|**X **|**! **||**Position Lieferung: Texte**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vTextType`**|**`TextType`**|**1 **|**X **|**! **||**Textart**|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|**! **||**Textsprache**|
|**`vTextContent`**|**`TextContent`**|**1+**|**X **|||**Textinhalt**|



Es wird wenigstens der Kurztext übermittelt bei einem Standardartikel, auf den Langtext kann in diesem Fall
verzichtet werden.
Anders verhält es sich bei modifizierten Artikeln und Kundenartikeln.
(vgl. globaler OEX-Werttyp `VendorArtNo`  `aStatus` ).


                               - 24 

**3.38** **Rahmenelement** **`itmReference`** **– Position Lieferung: Verweise**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmReference`**|**`Reference`**|*** **||||**Position Lieferung: Verweise**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vReferenceType`**|**`ReferenceType`**|**1 **|**X **|||**Typ Verweis**|
|**`vReferenceValue`**|**`Value`**|**1 **|**X **|||**Wert Verweis**|
|**`vReferenceDesc`**|**`Value`**|**1 **|**X **|||**Bezeichnung**<br>_(in Belegsprache)_|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrReference` abweicht oder zusätzliche positionsrelevante Informationen enthält.


**3.39** **Rahmenelement** **`itmPricing`** **– Position Lieferung: Preiskalkulation**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`itmPricing`**~~|~~**`Pricing`**~~|*** **||**! **||**Position Lieferung: Preiskalkulation**|
|~~**`itmPricing`**~~|**Attribut**|**Attribut**|||||
|~~**`itmPricing`**~~|`aCondNo`|`aCondNo`|X|!||Laufende Nummer der Belegposition|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vConditionType`**|**`ConditionType`**|**1 **|**X **|||**Konditionsart**|
|**`vConditionValue`**|**`ConditionValue`**|**1 **|**X **|||**Konditionswert**|
|**`vConditionRate`**|**`ConditionRate`**|**1 **||||**Konditionssatz**|
|**`vCondCurrency`**|**`CondCurrency`**|**1 **||||**Konditionswährung**|
|**`vConditionText`**|**`ConditionText`**|**1 **||||**Konditionsbezeichnung**<br>_(in Belegsprache)_|
|**`vPriceUnit`**|**`PriceUnit`**|**1 **||||**Preiseinheit**|
|**`vQuantUnit`**|**`QuantUnit`**|**1 **||||**Mengeneinheit**|



Die Angabe des Nettowertes der Position ( `TNET` ) ist Pflicht. Sie dient als Kontrollwert bei der Verarbeitung
des Dokuments und als Warenwert für den Versand bzw. die Verzollung.
Andere Angaben, wie z.B. Steuern. sind optional oder werden, wenn erforderlich, angegeben.
Die Währung wird, wenn hier nicht anders angegeben, durch die Belegwährung vorgegeben.
Der Konditionsbereich ist auf den Verkauf beschränkt ( `aCondArea="S"` ).
Die Mengeneinheit wird, wenn hier nicht anders angegeben, durch die Liefermengeneinheit ( `vDelivUnit` )
vorgegeben.

Beispiel 1 – Angabe des Nettowertes (Warenwert) der Lieferposition:
Nettoeinzelpreis der Position beträgt € 50,00
Liefermenge = 2

```
<itmPricing aCondNo="1">
    <vConditionType aCondArea="S"> TNET </vConditionType> ! TNET = Liefermenge x Nettoeinzelpreis
    <vConditionValue> 100.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

                               - 25 

Beispiel 2 – Angabe des Nettowertes (Warenwert), der Steuerkonditionen und dem Endbetrag der Position:
Nettoeinzelpreis der Position beträgt € 50,00 mit Steuerkennzeichen 1, 19%
Liefermenge = 2

```
<itmPricing aCondNo="1">
    <vConditionType aCondArea="S"> TNET </vConditionType> ! TNET = Liefermenge x Nettoeinzelpreis
    <vConditionValue> 100.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="2">
    <vConditionType aCondArea="S" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 100.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="3">
    <vConditionType aCondArea="S" aCondRef="2" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue> 19.00 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="S"> TOTL </vConditionType>
    <vConditionValue> 119.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

                               - 26 

### **4 Anhang**

**4.1** **Änderungshistorie**

|Version|Änderungen|
|---|---|
|3.1.0 – 8.5.2023| <br>Kleinere Umstellungen und Erweiterungen in der Einleitung<br> <br>Präzisierungen in den Rahmenelementen`hdrPricing` (Kopf Lieferavis: Preiskalkulation und Kopf<br>Lieferung: Preiskalkulation) sowie`itmPricing` (Position Lieferung: Preiskalkulation).|
|3.0.0 – 30.11.2017| <br>Globale Änderungen laut Spezifikation GLOBAL 3.0.0<br> <br>Umstrukturierung der Spezifikation<br> <br>Rahmenelement`docItem` (Belegposition Lieferung): Element`vClientArticleNo` hat nun den Typ<br>`ClientArtNo` (war`Value`).|
|2.3.0 – 1.7.2015| <br>Globale Änderungen laut Spezifikation GLOBAL 2.3.0<br> <br>Erweiterung: Rahmenelement Lieferung`docItem` (Belegposition Lieferung) ergänzt um optionales<br>Element für Angabe einer Klasse oder Kategorie:`vClassification`|
|2.2.0 – 11.10.2013| <br>Globale Änderungen laut Spezifikation GLOBAL 2.2.0<br> <br>Erweiterung: Rahmenelemente`docHeader` (Belegkopf Lieferavis und Belegkopf Lieferung) ergänzt<br>um optionale Elemente für Kunden-ID, Kunden-Klassifizierung, Lieferanten-ID und Lieferanten-<br>Klassifizierung:`vClientID`, `vClientClass`, `vSupplierID`und`vSupplierClass`. <br>Optionalen Elemente für ILN Kunde und ILN Lieferant ersetzt durch Kunden-ID und Lieferanten-ID.<br> <br>Erweiterung: Rahmenelemente`hdrAddress` (Kopf Lieferavis: Adressen und Kopf Lieferung:<br>Adressen) und`itmAddress` (Position Lieferung: Adressen) ergänzt um optionale Elemente „Straße 2“<br>und „Ortsteil“:`vStreet2`, `vDistrict`.<br>Optionales Element für ILN Adresse:`vAddressILN` ersetzt durch neues optionales Element für<br>Adress-ID:`vAddressID`. <br> <br>Erweiterung: Rahmenelement`docItem` (Belegposition Lieferung) ergänzt um optionales Element für<br>die Katalog-ID:`vCatalogId`. <br> <br>Logische strukturelle Anpassung: Rahmenelement`docItem` (Belegposition Lieferavis) ergänzt um<br>neues Rahmenelement`itmDocument` (Einzelnes Dokument) für Lieferungen (analog oexDocument),<br>das alle bisherigen Unterelemente aufnimmt. Dadurch auch Umbenennung der Rahmenelemente<br>`itmHeader` zu`docHeader` und`itmItem` zu`docItem`.|
|2.1.0 – 06.11.2009| <br>Globale Änderungen laut Spezifikation GLOBAL 2.1.0<br> <br>Präzisierung des Elements`vDelivItemNumber` (Nummer der Lieferposition) im Rahmenelement<br>`docItem` (Belegposition der Lieferung) vgl. Datentyp`CHAR(POS)`der Domäne`_Pos`. <br> <br>2.3 XML-Deklaration<br>Weiterführende Beschreibung zur Verwendung des XML-Schemas und dessen Version.|
|2.0.0 – 21.11.2008|Initialversion|



                               - 27 

