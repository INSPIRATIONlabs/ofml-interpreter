# Specification OEX OFML Business Data Exchange
## (OFML Part VII)

# **GLOBAL**

General provisions and definitions

# Version 3.1.0

English


Editors:
Markus Behrschmidt, Vitra Services GmbH

Thomas Gerth, EasternGraphicsGmbH


May 8, 2023


Copyright © 2006 - 2023 Industrieverband Büro und Arbeitswelt e. V. (IBA)


# Contents

**1** **Introduction ....................................................................................................... 3**

1.1 Overview of OEX Specifications ......................................................................... 4

1.2 Version Rules ...................................................................................................... 4

1.3 Legend ................................................................................................................ 5


**2** **Definitions ......................................................................................................... 6**

2.1 Types of Elements .............................................................................................. 6
**2.1.1** **Basic Element Types .................................................................................................................. 6**
2.1.1.1 `Frame` : Frame element ................................................................................................................. 6
2.1.1.2 `Value` : Value element .................................................................................................................. 6
2.1.1.3 `Empty` : Attribute element (empty element) ................................................................................... 6
**2.1.2** **OEX Frame Types (** **`Frame`** **) ......................................................................................................... 7**
2.1.2.1 `DocFrame` : OEX document frame ................................................................................................ 7
2.1.2.2 `Applic` : Applikation, die das OEX-Dokument erstellt hat ............................................................ 7
2.1.2.3 `File` : File of documents ............................................................................................................... 8
2.1.2.4 `Document` : Single document ........................................................................................................ 8
2.1.2.5 `Header` : Document header ........................................................................................................... 9
2.1.2.6 `Item` : Document item ................................................................................................................... 9
2.1.2.7 `DateTime` : Date and time details ................................................................................................. 9
2.1.2.8 `OrgData` : Organizational data .................................................................................................... 11
2.1.2.9 `Address` : Addresses .................................................................................................................. 12
2.1.2.10 `Com` : Communication ................................................................................................................... 13
2.1.2.11 `Contact` : Contacts ..................................................................................................................... 13
2.1.2.12 `Text` : Texts ................................................................................................................................. 14
2.1.2.13 `Reference` : References ............................................................................................................ 15
2.1.2.14 `Pricing` : Pricing ........................................................................................................................ 16
2.1.2.15 `Config` : Configuration data ........................................................................................................ 20
2.1.2.16 `ConfigText` : Configuration texts .............................................................................................. 21
2.1.2.17 `Payment` : Terms of payment ...................................................................................................... 22
2.1.2.18 `DocNo` : Document numbers ........................................................................................................ 23
2.1.2.19 `BankData` : Bank data ................................................................................................................. 24
**2.1.3** **OEX Value Types (** **`Value`** **) ........................................................................................................ 25**
**2.1.4** **OEX Attribute Types (** **`Empty`** **) ................................................................................................... 27**

2.2 Data Domains ................................................................................................... 28

2.3 Data Types ....................................................................................................... 42

2.4 Attributes ........................................................................................................... 45


**3** **OEX – Scenarios ............................................................................................. 52**

3.1 Order with follow-up Order Change (ideal case) ............................................... 52

3.2 Order and Order Change (delayed to Order Confirmation) ............................... 52

3.3 Order containing changes caused by vendor .................................................... 53

3.4 From Request to Invoice (ideal case) ............................................................... 53

3.5 From Request to Invoice including Order Change (ideal case) ......................... 54


                                  - 1 

**4** **Appendix ......................................................................................................... 55**

4.1 History of modification ....................................................................................... 55


                                  - 2 

### **1 Introduction**

Business data is exchanged via text files formatted using **XML** (Extensible Markup Language). For different
types of business data (e.g. order), specific document types are defined within OEX and their respective
structure is specified.

This specification contains general provisions for the transmission of OEX documents and definitions of data
type resp. structures applicable to all document types.

The exchange of OEX documents typically is operated by e-mail attachment between agreed e-mail
addresses of both partners. It is possible to send several OEX documents or other attachments, e.g. PDF
files, which are then referenced in the respective OEX document via element type `Reference` with
reference type `ATT` (see 2.1.2.13).

XML Version and Code Page

```
<?xml version="1.0" encoding="UTF-8"?>

```

**UTF-8** (Unicode Transformation Format) is used as standard code page **.** Optionally, the byte order mark can
be specified at the beginning of the file.

Both partners alternatively can agree on following code pages for their data transmission:
ISO-8859-1 (International Standardization Organization) – Latin-1: i.a. West-European code page
ISO-8859-2 (International Standardization Organization) – Latin-2: i.a. Central-European code page

These statements are placed at the beginning of an XML document.


XML Schema (XS) Integration

The structure and data types of the XML-files are defined and verified by an XML schema.
There is one schema per document type. The name of a schema is composed of the prefix `oex`, the
document type (e.g. `orders` for an order), the version number and the file extension `xsd` . Furthermore, the
general schema ( `global` ) is integrated in any document-type related schema.

`oex-<DocumentType>_<Major>.<Minor>.<Build>.xsd` document-type related schema
`oex-global_<Major>.<Minor>.<Build>.xsd` global schema

The integration of the document-type related schema is effected by attributes defined for XML schemes
within the frame element `oexDocFrame` :

```
<oexDocFrame aMajor="3"
xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xsi:noNamespaceSchemaLocation="oex-<DocumentType>_<Major>.<Minor>.<Build>.xsd">

```

                                  - 3 

**1.1** **Overview of OEX Specifications**

The OEX-GLOBAL specification is the leading document with regard to the valid versions of document-type
related specification.

|Document type|Description|Name of specification|XML Schema|Version|
|---|---|---|---|---|
|`GLOBAL`|superior|`oex-global_3.0.0_en.pdf`|`oex-global_3.0.x.xsd`|3.1|
|`REQOTE`|Request|`oex-reqote_3.0.x.pdf`|`oex-reqote_3.0.x.xsd`|3.1|
|`QUOTES`|Quotation|`oex-quotes_3.0.x.pdf`|`oex-quotes_3.0.x.xsd`|3.1|
|`ORDERS`|Order|`oex-orders_3.0.x_en.pdf`|`oex-orders_3.0.x.xsd`|3.1|
|`ORDRSP`|Order confirmation|`oex-ordrsp_3.0.x_en.pdf`|`oex-ordrsp_3.0.x.xsd`|3.1|
|`ORDCHG`|Order change|`oex-ordchg_3.0.x_en.pdf`|`oex-ordchg_3.0.x.xsd`|3.1|
|`DESADV`|Despatch Advice|`oex-desadv_3.0.x.pdf`|`oex-desadv_3.0.x.xsd`|3.1|
|`INVOIC`|Invoice|`oex-invoic_3.0.x.pdf`|`oex-invoic_3.0.x.xsd`|3.1|



The „x“ is a placeholder for the respectively highest build version number of the corresponding specification or XML schema.
(„_en“ refers to the English version of the specification.)


**1.2** **Version Rules**


The version number of all specifications, XML schemas and model files consists of 3 components and is
composed as follows:
Major **2** .3.17
Minor 2. **3** .17
Build 2.3. **17**

All OEX specifications have **major and minor version numbers** in **common** where the specifications with
the respective highest build version number apply.
If, for instance, version **2** . **3** .2 of specification ORDERS is the version with the highest build number within
minor number **2** . **3**, version **2** . **3** .17 of specification GLOBAL has to be applied if this is the version of that
specification with the highest build number within minor number **2** . **3** (see also sample constellation below).

With the build number, varying change states of the specifications are controlled, which do not always
immediately concern all other specifications. A change of GLOBAL, which has effects on the document-type
related specifications but not on the document-specific structures or derived elements, is handled within a
new build version/number. The version number of the document-type related specification remains
unaffected.
Vice versa too, a change of a document-type related specification does not influence the version number of
GLOBAL, if it has no effect on the defined elements and structures in GLOBAL.

As soon as a change of GLOBAL influences elements and structures of at least one existing document type
**all** specifications have to be raised to the next higher minor version number. The build version number then
is reset to zero („0“) for all document types.

Furthermore, changes can lead to the next major version number depending on their extent. Then, the minor
and build version numbers will be reset to zero („0“) for all specifications.

As well, the XML schemas (XSD) and the sample files (XML) have major and minor version numbers in
common, still in order to ensure an unambiguous mapping to the version of the corresponding document
type. Changes in these files require a new build version/number.
Within the XML file, the version of the corresponding document type and the associated XML schema the
XML file refers to, are indicated. Within the XSD file, the general XML schema it refers to, is indicated. Here
too, the respectively highest build version numbers apply.


                                  - 4 

A sample constellation of the versions for ORDERS (order):
Specification OEX-ORDERS **2.3.2** oex-orders_2.3.2_en.pdf
Specification OEX-GLOBAL **2.3.17** oex-global_2.3.17_en.pdf
Sample file ORDERS **2.3.5** oex-orders-sample_2.3.5.xml
XML scheme ORDERS **2.3.4** oex-orders_2.3.4.xsd
XML scheme GLOBAL **2.3.8** oex-global_2.3.8.xsd


**1.3** **Legend**
Explanation of specific columns used in the tables in chapter 2 “Definitions“.











|Column|Description|Values|Col4|Meaning|
|---|---|---|---|---|
|**`Rec`**|Recurrence|**1 **|**1 **|Element appears exactly once|
|**`Rec`**|Recurrence|**#+**|**#+**|Element has to appear minimum**#** times or more.<br>„**#**“ is a placeholder for any number.<br>(Example: 1+ = „must“ 1 time, „can“ several<br>times)|
|**`Rec`**|Recurrence|**#***|**#***|Element can appear 0 to several times, up to<br>maximum**#** times, where „**#**“ is a placeholder for<br>any number. If the element is a mandatory<br>element, it must occur at least once. (Ex.: 3* = 1<br>to 3 times)|
|**`Rec`**|Recurrence|*** **|*** **|Element can appear 0 to several times. If the<br>element is a mandatory element, it must occur at<br>least once.|
|**`M.`**<br>**`Mandat.`**|Mandatory element|**<empty>**|**<empty>**|Element may be available. If it is available it must<br>contain a value.|
|**`M.`**<br>**`Mandat.`**|Mandatory element|**X **|**X **|Element must be available and contain a value.|
|**`M.`**<br>**`Mandat.`**|Mandatory element|**# **|**# **|Element may be available. If it is available it must<br>contain a value. The placeholder**#** stands for a<br>consecutive number, starting with 1 for<br>subelements within a frame element which are<br>mutually dependent and in general have to be<br>indicated in combination.<br>(e.g. qantity and quantity unit)|
|**`Len`**|(Maximal) length of the<br>data domain (inclusive<br>decimals and<br>separator). Signs do<br>not contribute to the<br>length of numeric<br>values. (`NUM`)|**1 – n**|**1 – n**|From 1 to “infinite“|
|**`Len`**|(Maximal) length of the<br>data domain (inclusive<br>decimals and<br>separator). Signs do<br>not contribute to the<br>length of numeric<br>values. (`NUM`)|*** **|*** **|Any (common in relation to data domain)|
|**`Len`**|(Maximal) length of the<br>data domain (inclusive<br>decimals and<br>separator). Signs do<br>not contribute to the<br>length of numeric<br>values. (`NUM`)|**<empty>**|**<empty>**|For certain data types|
|**`Dec`**|Decimals|**1 – n**|**1 – n**|From 1 to “infinite“|
|**`Dec`**|Decimals|**<empty>**|**<empty>**|No decimals|
|**`Sep`**|Decimal separator||**. **|Usually decimal point|
|**`Sep`**|Decimal separator|**<empty>**|**<empty>**|No deciamal separator|
|**`Restrict.`**|Restrictions for value<br>tables|||See data domains|


Others:

OCD OFML Commercial Data


                                  - 5 

### **2 Definitions**

**2.1** **Types of Elements**
Used elements are typed, where the basic XML elements are related to basic element types and the OEX
elements based on it are related to OEX element types.
Every type is related to a data domain (abbr.: domain), which describes a type exactly.

Naming: Starting with a capital letter.


**2.1.1** **Basic Element Types**
Basic element types form the arrangement of the XML-elements and are the base for OEX-element types.


**2.1.1.1** **`Frame`** **: Frame element**

|Basic element type|Description/Explanation|Col3|
|---|---|---|
|**`Frame`**|Frame element, can contain attributes and subelements.<br>Basic domain: _`Frame` <br>Naming of these elements: Any three-digit prefix**`abc`** <br>E.g.:**`<oexFileaDocCount=`**`"`**`5`**`"`**`>`**`[subelements]`**`</oexFile>` **|Frame element, can contain attributes and subelements.<br>Basic domain: _`Frame` <br>Naming of these elements: Any three-digit prefix**`abc`** <br>E.g.:**`<oexFileaDocCount=`**`"`**`5`**`"`**`>`**`[subelements]`**`</oexFile>` **|
|**`Frame`**|**Subelements**|**Description**|
|**`Frame`**|`Frame`|Frame element|
|**`Frame`**|`Value`|Value element|
|**`Frame`**|`Empty`|Attribute (empty) element|



**2.1.1.2** **`Value`** **: Value element**

|Basic element type|Description/Explanation|
|---|---|
|**`Value`**|Value element, can contain attributes.<br>Basic domain: _`Value` <br>Naming of these elements: Prefix**`v`** (value)<br>E.g.:**`<vDocumentTypeaMajor=`**`"`**`3`**`" aMinor="`**`0`**`" `<br>`aBuild="`**`0`**`"`**`>ORDERS</vDocumentType>`**|



**2.1.1.3** **`Empty`** **: Attribute element (empty element)**

|Basic element type|Description/Explanation|
|---|---|
|**`Empty`**|Empty element, contains only attributes<br>Basic domain: _`Attribute` <br>Naming of these elements: Prefix**`e`** (empty)<br>E.g.:**<****`eAppVersion aMajor=`**`"`**`2`**`"`**` aMinor=`**`"`**`0`**`"`**`/>` **|



                                  - 6 

**2.1.2** **OEX Frame Types (** **`Frame`** **)**

All frame elements are based on the basic type `Frame` .
Note: Elements, which are in pointed brackets, have a variable naming (e.g. <Document>) and can contain
variable subelements <*>. They are defined refering to document types.


**2.1.2.1** **`DocFrame`** **: OEX document frame**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`DocFrame`**|**`_DocFrame`**|||||**OEX document frame**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<Applic>`**|**`Applic`**|**1 **|**X **|||**Application which has created the**<br>**document**|
|~~**`<File>`**~~|~~**`File`**~~|**1 **|**X **|||**File of documents**|



`DocFrame` is the main frame of every OEX-XML-document.
For example, the XML-schema (XSD) consistant with this document type is integrated by the attributes of
`DocFrame` .

Example:

```
<oexDocFrame aMajor="2" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xsi:noNamespaceSchemaLocation="oex-orders_3.0.0.xsd">
    <oexApplication>
       <vAppName> MyOrderEntryApplication </vAppName>
       <eAppVersion aMajor="7" aMinor="3"/>
    </oexApplication>
    <oexFile aDocumentCount="1">
       <vDocumentType aMajor="3" aMinor="0" aBuild="0"> ORDERS </vDocumentType>
       <... 1 Document ...>
    </oexFile>
</oexDocFrame>

```

**2.1.2.2** **`Applic`** **: Applikation, die das OEX-Dokument erstellt hat**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Applic`**|**`_Frame`**|||||**Application which has created the**<br>**document**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<AppName>`**|**`Value`**|**1 **|**X **|||**Name of application**|
|**`<AppVersion>`**|**`Version`**|**1 **|**X **|||**Version of application**|



`Applic` conduces to the identification of the application which creates the OEX-document.

Example:

```
<oexApplication>
       <vAppName> MyOrderEntryApplication </vAppName>
    <eAppVersion aMajor="7" aMinor="3"/>
</oexApplication>

```

                                  - 7 

**2.1.2.3** **`File`** **: File of documents**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`File`**|**`_File`**|||||**File of documents**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<DocumentType>`**|**`DocumentType`**|**1 **|**X **|||**Type of document**|
|~~**`<Document>`**~~|~~**`Document`**~~|**1+**|**X **|||**Single document**|



A document file can only contain several documents ( `Document` ) of one document type and version.
Therefore, a mixture of documents of different document types like e.g. `ORDERS` (order) and `ORDCHG` (order
change) is not permissible.

Example:
File of documents including 4 documents

```
<oexFile aDocumentCount="4">
    <vDocumentType aMajor="3" aMinor="0" aBuild="0"> ORDERS </vDocumentType>
    <oexDocument aDocNo="1" aItemCount="5">
       <... Content of document 1 (document-type related) ...>
    </oexDocument>
    <oexDocument aDocNo="2" aItemCount="2">
       <... Content of document 2 (document-type related) ...>
    </oexDocument>
    <oexDocument aDocNo="3" aItemCount="1">
       <... Content of document 3 (document-type related) ...>
    </oexDocument>
    <oexDocument aDocNo="4" aItemCount="3">
       <... Content of document 4 (document-type related) ...>
    </oexDocument>
</oexFile>

```

**2.1.2.4** **`Document`** **: Single document**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Document`**|**`_Document`**|||||**Single document**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<*>`**|**`* `**|*** **||||**document-type related**|



Example:
2 documents each of them containing Document Header and a different amount of Document Items

```
<oexDocument aDocNo="1" aItemCount="3" aAction="C">
    <docHeader aAction="C">
       <... Content of Document Header (document-type related) ...>
    </docHeader>
    <docItem aItemNo="1" aAction="C">
       <... Content of Document Item (document-type related) ...>
    </docItem>
    <docItem aItemNo="2" aAction="C">
       <... Content of Document Item (document-type related) ...>
    </docItem>
    <docItem aItemNo="3" aAction="C">
       <... Content of Document Item (document-type related) ...>
    </docItem>
</oexDocument>
<oexDocument aDocNo="2" aItemCount="1">
    <docHeader aAction="C">
       <... Content of Document Header (document-type related) ...>
    </docHeader>
    <docItem aItemNo="1" aAction="C">
       <... Content of Document Item (document-type related) ...>
</oexDocument>

```

                                  - 8 

