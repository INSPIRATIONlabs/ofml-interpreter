# Spezifikation OEX OFML Business Data Exchange
## (OFML Part VII)

# **INVOIC**
## Rechnung

# Version 3.1.0

Editoren:
Markus Behrschmidt, Vitra Services GmbH

Thomas Gerth, EasternGraphics GmbH


8.5.2023


Copyright © 2008 - 2023 Industrieverband Büro und Arbeitswelt e. V. (IBA)


# Inhalt

**1** **Einleitung .............................................................................................................. 3**

1.1 Verwendung dieser Spezifikation ............................................................................ 3

1.2 Dateinamenkonventionen ....................................................................................... 3

1.3 XML-Deklaration ..................................................................................................... 4

1.4 Prüfmechanismen ................................................................................................... 4

1.5 Vollständigkeit des Dokumentes ............................................................................. 4

1.6 Hinweise zur Ausstellung einer Rechnung ............................................................. 4

1.7 Legende .................................................................................................................. 5


**2** **Definitionen ........................................................................................................... 6**

2.1 Übergeordnete Spezifikation .................................................................................. 6

2.2 Dokumentenartbezogene Spezifikationen .............................................................. 6


**3** **Struktur .................................................................................................................. 7**

3.1 Übersicht Dokumentenstruktur ............................................................................... 7

3.2 Rahmenelement `oexDocFrame` - OEX Dokumentenrahmen ................................ 8

3.3 Rahmenelement `oexApplication` - Applikation, die das Dokument erstellt ....... 8

3.4 Rahmenelement `oexFile` - Dokumentenmappe .................................................. 8

3.5 Rahmenelement `oexDocument` - Einzelnes Dokument ........................................ 9

3.6 Rahmenelement `docHeader` - Belegkopf ............................................................. 9

3.7 Rahmenelement `hdrDocNo` - Kopf: Belegnummern ............................................ 10

3.8 Rahmenelement `hdrDateTime` - Kopf: Datums- und Zeitangaben .................... 11

3.9 Rahmenelement `hdrOrgData` - Kopf: Organisationsdaten ................................. 11

3.10 Rahmenelement `hdrAddress` - Kopf: Adressen ................................................. 11

3.11 Rahmenelement `hdrCom` - Kopf: Kommunikation ................................................ 12

3.12 Rahmenelement `hdrContact` - Kopf: Ansprechpartner ..................................... 12

3.13 Rahmenelement `hdrText` - Kopf: Texte ............................................................. 13

3.14 Rahmenelement `hdrReference` - Kopf: Verweise ............................................. 13

3.15 Rahmenelement `hdrPricing` - Kopf: Preiskalkulation ....................................... 13

3.16 Rahmenelement `hdrPayment` - Kopf: Zahlungsbedingungen ............................ 15

3.17 Rahmenelement `hdrBankData` - Kopf: Bankdaten ............................................ 16

3.18 Rahmenelement `docItem` - Belegposition .......................................................... 16

3.19 Rahmenelement `itmConfiguration` - Position: Konfigurationsdaten .............. 17


                                  - 1 

3.20 Rahmenelement `itmConfigText` - Position: Konfigurationstexte ..................... 17

3.21 Rahmenelement `itmDocNo` - Position: Belegnummern ...................................... 18

3.22 Rahmenelement `itmDateTime` - Position: Datums- und Zeitangaben ............... 18

3.23 Rahmenelement `itmOrgData` - Position: Organisationsdaten ........................... 18

3.24 Rahmenelement `itmAddress` - Position: Adressen ........................................... 19

3.25 Rahmenelement `itmCom` - Position: Kommunikation .......................................... 19

3.26 Rahmenelement `itmContact` - Position: Ansprechpartner ................................ 20

3.27 Rahmenelement `itmText` - Position: Texte ........................................................ 20

3.28 Rahmenelement `itmReference` - Position: Verweise ....................................... 20

3.29 Rahmenelement `itmPricing` - Position: Preiskalkulation ................................. 21


**4** **Anhang ................................................................................................................ 23**

4.1 Änderungshistorie ................................................................................................. 23


                                  - 2 

### **1 Einleitung**

Diese Spezifikation definiert alle Elemente, die für die Beschreibung einer Rechnung verwendet werden.

Einer Rechnung kann eine Bestellung (ORDERS) und/oder eine Lieferung (DESADV) vorausgegangen sein,
sofern diese Geschäftsfälle ebenfalls elektronisch abgewickelt wurden.

Angaben zur Rechnung erfolgen immer aus Sicht des Lieferanten.

Mitgeltende Spezifikationen (in der jeweils gültigen Version, siehe 2.1):


OEX-GLOBAL – dokumentenartübergreifende Spezifikation

Verwandte Dokumentenarten/Spezifikationen:


Anfrage (OEX-REQOTE), Angebot (OEX-QUOTES), Bestellung (OEX-ORDERS), Bestellbestätigung (OEXORDRSP), Bestelländerung (OEX-ORDCHG) und Lieferavis (OEX-DESADV).


**1.1** **Verwendung dieser Spezifikation**


In dieser Spezifikation werden speziell die Strukturen und Elemente für die Dokumentenart “INVOIC Rechnung“ beschrieben. Globale Strukturen und Elemente, die auch in anderen Dokumentenarten
Verwendung finden, werden im Detail in der übergeordneten Spezifikation “OEX-GLOBAL“ in der
korrespondierenden Version beschrieben. Nur von dort abgeleitete sowie dokumentenartbezogene
Strukturen und Elemente werden hier in dieser Spezifikation beschrieben (siehe auch 0 und 0).


**1.2** **Dateinamenkonventionen**


Als Dateinamenkonvention für die Dokumentenart “INVOIC“ gilt:

```
oex-invoic_<sender-id>_jjjjmmtt-hhmmss.xml

```

