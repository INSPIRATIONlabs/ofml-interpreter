# Specification OEX OFML Business Data Exchange
## (OFML Part VII)

# **ORDCHG**
## Order Change

# Version 3.1.0

English


Editors:
Markus Behrschmidt, Vitra Services GmbH

Thomas Gerth, EasternGraphicsGmbH


May 8, 2023


Copyright © 2006 - 2023 Industrieverband Büro und Arbeitswelt e. V. (IBA)


# Contents

**1** **Introduction ........................................................................................................... 3**

1.1 Using this specification ........................................................................................... 3

1.2 Filename convention ............................................................................................... 3

1.3 XML Declaration ..................................................................................................... 4

1.4 Validation methods ................................................................................................. 4

1.5 Completeness of the document .............................................................................. 4

1.6 Legend .................................................................................................................... 5


**2** **Definitions ............................................................................................................. 6**

2.1 Superior specification ............................................................................................. 6

2.2 Document-type related specifications ..................................................................... 6


**3** **Structure ................................................................................................................ 7**

3.1 Overview of the document structure ....................................................................... 7

3.2 Frame element `oexDocFrame` - OEX document frame ......................................... 8

3.3 Frame element `oexApplication` - Application, creating the document .............. 8

3.4 Frame element `oexFile` - File of documents ....................................................... 8

3.5 Frame element `oexDocument` - Single document ................................................ 9

3.6 Frame element `docHeader` - Document header ................................................... 9

3.7 Frame element `hdrDocNo` - Header: Document numbers ................................... 10

3.8 Frame element `hdrDateTime` - Header: Date and time details .......................... 10

3.9 Frame element `hdrOrgData` - Header: Organizational data .............................. 11

3.10 Frame element `hdrAddress` - Header: Addresses ............................................. 11

3.11 Frame element `hdrCom` - Header: Communication ............................................. 12

3.12 Frame element `hdrContact` - Header: Contacts ............................................... 12

3.13 Frame element `hdrText` - Header: Texts ........................................................... 12

3.14 Frame element `hdrReference` - Header: References ....................................... 13

3.15 Frame element `hdrPricing` - Header: Pricing ................................................... 13

3.16 Frame element `hdrPayment` - Header: Terms of payment ................................. 14

3.17 Frame element `docArticleItem` - Document item: Article ............................... 15

3.18 Frame element `docSetItem` - Document item: Set article ................................. 16

3.19 Frame element `docFolderItem` - Document item: Folder ................................. 17

3.20 Frame element `docTextItem` - Document item: Texts ...................................... 18


                                  - 1 

3.21 Frame element `itmConfiguration` - Item: Configuration details ..................... 18

3.22 Frame element `itmConfigText` - Item: Configuration texts .............................. 19

3.23 Frame element `itmDocNo` - Item: Document numbers ....................................... 19

3.24 Frame element `itmDateTime` - Item: Date and time details ............................... 19

3.25 Frame element `itmOrgData` - Item: Organizational data ................................... 19

3.26 Frame element `itmAddress` - Item: Addresses ................................................. 20

3.27 Frame element `itmCom` - Item: Communication .................................................. 20

3.28 Frame element `itmContact` - Item: Contacts .................................................... 21

3.29 Frame element `itmText` - Item: Texts ................................................................ 21

3.30 Frame element `itmReference` - Item: References ............................................ 21

3.31 Frame element `itmPricing` - Item: Pricing ....................................................... 22


**4** **Appendix ............................................................................................................. 24**

4.1 History of modification .......................................................................................... 24


                                  - 2 

### **1 Introduction**

This specification defines all elements that are used to describe an order change.

Before the first order change an order (ORDERS) must be preceded. It is not necessary that the order has
already been confirmed by the supplier (ORDRSP).
An order change normally results in an order confirmation (ORDRSP). Due to the chronology (processing
and confirmation of the order or a change of the order by the supplier), an order confirmation always refers to
the most recently received order change, i.e., always the most recent changes are confirmed.

Details on the order change are given from the point of view of the purchaser, not of the view of a possibly
involved end-customer, on whose behalf the order change is placed.

Further applicable specifications (in the respectively valid version, see 2.1):


OEX-GLOBAL – Superior specification (document type independent)

Related document types/specifications:


Request (OEX-REQOTE), quotation (OEX-QUOTES), order (OEX-ORDERS), order confirmation (OEXORDRSP), despatch advice (OEX-DESADV) and invoice (OEX-INVOIC).


**1.1** **Using this specification**


This specification describes especially the structure and elements for the document type “ORDCHG - order
change“. Global structures and elements being also used for other document types are described in detail in
the superior specification “OEX-GLOBAL“ of the corresponding version. Only structures and elements that
are derived from “OEX-GLOBAL“ and which are document-type specific are described in this specification.


**1.2** **Filename convention**


Filename convention for the document type “ORDCHG“ is:

```
oex-ordchg_<sender-id>_jjjjmmtt-hhmmss.xml

```

The base of the filename consists of the document type as well as of date and time (24-hours format) of the
creation of the file. The file extension is “ `xml` “.
`<sender-id>` is the variable part of the filename, which must be allocated by the sender of the file. Its
maximum length is 20 digits. For instance, this could be a consecutive numeration of the sender, or the
number of the client or supplier.
Only digits, letters and hyphens are permitted.
In case of failure it is also possible to draw a conclusion with these details in the file type, its sender and the
date when it was created.

Examples: `oex-ordchg_K000085162_20061102-050954.xml`
```
      oex-ordchg_ABC-9564154_20061025-152417.xml

```

                                  - 3 

**1.3** **XML Declaration**

XML Version and Code Page

See superior specification OEX-GLOBAL.


XML Scheme (XS) Integration

The integration of the document-type related schema is effected by attributes defined for XML schemes
within the frame element `oexDocFrame` :

```
<oexDocFrame aMajor="3"
xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xsi:noNamespaceSchemaLocation="oex-ordchg_<Major>.<Minor>.<Build>.xsd">

```

The integration of the superior schema ( `oex-global` ) is already defined in the document-type related
schema.


Valid Version of the XML Schemas

To this specification, the document-type related schema in version 3.1.0 **`oex-ordchg_3.1.0.xsd`** applies,
or in case of modification of the schema without effecting this specification, the schema with the highest build
number is.


**1.4** **Validation methods**


If appropriate XML parsers are used, the respectively valid XML schema (XS) can be applied to check an
OEX-ORDCHG document.
The schema is derived from the corresponding specifications and provided as master tool concerning
element structure and data definition. Further checks of logical contents and dependencies as well as a
mapping of the data are subject to the respectively used application.


**1.5** **Completeness of the document**


In principle, the document is transferred completely, i.e. also with document items (or data) containing no
modifications with respect to the preceding reference document ( `aAction = N` ), see also frame element
`oexDocument` .


                                  - 4 

**1.6** **Legend**

Explanation of specific columns used in the tables in chapter “Structure“.