**2.1.2.5** **`Header`** **: Document header**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Header`**|**`_Header`**|||||**Document header**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<*>`**|**`* `**|*** **||||**document-type related**|



Example:

```
<docHeader aAction="C">
    <... Content of Document Header (document-type related) ...>
</docHeader>

```

**2.1.2.6** **`Item`** **: Document item**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Item`**|**`_Item`**|||||**Document item**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<*>`**|**`* `**|*** **||||**document-type related**|



Example:
2 Document items

```
<docItem aItemNo="1" aAction="C">
    <... Content of Document Item (document-type related) ...>
</docItem>
<docItem aItemNo="2" aAction="C">
    <... Content of Document Item (document-type related) ...>
</docItem>

```

**2.1.2.7** **`DateTime`** **: Date and time details**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`DateTime`**|**`_DateTime`**|||||**Date and time details**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<DateTimeType>`**|**`DateTimeType`**|**1 **|**X **|||**Type of date/time**|
|**`<TimeZone>`**|**`TimeZone`**|**1 **|**X **|||**Time zone**|
|**`<DateValue>`**|**`Date`**|**1 **|**X **|||**Date**|
|**`<TimeValue>`**|**`Time`**|**1 **||||**Time**|



Date and time are indicated corresponding to the respective time zone (time lag).

Examples:

Document date on August 9th, 2006 at 2:35 p.m. Central European summer time (CEST) in the document
header:

```
<hdrDateTime>
    <vDateTimeType> DOC </vDateTimeType>
    <vTimeZone> +0200 </vTimeZone>
    <vDateValue aDateFormat="D"> 20060809 </vDateValue>
    <vTimeValue> 143500 </vTimeValue>
</hdrDateTime>

```

                                  - 9 

Document date on December 22nd, 2006 at 07:43 a.m. Western European Time (WET) in the document
header:

```
<hdrDateTime>
    <vDateTimeType> DOC </vDateTimeType>
    <vTimeZone> +0000 </vTimeZone>
    <vDateValue aDateFormat="D"> 20061222 </vDateValue>
    <vTimeValue> 074300 </vTimeValue>
</hdrDateTime>

```

Requested delivery date week 8/2006 Central European Time (CET) in the document header:

```
<hdrDateTime>
    <vDateTimeType> CRD </vDateTimeType>
    <vTimeZone> +0100 </vTimeZone>
    <vDateValue aDateFormat="W"> 200608 </vDateValue>
</hdrDateTime>

```

Order date on October 28th, 2006 at 11:27 a.m. New York winter time (Eastern Standard Time EST) in the
document header:

```
<hdrDateTime>
    <vDateTimeType> ORD </vDateTimeType>
    <vTimeZone> -0500 </vTimeZone>
    <vDateValue aDateFormat="D"> 20061028 </vDateValue>
    <vTimeValue> 112700 </vTimeValue>
</hdrDateTime>

```

Determination of the requested delivery date with specification of 10 calendar days at order entry:

```
<hdrDateTime>
    <vDateTimeType> DLD </vDateTimeType>
    <vTimeZone> +0200 </vTimeZone>
    <vDateValue aDateFormat="C" aDateCalcBase="*DIO" aDateCalcMode="+"> 0010 </vDateValue>
</hdrDateTime>

```

At order entry on July 1st, 2009, the requested delivery date would be **July 11th, 2009** .

|Juli 2009|Col2|Col3|Col4|Col5|Col6|Col7|Col8|
|---|---|---|---|---|---|---|---|
|**KW**|** Mo**|**Di**|**Mi**|**Do**|**Fr**|**Sa**|**So**|
|**27**|||**1 **|**2 **|**3 **|**4 **|**5 **|
|**28**|**6 **|**7 **|**8 **|**9 **|**10**|**11**|**12**|
|**29**|**13**|**14**|**15**|**16**|**17**|**18**|**19**|
|**30**|**20**|**21**|**22**|**23**|**24**|**25**|**26**|
|**31**|**27**|**28**|**29**|**30**|**31**|||



Determination of the delivery date with specification of 14 calendar days after the date of order confirmation:

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

                               - 10 

Calculation basis is the previous frame element with the order confirmation date 01.07.2009. The delivery date would therefore be **July**
**15th, 2009** .

|Juli 2009|Col2|Col3|Col4|Col5|Col6|Col7|Col8|
|---|---|---|---|---|---|---|---|
|**KW**|** Mo**|**Di**|**Mi**|**Do**|**Fr**|**Sa**|**So**|
|**27**|||**1 **|**2 **|**3 **|**4 **|**5 **|
|**28**|**6 **|**7 **|**8 **|**9 **|**10**|**11**|**12**|
|**29**|**13**|**14**|**15**|**16**|**17**|**18**|**19**|
|**30**|**20**|**21**|**22**|**23**|**24**|**25**|**26**|
|**31**|**27**|**28**|**29**|**30**|**31**|||



**2.1.2.8** **`OrgData`** **: Organizational data**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`OrgData`**|**`_OrgData`**|||||**Organizational data**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<OrgDataType>`**|**`OrgDataType`**|**1 **|**X **|||**Type of organizational data**|
|**`<OrgDataValue>`**|**`Value`**|**1 **|**X **|||**Value of organizational data**|



For possible organizational data for the data exchange, see domain `_OrgDataType` .

Examples:

Indication of a commission in the document header.

```
<hdrOrgData>
    <vOrgDataType> COM </vOrgDataType>
    <vOrgDataValue> Commission Smith </vOrgDataValue>
</hdrOrgData>

```

Indication of a project number in the document header

```
<hdrOrgData>
    <vOrgDataType> PJN </vOrgDataType>
    <vOrgDataValue> 65789198789 </vOrgDataValue>
</hdrOrgData>

```

Indication of an edited item number in the document item

```
<itmOrgData>
    <vOrgDataType> POS </vOrgDataType>
    <vOrgDataValue> 100.A.10-1 </vOrgDataValue>
</itmOrgData>

```

                               - 11 

**2.1.2.9** **`Address`** **: Addresses**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Address`**|**`_Address`**|||||**Addresses**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<AddressType>`**|**`AddressType`**|**1 **|**X **|||**Type of address**|
|**`<AddressNumber>`**|**`Value`**|**1 **||||**Address number**|
|**`<AddressID>`**|**`AddressID`**|**1 **||||**ILN of address**|
|**`<Title>`**|**`Value`**|**1 **||||**Title**|
|**`<Name1>`**|**`Name1`**|**1 **|**X **|||**Name 1**|
|**`<Name2>`**|**`Name2`**|**1 **||||**Name 2**|
|**`<Name3>`**|**`Name3`**|**1 **||||**Name 3**|
|**`<Name4>`**|**`Name4`**|**1 **||||**Name 4**|
|**`<Street>`**|**`Street`**|**1 **|**X **|||**Street**|
|**`<StreetNo>`**|**`Value`**|**1 **||||**Street number**|
|**`<Street2>`**|**`Street2`**|**1 **||||**Street 2**|
|**`<CountryCode>`**|**`CountryCode`**|**1 **|**X **|||**Country code**|
|**`<PostalCode>`**|**`PostalCode`**|**1 **|**X **|||**Postal code**|
|**`<Location>`**|**`Location`**|**1 **|**X **|||**Location (city)**|
|**`<District>`**|**`District`**|**1 **||||**District**|
|**`<CountyCode>`**|**`CountyCode`**|**1 **||||**County/district/state**|
|**`<PostalCodePOBox>`**|**`PostalCodePOB`**|**1 **||||**Postal code of P.O. Box**|
|**`<POBox>`**|**`Value`**|**1 **||||**P.O. Box (post-office box)**|
|**`<TaxCode>`**|**`Value`**|**1 **||||**Tax number at tax office/authorities**|
|**`<TaxCodeEU>`**|**`Value`**|**1 **||||**Sales tax identification number (EU)**|
|**`<TaxCodeUSA>`**|**`Value`**|**1 **||||**Sales tax code USA / Jurisdiction**|
|~~**`<Com>`**~~|~~**`Com`**~~|*** **||||**Communication**|
|~~**`<Contact>`**~~|~~**`Contact`**~~|*** **||||**Contacts**|



Example:
Address of sold-to party

```
<hdrAddress>
    <vAddressType> SO </vAddressType>
    <vAddressNumber> 2222222 </vAddressNumber>
    <vName1> Harrison Office Inc. </vName1>
    <vName2> The office experts </vName2>
    <vStreet> Central Road </vStreet>
    <vStreetNo> 11 </vStreetNo>
    <vCountryCode> US </vCountryCode>
    <vPostalCode> 10001 </vPostalCode>
    <vLocation> New York </vLocation>
    <vCountyCode> NY </vCountyCode>
    <vPostalCodePOBox> 456789 </vPostalCodePOBox>
    <vPOBox> 131343654 </vPOBox>
    <vTaxCodeUS> 3306120100 </vTaxCodeUS>
    <hdrCom>
       <vComType aScopeInfo="B"> TEL </vComType>
       <vComValue> +1-89-123456 </vComValue>
    </hdrCom>
    <hdrCom>
       <vComType aScopeInfo="B"> FAX </vComType>
       <vComValue> +1-89-123457 </vComValue>
    </hdrCom>
    <hdrCom>
       <vComType aScopeInfo="B"> WWW< /vComType>
       <vComValue> http://www.harrison-office.com </vComValue>
    </hdrCom>
    <hdrContact>
       <vContactType> SC </vContactType>
       <vContactNumber> 333333 </vContactNumber>
       <vTitle> Mr. </vTitle>
       <vFirstName> John </vFirstName>
```

                               - 12 

```
       <vLastName> Miller </vLastName>
       <hdrCom>
           <vComType aScopeInfo="B"> TEL </vComType>
           <vComValue> +1-89-123456 </vComValue>
       </hdrCom>
       <hdrCom>
           <vComType aScopeInfo="B"> EMA </vComType>
           <vComValue> John.Miller@harrison-office.com </vComValue>
       </hdrCom>
    </hdrContact>
</hdrAddress>

```

**2.1.2.10** **`Com`** **: Communication**

**OEX-Element type** **Domain** **Description**
**`Com`** **`_Frame`** **Communication**

|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<ComType>`**|**`ComType`**|**1 **|**X **|||**Type of communication**|
|**`<ComValue>`**|**`Value`**|**1 **|**X **|||**Value of communication**|



Example:
Business phone number within the document header

```
<hdrCom>
    <vComType aScopeInfo="B"> TEL </vComType>
    <vComValue> +1-1234-5678910 </vComValue>
</hdrCom>

```

**2.1.2.11** **`Contact`** **: Contacts**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Contact`**|**`_Frame`**|||||**Contacts**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<ContactType>`**|**`ContactType`**|**1 **|**X **|||**Type of contact**|
|**`<ContactNumber>`**|**`Value`**|**1 **||||**Contact number**|
|**`<Title>`**|**`Value`**|**1 **||||**Title**|
|**`<FirstName>`**|**`FirstName`**|**1 **||||**First name**|
|**`<LastName>`**|**`LastName`**|**1 **|**X **|||**Last name**|
|~~**`<Com>`**~~|~~**`Com`**~~|*** **||||**Communication**|



By means of the „Type of contact“, various persons can be transferred, who are directly (e.g. sales support)
or organizationally (e.g. sales representative) involved in a business case.
The number of the contact can be used as identifier. Then, it must be known by both business partners.

Example:
Contact sales representative with business telephone number and e-mail address in the document header

```
<hdrContact>
    <vContactType> SC </vContactType>
    <vContactNumber> 333333 </vContactNumber>
    <vTitle> Mr. </vTitle>
    <vFirstName> John </vFirstName>
    <vLastName> Miller </vLastName>
    <hdrCom>
       <vComType aScopeInfo="B"> TEL </vComType>
       <vComValue> +1-89-123456 </vComValue>
    </hdrCom>
    <hdrCom>
       <vComType aScopeInfo="B"> EMA </vComType>
       <vComValue> John.Miller@harrison-office.com </vComValue>
    </hdrCom>
</hdrContact>
```

                               - 13 

**2.1.2.12** **`Text`** **: Texts**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Text`**|**`_Text`**|||||**Texts**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<TextType>`**|**`TextType`**|**1 **|**X **|||**Type of text**|
|**`<TextLanguage>`**|**`TextLanguage`**|**1 **|**X **|||**Language of text**|
|**`<TextContent>`**|**`TextContent`**|**1+**|**X **|||**Content of text**|



The text structure is according to OCD as of version 4.
Texts are put unformatted in one or more text lines. Control characters for line breaks, tabulators, character
formattings etc. are not permitted.
The respective application must ensure that the text is written according to the specification when compiling
the XML text elements.
If several lines are permitted for one text type the element `TextContent` in the frame type `Text` is
accordingly repeated and thereby, the attribute `aTextLineNo` is incremented for the line number.
For every new text type or every new language within a text type the line numbering restarts with 1 (see
Type `TextContent` attribute `aTextLineNo` )
Note: A new language can already differ from an existing language by the attribute `aLocale` (locale). See
example with long text in one language but two locales.

When reading the lines from the XML text elements into a processing application, the attribute „line
format“ controls how a text is imported: either as single lines or as a continuous text (see type `TextContent`
attribute `aLineFormat` ).

Example: article long text ( `ARTL` ) for representation with automatic word-wrapping:
```
<itmText>
    <vTextType> ARTL </vTextType>
    <vTextLanguage> en </vTextLanguage>
    <vTextContent aTextLineNo="1" aLineFormat="\"> Office desk XYZ, </vTextContent>
    <vTextContent aTextLineNo="2" aLineFormat="~"> height-adjustable, base chromed. </vTextContent>
</itmText>

```

Expected representation in a text editor of an order entry application:
Office desk XYZ, height-adjustable, base chromed.

Note: The application may insert a line break depending on the length of the field for the text content.

Example: long text ( `ARTL` ) in 2 languages with required word wrap and a short text ( `ARTS` ):
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
    <vTextLanguage> en </vTextLanguage>
    <vTextContent aTextLineNo="1" aLineFormat="\"> Office desk XYZ </vTextContent>
</itmText>

```

Expected representation in a text editor of an order entry application:
Office desk XYZ,
height-adjustable, base chromed.

Note: The application may insert an additional line break depending on the length of the field for the text
content.


                               - 14 

Example: long text ( `ARTL` ) in one language, but 2 locales (= 2 laguage versions):
American English (enUS) and British English (enGB)
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

Note: The number of text lines of a text type depends on the respective language.


**2.1.2.13** **`Reference`** **: References**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Reference`**|**`_Reference`**|||||**References**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<ReferenceType>`**|**`ReferenceType`**|**1 **|**X **|||**Type of reference**|
|**`<ReferenceValue>`**|**`Value`**|**1 **|**X **|||**Value of reference**|
|**`<ReferenceDesc>`**|**`Value`**|**1 **|**X **|||**Description of reference**<br>_(language of document)_|



References to links, attachments or other documents. As a value of the reference, a complete URL (link) or a
complete file name (attachment) is indicated corresponding to the reference type.
A description is mandatory, because it is used by applications to display links or drop-down lists.

Examples:


Link to show the order status in the document header:

```
<hdrReference>
    <vReferenceType aMIMEType="text/html"> LNK </vReferenceType>
    <vReferenceValue> http://www.harrison-office.com/orderstatus.html?p=1213131 </vReferenceValue>
    <vReferenceDesc> Order status </vReferenceDesc>
</hdrReference>

```

Embedded product image:

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
```

                               - 15 

```
Iw5mnnCS/uj0LTHZXtPIQtHxHZfyfEU4ApFcysvYeds3ad5JGuCz9Ju2TxRzswbIK05DscOorZ8T2DtR0AX8Dc17pGaXKNr7Z1PG
b9QgPH1l1vzWZ9xynD1m2/4j021cMzJbqMvG4sYfFcPssK99XiXu2yzcY6e4YhrZ3/wXt/Mvbly7NRCKUUVCFrTtClEFl9pbv70Y
KsP0fTn96FpWaiu3kyNW7hvR3bbcKg8KaIf6cLbonq8pkapnDGjMNRAOtZkOnWUH6ULG9SyUTbyuRFEUooPmR4DXFoxpvVKkmuJ2
qEQUIiApBooUoN1w7d+HI+1ccD7SP8AOPSu90O/MUrXbRsIOwjeDzFeVxSvikZLH+pGczfi612mlX7JGRysPYeARzcy69L+YzeWx
0LQf7B7y7NkDSNLv47iWwdjQNMZc+Gp+VERTb3aFeqrkdDubW5ltjcislrIZbaTYWPcx0LupzHkEfEutWO0y41LqURFlRERAREQE
REBERAREQfMSIiIKERAUoiBVbLRr4wTeC4+zkNWczt460RXru/iV2ulX5Y5tCvQtF1Jt3AGOPtGjDnCIunfPP6nX62alEXJsREQE
REBERAREQEREH//2Q== </vReferenceValue>
    <vReferenceDesc> Product image </vReferenceDesc>
</hdrReference>