Grundlage bilden hier also die Dokumentenart sowie Datum und Zeit (24-Stunden-Format) der Erstellung der
Datei. Die Dateierweiterung lautet “ `xml` “.
`<sender-id>` ist der variable Teil des Dateinamens, der vom Sender der Datei vergeben werden muss. Die
max. Länge beträgt 20 Zeichen. Hierbei kann es sich beispielsweise um eine fortlaufende Nummerierung
des Senders handeln, der Kundennummer oder der Lieferantennummer.
Nur Ziffern, Buchstaben und Bindestriche sind hierbei erlaubt.
Mit diesen Angaben können dann auch gerade in einem Fehlerfall Rückschlüsse gezogen werden, um was
für eine Art der Datei es sich handelt, von wem sie kommt und wann sie erstellt wurde.

Beispiele: `oex-invoic_R000034956_20060112-050954.xml`
```
      oex-invoic_REN-9564154_20060809-174205.xml

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
xsi:noNamespaceSchemaLocation="oex-invoic_<Major>.<Minor>.<Build>.xsd">

```

Die Einbindung des übergeordneten Schemas ist bereits im dokumentenartbezogenen Schema definiert.


Gültige Version des XML Schemas


Für diese Spezifikation gilt das dokumentenartbezogene Schema in der Version 3.1.0
**`oex-invoic_3.1.0.xsd`** bzw. bei Änderungen des Schemas ohne Auswirkung auf diese Spezifikation,
das Schema mit der höchsten Build-Nummer.


**1.4** **Prüfmechanismen**


Bei der Verwendung entsprechender XML-Parser, kann zur Prüfung eines OEX-INVOIC-Dokuments (XMLDatei) das jeweils gültige XML Schema (XS) verwendet werden.
Das Schema wird von den entsprechenden Spezifikationen abgeleitet und als Prüftool bezüglich der
Elementstruktur und Datendefinition bereitgestellt. Darüber hinausgehende Prüfungen auf logische Inhalte
und Abhängigkeiten, sowie ein Mapping der Daten unterliegen der jeweils verwendeten Applikation.


**1.5** **Vollständigkeit des Dokumentes**
Das Dokument wird grundsätzlich vollständig übertragen, d.h. auch mit Positionen (oder Daten), die keine
Änderungen beinhalten ( `aAction = N` ), insbesondere, wenn das Dokument wegen Änderungen nochmal
übermittelt wird.


**1.6** **Hinweise zur Ausstellung einer Rechnung**


Für die Ausstellung einer Rechnung finden die im jeweiligen Land gültigen Gesetze, Richtlinien und
Empfehlungen Anwendung. In Deutschland bspw. § 14 UStG (Umsatzsteuergesetz).
Auf folgende 2 Punkte soll insbesondere hingewiesen werden:


**Echtheit und Unversehrtheit**


Bei einer auf elektronischem Weg übermittelten Rechnung müssen die Echtheit der Herkunft und die
Unversehrtheit des Inhalts gewährleistet sein (in Deutschland § 14 Abs. 3 UStG). Sie sind u.a.
Voraussetzungen für den Vorsteuerabzug und für Aufbewahrungsfristen (Revisionen/Steuerprüfung).
Elektronische Rechnungen müssen daher mit einer „qualifizierten Signatur“ versehen werden.
Die einzelnen Verfahren für die Signatur (bspw. Massensignatur oder Zeitstempel) sind nicht Bestandteil
dieser Spezifikation, da sie nicht Bestandteil des Datenformats sind und vom jeweiligen Versender der
Rechnung entsprechend frei gewählt werden können.


                                  - 4 

**Pflichtangaben**


Es gibt Regelungen (in Deutschland § 14 Abs. 4 UStG), die sich auf die Pflichtangaben einer Rechnung
beziehen. Diese können bspw. sein: Vollständige Adresse beider Geschäftspartner, eindeutige Rechnungsnummer des Rechnungsstellers, Rechnungsdatum, Steuernummer, Umsatzsteueridentifikationsnummer,
Menge und Art der gelieferten Ware oder der erbrachten Leistung, sowie deren Zeitpunkt (Leistungserstellungsdatum), Steuersatz, Steuerbetrag.


**1.7** **Legende**


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
|**`Sch`**|Schlüsselelement|**! **|Element muss vorhanden sein und einen Wert<br>beinhalten, außerdem muss das Element<br>zusammen mit Wert und ggfs. speziell angege-<br>benen Pflichtattributen eindeutig bei Wieder-<br>holungen innerhalb eines Rahmenelements sein.<br>Sind mehrere Elemente so gekennzeichnet,<br>bilden sie zusammen einen eindeutigen Wert.<br>(Wirkung wie bei einem Primärschlüssel)|
|**`Mod`**|Modifikation|**<empty>**|Element ist dokumentartbezogen und/oder<br>verweist auf den angegebenen Typ aus der<br>übergeordneten Spezifikation.|
|**`Mod`**|Modifikation|**D **|Element leitet sich vom angegebenen Typ aus<br>der übergeordneten Spezifikation ab und ist<br>dokumentenartbezogen angepasst (abgeleiteter<br>Typ).|



Elemente, die als optional gekennzeichnet sind (Pfl=<empty>), müssen auf Grund gesetzlicher
Gegebenheiten bezüglich des Geschäftsfalls ggfs. bestückt werden (abhängig von Land und/oder
Region/Bundesland/Bundesstaat).


                                  - 5 

### **2 Definitionen**

**2.1** **Übergeordnete Spezifikation**


Die dokumentenartübergreifenden Spezifikationen sind dem Dokument **OEX-GLOBAL** in der jeweiligen
gültigen Version 3.1.x zu entnehmen, wobei „x“ für die höchste Build-Versionsnummer steht.


**2.2** **Dokumentenartbezogene Spezifikationen**