|Column|Description|Values|Meaning|
|---|---|---|---|
|**`Rec`**|Recurrence|**1 **|Element appears exactly once|
|**`Rec`**|Recurrence|**#+**|Element has to appear minimum**#** times or more.<br>„**#**“ is a placeholder for any number.<br>(Example: 1+ = „must“ 1 time, „can“ several<br>times)|
|**`Rec`**|Recurrence|**#***|Element can appear 0 to several times, up to<br>maximum**#** times, where „**#**“ is a placeholder for<br>any number. If the element is a mandatory<br>element, it must occur at least once. (Ex.: 3* = 1<br>to 3 times)|
|**`Rec`**|Recurrence|*** **|Element can appear 0 to several times. If the<br>element is a mandatory element, it must occur at<br>least once.|
|**`M.`**|Mandatory element|**<empty>**|Element may be available. If it is available it must<br>contain a value.|
|**`M.`**|Mandatory element|**X **|Element must be available and contain a value.|
|**`M.`**|Mandatory element|**# **|Element may be available. If it is available it must<br>contain a value. The placeholder**#** stands for a<br>consecutive number, starting with 1 for sub<br>elements within a frame element which are<br>mutually dependent and in general have to be<br>indicated in combination.<br>(e.g. quantity and quantity unit)|
|**`Key`**|Key element|**! **|Element must be available and contain a value.<br>In addition, the element with its value and if the<br>case may be, with the specifically indicated<br>mandatory attributes must be well-defined in the<br>case of repetitions within a frame element. If<br>several elements are indicated that way they form<br>a unique value (effect as in the case of a primary<br>key).|
|**`Mod`**|Modification|**<empty>**|Element is document related and/or refers to the<br>indicated type of the superior specification.|
|**`Mod`**|Modification|**D **|Element derives from the indicated type of the<br>superior specification and is adapted to the<br>related document.|



                                  - 5 

### **2 Definitions**

**2.1** **Superior specification**


The superior specifications (applicable to all document-types) can be found in the document OEX-GLOBAL
in the respectively valid version 3.1.x. In which the „x“ refers to the highest build version number.


**2.2** **Document-type related specifications**


Specification of the document “ORDCHG“ – Order Change

**Version rules**


This specification is available as version 3.1.0:


Major **3** .1.0
Minor 3. **1** .0
Build 3.1. **0**


Detailed explanations of the version rules can be found in the superior specification (OEX-GLOBAL).

**Recurrence, mandatory and key elements**


Element features like recurrence, mandatory and key elements, can be set document-type related and do
not implicate a derivation to the referred types or domains of the superior specification (OEX-GLOBAL).

**Derived element types**


An element type is called “derived“ if it restricts itself to certain values, attributes and / or sub elements in
opposition to its superior specification (OEX-GLOBAL).


                                  - 6 

### **3 Structure**

**3.1** **Overview of the document structure**


Structure of the frame elements

```
<XML-Declaration>
```

`oexDocFrame` OEX document frame
`├─── oexApplication` Application which has created the document
`└─── oexFile` File of documents
`└─── oexDocument` Single document
`├─── docHeader` Document header
`│` `├─── hdrDocNo` Header: Document numbers
`│` `├─── hdrDateTime` Header: Date and time details
`│` `├─── hdrOrgData` Header: Organizational data
`│` `├─── hdrAddress` Header: Addresses
`│` `│` `├─── hdrCom` Header: Communication
`│` `│` `└─── hdrContact` Header: Contacts
`│` `│` `└─── hdrCom` Header: Communication
`│` `├─── hdrText` Header: Texts
`│` `├─── hdrReference` Header: References
`│` `├─── hdrPricing` Header: Pricing
`│` `└─── hdrPayment` Header: Terms of payment
`├─── docArticleItem` Document item: Article
`│` `├─── itmConfiguration` Item: Configuration details
`│` `│` `└─── itmConfigText` Item: Configuration texts
`│` `├─── itmDocNo` Item: Document numbers
`│` `├─── itmDateTime` Item: Date and time details
`│` `├─── itmOrgData` Item: Organizational data
`│` `├─── itmAddress` Item: Addresses
`│` `│` `├─── itmCom` Item: Communication
`│` `│` `└─── itmContact` Item: Contacts
`│` `│` `└─── itmCom` Item: Communication
`│` `├─── itmText` Item: Texts
`│` `├─── itmReference` Item: References
`│` `└─── itmPricing` Item: Pricing
`├─── docSetItem` Document item: Set article
`│` `├─── itmDocNo` Position: Document numbers
`│` `├─── itmOrgData` Item: Organizational data
`│` `├─── itmText` Position: Texts
`│` `├─── itmReference` Position: References
`│` `└─── itmPricing` Position: Pricing
`├─── docFolderItem` Document item: Folder
`│` `├─── itmText` Position: Texts
`│` `└─── itmReference` Position: References
`└─── docTextItem` Document item: Text
`└─── itmText` Item: Texts


                                  - 7 

**3.2** **Frame element** **`oexDocFrame`** **– OEX document frame**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`oexDocFrame`**|**`DocFrame`**|**1 **|**X **|||**OEX document frame**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`oexApplication`**|**`Applic`**|**1 **|**X **|||**Application which has created the**<br>**document**|
|~~**`oexFile`**~~|~~**`File`**~~|**1 **|**X **|||**File of documents**|



**3.3** **Frame element** **`oexApplication`** **– Application, creating the document**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`oexApplication`**|**`Applic`**|**1 **|**X **|||**Application which has created the**<br>**document**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vAppName`**|**`Value`**|**1 **|**X **|||**Name of application**|
|**`eAppVersion`**|**`AppVersion`**|**1 **|**X **|||**Version of application**|



**3.4** **Frame element** **`oexFile`** **– File of documents**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`oexFile`**|**`File`**|**1 **|**X **|||**File of documents**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vDocumentType`**|**`DocumentType`**|**1 **|**X **||**D **|**Type of document**|
|**`vDocumentType`**|**Attribute**|**Attribute**|||||
|**`vDocumentType`**|`aMajor`|`aMajor`|X|||Major version number|
|**`vDocumentType`**|`aMinor`|`aMinor`|X|||Minor version number|
|**`vDocumentType`**|`aBuild`|`aBuild`|X|||Build version number|
|**`vDocumentType`**|**Table of values**|**Table of values**|||**D **||
|**`vDocumentType`**|`ORDCHG`|`ORDCHG`||||Order change|
|~~**`oexDocument`**~~|~~**`Document`**~~|**1+**|**X **||**D **|**Single document**|



                                  - 8 

**3.5** **Frame element** **`oexDocument`** **– Single document**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|~~**`oexDocument`**~~|~~**`Document`**~~|**1+**|**X **|**! **||**Single Document**<br>_(regarding the type of document)_|
|~~**`oexDocument`**~~|**Attribute**|**Attribute**|||||
|~~**`oexDocument`**~~|`aDocNo`|`aDocNo`|X|!||Consecutive number of the document|
|~~**`oexDocument`**~~|`aItemCount`|`aItemCount`|X|||Total number of items within document|
|~~**`oexDocument`**~~|`aAction`|`aAction`|X|||Action|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`docHeader`**|**`Header`**|**1 **|**X **|||**Document header**|
|~~**`docArticleItem`**~~|~~**`Item`**~~|**1+**|**X **||**D **|**Document item: Article**|
|~~**`docSetItem`**~~|~~**`Item`**~~|*** **|||**D **|**Document item: Set article**|
|~~**`docFolderItem`**~~|~~**`Item`**~~|*** **|||**D **|**Document item: Folder**|
|~~**`docTextItem`**~~|~~**`Item`**~~|*** **|||**D **|**Document item: Text**|