```

**2.1.2.14** **`Pricing`** **: Pricing**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Pricing`**|**`_Pricing`**|||||**Pricing**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<ConditionType>`**|**`ConditionType`**|**1 **|**X **|||**Type of condition**|
|**`<ConditionValue>`**|**`Condition`**|**1 **|**X **|||**Value of condition**|
|**`<ConditionRate>`**|**`ConditionRate`**|**1 **||||**Rate of condition**|
|**`<CondCurrency>`**|**`CondCurrency`**|**1 **||||**Currency of condition**<br>_If not stated otherwise, currency of the_<br>_document is assumed._|
|**`<ConditionText>`**|**`ConditionText`**|**1 **||||**Description of condition**<br>_related to it’s condition type and as the_<br>_case may be to the type of surcharge_<br>_or discount (in document language)._<br>_Entering the condition rate_<br>_(_`ConditionRate`_) anew in the_<br>_description is not permitted._|
|**`<PriceUnit>`**|**`PriceUnit`**|**1 **||||**Price unit**<br>Unit, which the condition value relates<br>to (e.g. unit price).<br>Examples:**1** if price per unit or**10** if<br>price per 10 units)<br>_If not stated otherwise, 1 is assumed._<br>_Not valid for total conditions (sum) or if_<br>_a condition type refers to a total_<br>_condition._|
|**`<QuantUnit>`**|**`QuantUnit`**|**1 **||||**Quantity unit**<br>for price unit <br>_If not stated otherwise, order unit of the_<br>_order item is assumed._<br>_Not valid for total conditions (sum) or if_<br>_a condition type refers to a total_<br>_condition._|



Quoting a different quantity unit for a price than the order unit implies that the receiving application is working
with the same conversion rules.
The same occurs when using another currency than the document currrency. Also here, the receiving
application must be able to convert the value with the corresponding exchange rate.
Different merchandise management systems or ERP systems permit on header level so-called header
discounts `"DISH"` (discounts) or header surcharges `"SURH"` without breaking those down into the items and
displaying them there as discounts. As a consequence, sums which have been calculated before from
the items (e.g. `TNET` ) do not correspond to the total sum `"TNEH"` after header discounts and/or header
surcharges (compare also the following example 1).
This also applies to the value added tax. The tax-relevant net value ( `TTNE` ) has to be calculated on header
level corresponding to the header surcharges and discounts.


                               - 16 

Example 1 – Entire Scenario for purchase price of an order consisting of 2 items:

Order item 1: Order item 2:
Gross unit price (listed price) $ 50,00 Gross unit price (listed price) $ 20,00
Order quantity 2 Order quantity 1
Tax 19 % Reduced tax 7 %
Discount 1 (as basic discount) 20 % Absolute discount (as special discount) € 2,00
Discount 2 (as other discounts 1) 5 % from discounted value

Order header:
Header discount (as other discounts 2) 10 %

```
<!-- Header /-->
<hdrPricing aCondNo="1">
    <vConditionType aCondArea="P"> TGRO </vConditionType>
    <vConditionValue> 120.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="2">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 20.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vConditionText> Basic discount <vConditionText>
</hdrPricing>
<hdrPricing aCondNo="3">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="D1" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 4.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vConditionText> Show room discount <vConditionText>
</hdrPricing>
<hdrPricing aCondNo="4">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="SD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vConditionText> Special discount <vConditionText>
</hdrPricing>
<hdrPricing aCondNo="5">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 94.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="6">
    <vConditionType aCondArea="P" aCondRef="5" aTypeDis="D2" aCondSign="-"> DISH </vConditionType>
    <vConditionValue> 9.40 </vConditionValue>
    <vConditionRate> 10.00 </vConditionRate>
    <vCondCurrency> USD </vCondCurrency>
    <vConditionText> Promotion discount <vConditionText>
</hdrPricing>
<hdrPricing aCondNo="7">
    <vConditionType aCondArea="P"> TNEH </vConditionType>
    <vConditionValue> 84.60 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="8">
    <vConditionType aCondArea="P" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 68.40 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="9">
    <vConditionType aCondArea="P" aCondRef="8" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue> 13.00 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>
<hdrPricing aCondNo="10">
    <vConditionType aCondArea="P" aTaxCode="2"> TTNE </vConditionType>
    <vConditionValue> 16.20 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>

```

                               - 17 

```
<hdrPricing aCondNo="11">
    <vConditionType aCondArea="P" aCondRef="10" aTaxCode="2"> TTAX </vConditionType>
    <vConditionValue aCondValType="P"> 1.13 </vConditionValue>
    <vConditionRate> 7.00 </vConditionRate>
    <vCondCurrency> USD </vCondCurrency>
<hdrPricing>
<hdrPricing aCondNo="12">
    <vConditionType aCondArea="P"> TOTL </vConditionType>
    <vConditionValue> 98.73 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</hdrPricing>

<!-- Item 1 /-->
<vOrderQuantity>2</vOrderQuantity>
<itmPricing aCondNo="1">
    <vConditionType aCondArea="P"> SGRO </vConditionType>
    <vConditionValue> 50.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="2">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 10.00 </vConditionValue>
    <vConditionRate> 20.00 </vConditionRate>
    <vCondCurrency> USD </vCondCurrency>
    <vConditionText> Basic discount <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="3">
    <vConditionType aCondArea="P" aCondRef="2" aTypeDis="D1" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vConditionRate> 5.00 </vConditionRate>
    <vConditionText> Show room discount <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="P"> SNET </vConditionType>
    <vConditionValue> 38.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="5">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 76.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="6">
    <vConditionType aCondArea="P" aTaxCode="1"> TTNE </vConditionType>
    <vConditionValue> 76.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="7">
    <vConditionType aCondArea="P" aCondRef="6" aTaxCode="1"> TTAX </vConditionType>
    <vConditionValue aCondValType="P"> 14.44 </vConditionValue>
    <vConditionRate> 19.00 </vConditionRate>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="8">
    <vConditionType aCondArea="P"> TOTL </vConditionType>
    <vConditionValue> 90.44 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>

```

                               - 18 

```
<!-- Item 2 /-->
<vOrderQuantity>1</vOrderQuantity>
<itmPricing aCondNo="1">
    <vConditionType aCondArea="P"> SGRO </vConditionType>
    <vConditionValue> 20.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="2">
    <vConditionType aCondArea="P" aCondRef="1" aTypeDis="SD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vConditionText> Special discount <vConditionText>
    <vPriceUnit> 1 . 000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="3">
    <vConditionType aCondArea="P"> SNET </vConditionType>
    <vConditionValue> 18.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="P"> TNET </vConditionType>
    <vConditionValue> 18.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="5">
    <vConditionType aCondArea="P" aTaxCode="2"> TTNE </vConditionType>
    <vConditionValue> 18.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="6">
    <vConditionType aCondArea="P" aCondRef="5" aTaxCode="2"> TTAX </vConditionType>
    <vConditionValue aCondValType="P"> 1.26 </vConditionValue>
    <vConditionRate> 7.00 </vConditionRate>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>
<itmPricing aCondNo="7">
    <vConditionType aCondArea="P"> TOTL </vConditionType>
    <vConditionValue> 19.26 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>

```

Example 2 – Scenario of a complex discount collection of an invoice item:
Gross unit price of item $ 50,00
Discount 1 (as basic discount) of 20% from the gross unit price
Discount 2 (as other discounts 1) of 5% from the already discounted price of discount 1
Discount 3 (as special discount) of 10% from the resultant value from discounts 1 and 2
Invoice quantity = 2
Unit of invoice quantity = C62

```
<itmPricing aCondNo="1">
    <vConditionType aCondArea="S"> SGRO </vConditionType>
    <vConditionValue> 50.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vPriceUnit> 1 . 000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="2">
    <vConditionType aCondArea="S" aCondRef="1" aTypeDis="BD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 10.00 </vConditionValue>
    <vConditionRate> 20.00 </vConditionRate>
    <vCondCurrency> USD </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
    <vConditionText> Basic discount <vConditionText>
</itmPricing>

```

                               - 19 

```
<itmPricing aCondNo="3">
    <vConditionType aCondArea="S" aCondRef="2" aTypeDis="D1" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 2.00 </vConditionValue>
    <vConditionRate> 5.00 </vConditionRate>
    <vCondCurrency> USD </vCondCurrency>
    <vConditionText> Show room discount <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="4">
    <vConditionType aCondArea="S"> SUBI </vConditionType>
    <vConditionValue> 38.00 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vConditionText> Subtotal <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="5">
    <vConditionType aCondArea="S" aCondRef="4" aTypeDis="SD" aCondSign="-"> DISI </vConditionType>
    <vConditionValue> 3.80 </vConditionValue>
    <vConditionRate> 10.00 </vConditionRate>
    <vCondCurrency> USD </vCondCurrency>
    <vConditionText> Special discount <vConditionText>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="6">
    <vConditionType aCondArea="S"> SNET </vConditionType>
    <vConditionValue> 34.20 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
    <vPriceUnit> 1.000 </vPriceUnit>
    <vQuantUnit> C62 </vQuantUnit>
</itmPricing>
<itmPricing aCondNo="7">
    <vConditionType aCondArea="S"> TNET </vConditionType>
    <vConditionValue> 68.40 </vConditionValue>
    <vCondCurrency> USD </vCondCurrency>
</itmPricing>

```

**2.1.2.15** **`Config`** **: Configuration data**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Config`**|**`_Configuration`**|<br>||||**Configuration data**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<ClassID>`**|**`Value`**|**1 **||||**Class ID**|
|**`<OptionID>`**|**`Value`**|**1 **|**X **|||**Option**|
|**`<OptionEAN>`**|**`EAN_Option`**|**1 **||||**EAN of Option ID**|
|**`<ValueID>`**|**`Value`**|**1 **|**X **|||**Value ID**|
|**`<ValueEAN>`**|**`EAN_Value`**|**1 **||||**EAN of Value ID**|
|~~**`<ConfigText>`**~~|~~**`ConfigText`**~~|*** **||||**Configuration texts**|



Example:
Configuration consisting of 5 values incl. texts (en), value Y-LENGTH expects individual value input.

```
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> 10 </vOptionID>
    <vValueID> 2 </vValueID>
    <itmConfigText>
       <vTextLanguage> en </vTextLanguage>
       <vOptionText> Table top </vOptionText>
       <vValueText aTextLineNo="1" aLineFormat="\"> Beech </vValueText>
    </itmConfigText>
</itmConfiguration>

```

                               - 20 

```
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> Y-LENGTH </vOptionID>
    <vValueID> 50.00 </vValueID>
    <itmConfigText>
       <vTextLanguage> en </vTextLanguage>
       <vOptionText> Table length (inches) </vOptionText>
    </itmConfigText>
</itmConfiguration>
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> XYZ </vOptionID>
    <vValueID> A </vValueID>
    <itmConfigText>
       <vTextLanguage> en </vTextLanguage>
       <vOptionText> Table base </vOptionText>
       <vValueText aTextLineNo="1" aLineFormat="\"> chromed </vValueText>
    </itmConfigText>
</itmConfiguration>
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> 1M </vOptionID>
    <vValueID> C22 </vValueID>
    <itmConfigText>
       <vTextLanguage> en </vTextLanguage>
       <vOptionText> Table height </vOptionText>
       <vValueText aTextLineNo="1" aLineFormat="\"> 28.4 inches </vValueText>
    </itmConfigText>
</itmConfiguration>
<itmConfiguration>
    <vClassID> 1 </vClassID>
    <vOptionID> ZB50 </vOptionID>
    <vValueID> 4D </vValueID>
    <itmConfigText>
       <vTextLanguage> en </vTextLanguage>
       <vOptionText> Layout </vOptionText>
       <vValueText aTextLineNo="1" aLineFormat="\"> Pullout-Container left </vValueText>
       <vValueText aTextLineNo="2" aLineFormat="\"> PC-Container right </vValueText>
    </itmConfigText>
</itmConfiguration>

```

**2.1.2.16** **`ConfigText`** **: Configuration texts**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`ConfigText`**|**`_Frame`**|||||**Configuration texts**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<TextLanguage>`**|**`TextLanguage`**|**1 **|**X **|||**Text language**|
|**`<OptionText>`**|**`OptionText`**|**1 **|**X **|||**Option text**|
|**`<ValueText>`**|**`ValueText`**|*** **||||**Value text**<br>Here, the text is skipped if it is a freely<br>specifiable character value.|



This frame element represents the configuration texts in one or several languages of the previous
configuration details ( `Config` ).

(Example see Configuration data)


                               - 21 

**2.1.2.17** **`Payment`** **: Terms of payment**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`Payment`**|**`_Frame`**|**3***||**! **||**Terms of payment**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<PaymentPart>`**|**`PaymentPart`**|**1 **|**X **|**! **||**Part of payment term**|
|**`<PaymentRate>`**|**`PaymentRate`**|**1 **|**X **|||**Discount rate (%)**<br>0,00 means without discount (net).|
|**`<PaymentDays>`**|**`PaymentDays`**|**1 **|**X **|||**Number of days (payment target)**<br>days mean week days, 0 days means<br>immediately due.|



The terms of payment serve for the pure description of cash discount details and/or net payment in
connection with a credit period.
Otherwise, alternative terms of payment can textually be indicated by the header text segment `hdrText`
( `TextType="PAYC"` ).
These details are only required when they differ from contractual agreements, or if they are not agreed.
At present, maximum 3 parts for the term of payment are supported.

For the individual due dates, the following is supposed: invoice date + number of days (payment target)

Example 1 – term of payment with one part:
10 days without discount net

```
<hdrPayment>
    <vPaymentPart> 1 </vPaymentPart>
    <vPaymentRate> 0.00 </vPaymentRate>
    <vPaymentDays> 10 </vPaymentDays>
</hdrPayment>

```

Example 2 – term of payment with two parts:
14 days 2% discount, 30 days net

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

Example 3 – term of payment with three parts:
5 days 3% discount, 10 days 2%, 30 days net

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

                               - 22 

Example 4 – term of payment with one part:
Due net (without discount)

```
<hdrPayment>
    <vPaymentPart> 1 </vPaymentPart>
    <vPaymentRate> 0.00 </vPaymentRate>
    <vPaymentDays> 0 </vPaymentDays>
</hdrPayment>

```

**2.1.2.18** **`DocNo`** **: Document numbers**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`DocNo`**|**`_Frame`**|||||**Documents numbers**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<DocNoType>`**|**`DocNoType`**|**1 **|**X **|||**Type of document number**|
|**`<DocNo>`**|**`DocNo`**|**1 **|**X **|||**Document number**|
|**`<DocLine>`**|**`DocItemNo`**|**1 **||||**Number of document item**|



In the course of a business case diverse linked documents pile up. These can be references to previous
documents in the sequence of a business case (e.g. quotation  purchase order  sales order) additional
documents as references (e.g. a reference to another sales order). This frame element is used to keep these
details dynamic. This frame element can be a subelement in the document header ( `Header` ) as well as on
item level ( `Item` ), where the subelement `<DocLine>` typically is omitted in the header.

Examples:

Previous document numbers (sequence) of an invoice item of the vendor

```
<itmDocNo>
    <vDocNoType aDocContext=" S "> QUO </vDocNoType> !Item of a quotation
    <vDocNo> AN10040 </vDocNo>
    <vDocLine> 2 </vDocLine>
</itmDocNo>
<itmDocNo>
    <vDocNoType aDocContext=" S "> ORD </vDocNoType> !Item of an order
    <vDocNo> OR552244 </vDocNo>
    <vDocLine> 7 </vDocLine>
</itmDocNo>
<itmDocNo>
    <vDocNoType aDocContext=" S "> CNF </vDocNoType> !Item of an order confirmation
    <vDocNo> AB20050 </vDocNo>
    <vDocLine> 7 </vDocLine>
</itmDocNo>
<itmDocNo>
    <vDocNoType aDocContext=" S "> TSP </vDocNoType> !Shipment
    <vDocNo> TP30060 </vDocNo>
</itmDocNo>
<itmDocNo>
    <vDocNoType aDocContext=" S "> DEL </vDocNoType> !Item of a delivery note
    <vDocNo> LS40070 </vDocNo>
    <vDocLine> 2 </vDocLine>
</itmDocNo>

```

Reference to a sales order as additional information in the order header in case of the processing of a
complaint

```
<hdrDocNo>
    <vDocNoType aDocContext=" R "> CNF </vDocNoType> !Referred order confirmation
    <vDocNo> AB20011 </vDocNo>
</hdrDocNo>

```

                               - 23 

**2.1.2.19** **`BankData`** **: Bank data**

|OEX-Element type|Domain|Col3|Col4|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`BankData`**|**`_Frame`**|||||**Bank data**|


|Subelement|Type|Rec|M.|Col5|Col6|Description|
|---|---|---|---|---|---|---|
|**`<BankName>`**|**`BankName`**|**1 **|**X **|||**Name of Bank**|
|**`<BankCountry>`**|**`BankCountry`**|**1 **|**X **|||**Country of Bank**|
|**`<BankLocation>`**|**`BankLocation`**|**1 **|**X **|||**Location of Bank**|
|**`<SwiftBic>`**|**`SwiftBic`**|**1 **|**1 **|||**SWIFT-BIC Internat. bank code**|
|**`<Iban>`**|**`Iban`**|**1 **|**1 **|||**IBAN International account number**|
|**`<BankKey>`**|**`BankKey`**|**1 **|**2 **|||**Bank identifier**|
|**`<BankAccount>`**|**`BankAccount`**|**1 **|**2 **|||**Bank account**|
|**`<AccountHolder>`**|**`AccountHolder`**|**1 **|**X **|||**Account holder**|



Annotations to the mandatory entries:
**1 + 2** SWIFT-BIC and IBAN are always entered in pairs, _or_ bank identifier and bank account number, _or_ both
pairs.

Examples:

SWIFT-BIC and IBAN (international bank transaction)

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

Bank Identifier and Account Number (national bank transaction)

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

                               - 24 

**2.1.3** **OEX Value Types (** **`Value`** **)**


All value elements are based on the basic type `Value` .