Spezifikation des Dokuments “INVOIC“ – Rechnung

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
`├─── docHeader` Belegkopf
`│` `├─── hdrDocNo` Kopf: Belegnummern
`│` `├─── hdrDateTime` Kopf: Datums- und Zeitangaben
`│` `├─── hdrOrgData` Kopf: Organisationsdaten
`│` `├─── hdrAddress` Kopf: Adressen
`│` `│` `├─── hdrCom` Kopf: Kommunikation
`│` `│` `└─── hdrContact` Kopf: Ansprechpartner
`│` `│` `└─── hdrCom` Kopf: Kommunikation
`│` `├─── hdrText` Kopf: Texte
`│` `├─── hdrReference` Kopf: Verweise
`│` `├─── hdrPricing` Kopf: Preiskalkulation
`│` `├─── hdrPayment` Kopf: Zahlungsbedingungen
`│` `└─── hdrBankData` Kopf: Bankdaten
`└─── docItem` Belegposition
`├─── itmConfiguration` Position: Konfigurationsdaten
`│` `└─── itmConfigText` Position: Konfigurationstexte
`├─── itmDocNo` Position: Belegnummern
`├─── itmDateTime` Position: Datums- und Zeitangaben
`├─── itmOrgData` Position: Organisationsdaten
`├─── itmAddress` Position: Adressen
`│` `├─── itmCom` Position: Kommunikation
`│` `└─── itmContact` Position: Ansprechpartner
`│` `└─── itmCom` Position: Kommunikation
`├─── itmText` Position: Texte
`├─── itmReference` Position: Verweise
`└─── itmPricing` Position: Preiskalkulation


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
|**`vDocumentType`**|`INVOIC`|`INVOIC`||||Rechnung|
|~~**`oexDocument`**~~|~~**`Document`**~~|**1+**|**X **|||**Einzelnes Dokument**|



                                  - 8 

**3.5** **Rahmenelement** **`oexDocument`** **– Einzelnes Dokument**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`oexDocument`**~~|~~**`Document`**~~|**1+**|**X **|**! **||**Einzelnes Dokument**<br>_(in Bezug auf die Dokumentenart)_|
|~~**`oexDocument`**~~|**Attribut**|**Attribut**|||||
|~~**`oexDocument`**~~|`aDocNo`|`aDocNo`|X|!||Laufende Nummer des Dokuments|
|~~**`oexDocument`**~~|`aItemCount`|`aItemCount`|X|||Anzahl Positionen im Dokument<br>_(docItem)_|
|~~**`oexDocument`**~~|`aAction`|`aAction`|X|||Aktion|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`docHeader`**|**`Header`**|**1 **|**X **|||**Belegkopf**|
|~~**`docItem`**~~|~~**`Item`**~~|**1+**|**X **|||**Belegposition**|



**3.6** **Rahmenelement** **`docHeader`** **– Belegkopf**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`docHeader`**|**`Header`**|**1 **|**X **|||**Belegkopf**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vInvoiceNumber`**|**`DocNo`**|**1 **|**X **|||**Rechnungsnummer**<br>_Eindeutige Nummer der Rechnung des_<br>_Lieferanten._|
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
|**`vInvoiceType`**|**`InvoiceType`**|**1 **|**X **|||**Rechnungsart**|
|**`vGrossWeight`**|**`GrossWeight`**|**1 **|**2 **|||**Bruttogewicht (gesamt)**|
|**`vNetWeight`**|**`NetWeight`**|**1 **|**2 **|||**Nettogewicht (gesamt)**|
|**`vUnitWeight`**|**`UnitWeight`**|**1 **|**2 **|||**Gewichtseinheit**|
|**`vVolume`**|**`Volume`**|**1 **|**3 **|||**Volumen (gesamt)**|
|**`vUnitVolume`**|**`UnitVolume`**|**1 **|**3 **|||**Volumeneinheit**|
|~~**`hdrDocNo`**~~|~~**`DocNo`**~~|*** **|||**D **|**Kopf: Belegnummern**|
|~~**`hdrDateTime`**~~|~~**`DateTime`**~~|**4+**|**X **|||**Kopf: Datums- u. Zeitangaben**|
|~~**`hdrOrgData`**~~|~~**`OrgData`**~~|*** **||||**Kopf: Organisationsdaten**|
|~~**`hdrAddress`**~~|~~**`Address`**~~|**2+**|**X **|||**Kopf: Adressen**|
|~~**`hdrText`**~~|~~**`Text`**~~|*** **||||**Kopf: Texte**|
|~~**`hdrReference`**~~|~~**`Reference`**~~|*** **||||**Kopf: Verweise**|
|~~**`hdrPricing`**~~|~~**`Pricing`**~~|*** **|||**D **|**Kopf: Preiskalkulation**|



                                  - 9 

|hdrPayment|Payment|3*|Col4|Col5|Col6|Kopf: Zahlungsbedingungen|
|---|---|---|---|---|---|---|
|~~**`hdrBankData`**~~|~~**`BankData`**~~|*** **||||**Kopf: Bankdaten**|


Der Belegkopf enthält alle wichtigen Referenzen des Dokuments.
Über die Rechnungsart ( `vInvoiceType` ) wird angegeben, ob es sich um eine Rechnung oder Gutschrift
handelt (die Gutschrift ist der Rechnung gleichgestellt). In beiden Fällen wird mit positiven Mengen- und
Betragsangaben gearbeitet.

Erläuterungen zu Pflichtangaben:


**1** Die Ortsangabe zu Inco Terms muss angegeben werden, sobald die Lieferbedingung dies fordert.
**2** Die Gewichtseinheit muss angegeben werden, sobald Bruttogewicht und/oder Nettogewicht
angegeben wird.
**3** Die Volumeneinheit muss angegeben werden, sobald das Volumen angegeben wird.


**3.7** **Rahmenelement** **`hdrDocNo`** **– Kopf: Belegnummern**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrDocNo`**|**`DocNo`**|*** **|||**D **|**Kopf: Belegnummern**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDocNoType`**|**`DocNoType`**|**1 **|**X **|||**Belegnummernart**|
|**`vDocNo`**|**`DocNo`**|**1 **|**X **|||**Belegnummer**|



Dieses Rahmenelement enthält die Belegnummern der Vorgängerbelege in der Abfolge des Geschäftsfalls
und/oder zusätzliche Belege als Referenz für die Rechnung. Die Angabe der Belegposition entfällt hier auf
Kopfebene. Es kann somit nicht auf Positionen andere Belege referenziert werden. Eine solche
Referenzierung erfolgt auf Positionsebene ( `itmDocNo` ) mit Angabe der Belegposition.

Beispiel:
Setzt sich die Rechnung aus Positionen verschiedener Lieferungen oder Aufträgen zusammen, darf keine
Angabe hierüber im Kopf erfolgen, sondern muss auf Positionsebene ( `itmDocNo` ) mit Angabe der
Belegposition erfolgen.

Die Angabe der Rechnungsnummer selbst ist hier nicht erlaubt, da diese eindeutig für den Beleg ist und
bereits im Rahmenelement Belegkopf ( `docHeader` ) im Element `vInvoiceNumber` angegeben wird.

Pflichtangaben, sofern vorhanden und eindeutig (nicht aus verschiedenen Belegen):
Bestellnummer (entfällt bspw. bei telefonischer Bestellung/-Änderung ohne Angabe einer eindeutigen
Bestellnummer seitens des Kunden)
```
    <vDocNoType aDocContext="S"> ORD </vDocNoType>