The processing of the order change is operated and supported by the attribute `aAction` :
If a change has been made to at least one document item in comparison to the preceding document (reference document), value `M` must be specified for the attribute `aAction` . For the changed document items
( `doc*Item` ), the attribute has to be set accordingly. If there were no changes in the document items, value `N`
must be specified for attribute `aAction` here and in all document items.


**3.6** **Frame element** **`docHeader`** **– Document header**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`docHeader`**|**`Header`**|**1 **|**X **|||**Document header**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vOrderChangeNo`**|**`DocNo`**|**1 **|**X **|||**Order change number**<br>_Unique number of the order change_<br>_from the supplier._|
|**`vPrecedingDocType`**|**`DocNoType`**<br>|**1 **|**X **||**D **|**Type of (the number of) preceding**<br>**document**|
|**`vPrecedingDocType`**|**Table of values**|**Table of values**|**Table of values**|**Table of values**|**Table of values**|**Table of values**|
|**`vPrecedingDocType`**|`ORD`|||||Order number|
|**`vPrecedingDocType`**|`CHG`|||||Order change number|
|**`vPrecedingDocType`**|`CNF`|||||Order confirmation number|
|**`vPrecedingDocNo`**|**`DocNo`**|**1 **|**X **|||**Number of preceding document**<br>_The document to which possible_<br>_changes in this document are related._|


|vClientNumber|Value|1|X|Col5|Col6|Client number<br>Number, which is used by the vendor<br>(supplier) for his client.|
|---|---|---|---|---|---|---|
|**`vClientID`**|**`ClientID`**|*** **||||**Client ID**|
|**`vClientClass`**|**`ClientClass`**|*** **||||**Client classification**|
|**`vVendorNumber`**|**`Value`**|**1 **|**X **|||**Vendor (supplier) number**<br>_Number, which is used by the_<br>_purchaser (client) for his vendor._|
|**`vSupplierID`**|**`SupplierID`**|*** **||||**Supplier ID**|
|**`vSupplierClass`**|**`SupplierClass`**|*** **||||**Supplier classification**|
|**`vDocCurrency`**|**`DocCurrency`**|**1 **|**X **|||**Currency of document**|



                                  - 9 

|vIncoTerm|IncoTerm|1|1|Col5|Col6|Inco Terms (terms of delivery)<br>Different terms of delivery can be<br>specified within the header text<br>“Delivery conditions“.|
|---|---|---|---|---|---|---|
|**`vIncoTermLocation`**|**`IncoTermLoc`**|**1 **|**1 **|||**Location concerning Inco Terms**|
|**`vPartialDelivery`**|**`PartDelivery`**|**1 **|**X **|||**Allow partial deliveries?**|
|**`vDocLanguage`**|**`DocLanguage`**|**1 **|**X **|||**Language of document**|
|**`vOrderType`**|**`OrderType`**|**1 **||||**Type of order**|
|**`vGrossWeight`**|**`GrossWeight`**|**1 **|**2 **|||**Gross weight (total)**|
|**`vNetWeight`**|**`NetWeight`**|**1 **|**2 **|||**Net weight (total)**|
|**`vUnitWeight`**|**`UnitWeight`**|**1 **|**2 **|||**Weight unit**|
|**`vVolume`**|**`Volume`**|**1 **|**3 **|||**Volume (total)**|
|**`vUnitVolume`**|**`UnitVolume`**|**1 **|**3 **|||**Volume unit**|
|~~**`hdrDocNo`**~~|~~**`DocNo`**~~|*** **|||**D **|**Header: Document numbers**|
|~~**`hdrDateTime`**~~|~~**`DateTime`**~~|**1+**|**X **|||**Header: Date and time details**|
|~~**`hdrOrgData`**~~|~~**`OrgData`**~~|*** **||||**Header: Organizational data**|
|~~**`hdrAddress`**~~|~~**`Address`**~~|**1+**|**X **|||**Header: Addresses**|
|~~**`hdrText`**~~|~~**`Text`**~~|*** **||||**Header: Texts**|
|~~**`hdrReference`**~~|~~**`Reference`**~~|*** **||||**Header: References**|
|~~**`hdrPricing`**~~|~~**`Pricing`**~~|*** **|||**D **|**Header: Pricing**|
|~~**`hdrPayment`**~~|~~**`Payment`**~~|**3***||||**Header: Terms of payment**|


The document header contains all important references of the document.

Explanation of mandatory details:


**1** The location for Inco Terms has to be specified as soon as the delivery term requires it.
**2** The weight unit has to be specified as soon as the gross weight and/or the net weight are specified.
**3** The volume unit has to be specified as soon as the volume is specified.


**3.7** **Frame element** **`hdrDocNo`** **– Header: Document numbers**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrDocNo`**|**`DocNo`**|*** **||||**Header: Document numbers**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vDocNoType`**|**`DocNoType`**|**1 **|**X **|||**Type of document number**|
|**`vDocNo`**|**`DocNo`**|**1 **|**X **|||**Document number**|



This frame element contains the document numbers of the previous documents in the sequence of the
business case and/or additional documents as a reference to the order change.
As needed, specific items in other documents can be referenced in frame element `itmDocNo` .

The indication of the order change number itself as well as of the number of the direct preceding document is
not permitted here because they already are specified in frame element `docHeader` (element
`vOrderChangeNo` resp. `vPrecedingDocNo` ).


**3.8** **Frame element** **`hdrDateTime`** **– Header: Date and time details**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrDateTime`**|**`DateTime`**|**1+**|**X **|**! **||**Header: Date and time details**|



                               - 10 