|OEX-Element type|Domain|Description|
|---|---|---|
|**`AccountHolder`**|**`_AccountHolder`**|**Account holder**|
|**`AddressID`**|**`_BusPartID`**|**Address ID**|
|**`AddressType`**|**`_AddressType`**|**Type of address**|
|**`AddStateCode`**|**`_AddStateCode`**|**Additional state code**|
|**`BankAccount`**|**`_BankAccount`**|**Account number**|
|**`BankCountry`**|**`_Country`**|**Country of bank**|
|**`BankKey`**|**`_BankKey`**|**Bank identifier**|
|**`BankLocation`**|**`_Char35`**|**Location of bank**|
|**`BankName`**|**`_Char35`**|**Name of bank**|
|**`CatalogId`**|**`_CatalogId`**|**Catalog ID**|
|**`Classification`**|**`_Classification`**|**Universal classification**|
|**`ClientID`**|**`_BusPartID`**|**Client ID**|
|**`ClientClass`**|**`_BusPartClass`**|**Client classification**|
|**`CommodCode`**|**`_CommodCode`**|**Commodity code (INTRASTAT)**|
|**`CompSubArtId`**|**`_CompSubArtId`**|**Identification of sub article**|
|**`ComType`**|**`_ComType`**|**Type of communication**|
|**`ConditionText`**|**`_Char35`**|**Description of condition**|
|**`ConditionType`**|**`_ConditionType`**|**Type of condition**|
|**`ConditionRate`**|**`_ConditionRate`**|**Rate of condition**|
|**`ConditionValue`**|**`_Condition`**|**Value of condition**|
|**`CondCurrency`**|**`_Currency`**|**Curreny of condition**|
|**`ContactType`**|**`_ContactType`**|**Type of contact**|
|**`CountryCode`**|**`_CountryCode`**|**Country code**|
|**`CountryOrigin`**|**`_CountryCode`**|**Country of origin**|
|**`CountyCode`**|**`_CountyCode`**|**County/district/state**|
|**`CountyOrigin`**|**`_CountyCode`**|**County of origin**|
|**`CustomNumber`**|**`_Char35`**|**Customs number**|
|**`Date`**|**`_Date`**|**Date**|
|**`DateTimeType`**|**`_DateTimeType`**|**Type of Date/time**|
|**`DelivComplet`**|**`_DelivComplet`**|**Completeness of delivery**|
|**`District`**|**`_Char35`**|**District**|
|**`DocCurrency`**|**`_Currency`**|**Currency of document**|
|**`DocLanguage`**|**`_Language`**|**Language of document**|
|**`DocNo`**|**`_Char35`**|**Document number**|
|**`DocNoType`**|**`_DocNoType`**|**Type of document number**|
|**`DocItemNo`**|**`_PosNo`**|**Number of document item**|
|**`DocumentType`**|**`_DocumentType`**|**Type of document**|
|**`EAN_Article`**|**`_EAN`**|**EAN of article**|
|**`EAN_Option`**|**`_EAN`**|**EAN of option ID**|
|**`EAN_Value`**|**`_EAN`**|**EAN of value ID**|
|**`FirstName`**|**`_Char35`**|**First name**|
|**`FolderIsLOC`**|**`_YesNo`**|**Is the folder name an indication of location?**|
|**`GrossWeight`**|**`_Quantity`**|**Gross weight**|
|**`Height`**|**`_Quantity`**|**Height**|
|**`Iban`**|**`_Iban`**|**IBAN International account number**|
|**`IncoTerm`**|**`_IncoTerm`**|**Inco Terms (terms of delivery)**|
|**`IncoTermLoc`**|**`_Char35`**|**Location concerning Inco Terms**|
|**`InvoiceType`**|**`_InvoiceType`**|**Type of invoice**|



                               - 25 

|OEX-Element type|Domain|Description|
|---|---|---|
|**`LastName`**|**`_Char35`**|**Last name**|
|**`Length`**|**`_Quantity`**|**Length**|
|**`Location`**|**`_Char35`**|**Location (City)**|
|**`MeansTransp`**|**`_MeansTransp`**|**Means of transport**|
|**`MeasureUnit`**|**`_Unit`**|**Measurement unit**|
|**`Name1`**|**`_Char35`**|**Name 1**|
|**`Name2`**|**`_Char35`**|**Name 2**|
|**`Name3`**|**`_Char35`**|**Name 3**|
|**`Name4`**|**`_Char35`**|**Name 4**|
|**`NetWeight`**|**`_Quantity`**|**Net weight**|
|**`NumPackages`**|**`_Integer`**|**Number of packages**|
|**`NumArtPack`**|**`_Integer`**|**Number of articles per package**|
|**`OptionText`**|**`_Char80`**|**Value text**|
|**`OrderType`**|**`_OrderType`**|**Type of order**|
|**`OrgDataType`**|**`_OrgDataType`**|**Type of organizational data**|
|**`PackageNumber`**|**`_Char35`**|**Package number**|
|**`PackageType`**|**`_PackageType`**|**Type of package**|
|**`PartDelivery`**|**`_YesNo`**|**Allow partial deliveries?**|
|**`PaymentDays`**|**`_PaymentDays`**|**Number of days (payment target)**|
|**`PaymentPart`**|**`_PaymentPart`**|**Part of payment term**|
|**`PaymentRate`**|**`_PaymentRate`**|**Discount Rate (%)**|
|**`PostalCode`**|**`_PostalCode`**|**Postal code**|
|**`PostalCodePOB`**|**`_PostalCode`**|**Postal code of P.O. Box**|
|**`PriceUnit`**|**`_Quantity`**|**Price unit**|
|**`Quantity`**|**`_Quantity`**|**Quantity**|
|**`QuantUnit`**|**`_Unit`**|**Quantity unit**|
|**`ReferenceType`**|**`_ReferenceType`**|**Type of reference**|
|**`ShipmentBase`**|**`_ShipmentBase`**|**Shipment base**|
|**`Street`**|**`_Char35`**|**Street**|
|**`Street2`**|**`_Char35`**|**Street 2**|
|**`SupplierID`**|**`_BusPartID`**|**Supplier ID**|
|**`SupplierClass`**|**`_BusPartClass`**|**Supplier classification**|
|**`SwiftBic`**|**`_SwiftBic`**|**SWIFT-BIC Internat. bank code**|
|**`TextContent`**|**`_TextLine`**|**Content of text**|
|**`TextLanguage`**|**`_Language`**|**Language of text**|
|**`TextLineNo`**|**`_LineNo`**|**Line number of text**|
|**`TextType`**|**`_TextType`**|**Type of text**|
|**`Time`**|**`_Time`**|**Time**|
|**`TimeZone`**|**`_UTC`**|**Time zone**|
|**`TransportMode`**|**`_TransportMode`**|**Transport mode**|
|**`UnitVolume`**|**`_Unit`**|**Volume unit**|
|**`UnitWeight`**|**`_Unit`**|**Weight unit**|
|**`ValueText`**|**`_TextLine`**|**Value text**|
|**`VendorArtNo`**|**`_VendorArtNo`**|**Article number of vendor (supplier)**|
|**`VendorID`**|**`_VendorID`**|**Vendor (supplier) ID**|
|**`VendorSeries`**|**`_VendorSeries`**|**Vendor (supplier) series**|
|**`Volumen`**|**`_Quantity`**|**Volume**|
|**`Width`**|**`_Quantity`**|**Width**|



                               - 26 

**2.1.4** **OEX Attribute Types (** **`Empty`** **)**


All attribute elements are based on the basic type `Empty` .

|OEX-Element type|Domain|Description|
|---|---|---|
|`AppVersion`|`_Version`|Version of application|



                               - 27 

**2.2** **Data Domains**

Naming of domains: Prefix _ (underscore) + name starting with a capital letter.
The column “Restrict.“ (Restriction) differentiates in a table of values, under which conditions its values are
valid. Data types are explained in section 2.3, possible attributes in section 2.4.
For some data domains, a value is regarded as set if the value is "empty" `<empty>` and/or the element
referring to this data domain is skipped `<skipped>` .

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_AccountHolder`**|**`CHAR`**|**27**|||**Account holder**|
|**`_Address`**|**`FRAME`**||||**Addresses**|
|**`_Address`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_Address`**|`aAction`|`aAction`|||Action|
|**`_AddressType`**<br>|**`CHAR(UPPER)`**|**2 **|||**Type of Address**|
|**`_AddressType`**<br>|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_AddressType`**<br>|`SO`|`SO`|||Sold-to party|
|**`_AddressType`**<br>|`SH`|`SH`|||Ship-to party|
|**`_AddressType`**<br>|`IN`|`IN`|||Bill-to party|
|**`_AddressType`**<br>|`PA`|`PA`|||Payer|
|**`_AddressType`**<br>|`CA`|`CA`|||Carrier (shipper)|
|**`_AddressType`**<br>|`SU`|`SU`|||Supplier (vendor)|
|**`_AddressType`**<br>|`EU`|`EU`|||End user|
|**`_AddressType`**<br>|`IS`|`IS`|||Installation company (on-site installation)|
|**`_AddressType`**<br>|`IL`|`IL`|||Installation location|
|**`_AddressType`**<br>|`BR`|`BR`|||Branch (of sold-to party)|
|**`_AddStateCode`**|**`CHAR`**|*** **|||**Additional state information**<br>Encoding of states which is required –<br>beyond the commercial variant code – for<br>re-creation of an OFML instance (specific to<br>OFML Part III).|
|**`_Attribute`**|**`ATTR`**||||**Attribute element**|
|**`_BankAccount`**|**`CHAR(NUPPER)`**|**20**|||**Bank account number**<br>National account number|
|**`_BankKey`**|**`CHAR(NUPPER)`**|**10**|||**Bank identifier**<br>National bank identifier|
|**`_BusPartClass`**|**`CHAR`**|**20**|||**Business partner classification**|
|**`_BusPartClass`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_BusPartClass`**|`aBusPartClassType`|`aBusPartClassType`|X|X|Type of business partner classification|
|**`_BusPartID`**|**`CHAR`**|**20**|||**Business partner ID**|
|**`_BusPartID`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_BusPartID`**|`aBusPartIDType`|`aBusPartIDType`|X|X|Type of business partner ID|
|**`_CatalogId`**|**`CHAR(RX001)`**|*** **|||**Catalog ID**<br>Unique key of a catalog profile<br>Format: <identifier>.<revision><br>(see catalog profile specification)<br>Example: de-2011.1|
|**`_Char35`**|**`CHAR`**|**35**|||**Alphanumerical value 35**|
|**`_Char80`**|**`CHAR`**|**80**|||**Alphanumerical value 80**|
|**`_Classification`**|**`CHAR`**|*** **|||**Universal classification**|
|**`_Classification`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_Classification`**|`aClassSystem`|`aClassSystem`|X|X|Classification system|
|**`_ClientArtNo`**|**`CHAR`**|*** **|||**Article number of the client**|
|**`_ClientArtNo`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_ClientArtNo`**|`aAction`|`aAction`|||Action|



                               - 28 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_CommodCode`**|**`NUM(NOSIGN)`**|**8 **|||**Commodity code (INTRASTAT)**<br>Commodity code defined in the commodity<br>index for foreign-trade statistics.|
|**`_CompSubArtId`**|**`CHAR`**|*** **|||**Identification of the sub article**<br>within the composite article<br>(specific to OFML Part III)|
|**`_ComType`**|**`CHAR(UPPER)`**|**3 **|||**Types of communications**|
|**`_ComType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_ComType`**|`TEL`|`TEL`|||Telephone number|
|**`_ComType`**|`FAX`|`FAX`|||Fax number|
|**`_ComType`**|`MOB`|`MOB`|||Mobile number|
|**`_ComType`**|`WWW`|`WWW`|||Website|
|**`_ComType`**|`EMA`|`EMA`|||Email-address|
|**`_ComType`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_ComType`**|`aScopeInfo`|`aScopeInfo`|X|X|Scope of information|
|**`_Condition`**|**`NUM(NOSIGN)`**|*** **|**2 **|**. **|**Value of condition** (absolute)<br>Price, discount value, tax value etc.; is<br>defined by the condition type<br>(`_ConditionType`).|
|**`_ConditionRate`**|**`NUM(NOSIGN)`**|*** **|**2 **|**. **|**Rate of condition** (percentaged)<br>Discount rate, tax rate etc.; is defined by the<br>condition type (`_ConditionType`).|
|**`_ConditionType`**<br>(to be cont.)|**`CHAR(UPPER)`**|**4 **|||**Types of conditions**<br>Defines type and usuage of a condition<br>value (`_Condition`) respectively of a<br>condition rate (`_ConditionRate`).<br>Here, specifications like gross and net do<br>not apply to the value added tax.|
|**`_ConditionType`**<br>(to be cont.)|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_ConditionType`**<br>(to be cont.)|`SNET`|`SNET`|`ITM`<br>`A `|`ITM`<br>`A `|Net unit price<br>_Unit prices are sometimes already a total of_<br>_serveral prices, which are the result of a_<br>_configurable product but not stored or_<br>_shown separately._|
|**`_ConditionType`**<br>(to be cont.)|`SGRO`|`SGRO`|`ITM`<br>`A `|`ITM`<br>`A `|Gross unit price<br>_(surcharges and discounts allowed)_<br>_Unit prices are sometimes already a total of_<br>_serveral prices, which are the result of a_<br>_configurable product but not stored or_<br>_shown separately._|
|**`_ConditionType`**<br>(to be cont.)|`TNEH`|`TNEH`|`HDR`<br>`A `|`HDR`<br>`A `|Total net on header level<br>_After discount and/or surcharges on header_<br>_level (__`DISH`, __`SURH`). If those are not_<br>_specified, the condition type can be skipped._<br>_Then it is identical to the condition type total_<br>_net (__`TNET`) on header level._|
|**`_ConditionType`**<br>(to be cont.)|`TNET`|`TNET`|`A `|`A `|Total net|
|**`_ConditionType`**<br>(to be cont.)|`TGRO`|`TGRO`|`A `|`A `|Total gross|
|**`_ConditionType`**<br>(to be cont.)|`TOTL`|`TOTL`|`A `|`A `|Grand Total<br>_Total incl. taxes_|




- 29 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_ConditionType`**<br>(continued, to be cont.)|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_ConditionType`**<br>(continued, to be cont.)|`DISH`|`DISH`|`HDR`<br>`CR`<br>`TD`<br>`- `|`HDR`<br>`CR`<br>`TD`<br>`- `|Discount on header level<br>_is calculated from the total net value (__`TNET`) _<br>_of the header. After that, further_<br>_combinations or graduations with the_<br>_condition types__`DISH` and__`SURH` are_<br>_possible._<br>_If a discount shall be indicated as absolute_<br>_discount restriction “A“ applies._|
|**`_ConditionType`**<br>(continued, to be cont.)|`DISI`|`DISI`|`CR`<br>`TD`<br>`- `|`CR`<br>`TD`<br>`- `|Discount on item level<br>_Discounts are calculated from the gross_<br>_value. The corresponding condition type of_<br>_the gross value is indicated as standard_<br>_condition._<br>_Other discounts can also be calculated from_<br>_the already discounted value. Here, the_<br>_corresponding condition type is indicated as_<br>_standard condition._<br>_A combination with surcharges is possible,_<br>_too._<br>_If a discount shall be specified as absolute_<br>_discount restriction “A“ applies._<br>_On header level, this condition type is the_<br>_sum of all discounts of the items in_<br>_consideration of the discount type. Here, no_<br>_percentage is indicated. (Restriction “A“)_|
|**`_ConditionType`**<br>(continued, to be cont.)|`SURH`|`SURH`|`HDR`<br>`CR`<br>`TS`<br>`+ `|`HDR`<br>`CR`<br>`TS`<br>`+ `|Surcharge on header level<br>_is calculated from the total net value (__`TNET`) _<br>_of the header. After that, further_<br>_combinations or graduations with the_<br>_condition types__`DISH` and__`SURH` are_<br>_possible._<br>_If a discount shall be indicated as absolute_<br>_discount restriction “A“ applies._|
|**`_ConditionType`**<br>(continued, to be cont.)|`SURI`|`SURI`|`CR`<br>`TS`<br>`+ `|`CR`<br>`TS`<br>`+ `|Surcharge on item level<br>_Surcharges are calculated from the gross_<br>_value. But further surcharges can be_<br>_calculated from an already charged value.._<br>_In both cases the standard condition is_<br>_indicated as in the case of the discount. If a_<br>_discount shall be indicated as absolute_<br>_discount restriction “A“ applies._<br>_On header level this condition type is the_<br>_sum of all surcharges of the items in_<br>_consideration of the surcharge type. Here,_<br>_no percentage is displayed. (Restriction “A“)_|



- 30 



