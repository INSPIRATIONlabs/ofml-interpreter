# Spezifikation OEX OFML Business Data Exchange
## (OFML Part VII)

# **ORDERS**
## Bestellung

# Version 3.1.0

Editoren:
Markus Behrschmidt, Vitra Services GmbH

Thomas Gerth, EasternGraphics GmbH


8.5.2023


Copyright © 2006 - 2023 Industrieverband Büro und Arbeitswelt e. V. (IBA)


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

2.2 Dokumentenartbezogene Spezifikation .................................................................. 6


**3** **Struktur .................................................................................................................. 7**

3.1 Übersicht Dokumentenstruktur ............................................................................... 7

3.2 Rahmenelement `oexDocFrame` - OEX Dokumentenrahmen ................................ 8

3.3 Rahmenelement `oexApplication` - Applikation, die das Dokument erstellt ....... 8

3.4 Rahmenelement `oexFile` - Dokumentenmappe .................................................. 8

3.5 Rahmenelement `oexDocument` - Einzelnes Dokument ........................................ 9

3.6 Rahmenelement `docHeader` - Belegkopf ............................................................. 9

3.7 Rahmenelement `hdrDocNo` - Kopf: Belegnummern ............................................ 10

3.8 Rahmenelement `hdrDateTime` - Kopf: Datums- und Zeitangaben .................... 10

3.9 Rahmenelement `hdrOrgData` - Kopf: Organisationsdaten ................................. 11

3.10 Rahmenelement `hdrAddress` - Kopf: Adressen ................................................. 11

3.11 Rahmenelement `hdrCom` - Kopf: Kommunikation ................................................ 12

3.12 Rahmenelement `hdrContact` - Kopf: Ansprechpartner ..................................... 12

3.13 Rahmenelement `hdrText` - Kopf: Texte ............................................................. 12

3.14 Rahmenelement `hdrReference` - Kopf: Verweise ............................................. 13

3.15 Rahmenelement `hdrPricing` - Kopf: Preiskalkulation ....................................... 13

3.16 Rahmenelement `hdrPayment` - Kopf: Zahlungsbedingungen ............................ 14

3.17 Rahmenelement `docArticleItem` - Belegposition: Artikel ............................... 15

3.18 Rahmenelement `docSetItem` - Belegposition: Set-Artikel ................................. 16

3.19 Rahmenelement `docFolderItem` - Belegposition: Ordner ................................ 17

3.20 Rahmenelement `docTextItem` - Belegposition: Text ........................................ 18


                                  - 1 

3.21 Rahmenelement `itmConfiguration` - Position: Konfigurationsdaten .............. 18

3.22 Rahmenelement `itmConfigText` - Position: Konfigurationstexte ..................... 19

3.23 Rahmenelement `itmDocNo` - Position: Belegnummern ...................................... 19

3.24 Rahmenelement `itmDateTime` - Position: Datums- und Zeitangaben ............... 19

3.25 Rahmenelement `itmOrgData` - Position: Organisationsdaten ........................... 20

3.26 Rahmenelement `itmAddress` - Position: Adressen ........................................... 20

3.27 Rahmenelement `itmCom` - Position: Kommunikation .......................................... 21

3.28 Rahmenelement `itmContact` - Position: Ansprechpartner ................................ 21

3.29 Rahmenelement `itmText` - Position: Texte ........................................................ 21

3.30 Rahmenelement `itmReference` - Position: Verweise ....................................... 22

3.31 Rahmenelement `itmPricing` - Position: Preiskalkulation ................................. 22


**4** **Anhang ................................................................................................................ 24**

4.1 Änderungshistorie ................................................................................................. 24


                                  - 2 

### **1 Einleitung**

Diese Spezifikation definiert alle Elemente, die für die Beschreibung einer Bestellung verwendet werden.

Bestelländerungen wiederum werden über die Dokumentenart ORDCHG übermittelt. Auf eine Bestellung
folgt eine Bestellbestätigung (ORDRSP).
Der Bestellung kann ein Angebot des Lieferanten (QUOTES) vorausgegangen sein.

Angaben zur Bestellung erfolgen immer aus Sicht des Bestellers, nicht aus Sicht des evtl. involvierten EndKunden, in dessen Auftrag die Bestellung erfolgt.

Mitgeltende Spezifikationen (in der jeweils gültigen Version, siehe 2.1):


OEX-GLOBAL – dokumentenartübergreifende Spezifikation

Verwandte Dokumentenarten/Spezifikationen:


Anfrage (OEX-REQOTE), Angebot (OEX-QUOTES), Bestellbestätigung (OEX-ORDRSP), Bestelländerung
(OEX-ORDCHG), Lieferavis (OEX-DESADV) und Rechnung (OEX-INVOIC).


**1.1** **Verwendung dieser Spezifikation**


In dieser Spezifikation werden speziell die Strukturen und Elemente für die Dokumentenart “ORDERS Bestellung“ beschrieben. Globale Strukturen und Elemente, die auch in anderen Dokumentenarten
Verwendung finden, werden im Detail in der übergeordneten Spezifikation “OEX-GLOBAL“ in der
korrespondierenden Version beschrieben. Nur von dort abgeleitete sowie dokumentenartbezogene
Strukturen und Elemente werden hier in dieser Spezifikation beschrieben.


**1.2** **Dateinamenkonventionen**


Als Dateinamenkonvention für die Dokumentenart “ORDERS“ gilt:

```
oex-orders_<sender-id>_jjjjmmtt-hhmmss.xml

```

Grundlage bilden hier also die Dokumentenart sowie Datum und Zeit (24-Stunden-Format) der Erstellung der
Datei. Die Dateierweiterung lautet “ `xml` “.
`<sender-id>` ist der variable Teil des Dateinamens, der vom Sender der Datei vergeben werden muss. Die
max. Länge beträgt 20 Zeichen. Hierbei kann es sich beispielsweise um eine fortlaufende Nummerierung
des Senders handeln, der Kundennummer oder der Lieferantennummer.
Nur Ziffern, Buchstaben und Bindestriche sind hierbei erlaubt.
Mit diesen Angaben können dann auch gerade in einem Fehlerfall Rückschlüsse gezogen werden, um was
für eine Art der Datei es sich handelt, von wem sie kommt und wann sie erstellt wurde.

Beispiele: `oex-orders_VI00025030_20051025-110842.xml`
```
      oex-orders_ABC-9564154_20050809-213306.xml

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
xsi:noNamespaceSchemaLocation="oex-orders_<Major>.<Minor>.<Build>.xsd">

```

Die Einbindung des übergeordneten Schemas ( `oex-global` ) ist bereits im dokumentenartbezogenen
Schema definiert.


Gültige Version des XML Schemas


Für diese Spezifikation gilt das dokumentenartbezogene Schema in der Version 3.1.0
**`oex-orders_3.1.0.xsd`** bzw. bei Änderungen des Schemas ohne Auswirkung auf diese Spezifikation,
das Schema mit der höchsten Build-Nummer.