```

Bestellbestätigungsnummer (entfällt bspw. falls es sich um eine Lieferung ohne Auftragsbezug handelt)
```
    <vDocNoType aDocContext="S"> CNF </vDocNoType>
```

Liefernummer (entfällt bspw. bei nicht lieferrelevanten Bestellpositionen)
```
    <vDocNoType aDocContext="S"> DEL </vDocNoType>

```

                               - 10 

**3.8** **Rahmenelement** **`hdrDateTime`** **– Kopf: Datums- und Zeitangaben**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrDateTime`**|**`DateTime`**|**4+**|**X **|**! **||**Kopf: Datums- und Zeitangaben**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDateTimeType`**|**`DateTimeType`**|**1 **|**X **|**! **||**Typ Datum/Zeit**|
|**`vTimeZone`**|**`TimeZone`**|**1 **|**X **|||**Zeitzone**|
|**`vDateValue`**|**`Date`**|**1 **|**X **|||**Datumsangabe**|
|**`vTimeValue`**|**`Time`**|**1 **||||**Zeitangabe**|



Dieses Rahmenelement dient zur Übergabe von Datums- und Zeitangaben eines Rechnungskopfes.
Als Pflichtangaben sind hier das Belegdatum ( `DOC` ), Rechnungsdatum ( `INV` ), Fälligkeitsdatum ( `DUE` ) und
Leistungserstellungsdatum ( `DSR` ) erforderlich.


**3.9** **Rahmenelement** **`hdrOrgData`** **– Kopf: Organisationsdaten**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrOrgData`**|**`OrgData`**|*** **||**! **||**Kopf: Organisationsdaten**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vOrgDataType`**|**`OrgDataType`**|**1 **|**X **|**! **||**Arten Organisationsdaten**|
|**`vOrgDataValue`**|**`Value`**|**1 **|**X **|||**Wert Organisationsdaten**|



Verwendungsbeispiele: Kommissionsangabe ( `COM` ) `"Kommission Schmidt"`
Projektnummer ( `PJN` ) `"576134"`
Verkaufsorganisation ( `SOR` ) `"ABCD"`


**3.10** **Rahmenelement** **`hdrAddress`** **– Kopf: Adressen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrAddress`**|**`Address`**|**2+**|**X **|**! **||**Kopf: Adressen**|


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



                               - 11 

|vPostalCodePOBox|PostalCodePOB|1|Col4|Col5|Col6|Postleitzahl Postfach|
|---|---|---|---|---|---|---|
|**`vPOBox`**|**`Value`**|**1 **||||**Postfachnummer**|
|**`vTaxCode`**|**`Value`**|**1 **||||**Steuernummer Finanzamt**|
|**`vTaxCodeEU`**|**`Value`**|**1 **||||**Steuernummer EU / USt-IdNr.**|
|**`vTaxCodeUS`**|**`Value`**|**1 **||||**Steuernummer US / Jurisdiction**|
|~~**`hdrCom`**~~|~~**`Com`**~~|*** **||||**Kopf: Kommunikation**|
|~~**`hdrContact`**~~|~~**`Contact`**~~|*** **||||**Kopf: Ansprechpartner**|


Als Pflichtangaben werden vom Lieferanten hier die Adresse des Rechnungsempfängers (IN) und die
Lieferadresse (SH) angegeben.
Es empfiehlt sich auch alle von den Stammdaten abweichenden Adressen anzugeben.
In der Regel sind die Adressen insbesondere Auftraggeber (SO) und Lieferant (SU) den beiden
Geschäftspartnern bekannt und als Stammdaten hinterlegt und bedürfen nicht unbedingt einer Übertragung,
sie korrespondieren auch zur Kundennummer bzw. Lieferantennummer des Belegkopfes ( `docHeader` ).
Ggfs. wird mit einer Adresse aber auch eine für diese Rechnung zuständige Kontaktperson übermittelt.


**3.11** **Rahmenelement** **`hdrCom`** **– Kopf: Kommunikation**

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


**3.12** **Rahmenelement** **`hdrContact`** **– Kopf: Ansprechpartner**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrContact`**|**`Contact`**|*** **||||**Kopf: Ansprechpartner**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vContactType`**|**`ContactType`**|**1 **|**X **|||**Typ Ansprechpartner**|
|**`vContactNumber`**|**`Value`**|**1 **||||**Nummer Ansprechpartner**|
|**`vTitle`**|**`Value`**|**1 **||||**Anrede**|
|**`vFirstName`**|**`FirstName`**|**1 **||||**Vorname**|
|**`vLastName`**|**`LastName`**|**1 **|**X **|||**Nachname**|
|~~**`hdrCom`**~~|~~**`Com`**~~|*** **||||**Kopf: Kommunikation**|



Zur Angabe der Ansprechpartner, die für die Abwicklung des Geschäftsfalls erforderlich sind oder
organisatorisch zugeordnet werden (z.B. ein oder mehrere Vertriebsmitarbeiter bezüglich Provisionen).


                               - 12 

**3.13** **Rahmenelement** **`hdrText`** **– Kopf: Texte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrText`**|**`Text`**|*** **||**! **||**Kopf: Texte**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vTextType`**|**`TextType`**|**1 **|**X **|**! **||**Textart**|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|**! **||**Textsprache**|
|**`vTextContent`**|**`TextContent`**|**1+**|**X **|||**Textinhalt**|