|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_ConditionType`**<br>(continued)|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_ConditionType`**<br>(continued)|`SUBH`|`SUBH`|`HDR`<br>`A `<br>|`HDR`<br>`A `<br>|Subtotal on header level<br>_serves as reference condition for_<br>_subsequent discounts or surcharges (__`DISH`, _<br>_`SURH`). All previous discounts or surcharges_<br>_are calculated with their standard conditions_<br>_and form the respective subtotal._<br>_All subsequent discounts or surcharges may_<br>_not refer to conditions previous to the_<br>_subtotal. The indication of several subtotals_<br>_type__`SUBH` is permitted but not in direct_<br>_succession._|
|**`_ConditionType`**<br>(continued)|`SUBI`|`SUBI`|`A `<br>|`A `<br>|Subtotal on item and/or header level<br>_serves as reference condition for_<br>_subsequent discounts or surcharges (__`DISI`, _<br>_`SURI`). All previous discounts or surcharges_<br>_are calculated with their reference_<br>_conditions and form the respective subtotal._<br>_All subsequent discounts or surcharges may_<br>_not refer to conditions previous to this_<br>_subtotal. The indication of several subtotals_<br>_type__`SUBI` is permitted but not in direct_<br>_succession._|
|**`_ConditionType`**<br>(continued)|`TTNE`|`TTNE`|`A `<br>`TAX`|`A `<br>`TAX`|Tax net value<br>_This condition type is added on header level_<br>_in consideration of the tax code._|
|**`_ConditionType`**<br>(continued)|`TTAX`|`TTAX`|`CR`<br>`P `<br>`TAX`|`CR`<br>`P `<br>`TAX`|Tax rate<br>_Within a document, exactly one tax rate is_<br>_allocated to one tax code._|
|**`_ConditionType`**<br>(continued)|**Restrictions**|**Restrictions**|||**Usage**|
|**`_ConditionType`**<br>(continued)|`ITM`|`ITM`|||Document items only|
|**`_ConditionType`**<br>(continued)|`HDR`|`HDR`|||Document header only|
|**`_ConditionType`**<br>(continued)|`A `|`A `|||Absolute condition value only<br>_`_Condition` contains condition value._<br>_`_ConditionRate` inapplicable._|
|**`_ConditionType`**<br>(continued)|`P `|`P `|||Percent condition value only <br>_`_Condition` contains the value on bases_<br>_of the condition rate._<br>_`_ConditionRate` contains the percentage_<br>_rate._|
|**`_ConditionType`**<br>(continued)|`CR`|`CR`|||Specification of refered condition required|
|**`_ConditionType`**<br>(continued)|`TAX`|`TAX`|||Sepcification of tax code required|
|**`_ConditionType`**<br>(continued)|`TS`|`TS`|||Type of surcharge required|
|**`_ConditionType`**<br>(continued)|`TD`|`TD`|||Type of discount required|
|**`_ConditionType`**<br>(continued)|`+ `|`+ `|||Surcharge (`aCondSign="+"`)|
|**`_ConditionType`**<br>(continued)|`- `|`- `|||Discount (`aCondSign="-"`)|
|**`_ConditionType`**<br>(continued)|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_ConditionType`**<br>(continued)|`aCondArea`|`aCondArea`|X|X|Condition area|
|**`_ConditionType`**<br>(continued)|`aCondRef`|`aCondRef`|||Condition reference (base of calculation)|
|**`_ConditionType`**<br>(continued)|`aTaxCode`|`aTaxCode`|||Tax code|
|**`_ConditionType`**<br>(continued)|`aTypeDis`|`aTypeDis`|||Type of surcharge|
|**`_ConditionType`**<br>(continued)|`aTypeSur`|`aTypeSur`|||Type of discount|
|**`_ConditionType`**<br>(continued)|`aCondSign`|`aCondSign`|||Sign indicating surcharge or discount|




- 31 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_Configuration`**|**`FRAME`**||||**Feature of the configuration**<br>_If attribute_`aMustCheck`_ is not specified or_<br>_empty, value_`Y`_ (yes) is assumed._|
|**`_Configuration`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_Configuration`**|`aIsVisisble`<br>|`aIsVisisble`<br>|X|X|Is visible?|
|**`_Configuration`**|`aMustCheck`|`aMustCheck`|||Is relevant for checks?|
|**`_Configuration`**|`aAction`|`aAction`|||Action|
|**`_ContactType`**|**`CHAR(UPPER)`**|**2 **|||**Types of contacts**|
|**`_ContactType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_ContactType`**|`SC`|`SC`|||Sales contact|
|**`_ContactType`**|`WC`|`WC`|||Warehouse contact|
|**`_ContactType`**|`IN`|`IN`|||Installer|
|**`_ContactType`**|`EM`|`EM`|||Employee|
|**`_ContactType`**|`CL`|`CL`|||Client|
|**`_ContactType`**|`SU`|`SU`|||Support/Staffer|
|**`_CountryCode`**|**`CHAR(UPPER)`**|**2 **|||**Country code according to ISO 3166-1**<br>Examples:<br>`DE` <br>Germany<br> <br>`ES` <br>Spain<br>`GB` <br>Great Britain<br>`FR` <br>France|
|**`_CountyCode`**|**`CHAR`**|**6 **|||**Country/district/state accord. to**<br>**ISO 3166-2**<br>Only the second part is indicated. The first<br>part corresponds with the country code<br>according to ISO 3166-1 (`_CountryCode`).<br>Examples for country`US` (USA):<br>`CA` <br>California<br>`NY` <br>New York<br>`TX` <br>Texas<br>`WA` <br>Washington|
|**`_Currency`**|**`CHAR(UPPER)`**|**3 **|||**Currency code according to ISO 4217**<br>(for valid currencies at present)<br>Examples:<br>`EUR` Euro<br> <br>`GBP` Brit. Pound<br>`CHF` Swiss Francs<br>`USD` US Dollar|
|**`_Date`**|**`CHAR(DATE)`**|**8 **|||**Date**|
|**`_Date`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_Date`**|`aDateFormat`|`aDateFormat`|X|X|Date format|
|**`_Date`**|`aDateCalcBase`|`aDateCalcBase`|1|1|Base of date determination|
|**`_Date`**|`aDateCalcMode`|`aDateCalcMode`|1|1|Mode of date determination|
|**`_DateTime`**|**`FRAME`**<br>||<br>|<br>|**Date and time details**<br>|
|**`_DateTime`**|**Attribute** <br>|**Attribute** <br>|**Mandat.**|**Mandat.**||
|**`_DateTime`**|`aAction`|`aAction`|||Action|
|**`_DateTimeType`**<br>(to be cont.)|**`CHAR(UPPER)`**|**3 **|||**Types of date/time**|
|**`_DateTimeType`**<br>(to be cont.)|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_DateTimeType`**<br>(to be cont.)|`DOC`|`DOC`|`HDR`<br>`T `|`HDR`<br>`T `|Document date<br>_Date when the document was added to the_<br>_XML-file._|
|**`_DateTimeType`**<br>(to be cont.)|`CRD`|`CRD`|||Customer requested delivery date <br>_For a delivery as fast as possible, the_<br>_customer can enter for example a delivery_<br>_date at short notice to beckon the vendor to_<br>_confirm the best possible date (without_<br>_obligation)._|




- 32 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_DateTimeType`**<br>(continued)|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_DateTimeType`**<br>(continued)|`DLD`|`DLD`|||Delivery date (supplier)<br>_Delivery date without obligation of the_<br>_supplier. If a fixed delivery date should be_<br>_confirmed, use type “__`FXD` – Fixed delivery_<br>_date“ instead._|
|**`_DateTimeType`**<br>(continued)|`EPD`|`EPD`|||Earliest possible delivery date<br>_A delivery before this date is not_<br>_permissible. In combination with the__`LPD,` _<br>_latest possible delivery date, it can form a_<br>_delivery period._|
|**`_DateTimeType`**<br>(continued)|`LPD`|`LPD`|||Latest possible delivery date<br>_A delivery after this date is not permissible._<br>_In combination with__`EPD`, earliest possible_<br>_delivery date, it can form a delivery period._|
|**`_DateTimeType`**<br>(continued)|`FXD`|`FXD`|||Fixed delivery date|
|**`_DateTimeType`**<br>(continued)|`REQ`|`REQ`|`HDR`|`HDR`|Request date|
|**`_DateTimeType`**<br>(continued)|`QUO`|`QUO`|`HDR`|`HDR`|Quotation date|
|**`_DateTimeType`**<br>(continued)|`QUV`|`QUV`|`HDR`|`HDR`|Quotation validity date<br>(Quotation valid until)|
|**`_DateTimeType`**<br>(continued)|`ORD`|`ORD`|`HDR`|`HDR`|Order date<br>_Date when the order application has_<br>_ordered._|
|**`_DateTimeType`**<br>(continued)|`COD`|`COD`|`HDR`|`HDR`|Order confirmation date|
|**`_DateTimeType`**<br>(continued)|`DES`|`DES`|`HDR`|`HDR`|Despatch advice date|
|**`_DateTimeType`**<br>(continued)|`DND`|`DND`|`HDR`|`HDR`|Delivery note date|
|**`_DateTimeType`**<br>(continued)|`INV`|`INV`|`HDR`|`HDR`|Invoice date|
|**`_DateTimeType`**<br>(continued)|`DUE`|`DUE`|||Due date|
|**`_DateTimeType`**<br>(continued)|`DSR`|`DSR`|`HDR`|`HDR`|Date of services rendered|
|**`_DateTimeType`**<br>(continued)|`PRD`|`PRD`|||Price date<br>_Date when prices have been calculated_<br>_using a price list valid on this date._<br>_Therefore the designation of the price list is_<br>_corresponding to the “organizational data“._|
|**`_DateTimeType`**<br>(continued)|**Restrictions**|**Restrictions**|||**Usage**|
|**`_DateTimeType`**<br>(continued)|`ITM`|`ITM`|||Document items only|
|**`_DateTimeType`**<br>(continued)|`HDR`|`HDR`|||Document header only|
|**`_DateTimeType`**<br>(continued)|`T `|`T `|||Specification of time required|
|**`_DelivComplet`**|**`CHAR(UPPER)`**|**1 **|||**Completeness of delivery**<br>(regarding an order or an order item)|
|**`_DelivComplet`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_DelivComplet`**|`E `|`E `|||Entire delivery|
|**`_DelivComplet`**|`P `|`P `|||Partial delivery|
|**`_Document`**|**`FRAME`**||||**Single document**|
|**`_Document`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_Document`**|`aDocNo`|`aDocNo`|X|X|Consecutive number of the document|
|**`_Document`**|`aItemCount`|`aItemCount`|X|X|Total number of items in the document|
|**`_Document`**|`aAction`|`aAction`|X|X|Action|
|**`_DocFrame`**|**`FRAME`**||||**OEX document frame**|
|**`_DocFrame`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_DocFrame`**|`aMajor`|`aMajor`|X|X|Major version of OEX|
|**`_DocFrame`**|`aTransferMode`|`aTransferMode`|||Transfer mode of the XML-file|
|**`_DocFrame`**|`<XSD>`|`<XSD>`|X|X|XML schema integration (see 1)|



                               - 33 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_DocNoType`**|**`CHAR(UPPER)`**|**3 **|||**Types of document numbers**|
|**`_DocNoType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_DocNoType`**|`REQ`|`REQ`|||Request number|
|**`_DocNoType`**|`QUO`|`QUO`|||Quotation number|
|**`_DocNoType`**|`ORD`|`ORD`|||Order number|
|**`_DocNoType`**|`CHG`|`CHG`|||Order change number|
|**`_DocNoType`**|`CNF`|`CNF`|||Order confirmation number<br>_(corresponding to sales order number or_<br>_sales document number from the vendor’s_<br>_point of view)_|
|**`_DocNoType`**|`DEL`|`DEL`|||Delivery number<br>_(delivery note number)_|
|**`_DocNoType`**|`LOL`|`LOL`|||Loading list number|
|**`_DocNoType`**|`SHP`|`SHP`|||Shipment number<br>_A shipment is composed of one or more_<br>_deliveries (__`DEL`) and/or orders (__`CNF`)._<br>_(see dispatch notification (__`DESADV`))_|
|**`_DocNoType`**|`INV`|`INV`|||Invoice number|
|**`_DocNoType`**|`TAN`|`TAN`|||Transaction number|
|**`_DocNoType`**|`CON`|`CON`|||Contract number|
|**`_DocNoType`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_DocNoType`**|`aDocContext`|`aDocContext`|X|X|Document context|
|**`_DocumentType`**|**`CHAR(UPPER)`**|**6 **|||**Types of documents**|
|**`_DocumentType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_DocumentType`**|`REQOTE`|`REQOTE`|||Request (of a quotation)|
|**`_DocumentType`**|`QUOTES`|`QUOTES`|||Quotation|
|**`_DocumentType`**|`ORDERS`|`ORDERS`|||Order (purchase order)|
|**`_DocumentType`**|`ORDCHG`|`ORDCHG`|||Order change|
|**`_DocumentType`**|`ORDRSP`|`ORDRSP`|||Order confirmation (response)|
|**`_DocumentType`**|`DESADV`|`DESADV`|||Despatch advice|
|**`_DocumentType`**|`INVOIC`|`INVOIC`|||Invoice|
|**`_DocumentType`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_DocumentType`**|`aMajor`|`aMajor`|X|X|Major version number|
|**`_DocumentType`**|`aMinor`|`aMinor`|X|X|Minor version number|
|**`_DocumentType`**|`aBuild`|`aBuild`|X|X|Build version number|
|**`_EAN`**|**`CHAR`**|*** **|||**EAN**<br>International (**E**uropean)**A**rticle**N**umber|
|**`_EAN`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_EAN`**|`aEANType`|`aEANType`|X|X|EAN Type|
|**`_File`**|**`FRAME`**||||**File of documents**|
|**`_File`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_File`**|`aDocumentCount`|`aDocumentCount`|X|X|Number of documents within a file|
|**`_Frame`**|**`FRAME`**||||**Frame element**|
|**`_Header`**|**`FRAME`**||||**Document header**|
|**`_Header`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_Header`**|`aAction`|`aAction`|||Action|
|**`_Iban`**|**`CHAR(NUPPER)`**|**34**|||**IBAN**<br>**I**nternational**B**ank**A**ccount**N**umber<br>According to ISO 13616:2003|



                               - 34 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_IncoTerm`**|**`CHAR(UPPER)`**|**3 **|||**Inco Terms according to Inco Terms**<br>**2000**<br>(international terms of delivery)|
|**`_IncoTerm`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_IncoTerm`**|`CFR`|`CFR`|`POD`|`POD`|Cost and Freight|
|**`_IncoTerm`**|`CIF`|`CIF`|`POD`|`POD`|Cost, Insurance and Freight|
|**`_IncoTerm`**|`CIP`|`CIP`|`DST`|`DST`|Carriage and Insurance Paid To|
|**`_IncoTerm`**|`CPT`|`CPT`|`DST`|`DST`|Carriage Paid To|
|**`_IncoTerm`**|`DAF`|`DAF`|`PLA`|`PLA`|Delivered At Frontier|
|**`_IncoTerm`**|`DDP`|`DDP`|`PLA`|`PLA`|Delivered Duty Paid|
|**`_IncoTerm`**|`DDU`|`DDU`|`PLA`|`PLA`|Delivered Duty Unpaid|
|**`_IncoTerm`**|`DEQ`|`DEQ`|`POD`|`POD`|Delivered Ex Quay|
|**`_IncoTerm`**|`DES`|`DES`|`POD`|`POD`|Delivered Ex Ship|
|**`_IncoTerm`**|`EXW`|`EXW`|`PLA`|`PLA`|Ex Works|
|**`_IncoTerm`**|`FAS`|`FAS`|`POS`|`POS`|Free Alongside Ship|
|**`_IncoTerm`**|`FCA`|`FCA`|`PLA`|`PLA`|Free Carrier|
|**`_IncoTerm`**|`FOB`|`FOB`|`POS`|`POS`|Free On Board|
|**`_IncoTerm`**|**Restrictions**|**Restrictions**|||**Ortsangaben**|
|**`_IncoTerm`**|`PLA`|`PLA`|||Named place|
|**`_IncoTerm`**|`POS`|`POS`|||Named port of shipment|
|**`_IncoTerm`**|`POD`|`POD`|||Named port of destination|
|**`_IncoTerm`**|`DST`|`DST`|||Named place of destination|
|**`_InvoiceType`**|**`CHAR(UPPER)`**|**2 **|||**Types of invoices**|
|**`_InvoiceType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_InvoiceType`**|`IN`|`IN`|||Invoice|
|**`_InvoiceType`**|`CN`|`CN`|||Credit Note|
|**`_InvoiceType`**|`PI`|`PI`|||Proforma Invoice|
|**`_Integer`**|**`NUM`**|*** **|||**Integer value**|
|**`_Item`**|**`FRAME`**||||**Document item**<br>_In contrast to the document position counter_<br>_(_`aItemNo`_), the number of the document_<br>_item (_`DocItemNo`_), and other item number_<br>_specifications_`(_OrgDataType POS`_), the_<br>`aUUID`_ of a document item in the sequence_<br>_of the documents for the business case_<br>_must not change!_|
|**`_Item`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_Item`**|`aItemNo`|`aItemNo`|X|X|Consecutive number of document item|
|**`_Item`**|`aAction`|`aAction`|||Action|
|**`_Item`**|`aUUID`|`aUUID`|||Universally Unique Identifier|
|**`_Item`**|`aItemType`|`aItemType`|||Type of document item|
|**`_Language`**|**`CHAR(LOWER)`**|**2 **|||**Language code according to ISO 639-1**<br>Examples:<br>`de` <br>German<br> <br>`en` <br>English<br>`fr` <br>French<br> <br>`es` <br>Spanish<br> <br>If the language is determined by the country<br>(language version) the attribute for the<br>locale is indicated.<br>Example:<br>Language`en` (English) and locale<br>`aLocale="US"` for American English.|
|**`_Language`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_Language`**|`aLocale`|`aLocale`|||Locale|



                               - 35 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_LineNo`**|**`NUM(LIST1)`**|*** **|||**Line number**|
|**`_MeansTransp`**|**`CHAR(UPPER)`**|**3 **|||**Means of transport**|
|**`_MeansTransp`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_MeansTransp`**|`PAL`|`PAL`|||Pallet|
|**`_MeansTransp`**|`ROL`|`ROL`|||Roll|
|**`_MeansTransp`**|`SAC`|`SAC`|||Sack|
|**`_MeansTransp`**|`LAB`|`LAB`|||Lattice box|
|**`_MeansTransp`**|`CON`|`CON`|||Container|
|**`_MeansTransp`**|`BAR`|`BAR`|||Barrel|
|**`_OrderType`**|**`CHAR(UPPER)`**|**2 **|||**Types of orders**<br>Can be used for the further determination of<br>the sales order, especially for the purchase<br>order (ORDERS).|
|**`_OrderType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_OrderType`**|`SO`|`SO`|||Standard order|
|**`_OrderType`**|`XO`|`XO`|||Express order<br>_is generally offered for articles defined by_<br>_the vendor (for example in the context of a_<br>_sales promotion or contractual agreement)_<br>_which have a faster delivery time/ availability_<br>_than normally. Here, the order volume is_<br>_generally restricted to a certain quantity._<br>_If and how this type of order is processed_<br>_depends on the respective manufacturer._|
|**`_OrderType`**|`MU`|`MU`|||Mock-up order|
|**`_OrderType`**|`CP`|`CP`|||Complaint processing order<br>_Here, a corresponding complaint has been_<br>_received by the vendor before and the case_<br>_may be, a document number has been_<br>_allocated, which can be indicated in the_<br>_order (__`DocNo`)._|
|**`_OrderType`**|`SP`|`SP`|||Spare part order|
|**`_OrderType`**|`EO`|`EO`|||Order for own employee|
|**`_OrderType`**|`SR`|`SR`|||Show room order|
|**`_OrgData`**|**`FRAME`**<br>||<br>|<br>|**Organizational data**<br>|
|**`_OrgData`**|**Attribut** <br>|**Attribut** <br>|**Mandat.**|**Mandat.**||
|**`_OrgData`**|`aAction`|`aAction`|||Action|
|**`_OrgDataType`**<br>(to be cont.)|**`CHAR(UPPER)`**|**3 **|||**Type of organizational data**<br>_Note: The possibility to use various data in_<br>_the document does not mandatorily mean,_<br>_that the receiver is also able to process or to_<br>_return these data._|
|**`_OrgDataType`**<br>(to be cont.)|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_OrgDataType`**<br>(to be cont.)|`CCC`|`CCC`|||Customer cost center|
|**`_OrgDataType`**<br>(to be cont.)|`CNF`|`CNF`|`ITM`|`ITM`|Configuration ID<br>_e.g. as support for a configurator application_<br>_to identify created configuration by its own_<br>_(variant code)._|
|**`_OrgDataType`**<br>(to be cont.)|`COG`|`COG`|`ITM`|`ITM`|Class of goods|
|**`_OrgDataType`**<br>(to be cont.)|`COM`|`COM`|||Commission|
|**`_OrgDataType`**<br>(to be cont.)|`DIC`|`DIC`|||Distribution channel|
|**`_OrgDataType`**<br>(to be cont.)|`DIV`|`DIV`|||Division|
|**`_OrgDataType`**<br>(to be cont.)|`DLO`|`DLO`|||Despatch location|
|**`_OrgDataType`**<br>(to be cont.)|`DPL`|`DPL`|||Delivering plant|



                               - 36 

|Domain|Data type|Len|Dec|Sep|Col6|Description|
|---|---|---|---|---|---|---|
|**`_OrgDataType`** <br>(continued)|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Restrict.**||
|**`_OrgDataType`** <br>(continued)|`ITM`|`ITM`|`ITM`|`ITM`|`ITM`|Article ID<br>_E.g. for specific end article number_|
|**`_OrgDataType`** <br>(continued)|`LOC`|`LOC`||||Location, e.g. unloading point<br>(building/floor/room)|
|**`_OrgDataType`** <br>(continued)|`OVC`|`OVC`|`ITM`|`ITM`|`ITM`|OFML variant code<br>_see also CNF_|
|**`_OrgDataType`** <br>(continued)|`PGR`|`PGR`||||Purchase group|
|**`_OrgDataType`** <br>(continued)|`PJN`|`PJN`||||Project number|
|**`_OrgDataType`** <br>(continued)|`PLO`|`PLO`||||Point of loading|
|**`_OrgDataType`** <br>(continued)|`POR`|`POR`||||Purchase organization|
|**`_OrgDataType`** <br>(continued)|`POS`|`POS`|`ITM`|`ITM`|`ITM`|Position ID<br>_for the indication of an item number_<br>_especially edited or differing from the_<br>_distinct item number._<br>_Example: “100.A.10-1“._|
|**`_OrgDataType`** <br>(continued)|`PRI`|`PRI`||||Processing indicator<br>_for the differentiation between internal and_<br>_external processes of an organization._|
|**`_OrgDataType`** <br>(continued)|`SGR`|`SGR`||||Sales group|
|**`_OrgDataType`** <br>(continued)|`SOF`|`SOF`||||Sales office|
|**`_OrgDataType`** <br>(continued)|`SOR`|`SOR`||||Sales organization|
|**`_OrgDataType`** <br>(continued)|`TOU`|`TOU`||||Tour|
|**`_OrgDataType`** <br>(continued)|`TRZ`|`TRZ`||||Transport zone|
|**`_OrgDataType`** <br>(continued)|**Restrictions**|**Restrictions**||||**Usage**|
|**`_OrgDataType`** <br>(continued)|`ITM`|`ITM`||||Document items only|
|**`_OrgDataType`** <br>(continued)|`HDR`|`HDR`||||Document header only|
|**`_PackageType`**|**`CHAR(UPPER)`**|**3 **||||**Types of packages**|
|**`_PackageType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Restrict.**||
|**`_PackageType`**|`CBB`|`CBB`||||Cardboard box|
|**`_PackageType`**|`PAP`|`PAP`||||Paper|
|**`_PackageType`**|`FOI`|`FOI`||||Foil|
|**`_PackageType`**|`BOT`|`BOT`||||Bottle|
|**`_PackageType`**|`TIN`|`TIN`||||Tin|
|**`_PackageType`**|`CAN`|`CAN`||||Can|
|**`_PackageType`**|`BOX`|`BOX`||||Box|
|**`_PackageType`**|`BAG`|`BAG`||||Bag|
|**`_PaymentDays`**|**`NUM(NOSIGN)`**|**3 **||||**Number of days**|
|**`_PaymentPart`**|**`NUM(LIST1)`**|**1 **||||**Part of payment term**|
|**`_PaymentRate`**|**`NUM(NOSIGN)`**|**5 **|**2 **||**. **|**Discount rate (%)**|
|**`_PosNo`**|**`CHAR(POS)`**|**6 **||||**Position number**|
|**`_PostalCode`**|**`CHAR(POSTAL)`**|**10**||||**Postal code**|
|**`_Pricing`**|**`Frame`**|||||**Pricing**|
|**`_Pricing`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**|**Mandat.**||
|**`_Pricing`**|`aCondNo`<br>|`aCondNo`<br>|X|X|X|Consecutive number of condition|
|**`_Pricing`**|`aAction`|`aAction`||||Action|
|**`_Quantity`**|**`NUM(NOSIGN)`**|*** **|**3 **||**. **|**Quantity**|
|**`_Reference`**|**`Frame`**|||||**References**|
|**`_Reference`**|**Attribute**<br>|**Attribute**<br>|**Mandat.**|**Mandat.**|**Mandat.**||
|**`_Reference`**|`aAction`|`aAction`||||Action|



                               - 37 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_ReferenceType`**|**`CHAR(UPPER)`**|**3 **|||**Types of references**|
|**`_ReferenceType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_ReferenceType`**|`ATT`|`ATT`|||Attachment<br>_Complete name of a file that is sent with the_<br>_OEX document in an e-mail._<br>_(e.g. Product information.pdf)_|
|**`_ReferenceType`**|`DOC`|`DOC`|||Reference to a document<br>_Name or file name of a document_|
|**`_ReferenceType`**|`EDS`|`EDS`|||Embedded Data Stream, encoded as<br>Base64|
|**`_ReferenceType`**|`LNK`|`LNK`|||Executable link (entire URL)<br>_E.g. to follow a link for a HTML-site directly_<br>_through the internet-browser._|
|**`_ReferenceType`**|`XML`|`XML`|||Embedded CDATA element<br>_With a CDATA element, structured data in_<br>_XML format from a third-party system can_<br>_be embedded. The prerequisite for this is_<br>_that the embedded data itself does not use_<br>_CDATA elements and uses UTF-8 as_<br>_character encoding. (As MIME type_<br>_"text/ plain" has to be specified.)_|
|**`_ReferenceType`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_ReferenceType`**|`aMIMEType`|`aMIMEType`|X|X|MIME-Type<br>Type of data (according to RFC 2046)|
|**`_ShipmentBase`**|**`CHAR(UPPER)`**|**1 **|||**Shipment base**<br>defines, if a shipment is composed either of<br>orders or of deliveries.|
|**`_ShipmentBase`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_ShipmentBase`**|`O `|`O `|||Order (sales)|
|**`_ShipmentBase`**|`D `|`D `|||Deliveries (delivery notes)|
|**`_SwiftBic`**|**`CHAR(NUPPER)`**|**11**|||**SWIFT-BIC Internat. bank code**<br>**S**ociety for**W**orldwide**I**nterbank**F**inancial<br>**T**elecommunication<br>**B**ank**I**dentifier**C**ode<br>According to ISO 9362|
|**`_Text`**|**`FRAME`**||||**Texts**|
|**`_Text`**|**Attribute**<br>|**Attribute**<br>|**Mandat.**|**Mandat.**||
|**`_Text`**|`aAction`|`aAction`|||Action|
|**`_TextLine`**|**`CHAR`**|**80**|||**Text line**|
|**`_TextLine`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_TextLine`**|`aTextLineNo`|`aTextLineNo`|X|X|Text line number|
|**`_TextLine`**|`aLineFormat`|`aLineFormat`|X|X|Line format|



                               - 38 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_TextType`**<br>(tob e cont.)|**`CHAR (UPPER)`**|**4 **|||**Text types**|
|**`_TextType`**<br>(tob e cont.)|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_TextType`**<br>(tob e cont.)|`HEAD`|`HEAD`|`HDR`|`HDR`|General header text<br>_Texts, which are not covered by other text_<br>_types for document header._|
|**`_TextType`**<br>(tob e cont.)|`ITEM`|`ITEM`|`ITM`|`ITM`|General item text<br>_Texts, which are not covered by other text_<br>_types for documents items._|
|**`_TextType`**<br>(tob e cont.)|`ARTS`|`ARTS`|`ITM`|`ITM`|Article short text<br>_Used as short description of the article and_<br>_consists of only one line._<br>(`vTextLineNo = 1`) <br>_Besides the article number the article short_<br>_text in addition identifies an article in an_<br>_XML-file and provides a better readabilty of_<br>_the XML-file, if for instance shown directly in_<br>_a browser using a style-sheet._|
|**`_TextType`**<br>(tob e cont.)|`ARTL`|`ARTL`|`ITM`|`ITM`|Article long text<br>_Is used for detailed description of the article._<br>_The article long text doesn’t have to be_<br>_transferred, if both trading partners deal with_<br>_a non-modified standard article known by_<br>_each other._|
|**`_TextType`**<br>(tob e cont.)|`ARTV`|`ARTV`|`ITM`|`ITM`|Variant text<br>_Describes the configuration desired by the_<br>_user. (Due to the control options provided_<br>_for the variant text in OCD, this can differ_<br>_from the text that results from the_<br>_concatenation of the texts for the individual_<br>_configurable characteristics.)_|
|**`_TextType`**<br>(tob e cont.)|`ARTM`|`ARTM`|`ITM`|`ITM`|Modified article text<br>_In the case of a modified standard article_<br>_(original article of the manufacturer/ vendor),_<br>_its modified text is transferred and the article_<br>_is correspondingly indicated. Systems which_<br>_do not differentiate text types and manage_<br>_all texts in one text block must insert the_<br>_total text block here._<br>(`_VendorArtNo`  `aStatus = M`)|
|**`_TextType`**<br>(tob e cont.)|`ARTU`|`ARTU`|`ITM`|`ITM`|User-defined additional text<br>_A supplementary text entered by the user of_<br>_the ordering system, e.g. to provide_<br>_information on the installation. This text type_<br>_differs conceptually from the modified article_<br>_text (ARTM) and does not require_ aStatus =<br>M.|
|**`_TextType`**<br>(tob e cont.)|`PAYC`|`PAYC`|`HDR`|`HDR`|Payment terms<br>_If different from contracted agreements or_<br>_not agreed._|
|**`_TextType`**<br>(tob e cont.)|`GRTM`|`GRTM`|||Goods receiving times|
|**`_TextType`**<br>(tob e cont.)|`DNOT`|`DNOT`|||Despatch notes|
|**`_TextType`**<br>(tob e cont.)|`DCON`|`DCON`|`HDR`|`HDR`|Delivery terms<br>_If different from contracted agreements or_<br>_not agreed._|



                               - 39 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_TextType`**<br>(continued)|`INOT`|`INOT`|||Installation notes|
|**`_TextType`**<br>(continued)|`PRMD`|`PRMD`|||Terms of transaction<br>_Hints to support the processing of a_<br>_business case._<br>_Example: „Please provide staff for_<br>_unloading and distribution on the delivery.“_|
|**`_TextType`**<br>(continued)|`ADAG`|`ADAG`|||Additional agreement <br>_(with contractual relevance)_|
|**`_TextType`**<br>(continued)|**Restrictions**|**Restrictions**|||**Usage**|
|**`_TextType`**<br>(continued)|`ITM`|`ITM`|||Document items only|
|**`_TextType`**<br>(continued)|`HDR`|`HDR`|||Document header only|
|**`_Time`**|**`CHAR(TIME)`**|**6 **|||**Time**|
|**`_TransportMode`**|**`CHAR(UPPER)`**|**3 **|||**Transport mode**|
|**`_TransportMode`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_TransportMode`**|`SNA`|`SNA`|||Navigation|
|**`_TransportMode`**|`SIN`|`SIN`|||Inland navigation|
|**`_TransportMode`**|`SCO`|`SCO`|||Coasting|
|**`_TransportMode`**|`LRO`|`LRO`|||Road traffic|
|**`_TransportMode`**|`LRR`|`LRR`|||Railway traffic|
|**`_TransportMode`**|`AAV`|`AAV`|||Aviation|
|**`_TransportMode`**|`MMT`|`MMT`|||Multimodal transport<br>_(several transport modes)_|
|**`_Unit`**|**`CHAR(NUPPER)`**|**3 **|||**Measurement unit code**<br>According to Common Code of UN/ECE<br>Recommendation 20<br>Examples:<br>`C62` One (piece)<br>`MTR` Meter<br> <br> <br>`MTK` Square meters|
|**`_UTC`**|**`CHAR(UTC)`**|**5 **|||**Time zone according to UTC**<br>(UTC = Universal Time Coordinated)|
|**`_Value`**|**`CHAR`**|*** **|||**Any value**|
|**`_VendorArtNo`**|**`CHAR`**|*** **|||**Article number of the vendor (supplier)**<br>_This is the basic article number of the_<br>_vendor (supplier)._<br>_(analog OCD article table__ ArticleID)_<br>_In case of configurable articles the result of_<br>_the configuration is described by the frame_<br>_type “_`Config` –_configuration details“._<br>_Optionally, frame type “_`OrgData`_ –_<br>_Organizational data“ can be used for further_<br>_information on the article resp. the_<br>_configuration (e.g._`_OrgDataType "CNF"` <br>_or_`"ITM"`_)_|
|**`_VendorArtNo`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_VendorArtNo`**|`aStatus`|`aStatus`|X|X|Status of article|
|**`_VendorID`**|**`CHAR(NUPPER)`**|**4 **|||**Vendor (supplier) ID**<br>_According to OCD-Specification_|
|**`_VendorSeries`**|**`CHAR(NUPPER)`**|**4 **|||**Vendor (supplier) series**<br>_According to OCD-Specification_|



                               - 40 

|Domain|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`_Version`**|**`ATTR`**||||**Version**|
|**`_Version`**|**Attribute**|**Attribute**|**Mandat.**|**Mandat.**||
|**`_Version`**|`aMajor`|`aMajor`|X|X|Major version number|
|**`_Version`**|`aMinor`|`aMinor`|X|X|Minor version number|
|**`_Version`**|`aBuild`|`aBuild`|||Build version number|
|**`_YesNo`**|**`BOOL`**|**1 **|||**Yes/No**|
|**`_YesNo`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**||
|**`_YesNo`**|`Y `|`Y `|||Yes|
|**`_YesNo`**|`N `|`N `|||No|



                               - 41 

**2.3** **Data Types**

Naming of data types: completely in capital letters

|Data type|Options|Description/Explanation|
|---|---|---|
|**`ATTR`**|**Attribute element**|**Attribute element**|
|**`BOOL`**|**Boolean Value**|**Boolean Value**|
|**`BOOL`**|~~**`Y `**~~|Yes|
|**`BOOL`**|~~**`N `**~~|No|
|**`CHAR`**<br>(to be cont.)|**All characters of the basic code-page of the OEX-document**|**All characters of the basic code-page of the OEX-document**|
|**`CHAR`**<br>(to be cont.)|~~**`UPPER`**~~|**Capital letters only**<br>Valid characters:<br>**`ABCDEFGHIJKLMNOPQRSTUVWXYZ`**|
|**`CHAR`**<br>(to be cont.)|~~**`NUPPER`**~~|**Capital letters and digits**<br>Valid characters:<br>**`ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789`**<br>Examples:<br>`DE456271567`<br> <br>(EU VAT ID) <br>`UBSWCHZH80A`<br> <br>(SWIFT-BIC) <br>`DE68210501700012345678`(IBAN)|
|**`CHAR`**<br>(to be cont.)|~~**`XUPPER`**~~|**Capital letters and others**<br>Valid characters:<br>**`ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789`**<br>**`+-*=_\/.,:;()!?#&%"`** <br>Space character in between|
|**`CHAR`**<br>(to be cont.)|~~**`LOWER`**~~|**Lower case letters only**<br>Valid characters:<br>**`abcdefghijklmnopqrstuvwxyz`**|
|**`CHAR`**<br>(to be cont.)|~~**`NLOWER`**~~|**Lower case letters and digits**<br>Valid characters:<br>**`abcdefghijklmnopqrstuvwxyz0123456789`**|
|**`CHAR`**<br>(to be cont.)|~~**`XLOWER`**~~|**Lower case letters and others**<br>Valid characters:<br>**`abcdefghijklmnopqrstuvwxyz0123456789`**<br>**`+-*=_\/.,:;()!?#&%`**`"` <br>Space character in between|
|**`CHAR`**<br>(to be cont.)|~~**`RX001`**~~|**Regular expression 001**<br>`[a-z][a-z0-9_-]*.[0-9]* `<br>Example: de-2011.1|
|**`CHAR`**<br>(to be cont.)|~~**`NUMB`**~~|**Numeration, list, rank**<br>Valid characters: <br>**`0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-.`** <br>Space character in between<br>Examples:<br>`1, 1.1, 1.2 etc.`<br>`1, 1-1, 1-2 etc.`<br>`A, B, C etc.`<br>`I, II, III, IV etc.`<br>`I.1, I.2, etc.`|
|**`CHAR`**<br>(to be cont.)|~~**`POS`**~~|**Position numbering**<br>Valid characters: <br>**`0123456789 `**<br>_Usually a consecutive numbering using an increment._<br>_The number may be indicated with leading zeros._<br>Example (6-digits number, increment 1):<br>`000001, 000002, 000003 etc.`|



                               - 42 