|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vDateTimeType`**|**`DateTimeType`**|**1 **|**X **|**! **||**Type of date/time**|
|**`vTimeZone`**|**`TimeZone`**|**1 **|**X **|||**Time zone**|
|**`vDateValue`**|**`Date`**|**1 **|**X **|||**Date**|
|**`vTimeValue`**|**`Time`**|**1 **||||**Time**|


This frame element is used to transfer date and time details of the order change header.
At least the document date ( `DOC` ) must be specified.
Furthermore a requested delivery date ( `CRD` ) can be specified for instance.


**3.9** **Frame element** **`hdrOrgData`** **– Header: Organizational data**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrOrgData`**|**`OrgData`**|*** **||**! **||**Header: Organizational data**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vOrgDataType`**|**`OrgDataType`**|**1 **|**X **|**! **||**Type of organizational data**|
|**`vOrgDataValue`**|**`Value`**|**1 **|**X **|||**Value of organizational data**|



Examples of usage: Commission details ( `COM` ) `"Commission Smith"`
Project number ( `PJN` ) `"576134"`
Sales organization ( `SOR` ) `"ABCD"`


**3.10** **Frame element** **`hdrAddress`** **– Header: Addresses**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrAddress`**|**`Address`**|**1+**||**! **||**Header: Addresses**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vAddressType`**|**`AddressType`**|**1 **|**X **|**! **||**Type of address**|
|**`vAddressNumber`**|**`Value`**|**1 **||||**Address number**|
|**`vAddressID`**|**`AddressID`**|*** **||||**Address ID**|
|**`vTitle`**|**`Value`**|**1 **||||**Title**|
|**`vName1`**|**`Name1`**|**1 **|**X **|||**Name 1**|
|**`vName2`**|**`Name2`**|**1 **||||**Name 2**|
|**`vName3`**|**`Name3`**|**1 **||||**Name 3**|
|**`vName4`**|**`Name4`**|**1 **||||**Name 4**|
|**`vStreet`**|**`Street`**|**1 **|**X **|||**Street**|
|**`vStreetNo`**|**`Value`**|**1 **||||**Street number**|
|**`vStreet2`**|**`Street2`**|**1 **||||**Street 2**|
|**`vCountryCode`**|**`CountryCode`**|**1 **|**X **|||**Country code**|
|**`vPostalCode`**|**`PostalCode`**|**1 **|**X **|||**Postal code**|
|**`vLocation`**|**`Location`**|**1 **|**X **|||**Location (city)**|
|**`vDistrict`**|**`District`**|**1 **||||**District**|
|**`vCountyCode`**|**`CountyCode`**|**1 **||||**County/district/state**|
|**`vPostalCodePOBox`**|**`PostalCodePOB`**|**1 **||||**Postal code of P.O. Box**|
|**`vPOBox`**|**`Value`**|**1 **||||**P.O. Box (post-office box)**|
|**`vTaxCode`**|**`Value`**|**1 **||||**Tax number at tax office/authorities**|
|**`vTaxCodeEU`**|**`Value`**|**1 **||||**Sales tax identification number (EU)**|
|**`vTaxCodeUSA`**|**`Value`**|**1 **||||**Sales tax code USA / Jurisdiction**|
|~~**`hdrCom`**~~|~~**`Com`**~~|*** **||||**Header: Communication**|



                               - 11 

~~**`hdrContact`**~~ ~~**`Contact`**~~ ***** **Header: Contacts**

If no ship-to party address (SH) is specified, the address of the sold-to party (SO) or the address of the
master data of the supplier is used.
The business partner must define if the given shipping address is a differing shipping address that is possibly
conditioned differently than the shipment address(es) agreed on. As indicators, e.g. the address number or
the organization type `TRZ` transport zone can be used. The former would be defined by the master data, the
latter by transport zones. (e.g. 1 = zone 1 means no freight costs; 2 = zone 2 means freight costs of 100,00
and will be reflected in the pricing `Pricing` etc.)
In general, especially the addresses of the sold-to party (SO) and the supplier (SU) are known by both
business partners and saved as master data. They need not necessarily be transferred, they correspond to
the customer number or supplier number of the document header ( `docHeader` ).
If applicable, the contact responsible for the order is transmitted with an address.


**3.11** **Frame element** **`hdrCom`** **– Header: Communication**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrCom`**|**`Com`**|*** **||**! **||**Header: Communication**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vComType`**|**`ComType`**|**1 **|**X **|**! **||**Type of communication**|
|**`vComType`**|**Attribute**|**Attribute**|||||
|**`vComType`**|`aScopeInfo`|`aScopeInfo`|**X **|**! **||Scope of information|
|**`vComValue`**|**`Value`**|**1 **|**X **|||**Value of communication**|



For specifying a phone number, fax number, e-mail-address etc. appendant to the address and/or the
contact.


**3.12** **Frame element** **`hdrContact`** **– Header: Contacts**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrContact`**|**`Contact`**|*** **||||**Header: Contacts**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vContactType`**|**`ContactType`**|**1 **|**X **|||**Type of contact**|
|**`vContactNumber`**|**`Value`**|**1 **||||**Contact number**|
|**`vTitle`**|**`Value`**|**1 **||||**Title**|
|**`vFirstName`**|**`FirstName`**|**1 **||||**First name**|
|**`vLastName`**|**`LastName`**|**1 **|**X **|||**Last name**|
|~~**`hdrCom`**~~|~~**`Com`**~~|*** **||||**Header: Communication**|



For specifying contacts, that are required for processing the concerning business case or are
organizationally assigned to it (e.g. one or more sales persons regarding commissions).


**3.13** **Frame element** **`hdrText`** **– Header: Texts**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrText`**|**`Text`**|*** **||**! **||**Header: Texts**|



**Subelement** **Type** **Rec** **M. Key Mod Description**


                               - 12 

|vTextType|TextType|1|X|!|Col6|Type of text|
|---|---|---|---|---|---|---|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|**! **||**Language of text**|
|**`vTextContent`**|**`TextContent`**|**1+**|**X **|||**Content of text**|


**3.14** **Frame element** **`hdrReference`** **– Header: References**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrReference`**|**`Reference`**|*** **||||**Header: References**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vReferenceType`**|**`ReferenceType`**|**1 **|**X **|||**Type of reference**|
|**`vReferenceValue`**|**`Value`**|**1 **|**X **|||**Value of reference**|
|**`vReferenceDesc`**|**`Value`**|**1 **|**X **|||**Description of reference**<br>_(language of document)_|



If attachments of an OEX document are sent in an e-mail, they have to be indicated accordingly. This
enables the application to allocate different attachments to the corresponding OEX document and if
necessary to process them.

Example of usage: Internet link (LNK) to a tracking system
```
           "http://www.harrison-office.com/orderstatus.html?p=1213131"

```

**3.15** **Frame element** **`hdrPricing`** **– Header: Pricing**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrPricing`**|**`Pricing`**|*** **|||**D **|**Header: Pricing**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vConditionType`**|**`ConditionType`**|**1 **|**X **|||**Type of condition**|
|**`vConditionValue`**|**`ConditionValue`**|**1 **|**X **|||**Value of condition**|
|**`vConditionRate`**|**`ConditionRate`**|**1 **||||**Rate of condition**|
|**`vCondCurrency`**|**`CondCurrency`**|**1 **||||**Currency of condition**|
|**`vConditionText`**|**`ConditionText`**|**1 **||||**Description of condition**<br>_(language of document)_|



In this frame element the net total (purchase) of the document items of an order change ( `TNET`, resp. `TNEH` )
is specified (mandatory). This is used as check-sum during processing of the document.
Further details such as discounts are optional, but can be used for control at the processing. In addition,
sales prices can be returned for countercheck.
Unless otherwise stated, the condition currency is pre-defined by the document currency.

Note: In contrast to the price details on order item level the sub elements for price unit and quantity unit in
this frame element have been omitted, because here it’s always about total (sum) conditions.

Example 1 – Specification of the net value of the order change:
Net value of order change item 1 is $ 100,00
Net value of order change item 2 is $ 150,00

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>