**1.4** **Prüfmechanismen**


Bei der Verwendung entsprechender XML-Parser, kann zur Prüfung eines OEX-ORDERS-Dokuments (XMLDatei) das jeweils gültige XML Schema (XS) verwendet werden.
Das Schema wird von den entsprechenden Spezifikationen abgeleitet und als Prüftool bezüglich der
Elementstruktur und Datendefinition bereitgestellt. Darüber hinausgehende Prüfungen auf logische Inhalte
und Abhängigkeiten, sowie ein Mapping der Daten unterliegen der jeweils verwendeten Applikation.


**1.5** **Vollständigkeit des Dokumentes**


Das Dokument wird grundsätzlich vollständig übertragen, d.h. auch mit Positionen (oder Daten), die keine
Änderungen gegenüber einem möglicherweise vorausgegangenen Angebot beinhalten ( `aAction = N` ), s.a.
Rahmenelement `oexDocument` .


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
|**`Sch`**|Schlüsselelement|**! **|Element muss vorhanden sein und einen Wert<br>beinhalten, außerdem muss das Element<br>zusammen mit Wert und ggfs. speziell angege-<br>benen Pflichtattributen eindeutig bei Wieder-<br>holungen innerhalb eines Rahmenelements sein.<br>Sind mehrere Elemente so gekennzeichnet,<br>bilden sie zusammen einen eindeutigen Wert.<br>(Wirkung wie bei einem Primärschlüssel)|
|**`Mod`**|Modifikation|**<empty>**|Element ist dokumentartbezogen und/oder<br>verweist auf den angegebenen Typ aus der<br>übergeordneten Spezifikation.|
|**`Mod`**|Modifikation|**D **|Element leitet sich vom angegebenen Typ aus<br>der übergeordneten Spezifikation ab und ist<br>dokumentenartbezogen angepasst<br>(abgeleiteter Typ).|



                                  - 5 

### **2 Definitionen**

**2.1** **Übergeordnete Spezifikation**


Die dokumentenartübergreifenden Spezifikationen sind dem Dokument **OEX-GLOBAL** in der jeweiligen
gültigen Version 3.1.x zu entnehmen, wobei „x“ für die höchste Build-Versionsnummer steht.


**2.2** **Dokumentenartbezogene Spezifikation**


Spezifikation des Dokuments “ORDERS“ – Bestellung

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
`│` `└─── hdrPayment` Kopf: Zahlungsbedingungen
`├─── docArticleItem` Belegposition: Artikel
`│` `├─── itmConfiguration` Position: Konfigurationsdaten
`│` `│` `└─── itmConfigText` Position: Konfigurationstexte
`│` `├─── itmDocNo` Position: Belegnummern
`│` `├─── itmDateTime` Position: Datums- und Zeitangaben
`│` `├─── itmOrgData` Position: Organisationsdaten
`│` `├─── itmAddress` Position: Adressen
`│` `│` `├─── itmCom` Position: Kommunikation
`│` `│` `└─── itmContact` Position: Ansprechpartner
`│` `│` `└─── itmCom` Position: Kommunikation
`│` `├─── itmText` Position: Texte
`│` `├─── itmReference` Position: Verweise
`│` `└─── itmPricing` Position: Preiskalkulation
`├─── docSetItem` Belegposition: Set-Artikel
`│` `├─── itmDocNo` Position: Belegnummern
`│` `├─── itmOrgData` Position: Organisationsdaten
`│` `├─── itmText` Position: Texte
`│` `├─── itmReference` Position: Verweise
`│` `└─── itmPricing` Position: Preiskalkulation
`├─── docFolderItem` Belegposition: Ordner
`│` `├─── itmText` Position: Texte
`│` `└─── itmReference` Position: Verweise
`└─── docTextItem` Belegposition: Text
`└─── itmText` Position: Texte


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
|**`vDocumentType`**|`ORDERS`|`ORDERS`||||Bestellung|
|~~**`oexDocument`**~~|~~**`Document`**~~|**1+**|**X **||**D **|**Einzelnes Dokument**|



                                  - 8 

**3.5** **Rahmenelement** **`oexDocument`** **– Einzelnes Dokument**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`oexDocument`**~~|~~**`Document`**~~|**1+**|**X **|**! **||**Einzelnes Dokument**|
|~~**`oexDocument`**~~|**Attribut**|**Attribut**|||||
|~~**`oexDocument`**~~|`aDocNo`|`aDocNo`|X|!||Laufende Nummer des Dokuments|
|~~**`oexDocument`**~~|`aItemCount`|`aItemCount`|X|||Anzahl Positionen im Dokument|
|~~**`oexDocument`**~~|`aAction`|`aAction`|X|||Aktion|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`docHeader`**|**`Header`**|**1 **|**X **|||**Belegkopf**|
|~~**`docArticleItem`**~~|~~**`Item`**~~|**1+**|**X **||**D **|**Belegposition: Artikel**|
|~~**`docSetItem`**~~|~~**`Item`**~~|*** **|||**D **|**Belegposition: Set-Artikel**|
|~~**`docFolderItem`**~~|~~**`Item`**~~|*** **|||**D **|**Belegposition: Ordner**|
|~~**`docTextItem`**~~|~~**`Item`**~~|*** **|||**D **|**Belegposition: Text**|



Dieses Rahmenelement beinhaltet die Elemente für die Beschreibung einer Bestellung.

Der Bestellung kann ein Angebot des Lieferanten (QUOTES) vorausgegangen sein. Wurde an mindestens
einer Belegposition ( `doc*Item` ) im Vergleich zum Angebot eine Änderung vorgenommen, muss im Attribut
`aAction` der Wert `M` angegeben werden. Entsprechend muss auch bei den geänderten Belegpositionen das
Attribut gesetzt werden. Gab es keine Änderungen bei den Belegpositionen, ist hier und bei allen Belegpositionen für das Attribut `aAction` der Wert `N` anzugeben.
Ging der Bestellung kein Angebot voraus, muss hier und bei allen Belegpositionen für das Attribut `aAction`
der Wert `C` (Anlegen) angegeben werden.