|Data type|Options|Description/Explanation|
|---|---|---|
|**`CHAR`**<br>(continued)|~~**`POSTAL`**~~|**Postal codes**<br>Valid characters: <br>**`0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ`** <br>Space character and**`–`** in between<br>Examples:<br>`07743`<br>(Jena) <br>`170 00`<br>(Prag) <br>`ECM1 5PG`(London) <br>`00-023`<br>(Warschau)|
|**`CHAR`**<br>(continued)|~~**`DATE`**~~|**Date**<br>Valid characters:<br>**`0123456789`** <br>_Day format (YYYYMMDD) or Week format (YYYYWW)_<br>_see attributes_ `aDateFormat`|
|**`CHAR`**<br>(continued)|~~**`TIME`**~~|**Time**<br>Time format: 24 hours`HHMMSS` <br>`HH` <br> <br>Hours (**`00`** –**`23`**) <br>`MM` <br> <br>Minutes (**`00`** –**`59`**) <br>`SS` <br> <br>Seconds (**`00`** –**`59`**)|
|**`CHAR`**<br>(continued)|~~**`UTC`**~~|**UTC Universal Time Coordinated**<br>_The time zones are indicated as positive or negative_<br>_devation (time lag) from UTC._<br>Format:`SHHMM` <br>`S` <br>= Sign (**`+`** or**`-`** )<br>`HH` <br>= Hours (**`00`** –**`23`**) <br>`MM` <br>= Minutes (**`00`** –**`59`**) <br>Examples:<br>Western European Time (WET)<br>`+0000` (+0 Hours)<br>UTC<br>(Great Britain, Portugal, Iceland, etc.)<br>Central European Time (CET)<br>`+0100` (+1 Hour)<br>UTC+1<br>(Germany, France, Switzerland, etc.)<br>Central Europ. Summer Time (CEST)<br>`+0200` (+2 Hours)<br>UTC+2<br>Eastern Standard Time (EST)<br>`-0500` (-5 Hours)<br>UTC-5<br>(USA-New York, Cuba, Peru, etc.)|
|**`CHAR`**<br>(continued)|~~**`UUID`**~~|**Universally Unique Identifier**<br>_36-digit string representation according to RFC 4122_<br>Valid characters: <br>**`0123456789-abcdefABCDEF`**<br>Example:<br>`bbb5a714-27c6-416c-ad47-e4df02b6a93c`|



                               - 43 