```

                               - 13 

Example 2 – Specification of further conditions as total of the order change items:
Gross value of order item 1 is $ 125,00
Discount rate of order item 1 is 20% as basic discount
Net value of order item 1 is $ 100,00
Gross value of order item 2 is $ 200,00
Discount rate of order item 2 is 25% as basic discount
Net value of order item 2 is $ 150,00
In addition to this also the net value (sales) of the precedent order confirmation is quoted, which should
usually match the net value (purchase) of the order change.

```
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="P"> TGRO </vConditionType>
    <vConditionValue> 325.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="2">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 75.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vConditionText> Basic discount </vConditionText>
</hdrPricing>
<hdrPricing aCondNo="3">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="4">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 250.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>

```

The discounts are indicated as absolute total values resulting from the items with the same discount type
( `aTypeDis` ).


**3.16** **Frame element** **`hdrPayment`** **– Header: Terms of payment**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`hdrPayment`**|**`Payment`**|**3***||**! **||**Header: Terms of payment**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vPaymentPart`**|**`PaymentPart`**|**1 **|**X **|**! **||**Part of payment term**|
|**`vPaymentRate`**|**`PaymentRate`**|**1 **|**X **|||**Discount rate (%)**|
|**`vPaymentDays`**|**`PaymentDays`**|**1 **|**X **|||**Number of days (payment target)**|



                               - 14 

**3.17** **Frame element** **`docArticleItem`** **– Document item: Article**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|~~**`docArticleItem`**~~|~~**`Item`**~~|**1+**|**X **|**! **|**D **|**Document item: Article**|
|~~**`docArticleItem`**~~|**Attribute**|**Attribute**|||||
|~~**`docArticleItem`**~~|`aItemNo`|`aItemNo`|X|!||Consecutive number of document item|
|~~**`docArticleItem`**~~|`aAction`|`aAction`|X||D|Action|
|~~**`docArticleItem`**~~|`aUUID`|`aUUID`|X|||Universally Unique Identifier|
|~~**`docArticleItem`**~~|`aIsPseudo`|`aIsPseudo`||||Does the item represent a pseudo<br>article?|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vPrecDocItemNo`**|**`DocItemNo`**|**1 **|**X **|||**Number of item in preceding**<br>**document**<br>_The (direct) preceding document is_<br>_specified in the document header._|
|**`vOrdChgItemNumber`**|**`DocItemNo`**|**1 **|**X **|||**Order change item number**<br>_Unique order item number in an order_<br>_change._|
|**`vOrdChgTopLevelNo`**|**`DocItemNo`**|**1 **||||**Number of higher level order**<br>**change item**|
|**`vOrdChgCompNo`**<br>|**`DocItemNo`**<br>|**1 **|**3 **|||**Number of order change item of the**<br>**composite article** <br>_This reference defines that the article_<br>_is automatically created by the_<br>_composite article._|
|**`vOrdChgSubArtId`** <br>|**`CompSubArtId`** <br>|**1 **|**3 **|||**Identification of the sub article**<br>**for the order change item**<br>_The ID is assigned by the_ _composite_<br>_article._|
|**`vOrdChgAddStateCd`** <br>|**`AddStateCode`** <br>|**1 **||||**Additional state information**<br>**of the order change item**|
|**`vOrdChgAddStateCd2`**|**`AddStateCode`**|**1 **||||**Additional state information**<br>**for encapsulated OFML instance**|
|**`vClientArticleNo`**|**`ClientArtNo`**|**1 **||||**Article number of client**|
|**`vVendorArticleNo`**|**`VendorArtNo`**|**1 **|**X **|||**Article number of vendor (supplier)**|
|**`vVendorID`**|**`VendorID`**|**1 **|**X **|||**Vendor ID**|
|**`vVendorSeries`**|**`VendorSeries`**|**1 **|**X **|||**Vendor Series**|
|**`vCatalogId`**|**`CatalogId`**|**1 **||||**Catalog ID**|
|**`vArticleEAN`**|**`EAN_Article`**|**1 **||||**EAN of article**|
|**`vOrdChgQuantity`**|**`Quantity`**|**1 **|**X **|||**Order quantity**|
|**`vOrdChgUnit`**|**`QuantUnit`**|**1 **|**X **|||**Order unit**|
|**`vGrossWeight`**|**`GrossWeight`**|**1 **|**1 **|||**Gross weight (total)**|
|**`vNetWeight`**|**`NetWeight`**|**1 **|**1 **|||**Net weight (total)**|
|**`vUnitWeight`**|**`UnitWeight`**|**1 **|**1 **|||**Weight unit**|
|**`vVolume`**|**`Volume`**|**1 **|**2 **|||**Volume (total)**|
|**`vUnitVolume`**|**`UnitVolume`**|**1 **|**2 **|||**Volume unit**|
|**`vClassification`**|**`Classification`**|<br>*** **||||**Class/category of order item**|



                               - 15 

|itmConfiguration|Config|*|Col4|Col5|Col6|Item: Configuration details|
|---|---|---|---|---|---|---|
|~~**`itmDocNo`**~~|~~**`DocNo`**~~|*** **||||**Item: Document numbers**|
|~~**`itmDateTime`**~~|~~**`DateTime`**~~|*** **||||**Item: Date and time details**|
|~~**`itmOrgData`**~~|~~**`OrgData`**~~|*** **||||**Item: Organizational data**|
|~~**`itmAddress`**~~|~~**`Address`**~~|*** **||||**Item: Addresses**|
|~~**`itmText`**~~|~~**`Text`**~~|**1+**|**X **|||**Item: Texts**|
|~~**`itmReference`**~~|~~**`Reference`**~~|*** **||||**Item: References**|
|~~**`itmPricing`**~~|~~**`Pricing`**~~|*** **||||**Item: Pricing**|


Basic data of order change item.

If a change was made to at least one sub element compared to the preceding document (see document
header), value `M` must be specified for attribute `aAction` . For the modified sub elements, the attribute must
be set accordingly. If there were no changes, attribute `aAction` can be omitted or value `N` must be specified.

Due to the interaction of the item number ( `vOrdChgItemNumber` ) and the number of the higher-level item
( `vOrdChgTopLevelNo` ) a hierarchy structure (e.g. including sub articles) can be displayed. By referring to a
higher-level item of type `docFolderItem`, also folder structures can be displayed.
Specific item numbers, such as “100.A.10-1“, can be transmitted by the organization data `POS` . However, to
which extent another application can process those, return them or even use them for itself, remains
unsettled.

The additional state information for the encapsulated OFML instance ( `vOrdChgAddStateCd2` ) is required if
the instance that represents the article is encapsulated by a Meta type instance and shares a position with it.
(The code for the Meta type instance then must be specified in `vOrdChgAddStateCd` .)


Explanation of mandatory details:


**1** The **Weight unit** has to be specified as soon as the **Gross weight** and/or the **Net weight** are specified.
**2** The **Volume unit** has to be specified as soon as the **Volume** is specified.
**3** The **Identification of the sub article for the order change item** can only be specified if the **Number of**
**order change item of the composite article** is specified.

At least the short text of a standard article must be specified. A long text can be omitted in this case.
This is different for modified articles and custom articles (compare global OEX value type `VendorArtNo` 
`aStatus` ).


**3.18** **Frame element** **`docSetItem`** **– Document item: Set article**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|~~**`docSetItem`**~~|~~**`Item`**~~|*** **||**! **|**D **|**Document item: Set article**|
|~~**`docSetItem`**~~|**Attribute**|**Attribute**|||||
|~~**`docSetItem`**~~|`aItemNo`|`aItemNo`|X|!||Consecutive number of document item|
|~~**`docSetItem`**~~|`aAction`|`aAction`|X||D|Action|
|~~**`docSetItem`**~~|`aUUID`|`aUUID`|X|||Universally Unique Identifier|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vPrecDocItemNo`**|**`DocItemNo`**|**1 **|**X **|||**Number of item in preceding**<br>**document**<br>_The (direct) preceding document is_<br>_specified in the document header._|
|**`vOrdChgItemNumber`**|**`DocItemNo`**|**1 **|**X **|||**Order change item number**<br>_Unique order item number in an order_<br>_change._|



                               - 16 