**3.6** **Rahmenelement** **`docHeader`** **– Belegkopf**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`docHeader`**|**`Header`**|**1 **|**X **|||**Belegkopf**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vOrderNumber`**|**`DocNo`**|**1 **|**X **|||**Bestellnummer**<br>_Eindeutige Nummer der Bestellung._|
|**`vPrecedingDocType`**|**`DocNoType`**<br>|**1 **|**4 **||**D **|**Art (der Nummer) des**<br>**Vorgängerbelegs**|
|**`vPrecedingDocType`**|**Wertetabelle**|**Wertetabelle**|**Wertetabelle**|**Wertetabelle**|**Wertetabelle**|**Wertetabelle**|
|**`vPrecedingDocType`**|`QUO`|||||Angebotsnummer|
|**`vPrecedingDocNo`**|**`DocNo`**|**1 **|**4 **|||**Nummer des Vorgängerbelegs**|
|**`vClientNumber`**|**`Value`**|**1 **|**X **|||**Kundennummer**<br>_Nummer, unter der der Lieferant den_<br>_Besteller (Kunde) führt._|
|**`vClientID`**|**`ClientID`**|*** **||||**Kunden-ID**|
|**`vClientClass`**|**`ClientClass`**|*** **||||**Kunden-Klassifizierung**|
|**`vVendorNumber`**|**`Value`**|**1 **|**X **|||**Lieferantennummer**<br>_Nummer, unter der der Besteller_<br>_(Kunde) den Lieferanten führt._|
|**`vSupplierID`**|**`SupplierID`**|*** **||||**Lieferanten-ID**|
|**`vSupplierClass`**|**`SupplierClass`**|*** **||||**Lieferanten-Klassifizierung**|
|**`vDocCurrency`**|**`DocCurrency`**|**1 **|**X **|||**Belegwährung**|
|**`vIncoTerm`**|**`IncoTerm`**|**1 **|**1 **|||**Inco Terms (Lieferbedingung)**<br>_Anders lautende Lieferbedingungen_<br>_können über den Kopftext “Liefer-_<br>_bedingungen“ übergeben werden._|



                                  - 9 

|vIncoTermLocation|IncoTermLoc|1|1|Col5|Col6|Ortsangabe zu Inco Terms|
|---|---|---|---|---|---|---|
|**`vPartialDelivery`**|**`PartDelivery`**|**1 **|**X **|||**Teillieferungen erlaubt?**|
|**`vDocLanguage`**|**`DocLanguage`**|**1 **|**X **|||**Belegsprache**|
|**`vOrderType`**|**`OrderType`**|**1 **||||**Auftragsart**|
|**`vGrossWeight`**|**`GrossWeight`**|**1 **|**2 **|||**Bruttogewicht (gesamt)**|
|**`vNetWeight`**|**`NetWeight`**|**1 **|**2 **|||**Nettogewicht (gesamt)**|
|**`vUnitWeight`**|**`UnitWeight`**|**1 **|**2 **|||**Gewichtseinheit**|
|**`vVolume`**|**`Volume`**|**1 **|**3 **|||**Volumen (gesamt)**|
|**`vUnitVolume`**|**`UnitVolume`**|**1 **|**3 **|||**Volumeneinheit**|
|~~**`hdrDocNo`**~~|~~**`DocNo`**~~|*** **|||**D **|**Kopf: Belegnummern**|
|~~**`hdrDateTime`**~~|~~**`DateTime`**~~|**1+**|**X **|||**Kopf: Datums- u. Zeitangaben**|
|~~**`hdrOrgData`**~~|~~**`OrgData`**~~|*** **||||**Kopf: Organisationsdaten**|
|~~**`hdrAddress`**~~|~~**`Address`**~~|*** **||||**Kopf: Adressen**|
|~~**`hdrText`**~~|~~**`Text`**~~|*** **||||**Kopf: Texte**|
|~~**`hdrReference`**~~|~~**`Reference`**~~|*** **||||**Kopf: Verweise**|
|~~**`hdrPricing`**~~|~~**`Pricing`**~~|*** **|||**D **|**Kopf: Preiskalkulation**|
|~~**`hdrPayment`**~~|~~**`Payment`**~~|**3***||||**Kopf: Zahlungsbedingungen**|


Der Belegkopf enthält alle wichtigen Referenzen des Dokuments.

Erläuterungen zu Pflichtangaben:


**1** Die Ortsangabe zu Inco Terms muss angegeben werden, sobald die Lieferbedingung dies fordert.
**2** Die Gewichtseinheit muss angegeben werden, sobald Bruttogewicht und/oder Nettogewicht
angegeben wird.
**3** Die Volumeneinheit muss angegeben werden, sobald das Volumen angegeben wird.
**4** Wenn die Nummer des Vorgängerbelegs angegeben ist, muss auch die Art der Nummer angegeben
werden.


**3.7** **Rahmenelement** **`hdrDocNo`** **– Kopf: Belegnummern**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrDocNo`**|**`DocNo`**|*** **|||**D **|**Kopf: Belegnummern**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDocNoType`**|**`DocNoType`**|**1 **|**X **|||**Belegnummernart**|
|**`vDocNo`**|**`DocNo`**|**1 **|**X **|||**Belegnummer**|



Dieses Rahmenelement enthält die Belegnummern der Vorgängerbelege in der Abfolge des Geschäftsfalls
und/oder zusätzliche Belege als Referenz für die Bestellung.
Eine Referenzierung auf spezifische Positionen anderer Belege kann bei Bedarf im Rahmenelement
`itmDocNo` vorgenommen werden.

Die Angabe der Bestellnummer selbst sowie der Nummer des unmittelbaren Vorgängerbelegs ist hier nicht
erlaubt, da diese bereits im Rahmenelement `docHeader` angegeben werden (Element `vOrderNumber`
bzw. `vPrecedingDocNo` ).

**3.8** **Rahmenelement** **`hdrDateTime`** **– Kopf: Datums- und Zeitangaben**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrDateTime`**|**`DateTime`**|**1+**|**X **|**! **||**Kopf: Datums- und Zeitangaben**|



**Unterelement** **Typ** **Wdh Pfl Sch Mod Bezeichnung**

                               - 10 

|vDateTimeType|DateTimeType|1|X|!|Col6|Typ Datum/Zeit|
|---|---|---|---|---|---|---|
|**`vTimeZone`**|**`TimeZone`**|**1 **|**X **|||**Zeitzone**|
|**`vDateValue`**|**`Date`**|**1 **|**X **|||**Datumsangabe**|
|**`vTimeValue`**|**`Time`**|**1 **||||**Zeitangabe**|


Dieses Rahmenelement dient zur Übergabe von Datums- und Zeitangaben eines Bestellkopfes.
Als Mindestangabe ist hier das Belegdatum ( `DOC` ) erforderlich.
Des Weiteren können beispielsweise ein Wunschlieferdatum ( `CRD` ) und das Bestelldatum ( `ORD` ) angegeben
werden.


**3.9** **Rahmenelement** **`hdrOrgData`** **– Kopf: Organisationsdaten**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrOrgData`**|**`OrgData`**|*** **||**! **||**Kopf: Organisationsdaten**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vOrgDataType`**|**`OrgDataType`**|**1 **|**X **|**! **||**Arten Organisationsdaten**|
|**`vOrgDataValue`**|**`Value`**|**1 **|**X **|||**Wert Organisationsdaten**|



Verwendungsbeispiel: Kommissionsangabe ( `COM` ) `"Kommission Schmidt"`