**3.14** **Rahmenelement** **`hdrReference`** **– Kopf: Verweise**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrReference`**|**`Reference`**|*** **||||**Kopf: Verweise**|


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
           "http://www.moebel-huber.de/orderstatus.html?p=987654321"

```

**3.15** **Rahmenelement** **`hdrPricing`** **– Kopf: Preiskalkulation**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrPricing`**|**`Pricing`**|*** **|||**D **|**Kopf: Preiskalkulation**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vConditionType`**|**`ConditionType`**|**1 **|**X **|||**Konditionsart**|
|**`vConditionValue`**|**`ConditionValue`**|**1 **|**X **|||**Konditionswert**|
|**`vConditionRate`**|**`ConditionRate`**|**1 **||||**Konditionssatz**|
|**`vCondCurrency`**|**`CondCurrency`**|**1 **||||**Konditionswährung**|
|**`vConditionText`**|**`ConditionText`**|**1 **||||**Konditionsbezeichnung**<br>_(in Belegsprache)_|



In diesem Rahmenelement wird die Netto-Summe ( `TNET`, ggfs. `TNEH` ) sowie der Endbetrag ( `TOTL` ) der
Positionen einer Rechnung angegeben. Sie dienen als Kontrollsummen bei der Verarbeitung des
Dokuments.
Des Weiteren werden Konditionen angegeben, die die Mehrwertsteuer ( `TTNE`, `TTAX` ) betreffen.
Andere Angaben, wie z.B. Rabatte, sind optional, können aber auch zur Kontrolle bei der Verarbeitung
herangezogen werden.
Die Währung wird, wenn hier nicht anders angegeben, durch die Belegwährung vorgegeben.
Der Konditionsbereich ist auf den Verkauf beschränkt ( `aCondArea="S"` ).


                               - 13 

Hinweis: Im Gegensatz zu den Preisangaben auf Positionsebene entfallen in diesem Rahmenelement die
Unterelemente für Preiseinheit und Mengeneinheit, da es sich hier immer um Summenkonditionen handelt.

Beispiel 1 – Angabe verschiedener Gesamtwerte der Rechnung:
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

Beispiel 2 – Angabe weiterer Konditionen der Rechnung als Summe der Rechnungspositionen:
Bruttowert der Position 1 beträgt € 125,00 mit Steuerkennzeichen 1, 19%
Rabattsatz der Position 1 beträgt 20% als Grundrabatt
Nettowert der Position 1 beträgt € 100,00
Bruttowert der Position 2 beträgt € 200,00 mit Steuerkennzeichen 1, 19%
Rabattsatz der Position 2 beträgt 25% als Grundrabatt
Nettowert der Position 2 beträgt € 150,00

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="S"> TGRO </vConditionType>
    <vConditionValue> 325.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="2">
    <vConditionType aCondArea="S" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 75.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Grundrabatt </vConditionText>
</hdrPricing>
<hdrPricing aCondNo="3">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="4">
    <vConditionType aCondArea="S" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="5">
    <vConditionType aCondArea="S" aCondRef="4" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue> 47.50 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="6">
    <vConditionType aCondArea="S"> TOTL </vConditionType>
    <vConditionValue> 297.50 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
```

                               - 14 

Die Rabatte werden hier als Gesamtwert absolut aus den Positionen mit gleicher Art des Abschlags
( `aTypeDis` ) angegeben.

Beispiel 3 – Angabe verschiedener Gesamtwerte der Rechnung bei unterschiedlichen Steuersätzen:
Nettowert der Position 1 beträgt € 200,00 mit Steuerkennzeichen 1, 19%
Nettowert der Position 2 beträgt € 150,00 mit Steuerkennzeichen 2, 7%

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 350.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="2">
    <vConditionType aCondArea="S" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 200.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="3">
    <vConditionType aCondArea="S" aCondRef="2" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue> 38.00 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="4">
    <vConditionType aCondArea="S" aTaxCode="2"> TTNE </vConditionType>
    <vConditionValue> 150.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="5">
    <vConditionType aCondArea="S" aCondRef="4" aTaxCode="2"> TTAX </vConditionType>
    <vConditionValue> 10.50 </vConditionValue>
    <vConditionRate> 7.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="6">
    <vConditionType aCondArea="S"> TOTL </vConditionType>
    <vConditionValue> 398.50 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>

```

**3.16** **Rahmenelement** **`hdrPayment`** **– Kopf: Zahlungsbedingungen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrPayment`**|**`Payment`**|**3***||**! **||**Kopf: Zahlungsbedingungen**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vPaymentPart`**|**`PaymentPart`**|**1 **|**X **|**! **||**Bestandteil der Zahlungsbedingung**|
|**`vPaymentRate`**|**`PaymentRate`**|**1 **|**X **|||**Skonto-Satz (%)**|
|**`vPaymentDays`**|**`PaymentDays`**|**1 **|**X **|||**Anzahl Tage (Zahlungsziel)**|



                               - 15 

**3.17** **Rahmenelement** **`hdrBankData`** **– Kopf: Bankdaten**

|OEX-Elementtyp|Domäne|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrBankData`**|**`BankData`**|*** **||**! **||**Kopf: Bankdaten**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vBankName`**|**`BankName`**|**1 **|**X **|**! **||**Name der Bank**|
|**`vBankCountry`**|**`BankCountry`**|**1 **|**X **|**! **||**Land der Bank**|
|**`vBankLocation`**|**`BankLocation`**|**1 **|**X **|**! **||**Sitz der Bank**|
|**`vSwiftBic`**|**`SwiftBic`**|**1 **|**1 **|||**SWIFT-BIC Int. Bankenschlüssel**|
|**`vIban`**|**`Iban`**|**1 **|**1 **|||**IBAN Internationale Kontonummer**|
|**`vBankKey`**|**`BankKey`**|**1 **|**2 **|||**Bankenschlüssel (Bankleitzahl)**|
|**`vBankAccount`**|**`BankAccount`**|**1 **|**2 **|||**Kontonummer**|
|**`vAccountHolder`**|**`AccountHolder`**|**1 **|**X **|||**Kontoinhaber**|