|vOrdChgTopLevelNo|DocItemNo|1|Col4|Col5|Col6|Number of higher level order<br>change item|
|---|---|---|---|---|---|---|
|**`vClientArticleNo`**|**`ClientArtNo`**|**1 **||||**Article number of client**|
|**`vVendorArticleNo`**|**`VendorArtNo`**|**1 **|**1 **|||**Article number of vendor (supplier)**|
|**`vVendorID`**|**`VendorID`**|**1 **||||**Vendor ID**|
|**`vVendorSeries`**|**`VendorSeries`**|**1 **||||**Vendor Series**|
|**`vOrdChgQuantity`**|**`Quantity`**|**1 **|**X **|||**Order quantity**|
|**`vOrdChgUnit`**|**`QuantUnit`**|**1 **|**X **|||**Order unit**|
|~~**`itmDocNo`**~~|~~**`DocNo`**~~|*** **||||**Item: Document numbers**|
|~~**`itmOrgData`**~~|~~**`OrgData`**~~|*** **||||**Item: Organizational data**|
|~~**`itmText`**~~|~~**`Text`**~~|*** **|**1 **|||**Item: Texts**|
|~~**`itmReference`**~~|~~**`Reference`**~~|*** **||||**Item: References**|
|~~**`itmPricing`**~~|~~**`Pricing`**~~|*** **||||**Item: Pricing**|


A set article summarizes several articles (sub items) into one position. The price of a set article is
automatically calculated according to the articles contained in it (including quantities and discounts) and
according to the quantity of the set article. If the set position contains **`itmPricing`** sub elements, these only
serve for information, i.e., the prices indicated there are not part of the price calculation at header level
(document).

For the use of attribute `aAction` and of the item numbers, see frame element `docArticleItem` .

Explanation of mandatory details:


**1** If no **vendor** **article number** is specified, the **article short text** has to be specified (sub element
**`itmText`** ).


**3.19** **Frame element** **`docFolderItem`** **– Document item: Folder**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|~~**`docFolderItem`**~~|~~**`Item`**~~|*** **||**! **|**D **|**Document item: Folder**|
|~~**`docFolderItem`**~~|**Attribut**|**Attribut**|||**D **||
|~~**`docFolderItem`**~~|`aItemNo`|`aItemNo`|X|!||Consecutive number of document item|
|~~**`docFolderItem`**~~|`aAction`|`aAction`|X||D|Action|
|~~**`docFolderItem`**~~|`aUUID`|`aUUID`|X|||Universally Unique Identifier|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vPrecDocItemNo`**|**`DocItemNo`**|**1 **||||**Number of item in preceding**<br>**document**<br>_The preceding document is specified in_<br>_the document header._|
|**`vOrdChgItemNumber`**|**`DocItemNo`**|**1 **|**X **|||**Order change item number**<br>_Unique order item number (within the_<br>_order)._|
|**`vOrdChgTopLevelNo`**|**`DocItemNo`**|**1 **||||**Number of higher level order**<br>**change item**|
|**`vFolderName`**|**`Value`**|**1 **|** X**|||**Name of folder**<br>_(in language of document)_|
|**`vFolderIsLOC`**|**`FolderIsLOC`**|**1 **||||**Is the folder name an indication of**<br>**location?**|
|~~**`itmText`**~~|~~**`Text`**~~|*** **||||**Position: Texts**|
|~~**`itmReference`**~~|~~**`Reference`**~~|*** **||||**Position: References**|



For the use of attribute `aAction` and of the item numbers, see frame element `docArticleItem` .


                               - 17 

If the application that creates the document allows the user to explicitly mark the folder name
( `vFolderName` ) as a location description (room text), the folder name should be transferred in the element
`vFolderIsLOC` . In the case of `Y` (yes), the folder name should then also be transferred within sub elements
of type `docArticleItem` and `docSetItem` in a `itmOrgData` element with OrgDataType `LOC` . (This may
facilitate processing in receiving systems that manage room texts for article items)


**3.20** **Frame element** **`docTextItem`** **– Document item: Texts**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|~~**`docTextItem`**~~|~~**`Item`**~~|*** **||**! **|**D **|**Document item: Text**|
|~~**`docTextItem`**~~|**Attribut**|**Attribut**|||**D **||
|~~**`docTextItem`**~~|`aItemNo`|`aItemNo`|X|!||Consecutive number of document item|
|~~**`docTextItem`**~~|`aAction`|`aAction`|X||D|Action|
|~~**`docTextItem`**~~|`aUUID`|`aUUID`|X|||Universally Unique Identifier|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vPrecDocItemNo`**|**`DocItemNo`**|**1 **||||**Number of item in preceding**<br>**document**<br>_The preceding document is specified in_<br>_the document header._|
|**`vOrdChgItemNumber`**|**`DocItemNo`**|**1 **|**X **|||**Order change item number**<br>_Unique order item number (within the_<br>_order)._|
|**`vOrdChgTopLevelNo`**|**`DocItemNo`**|**1 **||||**Number of higher level order**<br>**change item**|
|**`vItemName`**|**`Value`**|**1 **|** X**|||**Name of item**<br>_(in language of document)_|
|~~**`itmText`**~~|~~**`Text`**~~|*** **||||**Position: Texts**|



For the use of attribute `aAction` and of the item numbers, see frame element `docArticleItem` .


**3.21** **Frame element** **`itmConfiguration`** **– Item: Configuration details**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmConfiguration`**|**`Config`**|*** **||||**Item: Configuration details**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vClassID`**|**`Value`**|**1 **||||**Class ID**|
|**`vOptionID`**|**`Value`**|**1 **|**X **|||**Option**|
|**`vOptionEAN`**|**`EAN_Option`**|**1 **||||**EAN of Option ID**|
|**`vValueID`**|**`Value`**|**1 **|**X **|||**Value ID**|
|**`vValueEAN`**|**`EAN_Value`**|**1 **||||**EAN of Value ID**|
|~~**`itmConfigText`**~~|~~**`ConfigText`**~~|*** **||||**Item: Configuration texts**|



                               - 18 