**3.10** **Rahmenelement** **`hdrAddress`** **– Kopf: Adressen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrAddress`**|**`Address`**|*** **||**! **||**Kopf: Adressen**|


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
|**`vTaxCodeEU`**|**`Value`**|**1 **||||**Steuernummer EU / U-ID-Nr.**|
|**`vTaxCodeUSA`**|**`Value`**|**1 **||||**Steuernummer USA / Jurisdiction**|
|~~**`hdrCom`**~~|~~**`Com`**~~|*** **||||**Kopf: Kommunikation**|
|~~**`hdrContact`**~~|~~**`Contact`**~~|*** **||||**Kopf: Ansprechpartner**|



                               - 11 

Wird keine Lieferadresse (SH) angegeben, wird die Adresse des Auftraggebers (SO) bzw. die Adresse aus
den Stammdaten des Lieferanten gezogen.
Die Thematik, ob es sich bei einer angegebenen Lieferadresse um eine abweichende Lieferadresse handelt,
die evtl. anders konditioniert ist als die vereinbarte(n) Lieferadresse(n), müssen die Geschäftspartner
definieren. Als Indikatoren können bspw. die Adress-Nummer oder die Verwendung der Organisationsdaten
Typ `TRZ` Transportzone herangezogen werden. Ersteres würde man über die Stammdaten definieren,
letzteres über Transportzonen (z.B. 1 = Zone 1, bedeutet: keine Transportkosten; 2 = Zone 2, bedeutet:
Transportkosten 100,00 und würde sich ebenfalls in der Preiskalkulation `Pricing` widerspiegeln; etc.).
In der Regel sind die Adressen, insbesondere Auftraggeber (SO) und Lieferant (SU), den beiden
Geschäftspartnern bekannt und als Stammdaten hinterlegt und bedürfen nicht unbedingt einer Übertragung,
sie korrespondieren auch zur Kundennummer bzw. Lieferantennummer des Belegkopfes ( `docHeader` ).
Ggf. wird mit einer Adresse aber auch eine für diese Bestellung zuständige Kontaktperson übermittelt.


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



Zur Angabe der Ansprechpartner, die für die Abwicklung des Geschäftsfalls erforderlich sind oder diesem
organisatorisch zugeordnet werden (z.B. ein oder mehrere Vertriebsmitarbeiter bezüglich Provisionen).


**3.13** **Rahmenelement** **`hdrText`** **– Kopf: Texte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrText`**|**`Text`**|*** **||**! **||**Kopf: Texte**|



**Unterelement** **Typ** **Wdh Pfl Sch Mod Bezeichnung**


                               - 12 

|vTextType|TextType|1|X|!|Col6|Textart|
|---|---|---|---|---|---|---|
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
           "http://www.dorfer.de/orderstatus.html?p=1213131"

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



In diesem Rahmenelement wird die Netto-Summe der Positionen einer Bestellung ( `TNET`, ggf. `TNEH` )
angegeben (Pflichtangabe). Sie dient als Kontrollsumme bei der Verarbeitung des Dokuments.
Andere Angaben, wie z.B. Rabatte, sind optional, können aber auch zur Kontrolle bei der Verarbeitung
herangezogen werden.
Die Währung wird, wenn hier nicht anders angegeben, durch die Belegwährung vorgegeben.
Der Konditionsbereich sollte auf den Einkauf beschränkt sein ( `aCondArea="P"` ).

Hinweis: Im Gegensatz zu den Preisangaben auf Positionsebene entfallen in diesem Rahmenelement die
Unterelemente für Preiseinheit und Mengeneinheit, da es sich hier immer um Summenkonditionen handelt.

Beispiel 1 – Angabe des Nettowertes der Bestellung:
Nettowert der Position 1 beträgt € 100,00
Nettowert der Position 2 beträgt € 150,00


                               - 13 

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>

```

Beispiel 2 – Angabe weiterer Konditionen der Bestellung als Summe der Positionen:
Bruttowert der Position 1 beträgt € 125,00
Rabattsatz der Position 1 beträgt 20% als Grundrabatt
Nettowert der Position 1 beträgt € 100,00
Bruttowert der Position 2 beträgt € 200,00
Rabattsatz der Position 2 beträgt 25% als Grundrabatt
Nettowert der Position 2 beträgt € 150,00

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="P"> TGRO </vConditionType>
    <vConditionValue> 325.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="2">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 75.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Grundrabatt </vConditionText>
</hdrPricing>
<hdrPricing aCondNo="3">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</hdrPricing>

```

Die Rabatte werden hier als Gesamtwert absolut aus den Positionen mit gleicher Art des Abschlags
( `aTypeDis` ) angegeben.


**3.16** **Rahmenelement** **`hdrPayment`** **– Kopf: Zahlungsbedingungen**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`hdrPayment`**|**`Payment`**|**3***||**! **||**Kopf: Zahlungsbedingungen**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vPaymentPart`**|**`PaymentPart`**|**1 **|**X **|**! **||**Bestandteil der Zahlungsbedingung**|
|**`vPaymentRate`**|**`PaymentRate`**|**1 **|**X **|||**Skonto-Satz (%)**|
|**`vPaymentDays`**|**`PaymentDays`**|**1 **|**X **|||**Anzahl Tage (Zahlungsziel)**|



                               - 14 

**3.17** **Rahmenelement** **`docArticleItem`** **– Belegposition: Artikel**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`docArticleItem`**~~|~~**`Item`**~~|**1+**|**X **|**! **|**D **|**Belegposition: Artikel**|
|~~**`docArticleItem`**~~|**Attribut**|**Attribut**|||**D **||
|~~**`docArticleItem`**~~|`aItemNo`|`aItemNo`|X|!||Laufende Nummer der Belegposition|
|~~**`docArticleItem`**~~|`aAction`|`aAction`|X||D|Aktion|
|~~**`docArticleItem`**~~|`aUUID`|`aUUID`|X|||Global eindeutiger Identifikator|
|~~**`docArticleItem`**~~|`aIsPseudo`|`aIsPseudo`||||Repräsentiert die Position einen<br>Pseudo-Artikel?|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vPrecDocItemNo`**|**`DocItemNo`**|**1 **||||**Nummer der Position im**<br>**Vorgängerbeleg**<br>_Der Vorgängerbeleg wird im Belegkopf_<br>_angegeben._|
|**`vOrderItemNumber`**|**`DocItemNo`**|**1 **|**X **|||**Nummer der Bestellposition**<br>_Eindeutige Positionsnummer innerhalb_<br>_der Bestellung._|
|**`vOrderTopLevelNo`**|**`DocItemNo`**|**1 **||||**Nummer der übergeordneten**<br>**Bestellposition**|
|**`vOrderComposNo`**|**`DocItemNo`**|**1 **|**3 **|||**Nummer der Bestellposition des**<br>**kompositen Artikels**<br>_Dieser Verweis legt fest, dass der_<br>_Artikel automatisch durch den_<br>_kompositen Artikel erzeugt worden ist._|
|**`vOrderSubArtId`**|**`CompSubArtId`**|**1 **|**3 **|||**Identifikation des Unterartikels**<br>_Die ID wird durch den übergeordneten_<br>_kompositen Artikel vergeben._|
|**`vOrderAddStateCd`**|**`AddStateCode`**|**1 **||||**Zusätzliche Zustandsinformationen**|
|**`vOrderAddStateCd2`**|**`AddStateCode`**|**1 **||||**Zusätzliche Zustandsinformationen**<br>**für gekapselte OFML-Instanz**|
|**`vClientArticleNo`**|**`ClientArtNo`**|**1 **||||**Kundenartikelnummer**|
|**`vVendorArticleNo`**|**`VendorArtNo`**|**1 **|**X **|||**Lieferantenartikelnummer**|
|**`vVendorID`**|**`VendorID`**|**1 **|**X **|||**Lieferantenkennung**|
|**`vVendorSeries`**|**`VendorSeries`**|**1 **|**X **|||**Lieferantenserie**|
|**`vCatalogId`**|**`CatalogId`**|**1 **||||**Katalog-ID**|
|**`vArticleEAN`**|**`EAN_Article`**|**1 **||||**EAN des Artikels**|
|**`vOrderQuantity`**|**`Quantity`**|**1 **|**X **|||**Bestellmenge**|
|**`vOrderUnit`**|**`QuantUnit`**|**1 **|**X **|||**Bestellmengeneinheit**|
|**`vGrossWeight`**|**`GrossWeight`**|**1 **|**1 **|||**Bruttogewicht (gesamt)**|
|**`vNetWeight`**|**`NetWeight`**|**1 **|**1 **|||**Nettogewicht (gesamt)**|
|**`vUnitWeight`**|**`UnitWeight`**|**1 **|**1 **|||**Gewichtseinheit**|
|**`vVolume`**|**`Volume`**|**1 **|**2 **|||**Volumen (gesamt)**|
|**`vUnitVolume`**|**`UnitVolume`**|**1 **|**2 **|||**Volumeneinheit**|
|**`vClassification`**|**`Classification`**|<br>*** **||||**Klasse/Kategorie der Bestellposition**|
|~~**`itmConfiguration`**~~|~~**`Config`**~~|*** **||||**Position: Konfigurationsdaten**|
|~~**`itmDocNo`**~~|~~**`DocNo`**~~|*** **||||**Position: Belegnummern**|
|~~**`itmDateTime`**~~|~~**`DateTime`**~~|*** **||||**Position: Datums- u. Zeitangaben**|
|~~**`itmOrgData`**~~|~~**`OrgData`**~~|*** **||||**Position: Organisationsdaten**|
|~~**`itmAddress`**~~|~~**`Address`**~~|*** **||||**Position: Adressen**|
|~~**`itmText`**~~|~~**`Text`**~~|**1+**|**X **|||**Position: Texte**|
|~~**`itmReference`**~~|~~**`Reference`**~~|*** **||||**Position: Verweise**|
|~~**`itmPricing`**~~|~~**`Pricing`**~~|*** **||||**Position: Preiskalkulation**|



                               - 15 