|Data type|Options|Description/Explanation|
|---|---|---|
|**`FRAME`**|**Frame element**|**Frame element**|
|**`NUM`**|**Numerical Value**<br>Containing decimals and separator, if applicable.<br>As decimal separator the dot “**.**“ (decimal point) is used.<br>Signs (`+` and`-`) are prefixed. If no sign is indicated “`+`“ is supposed.|**Numerical Value**<br>Containing decimals and separator, if applicable.<br>As decimal separator the dot “**.**“ (decimal point) is used.<br>Signs (`+` and`-`) are prefixed. If no sign is indicated “`+`“ is supposed.|
|**`NUM`**|~~**`LIST1`**~~|**List 1**<br>_Used in a certain recurring element. (e.g. textlines)_<br>**Increment 1, starting at 1, no signs**<br>Example:`1, 2, 3, 4` etc.|
|**`NUM`**|~~**`COUNT`**~~|**Number of list elements**<br>_Number of elements which are enumerated with Data_<br>_type NUM und Option LIST1 and have to contain at_<br>_least 1 element._<br>**Minimum value 1, no signs**|
|**`NUM`**|~~**`NOSIGN`**~~|**No signs**|
|**`NUM`**|~~**`VERSION`**~~|**Version number**<br>0 – 65535 (Integer), no signs|



                               - 44 

**2.4** **Attributes**

Naming of attributes: Prefix **`a`**
At some attributes, a value is regarded as set if the value is "empty" `<empty>` and/or the attribute is skipped
`<skipped>` .

|Attribute|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`aAction`**|**`CHAR(UPPER)`**|**1 **|||**Action**<br>_Processing method for the application_<br>_receiving the document._<br>_If the attribute is not mandatory in an_<br>_element, value__`N` is assumed if the attribute_<br>_is not specified._|
|**`aAction`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aAction`**|`C `|`C `|||Create|
|**`aAction`**|`D `|`D `|||Delete|
|**`aAction`**|`M `|`M `|||Modify|
|**`aAction`**|`N `|`N `|||No action / without modification|
|**`aBuild`**|**`NUM(VERSION)`**|*** **|||**Build version number**|
|**`aBusPartClassType`**|**`CHAR(UPPER)`**|*** **|||**Type of business partner classification**<br>_Determines the classification system_<br>_(standard, regulation) used for the_<br>_classification._|
|**`aBusPartClassType`**|**Table of values**<br>|**Table of values**<br>|**Restrict.**|**Restrict.**|**Description**|
|**`aBusPartClassType`**|`SIC` <br>|`SIC` <br>|||Standard Industrial Classification|
|**`aBusPartClassType`**|`ISIC` <br>|`ISIC` <br>|||International Standard Industrial<br>Classification|
|**`aBusPartClassType`**|`NACE` <br>|`NACE` <br>|||Statistical classification of economic<br>activities in the European Community|
|**`aBusPartClassType`**|`ICS`|`ICS`|||Individual classification system<br>(not corresponding to any norm)|
|**`aBusPartIDType`**|**`CHAR(UPPER)`**|*** **|||**Type of business partner ID**<br>_Determines the classification system_<br>_(standard, regulation) used for the ID._|
|**`aBusPartIDType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aBusPartIDType`**|`GLN`|`GLN`|||Global Location Number|
|**`aBusPartIDType`**|`DUNS`|`DUNS`|||Data Universal Numbering System|
|**`aBusPartIDType`**|`IIS`|`IIS`|||Individual ID system<br>(not corresponding to any norm)|
|**`aClassSystem`**|**`CHAR(XUPPER)`**|*** **|||**Classification system**<br>_Determines the system (standard) used to_<br>_specify a class or category._<br>_The following systems are predefined_<br>_(reserved):_|
|**`aClassSystem`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aClassSystem`**|`ECO_FR`|`ECO_FR`|||Article category for ECO-Tax France<br>(according to appendix of OCD<br>specification)|
|**`aClassSystem`**|`ECLASS-x.y`|`ECLASS-x.y`|||Classification according to eCl@ss model in<br>version x.y|
|**`aClassSystem`**|`UNSPSC`|`UNSPSC`|||Classification according to UN/SPSC<br>standard|



                               - 45 

|Attribute|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`aCondArea`**|**`CHAR(UPPER)`**|**2 **|||**Condition area**|
|**`aCondArea`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aCondArea`**|`P `|`P `|||Purchase|
|**`aCondArea`**|`S `|`S `|||Sales|
|**`aCondArea`**|`OP`|`OP`|||Original purchase price of furnisher (OCD)|
|**`aCondArea`**|`OS`|`OS`|||Original sales price of furnisher (OCD)|
|**`aCondNo`**|**`NUM(LIST1)`**|**6 **|||**Consecutive number of the condition**|
|**`aCondRef`**|**`NUM(NOSIGN)`**|**6 **|||**Condition reference**<br>for the current number of the condition<br>(`aCondNo`).<br>_For the calculation base, the_**_consecutive_**<br>**_number_**_ of the respective condition is_<br>_indicated as a reference. The following_<br>_restrictions contain the condition types that_<br>_can be referred to._|
|**`aCondRef`**|**Restrictions**|**Restrictions**|||**Usage**|
|**`aCondRef`**|`SGRO`|`SGRO`|||Gross unit price|
|**`aCondRef`**|`TGRO`|`TGRO`|||Total gross|
|**`aCondRef`**|`DISH`|`DISH`|||Discount on header level|
|**`aCondRef`**|`DISI`|`DISI`|||Discount on item level|
|**`aCondRef`**|`SURH`|`SURH`|||Surcharge on header level|
|**`aCondRef`**|`SURI`|`SURI`|||Surcharge on item level|
|**`aCondRef`**|`SUBH`|`SUBH`|||Subtotal on header level|
|**`aCondRef`**|`SUBI`|`SUBI`|||Subtotal on item and/or header level|
|**`aCondRef`**|`TTNE`|`TTNE`|||Tax net value|
|**`aCondSign`**|**`CHAR`**|**1 **|||**Sign indicating surcharge or discount**|
|**`aCondSign`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aCondSign`**|`+ `|`+ `|||Surcharge|
|**`aCondSign`**|`- `|`- `|||Discount|
|**`aDateCalcBase`**|**`CHAR(XUPPER)`**|**4 **|||**Base of date determination**<br>_Is used at fields of the data type_<br>_`CHAR(DATE)`in combination with the_<br>_attribute__`aDateFormat` and its indication for_<br>_a number of days. The number of days will_<br>_be used for the calculation basis_<br>_corresponding to the attribute_<br>_`aDateCalcMode` to determine the_<br>_respective date in the recipient system._|
|**`aDateCalcBase`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aDateCalcBase`**|`*DIO`|`*DIO`|||Date of order received<br>_Dynamic date; it is not known until the time_<br>_of processing._|
|**`aDateCalcBase`**|`<_DateTimeType>`|`<_DateTimeType>`|||Specification of a type “Date/Time“<br>_reference to a date that was indicated in the_<br>_previous element of the same type._|
|**`aDateCalcMode`**|**`CHAR`**|**1 **|||**Mode of date determination**<br>_Requires attribute__`aDateCalcBase`. _|
|**`aDateCalcMode`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aDateCalcMode`**|`+ `|`+ `|||Addition|
|**`aDateCalcMode`**|`- `|`- `|||Subtraction|



                               - 46 

|Attribute|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`aDateFormat`**|**`CHAR(UPPER)`**|**1 **|||**Date format**<br>`YYYY` <br>Year (4 digits) e.g.`2009` <br>`MM` <br> <br>Month (2 digits) e.g.`02` for<br> <br> <br> <br>February<br>`DD` <br> <br>Day (2 digits) e.g.`03` <br>`WW` <br> <br>Week (2 digits) e.g.`05` <br>`CCCC` <br>Number of calendar days<br> <br> <br>(4 digits) e.g.`0014` <br>Examples:`20090203` 3rd February 2009<br> <br> <br>`200905` <br>Week 5 in 2009|
|**`aDateFormat`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aDateFormat`**|`D `|`D `|||Day format YYYYMMDD|
|**`aDateFormat`**|`W `|`W `|||Week format YYYYWW|
|**`aDateFormat`**|`C `|`C `|||Number of calendar day CCCC|
|**`aDocContext`**|**`CHAR(XUPPER)`**|**1 **|||**Document context**|
|**`aDocContext`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aDocContext`**|`S `|`S `|||**Sequence**<br>Document sequence of a business case;<br>_at an invoice item e.g.: document number_<br>_(and item) of the quote (__`QUO`), the order_<br>_(__`ORD`), the delivery (__`DEL`)._|
|**`aDocContext`**|`R `|`R `|||**Reference**<br>By means of the reference, it is possible to<br>refer to documents which are no documents<br>in terms of the sequence of a business case<br>but serve as additional information to<br>process it.<br>_Example: In case of a complaint processing_<br>_the order in which the complaint occurred_<br>_can be referred to._|
|**`aDocNo`**|**`NUM(LIST1)`**|**6 **|||**Consecutive number of the document**|
|**`aDocumentCount`**|**`NUM(COUNT)`**|**6 **|||**Number of documents within a file**|
|**`aEANType`**|**`CHAR(XUPPER)`**|**6 **|||**EAN Type**|
|**`aEANType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aEANType`**|`EAN-8`|`EAN-8`|||EAN-Code 8 digits|
|**`aEANType`**|`EAN-13`|`EAN-13`|||EAN-Code 13 digits|
|**`aItemCount`**|**`NUM(COUNT)`**|**6 **|||**Number of document items within a**<br>**document** <br>_Independent of being a main item or a sub_<br>_item._|
|**`aIsPseudo`**|**`BOOL`**|**1 **|||**Does the item represent a pseudo**<br>**article?**<br>If yes (value`Y`), the item may need special<br>handling in the processing system.<br>_The article number of a pseudo article_<br>_typically does not exist in the ERP system of_<br>_the vendor, but was created artificially for_<br>_technical reasons during OFML data_<br>_creation._<br>_If the attribute is not specified or empty,_<br>_value_`N`_ (no) is assumed._|
|**`aIsVisible`**|**`BOOL`**|**1 **|||**Is the feature visible to the user?**|



                               - 47 

|Attribute|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`aItemNo`**|**`NUM(LIST1)`**|**6 **|||**Consecutive number of document item**|
|**`aItemType`**|**`CHAR(UPPER)`**|**1 **|||**Type of document item**<br>For the differentiated processing of the item.|
|**`aItemType`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aItemType`**|`<empty>/<skipped>`|`<empty>/<skipped>`|||Standard|
|**`aItemType`**|`O `|`O `|`1 `|`1 `|Optional|
|**`aItemType`**|`A `|`A `|`1 `|`1 `|Alternative|
|**`aItemType`**|**Restrictions**|**Restrictions**|||**Usage**|
|**`aItemType`**|`1 `|`1 `|||for Request and Quotation|
|**`aLineFormat`**|**`CHAR`**|**1 **|||**Line format**<br>according to OCD as from version 4|
|**`aLineFormat`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aLineFormat`**|~~**`\ `**~~|~~**`\ `**~~|||**Line feed**<br>_text is displayed in a new line._|
|**`aLineFormat`**|~~**`~ `**~~|~~**`~ `**~~|||**Word-wrapping (continuous text)**<br>_The text line is added as continuous text to_<br>_the previous text. If the text line does not_<br>_start with a space it has to be inserted by_<br>_the processing application._|
|**`aLocale`**|**`CHAR(UPPER)`**|**2 **|||**Locale**<br>For the assignment of country specifics like<br>language, measurements, etc.<br>Details based on the country code<br>according to ISO 3166-1<br>Examples:<br>`DE` <br>Germany<br> <br>`ES` <br>Spain<br>`GB` <br>Great Britain<br>`FR` <br>France|
|**`aMajor`**|**`NUM(VERSION)`**|*** **|||**Major version number**|
|**`aMIMEType`**|**`CHAR`**|*** **|||**MIME type**<br>(Multipurpose Internet Mail Extensions)<br>Format according to RFC 2046:<br><Media Type>/<Subtype><br>Examples:<br>text/html<br>text/plain<br>image/jpeg<br>application/pdf<br>application/msword|
|**`aMinor`**|**`NUM(VERSION)`**|*** **|||**Minor version number**|
|**`aMustCheck`**|**`BOOL`**|**1 **|||**Is relevant for checks?**<br>_Determines whether the entity (e.g., a_<br>_feature, domain__`_Configuration`) must_<br>_be validated during the processing of the_<br>_document if there are changes with respect_<br>_to the preceding document (see also_<br>_`aAction`). (In case of__`N`, a change_<br>_can/should be ignored.)_|
|**`aScopeInfo`**|**`CHAR(UPPER)`**|**1 **|||**Scope of information**|
|**`aScopeInfo`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aScopeInfo`**|`B `|`B `|||Business|
|**`aScopeInfo`**|`P `|`P `|||Private|



                               - 48 

|Attribute|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`aStatus`**<br>(to be cont.)|**`CHAR(UPPER)`**|**1 **|||**Status of article**<br>The article status defines the origin of the<br>article and shows if its master data<br>(structure/ text/configuration) have been<br>modified.<br>_This is to support the automatic processing._<br>_The article status does not refer to order_<br>_quantities, prices or other data_<br>_corresponding to the item._|
|**`aStatus`**<br>(to be cont.)|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aStatus`**<br>(to be cont.)|~~**`M `**~~|~~**`M `**~~|`REQOTE`<br>`QUOTES`<br>`ORDERS`<br>`ORDRSP`<br>`ORDCHG`|`REQOTE`<br>`QUOTES`<br>`ORDERS`<br>`ORDRSP`<br>`ORDCHG`|**Modification of the article**<br>_The original data of the article (O) or of the_<br>_special article (S) provided by the_<br>_manufacturer/ vendor have been modified_<br>_(e.g. article number, series code, texts)._<br>_Modified texts are provided by the item text_<br>_of the text type__`"ARTM"` (modified article_<br>_text)._<br>_The orderer has requested the modifications_<br>_from the manufacturer/ vendor before._<br>_Otherwise, the case may be that such an_<br>_article is rejected or not confirmed by the_<br>_manufacturer/ vendor._<br>_If the manufacturer/ vendor does not replace_<br>_this article by a special article (S), he also_<br>_returns it with the status (M) and the_<br>_modified text__`"ARTM"`. _|
|**`aStatus`**<br>(to be cont.)|~~**`S `**~~|~~**`S `**~~|||**Special article of the vendor/supplier**<br>_Article data that are returned by the_<br>_manufacturer/ vendor for a modified article_<br>_(M) or customer article (C) if he replaces the_<br>_article by one of his own._<br>_Article texts are returned as follows:_<br>_Article long text with text type_`"ARTL"`_, _<br>_article short text with text type_`"ARTS"`_, _<br>_possible configuration with configuration_<br>_frame elements_`"itmConfiguration"` <br>_and_`"itmConfigText"`_. _<br>_In the case of an order modification, this_<br>_attribute is also indicated by the orderer if he_<br>_has not modified the article._|
|**`aStatus`**<br>(to be cont.)|~~**`O `**~~|~~**`O `**~~|||**Original article**<br>_The article corresponds to the original data_<br>_provided by the manufacturer/vendor in an_<br>_electronical pricelist._|




- 49 