Erläuterungen zu Pflichtangaben:


**1 + 2** Es werden immer paarweise SWIFT-BIC und IBAN angegeben _oder_ Bankenschlüssel und
Kontonummer _oder_ beide Paare.


**3.18** **Rahmenelement** **`docItem`** **– Belegposition**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`docItem`**~~|~~**`Item`**~~|**1+**|**X **|**! **||**Belegposition**|
|~~**`docItem`**~~|**Attribut**|**Attribut**|||||
|~~**`docItem`**~~|`aItemNo`|`aItemNo`|X|!||Laufende Nummer der Belegposition|
|~~**`docItem`**~~|`aAction`|`aAction`||||Aktion|
|~~**`docItem`**~~|`aUUID`|`aUUID`||||Global eindeutiger Identifikator|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vInvoiItemNumber`**|**`DocItemNo`**|**1 **|**X **|||**Nummer der Rechnungsposition**<br>_Eindeutige Positionsnummer innerhalb_<br>_der Rechnung._|
|**`vInvoiTopLevelNo`**|**`DocItemNo`**|**1 **||||**Übergeordnete Nummer der**<br>**Rechnungsposition**<br>_Verweis auf die übergeordnete_<br>_Positionsnummer bei Verwendung von_<br>_Unterpositionen oder Stücklisten._|
|**`vClientArticleNo`**|**`ClientArtNo`**|**1 **||||**Kundenartikelnummer**|
|**`vVendorArticleNo`**|**`VendorArtNo`**|**1 **|**X **|||**Lieferantenartikelnummer**|
|**`vVendorID`**|**`VendorID`**|**1 **|**X **|||**Lieferantenkennung**|
|**`vVendorSeries`**|**`VendorSeries`**|**1 **|**X **|||**Lieferantenserie**|
|**`vCatalogId`**|**`CatalogId`**|**1 **||||**Katalog-ID**|
|**`vArticleEAN`**|**`EAN_Article`**|**1 **||||**EAN des Artikels**|
|**`vInvoiQuantity`**|**`Quantity`**|**1 **|**X **|||**Rechnungsmenge**|
|**`vInvoiUnit`**|**`QuantUnit`**|**1 **|**X **|||**Rechnungsmengeneinheit**|
|**`vGrossWeight`**|**`GrossWeight`**|**1 **|**1 **|||**Bruttogewicht (gesamt)**|
|**`vNetWeight`**|**`NetWeight`**|**1 **|**1 **|||**Nettogewicht (gesamt)**|
|**`vUnitWeight`**|**`UnitWeight`**|**1 **|**1 **|||**Gewichtseinheit**|
|**`vVolume`**|**`Volume`**|**1 **|**2 **|||**Volumen (gesamt)**|
|**`vUnitVolume`**|**`UnitVolume`**|**1 **|**2 **|||**Volumeneinheit**|
|**`vClassification`**|**`Classification`**|<br>*** **||||**Klasse/Kategorie der Bestellposition**|
|~~**`itmConfiguration`**~~|~~**`Config`**~~|*** **||||**Position: Konfigurationsdaten**|



                               - 16 

|itmDocNo|DocNo|*|Col4|Col5|Col6|Position: Belegnummern|
|---|---|---|---|---|---|---|
|~~**`itmDateTime`**~~|~~**`DateTime`**~~|*** **||||**Position: Datums- u. Zeitangaben**|
|~~**`itmOrgData`**~~|~~**`OrgData`**~~|*** **||||**Position: Organisationsdaten**|
|~~**`itmAddress`**~~|~~**`Address`**~~|*** **||||**Position: Adressen**|
|~~**`itmText`**~~|~~**`Text`**~~|**1+**|**X **|||**Position: Texte**|
|~~**`itmReference`**~~|~~**`Reference`**~~|*** **||||**Position: Verweise**|
|~~**`itmPricing`**~~|~~**`Pricing`**~~|*** **||||**Position: Preiskalkulation**|


Basisdaten der Belegposition.
Durch das Zusammenspiel von Positionsnummer mit der übergeordneten Positionsnummer lässt sich eine
„beliebig“ tiefe Hierarchiestruktur abbilden. Spezielle Strukturen der Positionsnummer lassen sich hierdurch
aber nicht abbilden, schon gar nicht Verzeichnisstrukturen. Positionsnummernangaben wie bspw. “100.A.101“ können über die Organisationsdaten Typ `POS` übermittelt werden, in wie weit eine andere Applikation
diese verarbeiten, zurückliefern kann oder gar für sich selbst verwendet, bleibt jedoch offen.

Erläuterungen zu Pflichtangaben:


**1** Die Gewichtseinheit muss angegeben werden, sobald Bruttogewicht und/oder Nettogewicht
angegeben wird.
**2** Die Volumeneinheit muss angegeben werden, sobald das Volumen angegeben wird.


**3.19** **Rahmenelement** **`itmConfiguration`** **– Position: Konfigurationsdaten**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmConfiguration`**|**`Config`**|*** **||||**Position: Konfigurationsdaten**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vClassID`**|**`Value`**|**1 **||||**Merkmalsklasse**|
|**`vOptionID`**|**`Value`**|**1 **|**X **|||**Merkmal**|
|**`vOptionEAN`**|**`EAN_Option`**|**1 **||||**EAN des Merkmals**|
|**`vValueID`**|**`Value`**|**1 **|**X **|||**Merkmalswert**|
|**`vValueEAN`**|**`EAN_Value`**|**1 **||||**EAN des Merkmalswertes**|
|~~**`itmConfigText`**~~|~~**`ConfigText`**~~|*** **||||**Konfigurationstexte**|



**3.20** **Rahmenelement** **`itmConfigText`** **– Position: Konfigurationstexte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmConfigText`**|**`ConfigText`**|*** **||||**Konfigurationstexte**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|||**Textsprache**|
|**`vOptionText`**|**`OptionText`**|**1 **|**X **|||**Merkmalstext**|
|**`vValueText`**|**`ValueText`**|*** **||||**Merkmalswertetext**<br>Hier fällt der Text unter Umständen<br>weg, wenn es sich um einen frei<br>bewertbaren Merkmalswert handelt.|



                               - 17 