Basisdaten der Bestellposition.

Ging der Bestellung kein Angebot voraus (siehe Belegkopf), muss für das Attribut `aAction` der Wert `C`
(Anlegen) angegeben werden. (Dies gilt auch für alle Unterelemente mit diesem Attribut.)
Ging der Bestellung ein Angebot voraus und wurde an mindestens einem Unterelement im Vergleich zum
Angebot eine Änderung vorgenommen, muss im Attribut `aAction` der Wert `M` angegeben werden. Bei den
geänderten Unterelementen muss das Attribut entsprechend gesetzt werden. Gab es keine Änderungen,
kann das Attribut `aAction` entfallen oder es ist der Wert `N` anzugeben.

Durch das Zusammenspiel von Positionsnummer ( `vOrderItemNumber` ) mit der Nummer der übergeordneten Bestellposition ( `vOrderTopLevelNo` ) lässt sich eine Hierarchiestruktur (z.B. mit Unterartikeln) abbilden.
Durch Verweis auf eine übergeordnete Position vom Typ `docFolderItem` lassen sich auch Ordnerstrukturen abbilden.
Spezielle Positionsnummernangaben wie bspw. “100.A.10-1“ können über die Organisationsdaten Typ `POS`
übermittelt werden, in wie weit eine andere Applikation diese verarbeiten, zurückliefern kann oder gar für
sich selbst verwendet, bleibt jedoch offen.

Die zusätzlichen Zustandsinformationen für die gekapselte OFML-Instanz ( `vOrderAddStateCd2` ) werden
benötigt, wenn die Instanz, die den Artikel repräsentiert, durch eine Metatyp-Instanz gekapselt ist und sich
mit dieser eine Position teilt. (In `vOrderAddStateCd` ist dann der Code für die Metatyp-Instanz
anzugeben.)

Erläuterungen zu Pflichtangaben:


**1** Die **Gewichtseinheit** muss angegeben werden, sobald **Bruttogewicht** und/oder **Nettogewicht**
angegeben wird.
**2** Die **Volumeneinheit** muss angegeben werden, sobald das **Volumen** angegeben wird.
**3** **Identifikation des Unterartikels** kann nur angegeben werden, wenn auch die **Nummer der**
**Bestellposition des kompositen Artikels** angegeben wurde.

Bei einem Standardartikel wird wenigstens der Kurztext übermittelt, auf den Langtext kann in diesem Fall
verzichtet werden. Anders verhält es sich bei modifizierten Artikeln und Kundenartikeln (vgl. globalen OEXWerttyp `VendorArtNo`  `aStatus` ).