|Attribute|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`aStatus`**<br>(continued)|~~**`C `**~~|~~**`C `**~~|`REQOTE`<br>`QUOTES`<br>`ORDERS`<br>`ORDRSP`<br>`ORDCHG`<br>|`REQOTE`<br>`QUOTES`<br>`ORDERS`<br>`ORDRSP`<br>`ORDCHG`<br>|**Custom article**<br>_An article created by the orderer himself in_<br>_his master data or directly in the order (single_<br>_use) for a special article of the manufacturer/_<br>_vendor._<br>_The orderer has to request such an article_<br>_from the manufacturer/vendor before._<br>_Otherwise, as the case may be, such an_<br>_article is rejected or not confirmed by the_<br>_manufacturer/ vendor. The manufacturer/_<br>_vendor may have already given an article_<br>_number to the orderer who can use it to_<br>_create an article by himself. Here, the article_<br>_texts are transferred as follows:_<br>_article long text with text type_`"ARTL"`_, article_<br>_short text with text type_`"ARTS"`_. _<br>_If the manufacturer/vendor does not replace_<br>_the article by a special article (S) he also_<br>_returns it with status (C)._|
|**`aStatus`**<br>(continued)|**Restrictions**|**Restrictions**|||**Usage**|
|**`aStatus`**<br>(continued)|`REQOTE`|`REQOTE`|||Request|
|**`aStatus`**<br>(continued)|`QUOTES`|`QUOTES`|||Quotation|
|**`aStatus`**<br>(continued)|`ORDERS`|`ORDERS`|||Order|
|**`aStatus`**<br>(continued)|`ORDRSP`|`ORDRSP`|||Order Confirmation|
|**`aStatus`**<br>(continued)|`ORDCHG`|`ORDCHG`|||Order Change|
|**`aStatus`**<br>(continued)|`DESADV`|`DESADV`|||Despatch Advice|
|**`aStatus`**<br>(continued)|`INVOIC`|`INVOIC`|||Invoice|
|**`aTaxCode`**|**`NUM(NOSIGN)`**|**3 **|||**Tax code**|
|**`aTaxCode`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aTaxCode`**|`0 `|`0 `|||Tax-excempt|
|**`aTaxCode`**|`1 - 6`|`1 - 6`|||Tax category for VAT according to appendix<br>of OCD specification:<br>1 = Standard rate<br>2 = Reduced rate<br>3 = Severely reduced rate<br>4 = Parking rate<br>5 = Service<br>6 = Zero rate|
|**`aTaxCode`**|`7 - 99`|`7 - 99`|||Reserved (for future standardization)|
|**`aTaxCode`**|`100 - 999`|`100 - 999`|||For free use|
|**`aTextLineNo`**|**`NUM(LIST1)`**|**6 **|||**Text line number**|
|**`aTransferMode`**|**`CHAR(UPPER)`**|**1 **|||**Transfer mode of the XML-file**|
|**`aTransferMode`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aTransferMode`**|`<empty>/<skipped>`|`<empty>/<skipped>`|||Operational transfer|
|**`aTransferMode`**|`R `|`R `|||Repeated transfer|
|**`aTransferMode`**|`T `|`T `|||Test transfer|
|**`aTypeDis`**|**`CHAR(NUPPER)`**|**2 **|||**Type of discounts**|
|**`aTypeDis`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aTypeDis`**|`BD`|`BD`|||Basic discount|
|**`aTypeDis`**|`VO`|`VO`|||Volume discount|
|**`aTypeDis`**|`SD`|`SD`|||Special discount|
|**`aTypeDis`**|`RD`|`RD`|||Reseller discount|
|**`aTypeDis`**|`AR`|`AR`|||Aggregated discount|
|**`aTypeDis`**|`D1 – D9`|`D1 – D9`|||Other discounts (1 – 9)|




- 50 

|Attribute|Data type|Len|Dec|Sep|Description|
|---|---|---|---|---|---|
|**`aTypeSur`**|**`CHAR(NUPPER)`**|**2 **|||**Type of surcharges**|
|**`aTypeSur`**|**Table of values**|**Table of values**|**Restrict.**|**Restrict.**|**Description**|
|**`aTypeSur`**|`PC`|`PC`|||Package surcharge|
|**`aTypeSur`**|`TP`|`TP`|||Transport surcharge|
|**`aTypeSur`**|`MO`|`MO`|||Installation surcharge|
|**`aTypeSur`**|`SQ`|`SQ`|||Less quantity surcharge|
|**`aTypeSur`**|`PS`|`PS`|||Processing surcharge|
|**`aTypeSur`**|`S1 – S9`|`S1 – S9`|||Other surcharges (1 – 9)|
|**`aUUID`**|~~**`CHAR(UUID)`**~~|**36**|||**Universally Unique Identifier**|



                               - 51 

### **3 OEX – Scenarios**

**3.1** **Order with follow-up Order Change (ideal case)**


`ORDERS`  `ORDRSP`  `ORDCHG`  `ORDRSP`
Scenario: (1) orderer orders, (2) vendor confirms the order, (3) orderer modifies the order,
(4) vendor confirms the modification


**Timeline** **Buyer** **Vendor**



**1**


**2**


**3**


**4**







**3.2** **Order and Order Change (delayed to Order Confirmation)**


`ORDERS`  `ORDCHG`  `ORDRSP`  `ORDCHG`  `ORDRSP`
Scenario: (1) orderer orders, (3) orderer modifies the order before it is confirmed, (3) vendor
confirms order including modification, (4) orderer changes the order again, (5) vendor confirms


**Timeline** **Buyer** **Vendor**



**1**


**2**


**3**


**4**


**5**








- 52 

**3.3** **Order containing changes caused by vendor**


`ORDERS`  `ORDRSP`  `ORDRSP`
Scenario: (1) orderer orders, (2) vendor confirms, (3) vendor modifies the order (e.g. delivery date)


**Timeline** **Buyer** **Vendor**



**1**


**2**


**3**







**3.4** **From Request to Invoice (ideal case)**


`REQOTE`  `QUOTES`  `ORDERS`  `ORDRSP`  `DESADV`  `INVOIC`
Scenario: (1) orderer sends a request, (2) vendor sends a quote, (3) orderer orders,
(4) vendor confirms, (5) vendor advises delivery, (6) vendor invoices


**Timeline** **Buyer** **Vendor**



**1**


**2**


**3**


**4**


**5**


**6**








- 53 

**3.5** **From Request to Invoice including Order Change (ideal case)**


`REQOTE`  `QUOTES`  `ORDERS`  `ORDRSP`  `ORDCHG`  `ORDRSP`  `DESADV`  `INVOIC`
Scenario: (1) orderer sends request, (2) vendor sends quote, (3) orderer orders,
(4) vendor confirms, (5) orderer sends order modification, (6) vendor confirms order modification,
(7) vendor announces delivery, (8) vendor invoices.


**Timeline** **Buyer** **Vendor**



**1**


**2**


**3**


**4**


**5**


**6**


**7**


**8**








- 54 

### **4 Appendix**

**4.1** **History of modification**









|Version|Modification|
|---|---|
|3.1.0 – May 8, 2023|1 Introduction<br>Advanced explanations<br> <br>2.1.3 OEX Value Types<br>New:<br> `FolderIsLOC`<br>– Is the folder name an indication of location?<br> <br>2.2 Data Domains<br>Deleted (unused):<br> `_Phone`<br> <br>– Number for Telephone etc (see general frame type`Com`) <br>Correction:<br> _TextType<br> <br>- Text types<br>For text type`ARTM` the attribute`aStatus` of`_VendorArtNo` must have the value`M`. <br>Extension:<br> _Configuration<br>– Feature of the configuration<br>New attribute<br>`aIsVisible` <br>– is visible?<br> <br>2.3 Data Types<br>`CHAR`– All characters of the basic code-page of the OEX-document<br> <br>Deleted option (unused):`PHONE` – Phone numbers<br>Deleted option (unused):`NUMCHAR` – Digits and Letters<br> <br>2.4 Attributes<br>New:<br> `aIsPseudo` <br>– does the item represent a pseudo article?<br> `aIsVisible`<br>– is the feature visible to the user?<br>|
|3.0.1 – March 8,<br>2023|2.2 Data Domains<br>Added missing values (from original German issue):<br> _ContactType<br>– Types of contact<br>value<br>`EM` <br>– Employee<br>value<br>`CL` <br>– Client<br> `_OrderType`<br>– Types of orders<br>value<br>`EO` <br>– Order for own employee|
|3.0.0 – 30.11.2017<br>(to be cont.)|1.1 Overview of OEX Specifications<br>New major versions of document types:<br> `REQOTE` – Request<br> `QUOTES` – Quotationt<br> `ORDERS` – Order<br> `ORDRSP` – Order confirmation<br> `ORDCHG` – Order change<br> `DESADV` – Dispatch Advice<br> `INVOIC` – Invoice<br> <br>2.1.2 OEX Frame Types<br>Changed:<br> 2.1.2.7`DateTime`: Date and time details<br>Domain`_DateTime` ► is substituted for`_Frame` <br> 2.1.2.8`OrgData`: Organizational data<br>Domain`_OrgData` ► is substituted for`_Frame` <br> 2.1.2.9`Address`: Addresses<br>Domain`_Address` ► is substituted for`_Frame` <br> 2.1.2.12`Text`: Texts<br>Domain`_Text` ► is substituted for`_Frame` <br> 2.1.2.13`Reference`: References<br>Domain`_Reference` ► is substituted for`_Frame` <br> 2.1.2.14`Pricing`: Pricing<br>`<QuantUnit>`<br>– Quantity Unit ► is substituted for`<MeasureUnit>`<br> 2.1.2.15`Config`: Configuration data<br>Domain`_Configuration` ► is substituted for`_Frame`|



                   - 55 

|Version|Modification|
|---|---|
|3.0.0 – 30.11.2017<br>(continued)|2.1.3 OEX Value Types<br>New:<br> `Quantity`<br>– Quantity<br> `QuantUnit`<br>– Quantity Unit<br>Deleted (general type`Quantity` is used instead):<br> `ChgOrdQuant`<br>– Changed order quantity<br> `ConfOrdQuant` <br>– Confirmed order quantity<br> <br> `DelivQuantity` <br>– Delivered quantity<br> `InvoiQuantity` <br>– Invoiced quantity<br> `OrderQuantity` <br>– Order quantity<br> `QuoteQuantity` <br>– Quotation quantity<br> `RequQuantity` <br>– Request quantity<br>Deleted (general type`QuantUnit` is used instead):<br> `ChgOrdUnit`<br>– Unit of changed order quantity<br> `ConfOrdUnit` <br>– Unit of confirmed order quantity<br> `DelivUnit` <br>– Unit of delivered quantity<br> `InvoiUnit` <br>– Unit of invoiced quantity<br> `OrderUnit` <br>– Unit of order quantity<br> `QuoteUnit` <br>– Unit of quotation quantity<br> `RequUnit` <br>– Unit of request quantity<br>Deleted (general type`DocNo` is used instead):<br> `DeliveryNumber`– Delivery number<br> `InvoiceNumber` <br>– Invoice number<br> <br> `OrdChangeNo` <br>– Order change number<br> `OrdConfirmNo` <br>– Order confirmation number<br> `OrderNumber` <br>– Order number<br> `QuoteNumber` <br>– Quotation number<br> `RequestNumber` <br>– Request number<br>Renamed<br> `DocLine`►`DocItemNo`– Number of document item<br>Deleted:<br> All value types in domain`_PosNo` except`DocItemNo` (general typ`DocItemNo` is used instead).<br> <br>2.2 Data Domains<br>New:<br> `_Address`<br>– Addresses<br> `_ClientArtNo`<br>– Article number oft he client<br> `_Configuration`– Feature of the configuration<br> `_DateTime`<br>– Date and time details<br> `_OrgData`<br>– Organizational data<br> `_Reference`<br>– References<br> `_Text`<br> <br>– Texts<br> <br>Extension:<br> New attribute`aAction` in:<br>_`AddStateCode`, _`CatalogId`, _`Classification`, _`CompSubArtId`, `_Pricing`, _`Quantity`, <br>_`Unit`, _`VendorArtNo`, _`VendorID`, _`VendorSer` <br> _Item<br> <br>– Document item<br>New attribute<br>`aUUID` – Universally Unique Identifier<br> `_OrgDataType`<br>– types of organizational data<br>new value<br> <br>`COG` <br>– class of goods<br>new value<br> <br>`OVC` <br>– OFML variant code<br> `_ReferenceType`– types of references<br>new value<br> <br>`XML` <br>– embedded CDATA element<br> `_TextType`<br>– types of texts<br>new value<br> <br>`ARTV` – variant text<br>new value<br> <br>`ARTU` – user-defined additional text<br>Change:<br> `_Header`<br> <br>– Document header<br>Attribute`aAction` is no longer mandatory<br> `_Item`<br> <br>– Document item<br>Attribute`aAction` is no longer mandatory<br>Attributes<br> <br>`aItemTypeClient` <br>– Type of client’s document item<br> <br> <br> <br>`aItemTypeVendor` <br>– Type of vendor’s document item<br>substituted by<br>`aItemType`<br> <br>– Type of document item|



                              - 56 

|Version|Modification|
|---|---|
|3.0.0 – 30.11.2017<br>(continued)|2.3 Date types<br>`CHAR`– All characters of the basic code-page of the OEX-document<br> <br>New option:`UUID` – Universally Unique Identifier<br> <br>2.4 Attributes<br>New:<br> `aMustCheck`<br>– Is relevant for checks?<br> `aUUID`<br> <br>– Universally Unique Identifier<br>Change:<br> `aAction` <br> <br>– Aktion<br>Added specification of default value if attribute is not mandatory in an element.<br>Correction:<br> `aBuild`,` aMajor`,` aMinor` – Version numbers<br>The number of digits no longer is limited to 2.<br>|
|2.3.1 – 13.1.2017|2.1.3 OEX Value Types<br>Added missing types:<br> `ChgOrdQuant`<br>– changed order quantity<br> `ChgOrdUnit`<br>– changed order unit<br> `OrdChangeNo`<br>– order change number<br> `OrdChgCompNo`<br>– number of the order change item oft he composite article<br> `OrdChgItemNo`<br>– order change item number<br> `OrdChgTopLevl`<br>– higher level order change item number<br> `QuoteItemNo`<br>– number of quotation item<br> `RequestItemNo`<br>– number of request item<br>2.2 Data Domains<br> `_DocNoType`<br>– types of document numbers<br>Added missing value:` CHG` – order change number|
|2.3.0 – 1.7.2015|1.1 Overview of OEX Specifications<br>New minor versions of document types:<br> `REQOTE` – Request<br> `QUOTES` – Quotationt<br> `ORDERS` – Order<br> `ORDRSP` – Order confirmation<br> `ORDCHG` – Order change<br> `DESADV` – Dispatch Advice<br> `INVOIC` – Invoice<br> <br>2.1.3 OEX Value Types<br>New:<br> `Classification`– universal classification<br> <br>2.2 Data Domains<br>New:<br> `_Classification`– universal classification<br> <br>2.4 Attributes<br>New:<br> `aClassSystem` <br>– classification system (3 predefined systems/values)<br>Extension:<br> `aCondArea` <br>– condition area<br>new value:<br> <br>`OP` <br>– original purchaces price of furnisher<br>new value:<br> <br>`OS` <br>– original sales price of furnisher<br>Change:<br> `aTaxCode` <br>– tax code<br>extended length to 3 digits, numbers 1-6 now are predefined (for standardized tax categories of VAT) and<br>numbers 7-99 now are reserved (for possible future standardization)|
|2.2.0 – 11.10.2013<br>(to be cont.)|1.1 Overview of OEX Specifications<br>New minor versions of document types:<br> `REQOTE` – Request<br> `QUOTES` – Quotationt<br> `ORDERS` – Order<br> `ORDRSP` – Order confirmation<br> `ORDCHG` – Order change<br> `DESADV` – Dispatch Advice<br> `INVOIC` – Invoice<br>|



                              - 57 

|Version|Modification|
|---|---|
|2.2.0 – 11.10.2013<br>(continued)|2.1.2 OEX Frame Types<br>Extension:<br>2.1.2.9`Address`: Address<br>new optional elements<br> `<AddressID>`<br>– address ID ► substitutes`<ILN_AddressID>` <br> `<Street2>`<br>– street 2<br> `<District>`<br>– district<br>Deleted:<br> `<ILN_AddressID>`– ILN-Nummer Adresse<br> <br>Examples:<br>2.1.2.13`Reference`: changed example and added one<br> <br>2.1.3 OEX Value Types<br>New:<br> `Street2`<br> <br>– street 2<br> `District`<br>– district<br> `CatalogId`<br>– catalog ID<br> `CompSubArtId`<br>– identification of sub article<br> `AddStateCode`<br>– additional state information<br> `ClientID`<br>– client ID ► substitutes`ILN_Client` <br> `ClientClass`<br>– client classification<br> `SupplierID`<br>– supplier ID ► substitutes`ILN_Vendor` <br> `SupplierClass`<br>– supplier classification<br> `AddressID`<br>– address ID ► substitutes`ILN_Address`<br>Deleted:<br> `ILN_Address`<br>– ILN of address<br> `ILN_Client`<br>– ILN of client<br> `ILN_Vendor`<br>– ILN of supplier (vendor)<br>Correction (typo):<br> `OrderCompNo ► OrderComposNo`– Number of order item of the composite article<br> <br>2.2 Data Domains<br>New:<br> `_CatalogId`<br>– catalog ID<br> `_CompSubArtId`<br>– identification of sub article<br> `_AddStateCode`<br>– additional state information<br> `_BusPartID`<br>– business partner ID ► substitutes`_ILN` <br> `_BusPartClass`<br>– business partner classification<br>Extension:<br> `_ReferenceType`– type of reference:<br>new value “`EDS` – Embedded Data Stream as Base64”<br>new attribute`aMIMEType` – MIME-Typ<br> `_AddressType`<br>– type of Address<br>new value „`IL` – installation location“<br>Change:<br> `_OrgDataType`<br>– type of organizational data<br>removed value „`PRL` – price list“, replaced by catalog ID<br>Deleted:<br>`_ILN`<br> <br>– ILN code<br>2.3 Data types<br>`CHAR` – all characters of the basic code-page of the OEX-document<br>New option:<br> `RX001` – regular expression 001 (affected domain:`_CatalogId`) <br> <br>2.4 Attributes<br>New:<br> `aMIMEType` <br> <br>– MIME type<br> `aBusPartClassType`<br>– type of business partner classification<br> `aBusPartIDType`– type of business partner ID|
|2.1.0 – 09.02.2010|Initial English version|



                              - 58 

