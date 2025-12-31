# Spezifikation OCD OFML Commercial Data [*] (OFML Part IV) Version 4.3

Status: Release


Thomas Gerth, EasternGraphics GmbH (Editor)


2020-06-25


- Copyright © 2003–2020 Industrieverband B¨uro und Arbeitswelt e. V. (IBA)


## **Inhaltsverzeichnis**

**1** **Einleitung** **3**


**2** **Die Tabellen** **5**


2.1 ¨Ubersicht . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5


2.2 Die Artikeltabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 6


2.3 Die Artikel–Identifikationstabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 7


2.4 Die Klassifikationstabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 8


2.5 Die Packaging–Tabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 9


2.6 Die Tabelle kompositer Artikel . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 11


2.7 Die St¨ucklistentabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 12


2.8 Die Merkmalsklassentabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13


2.9 Die Merkmalstabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 14


2.10 Die Merkmal–Identifikationstabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 18


2.11 Merkmalsgruppen . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 19


2.12 Die Artikelstammtabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 20


2.13 Die Merkmalswerttabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 20


2.14 Die Merkmalswert–Identifikationstabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . 22


2.15 Die Beziehungsobjekt–Tabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 23


2.16 Die Beziehungswissen–Tabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 25


2.17 Die Preistabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 26


2.18 Die Rundungsvorschrift–Tabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 27


2.19 Die Serientabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 28


2.20 Die Beschreibungstabellen . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 29


2.21 Wertkombinationstabellen . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 30


2.22 Die Identifikationstabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 31


2.23 Die Versionsinformationstabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 32


2.24 Die Nummernschema–Tabelle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 33


2.25 Besteuerungsschemata . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 35


**3** **Die Preisermittlung** **37**


3.1 ¨Uberblick . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 37


3.2 Relevante Tabelleneintr¨age . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 37


3.3 G¨ultige Tabelleneintr¨age . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 38


3.4 Preisfaktoren . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 39


**4** **Die Endartikelnummererzeugung** **40**


4.1 Die vordefinierten Schemata . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 40


4.2 Nutzerdefinierte Schemata . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 41


4.3 Mehrwertige Merkmale . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 41


1


**5** **Merkmalstext-Steuerung** **43**


**6** **Die Ermittlung von Verpackungsdaten** **45**


**A Sprachdefinition** `OCD_1` **46**


**B Sprachdefinition** `OCD_2` **48**


B.1 Constraints . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 48


B.2 Tabellenaufruf . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 51


B.2.1 Tabellenaufruf in Vorbedingungen . . . . . . . . . . . . . . . . . . . . . . . . . . . 51


B.2.2 Tabellenaufruf in Aktionen und Reaktionen . . . . . . . . . . . . . . . . . . . . . . 51


B.2.3 Tabellenaufruf in Constraints . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 52


**C Sprachdefinition** `OCD_3` **53**


C.1 mehrwertige Merkmale . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 53


C.2 mehrstufige Konfiguration . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 53


**D Sprachdefinition** `OCD_4` **54**


**E** **Sprachdefinition** `SAP_LOVC` **58**


**F** **Arithmetische Funktionen in Beziehungswissen** **59**


**G Reservierte Schl¨usselw¨orter** **60**


**H Steuerarten und Steuerkategorien** **62**


**I** **Begriffe** **64**


**J** **¨Anderungshistorie** **65**


J.1 OCD 4.3 vs. OCD 4.2 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 65


2


## **1 Einleitung**

OCD dient allgemein zur Anlage von Produktdaten, die in Gesch¨aftsprozessen des M¨obelhandels ben¨otigt
und ausgetauscht werden. Mit OCD sollen prim¨ar folgende Aufgaben abgedeckt und abgewickelt werden
k¨onnen:


 Konfiguration komplexer Artikel


 Preisermittlung


 Erstellung von Angebots- und Bestellformularen


OCD ist _kein_ Format zur Anlage von Katalogdaten. Diese m¨ussen anderweitig bereitgestellt werden. Die
Verkn¨upfung zwischen Katalog- und Produktdaten erfolgt durch das jeweilige Softwaresystem anhand
der Artikelnummern.


Das Datenmodell f¨ur OCD baut auf dem grundlegenden OFML–Produktdatenmodell auf (siehe Anhang
A des OFML–Standards, Version 2.0.2).


Als physisches Austauschformat zwischen OFML–konformen Applikationen werden CSV–Tabellen (comma separated values) verwendet. Hierzu gelten folgende Bestimmungen:


 Jede der unten beschriebenen Tabellen ist in genau einer Datei enthalten. Der Dateiname wird
durch den Pr¨afix ” `ocd_` “, den spezifizierten Tabellennamen und den Suffix ” `.csv` “ gebildet, wobei
der Tabellenname komplett klein geschrieben wird.


 Jede Zeile der Datei (abgeschlossen durch ein Zeichen f¨ur den Zeilenwechsel ’ `\n` ’) repr¨asentiert einen
Datensatz. Leerzeilen (solche aus Null oder mehr Leerzeichen oder Tabulator) werden ignoriert. Als
Zeichensatz wird `ISO-8859-1` (Latin-1) verwendet.


 Die Felder eines Datensatzes werden durch Semikolon voneinander getrennt.


 Zeilen, die mit einem Doppelkreuz (’ `#` ’) beginnen, werden als Kommentar interpretiert und von der
weiteren Bearbeitung ausgeschlossen.


Bei den folgenden Tabellenbeschreibungen wird ein Feld eines Datensatzes durch folgende Attribute
spezifiziert:


 Nummer


 Name


 Kennzeichen, ob das Feld zum Prim¨arschl¨ussel der Tabelle geh¨ort


 Datentyp (s.u.)


 maximale L¨ange des Feldes (Anzahl der Zeichen) [1]


 Kennzeichen, ob das Feld unbedingt gef¨ullt sein muss (Pflichtfeld)


1Bei CSV–Datens¨atzen bestehen prinzipiell zwar keine Beschr¨ankungen der einzelnen Feldl¨angen, bei bestimmten Feldern
des Datentyps `Char` werden hier jedoch sich aus dem Verwendungszweck ergebende maximal m¨ogliche bzw. sinnvolle L¨angen
angegeben. Dar¨uber hinaus sind bei der Datenanlage ggf. weitergehende Beschr¨ankungen zu beachten, die durch das im
Datenanlageprozeß verwendete Programm auferlegt werden.


3


Folgende **Datentypen** sind definiert:


**Char** Zeichenkette


Es gelten folgende lexikalischen und syntaktischen Bestimmungen:


1. Es sind alle druckbaren Zeichen bis auf das Feldtrennzeichen (Semikolon) erlaubt.

2. Soll ein Semikolon in der Zeichenkette enthalten sein, muss das ganze Feld in Anf¨uhrungszeichen (’ `"` ’) eingeschlossen werden (denen kein singul¨ares Anf¨uhrungszeichen nach- bzw. vorangestellt ist). Das ¨offnende und das schließende Anf¨uhrungszeichen werden beim Lesen des
Feldes nicht ¨ubernommen.

3. Ist das Feld in Anf¨uhrungszeichen eingeschlossen, werden beim Lesen des Feldes zwei aufeinanderfolgende Anf¨uhrungszeichen durch ein einzelnes ersetzt. Ein singul¨ares Anf¨uhrungszeichen
in einem durch Anf¨uhrungszeichen eingeschlossenen Feld ist nicht erlaubt.

4. Ist das Feld in Anf¨uhrungszeichen eingeschlossen, werden Leerzeichen zwischen dem schließenden Anf¨uhrungszeichen und dem n¨achsten Feldtrennzeichen bzw. dem Zeilenende ignoriert.


**Num** Zahl


alle Ziffern sowie Dezimalpunkt, evtl. Minuszeichen an erster Stelle


**Bool** boolescher Wert


’1’ – ja, ’0’ – nein


**Date** Datumsangabe


Format: 4 Stellen Jahr + 2 Stellen Monat + 2 Stellen Tag (entspricht ISO 8601, wobei Trennstriche
zwischen Jahr, Monat und Tag entfallen)


Das Pflichtfeld–Kennzeichen ist nur f¨ur Felder des Datentyps _Char_ relevant. Bei Feldern der anderen
Datentypen ist immer eine Angabe zu machen. Bei Feldern des Datentyps _Num_ sind die jeweils m¨oglichen
Werte der Beschreibung der jeweiligen Tabellen zu entnehmen.


In verschiedenen Tabellen werden Felder f¨ur _Einheiten_ definiert. Bez¨uglich der Angabe von Einheiten folgt
OCD dem Standard `openTRANS` f¨ur den zwischenbetrieblichen elektronischen Austausch von Gesch¨aftsdokumenten. Danach werden Einheiten gem¨aß dem _Common Code_ der UN/ECE Recommendation 20 [2]

angegeben.


2www.unece.org/cefact/rec/rec20en.htm


4


## **2 Die Tabellen**








|BillOfItems|Col2|
|---|---|
|**CompositeID**<br>**Position**<br>ItemID<br>RelObjID<br>Configurable<br><br>Quantity<br>_QuantUnit_||


























































|¨Ubersicht|Col2|Col3|
|---|---|---|
|IsFixedSet<br>BasketMode<br>PriceMode<br>ItemsConfigurable<br>Configurable<br>**CompositeID**<br>**LineNr**<br>TextID<br>**Article**<br>IdentKey<br>**System**<br>**ClassID**<br>_SchemeID_<br>FormatVersion<br>Region<br>DateTo<br>DateFrom<br>RelCoding<br>DataVersion<br>**_VariantCode_**<br>_VarCondVar_<br>_Comment_<br>Tables<br>PlaceHolderOn<br>**ArticleID**<br>**System**<br>ClassID<br>**ArticleID**<br>**PropertyName**<br>**LineNr**<br>**Value**<br>**_Variantcondition_**<br>_PackUnits_<br>_Width_<br>**PropertyClass**<br>**PropertyName**<br>IdentKey<br>_Height_<br>**Property**<br>_Depth_<br>**PropertyClass**<br>**PropertyName**<br>**Position**<br>_TextID_<br>_RelObjID_<br>IsDefault<br>_ValueTo_<br>_OpTo_<br>_ValueFrom_<br>_OpFrom_<br>_SuppressTxt_<br>_Raster_<br>_MeasureUnit_<br>_DateFrom_<br>_DateTo_<br>_Volume_<br>**PropertyClass**<br>**PropertyName**<br>**PropertyValue**<br>IdentKey<br>_VolumeUnit_<br>**PropValue**<br>_TaraWeight_<br>AddValues<br>Obligatory<br>_DecDigits_<br>Digits<br>Type<br>_RelObjID_<br>_TextID_<br>**Position**<br>**PropertyName**<br>**PropertyClass**<br>TxtControl<br>Scope<br>MultiOption<br>Restrictable<br>_HintTextID_<br>_NetWeight_<br>_WeightUnit_<br>_ItemsPerUnit_<br>**ArticleID**<br>**TaxID**<br>TaxCategory<br>TaxType<br>**Number**<br>**CompositeID**<br>**Position**<br>ItemID<br>RelObjID<br>Configurable<br>_TextID_<br>Quantity<br>_QuantUnit_<br>**_Region_**<br>**Country**<br>**Number**<br>_Minimum_<br>Type<br>_Maximum_<br>Precision<br>AddBefore<br>AddAfter<br>**ID**<br>DateTo<br>**DateFrom**<br>ArticleType<br>ManufacturerID<br>SeriesID<br>ShortTextID<br>_LongTextID_<br>RelObjID<br>FastSupply<br>_SchemeID_<br>_OrderUnit_<br>Discountable<br>**ArticleID**<br>IdentNr<br>**Type**<br>**Position**<br>PropGroupID<br>TextID<br>**ArticleID**<br>**EntityID**<br>_CatalogDir_<br>_CatalogFormat_<br>**ArticleID**<br>**_Variantcondition_**<br>**Type**<br>**Level**<br>Rule<br>_TextID_<br>PriceValue<br>DateTo<br>**DateFrom**<br>**Currency**<br>FixValue<br>**ScaleQuantity**<br>_RoundingID_<br>TextID<br>**SeriesID**<br>**Position**<br>RelName<br>Type<br>Domain<br>**Position**<br>PropertyClass<br>PropertyName<br>**PropGroupID**<br>_TextMode_<br>**RelObjID**<br>CodeBlock<br>**BlockNr**<br>**RelationName**<br>**PropertyName**<br>**PropertyValue**<br>**PropertyClass**<br>**ArticleID**<br>_RelObjID_<br>Name<br>_TextID_<br>**Position**<br>**ArticleID**<br>_MO_Bracket_<br>_MO_Sep_<br>Trim<br>_UnselectChar_<br>_Scheme_<br>**TextID**<br>**Language**<br>Textline<br>LineFormat<br>_VarCodeSep_<br>**ArticleID**<br>TaxID<br>**SchemeID**<br>_Visibility_<br>_InVisibleChar_<br>_ValueSep_<br>**ArticleTaxes**<br>**Group**<br>**PropGroupText**<br>**PriceText**<br>***Text**<br>**CodeScheme**<br>**PropertyClass**<br>**Relation**<br>**Price**<br>**PropGroup**<br>**Article2**<br>**Article**<br>**Identification**<br>**RelationObj**<br>**ArtBase**<br>**Series**<br>**SeriesText**<br>**Rounding**<br>**TaxScheme**<br>**BillOfItems**<br>**BillOfItemsText**<br>**PropertyText**<br>**PropHintText**<br>**Property**<br>**Identification**<br>**PropertyValue**<br>**Identification**<br>**<Value**<br>**Combination>**<br>**Packaging**<br>**Text**<br>**Classification**<br>**Classification**<br>**PropValueText**<br>**Version**<br>**Classification**<br>**Data**<br>**Identification**<br>**ArtShortText**<br>**ArtLongText**<br>**PropClassText**<br>**Property**<br>**Composite**|IsFixedSet<br>BasketMode<br>PriceMode<br>ItemsConfigurable<br>Configurable<br>**CompositeID**<br>**LineNr**<br>TextID<br>**Article**<br>IdentKey<br>**System**<br>**ClassID**<br>_SchemeID_<br>FormatVersion<br>Region<br>DateTo<br>DateFrom<br>RelCoding<br>DataVersion<br>**_VariantCode_**<br>_VarCondVar_<br>_Comment_<br>Tables<br>PlaceHolderOn<br>**ArticleID**<br>**System**<br>ClassID<br>**ArticleID**<br>**PropertyName**<br>**LineNr**<br>**Value**<br>**_Variantcondition_**<br>_PackUnits_<br>_Width_<br>**PropertyClass**<br>**PropertyName**<br>IdentKey<br>_Height_<br>**Property**<br>_Depth_<br>**PropertyClass**<br>**PropertyName**<br>**Position**<br>_TextID_<br>_RelObjID_<br>IsDefault<br>_ValueTo_<br>_OpTo_<br>_ValueFrom_<br>_OpFrom_<br>_SuppressTxt_<br>_Raster_<br>_MeasureUnit_<br>_DateFrom_<br>_DateTo_<br>_Volume_<br>**PropertyClass**<br>**PropertyName**<br>**PropertyValue**<br>IdentKey<br>_VolumeUnit_<br>**PropValue**<br>_TaraWeight_<br>AddValues<br>Obligatory<br>_DecDigits_<br>Digits<br>Type<br>_RelObjID_<br>_TextID_<br>**Position**<br>**PropertyName**<br>**PropertyClass**<br>TxtControl<br>Scope<br>MultiOption<br>Restrictable<br>_HintTextID_<br>_NetWeight_<br>_WeightUnit_<br>_ItemsPerUnit_<br>**ArticleID**<br>**TaxID**<br>TaxCategory<br>TaxType<br>**Number**<br>**CompositeID**<br>**Position**<br>ItemID<br>RelObjID<br>Configurable<br>_TextID_<br>Quantity<br>_QuantUnit_<br>**_Region_**<br>**Country**<br>**Number**<br>_Minimum_<br>Type<br>_Maximum_<br>Precision<br>AddBefore<br>AddAfter<br>**ID**<br>DateTo<br>**DateFrom**<br>ArticleType<br>ManufacturerID<br>SeriesID<br>ShortTextID<br>_LongTextID_<br>RelObjID<br>FastSupply<br>_SchemeID_<br>_OrderUnit_<br>Discountable<br>**ArticleID**<br>IdentNr<br>**Type**<br>**Position**<br>PropGroupID<br>TextID<br>**ArticleID**<br>**EntityID**<br>_CatalogDir_<br>_CatalogFormat_<br>**ArticleID**<br>**_Variantcondition_**<br>**Type**<br>**Level**<br>Rule<br>_TextID_<br>PriceValue<br>DateTo<br>**DateFrom**<br>**Currency**<br>FixValue<br>**ScaleQuantity**<br>_RoundingID_<br>TextID<br>**SeriesID**<br>**Position**<br>RelName<br>Type<br>Domain<br>**Position**<br>PropertyClass<br>PropertyName<br>**PropGroupID**<br>_TextMode_<br>**RelObjID**<br>CodeBlock<br>**BlockNr**<br>**RelationName**<br>**PropertyName**<br>**PropertyValue**<br>**PropertyClass**<br>**ArticleID**<br>_RelObjID_<br>Name<br>_TextID_<br>**Position**<br>**ArticleID**<br>_MO_Bracket_<br>_MO_Sep_<br>Trim<br>_UnselectChar_<br>_Scheme_<br>**TextID**<br>**Language**<br>Textline<br>LineFormat<br>_VarCodeSep_<br>**ArticleID**<br>TaxID<br>**SchemeID**<br>_Visibility_<br>_InVisibleChar_<br>_ValueSep_<br>**ArticleTaxes**<br>**Group**<br>**PropGroupText**<br>**PriceText**<br>***Text**<br>**CodeScheme**<br>**PropertyClass**<br>**Relation**<br>**Price**<br>**PropGroup**<br>**Article2**<br>**Article**<br>**Identification**<br>**RelationObj**<br>**ArtBase**<br>**Series**<br>**SeriesText**<br>**Rounding**<br>**TaxScheme**<br>**BillOfItems**<br>**BillOfItemsText**<br>**PropertyText**<br>**PropHintText**<br>**Property**<br>**Identification**<br>**PropertyValue**<br>**Identification**<br>**<Value**<br>**Combination>**<br>**Packaging**<br>**Text**<br>**Classification**<br>**Classification**<br>**PropValueText**<br>**Version**<br>**Classification**<br>**Data**<br>**Identification**<br>**ArtShortText**<br>**ArtLongText**<br>**PropClassText**<br>**Property**<br>**Composite**|IsFixedSet<br>BasketMode<br>PriceMode<br>ItemsConfigurable<br>Configurable<br>**CompositeID**<br>**LineNr**<br>TextID<br>**Article**<br>IdentKey<br>**System**<br>**ClassID**<br>_SchemeID_<br>FormatVersion<br>Region<br>DateTo<br>DateFrom<br>RelCoding<br>DataVersion<br>**_VariantCode_**<br>_VarCondVar_<br>_Comment_<br>Tables<br>PlaceHolderOn<br>**ArticleID**<br>**System**<br>ClassID<br>**ArticleID**<br>**PropertyName**<br>**LineNr**<br>**Value**<br>**_Variantcondition_**<br>_PackUnits_<br>_Width_<br>**PropertyClass**<br>**PropertyName**<br>IdentKey<br>_Height_<br>**Property**<br>_Depth_<br>**PropertyClass**<br>**PropertyName**<br>**Position**<br>_TextID_<br>_RelObjID_<br>IsDefault<br>_ValueTo_<br>_OpTo_<br>_ValueFrom_<br>_OpFrom_<br>_SuppressTxt_<br>_Raster_<br>_MeasureUnit_<br>_DateFrom_<br>_DateTo_<br>_Volume_<br>**PropertyClass**<br>**PropertyName**<br>**PropertyValue**<br>IdentKey<br>_VolumeUnit_<br>**PropValue**<br>_TaraWeight_<br>AddValues<br>Obligatory<br>_DecDigits_<br>Digits<br>Type<br>_RelObjID_<br>_TextID_<br>**Position**<br>**PropertyName**<br>**PropertyClass**<br>TxtControl<br>Scope<br>MultiOption<br>Restrictable<br>_HintTextID_<br>_NetWeight_<br>_WeightUnit_<br>_ItemsPerUnit_<br>**ArticleID**<br>**TaxID**<br>TaxCategory<br>TaxType<br>**Number**<br>**CompositeID**<br>**Position**<br>ItemID<br>RelObjID<br>Configurable<br>_TextID_<br>Quantity<br>_QuantUnit_<br>**_Region_**<br>**Country**<br>**Number**<br>_Minimum_<br>Type<br>_Maximum_<br>Precision<br>AddBefore<br>AddAfter<br>**ID**<br>DateTo<br>**DateFrom**<br>ArticleType<br>ManufacturerID<br>SeriesID<br>ShortTextID<br>_LongTextID_<br>RelObjID<br>FastSupply<br>_SchemeID_<br>_OrderUnit_<br>Discountable<br>**ArticleID**<br>IdentNr<br>**Type**<br>**Position**<br>PropGroupID<br>TextID<br>**ArticleID**<br>**EntityID**<br>_CatalogDir_<br>_CatalogFormat_<br>**ArticleID**<br>**_Variantcondition_**<br>**Type**<br>**Level**<br>Rule<br>_TextID_<br>PriceValue<br>DateTo<br>**DateFrom**<br>**Currency**<br>FixValue<br>**ScaleQuantity**<br>_RoundingID_<br>TextID<br>**SeriesID**<br>**Position**<br>RelName<br>Type<br>Domain<br>**Position**<br>PropertyClass<br>PropertyName<br>**PropGroupID**<br>_TextMode_<br>**RelObjID**<br>CodeBlock<br>**BlockNr**<br>**RelationName**<br>**PropertyName**<br>**PropertyValue**<br>**PropertyClass**<br>**ArticleID**<br>_RelObjID_<br>Name<br>_TextID_<br>**Position**<br>**ArticleID**<br>_MO_Bracket_<br>_MO_Sep_<br>Trim<br>_UnselectChar_<br>_Scheme_<br>**TextID**<br>**Language**<br>Textline<br>LineFormat<br>_VarCodeSep_<br>**ArticleID**<br>TaxID<br>**SchemeID**<br>_Visibility_<br>_InVisibleChar_<br>_ValueSep_<br>**ArticleTaxes**<br>**Group**<br>**PropGroupText**<br>**PriceText**<br>***Text**<br>**CodeScheme**<br>**PropertyClass**<br>**Relation**<br>**Price**<br>**PropGroup**<br>**Article2**<br>**Article**<br>**Identification**<br>**RelationObj**<br>**ArtBase**<br>**Series**<br>**SeriesText**<br>**Rounding**<br>**TaxScheme**<br>**BillOfItems**<br>**BillOfItemsText**<br>**PropertyText**<br>**PropHintText**<br>**Property**<br>**Identification**<br>**PropertyValue**<br>**Identification**<br>**<Value**<br>**Combination>**<br>**Packaging**<br>**Text**<br>**Classification**<br>**Classification**<br>**PropValueText**<br>**Version**<br>**Classification**<br>**Data**<br>**Identification**<br>**ArtShortText**<br>**ArtLongText**<br>**PropClassText**<br>**Property**<br>**Composite**|
|IsFixedSet<br>BasketMode<br>PriceMode<br>ItemsConfigurable<br>Configurable<br>**CompositeID**<br>**LineNr**<br>TextID<br>**Article**<br>IdentKey<br>**System**<br>**ClassID**<br>_SchemeID_<br>FormatVersion<br>Region<br>DateTo<br>DateFrom<br>RelCoding<br>DataVersion<br>**_VariantCode_**<br>_VarCondVar_<br>_Comment_<br>Tables<br>PlaceHolderOn<br>**ArticleID**<br>**System**<br>ClassID<br>**ArticleID**<br>**PropertyName**<br>**LineNr**<br>**Value**<br>**_Variantcondition_**<br>_PackUnits_<br>_Width_<br>**PropertyClass**<br>**PropertyName**<br>IdentKey<br>_Height_<br>**Property**<br>_Depth_<br>**PropertyClass**<br>**PropertyName**<br>**Position**<br>_TextID_<br>_RelObjID_<br>IsDefault<br>_ValueTo_<br>_OpTo_<br>_ValueFrom_<br>_OpFrom_<br>_SuppressTxt_<br>_Raster_<br>_MeasureUnit_<br>_DateFrom_<br>_DateTo_<br>_Volume_<br>**PropertyClass**<br>**PropertyName**<br>**PropertyValue**<br>IdentKey<br>_VolumeUnit_<br>**PropValue**<br>_TaraWeight_<br>AddValues<br>Obligatory<br>_DecDigits_<br>Digits<br>Type<br>_RelObjID_<br>_TextID_<br>**Position**<br>**PropertyName**<br>**PropertyClass**<br>TxtControl<br>Scope<br>MultiOption<br>Restrictable<br>_HintTextID_<br>_NetWeight_<br>_WeightUnit_<br>_ItemsPerUnit_<br>**ArticleID**<br>**TaxID**<br>TaxCategory<br>TaxType<br>**Number**<br>**CompositeID**<br>**Position**<br>ItemID<br>RelObjID<br>Configurable<br>_TextID_<br>Quantity<br>_QuantUnit_<br>**_Region_**<br>**Country**<br>**Number**<br>_Minimum_<br>Type<br>_Maximum_<br>Precision<br>AddBefore<br>AddAfter<br>**ID**<br>DateTo<br>**DateFrom**<br>ArticleType<br>ManufacturerID<br>SeriesID<br>ShortTextID<br>_LongTextID_<br>RelObjID<br>FastSupply<br>_SchemeID_<br>_OrderUnit_<br>Discountable<br>**ArticleID**<br>IdentNr<br>**Type**<br>**Position**<br>PropGroupID<br>TextID<br>**ArticleID**<br>**EntityID**<br>_CatalogDir_<br>_CatalogFormat_<br>**ArticleID**<br>**_Variantcondition_**<br>**Type**<br>**Level**<br>Rule<br>_TextID_<br>PriceValue<br>DateTo<br>**DateFrom**<br>**Currency**<br>FixValue<br>**ScaleQuantity**<br>_RoundingID_<br>TextID<br>**SeriesID**<br>**Position**<br>RelName<br>Type<br>Domain<br>**Position**<br>PropertyClass<br>PropertyName<br>**PropGroupID**<br>_TextMode_<br>**RelObjID**<br>CodeBlock<br>**BlockNr**<br>**RelationName**<br>**PropertyName**<br>**PropertyValue**<br>**PropertyClass**<br>**ArticleID**<br>_RelObjID_<br>Name<br>_TextID_<br>**Position**<br>**ArticleID**<br>_MO_Bracket_<br>_MO_Sep_<br>Trim<br>_UnselectChar_<br>_Scheme_<br>**TextID**<br>**Language**<br>Textline<br>LineFormat<br>_VarCodeSep_<br>**ArticleID**<br>TaxID<br>**SchemeID**<br>_Visibility_<br>_InVisibleChar_<br>_ValueSep_<br>**ArticleTaxes**<br>**Group**<br>**PropGroupText**<br>**PriceText**<br>***Text**<br>**CodeScheme**<br>**PropertyClass**<br>**Relation**<br>**Price**<br>**PropGroup**<br>**Article2**<br>**Article**<br>**Identification**<br>**RelationObj**<br>**ArtBase**<br>**Series**<br>**SeriesText**<br>**Rounding**<br>**TaxScheme**<br>**BillOfItems**<br>**BillOfItemsText**<br>**PropertyText**<br>**PropHintText**<br>**Property**<br>**Identification**<br>**PropertyValue**<br>**Identification**<br>**<Value**<br>**Combination>**<br>**Packaging**<br>**Text**<br>**Classification**<br>**Classification**<br>**PropValueText**<br>**Version**<br>**Classification**<br>**Data**<br>**Identification**<br>**ArtShortText**<br>**ArtLongText**<br>**PropClassText**<br>**Property**<br>**Composite**|**Identification**|**Identification**|
|IsFixedSet<br>BasketMode<br>PriceMode<br>ItemsConfigurable<br>Configurable<br>**CompositeID**<br>**LineNr**<br>TextID<br>**Article**<br>IdentKey<br>**System**<br>**ClassID**<br>_SchemeID_<br>FormatVersion<br>Region<br>DateTo<br>DateFrom<br>RelCoding<br>DataVersion<br>**_VariantCode_**<br>_VarCondVar_<br>_Comment_<br>Tables<br>PlaceHolderOn<br>**ArticleID**<br>**System**<br>ClassID<br>**ArticleID**<br>**PropertyName**<br>**LineNr**<br>**Value**<br>**_Variantcondition_**<br>_PackUnits_<br>_Width_<br>**PropertyClass**<br>**PropertyName**<br>IdentKey<br>_Height_<br>**Property**<br>_Depth_<br>**PropertyClass**<br>**PropertyName**<br>**Position**<br>_TextID_<br>_RelObjID_<br>IsDefault<br>_ValueTo_<br>_OpTo_<br>_ValueFrom_<br>_OpFrom_<br>_SuppressTxt_<br>_Raster_<br>_MeasureUnit_<br>_DateFrom_<br>_DateTo_<br>_Volume_<br>**PropertyClass**<br>**PropertyName**<br>**PropertyValue**<br>IdentKey<br>_VolumeUnit_<br>**PropValue**<br>_TaraWeight_<br>AddValues<br>Obligatory<br>_DecDigits_<br>Digits<br>Type<br>_RelObjID_<br>_TextID_<br>**Position**<br>**PropertyName**<br>**PropertyClass**<br>TxtControl<br>Scope<br>MultiOption<br>Restrictable<br>_HintTextID_<br>_NetWeight_<br>_WeightUnit_<br>_ItemsPerUnit_<br>**ArticleID**<br>**TaxID**<br>TaxCategory<br>TaxType<br>**Number**<br>**CompositeID**<br>**Position**<br>ItemID<br>RelObjID<br>Configurable<br>_TextID_<br>Quantity<br>_QuantUnit_<br>**_Region_**<br>**Country**<br>**Number**<br>_Minimum_<br>Type<br>_Maximum_<br>Precision<br>AddBefore<br>AddAfter<br>**ID**<br>DateTo<br>**DateFrom**<br>ArticleType<br>ManufacturerID<br>SeriesID<br>ShortTextID<br>_LongTextID_<br>RelObjID<br>FastSupply<br>_SchemeID_<br>_OrderUnit_<br>Discountable<br>**ArticleID**<br>IdentNr<br>**Type**<br>**Position**<br>PropGroupID<br>TextID<br>**ArticleID**<br>**EntityID**<br>_CatalogDir_<br>_CatalogFormat_<br>**ArticleID**<br>**_Variantcondition_**<br>**Type**<br>**Level**<br>Rule<br>_TextID_<br>PriceValue<br>DateTo<br>**DateFrom**<br>**Currency**<br>FixValue<br>**ScaleQuantity**<br>_RoundingID_<br>TextID<br>**SeriesID**<br>**Position**<br>RelName<br>Type<br>Domain<br>**Position**<br>PropertyClass<br>PropertyName<br>**PropGroupID**<br>_TextMode_<br>**RelObjID**<br>CodeBlock<br>**BlockNr**<br>**RelationName**<br>**PropertyName**<br>**PropertyValue**<br>**PropertyClass**<br>**ArticleID**<br>_RelObjID_<br>Name<br>_TextID_<br>**Position**<br>**ArticleID**<br>_MO_Bracket_<br>_MO_Sep_<br>Trim<br>_UnselectChar_<br>_Scheme_<br>**TextID**<br>**Language**<br>Textline<br>LineFormat<br>_VarCodeSep_<br>**ArticleID**<br>TaxID<br>**SchemeID**<br>_Visibility_<br>_InVisibleChar_<br>_ValueSep_<br>**ArticleTaxes**<br>**Group**<br>**PropGroupText**<br>**PriceText**<br>***Text**<br>**CodeScheme**<br>**PropertyClass**<br>**Relation**<br>**Price**<br>**PropGroup**<br>**Article2**<br>**Article**<br>**Identification**<br>**RelationObj**<br>**ArtBase**<br>**Series**<br>**SeriesText**<br>**Rounding**<br>**TaxScheme**<br>**BillOfItems**<br>**BillOfItemsText**<br>**PropertyText**<br>**PropHintText**<br>**Property**<br>**Identification**<br>**PropertyValue**<br>**Identification**<br>**<Value**<br>**Combination>**<br>**Packaging**<br>**Text**<br>**Classification**<br>**Classification**<br>**PropValueText**<br>**Version**<br>**Classification**<br>**Data**<br>**Identification**<br>**ArtShortText**<br>**ArtLongText**<br>**PropClassText**<br>**Property**<br>**Composite**|IdentNr<br>**Type**<br>**EntityID**|IdentNr<br>**Type**<br>**EntityID**|
||||