**3.21** **Rahmenelement** **`itmDocNo`** **– Position: Belegnummern**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmDocNo`**|**`DocNo`**|*** **||||**Position: Belegnummern**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDocNoType`**|**`DocNoType`**|**1 **|**X **|||**Belegnummernart**|
|**`vDocNo`**|**`DocNo`**|**1 **|**X **|||**Belegnummer**|
|**`vDocLine`**|**`DocItemNo`**|**1 **||||**Belegposition**|



Dieses Rahmenelement enthält die Positionsnummern der Vorgängerbelege in der Abfolge des Geschäftsfalls und/oder zusätzliche Belege als Referenz für die Rechnung. Die Angabe der Positionsnummern ist
immer erforderlich, solange es sich nicht um einen Beleg ohne Positionsangaben handelt.


**3.22** **Rahmenelement** **`itmDateTime`** **– Position: Datums- und Zeitangaben**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmDateTime`**|**`DateTime`**|*** **||**! **||**Position: Datums- und Zeitangaben**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDateTimeType`**|**`DateTimeType`**|**1 **|**X **|**! **||**Typ Datum/Zeit**|
|**`vTimeZone`**|**`TimeZone`**|**1 **|**X **|||**Zeitzone**|
|**`vDateValue`**|**`Date`**|**1 **|**X **|||**Datumsangabe**|
|**`vTimeValue`**|**`Time`**|**1 **||||**Zeitangabe**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrDateTime` abweicht oder zusätzliche positionsrelevante Informationen enthält.


**3.23** **Rahmenelement** **`itmOrgData`** **– Position: Organisationsdaten**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmOrgData`**|**`OrgData`**|*** **||**! **||**Position: Organisationsdaten**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vOrgDataType`**|**`OrgDataType`**|**1 **|**X **|**! **||**Art der Organisationsdaten**|
|**`vOrgDataValue`**|**`Value`**|**1 **|**X **|||**Wert Organisationsdaten**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrOrgData` abweicht oder zusätzliche positionsrelevante Informationen enthält.


                               - 18 

**3.24** **Rahmenelement** **`itmAddress`** **– Position: Adressen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmAddress`**|**`Address`**|*** **||**! **||**Position: Adressen**|


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
|~~**`itmCom`**~~|~~**`Com`**~~|*** **||||**Position: Kommunikation**|
|~~**`itmContact`**~~|~~**`Contact`**~~|*** **||||**Position: Ansprechpartner**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrAddress` abweicht oder zusätzliche positionsrelevante Informationen enthält.

**3.25** **Rahmenelement** **`itmCom`** **– Position: Kommunikation**

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


                               - 19 

**3.26** **Rahmenelement** **`itmContact`** **– Position: Ansprechpartner**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmContact`**|**`Contact`**|*** **||||**Position: Ansprechpartner**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vContactType`**|**`ContactType`**|**1 **|**X **|||**Typ Ansprechpartner**|
|**`vContactNumber`**|**`Value`**|**1 **||||**Nummer Ansprechpartner**|
|**`vTitle`**|**`Value`**|**1 **||||**Anrede**|
|**`vFirstName`**|**`FirstName`**|**1 **||||**Vorname**|
|**`vLastName`**|**`LastName`**|**1 **|**X **|||**Nachname**|
|~~**`itmCom`**~~|~~**`Com`**~~|*** **||||**Position: Kommunikation**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrAddress` abweicht oder zusätzliche positionsrelevante Informationen enthält.


**3.27** **Rahmenelement** **`itmText`** **– Position: Texte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmText`**|**`Text`**|**1+**|**X **|**! **||**Position: Texte**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vTextType`**|**`TextType`**|**1 **|**X **|**! **||**Textart**|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|**! **||**Textsprache**|
|**`vTextContent`**|**`TextContent`**|**1+**|**X **|||**Textinhalt**|



Es wird wenigstens der Kurztext übermittelt bei einem Standardartikel, auf den Langtext kann in diesem Fall
verzichtet werden.
Anders verhält es sich bei modifizierten Artikeln und Kundenartikeln.
(vgl. globalen OEX-Werttyp `VendorArtNo`  `aStatus` ).


**3.28** **Rahmenelement** **`itmReference`** **– Position: Verweise**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmReference`**|**`Reference`**|*** **||||**Position: Verweise**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vReferenceType`**|**`ReferenceType`**|**1 **|**X **|||**Typ Verweis**|
|**`vReferenceValue`**|**`Value`**|**1 **|**X **|||**Wert Verweis**|
|**`vReferenceDesc`**|**`Value`**|**1 **|**X **|||**Bezeichnung**<br>_(in Belegsprache)_|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrReference` abweicht oder zusätzliche positionsrelevante Informationen enthält.


                               - 20 

**3.29** **Rahmenelement** **`itmPricing`** **– Position: Preiskalkulation**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmPricing`**|**`Pricing`**|*** **||||**Position: Preiskalkulation**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vConditionType`**|**`ConditionType`**|**1 **|**X **|||**Konditionsart**|
|**`vConditionValue`**|**`ConditionValue`**|**1 **|**X **|||**Konditionswert**|
|**`vConditionRate`**|**`ConditionRate`**|**1 **||||**Konditionssatz**|
|**`vCondCurrency`**|**`CondCurrency`**|**1 **||||**Konditionswährung**|
|**`vPriceUnit`**|**`PriceUnit`**|**1 **||||**Preiseinheit**|
|**`vQuantUnit`**|**`QuantUnit`**|**1 **||||**Mengeneinheit**|
|**`vConditionText`**|**`ConditionText`**|**1 **||||**Konditionsbezeichnung**<br>_(in Belegsprache)_|