**3.18** **Rahmenelement** **`docSetItem`** **– Belegposition: Set-Artikel**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`docSetItem`**~~|~~**`Item`**~~|*** **||**! **|**D **|**Belegposition: Set-Artikel**|
|~~**`docSetItem`**~~|**Attribut**|**Attribut**|||**D **||
|~~**`docSetItem`**~~|`aItemNo`|`aItemNo`|X|!||Laufende Nummer der Belegposition|
|~~**`docSetItem`**~~|`aAction`|`aAction`|X||D|Aktion|
|~~**`docSetItem`**~~|`aUUID`|`aUUID`|X|||Global eindeutiger Identifikator|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vPrecDocItemNo`**|**`DocItemNo`**|**1 **||||**Nummer der Position im**<br>**Vorgängerbeleg**<br>_Der Vorgängerbeleg wird im Belegkopf_<br>_angegeben._|
|**`vOrderItemNumber`**|**`DocItemNo`**|**1 **|**X **|||**Nummer der Bestellposition**<br>_Eindeutige Positionsnummer innerhalb_<br>_der Bestellung._|
|**`vOrderTopLevelNo`**|**`DocItemNo`**|**1 **||||**Nummer der übergeordneten**<br>**Bestellposition**|
|**`vClientArticleNo`**|**`ClientArtNo`**|**1 **||||**Kundenartikelnummer**|
|**`vVendorArticleNo`**|**`VendorArtNo`**|**1 **|**1 **|||**Lieferantenartikelnummer**|



                               - 16 

|vVendorID|VendorID|1|Col4|Col5|Col6|Lieferantenkennung|
|---|---|---|---|---|---|---|
|**`vVendorSeries`**|**`VendorSeries`**|**1 **||||**Lieferantenserie**|
|**`vOrderQuantity`**|**`Quantity`**|**1 **|**X **|||**Bestellmenge**|
|**`vOrderUnit`**|**`QuantUnit`**|**1 **|**X **|||**Bestellmengeneinheit**|
|~~**`itmDocNo`**~~|~~**`DocNo`**~~|*** **||||**Position: Belegnummern**|
|~~**`itmOrgData`**~~|~~**`OrgData`**~~|*** **||||**Position: Organisationsdaten**|
|~~**`itmText`**~~|~~**`Text`**~~|*** **|**1 **|||**Position: Texte**|
|~~**`itmReference`**~~|~~**`Reference`**~~|*** **||||**Position: Verweise**|
|~~**`itmPricing`**~~|~~**`Pricing`**~~|*** **||||**Position: Preiskalkulation**|


Ein Set-Artikel fasst mehrere Artikel (Unterpositionen) zu einer Position zusammen.
Der Preis eines Set-Artikels wird automatisch entsprechend der in ihm enthaltenen Artikel (incl. Mengen und
Rabatten) und der Menge des Set-Artikels errechnet. Enthält die Set-Position `itmPricing` Unterelemente,
so haben diese rein informativen Charakter, d.h., die dort angegebenen Preise fließen nicht in die Preiskalkulation auf Kopfebene (Beleg) ein.

Zur Verwendung des Attributs `aAction` und der Positionsnummern siehe Rahmenelement
`docArticleItem` .

Erläuterungen zu Pflichtangaben:


**1** Wenn keine **Lieferantenartikelnummer** angegeben ist, muss der **Artikelkurztext** angegeben werden
(Unterelement **`itmText`** ).


**3.19** **Rahmenelement** **`docFolderItem`** **– Belegposition: Ordner**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`docFolderItem`**~~|~~**`Item`**~~|*** **||**! **|**D **|**Belegposition: Ordner**|
|~~**`docFolderItem`**~~|**Attribut**|**Attribut**|||**D **||
|~~**`docFolderItem`**~~|`aItemNo`|`aItemNo`|X|!||Laufende Nummer der Belegposition|
|~~**`docFolderItem`**~~|`aAction`|`aAction`|X||D|Aktion|
|~~**`docFolderItem`**~~|`aUUID`|`aUUID`|X|||Global eindeutiger Identifikator|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vPrecDocItemNo`**|**`DocItemNo`**|**1 **||||**Nummer der Position im**<br>**Vorgängerbeleg**<br>_Der Vorgängerbeleg wird im Belegkopf_<br>_angegeben._|
|**`vOrderItemNumber`**|**`DocItemNo`**|**1 **|**X **|||**Nummer der Bestellposition**<br>_Eindeutige Positionsnummer innerhalb_<br>_der Bestellung._|
|**`vOrderTopLevelNo`**|**`DocItemNo`**|**1 **||||**Nummer der übergeordneten**<br>**Bestellposition**|
|**`vFolderName`**|**`Value`**|**1 **|** X**|||**Bezeichnung des Ordners**<br>_(in Belegsprache)_|
|**`vFolderIsLOC`**|**`FolderIsLOC`**|**1 **||||**Ist die Bezeichnung des Ordners**<br>**eine Ortsangabe?**|
|~~**`itmText`**~~|~~**`Text`**~~|*** **||||**Position: Texte**|
|~~**`itmReference`**~~|~~**`Reference`**~~|*** **||||**Position: Verweise**|



Zur Verwendung des Attributs `aAction` und der Positionsnummern siehe Rahmenelement
`docArticleItem` .


                               - 17 

Wenn die Applikation, die das Dokument erstellt, es dem Anwender ermöglicht, die Ordnerbezeichnung
( `vFolderName` ) explizit als Ortsangabe (Raumtext) zu kennzeichnen, sollte dies entsprechend im Element
`vFolderIsLOC` übermittelt werden. Im Fall von `Y` (ja) sollte die Ordnerbezeichnung dann auch in den
Unterelementen vom Typ `docArticleItem` und `docSetItem` in einem `itmOrgData` Element mit
OrgDataType `LOC` übertragen werden. (Das kann die Verarbeitung in empfangenden Systemen erleichtern,
die Raumtexte für Artikel-Positionen verwalten.)


**3.20** **Rahmenelement** **`docTextItem`** **– Belegposition: Text**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|~~**`docTextItem`**~~|~~**`Item`**~~|*** **||**! **|**D **|**Belegposition: Text**|
|~~**`docTextItem`**~~|**Attribut**|**Attribut**|||**D **||
|~~**`docTextItem`**~~|`aItemNo`|`aItemNo`|X|!||Laufende Nummer der Belegposition|
|~~**`docTextItem`**~~|`aAction`|`aAction`|X||D|Aktion|
|~~**`docTextItem`**~~|`aUUID`|`aUUID`|X|||Global eindeutiger Identifikator|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vPrecDocItemNo`**|**`DocItemNo`**|**1 **||||**Nummer der Position im**<br>**Vorgängerbeleg**<br>_Der Vorgängerbeleg wird im Belegkopf_<br>_angegeben._|
|**`vOrderItemNumber`**|**`DocItemNo`**|**1 **|**X **|||**Nummer der Bestellposition**<br>_Eindeutige Positionsnummer innerhalb_<br>_der Bestellung._|
|**`vOrderTopLevelNo`**|**`DocItemNo`**|**1 **||||**Nummer der übergeordneten**<br>**Bestellposition**|
|**`vItemName`**|**`Value`**|**1 **|** X**|||**Bezeichnung**_(in Belegsprache)_|
|~~**`itmText`**~~|~~**`Text`**~~|*** **||||**Position: Texte**|



Zur Verwendung des Attributs `aAction` und der Positionsnummern siehe Rahmenelement
`docArticleItem` .


**3.21** **Rahmenelement** **`itmConfiguration`** **– Position: Konfigurationsdaten**

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
|~~**`itmConfigText`**~~|~~**`ConfigText`**~~|*** **||||**Position: Konfigurationstexte**|



                               - 18 

**3.22** **Rahmenelement** **`itmConfigText`** **– Position: Konfigurationstexte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmConfigText`**|**`ConfigText`**|*** **||||**Position: Konfigurationstexte**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|||**Textsprache**|
|**`vOptionText`**|**`OptionText`**|**1 **|**X **|||**Merkmalstext**|
|**`vValueText`**|**`ValueText`**|*** **||||**Merkmalswertetext**<br>Hier fällt der Text unter Umständen<br>weg, wenn es sich um einen frei<br>bewertbaren Merkmalswert handelt**. **|



Hinweis: Auf Übermittlung der Texte (Merkmal und Wert) kann verzichtet werden, wenn es sich um einen
Originalartikel des Lieferanten handelt, da diese inhaltlich nicht verändert werden können. Abweichende
Artikelbeschreibungen fließen in den modifizierten Artikeltext (siehe `vTextType = ARTM` ) ein, dabei muss
der Artikel dann als “Modifizierter Artikel“ markiert werden (siehe `vVendorArticleNo`  `aStatus = M` ).