Prim¨arschl¨usselfelder sind durch Fettdruck hervorgehoben und Felder, die keine Pflichtfelder sind, durch Kursivdruck.


Die Struktur der Text-Tabellen ist nur einmal dargestellt. Die alternativen Text-Tabellen (s. Abschn. 2.20) sind der Ubersichtlichkeit [¨]


halber nicht dargestellt.


5


**2.2** **Die Artikeltabelle**


Tabellenname: `Article`
Pflichttabelle: ja

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ArticleID|X|Char||X|Grundartikelnummer|
|2.|ArticleType||Char||X|Artikelart|
|3.|ManufacturerID||Char||X|Herstellerk¨urzel|
|4.|SeriesID||Char||X|Serienk¨urzel|
|5.|ShortTextID||Char||X|Kurztextnummer|
|6.|LongTextID||Char|||Langtextnummer|
|7.|RelObjID||Num||X|Beziehungsobjekt-Nummer|
|8.|FastSupply||Num||X|Schnell-Lieferungsz¨ahler|
|9.|Discountable||Bool||X|rabattierbarer Artikel?|
|10.|OrderUnit||Char|3||Bestell-Einheit|
|11.|SchemeID||Char|||Identiﬁkator des Kodierungs-<br>schemas f¨ur die Generierung<br>der Endartikelnummer|



Anmerkungen:


 Dies ist die Haupttabelle f¨ur alle Artikel. Die Artikelnummer wird als Schl¨ussel f¨ur den Zugriff auf
weitere Tabellen zur Preisbestimmung, Klassifizierung usw. verwendet [3] .
Als Artikelnummer ist hier die vom Hersteller verwendete Grundartikelnummer (Modellnummer)
anzugeben. Uber die Artikel–Identifikationstabelle (s. Abschn. [¨] 2.3) k¨onnen weitere Nummern zur
Identifikation des Artikels in verschiedenen Kontexten hinterlegt werden.


 Die Artikelart (Feld 2) bestimmt grundlegende Eigenschaften des Artikels. Folgende Artikelarten
sind m¨oglich:

|Artikelart|Erkl¨arung|
|---|---|
|P|einfacher Artikel (plain article):<br>nicht konﬁgurierbar, keine Unterartikel|
|C|konﬁgurierbarer Artikel:<br>Eigenschaften des Artikels k¨onnen durch den Anwender<br>festgelegt werden, keine Unterartikel|
|CS|zusammengesetzter Artikel (composite):<br>kann Unterartikel enthalten, kann selber konﬁgurierbar sein|



 Die Herstellerk¨urzel (Feld 3) werden zentral durch die Firma EasternGraphics GmbH vergeben und
verwaltet.


 F¨ur das Serienk¨urzel (Feld 4) sollten idealerweise nur die Großbuchstaben und Ziffern aus dem
ASCII–Zeichensatz sowie der Unterstrich ’ `_` ’ verwendet werden.
Die Verwendung folgender Zeichen wird _nicht_ empfohlen: `\/?:*"><|,;=` sowie das Leerzeichen [4] .


 Die Textnummern (Felder 5 und 6) dienen als Schl¨ussel f¨ur die Tabellen mit den Kurz- bzw. Langbeschreibungen der Artikel (s. Abschn. 2.20).


 Die Beziehungsobjekt-Nummer dient als Schl¨ussel f¨ur den Zugriff auf das Beziehungsobjekt in der
Beziehungsobjekt–Tabelle (s. Abschn. 2.15), an das Beziehungswissen zu dem Artikel gebunden ist.
(Wird kein Beziehungsobjekt ben¨otigt, so ist die Nummer `0` anzugeben.)


3Die Aufteilung der Artikel-bezogenen Informationen auf verschiedene Tabellen erh¨oht die ¨Ubersichtlichkeit durch Ausblenden von optionalen Informationen und erleichtert die Erweiterbarkeit als auch den inkrementellen Datenaustausch.
4Werden diese Zeichen verwendet, ist die korrekte Verarbeitung der Daten auf allen Plattformen und in allen OFMLAnwendungssystemen _nicht_ gew¨ahrleistet.


6


 Durch den Schnell-Lieferungsz¨ahler (Feld 8) wird die Anzahl der Artikel bestimmt, ab der eine
Schnell-Lieferung m¨oglich ist, wobei die Zahl `0` anzeigt, dass generell keine Schnell-Lieferung f¨ur
den Artikel m¨oglich ist.


 Das Kennzeichen im Feld 9 gibt an, ob auf den ggf. hinterlegten Einkaufspreis des Artikels Rabatte
angewendet werden k¨onnen oder nicht. Der Wert `0` (false) bedeutet dabei, dass f¨ur den Artikel
abweichend von der allgemeinen Konditionierung des Herstellers _keine_ Abschl¨age vom Einkaufspreis
erlaubt sind.


 Im Feld 10 wird die Einheit angegeben, in der der Artikel bestellt werden kann. Auf diese Einheit
bezieht sich sowohl die Mengenangabe in einer Bestellung als auch der Preis in der Preistabelle.
Die Einheit muß gem¨aß dem Common Code der UN/ECE Recommendation 20 angegeben werden.
Gebr¨auchliche Einheiten f¨ur die M¨obelbranche sind `C62`    - _St¨uck_, `MTR`    - _Meter_ und `MTK`    - _Quadrat-_
_meter_ . Erfolgt keine Angabe, wird als Standardeinheit _St¨uck_ verwendet.


 Der im 11. Feld angegebene Identifikator dient zur Referenzierung des bei der Generierung der Endartikelnummer zu verwendenden Kodierungsschemas aus der Tabelle `CodeScheme` (Abschn. 2.24).
Ist kein oder ein in der Schematabelle nicht referenzierter Identifikator angegeben, wird f¨ur den
Artikel keine spezifische Endartikelnummer erzeugt [5] .


**2.3** **Die Artikel–Identifikationstabelle**


Tabellenname: `ArticleIdentification`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ArticleID|X|Char||X|(Grund)Artikelnummer|
|2.|VariantCode|X|Char|||Code der Artikelvariante|
|3.|SchemeID||Char|||Schema f¨ur Variantencode|
|4.|IdentKey||Char||X|Schl¨ussel f¨ur Identiﬁkationstabelle|



Anmerkungen:


 Die Tabelle dient zur Angabe von zus¨atzlichen Identifikationsnummern (unterschiedlichen Typs)
f¨ur Artikel. Die zus¨atzlichen Identifikationsnummern werden dabei nicht direkt in dieser Tabelle, sondern indirekt ¨uber den im Feld 4 angegebenen Schl¨ussel in der Tabelle `Identification`
(s.Abschn. 2.22) abgelegt.


 Soll die Identifikationsnummer nicht dem Grundartikel zugeordnet werden, sondern einer bestimmten Variante des (konfigurierbaren) Artikels, so muß diese im 2. Feld durch einen entsprechenden
Code spezifiziert werden. Im Feld 3 muß dazu das dabei verwendete Codierungsschema angegeben
werden (Schl¨ussel f¨ur Nummernschema–Tabelle, s.Abschn. 2.24).


 Existieren in der Tabelle zu einem Grundartikel mehrere Eintr¨age, so muss jeweils ein Variantencode
angegeben sein. Wird zu einer gegebenen Konfiguration eines Artikels kein passender Variantcode
gefunden, so kann f¨ur den Artikel keine Identifikation des gew¨unschten Typs ermittelt werden.


5Die Endartikelnummer ist dann gleich der Grundartikelnummer.


7


**2.4** **Die Klassifikationstabelle**


Tabellenname: `Classification`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ArticleID|X|Char||X|Grundartikelnummer|
|2.|System|X|Char||X|Name des Klassiﬁkationssystems<br>inkl. Versionsangabe|
|3.|ClassID||Char||X|ID der Klasse des Artikels|



Anmerkungen:


 Die Tabelle dient zur Klassifizierung eines Artikels.


 Ein Artikel kann dabei nach verschiedenen Klassifikationssystemen klassifiziert werden. Aktuell sind
folgende Systeme erlaubt:

|System|Erkl¨arung|
|---|---|
|ECLASS-x.y|Klassiﬁzierung nach dem eClass-Modell mit Angabe der Version|
|UNSPSC|Klassiﬁzierung nach dem Standard UN/SPSC|
|_<_Manufacturer~~_>_ *~~|Hersteller-speziﬁsche Klassiﬁzierung:<br>die Systembezeichnung wird aus dem Herstellerk¨urzel,<br>einem Unterstrich und einem beliebigen Nachsatz gebildet|



Hersteller-spezifische Klassifizierungen k¨onnen zur Definition von Warengruppen, Produkthierarchien u.¨a. verwendet werden.


Tabellenname: `ClassificationData`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|System|X|Char||X|Name des Klassiﬁkationssystems<br>inkl. Versionsangabe|
|2.|ClassID|X|Char||X|ID der Klasse|
|3.|TextID||Char||X|Textnummer|



Anmerkungen:


 Die Tabelle dient zur Angabe von Informationen zu einer Klassifizierung [6] .


 Die Textnummer dient als Schl¨ussel f¨ur die Tabelle `ClassificationText` (s. Abschn. 2.20), in
welcher sprachspezifische Texte hinterlegt werden k¨onnen, die die Klassifizierung beschreiben.


6Momentan wird nur die Angabe eines Textes zur Beschreibung der Klassifizierung unterst¨utzt.


8


**2.5** **Die Packaging–Tabelle**


Tabellenname: `Packaging`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ArticleID|X|Char||X|Grundartikelnummer|
|2.|Variantcondition|X|Char|||Variantenkondition|
|3.|Width||Char|||Breite der Verpackungseinheit|
|4.|Height||Char|||H¨ohe der Verpackungseinheit|
|5.|Depth||Char|||Tiefe der Verpackungseinheit|
|6.|MeasureUnit||Char|3||Maßeinheit der Dimensionen 3 bis 5|
|7.|Volume||Char|||Volumen der Verpackungseinheit|
|8.|VolumeUnit||Char|3||Maßeinheit des Volumens|
|9.|TaraWeight||Char|||Gewicht der Verpackungseinheit|
|10.|NetWeight||Char|||Gewicht des Einzelartikels|
|11.|WeightUnit||Char|3||Maßeinheit der Gewichte 9 bis 10|
|12.|ItemsPerUnit||Char|||Anzahl der Artikel<br>pro Verpackungseinheit|
|13.|PackUnits||Char|||Anzahl der Verpackungseinheiten, die<br>f¨ur den Artikel verwendet werden|



Anmerkungen:


 Diese Tabelle dient zur Angabe von Informationen zur Verpackung eines Artikels, der komplett
geliefert wird.


 Eine Verpackungseinheit kann dabei mehrere Artikel (derselben Nummer) beinhalten (Feld 12).
Teile des Artikels, z.B. optionale Zubeh¨orteile, k¨onnen aber auch in separaten Verpackungseinheiten
geliefert werden (Feld 13).


 Mit Hilfe von _Variantenkonditionen_ (Feld 2) k¨onnen – in Abh¨angigkeit von speziellen Merkmalsauspr¨agungen – Maße, Volumen, Gewichte und Anzahlen von Verpackungseinheiten hinterlegt werden, die von der Grundausf¨uhrung des Artikels abweichen. Die Verwendung und Behandlung solcher
Variantenkonditionen ist in Abschn. 6 beschrieben. Die Betr¨age in den Eintr¨agen mit Variantenkonditionen (nicht-leeres Feld 2) werden dabei immer als Differenz zu dem jeweiligen Grundbetrag
aus dem Eintrag ohne Variantenkondition angegeben und k¨onnen negativ sein [7] .


 Wenn das Feld 2 (Variantenkondition) nicht leer ist, kann im Feld 1 der Joker-Artikel ”*“ zur Angabe artikel¨ubergreifend einheitlicher Werte f¨ur variantenabh¨angige L¨angen, Volumen und Gewichte
verwendet werden.
Dieser artikel-neutrale Tabelleneintrag wird jedoch nur dann ber¨ucksichtigt, wenn f¨ur den bearbeiteten Artikel kein eigener, spezifischer Eintrag mit der gleichnamigen Variantenkondition existiert.


 Die Felder 3-5, 7, 9, 10, 12 und 13 sind als optionale Zeichenkettenfelder deklariert, sie k¨onnen also
auch leer sein. Wenn nicht leer, m¨ussen die Felder Zeichenketten–Darstellungen numerischer Werte
enthalten [8] .


7Dies impliziert, daß f¨ur einen gegebenen Artikel immer ein Tabelleneintrag ohne Variantenkondition vorhanden sein
muss. Die Datenelemente darin k¨onnen bei Bedarf den Wert `0.0` aufweisen.
8¨aquivalent zu den Werten numerischer Merkmale in der Wertetabelle


9


 In den Feldern 6, 8 und 11 sind folgende Maßeinheiten erlaubt [9] :

|L¨angen (Feld 6):|Col2|
|---|---|
|Code der Maßeinheit|Erkl¨arung|
|CMT<br>FOT<br>INH<br>MMT<br>MTR|Zentimeter<br>Fuß (foot)<br>Zoll (inch)<br>Millimeter<br>Meter|



Das Feld darf nicht leer sein, wenn eines der Felder 3-5 nicht leer ist.

|Volumen (Feld 8):|Col2|
|---|---|
|Code der Maßeinheit|Erkl¨arung|
|INQ<br>LTR<br>MTQ|Kubikzoll (cubic inch)<br>Liter<br>Kubikmeter|



Das Feld darf nicht leer sein, wenn das Feld 7 nicht leer ist.

|Gewicht (Feld 11):|Col2|
|---|---|
|Code der Maßeinheit|Erkl¨arung|
|KGM<br>LBR<br>MGM|Kilogramm<br>Pfund<br>Milligramm|



Das Feld darf nicht leer sein, wenn eines der Felder 9 und 10 nicht leer ist.


 Das Volumen kann sich von dem rechnerisch aus Breite, H¨ohe und Tiefe ermittelten Volumen
unterscheiden, wenn Verpackungen verwendet werden, die ineinander gestapelt werden k¨onnen.


 Das Gesamtgewicht (Brutto) einer Verpackungseinheit ergibt sich aus dem Gewicht der Verpackung
(Feld 9) plus dem Produkt aus Gewicht des Einzelartikels (Feld 10) und der Anzahl der Artikel pro
Verpackungseinheit (Feld 12).


9entspricht dem Common Code der UN/ECE Recommendation 20 (www.unece.org/cefact/rec/rec20en.htm)


10


**2.6** **Die Tabelle kompositer Artikel**


Tabellenname: `Composite`
Pflichttabelle: nein [10]


_Komposite Artikel_ sind Artikel, die eine feste oder variable Anzahl von Unterartikeln (sub items) beinhalten. Dies kann eine simple Aggregation im Sinne eines Sets sein, aber auch eine Komposition aus
funktionalen Gesichtspunkten.

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|CompositeID|X|Char||X|Artikelnummer des<br>kompositen Artikels|
|2.|IsFixedSet||Bool||X|Anzahl der Unterartikel fest ?|
|3.|Conﬁgurable||Bool||X|kompositer Artikel konﬁgurierbar ?|
|4.|ItemsConﬁgurable||Bool||X|Unterartikel konﬁgurierbar ?|
|5.|PriceMode||Char|3|X|Modus der Preisermittlung|
|6.|BasketMode||Char|1|X|Modus der Darstellung der<br>Unterartikel im Warenkorb|
|7.|TextMode||Char|||Modus der Textdarstellung<br>der Unterartikel im Warenkorb|



Anmerkungen:


 Die Tabelle dient zur Festlegung der allgemeinen Eigenschaften eines kompositen Artikels. Die
Unterartikel werden in der St¨ucklistentabelle (s. n¨achster Abschnitt) festgelegt.


 Im Feld 2 wird angegeben, ob die Anzahl der Unterartikel fest ist. Wenn _nein_ angegeben ist, so
variiert die Anzahl der Unterartikel in Abh¨angigkeit von den in der St¨ucklistentabelle zu den Unterartikeln zu hinterlegenden Existenzbedingungen.
Ist _ja_ angegeben, werden evtl. dennoch hinterlegte Existenzbedingungen f¨ur Unterartikel nicht ausgewertet!


 Im Feld 3 wird angegeben, ob der komposite Artikel selber konfigurierbar ist. Wenn _nein_ angegeben ist, kann der komposite Artikel nicht konfiguriert werden, auch wenn ihm Merkmalsklassen
(s. Abschn. 2.8) zugeordnet sind.


 Ist der Wert im Feld 4 _nein_, so d¨urfen die Unterartikel generell nicht konfiguriert werden, auch wenn
dies f¨ur einzelne Unterartikel in der St¨ucklistentabelle anderweitig spezifiziert ist.
Andersherum k¨onnen einzelne Unterartikel in der St¨ucklistentabelle von der Konfigurierbarkeit
ausgeschlossen werden, auch wenn die Konfigurierbarkeit durch den Wert _ja_ im Feld 4 generell
freigeschaltet ist.


 Der Modus im Feld 5 bestimmt die Art und Weise der Preisermittlung f¨ur den kompositen Artikel:

|Preismodus|Erkl¨arung|
|---|---|
|C|Preis ist an den kompositen Artikel gebunden<br>(ggf. mit Variantenkonditionen)|
|S|Preis ergibt sich aus der Summe der Preise der Unterartikel|
|C+S|Preis des kompositen Artikel plus Summe der<br>Preise der Unterartikel|



Prinzipiell ist f¨ur jede m¨ogliche Wertkombination der Felder 2-4 jeder der drei Preis-Modi denkbar/m¨oglich. Es wird deswegen keine Begrenzung vorgegeben. Bei der Datenerfassung muß die Stimmigkeit beachtet/gew¨ahrleistet werden. Sind z.B. die Unterartikel konfigurierbar (Feld 4 wahr), so
macht der Preis-Modus “C“ in der Regel keinen Sinn, es sei denn, es ist durch Beziehungswissen
(s.u.) gew¨ahrleistet, daß die Wertemengen aller preisrelevanten Merkmale der Unterartikel auf je
genau einen Wert eingeschr¨ankt sind.


10Die Tabelle wird nur ben¨otigt, wenn die Datenbank auch tats¨achlich komposite Artikel enthalten soll.


11


 Uber den Modus im Feld 6 kann die Darstellung der Unterartikel in kaufm¨annischen Formularen [¨]
gesteuert werden:

|Basket-Modus|Erkl¨arung|
|---|---|
|H|Unterartikel werden als Unterpositionen (d.h. hierarchisch)<br>dargestellt, ggf. ohne Preis|
|T|die Unterartikel werden im Beschreibungstext des kompositen<br>Artikels angef¨uhrt|



 Der Modus im Feld 7 gibt an, wie die Unterartikel im Basket-Modus “T“ im Beschreibungstext des
kompositen Artikels beschrieben werden sollen [11] :

|Text-Modus|Erkl¨arung|
|---|---|
|BAN<br>FAN<br>ST<br>LT<br>BAN+ST<br>BAN+LT<br>FAN+ST<br>FAN+LT<br>ST+BAN<br>ST+FAN<br>LT+BAN<br>LT+FAN|durch die Grundartikelnummer (base article number)<br>durch die Endartikelnummer (ﬁnal article number)<br>durch Artikelkurztext (short text)<br>durch Artikellangtext (long text)<br>durch Grundartikelnummer und Artikelkurztext<br>durch Grundartikelnummer und Artikellangtext<br>durch Endartikelnummer und Artikelkurztext<br>durch Endartikelnummer und Artikellangtext<br>durch Artikelkurztext und Grundartikelnummer<br>durch Artikelkurztext und Endartikelnummer<br>durch Artikellangtext und Grundartikelnummer<br>durch Artikellangtext und Endartikelnummer|



**2.7** **Die St¨ucklistentabelle**


Tabellenname: `BillOfItems`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|CompositeID|X|Char||X|Artikelnummer des kompositen Artikels|
|2.|Position|X|Num||X|Position des Unterartikels|
|3.|ItemID||Char||X|Artikelnummer des Unterartikels|
|4.|RelObjID||Num||X|Beziehungsobjekt–Nummer|
|5.|Conﬁgurable||Bool||X|Unterartikel konﬁgurierbar ?|
|6.|Quantity||Num||X|Menge|
|7.|QuantUnit||Char|3||Mengeneinheit|
|8.|TextID||Char|||Textnummer|



Anmerkungen:


 In dieser Tabelle wird angegeben, welche Unterartikel (sub items) ein kompositer Artikel (s. vorheriger Abschn.) beinhalten kann.


 Die Position eines Unterartikels innerhalb des kompositen Artikels (Feld 2) wird bei der Bestelllistenausgabe ber¨ucksichtigt.


 Uber das Beziehungsobjekt (Feld 4) kann eine Existenzbedingung f¨ur den Unterartikel angegeben [¨]
werden (s. Abschn. 2.15). Existenzbedingungen sind als Beziehungsart _Vorbedingung_ mit Verwendungsgebiet “BOI“ anzugeben. Falls ein Existenzbedingung hinterlegt ist und diese nicht erf¨ullt ist,


11F¨ur den Basket-Modus“H“ ist dieser Modus nicht notwendig, da sich die Darstellung der Unterartikel als Bestell-Position
dann nach den Vorgaben der jeweiligen Applikation richtet.


12


wird der Unterartikel nicht in die aktuelle St¨uckliste des kompositen Artikels aufgenommen.
Existenzbedingungen werden jedoch grunds¨atzlich nur dann ausgewertet, wenn im Feld
_IsFixedSet_ der Tabelle `Composite` (s. vorherigen Abschn.) f¨ur den kompositen Artikel _nein_ angegeben ist.


 Im Feld 5 wird angegeben, ob der Unterartikel konfigurierbar ist. Wenn _nein_ angegeben ist, kann
er nicht konfiguriert werden, auch wenn ihm Merkmalsklassen (s. Abschn. 2.8) zugeordnet sind.
Unterartikel sind generell jedoch nur dann konfigurierbar, wenn dies durch den kompositen Artikel
freigegeben ist, siehe Feld _ItemsConfigurable_ in der Tabelle `Composite` .


 Uber die Mengenangabe in Feld 6 k¨onnen gleichartige Artikel auf einer St¨ucklistenposition zusam- [¨]
mengefaßt werden. Die zu dieser Position geh¨orenden Unterartikel d¨urfen dann nicht konfiguriert
werden, d.h. der Wert in Feld 5 wird ignoriert [12] .


 Im Feld 7 wird die Einheit angegeben, auf die sich die Mengenangabe in Feld 6 bezieht. Die Einheit
muß gem¨aß dem Common Code der UN/ECE Recommendation 20 angegeben werden. Erfolgt keine
Angabe, wird als Standardeinheit _St¨uck_ ( `C62` ) verwendet.


 Die Textnummer im Feld 8 dient als Schl¨ussel f¨ur die Tabelle `BillOfItemsText` (s. Abschn. 2.20),
in der Texte hinterlegt werden k¨onnen, die in einem kaufm¨annischen Formular zus¨atzlich zu den
normalen Artikelinformationen f¨ur eine St¨ucklistenposition auszugeben sind. Die Art und Weise
dieser Ausgabe ist abh¨angig von der verwendeten Applikation.


**2.8** **Die Merkmalsklassentabelle**


Tabellenname: `PropertyClass`
Pflichttabelle: ja [13]

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ArticleID|X|Char||X|Artikelnummer|
|2.|Position|X|Num||X|Position der Klasse|
|3.|Name||Char||X|Name der Klasse|
|4.|TextID||Char|||Textnummer|
|5.|RelObjID||Num||X|Beziehungsobjekt-Nummer|



Anmerkungen:


 In dieser Tabelle werden den Artikeln die Merkmalsklassen zugeordnet, die die Merkmale des Artikels beschreiben.


 Die Position der Klasse innerhalb der Menge der Merkmalsklassen eines Artikels beeinflußt die
Reihenfolge in Aufz¨ahlungen der Merkmale des Artikels (Variantentext, Eigenschaftseditoren u.¨a.).


 F¨ur den Namen einer Merkmalsklasse (Feld 3) sind alle alphanumerischen Zeichen inklusive dem
Unterstrich erlaubt, wobei das erste Zeichen kein numerisches sein darf.


 Uber das Beziehungsobjekt (Feld 5) k¨onnen an die Merkmalsklasse Beziehungen vom Typ [¨] _Vorbe-_
_dingung_ und _Aktion_ gebunden werden (s. Abschn. 2.15). (Wird kein Beziehungsobjekt ben¨otigt, so
ist die Nummer `0` anzugeben.)


12Hinweis: Sollen die Unterartikel in einer OFML–basierten Anwendung auch graphisch dargestellt werden, muß vor
der OCD–Datenanlage abgekl¨art werden, ob die jeweilige Anwendung eine graphische Darstellung f¨ur zusammengefasste
St¨ucklistenartikel unterst¨utzt.
13Die Tabelle kann entfallen, wenn in der Datenbank keine Konfigurationsdaten, sondern z.B. nur Artikeltexte und -preise
angelegt werden sollen.


13


**2.9** **Die Merkmalstabelle**


Tabellenname: `Property`
Pflichttabelle: ja [14]

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|PropertyClass|X|Char||X|Bezeichner der Merkmalsklasse|
|2.|PropertyName|X|Char||X|Bezeichner des Merkmals|
|3.|Position||Num||X|Position des Merkmals|
|4.|TextID||Char|||Textnummer|
|5.|RelObjID||Num||X|Beziehungsobjekt-Nummer|
|6.|Type||Char|1|X|Datentyp der Merkmalswerte:<br>C - Char<br>T - Text<br>N - Num<br>L - Length|
|7.|Digits||Num||X|Anzahl der Stellen (gesamt)|
|8.|DecDigits||Num|||(davon) Anzahl der Nachkommastellen|
|9.|Obligatory||Bool|1|X|Eingabe erfordert?|
|10.|AddValues||Bool|1|X|Zus¨atzliche Werte erlaubt?|
|11.|Restrictable||Bool|1|X|Wertemenge einschr¨ankbar?|
|12.|MultiOption||Bool|1|X|mehrwertiges Merkmal?|
|13.|Scope||Char|2|X|Geltungsbereich, s. Tabelle unten|
|14.|TxtControl||Num||X|Text-Steuerung|
|15.|HintTextID||Char|||Textnummer f¨ur Hinweistext|