**3.22** **Frame element** **`itmConfigText`** **– Item: Configuration texts**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmConfigText`**|**`ConfigText`**|*** **||||**Item: Configuration texts**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|||**Text language**|
|**`vOptionText`**|**`OptionText`**|**1 **|**X **|||**Option text**|
|**`vValueText`**|**`ValueText`**|*** **||||**Value text**<br>Here, the text is skipped if it is a freely<br>specifiable character value.|



**3.23** **Frame element** **`itmDocNo`** **– Item: Document numbers**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmDocNo`**|**`DocNo`**|*** **||||**Item: Document numbers**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vDocNoType`**|**`DocNoType`**|**1 **|**X **|||**Type of document number**|
|**`vDocNo`**|**`DocNo`**|**1 **|**X **|||**Document number**|
|**`vDocLine`**|**`DocItemNo`**|**1 **||||**Number of document item**|



This frame element contains the item numbers of the previous documents in the sequence of the business
case and/or additional documents as a reference to the order change. The indication of the item numbers
always is necessary except for documents without item details.


**3.24** **Frame element** **`itmDateTime`** **– Item: Date and time details**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmDateTime`**|**`DateTime`**|*** **||**! **||**Item: Date and time details**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vDateTimeType`**|**`DateTimeType`**|**1 **|**X **|**! **||**Type of date/time**|
|**`vTimeZone`**|**`TimeZone`**|**1 **|**X **|||**Time zone**|
|**`vDateValue`**|**`Date`**|**1 **|**X **|||**Date**|
|**`vTimeValue`**|**`Time`**|**1 **||||**Time**|



This frame element is only used, if its details differ from the data of the superior header frame element
`hdrDateTime` or if it is containing additional information relevant for the document item.

**3.25** **Frame element** **`itmOrgData`** **– Item: Organizational data**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmOrgData`**|**`OrgData`**|*** **||**! **||**Item: Organizational data**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vOrgDataType`**|**`OrgDataType`**|**1 **|**X **|**! **||**Type of organizational data**|
|**`vOrgDataValue`**|**`Value`**|**1 **|**X **|||**Value of organizational data**|



                               - 19 

This frame element is only used, if its details differ from the data of the superior header frame element
`hdrOrgData` or if it is containing additional information relevant for the document item.


**3.26** **Frame element** **`itmAddress`** **– Item: Addresses**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmAddress`**|**`Address`**|*** **||**! **||**Item: Addresses**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vAddressType`**|**`AddressType`**|**1 **|**X **|**! **||**Type of address**|
|**`vAddressNumber`**|**`Value`**|**1 **||||**Address number**|
|**`vAddressID`**|**`AddressID`**|**1 **||||**Address ID**|
|**`vTitle`**|**`Value`**|**1 **||||**Title**|
|**`vName1`**|**`Name1`**|**1 **|**X **|||**Name 1**|
|**`vName2`**|**`Name2`**|**1 **||||**Name 2**|
|**`vName3`**|**`Name3`**|**1 **||||**Name 3**|
|**`vName4`**|**`Name4`**|**1 **||||**Name 4**|
|**`vStreet`**|**`Street`**|**1 **|**X **|||**Street**|
|**`vStreetNo`**|**`Value`**|**1 **||||**Street number**|
|**`vStreet2`**|**`Street2`**|**1 **||||**Street 2**|
|**`vCountryCode`**|**`CountryCode`**|**1 **|**X **|||**Country code**|
|**`vPostalCode`**|**`PostalCode`**|**1 **|**X **|||**Postal code**|
|**`vLocation`**|**`Location`**|**1 **|**X **|||**Location (city)**|
|**`vDistrict`**|**`District`**|**1 **||||**District**|
|**`vCountyCode`**|**`CountyCode`**|**1 **||||**County/district/state**|
|**`vPostalCodePOBox`**|**`PostalCodePOB`**|**1 **||||**Postal code of P.O. Box**|
|**`vPOBox`**|**`Value`**|**1 **||||**P.O. Box (post-office box)**|
|**`vTaxCode`**|**`Value`**|**1 **||||**Tax number at tax office/authorities**|
|**`vTaxCodeEU`**|**`Value`**|**1 **||||**Sales tax identification number (EU)**|
|**`vTaxCodeUSA`**|**`Value`**|**1 **||||**Sales tax code USA / Jurisdiction**|
|~~**`itmCom`**~~|~~**`Com`**~~|*** **||||**Item: Communication**|
|~~**`itmContact`**~~|~~**`Contact`**~~|*** **||||**Item: Contacts**|



This frame element is only used, if its details differ from the data of the superior header frame element
`hdrAddress` or if it is containing additional information relevant for the document item.


**3.27** **Frame element** **`itmCom`** **– Item: Communication**

|Element|Type|Rec|Ma|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmCom`**|**`Com`**|*** **||**! **||**Item: Communication**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vComType`**|**`ComType`**|**1 **|**X **|**! **||**Type of communication**|
|**`vComType`**|**Attribut**|**Attribut**|||||
|**`vComType`**|`aScopeInfo`|`aScopeInfo`|**X **|**! **||Scope of information|
|**`vComValue`**|**`Value`**|**1 **|**X **|||**Value of communication**|



This frame element is only used, if its details differ from the data of the superior header frame element
`hdrAddress` or if it is containing additional information relevant for the document item.


                               - 20 

**3.28** **Frame element** **`itmContact`** **– Item: Contacts**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmContact`**|**`Contact`**|*** **||||**Item: Contacts**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vContactType`**|**`ContactType`**|**1 **|**X **|||**Type of contact**|
|**`vContactNumber`**|**`Value`**|**1 **||||**Contact number**|
|**`vTitle`**|**`Value`**|**1 **||||**Title**|
|**`vFirstName`**|**`FirstName`**|**1 **||||**First name**|
|**`vLastName`**|**`LastName`**|**1 **|**X **|||**Last name**|
|~~**`itmCom`**~~|~~**`Com`**~~|*** **||||**Item: Communication**|



This frame element is only used, if its details differ from the data of the superior header frame element
`hdrAddress` or if it is containing additional information relevant for the document item.


**3.29** **Frame element** **`itmText`** **– Item: Texts**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmText`**|**`Text`**|*** **||**! **||**Item: Texts**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vTextType`**|**`TextType`**|**1 **|**X **|**! **||**Type of text**|
|**`vTextLanguage`**|**`TextLanguage`**|**1 **|**X **|**! **||**Language of text**|
|**`vTextContent`**|**`TextContent`**|**1+**|**X **|||**Content of text**|



**3.30** **Frame element** **`itmReference`** **– Item: References**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmReference`**|**`Reference`**|*** **||||**Item: References**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vReferenceType`**|**`ReferenceType`**|**1 **|**X **|||**Type of Reference**|
|**`vReferenceValue`**|**`Value`**|**1 **|**X **|||**Value of Reference**|
|**`vReferenceDesc`**|**`Value`**|**1 **|**X **|||**Description of reference**<br>_(language of document)_|



This frame element is only used, if its details differ from the data of the superior header frame element
`hdrReference` or if it is containing additional information relevant for the document item.


                               - 21 

**3.31** **Frame element** **`itmPricing`** **– Item: Pricing**

|Element|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`itmPricing`**|**`Pricing`**|*** **||||**Item: Pricing**|


|Subelement|Type|Rec|M.|Key|Mod|Description|
|---|---|---|---|---|---|---|
|**`vConditionType`**|**`ConditionType`**|**1 **|**X **|||**Type of condition**|
|**`vConditionValue`**|**`ConditionValue`**|**1 **|**X **|||**Value of condition**|
|**`vConditionRate`**|**`ConditionRate`**|**1 **||||**Rate of condition**|
|**`vCondCurrency`**|**`CondCurrency`**|**1 **||||**Currency of condition**|
|**`vConditionText`**|**`ConditionText`**|**1 **||||**Description of condition**<br>_(language of document)_|
|**`vPriceUnit`**|**`PriceUnit`**|**1 **||||**Price unit**|
|**`vQuantUnit`**|**`QuantUnit`**|**1 **||||**Quantity unit**|



The specification of the net value (purchase) of the order item ( `TNET` ) is mandatory. This is used as checksum during processing of the document.
Other details such as discounts are optional, but can also be used as check during processing. In addition,
sales prices can be returned for countercheck
Unless otherwise stated here, the condition currency is pre-defined by the document currency.
Unless otherwise stated here, the quantity unit is pre-defined by the order quantity unit ( `vOrdChgUnit` ).

Example 1 – Specification of the net value (purchase) of the order change item:
Net unit price of order item is $ 50,00
Order quantity = 2
Order unit = C62

```
<itmPricing aCondNo="1">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 100.00 </vConditionValue>  ! TNET = Order quantity x Net unit price
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>