**3.23** **Rahmenelement** **`itmDocNo`** **– Position: Belegnummern**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmDocNo`**|**`DocNo`**|*** **||||**Position: Belegnummern**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vDocNoType`**|**`DocNoType`**|**1 **|**X **|||**Belegnummernart**|
|**`vDocNo`**|**`DocNo`**|**1 **|**X **|||**Belegnummer**|
|**`vDocLine`**|**`DocItemNo`**|**1 **||||**Nummer der Belegposition**|



Dieses Rahmenelement enthält die Positionsnummern der Vorgängerbelege in der Abfolge des Geschäftsfalls und/oder zusätzliche Belege als Referenz für die Bestellung. Die Angabe der Positionsnummer ist
erforderlich, solange es sich nicht um einen Beleg ohne Positionsangaben handelt.


**3.24** **Rahmenelement** **`itmDateTime`** **– Position: Datums- und Zeitangaben**

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


                               - 19 

**3.25** **Rahmenelement** **`itmOrgData`** **– Position: Organisationsdaten**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmOrgData`**|**`OrgData`**|*** **||**! **||**Position: Organisationsdaten**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vOrgDataType`**|**`OrgDataType`**|**1 **|**X **|**! **||**Art der Organisationsdaten**|
|**`vOrgDataValue`**|**`Value`**|**1 **|**X **|||**Wert Organisationsdaten**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrOrgData` abweicht oder zusätzliche positionsrelevante Informationen enthält.


**3.26** **Rahmenelement** **`itmAddress`** **– Position: Adressen**

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
|**`vTaxCodeEU`**|**`Value`**|**1 **||||**Steuernummer EU / U-ID-Nr.**|
|**`vTaxCodeUSA`**|**`Value`**|**1 **||||**Steuernummer USA / Jurisdiction**|
|~~**`itmCom`**~~|~~**`Com`**~~|*** **||||**Position: Kommunikation**|
|~~**`itmContact`**~~|~~**`Contact`**~~|*** **||||**Position: Ansprechpartner**|



Dieses Rahmenelement findet nur Verwendung, wenn es von den Daten des übergeordneten
Kopfrahmenelements `hdrAddress` abweicht oder zusätzliche positionsrelevante Informationen enthält.


                               - 20 

**3.27** **Rahmenelement** **`itmCom`** **– Position: Kommunikation**

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


**3.28** **Rahmenelement** **`itmContact`** **– Position: Ansprechpartner**

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


**3.29** **Rahmenelement** **`itmText`** **– Position: Texte**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmText`**|**`Text`**|*** **||**! **||**Position: Texte**|


|Unterelement|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`vTextType`**|**`TextType`**|**1 **|**X **|**! **||**Textart**|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|**! **||**Textsprache**|
|**`vTextContent`**|**`TextContent`**|**1+**|**X **|||**Textinhalt**|



                               - 21 

**3.30** **Rahmenelement** **`itmReference`** **– Position: Verweise**

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


**3.31** **Rahmenelement** **`itmPricing`** **– Position: Preiskalkulation**

|Element|Typ|Wdh|Pfl|Sch|Mod|Bezeichnung|
|---|---|---|---|---|---|---|
|**`itmPricing`**|**`Pricing`**|*** **||||**Position: Preiskalkulation**|


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
des Dokuments.
Andere Angaben, wie z.B. Rabatte, sind optional, können aber auch zur Kontrolle bei der Verarbeitung
herangezogen werden.
Die Währung wird, wenn hier nicht anders angegeben, durch die Belegwährung vorgegeben.
Die Mengeneinheit wird, wenn hier nicht anders angegeben, durch die Bestellmengeneinheit ( `vOrderUnit` )
vorgegeben.
Der Konditionsbereich sollte auf den Einkauf beschränkt sein ( `aCondArea="P"` ).

Beispiel 1 – Angabe des Nettowertes der Position:
Nettoeinzelpreis der Position beträgt € 50,00
Bestellmenge = 2
Bestellmengeneinheit = C62

```
<itmPricing aCondNo="1">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 100.00 </vConditionValue>  ! TNET = Bestellmenge x Nettoeinzelpreis
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

                               - 22 

Beispiel 2 – Angabe aller Konditionen einer Position:
Bruttoeinzelpreis der Position beträgt € 50,00 mit Steuerkennzeichen 1, 19%
Rabatt (als Grundrabatt) von 20% auf den Bruttoeinzelpreis
Rabatt (als Ausstellungsrabatt) von 5% auf den bereits rabattierten Preis
Bestellmenge = 2
Bestellmengeneinheit = C62
19% MwSt

```
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
    <vCondCurrency> EUR </vCondCurrency>
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
# Hier kommt nun die Bestellmenge von 2 Stück zum Tragen: TNET = SNET x 2
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
    <vConditionValue> 14.44 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="8">
    <vConditionType aCondArea="P"> TOTL </vConditionType>
    <vConditionValue> 90.44 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

                               - 23 

### **4  Anhang**

**4.1** **Änderungshistorie**