Angabe des Nettowertes ( `TNET` ) und des Endbetrages ( `TOTL` ) der Position, die als Kontrollwert bei der
Verarbeitung des Dokuments dienen.
Des Weiteren werden Konditionen angegeben, die die Mehrwertsteuer ( `TTNE`, `TTAX` ) betreffen.
Andere Angaben, wie z.B. Rabatte, sind optional, können aber auch zur Kontrolle bei der Verarbeitung
herangezogen werden.
Die Währung wird, wenn hier nicht anders angegeben, durch die Belegwährung vorgegeben.
Der Konditionsbereich ist auf den Verkauf beschränkt ( `aCondArea="S"` ).
Die Mengeneinheit wird, wenn hier nicht anders angegeben, durch die Rechnungsmengeneinheit
( `vInvoiUnit` ) vorgegeben.


Beispiel 1 – Angabe verschiedener Gesamtwerte der Rechnungsposition:
Nettoeinzelpreis der Position beträgt € 50,00 mit Steuerkennzeichen 1, 19%
Rechnungsmenge = 3
Rechnungsmengeneinheit = C62

```
<itmPricing aCondNo="1">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 150.00 </vConditionValue>  ! TNET = Rechnungsmenge x Nettoeinzelpreis
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="2">
    <vConditionType aCondArea="S" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 150.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="3">
    <vConditionType aCondArea="S" aCondRef="2" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue> 28.50 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="S"> TOTL </vConditionType>
    <vConditionValue> 178.50 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

                               - 21 

Beispiel 2 – Diverse Angaben von Konditionen einer Rechnungsposition:
Bruttoeinzelpreis der Position beträgt € 50,00 mit Steuerkennzeichen 1, 19%
Rabatt (als Grundrabatt) von 20% auf den Bruttoeinzelpreis
Rabatt (als Ausstellungsrabatt) von 5% auf den bereits rabattierten Preis
Rechnungsmenge = 2
Rechnungsmengeneinheit = C62

```
<itmPricing aCondNo="1">
    <vConditionType aCondArea="S"> SGRO </vConditionType>
    <vConditionValue> 50.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="2">
    <vConditionType aCondArea="S" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 10.00 </vConditionValue>
    <vConditionRate> 20.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
    <vConditionText> Grundrabatt <vConditionText>
</itmPricing>
<itmPricing aCondNo="3">
    <vConditionType aCondArea="S" aCondRef="2" aTypeDis="D1" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vConditionRate> 5.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
    <vConditionText> Ausstellungsrabatt <vConditionText>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="S"> SNET </vConditionType>
    <vConditionValue> 38.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
# Hier kommt nun die Rechnungsmenge von 2 Stück zum Tragen: TNET = SNET x 2
<itmPricing aCondNo="5">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 76.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="6">
    <vConditionType aCondArea="S" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 76.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="7">
    <vConditionType aCondArea="S" aCondRef="6" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue> 14.44 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="8">
    <vConditionType aCondArea="S"> TOTL </vConditionType>
    <vConditionValue> 90.44 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

                               - 22 

### **4 Anhang**

**4.1** **Änderungshistorie**

|Version|Änderungen|
|---|---|
|3.1.0 – 8.5.2023| <br>Kleinere Umstellungen und Erweiterungen in der Einleitung<br> <br>Präzisierung in den Rahmenelementen`hdrPricing` (Kopf: Preiskalkulation) und`itmPricing` <br>(Position: Preiskalkulation)|
|3.0.0 – 30.11.2017| <br>Globale Änderungen laut Spezifikation GLOBAL 3.0.0<br> <br>Umstrukturierung der Spezifikation<br> <br>Rahmenelement`docItem`: Element`vClientArticleNo` hat nun den Typ`ClientArtNo` (war<br>`Value`).|
|2.3.0 – 1.7.2015| <br>Globale Änderungen laut Spezifikation GLOBAL 2.3.0<br> <br>Erweiterung: Rahmenelement`docItem` (2.20 Belegposition) ergänzt um optionales Element für<br>Angabe einer Klasse oder Kategorie:`vClassification`|
|2.2.0 – 11.10.2013| <br>Globale Änderungen laut Spezifikation GLOBAL 2.2.0<br> <br>Erweiterung: Rahmenelement`docHeader` (2.8 Belegkopf) ergänzt um optionale Elemente für Kunden-<br>ID, Kunden-Klassifizierung, Lieferanten-ID und Lieferanten-Klassifizierung:`vClientID`, <br>`vClientClass`, `vSupplierID`und`vSupplierClass`. <br>Optionalen Elemente für ILN Kunde und ILN Lieferant ersetzt durch Kunden-ID und Lieferanten-ID.<br> <br>Erweiterung: Rahmenelemente`hdrAddress` (2.12 Kopf: Adressen) und`itmAddress` (2.26 Position:<br>Adressen) ergänzt um optionale Elemente für Straße 2 und Ortsteil:`vStreet2`und`vDistrict`. <br>Optionales Element für ILN Adresse:`vAddressILN` ersetzt durch neues optionales Element für<br>Adress-ID:`vAddressID`. <br> <br>Erweiterung: Rahmenelement`docItem` (2.20 Belegposition) ergänzt um optionales Element für die<br>Katalog-ID:`vCatalogId`.|
|2.1.0 – 06.11.2009| <br>Globale Änderungen laut Spezifikation GLOBAL 2.1.0<br> <br>Erweiterung: Rahmenelemente`docHeader` (2.8 Belegkopf) und`docItem` (2.20 Belegposition)<br>ergänzt um optionale Elemente für Volumen und Gewichte sowie deren Einheiten:`vGrossWeight`, <br>`vNetWeight`, `vUnitWeight`, `vVolume`, `vUnitVolume`. <br> <br>Präzisierung des Elements`vInvoiItemNumber` Nummer der Rechnungsposition im Rahmenelement<br>`docItem` (2.20 Belegposition), vgl. Datentyp`CHAR(POS)`der Domäne`_Pos`. <br> <br>2.3 XML-Deklaration<br>Weiterführende Beschreibung zur Verwendung des XML-Schemas und dessen Version.|
|2.0.0 – 21.11.2008|Initialversion|



                               - 23 