```

Example 2 – Specification of the net value purchase (client) and sales (supplier) of the order change tem:
Net unit price of order item is $ 50,00
Net unit price of the precedent order confirmation item is $ 50,00
Order quantity = 2
Order unit = C62

```
<itmPricing aCondNo="2">
    <vConditionType aCondArea="P"> TNET </vConditionType> ! TNET = Order quantity x Net unit price
    <vConditionValue> 100.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="1">
    <vConditionType aCondArea="S"> TNET </vConditionType> ! TNET = Order quantity x Net unit price
    <vConditionValue> 100.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>

```

                               - 22 

Example 3 – Specification of several conditions of the order change item:
Gross unit price of order item is $ 50,00 and tax code 1, 19%
Discount (as basic discount) of 20% from gross unit price
Discount (as showroom discount) of 5% from the already discounted price
Order quantity = 2
Order unit = C62
19% VAT
Quoting values of the preceding order confirmation (sales)

```
<itmPricing aCondNo="1">
    <vConditionType aCondArea="P"> SGRO </vConditionType>
    <vConditionValue> 50.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="2">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 10.00 </vConditionValue>
    <vConditionRate> 20.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Basic discount <vConditionText>
    <vPriceUnit> 1 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="3">
    <vConditionType aCondArea="P" aCondRef="2" aTypeDis="D1" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vConditionRate> 5.00 </vConditionRate>
    <vCondCurrency> EUR </vCondCurrency>
    <vConditionText> Showroom discount <vConditionText>
    <vPriceUnit> 1 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="P"> SNET </vConditionType>
    <vConditionValue> 38.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
# Here, the order quantity of 2 pieces takes effect: TNET = SNET x 2
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
# additional indication of the values of the previous order confirmation
<itmPricing aCondNo="8">
    <vConditionType aCondArea="S"> SNET </vConditionType>
    <vConditionValue> 38.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
    <vPriceUnit> 1 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="9">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 76.00 </vConditionValue>
    <vCondCurrency> EUR </vCondCurrency>
</itmPricing>

```

                               - 23 

### **4 Appendix**

**4.1** **History of modification**

|Version|Modification|
|---|---|
|3.1.0 – May 8, 2023| <br>Minor changes and extensions in the introduction<br> <br>Clarifications in the frame elements`hdrPricing` (Header: Pricing) and`itmPricing` (Item: Pricing)<br> <br>New attribute`aIsPseudo` in frame element`docArticleItem` <br> <br>New sub element`itmOrgData` in frame element`docSetItem`. <br> <br>Added element`vFolderIsLOC` in frame element`docFolderItem` plus recommendation to transfer<br>room texts in sub elements in an`itmOrgData` element with OrgDataType`LOC`|
|3.0.0 – 30.11.2017| <br>Global changes according to specification GLOBAL 3.0.0<br> <br>Changes in the structure of this specification<br> <br>Frame element`oexDocument` (Single document): Clarification on the use of attribute`aAction` <br> <br>Frame element`docHeader`: Element`vOrderChangeNo` now has type`DocNo`. Element<br>`vOrderNumber` has been replaced by new elements`vPrecedingDocType` and`vPrecedingDocNo` <br>indicating the (immediate) preceding document.<br> <br>Frame element`hdrDocNo`: The number (ID) of the immediate preceding document may no longer be<br>specified here.<br> <br>Frame element`docItem` renamed`docArticleItem`. <br> <br>Frame element`docArticleItem`: new element`vPrecDocItemNo` for the number of the item in the<br>preceding document. Following elements removed:`vOrderItemNumber`, `vOrderTopLevelNo`, <br>`vOrderComposNo`, `vOrderSubArtId`, `vOrderAddStateCd`, `vOrderQuantity` and`vOrderUnit`. <br> <br>Frame element`docArticleItem`: new (optional) element`vOrdChgAddStateCd2` for additional state<br>code for possibly encapsulated OFML instance.<br> <br>Frame element`docArticleItem`: element`vClientArticleNo` now has type`ClientArtNo` (was<br>`Value`).<br> <br>New frame elements`docFolderItem` (Document item: Folder),`docTextItem` (Document item: Text)<br>and`docSetItem` (Document item: Set article).|
|2.3.1 – 13.1.2017| <br>Correction: element`vOrderChangeNo` (order change number) in frame element`docHeader` (2.8<br>Document header) was substituted for`vOrderConfirmNo`.|
|2.3.0 – 1.7.2015| <br>Global changes according to specification GLOBAL 2.3.0<br> <br>Introduced new optional element`vClassification` in frame element`docItem` (2.19 Document item)<br>for universal specification of categories or classes.|
|2.2.0 – 11.10.2013| <br>Global changes according to specification GLOBAL 2.2.0<br> <br>Introduced new optional elements in frame element`docHeader` (2.8 Document header) for client ID,<br>client classification, supplier ID and supplier classification:`vClientID`, `vClientClass`, `vSupplierID`<br>and`vSupplierClass`. <br>(Elements`vClientILN`and` vVendorILN`were replaced by` vClientID`resp.` vSupplierID`).<br> <br>Introduced new optional elements in frame elements`hdrAddress` (2.12 Header: Addresses) and<br>`itmAddress` (2.25 Item: Addresses) for street 2 und district:`vStreet2`and`vDistrict`. <br>(Element`vAddressILN` was replaced by` vAddressID`).<br> <br>Introduced new optional elements in frame element`docItem` (2.19 Document item) for identification of<br>sub article and additional state information (each for the order item and the order confirmation item) as<br>well as for catalog ID:`vOrderSubArtId` resp.`vOrdConfSubArtId`, `vOrderAddStateCd` resp.<br>`vOrdConfAddStateCd` and`vCatalogId`|
|2.1.0 – 09.02.2010|Initial English version|



                               - 24 