|Version|Änderungen|
|---|---|
|3.1.0 – 8.5.2023| <br>Kleinere Umstellungen und Erweiterungen in der Einleitung´<br> <br>Präzisierungen in den Rahmenelementen`hdrPricing` (Kopf: Preiskalkulation) und`itmPricing` <br>(Position: Preiskalkulation)<br> <br>Neues Attribut`aIsPseudo` im Rahmenelement`docArticleItem` <br> <br>Neues Unterelement`itmOrgData` im Rahmenelement`docSetItem` <br> <br>Neues Element`vFolderIsLOC` im Rahmenelement`docFolderItem` plus Empfehlung, Raumtexte in<br>Unterelementen in einem`itmOrgData` Element mit OrgDataType`LOC` zu übermitteln.|
|3.0.0 – 30.11.2017| <br>Globale Änderungen laut Spezifikation GLOBAL 3.0.0<br> <br>Umstrukturierung der Spezifikation<br> <br>Rahmenelement`oexDocument` (Einzelnes Dokument): Attribut`aAction` ist nun Pflicht und<br>Präzisierung zur Verwendung des Attributs<br> <br>Rahmenelement`docHeader`: Element`vOrderNumber` hat nun den Typ`DocNo`. Neue Elemente<br>`vPrecedingDocType` und`vPrecedingDocNo` zur (optionalen) Angabe des (unmittelbaren)<br>Vorgängerbelegs (Angebot).<br> <br>Rahmenelement`hdrDocNo`: Die Nummer des unmittelbaren Vorgängerbelegs darf hier nicht mehr<br>angegeben werden.<br> <br>Rahmenelement`docItem` umbenannt in`docArticleItem`. <br> <br>Rahmenelement`docArticleItem`: neues (optionales) Element`vPrecDocItemNo` für die Nummer<br>der Position im Vorgängerbeleg.<br> <br>Rahmenelement`docArticleItem`: neues (optionales) Element`vOrderAddStateCd2` für<br>zusätzliche Zustandsinformationen für eine evtl. gekapselte OFML-Instanz.<br> <br>Rahmenelement`docArticleItem`: Element`vClientArticleNo` hat nun den Typ`ClientArtNo` <br>(war`Value`).<br> <br>Neue Rahmenelemente`docFolderItem` (Belegposition: Ordner),`docTextItem` (Belegposition:<br>Text) und`docSetItem` (Belegposition: Set-Artikel).|
|2.3.0 – 1.7.2015| <br>Globale Änderungen laut Spezifikation GLOBAL 2.3.0<br> <br>Erweiterung: Rahmenelement`docItem` (Belegposition) ergänzt um optionales Element für Angabe<br>einer Klasse oder Kategorie:`vClassification`|
|2.2.0 – 11.10.2013| <br>Globale Änderungen laut Spezifikation GLOBAL 2.2.0<br> <br>Erweiterung: Rahmenelement`docHeader` (Belegkopf) ergänzt um optionale Elemente für Kunden-ID,<br>Kunden-Klassifizierung, Lieferanten-ID und Lieferanten-Klassifizierung:`vClientID`, `vClientClass`, <br>`vSupplierID`und`vSupplierClass`. <br>Optionalen Elemente für ILN Kunde und ILN Lieferant ersetzt durch Kunden-ID und Lieferanten-ID.<br> <br>Erweiterung: Rahmenelemente`hdrAddress` (Kopf: Adressen) und`itmAddress` (Position: Adressen)<br>ergänzt um optionale Elemente für Straße 2 und Ortsteil:`vStreet2`und`vDistrict`. <br>Optionales Element für ILN Adresse:`vAddressILN` ersetzt durch neues optionales Element für<br>Adress-ID:`vAddressID`. <br> <br>Erweiterung: Rahmenelement`docItem` (Belegposition) ergänzt um optionales Element für die<br>Katalog-ID:`vCatalogId`, Identifikation des Unterartikels:`vOrderSubArtId` und Zusätzliche<br>Zustandsinformationen:`vOrderAddStateCd`.|
|2.1.0 – 06.11.2009| <br>Globale Änderungen laut Spezifikation GLOBAL 2.1.0<br> <br>Erweiterung: Rahmenelemente`docHeader` (Belegkopf) und`docItem` (Belegposition) ergänzt um<br>optionale Elemente für Volumen und Gewichte sowie deren Einheiten:`vGrossWeight`, `vNetWeight`, <br>`vUnitWeight`, `vVolume`, `vUnitVolume`. <br> <br>Präzisierung des Elements`vOrderItemNumber` Nummer der Bestellposition im Rahmenelement<br>`docItem` (Belegposition ), vgl. Datentyp`CHAR(POS)`der Domäne`_Pos`. <br> <br>Weiterführende Beschreibung zur Verwendung des XML-Schemas und dessen Version.|



                               - 24 

|Version|Änderungen|
|---|---|
|2.0.0 – 21.11.2008| <br>Globale Änderungen laut Spezifikation GLOBAL 2.0.0<br> <br>Neu: Unterelement`vOrderType` Auftragsart im Rahmenelement`docHeader (`Belegkopf)<br> <br>Neu: Unterelement`vOrderComposNo` Nummer der Bestellposition des kompositen Artikels im<br>Rahmenelement`docItem` (Belegposition)<br> <br>Neu: Rahmenelement`hdrDocNo` (Kopf: Belegnummern) und Rahmenelement`itmDocNo` (Position:<br>Belegnummern) zur Referenzierung auf verbundene Dokumente eines Geschäftsfalls.<br> <br>Erweiterung: Element`vConditionType` Konditionsart bezüglich Steuern im Rahmenelement<br>`hdrPricing` (Kopf: Preiskalkulation) und Rahmenelement`itmPricing` (Position: Preiskalkulation).<br>Neue Konditionsarten`SUBI` und`SUBH` für Rabattzwischensummen (siehe GLOBAL 2.0.0).<br> <br>Neu: Element`vConditionText` Konditionsbezeichnung im Rahmenelement`hdrPricing` (Kopf:<br>Preiskalkulation) und Rahmenelement`itmPricing` (Position: Preiskalkulation)<br> <br>Erweiterung: Präzisierung der Legende bezüglich Wiederholbarkeit, Schlüsselelemente und<br>Pflichtelemente mit Auswirkung auf die Rahmenelemente und deren Unterelemente.<br> <br>Erweiterung: Domäne`_TextLine` (Textzeile) bezüglich der Steuerung der Ausgabe der Textzeile über<br>das Attribut`aLineFormat` (siehe GLOBAL 2.0.0). Betrifft das Element`vTextContent` (Textinhalt) im<br>Rahmenelement`hdrText` (Kopf: Texte) und Rahmenelement`itmText` (Position: Texte), sowie<br>`vValueText` (Merkmalswertetext) im Rahmenelement`itmConfigText` (Position:<br>Konfigurationstexte)<br> <br>Erweiterung: Domäne`_Comtype` (Kommunikationsart) zur Unterscheidung gleicher<br>Kommunikationsarten in unterschiedlichen Anwendungsbereichen über das Attribut`aScopeInfo` <br>(siehe GLOBAL 2.0.0). Betrifft das Element`vComType` (Art der Kommunikation) in den<br>Rahmenelementen`hdrCom` (Kopf: Kommunikation) und`itmCom` (Position: Kommunikation)|
|1.1.1 – 24.04.2007| <br>Globale Änderungen laut Spezifikation GLOBAL 1.1.1<br> <br>Diverse Konkretisierungen|
|1.1.0 – 18.09.2006| <br>Zahlungsbedingungen hinzugefügt: Rahmenelement`hdrPayment` <br> <br>Konfigurationstexte hinzugefügt: Rahmenelement`itmConfigText` (Bestandteil von<br>`itmConfiguration`).<br> <br>Preiskalkulation geändert: Rahmenelemente`hdrPricing` und`itmPricing` <br> <br>Texte geändert: Rahmenelemente`hdrText` und`itmText` <br> <br>Datums- und Zeitangaben geändert: Rahmenelemente`hdrDateTime` und`itmDateTime`, <br>Zeitzone hinzugefügt<br> <br>Artikelstatus:`aStatus` ersetzt`aSpecial` der Lieferantenartikelnummer`vVendorArticleNo` <br> Details zu den betroffenen Typen und Domänen siehe übergeordnete Spezifikation GLOBAL 1.1.0|
|1.0.1 – 01.06.2006| <br>Globale Änderungen laut Spezifikation GLOBAL 1.0.3<br> <br>Mitgeltendes Dokument/Globale Spezifikation:`oex-global_1.0.3.pdf` <br> <br>Korrespondierende XML-Beispieldatei:`oex-orders-sample_1.0.1.xml` <br> <br>Korrespondierendes XML-Schema:`oex-orders_1.0.1.xsd`|
|1.0 – 18.01.2006|Initialversion|



                             - 25 