|Geltungsbereich|Erkl¨arung (Details s.u.)|
|---|---|
|C<br>R<br>RV<br>RG|konﬁgurierbar (sichtbar)<br>nur in Beziehungswissen<br>nicht konﬁgurierbar, aber f¨ur Anwender sichtbar<br>nicht konﬁgurierbar, aber Graﬁk-relevant|



Anmerkungen:


 In dieser Tabelle werden die Merkmale pro Merkmalsklasse aufgelistet.


 Die Namen der Merkmale sind symbolische (sprachunabh¨angige) Bezeichner. Verwendet werden
d¨urfen alphanumerische Zeichen inklusive dem Unterstrich, wobei das erste Zeichen kein numerisches
sein darf.
Sprechende (sprachabh¨angige) Bezeichner (zur Verwendung in den Benutzeroberfl¨achen) werden in
der Tabelle `PropertyText` abgelegt (s. Abschn. 2.20). Dazu wird im 4. Feld eine Textnummer als
Zugriffsschl¨ussel vergeben.
Die Angabe einer Textnummer ist nur bei sichtbaren Merkmalen gem¨aß Feld 13 notwendig (siehe
dazu auch Anmerkung unten).


 Innerhalb der Merkmalsklassen eines Artikels darf ein gegebenes Merkmal (Feld 2) nur einmal
vorkommen [15] .


 Die im Feld 3 angegebene Position des Merkmals innerhalb der Merkmalsklasse beeinflußt die Reihenfolge in Aufz¨ahlungen der Merkmale des Artikels (Variantentext, Eigenschaftseditoren u.¨a.).


14Die Tabelle kann entfallen, wenn in der Datenbank keine Konfigurationsdaten, sondern z.B. nur Artikeltexte und -preise
angelegt werden sollen.
15Ansonsten kann Beziehungswissen (s. Abschn. 2.16) nicht eindeutig ausgewertet werden, da dort die Merkmale nicht
mit ihrer Merkmalsklasse qualifiziert werden.


14


 Die Beziehungsobjekt-Nummer (Feld 5) dient als Schl¨ussel f¨ur den Zugriff auf das Beziehungsobjekt
in der Tabelle `RelationObj` (s. Abschn. 2.15), an welches das Beziehungswissen f¨ur das Merkmal
gebunden ist. (Wird kein Beziehungsobjekt ben¨otigt, so ist die Nummer `0` anzugeben.)


 Der Datentyp (Feld 6) legt den Typ der Werte fest, die das Merkmal annehmen kann, und bestimmt
die Art der Darstellung von Werten des Merkmals in der Merkmalswerttabelle:


**–** Werte des Datentyps ’C’ sind einfache Zeichenketten mit einer maximalen L¨ange gem¨aß Angabe
im Feld 7. Es d¨urfen alle darstellbaren Zeichen aus dem Zeichensatz `ISO 8859-1` (Latin-1)
außer dem Leerzeichen und dem Backslash (’ `\` ’) verwendet werden.


**–** Der Datentyp ’T’ wird f¨ur Merkmale verwendet, bei denen der Anwender einen beliebigen
Text inklusive Zeilenumbr¨uche eingeben kann. Werte dieses Typs sind Zeichenketten mit einer
maximalen L¨ange gem¨aß Angabe im Feld 7. (Ist im Feld 7 der Wert `0` eingetragen, besteht
keine L¨angenbegrenzung.) Zeilenumbr¨uche werden dabei durch ein Newline-Zeichen (’ `\n` ’) dargestellt. In der Merkmalswerttabelle d¨urfen keine Werte hinterlegt sein (bzw. diese werden
ignoriert).


**–** Werte der Datentypen ’N’ und ’L’ sind reelle oder ganze Zahlen, die in einfacher Dezimalpunktnotation dargestellt werden. Das Feld 7 gibt dabei die maximale Anzahl aller Stellen
ohne Dezimalpunkt vor, und Feld 8 die Anzahl der davon f¨ur den gebrochenen Teil verwendeten Darstellung. Bei negativen Zahlen steht an erster Stelle das Minuszeichen (womit f¨ur die
Ziffernstellen eine Stelle weniger zur Verf¨ugung steht als in Feld 7 angegeben).


**–** Die Datentypen ’N’ und ’L’ (Feld 6) werden im wesentlichen gleich behandelt. Der Unterschied
besteht in der Formatangabe des OFML–Merkmals, welches f¨ur das jeweilige OCD-Merkmal
generiert wird (s.a. OFML–Dokumentation, Abschnitt ”Formatspezifikationen f¨ur Properties“
im Anhang ”Formatspezifikationen“):

    - Das Format f¨ur Merkmale des Typs ’N’ ist `%` _<_ `Feld7` _>_ `.` _<_ `Feld8` _>_ `f`, wenn die Anzahl der
Nachkommastellen (Feld 8) ungleich 0 ist, sonst `%` _<_ `Feld7` _>_ `d` .

    - Das Format f¨ur Merkmale des Typs ’L’ ist `@L`, womit der Property–Editor der Applikation
aufgefordert ist, zur Darstellung bzw. Eingabe des Wertes die vom Nutzer eingestellte
Maßeinheit zu verwenden. Der Property–Editor f¨uhrt dazu eine Konvertierung zwischen
der nutzerdefinierten Maßeinheit und der in OFML f¨ur L¨angenmaßangaben verwendeten
Maßeinheit (m) durch. Werte f¨ur Merkmale dieses Typs in der Tabelle `PropertyValue`
m¨ussen also in Metern abgegeben werden.


 In Feld 9 wird spezifiziert, ob es sich bei dem Merkmal um ein Pflichtmerkmal (1) oder ein optionales
Merkmal (0) handelt. Ein Pflichtmerkmal muß durch den Anwender bewertet werden. Solange ein
Artikel nicht bewertete Pflichtmerkmale besitzt, ist seine Konfiguration nicht vollst¨andig (ung¨ultig).
Das Kennzeichen ist nur f¨ur Merkmale des Typs ’C’ relevant.
Ist f¨ur ein als optional gekennzeichnetes Merkmal eine Wertemenge (siehe Tabelle `PropertyValue`,
Abschn. 2.13) vorgegeben, so wird vom Anwendungssystem automatisch zus¨atzlich der Pseudo–
Wert `VOID` f¨ur den Zustand ”nicht ausgew¨ahlt“ generiert und verwendet.
Bei Pflicht–Merkmalen mit einer vorgegebenen Menge von Merkmalswerten und ohne M¨oglichkeit
der freien Werteingabe (siehe Feld 10) wird durch das Anwendungssystem gew¨ahrleistet, daß immer
ein Wert aus der Wertetabelle ausgew¨ahlt ist; ein solches Merkmal ist also immer bewertet. Bei anderen Merkmalen muß durch Bereitstellung von Produktbeziehungswissen (Auswahlbedingungen,
siehe Tabelle `RelationObj`, Abschn.2.15) die Vollst¨andigkeit der Konfiguration sichergestellt werden.
Bei der Verwendung dieses Kennzeichens ist zu beachten, daß es sich auf die Eingabepflicht f¨ur
den Anwender bezieht. Es besteht kein direkter Zusammenhang mit dem produktlogischen Begriff
_Option_ f¨ur Merkmale, bei dem der Anwender aus einer Menge von Werten (Choiceliste) ausw¨ahlen
kann, aber nicht muss. Optionen aus produktlogischer Sicht k¨onnen auch durch OCD–Merkmale
mit Pflicht–Kennzeichen abgebildet werden. Bei numerischen Merkmalen ist dies sogar erforderlich
(s.o.), der Zustand ”nicht ausgew¨ahlt“ kann und muß dann durch einen speziellen Wert (z.B. `0` )
abgebildet werden.


 Das Kennzeichen im Feld 10 ( `AddValues` ) gibt an, ob der Anwender frei Werte eingeben kann, ggf.
zus¨atzlich zu den in der Wertetabelle f¨ur das Merkmal hinterlegten Werten. Dies kann zur Eingabe


15


von freien Texten, Mengen oder Maßen genutzt werden.
Hinweis: Ist bei numerischen Merkmalen die Eingabe nur in bestimmten Wertebereichen erlaubt,
so m¨ussen in der Wertetabelle entsprechende Interval-Werte hinterlegt werden (siehe Tabelle
`PropertyValue`, Abschn. 2.13) und das Kennzeichen muß dann den Wert 0 ( _false_ ) besitzen.


Das Kennzeichen wird nur f¨ur einwertige und nicht-einschr¨ankbare konfigurierbare Merkmale ausgewertet, mit folgenden Einschr¨ankungen:

**–** Merkmalen des Typs T sowie Merkmalen ohne hinterlegte Werte in der Wertetabelle kann
generell jeder beliebige Wert zugewiesen werden, d.h. auch, wenn das Kennzeichen den Wert
0 ( _false_ ) hat.

**–** Sind f¨ur das Merkmal Werte in der Stammdatentabelle hinterlegt, k¨onnen auch nur diese Werte
zugewiesen bzw. gesetzt werden, d.h., das Kennzeichen wird dann ignoriert.


Bei allen anderen Merkmalsarten wird das Feld nicht ausgewertet. Bez¨uglich der Zuweisung von
Werten gelten statt dessen folgende Bestimmungen:

**–** Nicht-konfigurierbaren Merkmalen kann generell jeder beliebige Wert zugewiesen werden, es sei
denn, f¨ur das Merkmal sind Werte in der Wertetabelle hinterlegt: dann k¨onnen dem Merkmal
auch nur diese Werte zugewiesen werden.

**–** Einschr¨ankbaren bzw. mehrwertigen konfigurierbaren Merkmalen k¨onnen nur die in der Wertetabelle hinterlegten Werte zugewiesen werden.


 Soll die Wertemenge des Merkmals durch Beziehungen vom Typ _Constraint_ eingeschr¨ankt werden
k¨onnen (s. Abschn.2.15), muß das Kennzeichen im Feld 11 auf `1` gesetzt sein.
Die Menge der Werte eines normalen, _nicht_ einschr¨ankbaren Merkmals, aus denen der Anwender
ausw¨ahlen kann, umfasst alle Werte, die in der Merkmalswerttabelle f¨ur das Merkmal angelegt sind
**und** die entweder keine Vorbedingung besitzen oder deren Vorbedingungen hinsichtlich der aktuellen Konfiguration des Artikels erf¨ullt sind. Pflichtmerkmale sind dabei immer mit einem der Werte
belegt.
_Einschr¨ankbare Merkmale_ werden diesbez¨uglich anders behandelt. Die Wertemenge dieser Merkmale wird ausgehend von der Menge der Werte in der Merkmalswerttabelle durch Constraints eingeschr¨ankt. Sie gelten erst dann als bewertet, wenn die Wertemenge entweder durch ein Constraint
auf genau einen Wert eingeschr¨ankt wurde oder wenn eine Auswahl durch den Anwender erfolgt ist.
Die Konfiguration eines Artikels ist erst vollst¨andig, wenn alle einschr¨ankbaren Merkmale bewertet
sind. Ist ein einschr¨ankbares Merkmal nicht bewertet, kann der Artikel nicht bestellt werden.


 Mehrwertige Merkmale (Feld 12) sind Merkmale, die mehrere Werte auf einmal annehmen k¨onnen
(z.B. ”Sonderausstattung“).


 Der _Geltungsbereich_ (Feld 13: `Scope` ) gibt an, ob das Merkmal vom Anwender eines Konfigurationssystems konfiguriert (ver¨andert) werden darf, ob es f¨ur den Anwender sichtbar ist und ob es bei
der Erzeugung der grafischen Repr¨asentation des Artikels ben¨otigt wird:


**–** Nur Merkmale des Geltungsbereich ”C“ (oder Leerzeichen, d.h. keine Angabe) sind konfigurierbar. Sie sind damit per se auch sichtbar und k¨onnen bei der Erzeugung der grafischen
Repr¨asentation verwendet werden.


**–** Merkmale aller anderen Geltungsbereiche sind Hilfsmerkmale, die in Beziehungswissen verwendet werden k¨onnen.

**–** Merkmale des Geltungsbereichs ”RV“ werden dem Anwender dar¨uberhinaus als nur-lesbar
(read-only) angezeigt und k¨onnen auch bei der Erzeugung der grafischen Repr¨asentation verwendet werden.

**–** Merkmale des Geltungsbereichs ”RG“ sind f¨ur den Anwender nicht sichtbar, werden aber bei
der Erzeugung der grafischen Repr¨asentation ben¨otigt.


Der Geltungsbereich hat auch Einfluß auf die Persistenz und Initialisierung:

**–** Merkmale des Geltungsbereichs ”R“ werden nur innerhalb der Auswertung von Beziehungswissen verwendet. Der aktuelle Zustand dieser Merkmale ist deswegen außerhalb eines Konfigurationsvorgangs nicht verf¨ugbar (d.h., wird nicht persistent am Artikel gespeichert). Eine
Konsequenz davon ist, daß Merkmale dieses Scopes zu Beginn jedes Konfigurationsvorgangs
initialisiert werden (s.u.).


16


**–** Der Zustand der Merkmale aller anderen Geltungsbereiche wird auch nach Ausf¨uhrung eines
Konfigurationsvorgangs ben¨otigt [16] und deswegen persistent am Artikel gespeichert. Die Initialisierung (s.u.) findet bei diesen Merkmalen somit nur einmal unmittelbar bei Artikelerzeugung
statt.


 Das Feld 14 enth¨alt einen Code, der die Generierung des Textes steuert, der das Merkmal in kaufm¨annischen Formularen (Artikelliste u.¨a.) beschreibt. In Abschnitt 5 ist die Art und Weise der
Steuerung n¨aher beschrieben. (Der Code `0` kennzeichnet dabei das Standardverfahren bei einzeiligen Texten.)


 Im Feld 15 kann eine Textnummer (Zugriffsschl¨ussel) angegeben werden, unter der in der Tabelle
`PropHintText` (s. Abschn. 2.20) ein (mehrzeiliger) Hinweistext zu dem Merkmal hinterlegt ist.
Dieser kann von einer Applikation als Hint [17] angezeigt werden, wenn der Nutzer den Mauszeiger
¨uber den Merkmalsbezeichner im Eigenschaftseditor der Applikation bewegt.
Als grunds¨atzlicher Aufbau wird folgende Struktur empfohlen:


**–** Merkmalsbezeichner (als Sicherheit f¨ur den Fall, daß durch die Spaltenformatierung ein Teil
des Merkmalsbezeichner im Eigenschaftseditor nicht sichtbar ist)

**–** Angaben zu g¨ultigen Wertebereichen

**–** sonstige Verwendungshinweise


Zwischen den Attributen eines Merkmals bestehen _Abh¨angigkeiten_, die die m¨oglichen Kombinationen der
Attribute einschr¨anken. Diese Abh¨angigkeiten werden durch die folgenden Regeln beschrieben:


 Die Kennzeichen _Obligatory_ und _AddValues_ sind nur bei konfigurierbaren Merkmalen (Scope C)
relevant.


 Konfigurierbare numerische Merkmale sind immer Pflichtmerkmale [18] .


 Konfigurierbare einschr¨ankbare Merkmale sind immer Pflichtmerkmale.


 Merkmale zur freien Zeichenketten- bzw. Texteingabe durch den Nutzer sind Pflichtmerkmale, d.h.,
eine leere Zeichenkette bzw. ein leerer Text ist ein realer Wert.


 Merkmale zur freien Texteingabe durch den Nutzer (Typ T) sind implizit konfigurierbar (d.h. Scope
C ist vorausgesetzt). Die Kennzeichen _Restrictable_ und _MultiOption_ sind f¨ur diese Merkmale nicht
relevant. Desweiteren sind nur die Text-Steuercodes `0` und `5` erlaubt (s. Anhang 5).


 Nicht–konfigurierbare Merkmale k¨onnen einschr¨ankbar sein. Im Gegensatz zu konfigurierbaren einschr¨ankbaren Merkmalen m¨ussen diese jedoch nicht (durch das Beziehungswissen) bewertet sein,
damit die Konfiguration des Artikels vollst¨andig ist.


 Bei (konfigurierbaren) einschr¨ankbaren Merkmalen ist die Eingabe zus¨atzlicher Werte nicht m¨oglich.


 Bei konfigurierbaren einschr¨ankbaren Merkmalen muß eine Werteliste vorliegen, oder aber es muß
ein Constraint existieren (und wirksam sein), das eine Bewertung des Merkmals vornimmt. Ansonsten w¨are die Konfiguration niemals vollst¨andig.


 Interval–Werte sind nur bei konfigurierbaren nicht–einschr¨ankbaren numerischen Merkmalen relevant.


 Der Typ L ist nur bei sichtbaren Merkmalen (Scopes C und RV) sinnvoll.


 Merkmale des Typs T k¨onnen nicht mehrwertig sein.


 F¨ur mehrwertige Merkmale sind die Kennzeichen _AddValues_ und _Restrictable_ nicht relevant.


 Interval–Werte sind f¨ur mehrwertige Merkmale nicht m¨oglich.


16z.B. f¨ur die Anzeige in Eigenschaftseditoren und zum Aufbau der Grafik
17tempor¨ares Textfeld an der Stelle des Mauszeigers
18Bei numerischen Merkmalen ist immer eine Eingabe gefordert, da das System ansonsten keine Operationen mit solchen
Merkmalen durchf¨uhren kann.


17


Die Merkmale der verschiedenen Typen und Geltungsbereiche werden wie folgt _initialisiert_ [19] :


 Merkmale des Typs T werden mit einem Leerstring initialisiert.


 Der initiale Zustand von _einschr¨ankbaren_ Merkmalen ist undefiniert (nicht bewertet).


 Nicht-einschr¨ankbare _konfigurierbare_ Merkmale werden mit dem als Default gekennzeichneten Wert
aus der Wertetabelle initialisiert [20] .
Ist kein Wert der Wertetabelle als Default gekennzeichnet, so werden Pflichtmerkmale mit dem ersten Wert aus der Wertetabelle initialisiert, w¨ahrend der initiale Zustand von optionalen Merkmalen
dann undefiniert ist.
Sind gar keine Werte in der Wertetabelle hinterlegt, werden Merkmale des Typs C mit einer leeren
Zeichenkette und Merkmale der Typen N und L mit dem Wert `0` bzw. `0.0` initialisiert.


 Nicht-einschr¨ankbare _nicht-konfigurierbare Merkmale_ werden mit dem ersten Wert aus der Artikelstammdatentabelle initialisiert, nicht jedoch bei Merkmalen der Geltungsbereiche RG und RV,
wenn f¨ur diese in der Wertetabelle Werte hinterlegt sind.
Sind keine Werte in der Artikelstammdatentabelle hinterlegt, aber in der Wertetabelle, ist das Verhalten wie bei konfigurierbaren Merkmalen. (Bei Merkmalen des Geltungsbereichs R werden Werte
aus der Wertetabelle jedoch nur ber¨ucksichtigt, wenn dort genau ein Wert hinterlegt ist oder die
Werte mit Vorbedingungen versehen sind.)
Sind weder in der Artikelstammdatentabelle noch in der Wertetabelle Werte hinterlegt, ist der
initiale Zustand von Merkmalen des Geltungsbereichs R undefiniert, w¨ahrend Merkmale der Geltungsbereiche RG und RV je nach Typ mit einer leeren Zeichenkette oder `0` bzw. `0.0` initialisiert
werden.


**2.10** **Die Merkmal–Identifikationstabelle**


Tabellenname: `PropertyIdentification`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|PropertyClass|X|Char||X|Bezeichner der Merkmalsklasse|
|2.|PropertyName|X|Char||X|Bezeichner des Merkmals|
|3.|IdentKey||Char||X|Schl¨ussel f¨ur Identiﬁkationstabelle|



Anmerkungen:


 Die Tabelle dient zur Angabe von zus¨atzlichen Identifikationsnummern (unterschiedlichen Typs) f¨ur
Merkmale.


 Die zus¨atzlichen Identifikationsnummern werden dabei nicht direkt in dieser Tabelle, sondern indirekt ¨uber den im Feld 3 angegebenen Schl¨ussel in der Tabelle `Identification` (Abschn. 2.22)
abgelegt.


19Unter Initialisierung wird die Wertbelegung eines Merkmals vor der Auswertung von Beziehungswissen verstanden. In
Aktionen und Constraints k¨onnen die Werte dann noch ver¨andert werden.
20Es darf nur ein Wert als Default-Wert gekennzeichnet sein. Sind dennoch mehrere Werte als Default markiert, ist das
Verhalten der Applikation undefiniert.


18


**2.11** **Merkmalsgruppen**


Merkmalsgruppen k¨onnen von OFML-Applikationen zur Gruppierung der Merkmale eines Artikels im
Eigenschaftseditor verwendet werden. Dazu m¨ussen folgende beiden Tabellen angelegt werden:


Tabellenname: `Article2PropGroup`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ArticleID|X|Char||X|Grundartikelnummer|
|2.|Position|X|Int||X|Position der Merkmalsgruppe|
|3.|PropGroupID||Char||X|ID der Merkmalsgruppe|
|4.|TextID||Char|||Textnummer|



Anmerkungen:


 In dieser Tabelle wird ein Artikel einer oder mehreren Merkmalsgruppen zugeordnet.
Enth¨alt die Tabelle f¨ur einen gegebenen Artikel keine Eintr¨age, kann eine OFML-Applikation eine
Gruppierung anhand seiner Merkmalsklassenzuordnung vornehmen.


 Die Position (Feld 2) legt die Reihenfolge der Merkmalsgruppen bei der Anzeige im Eigenschaftseditor fest.
Ist f¨ur einen Artikel nur eine Merkmalsgruppe angegeben und enth¨alt diese alle (aktuell sichtbaren) Merkmale des Artikels, kann die Applikation auf die Anzeige der Gruppe verzichten (flache
Darstellung).


 Die ID der Merkmalsgruppe (Feld 3) dient als Fremdschl¨ussel f¨ur die Tabelle `PropertyGroup` (s.u.).


 Die Text-Nummer (Feld 4) dient als Fremdschl¨ussel f¨ur die Tabelle `PropGroupText` (s. Abschn. 2.20)
zur Hinterlegung einer (sprach-spezif **i** schen) Merkmalsgruppenbezeichnung.
Prinzipiell kann ein und dieselbe Merkmalsgruppe bei verschiedenen Artikeln mit unterschiedlichen
Textnummern und damit unterschiedlichen Bezeichnungen versehen werden. Im Normalfall wird
jedoch dieselbe Textnummer und damit eine einheitliche Bezeichnung verwendet.
Ist f¨ur eine gegebene Sprache keine Bezeichnung f¨ur eine gegebene Gruppe hinterlegt, wird die ID
der Gruppe zur Anzeige im Eigenschaftseditor verwendet.


Tabellenname: `PropertyGroup`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|PropGroupID|X|Char||X|ID der Merkmalsgruppe|
|2.|Position|X|Int||X|Position des Merkmals|
|3.|PropertyClass||Symbol||X|Bezeichner der Merkmalsklasse|
|4.|PropertyName||Symbol||X|Bezeichner des Merkmals|



Anmerkungen:


 In dieser Tabelle werden die Merkmale einer Merkmalsgruppe definiert.


 Die Position (Feld 2) legt die Reihenfolge der Merkmale innerhalb der Merkmalsgruppe bei der
Anzeige im Eigenschaftseditor fest.


 Merkmale (Felder 3 und 4), die ein gegebener Artikel nicht besitzt, werden ignoriert.


 Aktuell sichtbare Merkmale des Artikels, welche nicht in den Merkmalsgruppen des Artikels gelistet sind, erscheinen nach diesen in einer ”k¨unstlichen“ Merkmalsgruppe ”Sonstige“ in undefinierter
Reihenfolge.


19


**2.12** **Die Artikelstammtabelle**


Tabellenname: `ArtBase`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ArticleID|X|Char||X|Artikelnummer|
|2.|PropertyClass|X|Char||X|Name der Merkmalsklasse|
|3.|PropertyName|X|Char||X|Name des Merkmals|
|4.|PropertyValue|X|Char||X|Merkmalswert|



Anmerkungen:


 In der Tabelle k¨onnen artikelspezifische Angaben zu fixen bzw. erlaubten Werten von ausgew¨ahlten
Merkmalen gemacht werden, indem einem Merkmal des Artikels ein oder mehrere Werte (nacheinanderfolgende Datens¨atze) zugewiesen werden.


 Bei Merkmalen, f¨ur die in der Merkmalswerttabelle (s. Abschn. 2.13) Werte hinterlegt sind, bedeutet
dies eine Einschr¨ankung der Wertemenge in Bezug auf die in der Merkmalswerttabelle hinterlegten
diskreten Werte [21] . Dies impliziert, daß dann im Feld 4 nur Werte angegeben werden d¨urfen, die
auch in der Merkmalswerttabelle f¨ur das Merkmal hinterlegt sind.


 Die Wertzuweisungen f¨ur ein Merkmal in der Artikelstammtabelle haben Vorrang vor eventuellen
Vorschlagswerten f¨ur das Merkmal in der Merkmalswerttabelle!


**2.13** **Die Merkmalswerttabelle**


Tabellenname: `PropertyValue`
Pflichttabelle: ja [22]

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|PropertyClass|X|Char||X|Bezeichner der Merkmalsklasse|
|2.|PropertyName|X|Char||X|Bezeichner des Merkmals|
|3.|Position|X|Num||X|Position des Merkmalswertes|
|4.|TextID||Char|||Textnummer|
|5.|RelObjID||Num||X|Beziehungsobjekt-Nummer|
|6.|IsDefault||Bool|1|X|Vorschlagswert?|
|7.|SuppressTxt||Bool|1|X|Text-Unterdr¨uckungskennzeichen|
|8.|OpFrom||Char|2||Operator Von|
|9.|ValueFrom||Char|||Merkmalswert Von|
|10.|OpTo||Char|2||Operator Bis|
|11.|ValueTo||Char|||Merkmalswert Bis|
|12.|Raster||Char|||Schrittweite|
|13.|DateFrom||Date|8||G¨ultig von|
|14.|DateTo||Date|8||G¨ultig bis|



21In der Merkmalswerttabelle hinterlegte Interval-Werte sind von der Einschr¨ankung nicht betroffen.
22Die Tabelle kann entfallen, wenn in der Datenbank keine Konfigurationsdaten, sondern z.B. nur Artikeltexte und -preise
angelegt werden sollen.


20


Anmerkungen:


 In dieser Tabelle werden alle m¨oglichen Werte pro Merkmal aufgelistet.


 Die Werte (Felder 9 und 11) sind String-Darstellungen numerischer Werte oder symbolische (sprachunabh¨angige) Bezeichner. Sprechende (sprachabh¨angige) Bezeichner (zur Verwendung in den Benutzeroberfl¨achen) werden in der Tabelle `PropValueText` abgelegt (s. Abschn. 2.20). Dazu wird im
4. Feld eine Textnummer als Zugriffsschl¨ussel vergeben. Ist keine Textnummer angegeben bzw. kein
Text in einer geforderten Sprache hinterlegt, so wird bei Merkmalen des Typs ’C’ der symbolische
(sprachunabh¨angige) Bezeichner (aus dieser Tabelle) angezeigt, bei Merkmalen der anderen Typen
der entsprechende numerische Wert.


 Innerhalb eines Merkmals darf ein spezifischer Wert nur einmal angegeben werden.


 Die Beziehungsobjekt-Nummer dient als Schl¨ussel f¨ur den Zugriff auf das Beziehungsobjekt in der
Tabelle `RelationObj` (s. Abschn. 2.15), an welches das Beziehungswissen f¨ur den Merkmalswert
gebunden ist. (Wird kein Beziehungsobjekt ben¨otigt, so ist die Nummer `0` anzugeben.)


 Der im Feld 6 ( `IsDefault` ) als Vorschlagswert gekennzeichnete Wert wird bei der Initialisierung des
Merkmals (s. Merkmalstabelle, Abschn. 2.9) verwendet [23] .
Ist keiner der Werte eines Merkmals als Vorschlagswert gekennzeichnet, wird bei Pflichtmerkmalen
der erste Wert als initialer Wert verwendet, bei optionalen Merkmalen der virtuelle Wert ”nicht
ausgew¨ahlt“. F¨ur letzteren ist bei Merkmalen des Typs ’C’ das interne K¨urzel `VOID` vorgesehen und
reserviert. Dieses darf also nicht f¨ur einen realen Wert solcher Merkmale verwendet werden.


 Das Kennzeichen im Feld 7 gibt f¨ur ein konfigurierbares Merkmal an, ob es in der Beschreibung
des Artikels angezeigt werden soll ( `0` bzw. leer) oder nicht ( `1` ), wenn der Wert aktuell durch den
Anwender ausgew¨ahlt ist.


 Der Merkmalswert wird in den Feldern 8 bis 11 hinterlegt, wobei die M¨oglichkeit besteht, f¨ur den
Merkmalswert ein Intervall anzugeben. (Dies kann insbesondere bei Maßmerkmalen Anwendung
finden.)


**–** Ein fester Merkmalswert (ohne Eingabebereich) wird durch den Operator ’EQ’ gekennzeichnet.
Ob dabei die Felder f¨ur den Von-Wert oder die Felder f¨ur den Bis-Wert verwendet werden, ist
egal. Die Felder f¨ur den jeweils nicht benutzten Wert m¨ussen leer sein.


**–** Ein offener Eingabebereich wird durch die Operatoren ’GT’, ’GE’, ’LT’ oder ’LE’ f¨ur den Vonoder den Bis-Wert gekennzeichnet, wobei die Felder f¨ur den jeweils nicht benutzten Wert leer
sein m¨ussen.


**–** Ein geschlossener Eingabebereich wird entsprechend durch die Operatoren ’GT’, ’GE’, ’LT’
oder ’LE’ f¨ur den Von- bzw. den Bis-Wert gekennzeichnet, wobei beide Werte bestimmt sein
m¨ussen.


**–** Folgen dem Intervall–Wert in der Tabelle weitere Einzelwerte f¨ur das Merkmal, werden diese
Werte ebenfalls in die Auswahlliste der f¨ur das Merkmal generierten Property aufgenommen.
Dies kann f¨ur Standard- bzw. Vorschlagswerte innerhalb des Intervalls verwendet werden.


**–** Ist dar¨uberhinaus einer der (dem Intervall–Wert folgenden) Einzelwerte als Default–Wert gekennzeichnet (Feld 6), wird der bisherige Default-Wert ¨uberschrieben. Bei einem geschlossenen
Intervall k¨onnte damit die obere Grenze als Default–Wert gesetzt werden.


**–** Es k¨onnen mehrere Interval–Werte angegeben werden. Mit Hilfe von Vorbedingungen (s. Abschn. 2.15) kann dann gesteuert werden, welche Intervalle in einer bestimmten Konfiguration
tats¨achlich zur Anzeige kommen.


 Im Feld 12 kann bei numerischen Merkmalen eine Schrittweite angegeben werden, die bei der Eingabe innerhalb des durch die Felder 8 bis 11 bestimmten Eingabebereiches eingehalten werden muß.


23Dabei ist zu beachten, daß der Wert nicht mit einer Vorbedingung versehen ist, die in der initialen Konfiguration des
Artikels nicht g¨ultig ist. Ansonsten wird das Merkmal in der initialen Konfiguration letztendlich mit einem anderen Wert
oder gar nicht vorbelegt.


21


 Numerische Werte m¨ussen in den Feldern 9, 11 und 12 gem¨aß der in der Merkmalstabelle (s.
Abschn. 2.9) f¨ur das Merkmal spezifizierten Anzahl von Stellen bzw. Nachkommastellen angegeben
werden. F¨uhrende Nullen bzw. Nullen am Ende des Dezimalteils m¨ussen dabei nicht angegeben
werden. Beispiele:


**–** Format: 4 Stellen, davon 0 Nachkomma; Wert: 1200 _→_ ’1200’

**–** Format: 3 Stellen, davon 1 Nachkomma; Wert: 1.5 _→_ ’1.5’

**–** Format: 4 Stellen, davon 2 Nachkomma; Wert: 20.7 _→_ ’20.70’ oder ’20.7’


 Falls in der OCD–Datenbank mehrere Preislisten abgebildet sind und der Wert bzw. das Intervall
nur in einer Preisliste g¨ultig ist, muß in den Feldern 13 und 14 der G¨ultigkeitszeitraum angegeben
werden [24] .
Sind die Felder leer, ist der Wert unbegrenzt g¨ultig.


**2.14** **Die Merkmalswert–Identifikationstabelle**


Tabellenname: `PropValueIdentification`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|PropertyClass|X|Char||X|Bezeichner der Merkmalsklasse|
|2.|PropertyName|X|Char||X|Bezeichner des Merkmals|
|3.|PropertyValue|X|Char||X|Merkmalswert|
|4.|IdentKey||Char||X|Schl¨ussel f¨ur Identiﬁkationstabelle|



Anmerkungen:


 Die Tabelle dient zur Angabe von zus¨atzlichen Identifikationsnummern (unterschiedlichen Typs) f¨ur
Merkmalswerte.


 Die zus¨atzlichen Identifikationsnummern werden dabei nicht direkt in dieser Tabelle, sondern indirekt ¨uber den im Feld 4 angegebenen Schl¨ussel in der Tabelle `Identification` (Abschn. 2.22)
abgelegt.


 Zus¨atzliche Identifikationsnummern k¨onnen nur f¨ur Werte angegeben werden, die auch in der Merkmalswerttabelle (s.Abschn. 2.13) angelegt sind und die keine Interval-Werte sind [25] .


24Dieser sollte mit dem G¨ultigkeitszeitraum der Preiskomponenten f¨ur die entsprechende Preisliste in der Preistabelle
¨ubereinstimmen, s.a. Abschn. 3).
25Frei durch den Nutzer eingegebene Werte — bei Merkmalen, die dies erlauben — m¨ussen bei einem entsprechenden
Export, z.B. zum Bestelldatenaustausch, direkt angebenen werden.


22


**2.15** **Die Beziehungsobjekt–Tabelle**


Tabellenname: `RelationObj`
Pflichttabelle: ja [26]

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|RelObjID|X|Num||X|Beziehungsobjekt-Nummer (gr¨oßer` 0`)|
|2.|Position|X|Num||X|Position der Beziehung|
|3.|RelName||Char||X|Beziehungsname|
|4.|Type||Char|1|X|Art der Beziehung:<br>1 - Vorbedingung<br>2 - Auswahlbedingung<br>3 - Aktion<br>4 - Constraint<br>5 - Reaktion<br>6 - Post-Reaktion|
|5.|Domain||Char|4|X|Verwendungsgebiet:<br>C - Konﬁguration<br>P - Preisermittlung<br>BOI - St¨uckliste (bill of items)<br>PCKG - Verpackung<br>TAX - Besteuerungschemata|



Anmerkungen:


 In dieser Tabelle werden Beziehungen zu Beziehungsobjekten geb¨undelt.


 Aktuell sind folgende Beziehungsarten m¨oglich:


**–** Mittels _Vorbedingungen_ kann generell die G¨ultigkeit der Entit¨aten festgelegt werden, f¨ur die
die Vorbedingungen angelegt sind. Vorbedingungen sind f¨ur folgende Entit¨aten m¨oglich:


      - Die Vorbedingung eines _Merkmals_ legt fest, ob das Merkmal f¨ur den Anwender sichtbar
ist (Scope RV) bzw. durch den Anwender bewertet werden darf (Scope C).

      - Die Vorbedingung einer _Merkmalsklasse_ legt fest, ob die Merkmale der Klasse f¨ur den
Anwender sichtbar sind (Scope RV) bzw. durch den Anwender bewertet werden d¨urfen
(Scope C).
Vorbedingungen von Merkmalsklassen haben Vorrang vor evtl. vorhandenen Vorbedingungen der Merkmale der Klasse, d.h., die Vorbedingungen der Merkmale der Klasse werden
gar nicht erst ausgewertet, wenn die Klasse aufgrund ihrer Vorbedingung(en) aktuell nicht
g¨ultig ist.

      - Die Vorbedingung eines _Merkmalswertes_ legt fest, ob dieser Wert gesetzt werden darf.
Vorbedingungen von Werten von konfigurierbaren Merkmalen werden ausgewertet, wenn
das Merkmal selber g¨ultig ist.
Vorbedingungen von Werten von nicht–konfigurierbaren Merkmalen werden ausgewertet,
wenn dem Merkmal in der Stammdatentabelle kein Wert zugewiesen ist.

      - Die Vorbedingung einer _St¨ucklistenkomponente_ legt fest, ob diese verwendet werden darf.


Sind f¨ur eine Entit¨at mehrere Vorbedingungen angegeben, so ist die Entit¨at nur dann g¨ultig,
wenn alle Bedingungen erf¨ullt sind. Sind f¨ur eine gegebene Entit¨at keine Vorbedingungen
angegeben, so ist diese Entit¨at generell g¨ultig.


**–** _Auswahlbedingungen_ legen fest, ob ein Merkmal bewertet werden muß. Auswahlbedingungen
werden bei der Konsistenzpr¨ufung w¨ahrend der Bestelllistengenerierung f¨ur aktuell nicht bewertete optionale Merkmale sowie f¨ur Merkmale zur freien Zeichenketteneingabe mit aktuell
zugewiesener leerer Zeichenkette ausgewertet. Ist eine der ausgewerteten Auswahlbedingungen


26Die Tabelle kann entfallen, wenn kein Beziehungswissen ben¨otigt wird.


23


erf¨ullt, erscheint eine entsprechende Fehlermeldung und die Bestelllistengenerierung wird abgebrochen.
Sind f¨ur ein relevantes Merkmal mehrere Auswahlbedingungen angegeben, so muß es bewertet
werden, wenn mindestens eine der Auswahlbedingungen erf¨ullt ist.


**–** _Aktionen_ dienen zur Herleitung von Merkmalswerten oder zur Ausgabe von Nachrichten an
den Anwender.
Aktionen an Artikeln und Merkmalsklassen werden bei jedem Konfigurationsschritt angewendet. Aktionen an Merkmalen werden angewendet, wenn diese nicht durch eine Vorbedingung
ausgeblendet sind [27] . Aktionen an Merkmalswerten werden angewendet, wenn der Wert in der
aktuellen Konfiguration des Artikels gesetzt ist.

**–** _Constraints_ werden zur Uberwachung und Sicherstellung der Konsistenz der Konfiguration von [¨]
Artikeln verwendet. Dabei k¨onnen auch Werte hergeleitet oder Wertemengen eingeschr¨ankt
werden. Constraints m¨ussen an Artikel gebunden sein und werden bei jedem Konfigurationsschritt angewendet.
Nicht jede Sprache, die zur Codierung von Beziehungswissen verwendet werden kann, unterst¨utzt auch Constraints (s.a. Abschnitte 2.16 und 2.23).


**–** _Reaktionen_ dienen wie Aktionen zur Herleitung von Merkmalswerten oder zur Ausgabe von
Nachrichten an den Anwender. Sie werden aber nicht bei jedem Konfigurationsschritt angewendet, sondern nur bei bestimmten Ereignissen. Reaktionen k¨onnen nur an Artikel oder
konfigurierbare Merkmale gebunden werden.
Reaktionen von Artikeln werden einmalig zu Beginn der Artikelinitialisierung vor Auswertung
aller sonstigen Beziehungen angewendet.
Reaktionen von Merkmalen werden dann angewendet, wenn der Wert des Merkmals durch den
Anwender ge¨andert wurde. Die Anwendung erfolgt einmalig _vor_ Auswertung aller sonstigen
Beziehungen.


**–** _Post-Reaktionen_ von Artikeln oder konfigurierbaren Merkmalen dienen demselben Zweck wie
_Reaktionen_, werden im Gegensatz zu diesen aber _nach_ Auswertung aller sonstigen Beziehungen
angewendet.
Da nach Post-Reaktionen keine weiteren Beziehungen ausgewertet werden, d¨urfen in PostReaktionen keine Anderungen an der Konfiguration des Artikels vorgenommen werden, die [¨]
Auswirkungen auf abh¨angige Merkmale haben! Ansonsten kann die Artikelkonfiguration in
einem inkonsistenten Zustand verbleiben.


 Die Nummer in Feld 2 bestimmt die Position der Beziehung in der Auswertungsreihenfolge. Die
Beziehungen eines Beziehungsobjektes mit demselben Typ (Feld 4) und demselben Verwendungsgebiet (Feld 5) werden in aufsteigender Reihenfolge der Positionsnummern ausgewertet, wobei die
Nummernreihenfolge nicht kontinuierlich sein muß.
Beziehungen mit gleicher Positionsnummer werden in undefinierter Reihenfolge ausgewertet.


 Das Verwendungsgebiet (Feld 5) gibt an, in welchem Kontext die Beziehung anzuwenden ist:


**C** Diese Beziehungen werden bei der Definition der Konfigurationsm¨oglichkeiten eines konfigurierbaren Artikels w¨ahrend seiner initialen Erzeugung als auch bei jedem Konfigurationsschritt ausgewertet.


**P** Diese Beziehungen werden bei der Preisermittlung ausgewertet (s.Abschn. 3).


**BOI** Beziehungen dieses Verwendungsgebiets steuern die Sichtbarkeit (Existenz) einer St¨ucklistenkomponente (s.Abschn. 2.7).


**PCKG** Diese Beziehungen werden bei der Ermittlung von Verpackungsdaten ausgewertet
(s.Abschn. 6).


**TAX** Diese Beziehungen werden bei der Ermittlung der Besteuerungsinformationen zu einem
Artikel auf der Basis von Besteuerungsschemata ausgewertet (s.Abschn. 2.25). Sie werden
ben¨otigt, wenn die Zuordnung eines Artikels zu einer Steuerkategorie von einer bestimmten
Konfiguration des Artikels abh¨angig ist [28] .


27gilt also generell f¨ur Merkmale der Scopes R und RG
28Ein Beispiel w¨are die ¨Anderung der Materialkategorie in der Steuerart `ECO_FR` .


24


Preis-, `PCKG`    - und `TAX` –Beziehungen d¨urfen nur vom Typ _Aktion_ sein.


F¨ur Preis-, `PCKG`    - und `TAX` –Beziehungen ist auch noch folgendes zu beachten:
Diese Beziehungen d¨urfen keinen Einfluss auf die aktuelle Konfiguration bzw. auf nachfolgende
Auswertungen von Konfigurationsbeziehungen haben. Deswegen sind in diesen Beziehungen Zuweisungen nur an interne (Hilfs-)Merkmale (Geltungsbereich R) erlaubt [29] .


Die nachfolgende Tabelle gibt einen Uberblick, f¨ur welche Daten–Entit¨aten und in welchen Verwendungs- [¨]
gebieten die einzelnen Beziehungsarten angewendet werden k¨onnen:

|Beziehungsart|Daten-Entit¨at|Col3|Col4|Col5|Col6|Verwendungsgebiet|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|Beziehungsart|Artikel|M.Klasse|Merkmal|M.Wert|BOI-Part|C|P|PCKG|TAX|BOI|
|Vorbedingung||X|X|X|X|X||||X|
|Auswahlbed.|||X|||X|||||
|Aktion|X|X|X|X||X|X|X|X||
|Reaktion|X||X|||X|||||
|Post-Reaktion|X||X|||X|||||
|Constraint|X|||||X|||||



**2.16** **Die Beziehungswissen–Tabelle**


Tabellenname: `Relation`
Pflichttabelle: ja [30]

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|RelationName|X|Char||X|Beziehungsname|
|2.|BlockNr|X|Num||X|Codeblock-Nummer|
|3.|CodeBlock||Char||X|Codeblock|



Anmerkungen:


 In dieser Tabelle wird das ”Wissen“ (die Logik) ¨uber die Beziehungen abgelegt. Dazu wird eine
verschiedene Sprachen verwendet werden, deren Syntax und Semantik im Anhang beschrieben ist.
Welche Sprache verwendet wird, muß in der Versionsinformationstabelle (Abschn. 2.23) angegeben
werden.


 Die zu einer Beziehung geh¨orenden Codebl¨ocke werden vor der Auswertung entsprechend ihrer
Nummer zu einem ganzen Codeblock zusammengesetzt.


29s.a. Anmerkung zum Einfluss des Geltungsbereichs auf Persistenz und Initialisierung von Merkmalen in Abschn. 2.9
30Die Tabelle kann entfallen, wenn kein Beziehungswissen ben¨otigt wird.


25


**2.17** **Die Preistabelle**


Tabellenname: `Price`
Pflichttabelle: ja [31]

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ArticleID|X|Char||X|(Grund)Artikelnummer|
|2.|Variantcondition|X|Char|||Variantenkondition|
|3.|Type|X|Char|1|(X)|Preisart:<br>S - Verkaufspreis (sales)<br>P - Einkaufspreis (purchase)|
|4.|Level|X|Char|1|X|Preisebene:<br>B - Grundpreis (Base)<br>X - Zuschlagspreis (eXtra charge)<br>D - Rabatt (Discount)|
|5.|Rule||Char||(X)|Rechenregel|
|6.|TextID||Char|||Textnummer|
|7.|PriceValue||Num||X|Preis/Betrag|
|8.|FixValue||Bool|1|X|Festbetrag (vs. Prozentangabe) ?|
|9.|Currency|X|Char|3|(X)|W¨ahrung f¨ur Festbetrag|
|10.|DateFrom|X|Date|8|X|G¨ultig von|
|11.|DateTo||Date|8|X|G¨ultig bis|
|12.|ScaleQuantity|X|Num||X|Staﬀelpreis–Mindestmenge|
|13.|RoundingID||Char|||Identiﬁkator f¨ur Rundungsvorschrift|



Anmerkungen:


 In dieser Tabelle werden zu jedem Artikel die Grund- und die Zuschlagspreise als auch m¨ogliche
Rabatte verzeichnet.


 Alle Preise werden als Netto–Preise, also ohne Steuern angegeben! Der Vorgang der Preisermittlung
ist in Abschn. 3 genauer beschrieben.


 Ist f¨ur einen Preisposten eine _Variantenkondition_ angegeben (Feld 2), so wird dieser Preisposten bei
der Preisermittlung nur dann ber¨ucksichtigt, wenn die angegebene Variantenkondition g¨ultig ist.
Die f¨ur eine bestimmte Konfiguration g¨ultigen Variantenkonditionen werden durch Preisbeziehungen
(aus den Tabellen `RelationObj` und `Relation` ) ermittelt.


Variantenkonditionen m¨ussen komplett groß geschrieben sein (ausgenommen das deutsche Eszett
’ _ß_ ’).


 F¨ur jeden Preisposten kann sowohl ein Verkaufspreis als auch ein Einkaufspreis angegeben werden
(Feld 3).


 Bei Eintr¨agen f¨ur Zuschl¨age und Rabatte (Preisebenen ’X’ und ’D’, Feld 4) kann der Joker-Artikel

” [*“ (Feld 1) zur Angabe artikel¨ubergreifender Zuschl¨age bzw. Rabatte verwendet werden. Das Feld]
2 (Variantenkondition) darf dabei nicht leer sein.
Dieser artikel-neutrale Tabelleneintrag wird jedoch nur dann ber¨ucksichtigt, wenn f¨ur den bearbeiteten Artikel kein eigener, spezifischer Eintrag mit der gleichnamigen Variantenkondition existiert.


 Die Rechenregel (Feld 5) modifiziert die Art und Weise der Verwendung des Preispostens bei der
Preisermittlung:


**–** F¨ur Grund- und Zuschlagspreise werden aktuell keine speziellen Rechenregeln unterst¨utzt. Der
im Feld 7 angegebene Betrag wird bei der Preisermittlung immer dem bereits akkumulierten
Preis hinzugef¨ugt. Zuschlagspreise k¨onnen auch als prozentuale Werte angegeben werden. In


31Die Tabelle kann entfallen, wenn keine Preise angelegt werden sollen.


26


diesem Fall ergibt sich der absolute Betrag aus dem betreffenden prozentualen Anteil des
Grundpreises.


**–** Bei Rabatten mit Prozentangabe muss eine Rechenregel angegebenen werden:
Rechenregel ’1’ definiert, dass der Preis in Bezug auf den Grundpreis berechnet wird.
Rechenregel ’2’ gibt an, dass der Preis in Bezug auf den w¨ahrend der Preisermittlung bereits
akkumulierten Preis berechnet wird.


 Uber die Textnummer (Feld 6) kann in der Preistext–Tabelle eine Erkl¨arung zu dem Preiseintrag [¨]
hinterlegt werden, z.B. Grund f¨ur Aufpreis. Ist f¨ur einen Preiseintrag keine Beschreibung hinterlegt,
wird vom Anwendungssystem ggf. eine automatische Beschreibung generiert.


 Im Feld 8 wird spezifiziert, ob der Betrag in Feld 7 einen festen Betrag in der W¨ahrung gem¨aß Feld
9 darstellt ( `1` ) oder eine Prozentangabe ist ( `0` ).


 Bei Zuschl¨agen (Preisebene ’X’) kann der Betrag in Feld 7 auch negativ sein. Dies kann zur Abbildung von Abschl¨agen (Minderpreisen) genutzt werden.


 W¨ahrungen (Feld 9) sind gem¨aß ISO 4217 anzugeben, z. B. EUR, CHF, GPB, USD.


 Feld 12 dient zur Angabe von Staffelpreisen, die ab einer bestimmten Menge von bestellten Artikeln
wirksam werden. In dem Feld 12 wird dazu die Anzahl der Artikel angegeben, ab der der Tabelleneintrag verwendet werden kann. Standardm¨aßig ist hier `1` einzutragen (kein Staffelpreis).
Achtung: Staffelpreise werden aktuell immer nur pro Bestell- bzw. Auftragsposition berechnet, nicht
¨uber die gesamte Bestellung hinweg.


 Im Feld 13 kann auf eine Rundungsvorschrift in der Tabelle `Rounding` (s. n¨achster Abschn.) verwiesen werden. Diese wird dann anstelle der im Abschn. 3 beschriebenen Standard-Rundung verwendet,
bevor der absolute Betrag der Preiskomponente zu dem (aktuellen) Gesamtpreis dazu addiert wird.


**2.18** **Die Rundungsvorschrift–Tabelle**


Tabellenname: `Rounding`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ID|X|Char||X|Identiﬁkator der Rundungsvorschrift|
|2.|Number|X|Num||X|laufende Nummer|
|3.|Minimum||Char|||kleinster Betrag, der nach dieser<br>Methode gerundet werden soll|
|4.|Maximum||Char|||Betrag, ab dem nicht mehr nach<br>dieser Methode gerundet werden soll|
|5.|Type||Char||X|Rundungsmethode (s.u.)|
|6.|Precision||Num||X|Genauigkeit der Rundung|
|7.|AddBefore||Num||X|Betrag, der vor der Rundung<br>aufgeschlagen werden soll|
|8.|AddAfter||Num||X|Betrag, der nach der Rundung<br>aufgeschlagen werden soll|



Anmerkungen:


 Unter einer Rundungsvorschrift–ID k¨onnen mehrere Eintr¨age angelegt werden. Diese werden in der
durch die laufende Nummer im Feld 2 bestimmten Reihenfolge abgearbeitet.


 Die Felder 3 (Minimum) und 4 (Maximum) bestimmen den Betragsbereich, f¨ur den die Rundung
nach der im Feld 5 angegebenen Methode erfolgen soll, d.h. Betr¨age ausserhalb des durch die Felder
3 und 4 bestimmten Bereichs werden nicht gerundet. Der im Feld 4 angegebene maximale Betrag


27


fließt dabei nicht mehr in den Bereich ein [32] .
Die Felder sind als optionale Zeichenkettenfelder deklariert, sie k¨onnen also auch leer sein. Wenn
nicht leer, enthalten die Felder String-Darstellungen numerischer Werte [33] .
Ein leeres Feld 3 bedeutet einen nach unten offenen Bereich, ein leeres Feld 4 einen nach oben
offenen Bereich.


 Da unter einer ID mehrere Eintr¨age angelegt werden k¨onnen, ist es m¨oglich, mehrere Betragsbereiche abzubilden. Werden dabei f¨ur dieselbe Rundungsvorschrift–ID mehrere sich ¨uberschneidende
Betragsbereiche angegeben, finden u.U. mehrere Rundungen statt (wenn die zu rundenden Betr¨age
in dem sich ¨uberschneidenden Bereich liegen). Dann ist die durch Feld 2 festgelegte Abarbeitungsreihenfolge entscheidend, da das Ergebnis einer Rundung jeweils den Eingangsbetrag f¨ur die darauffolgende Rundung bildet.


 Die erlaubten Rundungsmethoden sind:


**DOWN** Abrunden


**UP** Aufrunden


**COM** kaufm¨annisches Runden (X.5 rundet auf)


**ECOM** erweitertes (unverzerrtes) kaufm¨annisches Runden (X.5 rundet auf gerade Zahlen ab oder
auf)


 Die Genauigkeit in Feld 6 bestimmt den Betrag, durch den der gerundete Betrag ohne Rest teilbar sein muß. `0.01` z.B. rundet auf die zweite Stelle nach dem Dezimalpunkt (und entspricht der
Genauigkeit bei der Standard–Rundung).


 Die Betr¨age in den Feldern 7 und 8 k¨onnen auch negativ sein.


Beispiel:


Angenommen, eine Preiskomponente soll abh¨angig vom nominalen Betrag wie folgt gerundet werden:


 Betrag _<_ 10 EUR, auf 0.1 EUR kaufm¨annisch runden

 Betrag _>_ = 10 EUR bis _<_ 100 EUR auf 0.5 EUR kaufm¨annisch runden

 Betrag _>_ = 100 EUR auf 1.0 EUR aufrunden und 1 Cent abziehen.


Die Tabelle m¨ußte dann folgende Eintr¨age f¨ur die Rundungsvorschrift enthalten:

|ID|Nr.|Minimum|Maximum|Type|Precision|AddBefore|AddAfter|
|---|---|---|---|---|---|---|---|
|R1|1|0.0|10.0|COM|0.1|0.0|0.0|
|R1|2|10.0|100.0|COM|0.5|0.0|0.0|
|R1|3|100.0||UP|1.0|0.0|-0.01|



**2.19** **Die Serientabelle**


Tabellenname: `Series`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|SeriesID|X|Char||X|Serienk¨urzel|
|2.|TextID||Char||X|Textnummer|
|3.|CatalogFormat||Char|||Format der Katalogdaten|
|4.|CatalogDir||Char|||Verzeichnis der Katalogdaten|



32d.h., es wird der Vergleichsoperator `LT` ( _kleiner als_ ) angewendet
33¨aquivalent zu den Werten numerischer Merkmale in der Wertetabelle


28


Anmerkungen:


 Diese Tabelle unterst¨utzt Applikationen, die nicht das Datenregistrierungssystem _DSR_ der Firma
EasternGraphics verwenden, bei der Registrierung von kaufm¨annischen Serien.


 Die Textnummer im Feld 2 dient als Schl¨ussel f¨ur die Beschreibungstabelle `SeriesText` (s. Abschn. 2.20). Der sprachspezifische Text zu einer Serie enth¨alt in der ersten Zeile die Serienbezeichnung. Optional k¨onnen weitere Zeilen mit zus¨atzlichen Erkl¨arungen folgen.


 Im Feld 4 kann ein Verzeichnis angegeben werden, daß die Daten enth¨alt, die durch die Benutzeroberfl¨ache zur Artikelauswahl verwendet werden k¨onnen. Das dabei verwendete Datenformat
muss dann im Feld 3 spezifiziert werden. Die Formatangabe wird gebildet aus der Kurzbezeichnung des Formats (s.u.), gefolgt von einem Bindestrich (’ `-` ’) und der Versionsangabe, die sich aus
Hauptnummer, Punkt (’ `.` ’) und Unternummer zusammensetzt.


 Folgende Formate k¨onnen im Feld 3 angegeben werden:

|Kurzbezeichnung|Erkl¨arung|
|---|---|
|OAS<br>XCF|OFML Article Selection (OFML Part V)<br>eXtensible Catalog Format (Firma EasternGraphics)|



**2.20** **Die Beschreibungstabellen**


Alle Texttabellen


Artikelkurzbeschreibungen: `ArtShortText` (Pflichttabelle)
Artikellangbeschreibungen: `ArtLongText`
Merkmalsklassenbezeichnungen: `PropClassText`
Merkmalsbezeichnungen: `PropertyText` (Pflichttabelle)
Hinweise zu Merkmalen: `PropHintText`
Merkmalsgruppenbezeichnungen: `PropGroupText`
Merkmalswertbezeichnungen: `PropValueText`
Preistexte (Erkl¨arungen zu Preiskomponenten): `PriceText`
Serienbezeichnungen: `SeriesText`
Zusatztexte f¨ur St¨ucklistenpositionen: `BillOfItemsText`
Nachrichten f¨ur den Anwender: `UserMessage`
Bezeichnungen f¨ur Klassifikationen: `ClassificationText`


besitzen den gleichen Aufbau:

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|TextID|X|Char||X|Textnummer|
|2.|Language|X|Char|2|X|Sprache|
|3.|LineNr|X|Num||X|Zeilennummer|
|4.|LineFormat||Char||X|Zeilenformatsteuerung|
|5.|Textline||Char||X|Textzeile|



Anmerkungen:


 Der Zugriff erfolgt ¨uber Textnummern, die in den jeweiligen Tabellen vergeben werden.

 Die Sprache ist gem¨aß ISO 639-1 anzugeben, z.B. ’de’ (Deutsch), ’en’ (Englisch), ’fr’ (Franz¨osisch) [34] .


34Im Sinne einer Durchg¨angigkeit ¨uber alle OFML–Daten hinweg wurde hier die Entscheidung f¨ur den 2-stelligen Sprachcode getroffen. Beim Export in ein Format, das eine 3-stellige Kodierung nach ISO 639-2 verwendet, z.B. BMEcat, ist die
jeweilige Applikation daf¨ur verantwortlich, eine entsprechende Umwandlung durchzuf¨uhren.


29


 Ein Text besteht aus ein oder mehreren Zeilen. Mehrere Zeilen sind jedoch nur bei den Tabellen
`ArtLongText`, `PropHintText`, `UserMessage` sowie `PropValueText` erlaubt. (Bei anderen Texttypen
werden zus¨atzliche Zeilen nicht verarbeitet.)
Bei der Tabelle `PropValueText` erfolgt die Verarbeitung der Zeilen im Zusammenspiel mit dem Feld
_TxtControl_ der Merkmalstabelle (s. Abschn 2.9).


 Bei der Gestaltung der Texte sollten die Empfehlungen des OFML–Normungsausschusses zur einheitlichen Gestaltung kundenorientierter Artikelbeschreibungen ber¨ucksichtigt werden.


 Das Feld 4 enth¨alt Formatierungshinweise f¨ur Formularlayout–Module eines OFML–basierten Vertriebssystems. Das Erscheinungsbild des Textes kann damit in einem gewissen Rahmen beeinflusst
werden.


Folgende Formatierungscodes sind m¨oglich:


_\_ (Zeilenvorschub)


Die Textzeile wird in einer neuen Zeile ausgegeben. Dies ist der Standard.
Bei einem Merkmalstext-Steuercode `0` wird dieser Code in der ersten Zeile des Merkmalswert–
Textes ignoriert und stattdessen der Code `~` (s.u.) verwendet.


**˜** (Fließtext)


Die Textzeile wird als Fließtext an den vorherigen Text geh¨angt. Beginnt die Textzeile selber
nicht mit einem Leerzeichen, wird dieses vom Formularlayout–Modul eingef¨ugt.


**ˆ** (Bedingter Fließtext)


Findet bei Textbausteinen Anwendung, die sich aus verschiedenen Textarten zusammensetzen,
z.B. Merkmalsbezeichner plus Wertbezeichner bei Merkmalstext-Steuercode `0` : Passt die Textzeile (inkl. der ggf. einzuf¨ugenden Trennzeichen) noch in den zusammengesetzten Textbaustein,
ohne die vom Formularlayout–Modul vorgegebene Zeilenbreite zu ¨uberschreiten, wird sie als
Fließtext an den vorherigen Text geh¨angt. Anderenfalls erfolgt ein Zeilenvorschub zu Beginn
der Textzeile.


**2.21** **Wertkombinationstabellen**


Verschiedene Sprachen zur Codierung von Beziehungswissen (s. Abschn. 2.16) erlauben die Verwendung
von Wertkombinationstabellen. Wertkombinationstabellen werden in Beziehungswissen dazu genutzt, die
Konsistenz einer Wertkombination zu pr¨ufen, Werte herzuleiten oder den Wertebereich eines Merkmals
einzuschr¨anken.


In einer Wertkombinationstabelle werden alle m¨oglichen Wertkombinationen ¨uber eine definierte Menge
von Merkmalen hinweg angegeben.


Bsp.:

|Col1|AUSF GRUPPE|FARBE KORPUS|
|---|---|---|
|1<br>2<br>3<br>4|A<br>A<br>B<br>B|F001<br>F002<br>F002<br>F003|



Der Dateiname einer Wertkombinationstabelle wird aus dem Bezeichner, unter dem die Tabelle in Beziehungswissen angesprochen wird, dem Nachsatz ” `_tbl` “ und dem Suffix ” `.csv` “ gebildet, wobei der
Tabellenname komplett klein geschrieben wird.


Bsp.:


Tabellenbezeichner: `FARBEN_KORPUS`
Dateiname: `farben_korpus_tbl.csv`


30


Die Tabellendefinition ist f¨ur alle OCD–Wertkombinationstabellen gleich:

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|LineNr|X|Num||X|Nummer der Tabellenzeile|
|2.|PropertyName|X|Char||X|Name des Merkmals|
|3.|Value|X|Char||X|Merkmalswert|



Die Felder einer Zeile der logischen Wertkombinationstabelle (s. Beispiel oben) werden also ¨uber ihre
(imagin¨are) Zeilennummer zusammengefasst [35] .


Sowohl Merkmalsnamen als auch Merkmalswerte m¨ussen komplett groß geschrieben sein (ausgenommen
das deutsche Eszett ’ _ß_ ’).


Bsp.:
Die (logische) Wertkombinationstabelle aus dem Beispiel oben w¨urde wie folgt in einer OCD–
Wertkombinationstabelle abgebildet werden:

```
   1;AUSF_GRUPPE;A
   1;FARBE_KORPUS;F001
   2;AUSF_GRUPPE;A
   2;FARBE_KORPUS;F002
   3;AUSF_GRUPPE;B
   3;FARBE_KORPUS;F002
   4;AUSF_GRUPPE;B
   4;FARBE_KORPUS;F003

```

Enth¨alt die Wertkombinationstabelle ein einzelnes Merkmal, das sich auf ein einschr¨ankbares Produktmerkmal bezieht (s. Abschn. 2.9), so kann in dem Feld einer Tabellenzeile f¨ur dieses Merkmal auch eine
Wertemenge angegeben werden.


Bsp.:
Angenommen, das Merkmal `FARBEN_KORPUS` aus dem obigen Beispiel ist einschr¨ankbar, so k¨onnten
logische Wertkombinationstabelle und entsprechende OCD–Wertkombinationstabelle so aussehen:

|Col1|AUSF GRUPPE|FARBE KORPUS|
|---|---|---|
|1<br>2|A<br>B|F001, F002<br>F002, F003|


```
   1;AUSF_GRUPPE;A
   1;FARBE_KORPUS;F001
   1;FARBE_KORPUS;F002
   2;AUSF_GRUPPE;B
   2;FARBE_KORPUS;F002
   2;FARBE_KORPUS;F003

```

**2.22** **Die Identifikationstabelle**


Tabellenname: `Identification`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|EntityID|X|Char||X|ID der Entit¨at (Artikel, Merkmal, Wert)|
|2.|Type|X|Char||X|Typ der Identiﬁkationsnummer|
|3.|IdentNr||Char||X|Identiﬁkationsnummer|



35Damit muß nicht f¨ur jede logische Wertkombinationstabelle eine eigene Tabellendefinition bereitgestellt werden.


31


Anmerkungen:


 Die Tabelle dient zur Angabe von zus¨atzlichen Identifikationsnummern f¨ur Artikel, Merkmale und
Merkmalswerte.


 Die Art und Weise bzw. der Kontext der Verwendung einer Identifikationsnummer wird durch ihren
Typ (Feld 2) bestimmt. Aktuell sind folgende Typen erlaubt (s.a. Begriffe im Anhang) [36] :

|Typ|Erkl¨arung|
|---|---|
|CustomID|H¨andler- bzw. großkundenspeziﬁsche Artikelnummer|
|EAN.UCC-8|achtstellige ID nach EAN.UCC|
|EAN.UCC-13|dreizehnstellige ID nach EAN.UCC|
|EAN.UCC-14|vierzehnstellige ID nach EAN.UCC|
|GLN|Globale Lokationsnummer|
|Intrastat|Intrastat–Nummer|
|CustomsTarif|Zolltarifnummer|



Der Typ _CustomID_ wird benutzt, wenn der Datenbestand f¨ur einen Fachh¨andler bzw. Großkunden bestimmt ist, der vom Hersteller abweichende Artikelnummern verwendet. Von der OFML–
Anwendung ist dann ggf. diese kundenspezifische Artikelnummer anzuzeigen/zu verwenden.


**2.23** **Die Versionsinformationstabelle**


Tabellenname: `Version`
Pflichttabelle: ja

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|FormatVersion||Char||X|Nummer der verwendeten<br>OCD–Formatversion|
|2.|RelCoding||Char||X|verwendete Sprache f¨ur<br>Beziehungswissen|
|3.|DataVersion||Char||X|Datenbank-Version|
|4.|DateFrom||Date|8|X|Verwendbarkeitsdatum von|
|5.|DateTo||Date|8|X|Verwendbarkeitsdatum bis|
|6.|Region||Char||X|Vertriebsgebiet|
|7.|VarCondVar||Char|||Variable f¨ur<br>Variantenkonditionen|
|8.|PlaceHolderOn||Bool||X|Platzhalter in IN–Vergleichen?|
|9.|Tables||Char||X|enthaltene Tabellen|
|10.|Comment||Char|||freie Kommentare,<br>Zusatzinformationen|



Anmerkungen:


 Die Tabelle dient zur Angabe von Informationen ¨uber das verwendete Format und ¨uber die Produktdatenbank. Damit k¨onnen durch ein Versionskontrollsystem o.¨a. Aussagen ¨uber den Aufbau
und die Verwendbarkeit der Datenbank getroffen werden.
Die Tabelle darf nur einen Eintrag enthalten.


 Die OCD–Formatversion (Feld 1) ist in der Form `MajorNumber.MinorNumber` gem¨aß OCD–
Formatspezifikation anzugeben.


36Nicht alle der ID-Typen sind auch bei jedem Entit¨atstyp anwendbar.


32


 Im Feld 2 muß die Sprache spezifiziert werden, die zur Codierung von Beziehungswissen verwendet
wird (s. Abschn. 2.16). Folgende Sprachen k¨onnen verwendet werden: `OCD_1`, `OCD_2`, `OCD_3`, `OCD_4`,
`SAP_LOVC` . Die Beschreibung der Sprachen ist im Anhang enthalten.


 Die Datenbank-Version (Feld 3) ist in der Form `MajorNumber.MinorNumber.BuildNumber` anzugeben. Die Nummern k¨onnen vom Hersteller frei vergeben werden, sind dabei aber streng monoton
wachsend zu vergeben.


 Der Bezeichner f¨ur das Vertriebsgebiet [37] (Feld 6) kann frei vergeben werden. Er muss jedoch mit
dem Bezeichner korrespondieren, mit dem im jeweiligen Software-System weitere ben¨otigte Daten
(Geometrie, Katalog) zu dem Vertriebsgebiet referenziert werden.


 Im Feld 7 kann die Variable spezifiziert werden, die in Preisbeziehungen anstelle von `$VARCOND`
(OCD-Sprachsets) bzw. von `$self.variant_condition` (SAP-Sprachsets) zur Zuweisung von Variantenkonditionen verwendet wird (s.a. 3).
F¨ur den Bezeichner der Variablen sind alle alphanumerischen Zeichen inklusive dem Unterstrich
erlaubt, wobei das erste Zeichen kein numerisches sein darf. Bei der Verwendung der Variablen
innerhalb von Beziehungen in der Tabelle `Relation` muß dem Bezeichner das Dollar–Zeichen (’ `$` ’)
vorangestellt werden.


 Im Feld 8 wird angegeben, ob im Beziehungswissen in IN–Vergleichen Platzhalterzeichen in Zeichenkettenkonstanten ersetzt werden sollen (s. Anhang D).


 Im Feld 9 werden, durch ein Komma separiert, die aktuell in der Datenbank enthaltenen Tabellen
aufgelistet. Dies betrifft auch die Pflichttabellen, nicht aber Wertkombinationstabellen. Dabei sind
die Tabellennamen gem¨aß der Spezifikation der verwendeten OCD–Formatversion (Feld 1) anzugeben [38] .
Zus¨atzliche Leerzeichen nach den Kommata sind erlaubt.


**2.24** **Die Nummernschema–Tabelle**


Tabellenname: `CodeScheme`
Pflichttabelle: nein


Anmerkungen:


 Die Tabelle dient zur Angabe von Kodierungsschemata und Parametern f¨ur die Generierung von
Endartikelnummern.


 Endartikelnummern werden konzeptionell aus der Grundartikelnummer und dem sogenannten _Va-_
_riantencode_ zusammengesetzt, wobei die konkrete Stellung von Grundartikelnummer und Variantencode in der Endartikelnummer durch die einzelnen Kodierungsschemata bestimmt wird. Im Variantencode werden die aktuellen Auspr¨agungen der konfigurierbaren Merkmale kodiert.


 Das f¨ur einen Artikel zu verwendende Kodierungsschema wird in der Artikeltabelle (Abschn. 2.2)
anhand des Schema-Identifikators bestimmt [39] .


 Es wird zwischen _vordefinierten_ und _nutzerdefinierten_ Kodierungsschemata unterschieden. Die jeweiligen Kodierungsvorg¨ange sind im Abschn. 4 genau beschrieben.


 Die Felder 3 bis 10 dienen der Parametrisierung der Kodierungsschemata wie im Abschn. 4 beschrieben.


 Die Zeichenkette in Feld 3 darf (bei vordefinierten Schemata) nicht leer sein. Ist das Feld 3 dennoch
leer, wird eine Zeichenkette bestehend aus einem einzelnen Leerzeichen verwendet.


37Großkunden mit spezifischen Preislisten oder abweichenden Konfigurationsdaten werden in diesem Kontext ebenfalls
durch das Konzept des _Vertriebsgebiets_ abgebildet.
38d.h., ohne Pr¨afix `ocd_` und ohne Suffix `.csv` .
39Damit kann prinzipiell f¨ur jeden Artikel ein eigenes Kodierungsschema definiert werden.


33


|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|SchemeID|X|Char||X|eindeutiger Identiﬁkator zur<br>Referenzierung des Schemas|
|2.|Scheme||Char|||Beschreibung des Schemas|
|3.|VarCodeSep||Char|||Zeichenkette zum Trennen von Grund-<br>artikelnummer und Variantencode<br>(nur f¨ur vordeﬁnierte Schemata)|
|4.|ValueSep||Char|||Zeichenkette zum Trennen von<br>Merkmalswerten<br>(nur f¨ur vordeﬁnierte Schemata)|
|5.|Visibility||Char|1||Sichtbarkeitsmodus – gibt an, welche<br>Merkmale im Variantencode enthalten<br>sein sollen<br>`0` – nur die aktuell g¨ultigen und<br>sichtbaren Merkmale<br>`1` – alle konﬁgurierbaren Merkmale|
|6.|InVisibleChar||Char|1||Ersetzungszeichen f¨ur aktuell ung¨ultige<br>bzw. nicht sichtbare Merkmale:<br>Es werden so viele Zeichen dargestellt,<br>wie im L¨angenfeld der Property–Tabelle<br>f¨ur das Merkmal angegeben.<br>Ist das Feld leer, wird ’`-`’ verwendet.|
|7.|UnselectChar||Char|1||Ersetzungszeichen f¨ur aktuell nicht<br>bewertete/ausgew¨ahlte optionale bzw.<br>einschr¨ankbare Merkmale:<br>Es werden so viele Zeichen dargestellt,<br>wie im L¨angenfeld der Property–Tabelle<br>f¨ur das Merkmal angegeben.<br>Ist das Feld leer, wird ’`X`’ verwendet.|
|8.|Trim||Bool|1|X|Trimm–Kennzeichen – gibt an, ob<br>die einzelnen Merkmalswerte exakt<br>gem¨aß der Angabe im L¨angenfeld<br>der Property–Tabelle dargestellt<br>werden sollen (`0`), oder ob nicht<br>belegte Stellen (Leerzeichen) am<br>Ende entfernt werden k¨onnen (`1`).|
|9.|M~~O S~~ep||Char|||Zeichenkette zum Trennen der Werte<br>von mehrwertigen Merkmalen|
|10.|M~~O B~~racket||Char|2*N||Zeichen f¨ur Klammersetzung<br>bei mehrwertigen Merkmalen|


 Damit eine gegebene Artikelkonfiguration anhand ihrer Endartikelnummer eindeutig rekonstruiert
werden kann, darf in den Feldern 6 und 7 kein Ersetzungszeichen verwendet werden, das zu einem
regul¨aren Wert von optionalen oder einschr¨ankbaren Merkmalen f¨uhren w¨urde.


 Wenn das Trimm–Kennzeichen in Feld 8 gesetzt ist, m¨ussen die Inhalte in den Feldern 6 und 7 von
dem Inhalt in Feld 4 verschieden sein und Feld 4 darf nicht leer sein.


 Das Trimm–Kennzeichen darf nicht gesetzt sein, wenn der Artikel (codierte) Merkmale besitzt,
f¨ur die der Anwender Werte frei eingeben kann. Anderenfalls kann die korrekte Verarbeitung der
Endartikelnummer nicht in allen F¨allen sichergestellt werden.


34


**2.25** **Besteuerungsschemata**


Zu Artikeln werden in den OCD–Daten keine konkreten Steuers¨atze hinterlegt, sondern jeder Artikel
wird einem sogenannten Besteuerungsschema zugeordnet, welches pro Land (des im OCD–Datenstand
abgebildeten Vertriebsgebietes) f¨ur jede relevante Steuerart die jeweils zu verwendende (abstrakte) Steuerkategorie beschreibt. Die aktuellen gesetzlichen Steuers¨atze f¨ur jede Steuerkategorie werden dann in
den OFML–Anwendungen selber verwaltet und bei der Belegerstellung herangezogen.


Tabellenname: `ArticleTaxes`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|ArticleID|X|Char||X|(Grund)Artikelnummer|
|2.|TaxID||Char||X|ID des Besteuerungsschemas|
|3.|DateFrom|X|Date|8||G¨ultig von (inklusive)|
|4.|DateTo||Date|8||G¨ultig bis (inklusive)|



Anmerkungen:


 Uber diese Tabelle erfolgt die Zuordnung eines Artikels zu einem Besteuerungsschema. [¨]


 Ist f¨ur einen Artikel keine Zuordnung zu einem Besteuerungsschema hinterlegt (z.B. auch, wenn die
in diesem Abschnitt beschriebenen Tabellen gar nicht existieren), so wird ein im Anwendungssystem
festgelegtes bzw. eingestelltes Besteuerungsschema verwendet.


 Uber die Felder 3 und 4 k¨onnen Stichtage abgebildet werden, an denen gesetzliche [¨] Anderungen [¨]
hinsichtlich der Zuordnung von Artikeln zu Steuerkategorien wirksam werden. Sobald es mehrere
Eintr¨age zu einem Artikel in der Tabelle gibt, d¨urfen diese beiden Felder nicht leer sein (und m¨ussen
sich nicht ¨uberlappende Zeitr¨aume spezifizieren).


Tabellenname: `TaxScheme`
Pflichttabelle: nein

|Nr.|Name|Key|Typ|L¨ange|Pflicht|Erkl¨arung|
|---|---|---|---|---|---|---|
|1.|TaxID|X|Char||X|ID des Besteuerungsschemas|
|2.|Country|X|Char||X|L¨andercode (ISO 3166-1)|
|3.|Region|X|Char|||Regionscode (ISO 3166-2)|
|4.|Number|X|Num||X|Nummer in der Anwendungsreihenfolge|
|5.|TaxType||Char||X|Bezeichner der Steuerart|
|6.|TaxCategory||Char||X|Bezeichner der Steuerkategorie|



Anmerkungen:


 Diese Tabelle definiert L¨ander–spezifisch und ggf. auch Regionen–spezifisch die Besteuerungsschemata, die f¨ur Artikel des OCD–Datenstandes relevant sind.


 Das Feld 3 ( _Region_ ) kann zur Abbildung spezifischer Regelungen f¨ur Regionen innerhalb eines
Landes genutzt werden. Die Eintr¨age f¨ur ein gegebenes Besteuerungsschema und ein gegebenes
Land, in denen das Feld 3 leer ist, beschreiben dabei die Regionen–¨ubergreifende Normalregelung
f¨ur das Land.
Als Code ist in diesem Feld nicht der vollst¨andige Code gem¨aß ISO 3166-2 anzugeben, sondern
nur der regionsspezifische Teil nach L¨andercode und Trennzeichen (z.B. also ”TH“ f¨ur Th¨uringen
anstatt von ”DE-TH“).


35


 Die Nummer der Anwendungsreihenfolge (Feld 4) ist relevant f¨ur die F¨alle, in denen mehrere Steuerarten f¨ur den Artikel erhoben werden. Die Nummer gibt dann die Reihenfolge vor, in der die
jeweiligen Steuers¨atze angewendet werden. Die letztliche Entscheidungsgewalt bei der Anwendungsreihenfolge hat jedoch die jeweilige OFML–Anwendung, da sie ggf. gesetzlichen Regelungen den
Vorrang geben muss.


 Die Bezeichner f¨ur Steuerart und Steuerkategorie (Felder 5 und 6) k¨onnen im Prinzip frei vergeben werden. Dies ist jedoch nur f¨ur spezifische OFML–Anwendungen sinnvoll. Um eine einheitliche
Verwendung ¨uber unterschiedliche OFML-Anwendungen hinweg zu gew¨ahrleisten, werden die Bezeichner f¨ur die gew¨ohnlichen Steuerarten und Steuerkategorien standardisiert. Dies erfolgt in einem
separaten Anhang zur OCD–Spezifikation (s. Anh. H), der bei Bedarf nach Antrag und Abstimmung
durch das OFML–Normungsgremium dynamisch erweitert werden kann.


 Ist ein Artikel keinem Besteuerungsschema zugeordnet oder enth¨alt das Schema weder Angaben zu
einer angeforderten Region noch zu einem angeforderten Land, so wird von der OFML–Anwendung
in der Steuerart _Mehrwertsteuer_ die Standard–Kategorie (mit dem daf¨ur in dem Land geltenden
Steuersatz) angewendet.


 Mit Hilfe der Funktion `SET_TAX_CATEGORY` (s. Anh. D) kann in `TAX` –Beziehungen einem Artikel in einer gegebenen Steuerart, abh¨angig von bestimmten Artikelvarianten, eine vom Besteuerungsschema
abweichende Steuerkategorie zugeordnet werden.


36


## **3 Die Preisermittlung**

In diesem Abschnitt wird beschrieben, wie mit Hilfe der Preistabelle und Beziehungswissen der Preis zu
einem Artikel in einer konkreten Konfiguration ermittelt wird.


**3.1** **¨Uberblick**


Der Gesamtpreis (Endpreis) eines Artikels in einer Preisart (Verkaufs- vs. Einkaufspreis) setzt sich konzeptionell aus _Preiskomponenten_ verschiedener Ebenen zusammen. Es wird zwischen nicht-konditionierten
und konditionierten Preiskomponenten unterschieden. _Nicht-konditionierte_ Preiskomponenten gelten f¨ur
den Artikel unabh¨angig von seiner Konfiguration. _Konditionierte_ Preiskomponenten hingegen gelten nur
bedingt f¨ur bestimmte Konfigurationen des Artikel. Die Bedingung wird mit Hilfe einer _Variantenkondi-_
_tion_ (Feld 2 in der Preistabelle) formuliert, welche in Preisbeziehungen entsprechend hergeleitet werden
muß (s. Abschn. 3.2).


Das im weiteren beschriebene Verfahren gilt gleichermaßen f¨ur beide Preisarten.


Zu einer gegebenen Preiskomponente k¨onnen in einer Preisebene mehrere Eintr¨age vorliegen, die sich
durch ihre G¨ultigkeitszeitr¨aume, W¨ahrungen und/oder sonstige Attribute unterscheiden. Aus diesen Tabelleneintr¨agen wird anhand der Parameter der Preisermittlung der passendste Eintrag ermittelt (s. Abschn. 3.3).


Die relevanten Preiskomponenten der verschiedenen Preisebenen (Feld 4) werden in folgender Reihenfolge
ermittelt:


1. Grundpreise (Ebene ’B’)


2. Zuschlagspreise (Ebene ’X’)


3. Rabatte (Ebene ’D’)


Bei jeder ermittelten (g¨ultigen) Preiskomponente wird der Gesamtpreis f¨ur den Artikel entsprechend der
Rechenregel (Feld 5 in der Preistabelle) akkumuliert. Dabei erfolgt ggf. eine Rundung des ermittelten
absoluten Betrages auf 2 Stellen nach dem Dezimalpunkt nach der Methode des kaufm¨annischen Rundens (X.5 rundet auf), insofern f¨ur die Preiskomponente keine eigene Rundungsvorschrift hinterlegt ist
(Feld 13).


Abschließend werden die Steuern f¨ur den Artikel ermittelt. Dabei werden die Angaben aus den hinterlegten
Besteuerungsschemata verwendet (s. Abschn. 2.25). Die genaue Art und Weise der Steuerbetragsermittlung (auftrags¨ubergreifend oder positionsweise) und die Art der Ausweisung im Formular sind abh¨angig
von der jeweiligen OFML–Applikation.


**3.2** **Relevante Tabelleneintr¨age**


Innerhalb einer Preisebene werden die relevanten Preiskomponenten wie folgt ermittelt:


1. Ermitteln der Preiskomponenten f¨ur den Artikel ohne Variantenkondition.


2. Auswerten aller relevanten Preisbeziehungen und Ermitteln der Preiskomponenten f¨ur den Artikel mit den Variantenkonditionen, die in diesen Preisbeziehungen hergeleitet wurden. Preisbeziehungen sind in der Tabelle `RelationObj` (Feld 4) durch die Verwendungsart ’P’ gekennzeichnet
und m¨ussen vom Typ Aktion (’3’) sein (Feld 3). Variantenkonditionen werden in Preisbeziehungen (Tabelle `Relation` ) durch Zuweisung der Bezeichnung der Variantenkondition an die spezielle
Variable `$VARCOND` (OCD-Sprachsets) bzw. an das Hilfsmerkmal `$self.variant_condition` (SAPSprachsets) hergeleitet [40] . Sowohl `$VARCOND` als auch `$self.variant_condition` werden dabei wie
mehrwertige Merkmale behandelt, d.h., ihnen k¨onnen in einer Preisbeziehungen mehrere Bezeichner
von Variantenkonditionen zugewiesen werden.


40Anstelle dieser Standard-Variablen kann auch eine eigene, spezifische Variable verwendet werden, die dann in der
Versionsinformationstabelle (s. 2.23) anzugeben ist.


37


Vor dem Zugriff auf die Preistabelle werden die in den Preisbeziehungen zugewiesenen Variantenkonditionen in Großbuchstaben umgewandelt [41] .


Die relevanten Preisbeziehungen werden in der genannten Reihenfolge aus den Beziehungsobjekten


1. des Artikels

2. der Merkmalsklassen des Artikels
3. der aktuell bewerteten Merkmale des Artikels [42]


4. der Werte der aktuell bewerteten Merkmale


bestimmt.
Wird ein und dieselbe Variantenkondition in verschiedenen Preisbeziehungen gesetzt, ist nicht definiert [43], ob die zur Variantenkondition geh¨orende Preiskomponente entsprechend oft in die Preisfindung eingeht oder nur einmal [44] .


Neben der Zuweisung einer Variantenkondition k¨onnen Preisbeziehungen auch die Bewertung von
Hilfsmerkmalen f¨ur nachfolgend ausgewertete Preisbeziehungen beinhalten. Eine solche Zuweisung
ist jedoch nur w¨ahrend der Auswertung der Preisbeziehungen innerhalb einer Preisebene wirksam.


**3.3** **G¨ultige Tabelleneintr¨age**


Aus den f¨ur eine gegebene Preiskomponente aus der Preistabelle gelesenen Eintr¨agen (s. Abschn. 3.2)
wird der g¨ultige (am besten passende) wie folgt ermittelt.


1. Eintr¨age f¨ur die Preisebene ’B’ (Grundpreis) werden ignoriert, wenn sie keinen festen (absoluten)
Preisbetrag (sondern eine Prozentangabe) enthalten.


2. Eintr¨age ohne eine korrekte Datumsangabe in den Feldern 10 und 11 (G¨ultigkeitszeitraum) bzw.
mit einem G¨ultigkeitszeitraum, der f¨ur das geforderte Datum der Preisermittlung nicht erf¨ullt ist,
werden ignoriert.


3. Soll der Preis in einer bestimmten W¨ahrung ermittelt werden, werden nur die Eintr¨age mit dieser
W¨ahrung ber¨ucksichtigt. Besitzt keiner der (zeitlich g¨ultigen) Eintr¨age die geforderte W¨ahrung,
kommen alle Eintr¨age in die weitere Auswahl.


4. Aus den verbliebenen Eintr¨agen werden die Eintr¨age ausgeschlossen, deren Mengeangabe zur Staffelpreisbildung (Feld 12) kleiner ist als die Menge zur aktuellen Bestell- bzw. Auftragsposition.

5. Aus den verbliebenen Eintr¨agen wird der Eintrag mit dem j¨ungsten Datum im Feld 10 (”G¨ultig
von“) verwendet.
(Gibt es auch hinsichtlich dieses Kriteriums mehrere Eintr¨age, ist nicht definiert, welcher der Eintr¨age durch die OFML–Applikation verwendet wird [45] .)


6. Ist in dem verbliebenen Eintrag die Rechenregel bzw. die Fixpreis-/Prozentangabe f¨ur die spezifizierte Preisebene nicht erlaubt, so wird auch dieser Eintrag ignoriert, d.h. die gew¨unschte Preiskomponente kann nicht ermittelt werden.


41s.a. Anmerkung zu den Variantenkonditionen in der Preistabelle, Abschn. 2.17
42das schließt nicht-konfigurierbare Merkmale mit ein
43abh¨angig von der verwendeten OFML–Applikation
44Eine derartige Datenanlage ist also nicht zu empfehlen.
45Wenn – bei einer korrekten Datenanlage – f¨ur eine Preiskomponente und eine gegebene W¨ahrung keine 2 Tabelleneintr¨age mit demselben Datum in Feld 10 vorhanden sind, kann dieser Fall nur auftreten, wenn f¨ur die Preiskomponente
Eintr¨age mit verschiedenen W¨ahrungen (und demselben Datum in Feld 10) vorliegen, aber keine von den W¨ahrungen der
angeforderten W¨ahrung entspricht. F¨ur die Applikation ist dann nicht ersichtlich, welches der passendste Tabelleneintrag
ist.


38


**3.4** **Preisfaktoren**


In Preisbeziehungen (Tabelle `Relation` ) k¨onnen Preisfaktoren zu einem Preisposten angegeben werden,
der an eine Variantenkondition gebunden ist. Dazu wird die Funktion _$SET_ ~~_P_~~ _RICIN_ ~~_G F_~~ _ACTOR()_ verwendet, die in den OCD-Sprachsets wie folgt spezifiziert ist:


 _$SET_ ~~_P_~~ _RICIN_ ~~_G F_~~ _ACTOR(<Variantenkondition>, <Faktor>)_


Die Funktion definiert den _<_ Faktor _>_, mit dem der Preis multipliziert werden soll, der f¨ur die
angegebene _<_ Variantenkondition _>_ in der Tabelle `Price` festgesetzt ist.


_<_ Variantenkondition _>_ kann als Ausdruck angegeben werden, der als Ergebnis eine Zeichenkette
liefert.


_<_ Faktor _>_ ist ein arithmetischer Ausdruck. (Ist der Ausdruck undefiniert, hat die Funktion keinen
Effekt.) Preisfaktoren k¨onnen auch einen negativen Betrag haben.


Der Aufruf der Funktion folgt in der Regel der Zuweisung der Variantenkondition an die Variable
`$VARCOND` im selben Beziehungswissen.


Beispiel:


Ist das Merkmal ”Elektrifizierung“ mit dem Wert ”Set 1“ belegt, soll der Aufpreis in Abh¨angigkeit von
der Breite des Tisches festgelegt werden. Die Preisbeziehung, die an das Beziehungsobjekt f¨ur den
Wert ”Set 1“ des Merkmals ”Elektrifizierung“ gebunden ist, k¨onnte dann wie folgt definiert werden:

```
   $VARCOND = ’ABC123_ELEKTR_1’, $SET_PRICING_FACTOR(’ABC123_ELEKTR_1’, BREITE / 1000)

```

Anmerkung:

Da das Merkmal BREITE in mm angegeben ist, wird hier der Aufpreis (aus der Preistabelle) mit der

aktuellen Breite in Metern multipliziert.


Der Funktionsaufruf kann auch mit einer Bedingung verkn¨upft werden. Der Faktor wird dann nur verwendet, wenn die Bedingung eindutig erf¨ullt ist.


Beispiel:


Ist das Merkmal ”Elektrifizierung“ mit dem Wert ”Set 2“ belegt, soll der Aufpreis um 10 Prozent
erh¨oht werden, wenn die Breite des Tisches gr¨oßer als ein Meter ist. Die Preisbeziehung, die an das
Beziehungsobjekt f¨ur den Wert ”Set 2“ gebunden ist, k¨onnte dann wie folgt definiert werden:

```
   $VARCOND = ’ABC123_ELEKTR_2’,
   $SET_PRICING_FACTOR(’ABC123_ELEKTR_2’, 1.1) IF BREITE > 1000

```

In den SAP-Sprachsets besitzt die Funktion _$SE_ ~~_T_~~ _PRICIN_ ~~_G_~~ _FACTOR()_ zwei zus¨atzliche Parameter am
Anfang, f¨ur die im OCD jeweils nur ein fester Wert ¨ubergeben werden kann:


 _$SET_ ~~_P_~~ _RICIN_ ~~_G F_~~ _ACTOR($self, varian_ ~~_t_~~ _condition, <Variantenkondition>, <Faktor>)_


Erfolgen in einer Preisbeziehung mehrere Aufrufe zum Setzen eines Faktors f¨ur ein und dieselbe Variantenkondition, so wird nur der letzte Aufruf wirksam.


39


## **4 Die Endartikelnummererzeugung**

Die Endartikelnummer f¨ur einen Artikel wird gem¨aß dem Schema erzeugt, das f¨ur den Artikel in der
Artikeltabelle anhand des Schema-Identifikators angegeben ist. Ist in der Schematabelle (s. Abschn. 2.24)
kein oder ein nicht referenzierter Identifikator angegeben, wird f¨ur den Artikel keine spezielle Endartikelnummer erzeugt. Diese ist dann gleich der Grundartikelnummer.


In den folgenden Ausf¨uhrungen wird auf ein einfaches Beispiel eingegangen:


Ein Schrank mit der Grundartikelnummer `0815` geh¨ore der Merkmalsklasse `Schrank` an und besitze
aktuell folgende Merkmalsauspr¨agungen:

```
   Oberflaeche: 03
   Hoehe: 5H
   Zubehoer (optional): nicht ausgewaehlt

```

Aktuell nicht sichtbar (ung¨ultig) sei das Merkmal `Schloss` . Die Feldl¨ange f¨ur alle Merkmale betrage
2.


**4.1** **Die vordefinierten Schemata**


Bei diesen Schemata beginnt die Endartikelnummer grunds¨atzlich mit der Grundartikelnummer aus der
Artikeltabelle. Dieser folgt die im 3. Feld der Schematabelle angegebene Zeichenkette. Der Trennzeichenkette folgt schließlich der Variantencode, der in den einzelnen vordefinierten Schemata wie unten
beschrieben generiert wird.


Der Identifikator f¨ur ein vordefiniertes Schema (s.u.) muß im 2. Feld der Schematabelle angegeben werden.


F¨ur die Beispiele wird die Trennzeichenkette ” `-` “ angenommen.


 **KeyValueList**


Jedes aktuell g¨ultige Merkmal wird in der Reihenfolge gem¨aß der Property–Tabelle wie folgt dargestellt:


_<_ `Merkmalsklasse` _>_ `.` _<_ `Merkmal` _>_ `=` _<_ `Merkmalswert` _>_


Als Trennzeichen zwischen den Merkmalen wird das Semikolon verwendet. F¨ur aktuell nicht ausgew¨ahlte optionale Merkmale wird das interne Wertk¨urzel ” `VOID` “ verwendet.

Die Parameterfelder 4 bis 7 haben f¨ur dieses Schema keine Bedeutung. Als Trimm-Kennzeichen
wird immer `1` verwendet (unabh¨angig von der Angabe im Feld 8).


F¨ur das Beispiel ergibt sich damit:

```
     0815-Schrank.Oberflaeche=03;Schrank.Hoehe=5H;Zubehoer=VOID

```

 **ValueList**


Die Merkmale werden in der Reihenfolge gem¨aß der Property–Tabelle alleine anhand des aktuellen
Merkmalswertes dargestellt.


Die Darstellung kann dabei mit Hilfe der Parameterfelder 4 bis 10 gesteuert werden. Das Feld 5
(Sichtsbarkeitsmodus) sollte bei Neu–Datenanlagen immer gef¨ullt sein, um ein definiertes Verhalten
zu erzielen.


F¨ur das Beispiel sei als Zeichenkette zum Trennen der Merkmalswerte (Feld 4) eine leere Zeichenkette angenommen. F¨ur die Ersetzungszeichen (Felder 6 und 7) seien die Standardzeichen
’ `-` ’ bzw. ’ `X` ’ verwendet. (Das Trimm-Kennzeichen hat keine Bedeutung, da alle Werte exakt die
in der Property–Tabelle angegebene L¨ange besitzen.)

Damit ergibt sich f¨ur das Beispiel im Sichtbarkeitsmodus `0` :

```
     0815-035HXX

```

und im Sichtbarkeitsmodus `1` :

```
     0815-035HXX
```

40


**4.2** **Nutzerdefinierte Schemata**


Im Feld 2 k¨onnen von den vordefinierten Schemata abweichende Kodierungsvorschriften definiert werden.


Die Syntax der Schemabeschreibung ist:


_<_ Scheme _>_ := [ _<_ PropertyClass _>_ : _<_ PropertyName _> | <_ TableCall _> |_ @ _| <_ Char _>_,] 1:n


_<_ PropertyClass _>_ := Name der Merkmalsklasse in der Tabelle Property


_<_ PropertyName _>_ := Name des Merkmals in der Tabelle Property


_<_ TableCall _>_ := Tabellenaufruf gem¨aß Syntax und Semantik im Sprachset `OCD_2`


_<_ Char _>_ := Ein einzelnes Zeichen (außer dem Komma und ’ `@` ’)


Beim Aufbau der Endartikelnummer wird die Schemabeschreibung von links nach rechts abgearbeitet.
Dabei werden folgende Ersetzungen ausgef¨uhrt:


 _<PropertyClass>:<PropertyName>_ wird durch den aktuellen Merkmalswert ersetzt. Dabei werden
die Formatierungen aus der Property–Tabelle und die Ersetzungs- und Formatierungsanweisungen
aus den Feldern 5 bis 10 der Schematabelle ber¨ucksichtigt. Ist das Feld 5 (Sichtsbarkeitsmodus)
leer, wird der Modus `1` verwendet.


 Beim Tabellenaufruf werden Aktualparameter, die auf ein Merkmal verweisen, durch den aktuellen
Wert des Merkmals ersetzt. (Der spezielle Aktualparameter `$BAN` wird durch die Grundartikelnummer ersetzt.) Enth¨alt die angesprochene Wertkombinationstabelle f¨ur diese Parameter genau eine
passende logische Zeile und enth¨alt diese einen Wert f¨ur den speziellen Tabellenparameter `$FAN`,
so wird dieser an der aktuellen Position der Endartikelnummer eingesetzt. (Wird kein oder kein
eindeutiges Ergebnis f¨ur `$FAN` geliefert, ist das Verhalten der verwendeten OFML–Applikation undefiniert.)


 _’@’_ wird durch das n¨achste Zeichen der Grundartikelnummer ersetzt. Die Grundartikelnummer wird
ebenfalls von links nach rechts abgearbeitet.


 F¨ur _<Char>_ erfolgt keine Ersetzung. Es wird das angegebene Zeichen an der aktuellen Position
eingesetzt.
Es k¨onnen alle druckbaren Zeichen aus dem ASCII-Zeichensatz verwendet werden außer dem Komma und ’ `@` ’.


F¨ur das Beispiel ergibt sich bei der Schemabeschreibung:

```
   Schrank:Hoehe,_,@,@,-,@,@,_,Schrank:Oberflaeche

```

die Endartikelnummer:

```
   5H_08-15_03

```

Hinweis:


Nutzerdefinierte Endartikelnummern sind nur bedingt zur Rekonstruktion einer Artikelkonfiguration geeignet. Dazu m¨ussen prinzipiell alle konfigurierbaren Merkmale kodiert werden, wobei die in Abschn. 2.24
genannten Einschr¨ankungen bzgl. der Felder 6 bis 8 einzuhalten sind. _Nicht_ geeignet zur Rekonstruktion
einer Artikelkonfiguration ist ein nutzerdefiniertes Schema, wenn es einen Tabellenaufruf verwendet, oder
wenn das Trimm–Kennzeichen gesetzt ist und im Schema zwei Merkmale unmittelbar aufeinander folgen,
d.h. nicht durch Zeichen getrennt sind.


**4.3** **Mehrwertige Merkmale**


Mit den Feldern 9 und 10 der Schematabelle wird in allen Codierungsarten die Darstellung der aktuellen
Werte von mehrwertigen Merkmalen gesteuert:


 Prinzipiell werden alle aktuell gesetzten Werte in der Reihenfolge angezeigt, die durch die Positionsangabe in der Merkmalswerttabelle (s. Abschn. 2.13) bestimmt ist.


41


 Die in Feld 9 angegebene Zeichenkette wird dabei zum Trennen der Werte verwendet. Ist das Feld
leer, wird ein Komma (” `,` “) verwendet. Ist genau ein Zeichen angegeben, so darf es beim vordefinierten Schema `ValueList` nicht identisch sein mit dem Trennzeichen aus Feld 4 der Schematabelle.


 Ist das Feld 10 nicht leer und enth¨alt es eine gerade Anzahl von Zeichen, so wird die erste H¨alfte
der Zeichenkette der Werte-Auflistung vorangestellt und die zweite H¨alfte an die Werte-Auflistung
angeh¨angt. Damit kann eine Klammerdarstellung realisiert werden.


Beispiel: ” `[ABS,ZV]` “


42


## **5 Merkmalstext-Steuerung**

In diesem Abschnitt wird beschrieben, wie mit Hilfe des Steuercodes im Feld _TxtControl_ der Merkmalstabelle (s. Abschn. 2.9) die Generierung des Textes gesteuert werden kann, der das Merkmal in
kaufm¨annischen Formularen (Artikelliste u.¨a.) beschreibt.


Prinzipiell wird der das Merkmal beschreibende Text aus der (einzeiligen) Bezeichnung des Merkmals
(aus Tabelle `PropertyText` ) und dem Text des aktuell zugewiesenen Wertes gebildet. MerkmalswertTexte (Tabelle `PropValueText` oder frei eingegebene beim Merkmalstyp T) k¨onnen mehrzeilig sein. Je
nach Steuercode werden davon jedoch nicht alle Zeilen f¨ur die Merkmalsbeschreibung verwendet.


Im Gegensatz zu den Merkmalsbeschreibungen in kaufm¨annischen Formularen kann das Erscheinungsbild
eines Merkmals in Komponenten (grafischer) Benutzeroberfl¨achen zur Bewertung von Merkmalen (Eigenschaftseditoren) nicht beeinflusst werden. Ein Merkmal wird dort (außer beim Typ T) immer durch seine
(einzeilige) Bezeichnung und die erste Zeile des Textes des aktuell zugewiesenen Wertes dargestellt.


Folgende Codes sind definiert [46] :


**0** Die erste Zeile der Merkmalsbeschreibung wird aus der Merkmalsbezeichnung und der ersten Zeile
des Textes des aktuellen Merkmalswerts gebildet [47] . Es folgen die restlichen Zeilen des MerkmalswertTextes.


Bei mehrwertigen Merkmale (siehe Feld _MultiOption_ in der Merkmalstabelle, Abschn. 2.9) werden
jeweils die ersten Zeilen der Texte der aktuell gesetzten Werte aneinander gereiht, wobei die Reihenfolge durch die Positionsangabe in der Merkmalswerttabelle (s. Abschn. 2.13) bestimmt ist und
als Trenn–Zeichenkette zwischen den Werten die Angabe aus dem Feld _M_ ~~_O_~~ _Sep_ der Schema–Tabelle
(Abschn. 2.24) verwendet wird (bzw. ” `,` “, wenn das Feld leer ist).

Dies ist das Standardverfahren, insbesondere bei einzeiligen Merkmalswert-Texten.


**1** Die Merkmalsbeschreibung entspricht dem (mehrzeiligen) Text des aktuellen Merkmalswerts, d.h.,
die ¨ubliche Merkmalsbezeichnung in der ersten Zeile (Standard) wird unterdr¨uckt.



Beispiel:
Ein Stuhl besitze das Merkmal ”Mechanik“ mit den Werten:



” [Standardmechanik mit Gasdruckfeder“ (1 Zeile)]



” [Synchronmechanik Optima Plus“ (1 Zeile)]
Die Merkmalsbeschreibung im Formular lautet bei Auswahl des ersten Wertes dann



” [Standardmechanik mit Gasdruckfeder“]
anstelle von

” [Mechanik: Standardmechanik mit Gasdruckfeder“]

im Standardfall.


**2** Die erste Zeile der Merkmalsbeschreibung besteht nur aus der Merkmalsbezeichnung. Die restlichen
Zeilen entsprechen den Zeilen 2..n des Textes des aktuellen Merkmalswertes.


Beispiel:
Ein Schrank besitze das Merkmal ”Fachb¨oden verst¨arkt“ mit den Werten ”Ja“ und ”Nein“.
F¨ur den Wert ”Nein“ wird in der Tabelle `PropertyValue` das Feld _SuppressTxt_ auf True gesetzt.
Bei Auswahl dieses Wertes erscheint im Formular kein Text zu diesem Merkmal.
Beim Wert ”Ja“ ist ”SuppressTxt“ auf False gesetzt. Als Merkmalsbeschreibung erscheint

” [Fachb¨oden verst¨arkt“]
anstelle von

” [Fachb¨oden verst¨arkt: Ja“]

im Standardfall.


**3** Die Merkmalsbeschreibung wird aus den Zeilen 2..n des Textes des aktuellen Merkmalswertes gebildet, d.h. Merkmalsbezeichnung und erste Zeile des Merkmalswert-Textes werden unterdr¨uckt.


46Bei Merkmalen des Typs T sind nur die Codes `0` und `5` erlaubt
47Die beiden Textbausteine werden dabei durch einen Doppelpunkt und ein Leerzeichen getrennt, falls diese Zeichen nicht
schon am Ende der Merkmalsbezeichnung enthalten sind.


43


Beispiel:
Ein Tisch besitze das Merkmal ”Elektrifizierung“ und f¨ur dessen Wert `E01` sei folgender Text
hinterlegt:

Zeile 1: Set 1
Zeile 2: Elektrifizierung bestehend aus:
Zeile 3: - 2x Kabelschlange
Zeile 4: - 2x Vielfach-Steckdose
Im Eigenschaftseditor erscheint: ”Elektrifizierung: Set 1“.
Die Merkmalsbeschreibung lautet aber:

” [Elektrifizierung bestehend aus:]

     - 2x Kabelschlange

     - 2x Vielfach-Steckdose“


**4** Keine Beschreibung des Merkmals im Formular.


Die gleiche Wirkung kann erzielt werden, wenn f¨ur alle Werte des Merkmals in der Tabelle
`PropertyValue` das Feld _SuppressTxt_ auf True gesetzt ist.


Sinnvoll ist der Einsatz dieses Modus z.B. f¨ur Hilfsmerkmale, die vom Nutzer konfiguriert werden
k¨onnen, aber nicht gedruckt werden sollen.


Beispiel:
Ein Stuhl besitze das Merkmal ”zweifarbiger Bezug“ mit den Werten ”Ja“ und ”Nein“. Bei Auswahl
von ”Nein“ werde das Merkmal ”Farbe Bezug“ freigeschaltet, bei ”Ja“ hingegen die Merkmale

” [Farbe Sitzbezug“ und ] ” [Farbe R¨uckenbezug“.]

Das Merkmal ”zweifarbiger Bezug“ soll/muss in diesem Fall nicht im Formular beschrieben werden.
Dies wird durch den Code 4 bewerkstelligt.


**5** Gilt speziell f¨ur mehrwertige Merkmale und Merkmale des Typs T.


F¨ur mehrwertige Merkmale gilt:
Die erste Zeile der Merkmalsbeschreibung besteht aus der Merkmalsbezeichnung. Es folgen die (evtl.
mehrzeiligen) Bezeichnungen der aktuell gesetzten Werte in der Reihenfolge, die durch die Positionsangabe in der Merkmalswerttabelle (s. Abschn. 2.13) bestimmt ist. Die Bezeichnungen des ersten
bis vorletzten Wertes werden dabei mit der Zeichenkette abgeschlossen, die im Feld _M_ ~~_O S_~~ _ep_ der
Schema–Tabelle (Abschn. 2.24) angegeben ist (bzw. ” `,` “, wenn das Feld leer ist).

F¨ur Merkmale des Typs T gilt:
Die erste Zeile der Merkmalsbeschreibung besteht aus der Merkmalsbezeichnung. Es folgen die Zeilen
des aktuell durch den Anwender eingegebenen Textes.


44


## **6 Die Ermittlung von Verpackungsdaten**

In diesem Abschnitt wird beschrieben, wie mit Hilfe der Packaging–Tabelle (Abschn. 2.5) und Beziehungswissen (Abschn. 2.15 und 2.16) die Verpackungsdaten (Maße, Volumen, Gewichte, Anzahl Verpackungseinheiten) zu einem Artikel in einer konkreten Konfiguration ermittelt werden.


Die Berechnung der Daten erfolgt in folgenden Schritten:


1. Ermitteln der Grunddaten f¨ur den Artikel in Grundausf¨uhrung durch Auslesen des Eintrags ohne
Variantenkondition (leeres Feld 2) aus der Tabelle `Packaging` .


2. Ermitteln der abweichenden Daten mit Hilfe von Variantenkonditionen:


1. Ermitteln aller f¨ur die aktuelle Konfiguration geltenden Variantenkonditionen anhand der
Packaging-Beziehungen.


2. F¨ur jede ermittelte, aktuell g¨ultige Variantenkondition:
Auslesen des zugeh¨origen Eintrags aus der Tabelle `Packaging` und Addition der Betr¨age aller
nicht-leeren Felder zu den entsprechenden Betr¨agen aus den Grunddaten bzw. zu den durch
vorhergehende Variantenkonditionen evtl. bereits akkumulierten Betr¨agen.


Zum Schritt 2.1:


 Packaging-Beziehungen sind in der Tabelle `RelationObj` (Feld 5) durch die Verwendungsart ’PCKG’
gekennzeichnet und m¨ussen vom Typ Aktion (’3’) sein (Feld 4).


 Variantenkonditionen werden in Packaging-Beziehungen (Tabelle `Relation` ) durch Zuweisung der
Bezeichnung der Variantenkondition an die spezielle Variable `$VARCOND` hergeleitet.


 Eine Variantenkondition kann mit einem Faktor verkn¨upft werden. Dazu wird folgende Funktion
verwendet:


_$SET_ ~~_P_~~ _CK_ ~~_G F_~~ _ACTOR(<Variantenkondition>, <Datenelement>, <Faktor>)_


Die Funktion definiert den _<_ Faktor _>_, mit dem der Wert des _<_ Datenelementes _>_ aus dem Eintrag der
Packaging–Tabelle multipliziert werden soll, der f¨ur die angegebene _<_ Variantenkondition _>_ gelesen
wurde.


_<_ Variantenkondition _>_ kann als Ausdruck angegeben werden, der als Ergebnis eine Zeichenkette
liefert.


_<_ Datenelement _>_ ist der Name des betreffenden Feldes der Packaging–Tabelle in Großschreibung,
eingeschlossen in Hochkommata, z.B. ’NETWEIGHT’.


_<_ Faktor _>_ ist ein arithmetischer Ausdruck. (Ist der Ausdruck undefiniert, hat die Funktion keinen
Effekt.)


Der Aufruf der Funktion folgt in der Regel der Zuweisung der Variantenkondition an die Variable
`$VARCOND` im selben Beziehungswissen.


Der Funktionsaufruf kann mittels einer IF–Klausel (s. Anh. B) an eine Bedingung gekn¨upft werden.


 Die relevanten Packaging-Beziehungen werden in der genannten Reihenfolge aus den Beziehungsobjekten


1. des Artikels

2. der Merkmalsklassen des Artikels
3. der aktuell bewerteten Merkmale des Artikels [48]


4. der Werte der aktuell bewerteten Merkmale


bestimmt.


48das schließt nicht-konfigurierbare Merkmale mit ein


45


## A Sprachdefinition OCD_1

 Diese einfache Sprache erlaubt die Angabe von Bedingungen (alle Arten von Beziehungen) und von
Zuweisungen an die Merkmale eines Artikels (in Aktionen, Reaktionen und Post-Reaktionen). Innerhalb einer Aktion, Reaktion oder Post-Reaktion k¨onnen dabei mehrere Zuweisungen stattfinden.
Diese sind durch Kommata voneinander zu trennen.


 Zeichenkettenkonstanten sind in Hochkommata ( `’` ) einzuschließen.


 Bei Schl¨usselw¨ortern sowie bei Bezeichnern f¨ur Merkmale und bei Werten von Zeichenketten–
Merkmalen wird Groß-/Kleinschreibung ignoriert.
Bsp.: `IF` ist identisch mit `if` .


 In logischen und arithmetischen Ausdr¨ucken k¨onnen Merkmalsnamen im Sinne von Variablen verwendet werden. Bei der Auswertung des Ausdrucks werden sie durch den aktuellen Wert des Merkmals ersetzt. Die spezielle Variable `$BAN` wird bei der Auswertung durch die Grundartikelnummer
des aktuellen Artikels ersetzt.


 **Arithmetische Ausdr¨ucke** :


**–** Arithm. Ausdr¨ucke k¨onnen verwendet werden


      - als numerische Operanden in Vergleichen,

      - auf der rechten Seite von Zuweisungen und

      - als Faktoren in den Builtin-Funktionen _SE_ ~~_T P_~~ _RICIN_ ~~_G_~~ _FACTOR()_ und
_SE_ ~~_T P_~~ _CK_ ~~_G F_~~ _ACTOR()_ .


**–** Ein komplexer arithmetischer Ausdruck wird mittels der arithmetischen Grundoperationen
und Klammer-Setzung aus untergeordneten arithmetischen Ausdr¨ucken gebildet.


**–** Einfache arithmetische Ausdr¨ucke sind:


      - numerische Konstanten

      - Verweise auf numerische Merkmale

      - Aufrufe einer arithmetischen Funktion (s. Anhang F)


Bezieht sich ein einfacher arithmetischer Ausdruck auf ein Merkmal, das der betreffende Artikel
nicht besitzt, oder das nicht bewertet ist, so ist der Wert des Ausdrucks _undefiniert_ . Ist ein
undefinierter arithmetischer Ausdruck Teil eines ¨ubergeordneten, komplexen arithmetischen
Ausdrucks, so ist auch der Wert dieses Ausdrucks undefiniert.


 **Bedingungen** :


**–** Bedingungen sind einfache oder komplexe boolesche (logische) Ausdr¨ucke. Ein logischer Ausdruck wird entweder als _wahr_ oder _falsch_ bewertet. Unter Umst¨anden kann ein logischer Ausdruck nicht bewertet werden, das Ergebnis ist dann _undefiniert_ (s.u.).
Komplexe boolesche Ausdr¨ucke werden mittels der Operatoren `AND` und `OR` aus Unterausdr¨ucken gebildet.
Bei verketteten `AND` und `OR` Operatoren werden zuerst die `AND` Verkn¨upfungen ausgewertet.
Die Reihenfolge der Auswertung kann durch Klammer-Setzung gesteuert werden. Vergleiche
z.B. `A and B or C` gegen¨uber `A and (B or C)` .


**–** Einfache logische Ausdr¨ucke sind:


      - Vergleiche

      - Verneinung (Negation)

      - Spezielle Bedingungen


**–** Vergleiche werden mittels der bekannten Vergleichsoperatoren notiert: _<_ (oder `LT` ), _<_ `=` (oder
`LE` ), `=` (oder `EQ` ), _<>_ (oder `NE` ), `=` _>_ [49] (oder `GE` ) und _>_ (oder `GT` ).
Die Operanden auf beiden Seiten des Vergleichs m¨ussen vom selben Typ sein (nur Zeichenkette
oder nur numerisch).


49alternativ kann die Form _>_ `=` verwendet werden


46


**–** Der Vergleich von Zeichenketten basiert auf der lexikographischen Ordnung, d.h. zwei Zeichenketten werden Zeichen f¨ur Zeichen verglichen, bis zwei sich unterscheidende Zeichen gefunden
werden. Die Zeichenkette, dessen sich unterscheidendes Zeichen lexikographisch kleiner als das
entsprechende Zeichen der anderen Zeichenkette ist, gilt dann als die kleinere Zeichenkette.
Falls eine Zeichenkette komplett abgearbeitet wurde, bevor ein Unterschied festgestellt wird,
gilt die k¨urzere Zeichenkette als die kleinere Zeichenkette.
Die lexikographische Ordnung der Zeichen ergibt sich aus der Codierung der Zeichen im verwendeten Zeichensatz. Beim im OCD verwendeten Zeichensatz ISO-8859-1 (Latin-1) gilt somit
z.B. `’A’ < ’a’` .
Die lexikographische Ordnung ist insbesondere bei Zeichenketten zu beachten, die komplett
aus Ziffern bestehen. Dort kann es zu unterschiedlichen Ergebnissen gegen¨uber dem Vergleich
entsprechender Zahlen kommen. So ist z.B. `900 < 1000` aber `’900’ > ’1000’` !

**–** Mittels `NOT` –Operator k¨onnen logische Ausdr¨ucke verneint werden.

**–** _Spezielle Bedingungen_ sind:


    - `SPECIFIED` _<_ `Merkmalsname` _>_
Diese Bedingung ist wahr, wenn der Artikel das angegebene Merkmal besitzt und dieses
mit einem Wert belegt ist.

    - _<_ `Merkmalsname` _>_ `IN (` _<_ `Wertemenge` _>_ `)`
Diese Bedingung ist wahr, wenn der aktuelle Wert des im linken Operanden angegebenen
Merkmals in der Wertemenge enthalten ist, die im rechten Operanden angegeben ist. Die
Werte in der Wertemenge sind durch Kommata zu trennen.

**–** Bezieht sich ein einfacher logischer Ausdruck auf ein Merkmal, das der betreffende Artikel
nicht besitzt, oder das nicht bewertet ist, so kann der Ausdruck nicht bewertet werden und
das Ergebnis ist _undefiniert_ . Einzige Ausnamhe ist der `SPECIFIED` -Ausdruck, der dazu genutzt
werden kann, undefinierte logische Ausdr¨ucke zu vermeiden.
F¨ur die Verkn¨upfungsoperatoren `AND` und `OR` gelten dann folgende Regeln:


    - Das Ergebnis einer `OR` Verkn¨upfung ist undefiniert, wenn entweder beide Unterausdr¨ucke
undefiniert sind, oder wenn ein Unterausdruck undefiniert und der andere nicht wahr ist.
(Ist wenigstens ein Unterausdruck wahr, so ist die `OR` Verkn¨upfung in jedem Fall wahr,
auch wenn der andere Unterausdruck undefiniert ist.)

    - Das Ergebnis einer `AND` Verkn¨upfung ist undefiniert, wenn entweder beide Unterausdr¨ucke
undefiniert sind, oder wenn ein Unterausdruck undefiniert und der andere wahr ist. (Ist
wenigstens ein Unterausdruck nicht wahr, so ist die `AND` Verkn¨upfung in jedem Fall nicht
wahr, auch wenn der andere Unterausdruck undefiniert ist.)

F¨ur die verschiedenen Beziehungsarten gelten folgende Regeln in Bezug auf undefinierte logische Ausdr¨ucke:


    - Eine _Vorbedingung_ ist verletzt, wenn sie eindeutig falsch ist, d.h. sie ist nicht verletzt, wenn
der logische Ausdruck undefiniert ist.

    - Eine _Auswahlbedingung_ ist verletzt, wenn sie nicht eindeutig wahr ist, d.h. sie ist auch
verletzt, wenn der logische Ausdruck undefiniert ist.


 **Zuweisungen** :


**–** Zuweisungen erfolgen ¨uber den Zuweisungsoperator `=` . Der linke Operand ist ein Merkmal oder
die spezielle Variable `$VARCOND` (s. Abschn. 3).

**–** Die Operanden auf beiden Seiten der Zuweisung m¨ussen vom selben Typ sein (Zahl vs. Zeichenkette).

**–** Ist der linke Operand ein numerisches Merkmal, so findet ggf. eine Rundung des rechten Operandes gem¨aß der f¨ur das Merkmal festgelegten Anzahl erlaubter Nachkommastellen statt.
Dabei wird die Methode des mathematischen Rundens angewendet.

**–** Steht auf der rechten Seite der Zuweisung ein undefinierter arithmetischer Ausdruck, so findet
keine Zuweisung statt.

**–** Eine Zuweisung kann mit einer Bedingung versehen werden. Diese ist nach dem Schl¨usselwort
`IF` anzugeben. Die Zuweisung findet dann nur statt, wenn die Bedingung eindeutig erf¨ullt ist
(also nicht, wenn der logische Ausdruck undefiniert ist).


47


## B Sprachdefinition OCD_2

Diese Sprachdefinition umfasst alle Festlegungen aus der Sprachdefinition `OCD_1` . Dar¨uberhinaus gelten
folgende weitergehende Festlegungen:


 Zeichenketten k¨onnen mittels des Operators + verkettet werden.
Dadurch entstehen _Zeichenketten–Ausdr¨ucke_, in denen Zeichenkettenkonstanten und Werte von
Zeichenketten–Merkmalen miteinander verkn¨upft werden k¨onnen.


 Numerische Werte k¨onnen mittels der Funktion `STRING()` in eine Zeichenkette konvertiert werden:
Die Ergebnis-Zeichenkette enth¨alt die einfache Dezimalpunktnotation der ¨ubergegebenen Zahl mit
dem Punkt (’ `.` ’) als Dezimaltrennzeichen und ohne Tausender- oder sonstige Trennzeichen. Nichtsignifikante gebrochene Teile werden nicht dargestellt, z.B. `9.0` _→_ `9` “.
”
Die Funktion kann auch in Zeichenketten–Ausdr¨ucken verwendet werden.


 In logischen Ausdr¨ucken k¨onnen die boolschen Konstanten `FALSE` und `TRUE` verwendet werden.


 F¨ur _einschr¨ankbare Merkmale_ gelten folgende Konkretisierungen in Bezug auf Bedingungen:


**–** Die Bedingung `SPECIFIED` ist erf¨ullt, wenn der Wertebereich f¨ur das Merkmal auf genau einen
Wert eingeschr¨ankt wurde.

**–** Ein Vergleich (inklusive der `IN` -Bedingung) ist nur m¨oglich, wenn der Wertebereich auf genau
einen Wert eingeschr¨ankt wurde (anderenfalls ist das Ergebnis des Ausdrucks undefiniert).


 F¨ur den Beziehungstyp _Constraint_ wird eine Syntax und Semantik definiert, die unten genauer
beschrieben ist.


 In Vorbedingungen, Constraints, Aktionen und Reaktionen k¨onnen Pr¨ufungen und Werteherleitungen mit Hilfe von Wertkombinationstabellen und der Funktion `TABLE()` realisiert werden.
Die diesbez¨ugliche Syntax und Semantik wird unten genauer beschrieben.


**B.1** **Constraints**


 Ein Constraint ist ein komplexes Sprachkonstrukt, das vornehmlich zur Uberwachung der Konsi- [¨]
stenz einer Konfiguration verwendet wird, aber auch zur Herleitung von Merkmalswerten oder zur
Einschr¨ankung von Wertebereichen eingesetzt werden kann.


 Constraints m¨ussen immer an Artikel gebunden sein (s. Abschn. 2.15). Damit k¨onnen in einem
Constraint Aussagen ¨uber Merkmale von mehreren Merkmalsklassen des Artikels gemacht werden.


 Ein Constraint besteht aus bis zu vier Teilen, die jeweils durch ein Schl¨usselwort plus Doppelpunkt
eingeleitet und durch einen Punkt abgeschlossen werden:


 **Objects:**


In diesem Teil werden die Objekte benannt, ¨uber die Aussagen im Constraint gemacht werden,
wobei hier unter Objekten Merkmalsklassen und Merkmale verstanden werden. Mehrere Objektdeklarationen werden durch Kommatas getrennt.


Merkmalsklassen und Merkmale werden in den folgenden Constraint-Teilen ¨uber den Namen von
Variablen angesprochen, die f¨ur sie im `Objects` -Teil deklariert wurden:


**–** Variablen f¨ur Merkmalsklassen werden mit Hilfe des Konstrukts `IS_A` deklariert:

_<_ `Variable` _>_ `IS_A` _<_ `Merkmalsklasse` _>_

Ist einer der deklarierten Merkmalsklassen nicht dem Artikel zugeordnet, an den das Constraint
gebunden ist, wird das Constraint nicht ausgewertet.


**–** Deklarationen von Variablen f¨ur Merkmale folgen der Deklaration der Klasse, zu dem die Merkmale geh¨oren. Die Merkmalsdeklarationen werden durch das Schl¨usselwort `WHERE` eingeleitet.
Mehrere Merkmalsdeklarationen sind durch Semikolon zu trennen. Eine Merkmalsdeklaration
hat folgende Form:

_<_ `Variable` _>_ `=` _<_ `Merkmal` _>_


48


Wird f¨ur ein Merkmal keine eigene Variable definiert, so kann es in den folgenden Constraint-Teilen ¨uber die Variable seiner Merkmalsklasse angesprochen werden:
_<_ `Merkmalsklassenvariable` _>_ `.` _<_ `Merkmal` _>_ .


 **Condition:**


gibt die Bedingung an, die erf¨ullt sein muß, damit das Constraint ausgewertet wird.


F¨ur die Syntax dieses Teils gelten die allgemeinen Festlegungen f¨ur Bedingungen gem¨aß Sprachdefinition `OCD_1` .


Ist das Ergebnis der Auswertung der Bedingung undefiniert, so wird das Constraint nicht ausgewertet, da keine sichere Aussage dar¨uber m¨oglich ist, ob es ausgewertet werden darf.


Dieser Constraint-Teil ist optional. Fehlt er, so wird das Constraint ausgewertet.


 **Restrictions:**


gibt die Beziehungen an, die zwischen den Merkmalen des Artikels bestehen m¨ussen, damit die
aktuelle Konfiguration des Artikels als konsistent betrachtet wird. Mehrere Beziehungen werden
durch Kommata voneinander getrennt.


Ohne den `Inferences` –Teil (s.u.) stellen die Beziehungen einfach Bedingungen dar. Ist eine der
Beziehungen nicht erf¨ullt bzw. keine eindeutige Aussage dar¨uber m¨oglich (undefinierter logischer
Ausdruck), ist das Constraint nicht erf¨ullt und der Artikel besitzt damit eine inkonsistente (ung¨ultige) Konfiguration. Von der Laufzeitumgebung wird sichergestellt, daß entweder ein inkonsistenter
Artikel nicht bestellt werden kann oder daß eine Merkmals¨anderung, die zu einem inkonsistenten
Zustand f¨uhren w¨urde, nicht ausgef¨uhrt werden darf.


Zusammen mit dem `Inferences` –Teil k¨onnen die Beziehungen gleichzeitig auch Werteherleitungen oder Wertebereichseinschr¨ankungen bewirken. In diesen F¨allen erzwingt das Constraint eine
konsistente Konfiguration.


Alle Beziehungen k¨onnen mit einer Bedingung versehen werden. Diese ist nach dem Schl¨usselwort
`IF` anzugeben. Die Beziehung wird dann nur ausgewertet, wenn die Bedingung erf¨ullt ist.


Beziehungen k¨onnen beschrieben werden durch:


**–** _Wertevergleich_

Die Ausdr¨ucke auf beiden Seiten des Gleichheit-Operators `=` m¨ussen gleich sein.


**–** _Wertemengen-Pr¨ufung_ : _<_ `Merkmalsname` _>_ `IN (` _<_ `Wertemenge` _>_ )

Der Wert des auf der linken Seite angegebenen Merkmals muß in der im rechten Teil des
Ausdrucks angebenenen Wertemenge enthalten sein.


**–** _Aufruf einer Wertkombinationstabelle_

Zu der aktuellen Konfiguration des Artikels muß ein passender Eintrag in der Wertkombinationstabelle enthalten sein.

Syntax und Semantik von Tabellenaufrufen werden im n¨achsten Abschnitt beschrieben.


 **Inferences:**


bestimmt die Merkmale (komma–separierte Aufz¨ahlung), f¨ur welche durch das Constraint Werte
hergeleitet werden sollen oder f¨ur die deren Wertebereich eingeschr¨ankt werden soll [50] .


Dieser Constraint-Teil ist optional. Er entf¨allt, wenn keine Werte hergeleitet oder Wertebereiche
eingeschr¨ankt werden sollen.


Die Herleitung bzw. die Einschr¨ankung erfolgt anhand der Beziehungen im `Restrictions` –Teil.


**–** _Wertherleitung mittels Wertevergleich_ :

Ist ein Operand des Gleichheit-Operators eine Merkmalsvariable und ist das Merkmal unter
`Inferences` angef¨uhrt, bewirkt die Beziehung eine Zuweisung des Werts des anderen Operanden an das Merkmal, vorausgesetzt, der Wert des anderen Operanden ist bestimmt [51] .


50Merkmale, deren Wertebereiche mittels eines Constraints eingeschr¨ankt werden k¨onnen, m¨ussen in der Merkmalstabelle
als einschr¨ankbar gekennzeichnet sein.
51Der Vergleich ist damit immer erf¨ullt.


49


Beispiel:

```
      Objects:
        cup IS_A cupboard_a.
      Condition:
        cup.ausf_gruppe = ’A’.
      Restrictions:
        cup.farbe_tuer = ’F002’.
      Inferences:
        cup.farbe_tuer.

```

Ist der linke Operand eine Merkmalsvariable und das Merkmal _nicht_ einschr¨ankbar und der
Wert des rechten Operanden bestimmt, so erfolgt in jedem Fall eine Zuweisung, auch wenn
das Merkmal nicht unter `Inferences` angef¨uhrt ist.


**–** _Wertebereich-Einschr¨ankung mittels_ `IN` _–Ausdruck_ :

Ist das Merkmal auf der linken Seite des `IN` –Ausdrucks unter `Inferences` angef¨uhrt und einschr¨ankbar, bewirkt der Ausdruck eine Einschr¨ankung der Wertemenge des Merkmals auf die
im rechten Teil des Ausdrucks angebenene Menge [52] . Die im rechten Teil des Ausdrucks angegebene Menge darf in diesem Fall keine Interval-Werte beinhalten.


Beispiel:

```
      Objects:
        cup IS_A cupboard_a.
      Restrictions:
        cup.farbe_korpus IN (’F001’, ’F002’, ’F003’).
      Inferences:
        cup.farbe_korpus.

```

**–** _Wertherleitung und Wertebereich-Einschr¨ankung mittels Tabellenaufruf_ :


Beispiel:

```
      Objects:
        cup IS_A cupboard_a
        where
          ausf = ausf_gruppe;
          korpus = farbe_korpus.
      Restrictions:
        TABLE farben_korpus (ausf_gruppe = ausf,
                   farbe_korpus = korpus).
      Inferences:
        korpus.

```

Syntax und Semantik des Tabellenaufrufs wird im n¨achsten Abschnitt n¨aher beschrieben.


Eine einmal eingeschr¨ankte Wertemenge eines einschr¨ankbaren Merkmals kann durch ein nachfolgend ausgewertetes Constraint nicht wieder erweitert werden. D.h., enth¨alt die Wertemenge, die
durch einen `IN` –Ausdruck oder durch einen Tabellenaufruf bestimmt ist, einen Wert, der in der
aktuellen Wertemenge nicht enthalten ist, so wird der Wert auch nicht in die neue Wertemenge
¨ubernommen.


52Die `IN` -Beziehung ist damit immer erf¨ullt.


50


**B.2** **Tabellenaufruf**


Mit Hilfe der Funktion `TABLE()` kann in Vorbedingungen, Aktionen, Reaktionen und Constraints auf
Wertkombinationstabellen (s.Abschn. 2.21) zugegriffen werden.


Die allgemeine Syntax des Aufrufs ist:


`TABLE` _<_ `Tabellenname` _>_ `(` _<_ `Parameterliste` _>_ `)`


Im Tabellennamen d¨urfen alphanumerische Zeichen inklusive dem Unterstrich verwendet werden, wobei
das erste Zeichen kein numerisches sein darf.


_<_ `Parameterliste` _>_ ist eine komma-separierte Auflistung der Zugriffsparameter:
_<_ `Tabellenmerkmal` _>_ `=` _<_ `Aktualparameter` _>_ .


_Aktualparameter_ k¨onnen sein:


 numerische oder Zeichenkettenkonstante


 Merkmalsvariable


Namen von Merkmalen von Artikel und Tabelle m¨ussen nicht identisch sein, der Aktualparameter muß
jedoch in jedem Fall mit dem Wert des zugeordneten Tabellenmerkmals verglichen werden k¨onnen.


Die Semantik des Tabellenaufrufs unterscheidet sich in den verschiedenen Beziehungsarten und wird in
den folgenden Unterabschnitten beschrieben.


**B.2.1** **Tabellenaufruf in Vorbedingungen**


In Vorbedingungen stellt der Tabellenaufruf einen logischen Ausdruck dar:


 Ist einer der Aktualparameter in der aktuellen Konfiguration nicht bewertet, so findet kein Tabellenzugriff statt und der Ausdruck ist undefiniert. (Zur Behandlung von undefinierten logischen
Ausdr¨ucken s.Anhang A).


 Wird zu den Aktualparametern genau ein Eintrag in der Tabelle gefunden, ist das Ergebnis _wahr_,
ansonsten _falsch_ .


**B.2.2** **Tabellenaufruf in Aktionen und Reaktionen**


In Aktionen und Reaktionen werden Wertkombinationstabellen dazu verwendet, um Werte f¨ur Merkmale
herzuleiten.


Die Merkmale, f¨ur die durch einen Tabellenaufruf Werte herzuleiten sind, m¨ussen in der Parameterliste
als Aktualparameter angegeben sein und mit dem Pr¨afix ” `$SELF.` “ versehen sein. Alle anderen Aktualparameter dienen als Zugriffsschl¨ussel. Eine Ausnahme ist die Verwendung der spezifischen Variablen
`$VARCOND` [53] in Preisbeziehungen zum Setzen von Variantenkonditionen: Ist diese Variable als Aktualparameter angegeben, so erfolgt f¨ur diese immer eine Werteherleitung durch den Tabellenaufruf.


Sind keine oder alle Aktualparameter mit dem Pr¨afix ” `$SELF.` “ versehen, so werden f¨ur alle diejenigen
Merkmale Werte hergeleitet, die als Aktualparameter angegeben, zum Zeitpunkt des Aufrufs aber nicht
bewertet sind.


Es findet kein Tabellenzugriff und damit keine Werteherleitung statt, wenn entweder keine Aktualparameter gem¨aß der obigen Festlegungen als Zugriffsschl¨ussel bestimmt werden konnten oder wenn einer der
explizit als Schl¨usselparameter definierten Aktualparameter nicht bewertet ist.


Der Tabellenaufruf muß f¨ur jedes herzuleitende Merkmal zu einen eindeutigen Ergebnis f¨uhren. Anderenfalls schl¨agt der Aufruf fehl und es findet keine Werteherleitung statt.


53bzw. der in der Versionsinformationstabelle daf¨ur als Ersatz festgelegten Variablen


51


**B.2.3** **Tabellenaufruf in Constraints**


Die Behandlung von Tabellenaufrufen in Constraints ist komplexer als in Aktionen und h¨angt sowohl
vom aktuellen Bewertungskontext hinsichtlich der Aktualparameter als auch vom `Inferences` –Teil ab:


 Fehlt der `Inferences` –Teil oder sind alle Aktualparameter bewertet, so dient der Tabellenaufruf
alleine der Konsistenzpr¨ufung:
Alle Aktualparameter dienen dann als Schl¨ussel f¨ur den Tabellenzugriff.


Ist einer der Aktualparameter in der aktuellen Konfiguration nicht bewertet (bei fehlendem
`Inferences` –Teil), so findet kein Tabellenzugriff statt und das Constraint ist nicht erf¨ullt. Enth¨alt die Tabelle keinen Eintrag f¨ur den durch die Aktualparameter bestimmten Schl¨ussel, so ist das
Constraint ebenfalls nicht erf¨ullt.


 Sind ein oder mehrere einschr¨ankbare Merkmale, die beim Tabellenaufruf als Aktualparameter ¨ubergeben werden, nicht bewertet, so dienen die anderen Aktualparameter als Schl¨ussel f¨ur den Tabellenzugriff, ausgenommen von nicht einschr¨ankbaren Merkmalen, die im `Inferences` –Teil angef¨uhrt
sind. Es erfolgt dann eine Herleitung von Werten f¨ur diejenigen Merkmale, die unter `Inferences`
angef¨uhrt sind:


**–** Bei einem einschr¨ankbaren Merkmal wird die Wertemenge auf die Schnittmenge der Wertemenge vor der Auswertung des Constraints und der Wertemenge eingeschr¨ankt, die durch den
Tabellenzugriff geliefert wurde.

**–** Einem nicht einschr¨ankbaren Merkmal wird der durch den Tabellenzugriff gelieferte Wert zugewiesen. Hat der Tabellenzugriff mehrere Werte geliefert, ist das Constraint nicht erf¨ullt.


Ist ein als Aktualparameter angegebenes einschr¨ankbares Merkmal nicht bewertet und nicht unter
`Inferences` angef¨uhrt, so bleibt seine Wertemenge unver¨andert.


 Sind alle Aktualparameter Merkmalsvariablen und sind alle diese Merkmale einschr¨ankbar, aber in
der aktuellen Konfiguration nicht bewertet, so existiert kein Schl¨ussel f¨ur einen Tabellenzugriff.


In diesem Fall wird der Tabellenaufruf wie folgt behandelt:
Es werden alle logischen Zeilen der Tabelle der Reihe nach ausgelesen. F¨ur jedes herzuleitende (unter
`Inferences` aufgef¨uhrte) Merkmal wird gepr¨uft, ob die f¨ur dieses Merkmal in der logischen Zeile
angegebenen Werte in der aktuell eingeschr¨ankten Wertemenge enthalten sind. Falls die Werte aller
herzuleitenden Merkmale der logischen Tabellenzeile g¨ultig sind, werden die Werte jeweils in eine
Liste pro Merkmal ¨ubernommen (wobei ein mehrfaches Auftreten von Werten ausgeschlossen wird).
Nach der Verarbeitung aller logischen Tabellenzeilen werden die so ermittelten Wertemengen dann
den jeweiligen herzuleitenden Merkmalen als neue (eingeschr¨ankte) Wertemenge zugewiesen.


52


## C Sprachdefinition OCD_3

Diese Sprachdefinition umfasst alle Festlegungen aus der Sprachdefinition `OCD_2` . Dar¨uberhinaus gelten
folgende weitergehende Festlegungen zur Verarbeitung mehrwertiger Merkmale und zur mehrstufigen
Konfiguration.


**C.1** **mehrwertige Merkmale**


 Der logische Ausdruck


`SPECIFIED` _<_ `mehrwertiges Merkmal` _>_


ist wahr, wenn mindestens ein Wert gesetzt ist.


 Bei `IN` –Vergleichen darf ein mehrwertiges Merkmal nur auf der rechten Seite stehen. Auf der linken
Seite muß eine Konstante stehen.


Beispiel: `’ABS’ IN Sonderaustattung`


 Bei normalen Vergleichen d¨urfen 2 mehrwertige Merkmale _nicht_ miteinander verglichen werden. Ein
mehrwertiges Merkmal darf immer nur mit einer Konstante verglichen werden.


Beispiel: `Sonderaustattung = ’ABS’` ist wahr, wenn der Wert `ABS` gesetzt ist (unabh¨angig davon,
ob evtl. noch weitere Werte gesetzt sind).


 Im `Condition` –Teil von Constraints d¨urfen auch 2 mehrwertige Merkmale miteinander verglichen
werden. Der Vergleich ist wahr, wenn beide Mengen gesetzter Werte gleich sind.


 Im Zuweisungsteil von Aktionen bzw. im `Restrictions` –Teil von Constraints sind auf beiden Seiten
des Ausdrucks mehrwertige Merkmale erlaubt. In diesem Fall werden alle aktuell gesetzten Werte
des Merkmals auf der rechten Seite f¨ur das Merkmal auf der linken Seite ¨ubernommen.


**C.2** **mehrstufige Konfiguration**


In Beziehungen von Artikeln, die Unterartikel eines kompositen Artikels (s. Abschn. 2.6) sind oder sein
k¨onnen, ist es notwendig zu unterscheiden, ob ein Merkmal des aktuell zu konfigurierenden Artikels
oder ein Merkmal eines ¨ubergeordneten Artikels referenziert wird. Dazu sind folgende Klassifikatoren f¨ur
Merkmale definiert:


 `$self` :
bezieht sich auf den aktuell konfigurierten Artikel


 `$parent` :
bezieht sich auf den unmittelbar ¨ubergeordneten Artikel


 `$root` :
bezieht sich auf den obersten Artikel in einer mehrstufigen Konfiguration


Die Klassifikatoren werden dem Merkmalsnamen durch einen Punkt getrennt vorangestellt. Ist ein Merkmal nicht klassifiziert, so wird `$self` angenommen.


53


## D Sprachdefinition OCD_4

Diese Sprachdefinition umfasst alle Festlegungen aus der Sprachdefinition `OCD_3` . Dar¨uberhinaus gelten
folgende weitergehende Festlegungen:


 Anweisungsblock:


In den Beziehungsarten _Aktion_, _Reaktion_ und _Post–Reaktion_ k¨onnen Anweisungen mittels geschweifter Klammern `{...}` in einem Block zusammenfasst werden.
Dies ist inbesondere dann hilfreich, wenn mehrere Anweisungen mit ein und derselben Bedingung
verkn¨upft werden sollen.


 Wertebereiche in `IN` –Ausdr¨ucken:


Neben diskreten Werten k¨onnen in der Werteliste auch nach oben bzw. unten offene sowie geschlossene Wertebereiche (Intervalle) angegeben werden.


Operatoren f¨ur Obergrenze: _<_ (oder `LT` ), _<_ `=` (oder `LE` )
Operatoren f¨ur Untergrenze: `=` _>_ [54] (oder `GE` ) und _>_ (oder `GT` )


nach unten offener Bereich: _<_ `Obergrenze-Operator` _><_ `diskreter Wert` _>_


nach oben offener Bereich: _<_ `Untergrenze-Operator` _><_ `diskreter Wert` _>_


geschlossener Wertebereich: _<_ `diskreter Wert` _>_ `-` _<_ `diskreter Wert` _>_


Die f¨ur einen geschlossenen Wertebereich als untere und obere Grenze angegebenen Werte geh¨oren
zum Wertebereich.


Beispiel:

```
     Breite IN (5-10, 20, >30)

```

 Platzhalter in IN–Ausdr¨ucken:

Bei Vergleichen mittels IN–Ausdruck [55] k¨onnen in Zeichenkettenkonstanten in der Wertemenge die
Platzhalter–Zeichen ’*’ und ’?’ verwendet werden. Das Zeichen ’*’ ersetzt dabei beliebig viele (0-n)
Zeichen und das Zeichen ’?’ genau ein Zeichen des Vergleichsoperanden auf der linken Seite.


Beispiele:


**–** Der Vergleich mit `IN (’F*O’)` ist wahr, wenn der Wert des linken Operanden mit dem
Zeichen ’F’ beginnt und mit dem Zeichen ’O’ endet. Dazwischen k¨onnen sich beliebig viele
andere Zeichen befinden (auch keins).

**–** Der Vergleich mit `IN (’F?O’)` ist wahr, wenn der Wert des linken Operanden mit dem
Zeichen ’F’ beginnt, mit dem Zeichen ’O’ endet, und sich dazwischen genau ein weiteres
Zeichen befindet.


Die Platzhalter–Ersetzung wird jedoch nur wirksam, wenn sie durch das Feld _PlaceHolderOn_ in der
Versionsinformationstabelle (Abschn. 2.23) frei geschaltet ist. (Anderenfalls sind die beiden obigen
Beispielsvergleiche nur wahr, wenn der linke Operand genau die Zeichenkette `’F*O’` bzw. `’F?O’`
enth¨alt.)
Werden die Ersetzungszeichen (’*’, ’?’) auch als regul¨are Zeichen in Werten von Zeichenketten–
Merkmalen verwendet, so kann bei Platzhalter–Freischaltung ein exakter Vergleich mit Hilfe der
normalen Vergleichsoperatoren vorgenommen werden, da dort generell keine Platzhalter–Ersetzung
stattfindet.


 Optionaler `Objects` –Teil in Constraints:


Fehlt der `Objects` –Teil, so beziehen sich die in den anderen Teilen des Constraints verwendeten
Bezeichner auf die gleichnamigen Merkmale des Artikels.


54alternativ kann die Form _>_ `=` verwendet werden
55also nicht, wenn der IN–Ausdruck in einem Constraint zur Wertebereichseinschr¨ankung verwendet wird


54


 Logische Ausdr¨ucke im `Restrictions` –Teil von Constraints:


Bei Wertevergleichen im `Restrictions` –Teil sind generell logische Ausdr¨ucke erlaubt (nicht nur der
Gleichheitsoperator wie in der Sprachdefinition `OCD_3` ). Das Constraint schl¨agt fehl, wenn die durch
den Ausdruck bestimmte Bedingung nicht eindeutig erf¨ullt ist.


Die in dem Ausdruck referenzierten Merkmale d¨urfen nicht im evtl. vorhandenen `Inferences` –Teil
des Constraints angef¨uhrt sein!


 Ausgabe von Meldungen an den Anwender:


In den Beziehungsarten _Aktion_, _Reaktion_ und _Post–Reaktion_ des Verwendungsgebiets C (Konfiguration) kann die Funktion


`USER_MESSAGE(` _<_ `Textnummer` _>_ `)`


verwendet werden. Diese gibt die in der Texttabelle `UserMessage` (s.Abschn. 2.20) unter der angegebenen Textnummer (ID) hinterlegte Nachricht an den Anwender aus. (Die Textnummer ist dabei
als Zeichenkettenkonstante anzugeben, d.h. in Hochkommata einzuschließen.)
Dies kann dazu genutzt werden, bei komplexem Beziehungswissen dem Anwender Hilfestellung zu
geben.
Bei der Ausgabe wird die Sprache verwendet, die aktuell in der Applikation eingestellt ist. Ist f¨ur
diese Sprache in der Tabelle `UserMessage` unter der angegebenen Textnummer kein Text hinterlegt,
erfolgt auch keine Ausgabe einer Nachricht.
Wie bei Zuweisungen kann dem Aufruf der Funktion `USER_MESSAGE()` nach dem Schl¨usselwort `IF`
eine Bedingung folgen, die erf¨ullt sein muss, damit die Ausgabe der Nachricht erfolgt.


 Abbruch der Auswertung von Beziehungswissen:


In den Beziehungsarten _Reaktion_ (f¨ur ein Merkmal) und _Post–Reaktion_ kann die Funktion

```
 ABORT()

```

verwendet werden, um die Auswertung von Beziehungswissen nach einer Merkmals¨anderung abzubrechen. (Der Konfigurationszustand des Artikels ist danach derselbe wie vor der versuchten
Merkmals¨anderung.)


Dies kann bei Merkmalen zur freien Eingabe von Werten durch den Anwender sinnvoll/notwendig
sein, wenn nicht alle Eingaben zul¨assig sind. Typischerweise ist der Funktion eine `IF` –Klausel nachgestellt, welche die Bedingung formuliert, die eindeutig erf¨ullt sein muss, damit die Funktion ausgef¨uhrt wird.


 Teilzeichenketten–Generierung:


Die Funktion `SUBSTR(` _<_ `Zeichenkette` _>_ `,` _<_ `Position` _>_ [ `,` _<_ `Laenge` _>_ ] `)` liefert als Ergebnis eine Zeichenkette, deren Inhalt der Teilzeichenkette der im ersten Parameter ¨ubergebenen Zeichenkette
entspricht, die bei der im 2. Parameter angegebenen Position beginnt und die im 3. Parameter
¨ubergebene (maximale) L¨ange besitzt.


Im ersten Parameter kann ein Ausdruck angegeben werden, der als Ergebnis eine Zeichenkette
liefert. (Die Funktion selber kann auch in Zeichenketten–Ausdr¨ucken verwendet werden.)


Die Nummerierung der Zeichenpositionen beginnt dabei mit 0. Ergibt die Summe von Position
(2. Parameter) und L¨ange (3. Parameter) eine Position, die ¨uber die L¨ange der ¨ubergebenen Zeichenkette hinausreicht, so wird die Teilzeichenkette ab der angegebenen Position bis zum Ende der
¨ubergebenen Zeichenkette gebildet.


Der L¨angenparameter ist optional. Ist er nicht angegeben, so wird die Teilzeichenkette ab der
angegebenen Position bis zum Ende der ¨ubergebenen Zeichenkette gebildet.


Im Fall von folgenden Fehlern in den Parametern liefert die Funktion eine leere Zeichenkette:


**–** Der erste Parameter ist keine Zeichenkette.

**–** Die Werte f¨ur Positions- und/oder L¨angenparameter sind nicht ganzzahlig.

**–** Die Werte f¨ur Positions- und/oder L¨angenparameter sind negativ.

**–** Die Position ¨uberschreitet die L¨ange der Zeichenkette.


55


 Es stehen folgende Funktionen zur Verarbeitung von Zeichenketten zur Verf¨ugung:


**–** Die Funktion `SIZE(` _<_ `Zeichenkette` _>_ `)` liefert als Ergebnis die Anzahl der Zeichen, die die im
Parameter ¨ubergebene Zeichenkette enth¨alt.


**–** Die Funktion `ToUPPER(` _<_ `Zeichenkette` _>_ `)` liefert als Ergebnis eine Zeichenkette, in der alle
Kleinbuchstaben der im Parameter ¨ubergebenen Zeichenkette in Großbuchstaben umgewandelt
sind. Das deutsche Eszett (’ _ß_ ’) wird dabei nicht umgewandelt.


**–** Die Funktion `ToLOWER(` _<_ `Zeichenkette` _>_ `)` liefert als Ergebnis eine Zeichenkette, in der alle
Großbuchstaben der im Parameter ¨ubergebenen Zeichenkette in Kleinbuchstaben umgewandelt
sind.


**–** Die Funktion `TRIM(` _<_ `Zeichenkette` _>_ `)` liefert als Ergebnis eine Zeichenkette, in der alle Leerzeichen am Anfang und am Ende der im Parameter ¨ubergebenen Zeichenkette entfernt sind.


**–** Die Funktion `RTRIM(` _<_ `Zeichenkette` _>_ `)` liefert als Ergebnis eine Zeichenkette, in der alle Leerzeichen am Ende der im Parameter ¨ubergebenen Zeichenkette entfernt sind.


**–** Die Funktion `LTRIM(` _<_ `Zeichenkette` _>_ `)` liefert als Ergebnis eine Zeichenkette, in der alle Leerzeichen am Anfang der im Parameter ¨ubergebenen Zeichenkette entfernt sind.


Im Zeichenketten-Parameter kann ein Ausdruck angegeben werden, der als Ergebnis eine Zeichenkette liefert. (Die Funktionen selber k¨onnen auch in Zeichenketten–Ausdr¨ucken verwendet werden.)


 Es stehen folgende Funktionen zur Umwandlung von Zeichenketten in numerische Werte zur Verf¨ugung:


**–** Die Funktion `FLOAT(` _<_ `Ausdruck` _>_ `,` _<_ `Fallback` _>_ `)` liefert als Ergebnis eine gebrochene Zahl in
Abh¨angigkeit vom Typ des im ersten Parameter ¨ubergebenen Ausdrucks:


    - Ergibt der Ausdruck selber eine gebrochene Zahl, so ist diese auch das Ergebnis der Funktion.

    - Ergibt der Ausdruck eine ganze Zahl, so ist das Ergebnis die entsprechende gebrochene
Zahl, d.h. mit gebrochenem Teil `0` .

    - Ergibt der Ausdruck eine Zeichenkette, so wird versucht, diese in eine gebrochene Zahl umzuwandeln. Im Erfolgsfall ist diese das Ergebnis der Funktion, anderenfalls der im zweiten
Parameter angegebene Fallback–Wert. F¨ur die Darstellung der Zahl in der Zeichenkette
gelten dieselben Bestimmungen wie f¨ur die Darstellung von gebrochenen Zahlen in der
Merkmalswerttabelle, d.h., als Dezimaltrennzeichen wird der Punkt (’ `.` ’) verwendet, ein
Tausendertrennzeichen wird _nicht_ verwendet und an erster Stelle kann ein Minuszeichen
stehen.


Ist der 2. Parameter keine konstante gebrochene Zahl, so wird ein Syntaxfehler ausgel¨ost (und
die Auswertung der Beziehung abgebrochen).


**–** Die Funktion `INT(` _<_ `Ausdruck` _>_ `,` _<_ `Fallback` _>_ `)` liefert als Ergebnis eine ganze Zahl in Abh¨angigkeit vom Typ des im ersten Parameter ¨ubergebenen Ausdrucks:


    - Ergibt der Ausdruck selber eine ganze Zahl, so ist diese auch das Ergebnis der Funktion.

    - Ergibt der Ausdruck eine gebrochene Zahl mit gebrochenem Teil `0`, so ist das Ergebnis der
ganzzahlige Teil.

    - Ergibt der Ausdruck eine gebrochene Zahl mit einem gebrochenem Teil ungleich `0`, so ist
das Ergebnis der im zweiten Parameter angegebene Fallback–Wert.

    - Ergibt der Ausdruck eine Zeichenkette, so wird versucht, diese in eine ganze Zahl umzuwandeln. Im Erfolgsfall ist diese das Ergebnis der Funktion, anderenfalls der im zweiten
Parameter angegebene Fallback–Wert. F¨ur die Darstellung der Zahl in der Zeichenkette
gelten dieselben Bestimmungen wie f¨ur die Darstellung von ganzen Zahlen in der Merkmalswerttabelle, d.h., ein Tausendertrennzeichen wird _nicht_ verwendet und an erster Stelle
kann ein Minuszeichen stehen.


Ist der 2. Parameter keine konstante ganze Zahl, so wird ein Syntaxfehler ausgel¨ost (und die
Auswertung der Beziehung abgebrochen).


56


 Sichtbarkeit von Merkmalen:


Die Funktion


`SET_VISIBILITY(` _<_ `Merkmal` _>_ `,` _<_ `logischer Ausdruck` _>_ `)`


kann in Beziehungen der Arten _Aktion_, _Reaktion_ und _Post-Reaktion_ dazu verwendet werden, die
Sichtbarkeit eines Merkmals f¨ur den Anwender einer OFML-Applikation zu steuern:


**–** Liefert der logische Ausdruck den Wert `false`, wird das angegebene Merkmal f¨ur den Anwender
unsichtbar geschaltet.

**–** Liefert der logische Ausdruck den Wert `true`, wird das angegebene Merkmal f¨ur den Anwender
sichtbar geschaltet.

**–** Liefert der logische Ausdruck ein undefiniertes Ergebnis, hat der Funktionsaufruf keinen Effekt
(d.h., der Sichtbarkeitszustand des Merkmals ¨andert sich nicht).


Der Funktionsaufruf kann mit einer Bedingung versehen werden. Diese ist nach dem Schl¨usselwort
`IF` anzugeben. Der Funktionsaufruf findet dann nur statt, wenn die Bedingung eindeutig erf¨ullt ist.


Die Sichtbarkeitssteuerung mittels `SET_VISIBILITY()` ist eine Alternative zur Verwendung von
_Vorbedingungen_ :
Die Funktion `SET_VISIBILITY()` wirkt sich nur auf die Sichtbarkeit des Merkmals f¨ur den Anwender
einer OFML-Applikation aus (Eigenschaftseditor, Variantentext), in technischen Kontexten [56] steht
es (mit dem zuletzt zugewiesenen Wert) jedoch noch zur Verf¨ugung.
Ein per Vorbedingung ausgeblendetes Merkmal hingegen ist komplett ung¨ultig (unsichtbar) und
steht auch nicht in technischen Kontexten zur Verf¨ugung.


Eine Konsequenz aus der Semantik von Vorbedingungen ist damit auch, dass der Aufruf von
`SET_VISIBILITY()` keinen Effekt hat, wenn das angegebene Merkmal aktuell per Vorbedingung
ausgeblendet ist [57] ! Eine Kombination von Vorbedingungen und `SET_VISIBILITY()` f¨ur ein und
dasselbe Merkmal ist somit nicht sinnvoll und nicht zu empfehlen.


 Anderung von Steuerkategorien: [¨]


Mit Hilfe der Funktion


`SET_TAX_CATEGORY(` _<_ `Steuerart` _>_ `,` _<_ `Steuerkategorie` _>_ `)`


kann in `TAX` –Beziehungen (s. Abschn. 2.15) die dem Artikel ¨uber sein Besteuerungsschemata zugeordnete Steuerkategorie in der angegebenen Steuerart ge¨andert (¨uberschrieben) werden.

Typischerweise wird eine derartige Anderung mittels einer [¨] `IF` -Klausel an eine Bedingung gekn¨upft [58] .
(Die Bedingung muß eindeutig erf¨ullt sein, damit der Funktionsaufruf stattfindet.)


Als Bezeichner f¨ur Steuerart und Steuerkategorie sind die im Anhang H vordefinierten Bezeichner
zu verwenden.


`TAX` –Beziehungen k¨onnen an folgende Entit¨aten gebunden werden. Die Auswertung erfolgt dabei in
der angegebenen Reihenfolge:


1. Artikel

2. Merkmalsklassen des Artikels
3. aktuell bewertete Merkmale des Artikels [59]


4. Werte der aktuell bewerteten Merkmale


Finden Aufrufe der Funktion f¨ur eine Steuerart in mehreren Beziehungen statt, so gilt die Zuweisung
des jeweils letzten Aufrufs.


56Auswertung von Beziehungswissen, Grafikdaten, Variantencodes und sonstige technische Beschreibungen der Konfiguration eines Artikels
57D.h., das Merkmal hat, wenn es per Vorbedingung wieder g¨ultig wird, denselben Sichtbarkeitsstatus wie zu dem Zeitpunkt, als es per Vorbedingung ausgeblendet wurde.
58Ein Beispiel w¨are die ¨Anderung der Materialkategorie in der Steuerart `ECO_FR` in Abh¨angigkeit von einer bestimmten
Merkmalsauspr¨agung.
59das schließt nicht-konfigurierbare Merkmale mit ein


57


## E Sprachdefinition SAP_LOVC

Diese Sprachdefinition bezieht sich auf die Sprache, welche in der Variantenkonfiguration des SAP ERP
(Teil des Logistikmoduls) zur Codierung von Beziehungswissen verwendet wird. Standardm¨aßig werden
alle Sprachkonstrukte unterst¨utzt, die syntaktisch und semantisch mit den gleichnamigen Konstrukten
aus den OCD-Sprachdefinitionen identisch sind [60]


Demnach werden (aktuell) folgende Sprachelemente und –konstrukte unterst¨utzt:


 Beziehungsarten: _Vorbedingung_, _Auswahlbedingung_, _Prozedur_ [61], _Constraint_


 Bedingungsteil: `IF`


 Logische Operatoren: `AND`, `OR`, `NOT`


 Built-in Bedingungen: `IN`, `SPECIFIED`


 Vergleichsoperatoren: _<_ oder `LT`, _<_ `=` oder `LE`, `=` oder `EQ`, _<>_ oder `NE`, _>_ `=` oder `GE`, _>_ oder `GT`


 Arithm. Operatoren: `+`, `-`, `/`, `*`


 Arithm. Funktionen: `sqrt()`, `abs()`, `sign()`, `frac()`, `trunc()`, `ceil()`, `floor()`

 Preisfaktoren: `SET_PRICING_FACTOR()` [62]


 Auswertung von Variantentabellen: `TABLE()`


Der Tabellenaufruf in OCD-Aktionen [63] (s. Abschn. B.2.2) weist folgende Besonderheit auf:
Da die im SAP hinterlegten Auswertungsalternativen im OCD nicht bekannt sind, muß auf andere
Weise bestimmt werden, welche Merkmale als Schl¨ussel f¨ur den Tabellenzugriff dienen, und f¨ur
welche Merkmale eine Herleitung eines Wertes stattfinden soll. Im OCD erfolgt dies, indem die
Merkmale, f¨ur die eine Werteherleitung stattfinden soll, mit `$self` qualifiziert werden. Das bedeutet
im Umkehrschluß, dass explizite Schl¨usselmerkmale _nicht_ mit `$self` qualifiziert werden d¨urfen [64] !


60Dar¨uber hinaus k¨onnen die Anbieter von OCD-Implementierungen weitere Sprachkonstrukte unterst¨utzen. Diese sind
bei Bedarf von dem entsprechenden Anbieter der jeweiligen OFML-Anwendung zu erfragen.
61wird nur in dem Umfang unterst¨utzt, wie er f¨ur die OCD-Beziehungsart _Aktion_ definiert ist
62etwas abweichende Signatur (s. Abschn. 3.4)
63SAP: Prozeduren
64In der Variantenkonfiguration des SAP ERP wird f¨ur diese Merkmale dann der Qualifizierer `$root` angenommen.


58


## **F Arithmetische Funktionen in Beziehungswissen**

Folgende Funktionen k¨onnen in arithmetischen Ausdr¨ucken im Code von Beziehungen verwendet werden [65] .


Die Argumente selber k¨onnen arithmetische Ausdr¨ucke sein. Ist ein Argument ein undefinierter arithmetischer Ausdruck, so ist das Ergebnis der Funktion ebenfalls undefiniert.


Bei ung¨ultigen Argumenten wird die Auswertung der Beziehung abgebrochen.


_pow(x(Float), y(Int)) →_ _Float_

Die Funktion _pow_ () berechnet _x_ hoch _y_ . Wenn _x_ negativ ist, muss _y_ ein ganzzahliger Wert sein.
Wenn _x_ 0 ist, muss _y_ positiv sein. Das Ergebnis ist 1 _._ 0, wenn sowohl _x_ und _y_ 0 sind.


_sqrt(x(Float)) →_ _Float_

Die Funktion _sqrt_ () berechnet die nicht–negative Quadratwurzel von _x_ . _x_ darf nicht negativ sein.


_fabs(x(Float)) →_ _Float_

Die Funktion _fabs_ () berechnet den Betrag von _x_ .


_ceil(x(Float)) →_ _Float_

Die Funktion _ceil_ () berechnet den kleinsten ganzzahligen Wert, der nicht kleiner als _x_ ist.


_floor(x(Float)) →_ _Float_

Die Funktion _floor_ () berechnet den gr¨oßten ganzzahligen Wert, der nicht gr¨oßer als _x_ ist.


_sign(x(Float)) →_ _Int_

Die Funktion _sign_ () liefert das Vorzeichen ( `-1` bzw. `+1` ) von _x_ .


_trunc(x(Float)) →_ _Float_

Die Funktion _trunc_ () liefert den ganzzahligen Teil von _x_ .


_frac(x(Float)) →_ _Float_

Die Funktion _frac_ () liefert den Dezimalteil von _x_ .


65Bis auf die Funktionen `sign`, `trunc` und `frac` geh¨oren alle Funktionen auch zu den arithmetischen Standardfunktionen
von OFML.


59


## **G Reservierte Schl¨usselw¨orter**

Im folgenden werden f¨ur jede Sprachdefinition die reservierten Schl¨usselw¨orter aufgelistet.


Diese d¨urfen bei Verwendung der jeweiligen Sprachdefinition nicht als Bezeichner f¨ur Merkmale verwendet
werden! [66]


Zur Beachtung:
Hier sind die Schl¨usselw¨orter nur in kompletter Großschreibung angef¨uhrt. Da in den OCD–Beziehungen
jedoch generell Groß-/Kleinschreibung ignoriert wird (s. A), d¨urfen auch die gemischten Schreibweisen
nicht als Bezeichner f¨ur Merkmale verwendet werden.


Sprachdefinition **OC** ~~**D 1**~~ :


 `AND`


 `BAN`


 `EQ`


 `GE`


 `GT`


 `IF`


 `IN`


 `LE`


 `LT`


 `NOT`


 `OR`


 `SET_PCKG_FACTOR`


 `SET_PRICING_FACTOR`


 `SPECIFIED`


Sprachdefinition **OC** ~~**D 2**~~ :


alle Schl¨usselw¨orter aus `OCD_1` sowie:


 `CONDITION`


 `FALSE`


 `INFERENCES`


 `IS_A`


 `OBJECTS`


 `RESTRICTIONS`


 `TABLE`


 `WHERE`


Sprachdefinition **OC** ~~**D 3**~~ : wie `OCD_2`


66Dar¨uberhinaus k¨onnen Softwareanbieter von OCD–Implementierungen weitere Beschr¨ankungen definieren. Diese sind
beim jeweiligen Softwareanbieter zu erfragen.


60


Sprachdefinition **OC** ~~**D 4**~~ :


alle Schl¨usselw¨orter aus `OCD_3` sowie:


 `ABORT`


 `USER_MESSAGE`


Sprachdefinition **SA** ~~**P L**~~ **OVC** :


alle Schl¨usselw¨orter aus `OCD_2` (außer `BAN` und `SET_PCKG_FACTOR` ) sowie alle Schl¨usselw¨orter gem¨aß der
SAP ERP–Spezifikation


61


## **H Steuerarten und Steuerkategorien**

Diese Tabelle enth¨alt die — innerhalb von OCD — standardisierten Steuerarten und zugeh¨origen Steuerkategorien [67] .

|Steuerart<br>(Erkl¨arung)|Steuerkategorie|Erkl¨arung|
|---|---|---|
|VAT<br>(Mehrwertsteuer,<br>value added tax)|standar~~d r~~ate|Normalsatz|
|VAT<br>(Mehrwertsteuer,<br>value added tax)|reduce~~d r~~ate|erm¨aßigter Satz|
|VAT<br>(Mehrwertsteuer,<br>value added tax)|supe~~r r~~educe~~d r~~ate|stark erm¨aßigter Satz|
|VAT<br>(Mehrwertsteuer,<br>value added tax)|parkin~~g r~~ate|Zwischensatz|
|VAT<br>(Mehrwertsteuer,<br>value added tax)|services|Dienstleistungen|
|VAT<br>(Mehrwertsteuer,<br>value added tax)|zero rate|Nullsatz|
|VAT<br>(Mehrwertsteuer,<br>value added tax)|exemption|steuerbefreit|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|sea~~t m~~etal|Sitzplatz, mehrheitlich aus Metall bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|sea~~t m~~etal95|Sitzplatz, zu mehr als 95% aus Metall bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|sea~~t w~~ood|Sitzplatz, mehrheitlich aus Holz bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|sea~~t p~~lastics|Sitzplatz, mehrheitlich aus Plastik bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|sea~~t o~~ther|Sitzplatz, sonstige Materialien|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|storag~~e m~~etal|Aufbewahrung, mehrheitlich aus Metall bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|storag~~e m~~etal95|Aufbewahrung, zu mehr als 95% aus Metall<br>bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|storag~~e w~~ood|Aufbewahrung, mehrheitlich aus Holz bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|storag~~e p~~lastics|Aufbewahrung, mehrheitlich aus Plastik bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|storag~~e o~~ther|Aufbewahrung, sonstige Materialien|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|workplace~~ m~~etal|Arbeitsplatz, mehrheitlich aus Metall bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|workplace~~ m~~etal95|Arbeitsplatz, zu mehr als 95% aus Metall bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|workplace~~ w~~ood|Arbeitsplatz, mehrheitlich aus Holz bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|workplace~~ p~~lastics|Arbeitsplatz, mehrheitlich aus Plastik bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|workplace~~ o~~ther|Arbeitsplatz, sonstige Materialien|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|othe~~r m~~etal|Sonstiges M¨obel, mehrheitlich aus Metall bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|othe~~r m~~etal95|Sonstiges M¨obel, zu mehr als 95% aus Metall<br>bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|othe~~r w~~ood|Sonstiges M¨obel, mehrheitlich aus Holz bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|othe~~r p~~lastics|Sonstiges M¨obel, mehrheitlich aus Plastik bestehend|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|othe~~r o~~ther|Sonstiges M¨obel, sonstige Materialien|
|EC~~O F~~R<br>(ECO–Tax<br>Frankreich)|none|nicht steuerpﬂichtig|



Anmerkungen zur ECO–Tax Frankreich [68] :


 Das Non-Profit Unternehmen Valdelia [69] ist von der franz¨osischen Regierung bevollm¨achtigt, die
rechtlichen Bestimmungen zur Oko-Steuer im B2B-Gesch¨aft von professionellen Objekt- und B¨uro- [¨]
m¨obeln umzusetzen [70] . Zweck der Steuer in dieser Branche ist die Beteiligung der K¨aufer an der
Entsorgung und Verwertung von M¨obel-Sperrm¨ull.


 Die Steuer ist abh¨angig vom Netto–Gewicht des Artikels und seiner Steuerkategorie, welcher ein
Preisfaktor zugeordnet ist. Das Netto–Gewicht multipliziert mit diesem Preisfaktor ergibt die Steuer. Die Preisfaktoren variieren j¨ahrlich und werden im Oktober bekanntgegeben. Die Steuer wird
erstmalig seit dem 1. M¨arz 2013 erhoben.


67Antr¨age zur Aufnahme weiterer Steuerarten und Steuerkategorien sind an das OFML–Normungsgremium zu richten.
68Die Mehrwertsteuer wird auch auf die ECO–Tax angerechnet und die Steuer ist nicht rabattierbar.
69www.valdelia.org
70F¨ur das Recycling von M¨obeln, die f¨ur Privatkunden bestimmt sind (B2C-Gesch¨aft), ist die Organisation
´eco-mobilier verantwortlich. Deren Steuermodell ist (noch) nicht Gegenstand der OFML-Normung.


62


 Die oben genannten Steuerkategorien entsprechen den Steuerkategorien gem¨aß dem Dokument

” [Bar`eme eco-contribution“][71][. Die Steuerkategorie ist eine Kombination aus M¨obeltyp und Ma-]
terialkategorie [72] . Die Zuordnung zu einer Materialkategorie ergibt sich aus dem Gewichtsanteil.
Machen z.B. Materialien aus Holz mehr als 50 Prozent des Gewichts eines Artikels aus, so wird er
der Materialkategorie _wood_ zugeordnet. Hat keine Materialart die Mehrheit am Gewicht, so ist der
Artikel der Kategorie _other_ zuzuordnen.


71herausgegeben von Valdelia
72Die Materialkategorie `metal95` ist ab dem 1.7.2014 g¨ultig.


63


## **I Begriffe**

 **EAN**


**–** Abk¨urzung f¨ur European Article Numbering


**–** 1977 gegr¨undete Non-Profit Organisation mit dem Ziel der Standardisierung von Identifikationen von Produkten, Einheiten, Einrichtungen usw. zur effizienten Abwicklung von Gesch¨aftsprozessen im Handel.


**–** Seit 1992, nach dem Beitritt von Mitgliedern aus anderen Kontinenten, als EAN International
t¨atig.

**–** Zusammen mit dem Uniform Code Council (UCC) [73], der entsprechenden Beh¨orde f¨ur Nordamerika, wurde das EAN.UCC–System zur Identifizierung von Produkten ( _→_ GTIN), Lokationen ( _→_ GLN) usw. entwickelt.


 **GTIN**


**–** Abk¨urzung f¨ur Global Trade Item Number


**–** Eindeutig im Rahmen des _→_ EAN.UCC-Systems vergebene Identifikationsnummer f¨ur eine
Ware (Produkt oder Dienstleistung), die innerhalb von Gesch¨aftsprozessen im Handel bestellt
und verrechnet werden kann.


**–** Innerhalb des EAN.UCC-Systems k¨onnen dabei verschiedene definierte Nummerierungsschemata verwendet werden (EAN.UCC-14, EAN.UCC-13, EAN.UCC-8).


 **GLN**


**–** Abk¨urzung f¨ur Global Location Number


**–** Eindeutig im Rahmen des _→_ EAN.UCC-Systems vergebene 13-stellige Identifikationsnummer
f¨ur physische und elektronische Adressen von Unternehmen, Tochterunternehmen, Niederlassungen sowie organisatorisch relevante Betriebsteile


 **Intrastat**


**–** Intrastat ist eine vom Statistischen Bundesamt in Wiesbaden gef¨uhrte Statistik, die den innereurop¨aischen Handel mit Deutschland umfaßt.


**–** Jeder deutsche Betrieb, der Handel innerhalb Europas betreibt, muß dies nach genau festgelegten Vorgaben melden. Diese Meldung hat monatlich zu erfolgen.


**–** Jede Ware besitzt eine achtstellige Nummer, die im Warenverzeichnis f¨ur den Außenhandel
aufgef¨uhrt ist, und die zusammen mit Gewicht, Wert, Transportweg etc. angegeben werden
muß.


 **Zolltarifnummer**


**–** Zolltarifnummern kommen bei der Abwicklung von Gesch¨aften mit L¨andern zur Anwendung,
die nicht zur EU geh¨oren.


**–** Ein deutscher Exporteur muss ab einem Warenwert von 1.000 EUR eine schriftliche Ausfuhranmeldung f¨ur die Zollbeh¨orden und das Statistische Bundesamt ausf¨ullen.


**–** Zur Anmeldung jeder Ware ist eine Warentarifnummer erforderlich. Um die Zuordnung zu
erm¨oglichen, ist eine pr¨azise Deklaration der Waren gem¨aß dem _Warenverzeichnis f¨ur die_
_Außenhandelsstatistik_ notwendig.


73seit 2005 unter _GS1_ (Global Standards One) firmierend


64


## **J ¨Anderungshistorie**

**J.1** **OCD 4.3 vs. OCD 4.2**


 Korrektur zur Mehrwertigkeit von Merkmalen in Abschn. 2.9.


 Einf¨uhrung des Konzepts der _Merkmalsgruppen_ (Abschn. 2.11).


 Post-Reaktionen k¨onnen nun auch f¨ur Artikel angegeben werden (Abschn. 2.15).


 Mit Hilfe von `TAX` –Beziehungen k¨onnen Steuerkategorien abh¨angig von bestimmten Artikelvarianten
definiert werden (s. Abschnitte 2.15 und 2.25).


 Aufhebung der Beschr¨ankung der Zeilenl¨ange in den Text-Tabellen (Abschn. 2.20).


 Die alternativen Text-Tabellen wurden entfernt (Abschn. 2.20).


 In der Sprachdefinition `OCD_2` ist nun auch die boolsche Konstante `TRUE` definiert (Anhang B).


 Die Sprachdefinition `OCD_4` enth¨alt die neuen Funktionen `SET_VISIBILITY` zur Steuerung der Sichtbarkeit eines Merkmals f¨ur den Anwender einer OFML-Applikation sowie `SET_TAX_CATEGORY` zur
Zuordnung einer vom Besteuerungsschema abweichenden Steuerkategorie (Anhang D).


 Die Sprachdefinitionen `SAP_3_1` und `SAP_4_6` wurden zu `SAP_LOVC` zusammengefaßt.


65


---

## **K Implementation Notes (OfficeRocket4000)**

This section contains implementation notes specific to the ofml-interpreter in OfficeRocket4000.

### K.1 Price Table (ocd_price) Implementation

The ofml-interpreter reads the price table from pdata.ebase files and caches it per manufacturer.

**Key fields used:**
- `article_nr` - Matched against ProductFamily.base_article_nr
- `var_cond` - Variant condition for surcharge matching
- `price_level` - 'B' for base price, 'X' for surcharges
- `price` - Amount in currency
- `date_from` / `date_to` - For date-based price filtering

**Base price identification:**
The implementation recognizes these var_cond values as base price indicators:
- `S_PGX` (Sedus convention)
- `BASE`
- `STANDARD`
- Empty string

### K.2 Variant Condition Matching

The var_cond field contains surcharge identifiers that must be matched against the current configuration. The implementation uses multiple matching strategies:

**Strategy 1: Direct formula matching**
```
var_cond = "COLOR=white"  matches  variant_code containing "COLOR=white"
var_cond = "WIDTH>1200"   matches  if WIDTH value > 1200
```

**Strategy 2: Numeric code matching (Sedus-style)**
```
var_cond = "S_166"   matches  if any property value is "166" or ends with "166"
var_cond = "S_1701"  matches  if any property value starts with "1701"
```

### K.3 Manufacturer-Specific Findings

#### Sedus (manufacturer ID: sex)

**Property class format:** `KLASSE_XXXXXXXXXXXXXXXXXX` (18-digit numeric suffix)

**Surcharge codes observed:**
| Code | Description | Typical Amount |
|------|-------------|----------------|
| S_PGX | Base price | Variable |
| S_1513 | Counter design | 228 EUR |
| S_166/167/168 | Model colors | 44 EUR |
| S_1801 | Lumbar adjustment | 21 EUR |
| S_6004/6044 | Fire protection | 10 EUR |

### K.4 Limitations

1. **Relation evaluation not implemented**: The ocd_relation rules for determining variant conditions are not evaluated. Surcharge matching uses heuristic pattern matching instead.

2. **SAP LOVC syntax not parsed**: Complex SAP-style variant conditions in ocd_relation are not interpreted.

3. **Discount level ('D') not implemented**: Only base prices and surcharges are calculated.

See `/workspace/ofml-interpreter/docs/OCD-PRICING-IMPLEMENTATION.md` for full implementation details.


