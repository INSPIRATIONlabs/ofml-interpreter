### OFML – Standardized Data Description Format of the Office Furniture Industry Version 2.0 3rd revised edition

Copyright c _⃝_ 1998 – 2015
Der Verband B¨uro-, Sitz- und Objektm¨obel e.V. (BSO)


November 4, 2015


Copyright c _⃝_ 1998 – 2015
Verband B¨uro-, Sitz- und Objektm¨obel e.V. (BSO)
Bierstadter Strasse 3
**D-65189 Wiesbaden**
_www.buero-forum.de_


The scientific support and coordination of the development of the OFML data standard was performed by
Dr. Ing. habil. Ekkehard Beier from the Institute of Practical Informatics and Media Informatics of the
Faculty of Informatics and Automation at the Technical University Ilmenau.


Ekkehard Beier holds the intellectual copyright for the OFML object model, including the scene architecture, rules, and base interfaces. Referring to this, any scientific, patent-related or in any other way
copyright-related exploitation requires the permission of Ekkehard Beier.


The OFML standard (parts I-III) was developed by EasternGraphics GmbH on behalf of industrial association B¨uro-, Sitz- und Objektm¨obel e.V. (BSO).


EasternGraphics GmbH holds the intellectual copyright for the segments Global Planning Types, Product
Data Model, and Planning Environment. The same applies to the OFML Database (ODB), the OFML
Metafile Format EGM, and the OFML 2D Interface. Referring to this, any scientific, patent-related or in
any other way copyright-related exploitation requires the permission of EasternGraphics GmbH.


Basic syntax and semantics of OFML are based on the _Cobra_ programming language from EasternGraphics
GmbH. Copyright c _⃝_ 1995 – 2015 EasternGraphics GmbH and Jochen Pohl.


The OFML standard was developed with great care. Nevertheless, mistakes and inconsistencies cannot

be ruled out. The industrial association B¨uro-, Sitz- und Objektm¨obel e.V. as well as EasternGraphics

GmbH refuse to accept any respective liability in this regard.


# **Contents**

References . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 6


**1** **Introduction** **7**


**2** **Concepts** **10**


2.1 Types . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 10


2.2 Entities . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 11


2.3 Property . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 14


2.4 Methods . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 14


2.5 Rules . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 15


2.6 Categories . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 16


2.7 Initialization . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 16


2.8 Interactors . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 17


**3** **Basic Syntax and Semantics** **18**


3.1 Introduction . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 18


3.2 Lexical Structure . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 19


3.3 Types . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 25


3.4 Predefined Reference Types . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 30


3.5 Statements . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 43


3.6 Expressions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 51


3.7 Packages and Namespaces . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 64


3.8 Classes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 69


3.9 Predefined Functions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 73


1


**4** **Basic Interfaces** **77**


4.1 MObject . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 77


4.2 Base . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 79


4.3 Material . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 87


4.4 Property . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 89


4.5 Complex . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 94


4.6 Article . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 98


**5** **Predefined Rule Reasons** **103**


5.1 Element Rules . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 103


5.2 Selection Rules . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 105


5.3 Move Rules . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 105


5.4 Persistence Rules . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 107


5.5 Other Rules . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 108


**6** **Global functions** **109**


6.1 Formatted Output . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 109


6.2 oiApplPaste() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 110


6.3 oiClone() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 110


6.4 oiCollision() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 111


6.5 oiCopy() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 111


6.6 oiCut() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 111


6.7 oiDialog() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 111


6.8 oiDump2String() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 112


6.9 oiExists() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 113


6.10 oiGetDistance() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 113


6.11 oiGetNearestObject() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 113


6.12 oiGetRoots() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 113


6.13 oiGetStringResource() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 113


6.14 oiLink() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 114


6.15 oiOutput() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 114


6.16 oiPaste() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 114


6.17 oiReplace() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 115


6.18 oiSetCheckString() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 115


6.19 oiTable() . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 115


2


**7** **Geometric types** **117**


7.1 OiGeometry . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 117


7.2 OiBlock . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 118


7.3 OiCylinder . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 119


7.4 OiEllipsoid . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 120


7.5 OiFrame . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 121


7.6 OiHole . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 122


7.7 OiHPolygon . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 124


7.8 OiImport . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 125


7.9 OiPolygon . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 126


7.10 OiRotation . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 127


7.11 OiSphere . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 128


7.12 OiSweep . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 129


7.13 OiSurface . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 131


**8** **Global Planning Types** **132**


8.1 OiPlanning . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 132


8.2 OiProgInfo . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 140


8.3 OiPlElement . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 142


8.4 OiPart . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 148


8.5 OiUtility . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 153


8.6 OiPropertyObj . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 153


8.7 OiOdbPlElement . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 154


**9** **Types for Product Data Management** **156**


9.1 OiPDManager . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 157


9.2 OiProductDB . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 160


**10 Types of the Planning Environment** **164**


10.1 The Wall Interface . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 164


10.2 OiLevel . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 164


10.3 OiWall . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 166


10.4 OiWallSide . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 166


3


**A Product Data Model** **167**


**B The 2D Interface** **169**


B.1 Introduction . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 169


B.2 The 2D Object Hierarchy . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 169


B.3 Coordinates . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 170


B.4 Methods . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 170


B.5 Object Types . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 171


B.6 Attributes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 174


**C The 2D vector file format** **180**


C.1 Introduction . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 180


C.2 data types . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 180


C.3 File header . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 186


C.4 General structured data types . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 187


C.5 Graphic 2D objects . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 188


**D External data formats** **200**


D.1 Geometries . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 200


D.2 Materials . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 201


D.3 Fonts . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 203


D.4 External Tables . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 204


D.5 Text Resources . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 204


D.6 Archives . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 205


**E Format Specifications** **206**


E.1 Format Specifications for Properties . . . . . . . . . . . . . . . . . . . . . . . . . . 206


E.2 Definition Format for Properties . . . . . . . . . . . . . . . . . . . . . . . . . . . . 207


**F Additional Types** **209**


F.1 Interactor . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 209


F.2 Light . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 210


F.3 MLine . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 211


F.4 MSymbol . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 213


F.5 MText . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 214


4


**G Applied Notation** **216**


G.1 Class Diagrams based on Rumbaugh . . . . . . . . . . . . . . . . . . . . . . . . . . 216


**H Categories** **218**


H.1 Interface Categories . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 218


H.2 Material Categories . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 218


H.3 Planning Categories . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 219


**I** **Terms** **220**


**Index** **223**


5


# **References**


[GO] EasternGraphics GmbH: _GO – Generic OFML types (OFML part II)_ .


[OAM] Verband B¨uro-, Sitz- und Objektm¨obel e.V.: _OAM – OFML Article Mappings (OFML Part_
_VI)_ .


[OAS] Verband B¨uro-, Sitz- und Objektm¨obel e.V.: _OAS – OFML Article Selection (OFML Part V)_ .


[OCD] Verband B¨uro-, Sitz- und Objektm¨obel e.V.: _OCD – OFML Commercial Data (OFML Part_
_IV)_ .


[ODB] EasternGraphics GmbH: _ODB – OFML database (OFML part I)_ .


[OEX] Verband B¨uro-, Sitz- und Objektm¨obel e.V.: _OEX – OFML Business Data Exchange (OFML_
_Part VII)_ .


[Rumb91] J.Rumbaugh et al: _Object–Oriented Modelling And Design_ . Prentice–Hall, New Jersey, 1991


6


## **Chapter 1**

# **Introduction**

The motivation for the new standard of office furniture (OFML [1] ) is the result of a series of requirements that could generally not be met with past and present solutions:


_•_ The new requirements in the area of planning and visualization of (office) furniture cannot be
met sufficiently by CAD-based systems. The main problems of CAD-based solutions are their
enormous data size, the poor parameterizability and configurability, insufficient coverage of
product logics, insufficient display quality in the interactive range, complicated operation,
and costly licensing.


_•_ These disadvantages are magnified in the area of marketing-oriented solutions that may, for
example, set the framework for using end user-oriented systems on CD-ROM or the Internet.


_•_ A platform-independent and (software) manufacturer-independent data format allows an unlimited number of software manufacturers to offer systems and solutions so that monopolizing
conditions can be avoided or eliminated.


_•_ The new data format also allows for the implementation of a series of applications that are
compatible with respect to the data in spite of a different orientation. In this way it is possible
to achieve a compatibility and, therefore, technological uniformity between manufacturer,
trade, and end user systems.


(Traditional) CAD systems continue to have a raison d’tre, especially through their abilities in the
design and manufacturing sector. Consequently, the new standard does not lay claim to a complete
removal of existing CAD-based solutions. Instead, a coexistence between traditional CAD solutions
and the new standard is aimed at. The coexistence should be implemented on the basis of directly
compatible data formats or suitable conversion tools.


In particular, the OFML standard offers the following features:


_•_ consistent application of the object-oriented paradigm,


1 _Office Furniture Modeling Language_


7


_•_ conversion of concepts of semantic modeling to achieve a match of virtual objects with actual
products,


_•_ combination of geometric, visual, interactive, and semantic features of real products in a
uniform and holistic data model,


_•_ mapping of real configuration logics and parametrics,


_•_ independence of system or interface platforms, and


_•_ independence of a concrete runtime environment.


The OFML standard consists of the following **parts**, each covering different aspects of OFML
data creation or various application processes. The parts are more or less strongly linked together,
primarily by cross-reference such as article numbers and type identifiers.


1. OFML database (ODB)


The OFML database [ODB] defines a table-based interface for description of hierarchical
geometries in 2D and 3D.


2. Generic Office library (GO)


The class library GO [GO] provides basic functionality for the scope of the office furniture
industry.


3. Object model


This part defines a complete programming language, basic interfaces of OFML types, predefined rule reasons, global (type-independent) functions as well as a set of base types. On the
basis of this object model arbitrarily complex data can be created and external commercial
data can be integrated.


4. OFML Commercial Data (OCD)


OCD [OCD] defines a set of tables for the creation of (commercial) product data which is
needed and exchanged within business processes of the furniture trade. Primarily, OCD
is supposed to cover tasks like configuration of complex articles, price determination and
creation of offer resp. order forms.


5. OFML Article Selection (OAS)


OAS [OAS] describes a format for structured representation and selection of articles in digital
catalogs.


6. OFML Article Mappings (OAM)


The tables specified in this part [OAM] are used to define more complex relationships between
data that has been created according to the specification of various other OFML parts.


7. OFML Business Data Exchange (OEX)


OEX [OEX] describes a format for the electronic exchange of business documents, such as
purchase orders and invoices.


8


Parts I-III were developed by EasternGraphics GmbH on behalf of industrial association B¨uro-,
Sitz- und Objektm¨obel e.V. (BSO). All other parts are specified by the standardization committee
of the BSO.


In the following, in this document only the object model (part III) is described. All other parts
are specified in separate documents (see references above).


This document is structured as follows:


**Introduction and Overview**


_•_ This chapter describes motivation, features and the parts of the OFML standard, and presents
an overview of the document.


_•_ Chapter 2 summarizes the relevant concepts and metaphors of the object model.


**OFML part III (Object model)**


_•_ Chapter 3 describes the basic syntax and semantics of the programming language underlying
OFML.


_•_ Chapter 4 presents an overview of the basic interfaces that form the basis for the concrete
types of the standard.


_•_ Chapter 5 describes the set of predefined rule reasons.


_•_ Chapter 6 describes the set of type-independent functions predefined for OFML.


_•_ Chapters 7 and 8 describe the complete set of OFML basic types.


_•_ Chapter 9 specifies types that are required for access to external product data.


_•_ Chapter 10 describes the generic types of planning environments.


**Appendix**


_•_ Appendix A describes a generic format for the external writing of product data.


_•_ Appendix B documents an explicit 2D programming interface available in OFML.


_•_ Appendix C documents a metafile format that is used by OFML to describe 2D vector
graphics.


_•_ Appendix D documents the set of external data formats and their application.


_•_ Appendix E describes the formats relevant for using properties.


_•_ Appendix F describes the additional types that may be applied within the framework of
OFML.


_•_ Appendix G describes the notational conventions used within the framework of this standard.


_•_ Appendix H describes the categories predefined within the framework of this standard.


_•_ Appendix I defines the most important terms used within the framework of this standard.


9


## **Chapter 2**

# **Concepts**

This chapter contains a description of the basic OFML concepts. All concepts documented in the
subsequent chapters are invariably based on these fundamentals. As such, an understanding of
these concepts is a necessary basis for additional dealings with the standard.

#### **2.1 Types**


A type [1] is a combination of entities of the same kind. A type defines the following for these entities:


_•_ a set of methods,


_•_ a set of rules,


_•_ a set of instance variables, and


_•_ exactly one initialization function.


Each instance belongs to exactly one immediate type. A type may have only one direct super type;
the features are inherited from this super type in a certain way. As such, a type should always be
considered in connection with its (direct or indirect) super types.


A type name must be unique within the defining module. A type name must also be unique within
the global context. This is accomplished using a uniform name prefix or by integrating it in a
name range.


A type is either abstract or concrete. An abstract type cannot be used to form entities.


**Example:** The concept of a carcass cabinet can be interpreted as type. A certain carcass cabinet (type)

that also features a corresponding order number is one example of a concrete type. The generalization of

all carcass cabinet types is an example of an abstract type.


1The terms type and class are synonymous.


10


The term interface resembles the term type with respect to its use within OFML. However, the
following exception exists:


_•_ An interface is a descriptive tool and does not necessarily correspond to a type.


_•_ An interface is not derived from another interface.


_•_ The name of an interface does not receive a name prefix.

#### **2.2 Entities**


An instance [2] is a concrete embodiment of a type. It distinguishes itself from other entities through
its identity which is implemented through a hierarchical name. In general, it also distinguishes
itself through the assignment of the instance variables of which it always possesses its own copy.


**Example:** Two carcass cabinets with the same order number are referred to as entities of the same type.

They have common features, such as the same order number or the same physical dimensions. They differ

from each other, for example, through the or the material design.


In general, entities should be **topologically independent** . This means:


_•_ An instance should not store any object references in its instance variables, that is, name
references to objects.


**Example:** This would be violated if one instance _remembers_ a certain other instance (e.g., on the

same topological level).


_•_ An instance cannot assume that its topological ancestors are from a certain type.


**Example:** In the course of the temporary generation of entities, any random instance can be an

ancestor of an instance.


Under special circumstances, these rules may be violated. The resulting consequences may include:


_•_ loss of ability to save and


_•_ incorrect behavior during copying and inserting.


2The terms instance and object are synonymous.


11


**2.2.1** **Children**


An instance can have a number of children. A child is an instance that exists in the name space
of the father object. The father-child-relation is described as follows in OFML:


_•_ Children are generated, modified and deleted during runtime. As such, the number of children
is time-dependent.


_•_ The father must be indicated at the generation of an instance and cannot be changed afterwards.


_•_ Deleting an instance always results in the deletion of its children.


_•_ A child inherits the features of its father in a certain way. For example, the complete global
space modeling of the child results from interlinking the global space modeling of the father
and the local space modeling of the child.


_•_ A child knows its father. This fact may be used for an upward traversing within the scene.


_•_ A father knows his children. This fact may be used for a downward traversing within the
scene.


Entities are placed in a scene. Based on the features of the father-child relation described above,
the resulting scene topology is a set of trees.


The set of elements is a subset of the set of children. An element is a special child whose generation
and removal can be controlled via rules (see below). Thus, every element is a child, but not every
child is an element. Elements are normally used for accessible components of a complex instance.
Non-elements are normally components of a combined instance that evade access by the user.


**Example:** The children of a carcass cabinet are stringer, back wall, base, front, and built-in components.
The shelves are elements that can be inserted, moved, and deleted separately.


The individual boards of the carcass, on the other hand, are non-elements since there is usually no access
to them.


Moving a shelf is controlled by the father of the shelf, that is, the carcass cabinet (rasterization, avoiding
collision, sector monitoring). The shelf must, therefore, know its father to transfer control over the desired
move to him.


If the carcass cabinet is moved, the children must be moved accordingly. For this reason, the children must
be known to the father.


If a carcass cabinet is deleted, all shelves, etc. of this carcass cabinet are automatically deleted.


Syntactically, children are treated like instance variables (Section 3.8.4). Since they are created
dynamically, access by name within methods must be carried out using a prefixed _self_ in addition
to an access operator, e.g., for the child _b5_ : _self.b5_ .


12


**2.2.2** **Instance Identity**


The identity of an instance is implemented by means of a hierarchical name space. Every name
within this name space corresponds biuniquely to a topological position in the object world (scene).
The name of an instance results from the following rule:

```
  Name : Name(Father) ’.’ LocalName
        | LocalName

  LocalName : Character
        | LocalName Character

  Character : ’A’ - ’z’ | ’0’ - ’9’ | ’_’

```

Consequently, the name of an instance results from the interlinking of the name of the father, if
one exists, via a point-to-point operator with the local name.


**Example:**


_• env_  - is the name of a fatherless root object.


_• env.sky_   - is the name of a child of _env_ . The local name is _sky_ .


_• env.sky-1_   - is an invalid name.


_• env.sky_ ~~_1_~~   - is a valid name.


_• env.env_  - is the name of a child of _env_ and designates a sibling object of _env.sky_ at the same time.


_• top_  - is the name of another fatherless root object.


_•_  - is not allowed, neither as global nor as local name.


The following absolute names are predefined:


_• t_  - is the root object that combines the planning hierarchy.


_• e_  - is the root object that combines the environment hierarchy, if necessary.


_• m_  - is the root object that combines the dimensioning hierarchy, if necessary.


At the same time, additional root objects can be defined for specific applications.


**Restriction:** The (local) names in the form _e<n>_, whereby _n_ is a natural number, are reserved and may

not be assigned explicitly. These names are assigned automatically during the generation of elements.


13


**2.2.3** **Instance Variables**


A type (in combination with its super types) defines a set of instance variables of which each
instance owns its own copy. The conventions dictate that the name of an instance variable consists
of the prefix _m_ plus a non-empty set of words that each start with a capital letter. In addition,
the name of an instance variable is a valid designator as defined by the basic syntax (Chapter 3).
Examples for valid names of instance variables include: _mWidth_ and _mIsCutable_ .


An instance variable that is defined in a type, may not be re-defined in a derived type. In addition,
an instance variable must at least be initialized in the type in which it was defined. Direct access
to an instance variable is permitted only within the defined type. An external access can be
accomplished only via respective methods.


Instance variables may also be defined via interfaces.


**Example:** An instance variable could be used to define whether a roll container features an espagnolette

or not.

#### **2.3 Property**


A property ( _property_ ) is a special instance variable that represents an implicit interface of an
instance to the (graphical) user interface. A property has a type, a symbolic designator, and an
actual value. In most cases, a discrete value range is assigned to a property. Additional optional
features of a property include the initial assignment as well as usually for geometric properties
the minimum value and the maximum value.


The current embodiment of the set of properties of an instance generally corresponds to a concrete
article number.


Properties are read out by a ( _property editor_ ) and can interactively be set by this editor.


The concept of properties allows for combining any large set of configurations that correspond to
exactly one article number each by using a type that covers all possible configurations, while also
considering dependencies between individual properties.


**Example:** The (interactive) configurability of a carcass cabinet can be implemented using the three

properties _width_, _height_, and _depth_ . In general, a manufacturer-specific discrete value range is defined for

each of these properties, e.g., for the width: _600 mm_, _800 mm_, _1000 mm_, and _1200 mm_ .

#### **2.4 Methods**


A type (in combination with a super type) defines a set of methods or type-specific functions
(Section 3.8.4). The name of a method results from a non-empty set of words that all start with a
capital letter, except for the first one. In addition, the name of a method is a valid designator as


14


defined by the basic syntax (Chapter 3). Examples for valid names of methods include: _selectable()_
and _isSelectable()_ .


A method that is defined in a type, may be redefined in a derived type only if it features the same
signature. In the case of OFML this means that number, format, and semantics of the parameters
must be identical.


Methods may also be defined via interfaces.


**Example:** The stop change of a door can be implemented via a corresponding method. This method then

implements the stop change without the internal design of the door being known to the outside.

#### **2.5 Rules**


A type (in combination with its super types) defines a set of rules. A rule is a procedural construct
that is defined analogous to a method within the range of a type. A rule differs from a method
through the following features:


_•_ A rule is a type-dependent construct whose signature consists of a rule reason in form of a
predefined or user-defined symbols, an optional specific rule parameter, and a formal parameter.


_•_ The return value of a rule is of type _Int_ . The value 0 signals the successful processing of the
rule. The value _−_ 1 denotes a failed rule. The user can be informed about the failure of a
rule, if required, through the use of a corresponding text message.


_•_ Several rules may exist for one and the same rule reason within a type or a hierarchy of a
type.


_•_ A rule cannot be overwritten, for example, by a rule with identical reason in a derived type.


_•_ A rule is classified as anterior rule or posterior rule. An anterior rule is called before an
action is performed. The failure of an anterior rule prevents the corresponding action from
being performed. A posterior rule is called after an action was performed. Consequently, this
action cannot be prevented. However, the effect of the action can be reversed by applying a
suitable counter-action.


For an action that was performed or still needs to be performed and a given instance, a list is first
compiled that contains the rules defined by the type and its super type for the respective reason.
The order of the rules in the list corresponds to the derivative hierarchy of the respective type.
That is, a rule defined by a certain type is located in the list ahead of a rule defined by a derived
type. The list of rules is then processed sequentially. Processing is interrupted provided that a
rule has failed. In this case, and if the rule was an anterior rule, the corresponding action is not
performed.


The rule reasons predefined in OFML are documented in chapter 5.


15


**Example:** Inserting any object, e.g. in a carcass cabinet, can be controlled by a corresponding anterior
rule. For example, the carcass cabinet can ensure using this rule that only shelves of a certain type and a
certain number can be inserted.


Moving an object can be controlled by a corresponding posterior rule. For example, if the move results in

a collision, the move should subsequently be corrected accordingly.

#### **2.6 Categories**


A category is a classification of types or entities that results from a certain philosophy.


Categories represent an extension to the concept of types: types that belong to a common category
do not have to be derived from a common type. In addition, a type can be assigned to several
categories.


The association with a category is determined by each type itself. It can be determined for an
instance whether its type or super types belong to a certain category (Section 4.1).


The concept of categories can be used to circumvent the limitation of simple inheritance of types
in the classification of entities based on orthogonal categorization criteria. It is also useful if _rolls_
must be modeled.


**Examples** Material and planning categories (see Appendix H).

#### **2.7 Initialization**


The initialization of an instance is carried out via the _initialize()_ procedure. The functions of
initialization are essentially the initialization of instance variables and the generation of child
objects. The following properties refer to the initialization:


_•_ Exactly one initialization function exists for each type. It is labeled _initialize()_ .


_•_ Within the implementation of the initialization function, the initialization function of the
direct super type is called first.


The standard signature for the initialization function is as follows:


_initialize(pFather(MObject), pName(Symbol)) →_ _MObject_


Where _pFather_ is the father object and _pName_ the local name of the new object to be created.
The return value of the initialization function is a reference to the created object.


If required, additional random parameters can be defined for the initialization function of a type.
However, this is only allowed for abstract types or internal components. All types that can be
instantiated interactively must conform with the standard signature of the initialization function.


16


**Example:** The initialization function of a carcass cabinet must create and parameterize the corresponding

children (stringer, base, back wall, etc.). However, the creation of shelves can be done interactively at a

later time.

#### **2.8 Interactors**


In OFML, interactors represent a special type that, in contrast to most other OFML objects, does
not represent an object of the real world. Interactors are objects that exist only at runtime and in
a simple way allow the user actions that go beyond elementary manipulations such as translation
and rotation. Corresponding examples are the marking of connection points or ”handles” for
interactively changing the size of an object.


Interactors distinguish themselves with respect to other objects through the following features:


_•_ They are not stored persistently.


_•_ They cannot be selected directly. The attempt to select an interactor triggers the _INTER-_
_ACTOR_ rule at the father (Section 5.5).


_•_ Interactors cannot cause a collision.


_•_ They are ignored during photorealistic output and export into an external data format.


**Example:** Designs can be mounted to an organizational wall at different positions. If interactors are

defined for these positions, the user can interactively select the desired mounting point.


17


## **Chapter 3**

# **Basic Syntax and Semantics**

#### **3.1 Introduction**

This chapter describes the programming language fundamentals of OFML whose syntax is oriented
to the programming languages C, C++, and Java. From a semantics point of view, OFML is similar
to Smalltalk or Python since it is based on a dynamic type concept.


**3.1.1** **Syntax Representation**


A slightly modified version of the familiar Backus-Naur form is used in this document to represent syntax. The following typographical conventions apply: reserved identifiers, characters and
character combinations are represented in `Schreibmaschine` . All other grammatical symbols are
written in _kursiv_ . Multiple alternatives for the right side of a production are separated either by
a linebreak and indent or by ”‘ _|_ ”’ within a line. Optional symbols are identified by a subscript
”‘ _opt_ ”’:


`{` _stmtopt_ `}`


**3.1.2** **Implementation**


The language definition of OFML assumes that an OFML program is converted into a processible
form by a compiler [1] . This takes place in two phases:


1. The translation o f al definitions to module and class levels. In this step, executable statements and definitions within compound statements are translated only partially or not at
all.


1This can be, for example, bytecode, machine code or vectored graphs.


18


2. The translation of all executable statements and definitions within compound statements.
Depending on implementation, this step can be delayed for each compound statement until
just before it is first processed.


The purpose of this division is to handle translation units that reference each other through variables, functions or classes defined by them. Translation units that form a loop based on the
super-classes they reference are not permitted.


Another reason for the division is to partially distribute the time needed to translate the program
to the runtime, which can be achieved through delayed translation of compound statements.


**3.1.3** **Program Structure**


An OFML program consists of one or more translation units. Each unit represents a sequence of
characters of the character set (see Section3.2.1), which can exist in the form of a file and string.
Each translation unit is conceptually closed by the _EOF_ (end of file) character. This character is
not a part of the source text, but instead is used only to represent the end of the input stream in
the syntax description.

#### **3.2 Lexical Structure**


The first pass during processing reads a sequence of input characters and produces as the result a
series of lexical symbols ( _Token_ ) [2] .


**3.2.1** **Character Set**


The character set processed by the compiler is the set of printable ASCII characters, i.e. 8-bit characters with an integer value from 32 to 126 and the control characters mentioned in Section 3.2.1.
Exceptions are permitted only in comments and literal characters and character string constants.
In the latter case, the programmer is responsible for ensuring that the corresponding characters are
processed correctly by the runtime environment. In the following, non-printable ASCII characters
are represented by hexadecimal numbers in the manner common to C; the grammatical _any-chars_
symbol denotes any sequence of characters from the entire character set of the implementation.


**Alphanumeric Characters**


The following productions define letters ( _alpha_ ), numbers ( _num_ ) and alphanumeric characters
( _alnum_ ). Note that the underscore also belongs to the letters.


2The English term is used here to avoid mix ups with OFML symbols.


19


_alpha_ :
`A` _|_ `B` _|_ `C` _|_ `D` _|_ `E` _|_ `F` _|_ `G` _|_ `H` _|_ `I` _|_ `J` _|_ `K` _|_ `L` _|_ `M`
`N` _|_ `O` _|_ `P` _|_ `Q` _|_ `R` _|_ `S` _|_ `T` _|_ `U` _|_ `V` _|_ `W` _|_ `X` _|_ `Y` _|_ `Z`
`a` _|_ `b` _|_ `c` _|_ `d` _|_ `e` _|_ `f` _|_ `g` _|_ `h` _|_ `i` _|_ `j` _|_ `k` _|_ `l` _|_ `m`
`n` _|_ `o` _|_ `p` _|_ `q` _|_ `r` _|_ `s` _|_ `t` _|_ `u` _|_ `v` _|_ `w` _|_ `x` _|_ `y` _|_ `z`


_num:_
`1` _|_ `2` _|_ `3` _|_ `4` _|_ `5` _|_ `6` _|_ `7` _|_ `8` _|_ `9` _|_ `0`


_alnum:_
_alpha | num_


**Spaces**


The following characters, as sequences or combined with comments (see Section 3.2.1) form _Zwischen-_
_r”aume_ ( _white-space_ ): horizontal tabs (HT), linebreaks (NL), vertical tabs (VT), formfeeds (FF),
carriage returns (CR) and spaces (SP). If an identifier or keyword follows an identifier, a keyword
or a symbol, both have to be separated by a white space. The same applies to integer constants
(excluded character constants) and floating-point constants. Otherwise, white spaces have no
meaning, but are used only to improve program readability.


_white-space:_
HT _|_ NL _|_ VT _|_ FF _|_ CR _|_ SP _| comment_


**Comments**


Comments begin with the `//` character combination and end with a linebreak (NL), carriage return
(CR) or a combination of the both.


_comment:_
`//` _any-chars eol_
_eol:_
CR _|_ NL _|_ CR NL _|_ NL CR


The `#` sign is different: If it occurs at the start of the first line of a file, the rest of the line is
interpreted as a comment.


20


**3.2.2** **Token**


There are various classes of token: keywords, identifiers, literal constants, operators and delimiters.


**3.2.3** **Identifiers**


_ident_ identifiers begin with a letter, which can be followed by any number of alphanumeric characters in sequence.


_ident:_
_alpha alnum-seq_
_alnum-seq:_
_alnum alnum-seqopt_


The keywords mentioned in the next section cannot be used as identifiers.


**3.2.4** **Keywords**


The following keywords are reserved and cannot be used as identifiers:

```
 abstract break case catch class
 continue default do else final
 finally for foreach func goto
 if import instanceof native operator
 package private protected public return
 rule self static super switch
 throw transient try var while

```

**3.2.5** **Literal Constants**


OFML includes literal constants of the following types (see Section 3.3): integers, floating-point
numbers, character strings and symbols.


_constant:_
_integer-constant_
_float-constant_
_string-constant_
_symbol-constant_


21


**Integer Constants**


Integer constants ( _integer-constant_ ) can be specified in three different numerical systems: decimal,
octal and hexadecimal. Because OFML does not distinguish character types, character constants
( _character-constant_ ) are also interpreted as integers.


_integer-constant:_
_dec-constant_
_oct-constant_
_hex-constant_
_character-constant_


Decimal numbers begin with a digit unequal to `0`, followed by any sequence of digits:


_dec-constant:_
_dec-start dec-restopt_
_dec-start:_
`1` _|_ `2` _|_ `3` _|_ `4` _|_ `5` _|_ `6` _|_ `7` _|_ `8` _|_ `9`
_dec-rest:_
_num dec-restopt_


Octal numbers begin with the digit `0`, followed by any sequence of digits from `0` to `7` :


_oct-constant:_
`0` _oct-restopt_
_oct-rest:_
_oct-num oct-restopt_
_oct-num:_
`0` _|_ `1` _|_ `2` _|_ `3` _|_ `4` _|_ `5` _|_ `6` _|_ `7`


Hexadecimal numbers begin with the `0x` or `0X` character string, followed by any sequence of digits
and the letters `A` to `Z` and `a` to `z` :


_hex-constant:_
_hex-start hex-rest_
_hex-start:_
`0X` _|_ `0x`
_hex-rest:_
_hex-num hex-restopt_
_hex-num:_
_num |_ `A` _|_ `B` _|_ `C` _|_ `D` _|_ `E` _|_ `F` _|_ `a` _|_ `b` _|_ `c` _|_ `d` _|_ `e` _|_ `f`


An integer constant must be smaller or equal to the largest representable value in the implementation. Otherwise, an error is generated during the translation.


22


Character constants consist of a character enclosed in single quotation marks ”‘ `’` ”’:


_char-constant:_
`’` _char-char_ `’`


The number of characters allowed in character constants is indicated by _char-char_ . The single
quotation mark itself is not allowed in character constants, nor are linebreaks. To represent these
and other special characters, use the escape sequences described in Section 3.2.5.


The value of a character constant is the numerical value of the character in the character set of
the runtime environment.


**Floating-point Constants**


Floating-point constants begin with an integer part, followed by a decimal point, the broken part
and the exponents. The exponent consists of the `E` or `e` character, an optional +/- sign and an
integer value. Either the integer or broken part, but not both, can be omitted. Furthermore, either
the decimal point or the exponent can be omitted.


_float-constant:_
_dec-rest_ `.` _dec-rest float-expopt_
_dec-rest_ `.` _float-expopt_
`.` _dec-rest float-expopt_
_dec-rest float-exp_
_float-exp:_
_exp-char signopt dec-rest_
_exp-char:_
`E` _|_ `e`
_sign:_
`+` _|_ `-`


If an underflow occurs during conversion of the floating-point constants in the Internal representation, the value of the constants is `0.0` . If an overflow occurs, it becomes `Float::HUGE_VAL` . If the
accuracy of the floating-point constants is greater than supported by the internal representation,
excess positions are ignored.


**Constant Strings**


Constant strings ( _string-constant_ ) consist of a sequence of characters enclosed in single quotation
marks (”‘ `"` ”’). The quotation mark itself is not allowed in character strings. To represent this and
other certain special characters, use the following escape sequences:


23


`\a` bell character (BEL)
`\b` backspace (BS)
`\t` horizontal tab (HT)
`\n` linebreak (NL)
`\v` vertical tab (VT)
`\f` formfeed (FF)
`\r` carriage return (CR)
`\"` quotation mark
`\’` single quotation mark
`\\` backslash
`\` _oct-rest_ octal character code
`\x` _hex-rest_ hexadecimal character code


The number of constant character string allowed is indicated by _string-char_ .


The _oct-rest_ octal character code consists of a sequence of up to three octal digits and ends with
the first not-octal character. The _hex-rest_ hexadecimal character code consists of a sequence of
any number of hexadecimal digits and ends with the first not-hexadecimal character.


If an overflow in a character occurs during translation while converting an octal or hexadecimal
character code, an error is generated.


_string-constant:_
`"` _string-char-seqopt_ `"`
_string-char-seq:_
_string-char string-char-seqopt_


**Literal Symbols**


Literal symbols in OFML always begin with the special character ”‘ `@` ”’, directly followed by a
character string, which passes the rules for identifiers. [3]


_symbol-constant:_
`@` _ident_


**3.2.6** **Operators**


The following tokens are handled by OFML as operators:


3By using the symbol(. . . ) constructor, it is possible to generate symbols from any character string; see Section
3.3.3.


24


_operator:_
`.` _|_ `(` _|_ `[` _|_ `++` _|_ `--` _|_ `!` _|_ `!!` _|_ `~` _|_ `$`

`*` _|_ `/` _|_ `%` _|_ `+` _|_ `-` _|_ `<<` _|_ `>>` _|_ `>>>` _|_ `<`
`<=` _|_ `>=` _|_ `>` _|_ `==` _|_ `!=` _|_ `~=` _|_ `<?` _|_ `>?` _|_ `&`
`^` _|_ `|` _|_ `&&` _|_ `||` _|_ `=>` _|_ `?` _|_ `:` _|_ `*=` _|_ `/=`
`%=` _|_ `+=` _|_ `-=` _|_ `&=` _|_ `^=` _|_ `|=` _|_ `<<=` _|_ `>>=` _|_ `>>>=`
`=` _|_ `,` _|_ `@(` _|_ `::` _|_ `instanceof`


**3.2.7** **Delimiters**


The following tokens in OFML represent delimiters:


_delimiter:_
`::` _|_ `{` _|_ `}` _|_ `;` _|_ `)` _|_ `]`

#### **3.3 Types**


OFML is a dynamically typified language, meaning that the type of a variable or expression
generally is not known until runtime.


Apart from the class definition, there are no special syntactical constructs for types OFML. Types
are objects and, as such, are also stored in variables like all other objects. Within the framework
of the operations defined for types, they can be handled like any other object. Mainly this means
that they can be assigned, passed to functions and called.


The two basic kinds of types in OFML are the simple types and the reference types. Simple types
are the numerical types, the symbol type and the `Void` type. The reference types are predefined
reference types or user-defined classes.


**3.3.1** **Objects and Variables**


An object is an instance of a class. It is generated by the calling of the corresponding class. Objects
are accessed via references.


A variable is a memory region where the value of a simple type or the reference to an object of a
reference type is stored.


There are two kinds of variables, named and unnamed. Named variables are all the variables that
can be specified by an identifier. Unnamed variables have to be accessed using an operator (such
as the `[]` index operator).


25


**3.3.2** **Operations for all Types**


All types inherit from the `Object` root type. This makes the following operations available to all
types:


_•_ The constructor. This is a function (see Section 3.6.3) that requires a type-specific number
of parameters and, for simple types, returns a new value of the type or, for reference types,
a reference to a newly generated object.


_•_ The assignment via the `=` operator (see Section 3.6). Here, the variables on the left side of
the assignment operator are assigned the value from the result of the expression on the right
side. If the result has a reference type, the reference is assigned, without a new instance of
the referenced object being created.


_•_ The passing as argument to a function. This takes place according to the rules of assignment
by the `=` operator, where the argument is assigned the corresponding, formal parameters of
the function.


_•_ The comparison using the `==` or `!=` operator. For simple types, the values themselves are
compared, while, if not otherwise defined, for reference types, object identity is verified.


_•_ The verification of the type using the `instanceof` operator.


**3.3.3** **Simple Types**


All simple types are defined in the `::cobra::lang` package.


**The** `Void` **Type**


The `Void` type is always used if a variable is to have a non-concrete value. The only possible value
for the `Void` type is `NULL` .


**Integers**


Integers are represented by the `Int` type and have a size determined by the machine [4] . The available
value range can be found using the static `Int::MIN_VALUE` member (the amount being the largest
representable negative value) and `Int::MAX_VALUE` (largest representable positive value).


The `Int()` constructor can be called either without arguments (in which case the value of 0 is
returned) or with an argument with one of the following types:


_•_ `Int` : The value of the argument is copied.


_•_ `Float` : A conversion from `Float` to `Int` is carried out and any fractional part is cut off. If
the available value range is exceeded, the result is undefined.


4With most currently distributed architectures, these are 32-bit numbers in with complement of two. The
available value range is [ _−_ 2147483648 _,_ 2147483647].


26


_•_ `Symbol` : A number that is unambiguously assigned to the symbol is returned.


_•_ `String` : An attempt is made to interpret the string as an integer constant. If, in doing so,
the rules specified in Section 3.2.5 are violated, an exception is triggered (see Section 3.5.3).


The following operators (see Section 3.6) can be applied to the `Int` type.


_•_ The arithmetical operators: the `+` and `-` operators in prefix and infix form, the `++` and `--`
operators in prefix and postfix form and the `*`, `/` and `%` infix operators.


_•_ The relational operators: `==`, `!=`, `<`, `>`, `<=`, `>=`, `<?` and `>?` .


_•_ The logical operators: `!` and `!!` .


_•_ The bitwise operators: `&`, `|`, `^`, `~`, `<<`, `>>` and `>>>` .


_•_ All combined assignments that can be formed using the above operators.


**Floating-point Numbers**


Floating-point numbers are represented by the `Float` type and have a size determined by the
machine [5] . The available value range can be found using the static `Float::MIN_VALUE` member
(the amount being the largest representable negative value) and `Float::MAX_VALUE` (largest representable positive value).


Depending on implementation, the static `Float::HUGE_VAL` member is either infinite positive or
the largest representable positive value. It is used by arithmetical operations on floating-point
values, sometimes with a minus sign, to signalize an overflow.


The `Float()` constructor can be called either without arguments (in which case the value of 0.0 is
returned) or with an argument with one of the following types:


_•_ `Float` : The value of the argument is copied.


_•_ `Int` : A conversion from `Int` to `Float` is carried out.


_•_ `String` : An attempt is made to interpret the string as a floating-point constant. If, in doing
so, the rules specified in Section 3.2.5 are violated, an exception is triggered (see Section
3.5.3).


The following operators (see Section 3.6) can be applied to the `Float` type.


_•_ The arithmetical operators: the `+` and `-` operators in prefix and infix form, the `++` and `--`
operators in prefix and postfix form and the `*`, `/` and `%` infix operators.


_•_ The relational operators: `==`, `!=`, `<`, `>`, `<=`, `>=`, `<?` and `>?` .


_•_ The logical operators: `!` and `!!` .


_•_ All combined assignments that can be formed using the above operators.


5With most currently distributed architectures, the amount of the smallest representable number is _±_ 2 _._ 2 _·_ 10 _−_ 308,
the largest is _±_ 1 _._ 8 _·_ 10 [308] and the accuracy is 15 decimal places.


27


**Arithmetic and Type Conversion**


Depending on the types of the operands, arithmetical calculations are carried out either in `Int`
or `Float` . `Float` is used if at least one of the operands is a `Float` type, except for combined
assignments, in which an `Int` type value is located on the left side.


Implicit type conversions for numerical types occur under the following conditions:


_•_ If one of the operands is of the `Int` type and the calculation takes place in `Float`, the operand
is converted to `Float` .


_•_ If one of the operands is of the `Float` type and the calculation takes place in `Int`, the operand
is converted to `Int` . Any fractional part is cut off. If an overflow occurs during the conversion,
the result is undefined.


The following rules apply to calculations in `Int` :


_•_ The complement of two is used for the internal representation of integer values.


_•_ The result is undefined if it is not representable in a value range of `Int` . Addition and
subtraction operations represent exceptions, for which the result comes from the lowest-value
bit of an integer value of sufficient size.


_•_ Division by `0` triggers an exception.


The following rules apply to calculations in `Float` :


_•_ If the exact result cannot be represented, either the next higher or next lower representable
value is applied, depending on implementation [6] .


_•_ The amount of the result is `Float::HUGE_VAL` if it is not representable in the value range of
`Float` due to an overflow. The +/- sign corresponds to the +/- sign of the correct value.


_•_ The result is `0` if it is no longer representable in the value range of `Float` due to an underflow.
Whether the +/- sign is preserved is dependent on implementation.


_•_ An exception is triggered if the operand is not within the range of definition of the operation.


**Symbols**


Symbols represent the dynamic counterpart to numeration constants in statically typified languages. Internally, they are represented by unique integers, which, using the `Int()` function, are
also available externally (see Section 3.3.3). With this representation, very fast comparison of
symbols is attainable (in contrast to string comparison).


6The direction of rounding can differ from operation to operation and is not dependent on the amount of the
difference to the next lower or next higher representable value.


28


In various instances of an OFML program, the conversion of the same string to a symbol can
lead to variously applied integers for the internal representation. Due to this, the outcome of
comparisons on symbols that are based on an order is not reproducible in various instances of an
OFML program.


The `Symbol()` constructor demands an argument with one of the following types:


_•_ `Symbol` : The value of the argument is copied.


_•_ `String` : The string (without the leading `@` ) is converted to a symbol. The `@foo` and
`Symbol("foo")` expressions are thereby equivalent. This method also makes it possible
to convert strings into symbols that do not meet the requirements for identifiers, such as
`Symbol("500 Motels")` .


The following operators (see Section 3.6) can be applied to the `Symbol` type.


_•_ The `==`, `!=`, `<`, `>`, `<=`, `>=`, `<?` and `>?` relational operators.


**3.3.4** **Reference Types**


**Automatic Garbage Collection**


The language definition of OFML requires the implementation of an automatic garbage collection.
Objects of reference types are generated implicitly when the constructor is called (see exception
below). There is no way of releasing objects explicitly. Instead, they can be released automatically
by the system as soon as no more references to the object exist. However, when and if objects that
are no longer referenced are released is not fixed [7] .


The language definition makes the manner of implementation of the automatic garbage collection
optional.


**Operators to Reference Types**


The behavior of operators, which can be used within expressions, is firmly defined for the simple
types. If the operand of a unary operator or the left operand of a binary operator delivers a
reference type, an instance-oriented method, specific to the operator, is called for the reference
type. These methods are freely definable for classes. Exceptions are the `$` (symbol resolution
operator), `!` (logical negation), `instanceof` (type verification), `>?` (maximum), `<?` (minimum),


7This differs greatly from the algorithm used for automatic garbage collection. When using reference counters,
objects are generally released as soon as there are no more references to them. However, a garbage collection based
only on reference counters does not release data structures with cycles. Due to this, the programmer has to break
such cycles before the last reference to such a data structure is released.


Other methods, for which, based on a known number of referenced objects, all reachable (thus referenced) objects
are determined, delay the release of objects that are no longer referenced and, under certain circumstances, do not
release all objects if conservative algorithms are used.


A combination of both methods is also conceivably.


29


`&&` (logical AND), `||` (logical OR), `?:` (conditional expression), `=` (assignment) and `,` (comma
operator) operators, whose behaviors either are firmly preset for reference types, are mapped to
other operators or fundamentally cannot be applied to reference types.


The operator methods to be used in class definitions are described in Section 3.6 below their
corresponding operators.


**Sequence Types**


Sequence types are all of the reference types that can be seen as sequences of objects. For this to
be the case, they have to meet the following conditions:


_•_ The `size()` method must be defined and return a nonnegative _size_ `Int` value.


_•_ The _operator[](pIdx(Int))_ and _operator[](pIdx(Int), pValue(Object))_ index operators must be
defined for each _pIdx_ integer index within the range of [0 _, size_ ).


_•_ The sequential access via the index operators, forward or backward, should require constant
time.


Of the predefined types in OFML, `String`, `Vector` and `List` are sequence types.

#### **3.4 Predefined Reference Types**


The following sections describe the predefined reference types in OFML; user-defined classes are
described in Section 3.8.


All predefined reference types are defined in the `::cobra::lang` package.


**3.4.1** **The** `Type` **Metatype**


All types, including the `Type` type, are instances of the `Type` type.


Type names OFML are variables having a reference to an instance of `Type` .


The following operators (see Section 3.6) and methods (see Section 3.8.4) can be applied to instances
of `Type` :


_operator()(parameters) →_ _Object_

The function call operator defined for all types is called a constructor. The constructor
generates an instance of the type for which it is called and calls the `initialize()` method
for the instance if it exists. The arguments passed to the constructor are forwarded to the
initialization method.


_getName() →_ _Symbol_

The _getName_ () method returns the simple name of the type as a `Symbol` .


30


_getFullName() →_ _String_

The _getFullName_ () method returns the fully qualified name of the type as a string.


_subClassOf(pType(Type)) →_ _Int_

The _subClassOf_ () method returns 1 if the type for which it was called is either identical to
the type passed as an argument or is derived from this type. Otherwise, the value returned
is 0. If the argument is not of the `Type` type, an exception is triggered.


**3.4.2** **Functions**


Functions are represents in OFML by the `Func` and `CFunc` types. `Func` is the type for functions
defined in OFML, while `CFunc` is the type for predefined functions. In addition to the operators
that are available to all types (see Section 3.3.2), `Func` and `CFunc` implement the ”‘ `()` ”’ function
call operator.


**3.4.3** **Character Strings**


Character strings are represented by the `String` type and are represented Internally by a sequence
of 8-bit values, where each value corresponds to one character. Whether the null character ( `’\0’` )
can be a component of a string depends on implementation.


The `String()` constructor can be called either without arguments (in which case the empty string,
`""`, is returned) or with an argument with one of the following types:


_•_ `String` : A copy of the string passed as an argument is created.


_•_ `Symbol` : A new string is generated, the content of which is equivalent to the string represented
by the symbol.


_•_ `Int`, `Float` : A new string is generated, which contains the result of the conversion of numbers
in a string.


A string constant in an expression causes an implicit call of the `String()` constructor, for which
the string is passed as an argument.


The following operators (see Section 3.6) and methods (see Section 3.8.4) can be applied to the
`String` type:


_operator==(pString(String)) →_ _Int_
_operator!=(pString(String)) →_ _Int_
_operator<(pString(String)) →_ _Int_
_operator<=(pString(String)) →_ _Int_
_operator>=(pString(String)) →_ _Int_
_operator>(pString(String)) →_ _Int_


The result is 1 if the character-to-character comparison of both strings turns out identical. Otherwise the result is 0. the character-to-character comparison of strings is described
on page 35 under the _compare_ () function.


31


_operator+(pString(String)) →_ _String_

The `+` addition operator anticipates a string on the right side. Otherwise, an exception is
triggered. It in turn creates a new string consisting of the linked strings on the left and right
sides of the operator.


_operator+=(pString(String)) →_ _String_

The `+=` addition operator anticipates a string on the right side. Otherwise, an exception is
triggered. It in turn appends the string on the right side of the operator onto the string on
the left side. The result is the combined string.


_operator[](pIdx(Int)) →_ _Int_
_operator[](pIdx(Int), pChar(Int))_


The index operators anticipate a value of the `Int` type as the _pIdx_ index. Otherwise, an
exception is triggered. Assume the length of the string is _len_ . If _pIdx <_ 0, _pIdx_ is set to
_pIdx_ + _len_ . If afterwards, _pIdx <_ 0 _∨_ _pIdx ≥_ _len_, an exception is triggered. Otherwise, the
character at the _pIdx_ position is returned as a positive `Int` .


If the index operator is used on the left side of the assignment operator (the second form of
the index operator), the expression on the right side of the assignment operator must return
a value of the `Int` type. Otherwise, an exception is triggered. This value, modulo 2 [8], is
assigned the to the string at the _pIdx_ position.


_operator[:](pBegin(Int), pEnd(Int)) →_ _String_
_operator[:](pBegin(Int), pEnd(Int), pChar(Int))_
_operator[:](pBegin(Int), pEnd(Int), pString(String))_


The `[:]` range operator anticipates both the _pBegin_ and _pEnd_ indexes of the `Int` type.
Assume the length of the string is _len_ . If _pBegin <_ 0, _pBegin_ is set to _pBegin_ + _len_ . Likewise, _pEnd_ is set to _pEnd_ + _len_ if _pEnd <_ 0. If, afterwards, _pBegin <_ 0 _∨_ _pBegin > len_ or
_pEnd <_ 0 _∨pEnd > len_ or _pBegin > pEnd_, an exception is triggered. Otherwise, a substring
is returned, starting with the _pBegin_ position and ending with the _pEnd −_ 1 position.


If the range operator is used on the left side of the assignment operator (the second and third
form of the range operator), the value of the expression on the right side of the assignment
operator must be either an `Int` value or a string of any length. Otherwise, an exception
is triggered. The substring specified by the _pBegin_ and _pEnd_ indexes is replaced by the
character, modulo 2 [8], specified by the integer value, or by the string.


_operator!!() →_ _Int_

The `!!` test operator returns 1 if the length of the string is not null. Otherwise, it returns 0.


_getAt(pIdx(Int)) →_ _Int_

If _pIdx <_ 0, _pIdx_ is set to _pIdx_ + _size_ (). If afterwards, _pIdx <_ 0 _∨_ _pIdx ≥_ _size_ (), an
exception is triggered. Otherwise, the character at the _pIdx_ position is returned as a positive
`Int` .


_setAt(pIdx(Int), pChar(Int))_

If _pIdx <_ 0, _pIdx_ is set to _pIdx_ + _size_ (). If, afterwards, _pIdx <_ 0 _∨_ _pIdx ≥_ _size_ () or
_pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, _pChar_ is assigned to the
character at the _pIdx_ position.


32


_size() →_ _Int_

returns the number of characters in the string.


_empty() →_ _Int_

returns 1 if the length of the string is null. Otherwise, it returns 0.


_resize(pSize(Int), pChar(Int) = ’ ’)_

If _pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, the new length of the
string is set to _pSize_ . If the new value is greater than the old length, the _pChar_ character is
used to fill it in.


_append(pString(String), pPos(Int) = 0, pLen(Int) = Int::MAX_ ~~_V_~~ _ALUE)_

If _pPos <_ 0 _∨_ _pPos > pString.size_ () or _pLen <_ 0, an exception is triggered. Otherwise,
_pLen_ = min( _pLen, pString.size_ () _−_ _pPos_ ). The substring of _pString_ with the _pLen_ length
is then, starting at the _pPos_ position appended to the string.


_append(pNum(Int), pChar(Int) = ’ ’)_

If _pNum <_ 0 or _pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, the _pChar_
character is appended to the string _pNum_ number of times.


_assign(pString(String), pPos(Int) = 0, pLen(Int) = Int::MAX_ ~~_V_~~ _ALUE)_

If _pPos <_ 0 _∨_ _pPos > pString.size_ () or _pLen <_ 0, an exception is triggered. Otherwise,
_pLen_ = min( _pLen, pString.size_ () _−pPos_ ). The string is then set to the substring of _pString_,
which begins at the _pPos_ position and has a length of _pLen_ .


_assign(pNum(Int), pChar(Int) = ’ ’)_

If _pNum <_ 0 or _pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, the string
is set to a sequence of _pLen_ times the _pChar_ character.


_insert(pPos1(Int), pString(String), pPos2(Int) = 0, pLen(Int) = Int::MAX_ ~~_V_~~ _ALUE)_

If _pPos_ 1 _<_ 0 _∨_ _pPos_ 1 _> size_ () or _pPos_ 2 _<_ 0 _∨_ _pPos_ 2 _> pString.size_ () or _pLen <_ 0, an
exception is triggered. Otherwise, _pLen_ is set to min( _pLen, pString.size_ () _−_ _pPos_ 2). Then,
the substring from _pString_, beginning at the _pPos_ 2 position and having a length of _pLen_,
is inserted at position _pPos_ 1.


_insert(pPos(Int), pNum(Int), pChar(Int) = ’ ’)_

If _pPos <_ 0 _∨_ _pPos > size_ () or _pNum <_ 0 or _pChar <_ 0 _∨_ _pChar >_ = 2 [8], an exception is
triggered. Otherwise, the _pChar_ character is inserted at the _pPos_ position _pNum_ number
of times.


_remove(pPos(Int) = 0, pLen(Int) = Int::MAX_ _VALUE)_

If _pPos <_ 0 _∨_ _pPos > size_ () or _pLen <_ 0, an exception is triggered. Otherwise, _pLen_ is set
to min( _pLen, size_ () _−_ _pPos_ ) Then, _pLen_ characters are removed starting at position _pPos_ .


_replace(pPos1(Int), pLen1(Int), pString(String), pPos2(Int) = 0, pLen2(Int) = Int::MAX_ _VALUE)_


If _pPos_ 1 _<_ 0 _∨_ _pPos_ 1 _> size_ () or _pPos_ 2 _<_ 0 _∨_ _pPos_ 2 _> pString.size_ () or _pLen_ 1 _<_ 0 or
_pLen_ 2 _<_ 0, an exception is triggered. Otherwise, _pLen_ 1 is set to min( _pLen_ 1 _, size_ () _−_ _pPos_ 1)
and _pLen_ 2 is set to min( _pLen_ 2 _, pString.size_ () _−_ _pPos_ 2). Then, _pLen_ 1 characters starting
at position _pPos_ 1 are replaced by a substring from _pString_, which begins at position _pPos_ 2
and is _pLen_ 2 characters long.


33


_replace(pPos(Int), pLen(Int), pNum(Int), pChar(Int) = ’ ’)_

If _pPos <_ 0 _∨_ _pPos > size_ () or _pLen <_ 0 or _pNum <_ 0 or _pChar <_ 0 _∨_ _pChar ≥_ 2 [8],
an exception is triggered. Otherwise, _pLen_ is set to min( _pLen, size_ () _−_ _pPos_ ) Then, _pLen_
characters starting at position _pPos_ are replaced by _pNum_ number of new _pChar_ characters.


_swap(pString(String))_

swaps the contents of two strings.


_find(pString(String), pPos(Int) = 0) →_ _Int_

If possible, the smallest _res_ value is returned for which these are valid:
_res ≥_ _pPos ∧_ _res_ + _pString.size_ () _≤_ _size_ () and
_getAt_ ( _res_ + _i_ ) = _pString.getAt_ ( _i_ ) for all _i ≥_ 0 _∧_ _i < pString.size_ ()
Otherwise, _−_ 1 is returned.


_find(pChar(Int), pPos(Int) = 0) →_ _Int_

If _pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, if possible, the smallest
_res_ value is returned for which these are valid:
_res ≥_ _pPos ∧_ _res < size_ () and _getAt_ ( _res_ ) = _pChar_
Otherwise, _−_ 1 is returned.


_rfind(pString(String), pPos(Int) = Int::MAX_ ~~_V_~~ _ALUE) →_ _Int_

If possible, the largest _res_ value is returned for which these are valid:
_res ≤_ _pPos ∧_ _res_ + _pString.size_ () _≤_ _size_ () and
_getAt_ ( _res_ + _i_ ) = _pString_ [ _i_ ] for all _i ≥_ 0 _∧_ _i < pString.size_ ()
Otherwise, _−_ 1 is returned.


_rfind(pChar(Int), pPos(Int) = Int::MAX_ ~~_V_~~ _ALUE) →_ _Int_

If _pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, if possible, the largest _res_
value is returned for which these are valid:
_res ≤_ _pPos ∧_ _res < size_ () and _getAt_ ( _res_ ) = _pChar_
Otherwise, _−_ 1 is returned.


_findFirstOf(pString(String), pPos(Int) = 0) →_ _Int_

If possible, the smallest _res_ value is returned for which these are valid:
_res ≥_ _pPos ∧_ _res < size_ () and
_getAt_ ( _res_ ) = _pString.getAt_ ( _i_ ) for at least one _i ≥_ 0 _∧_ _i < pString.size_ ()
Otherwise, _−_ 1 is returned.


_findFirstOf(pChar(Int), pPos(Int) = 0) →_ _Int_

If _pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, if possible, the smallest
_res_ value is returned for which these are valid:
_res ≥_ _pPos ∧_ _res < size_ () and _getAt_ ( _res_ ) = _pChar_ Otherwise, _−_ 1 is returned.


_findLastOf(pString(String), pPos(Int) = Int::MAX_ _VALUE) →_ _Int_

If possible, the largest _res_ value is returned for which these are valid:
_res ≤_ _pPos ∧_ _pPos < size_ () and
_getAt_ ( _res_ ) = _pString.getAt_ ( _i_ ) for at least one _i ≥_ 0 _∧_ _i < pString.size_ ()
Otherwise, _−_ 1 is returned.


_findLastOf(pChar(Int), pPos(Int) = Int::MAX_ ~~_V_~~ _ALUE) →_ _Int_

If _pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, if possible, the largest _res_


34


value is returned for which these are valid:
_res ≤_ _pPos ∧_ _pPos < size_ () and _getAt_ ( _res_ ) = _pChar_
Otherwise, _−_ 1 is returned.


_findFirstNotOf(pString(String), pPos(Int) = 0) →_ _Int_

If possible, the smallest _res_ value is returned for which these are valid:
_res ≥_ _pPos ∧_ _res < size_ () and
_getAt_ ( _res_ ) = _pString.getAt_ ( _i_ ) for no _i ≥_ 0 _∧_ _i < pString.size_ ()
Otherwise, _−_ 1 is returned.


_findFirstNotOf(pChar(Int), pPos(Int) = 0) →_ _Int_

If _pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, if possible, the smallest
_res_ value is returned for which these are valid:
_res ≥_ _pPos ∧_ _res < size_ () and _getAt_ ( _res_ ) _̸_ = _pChar_
Otherwise, _−_ 1 is returned.


_findLastNotOf(pString(String), pPos(Int) = Int::MAX_ _VALUE) →_ _Int_

If possible, the largest _res_ value is returned for which these are valid:
_res ≤_ _pPos ∧_ _pPos < size_ () and
_getAt_ ( _res_ ) = _pString.getAt_ ( _i_ ) for no _i ≥_ 0 _∧_ _i < pString.size_ ()
Otherwise, _−_ 1 is returned.


_findLastOf(pChar(Int), pPos(Int) = Int::MAX_ ~~_V_~~ _ALUE) →_ _Int_

If _pChar <_ 0 _∨_ _pChar ≥_ 2 [8], an exception is triggered. Otherwise, if possible, the largest _res_
value is returned for which these are valid:
_res ≤_ _pPos ∧_ _pPos < size_ () and _getAt_ ( _res_ ) _̸_ = _pChar_
Otherwise, _−_ 1 is returned.


_substr(pPos(Int) = 0, pLen(Int) = Int::MAX_ _VALUE) →_ _String_

If _pPos <_ 0 _∨_ _pPos > size_ () or _pLen <_ 0, an exception is triggered. Otherwise, _pLen_ is set
to min( _pLen, size_ () _−_ _pPos_ ) Then, a new string is created and returned whose contents are
equivalent to the substring beginning at _pPos_ and having a length of _pLen_ .


_toUpper(pPos(Int) = 0, pLen(Int) = Int::MAX_ ~~_V_~~ _ALUE)_

If _pPos <_ 0 _∨_ _pPos > size_ () or _pLen <_ 0, an exception is triggered. Otherwise, _pLen_ is set
to min( _pLen, size_ () _−_ _pPos_ ) Then, if _pLen >_ 0, all lowercase letters from position _pPos_ up
to and including position _pPos_ + _pLen −_ 1 are converted to uppercase letters.


_toLower(pPos(Int) = 0, pLen(Int) = Int::MAX_ ~~_V_~~ _ALUE)_

If _pPos <_ 0 _∨_ _pPos > size_ () or _pLen <_ 0, an exception is triggered. Otherwise, _pLen_ is set
to min( _pLen, size_ () _−_ _pPos_ ) Then, if _pLen >_ 0, all uppercase letters from position _pPos_ up
to and including position _pPos_ + _pLen −_ 1 are converted to lowercase letters.


_compare(pPos1(Int), pLen1(Int), pString(String), pPos2(Int) = 0, pLen2(Int) = Int::MAX_ _VALUE) →_ _Int_


runs a character-to-character comparison on the _pStr_ 1 = _substr_ ( _pPos_ 1 _, pLen_ 1) and _pStr_ 2 =
_pString.substr_ ( _pPos_ 2 _, pLen_ 2) strings. The result is _−_ 1 if _pStr_ 1 is smaller than _pStr_ 2. It is
+1 if _pStr_ 1 is larger than _pStr_ 2. And it is 0 if _pStr_ 1 and _pStr_ 2 are the same.


When two strings are compared character-to-character, the characters of both strings, starting at position 0, are compared to each other in pairs. The comparison is terminated as soon


35


as a pair of unidentical characters or the end of at least one string is reached. In the first
case, the result of the comparison is _−_ 1 if the code of the character in the first (or left) string
is less than the code of the character in the second (or right) string. Accordingly, the result
is +1 if the code of the character in the first string is greater than the code of the character
in the second string. In the second case, the result is 0 if the ends of both strings are reached
simultaneously. It is _−_ 1 if the end of the first sting was reached and +1 if the end of the
second string was reached.


_compare(pString(String), pPos(Int) = 0, pLen(Int) = Int::MAX_ ~~_V_~~ _ALUE) →_ _Int_

Corresponds to calling _compare_ (0 _,_ `Int::MAX_VALUE` _, pString, pPos, pLen_ ).


_getHashValue() →_ _Int_

The _getHashV alue_ () method returns a hash value for the string. Like the `==` operator, it
always returns the same hash value for identical strings, but can also return the same hash
value for unidentical strings.


**3.4.4** **Vectors**


The `Vector` type represents one-dimensional vectors. Multidimensional fields can be formed from
vectors of vectors, whereby the dimensions of the individual vectors do not have to be identical.


Random access to individual vector elements through their indexes requires constant time, as do
insert and delete operations at the ends of vectors. For insert and delete operations at the start or
in the middle of the vector, the required time is proportional to the number of subsequent vector
elements.


Insert operations might require additional time for reallocation of the vector.


Vectors can be created in two ways:


_•_ By calling the `Vector` constructor. _Vector(pSize(Int), ...)_ creates a vector with _pSize_ elements, which are initialized with `NULL` . The entry of a second _pSize_ 2 argument of the `Int`
type initializes the vector with vectors of size _pSize_ 2, thus creating a two-dimensional field.
This can be continued recursively by entering three and more arguments of the `Int` type to
create three and higher multidimensional fields.


_•_ By entering the elements in brackets, separated by commas. For every element there can be
any type of assignment expression, the result of which is to be used to initialize the element.


_special-ctor:_

`[` _arg-expr-listopt_ `]`
_arg-expr-list:_
_assign-expr_
_arg-expr-list_ `,` _assign-expr_


The following operators (see Section 3.6) and methods (see Section 3.8.4) can be applied to the
`Vector` type:


36


_operator==(pSeq(Object)) →_ _Int_
_operator!=(pSeq(Object)) →_ _Int_


The `==` and `!=` relational operators with a _vec_ vector on the left side anticipate a _pSeq_
instance of a sequence type (see Section 3.3.4) on the right side. The _vec_ vector and the _pSeq_
sequence are the same if:


_•_ The length of _vec_ is equal to the length of _pSeq_ .


_•_ For every _idx_ integer index in the range of [0 _, vec.size_ ()), the comparison of _vec_ [ _idx_ ]
to _pSeq_ [ _idx_ ] using the `==` _true_ operator yields ( _̸_ = 0). The first comparison of elements
that triggers an exception or does not yield _true_ terminates the comparison of the _vec_
vector to the _pSeq_ sequence.


_operator[](pIdx(Int)) →_ _Object_
_operator[](pIdx(Int), pObj(Object))_


The `[]` index operator anticipates a value of the `Int` type as index _pIdx_ . Assume the length of
the vector is _len_ . If _pIdx <_ 0, _pIdx_ is set to _pIdx_ + _len_ . If afterwards, _pIdx <_ 0 _∨pIdx ≥_ _len_,
an exception is triggered. Otherwise, the vector element indexed by _pIdx_ is returned.


If the index operator is used on the left side of the assignment operator (the second form of
the index operator), the result of the expression on the right side of the assignment operator
is assigned the vector element indexed by _pIdx_ .


_operator[:](pBegin(Int), pEnd(Int)) →_ _Vector_
_operator[:](pBegin(Int), pEnd(Int), pSeq(Object))_


The `[:]` range operator anticipates both the _pBegin_ and _pEnd_ indexes of the `Int` type.
Assume the length of the vector is _len_ . If _pBegin <_ 0, _pBegin_ is set to _pBegin_ + _len_ . Likewise, _pEnd_ is set to _pEnd_ + _len_ if _pEnd <_ 0. If, afterwards, _pBegin <_ 0 _∨_ _pBegin > len_ or
_pEnd <_ 0 _∨_ _pEnd > len_ or _pBegin > pEnd_, an exception is triggered. Otherwise, a vector
is returned that consists of the elements of the original vector that are indexed by _pBegin_
to _pEnd −_ 1.


If the range operator is used on the left side of the assignment operator (the second form of
the range operator), the result of the expression on the right side of the assignment operator
must be a sequence (see Section 3.3.4) of any length. The elements of the vector indexed by
_pBegin_ to _pEnd −_ 1 are replaced by all of the elements of the sequence.


_operator!!() →_ _Int_

The `!!` test operator returns 1 if the length of the vector is not null. Otherwise, it returns 0.


_getAt(pIdx(Int)) →_ _Object_

If _pIdx <_ 0, _pIdx_ is set to _pIdx_ + _size_ (). If afterwards, _pIdx <_ 0 _∨_ _pIdx ≥_ _size_ (), an
exception is triggered. Otherwise, the element with index _pIdx_ is returned.


_setAt(pIdx(Int), pObject(Object))_

If _pIdx <_ 0, _pIdx_ is set to _pIdx_ + _size_ (). If afterwards, _pIdx <_ 0 _∨_ _pIdx ≥_ _size_ (), an
exception is triggered. Otherwise, _pObject_ is assigned the element with the _pIdx_ index.


_size() →_ _Int_

The number of elements of the vector is returned.


37


_empty() →_ _Int_

If _size_ () = 0, 1 is returned;otherwise 0.


_front() →_ _Object_

If _size_ () = 0, an exception is triggered. Otherwise, the first element of the vector is returned.


_back() →_ _Object_

If _size_ () == 0, an exception is triggered. Otherwise, the last element of the vector is returned.


_pushBack(pObject(Object))_

As the last element, _pObject_ is appended to the vector.


_popBack() →_ _Object_

If _size_ () = 0, an exception is triggered. Otherwise, the last element of the vector is removed
from this and returned.


_insert(pPos(Int), pNum(Int) = 1, pObj(Object))_

If _pPos <_ 0 _∨_ _pPos > size_ () or _pNum <_ 0, an exception is triggered. Otherwise, _pObj_ is
inserted _pNum_ number of times at the _pPos_ index.


_erase(pBegin(Int), pEnd(Int) = pBegin + 1)_

If _pBegin <_ 0 _∨_ _pBegin > size_ () or _pEnd <_ 0 _∨_ _pEnd > size_ () or _pBegin > pEnd_, an
exception is triggered. Otherwise, if _pBegin < pEnd_, the elements indexed by _pBegin_ to
_pEnd −_ 1 are deleted from the vector.


_swap(pVec(Vector))_

If _pV ec_ is not an instance of the `Vector` type, an exception is triggered. Otherwise, the
contents of both vectors are swapped.


**3.4.5** **Lists**


The `List` type represents double-chained lists.


Sequential access to individual elements of a list, both forwards and backwards, as well as insert
and delete operations at any position, require constant time. The most required time for random
access to list elements is proportional to the minimum distance to the start or end of the list.


Lists can be created in two ways:


_•_ By calling the `List` constructor. This can be called with zero or more arguments. The
arguments form the individual elements of the list.


_•_ By entering the elements in `@()`, separated by commas. For every element there can be any
type of assignment expression, the result of which is to be used to initialize the element.


_special-ctor:_
`@(` _arg-expr-listopt_ `)`
_arg-expr-list:_
_assign-expr_
_arg-expr-list_ `,` _assign-expr_


38


The following operators (see Section 3.6) and methods (see Section 3.8.4) can be applied to the
`List` type:


_operator==(pSeq(Object)) →_ _Int_
_operator!=(pSeq(Object)) →_ _Int_


The `==` and `!=` relational operators with a _list_ list on the left side anticipate a _pSeq_ instance of a sequence type (see Section 3.3.4) on the right side. The _list_ list and the _pSeq_
sequence are the same if:


_•_ The length of _list_ is equal to the length of _pSeq_ .


_•_ For every _idx_ integer index in the range of [0 _, list.size_ ()), the comparison of _list_ [ _idx_ ]
to _pSeq_ [ _idx_ ] using the `==` _true_ operator yields ( _̸_ = 0). The first comparison of elements
that triggers an exception or does not yield _true_ terminates the comparison of the _list_
list to the _pSeq_ sequence.


_operator[](pIdx(Int)) →_ _Object_
_operator[](pIdx(Int), pObject(Object))_


The `[]` index operator anticipates a value of the `Int` type as index _pIdx_ . Assume the length
of the list is _len_ . If _pIdx <_ 0, _pIdx_ is set to _pIdx_ + _len_ . If afterwards, _pIdx <_ 0 _∨_ _pIdx ≥_ _len_,
an exception is triggered. Otherwise, the list element indexed by _pIdx_ is returned.


If the index operator is used on the left side of the assignment operator (the second form of
the index operator), the result of the expression on the right side of the assignment operator
is assigned the list element indexed by _pIdx_ .


_operator[:](pBegin(Int), pEnd(Int)) →_ _List_
_operator[:](pBegin(Int), pEnd(Int), pSeq(Object))_


The `[:]` range operator anticipates both the _pBegin_ and _pEnd_ indexes of the `Int` type.
Assume the length of the list is _len_ . If _pBegin <_ 0, _pBegin_ is set to _pBegin_ + _len_ . Likewise,
_pEnd_ is set to _pEnd_ + _len_ if _pEnd <_ 0. If, afterwards, _pBegin <_ 0 _∨_ _pBegin > len_ or
_pEnd <_ 0 _∨_ _pEnd > len_ or _pBegin > pEnd_, an exception is triggered. Otherwise, a list
is returned that consists of the elements of the original list that are indexed by _pBegin_ to
_pEnd −_ 1.


If the range operator is used on the left side of the assignment operator (the second form of
the range operator), the result of the expression on the right side of the assignment operator
must be a sequence (see Section 3.3.4) of any length. The elements of the list indexed by
_pBegin_ to _pEnd −_ 1 are replaced by all of the elements of the sequence.


_operator!!() →_ _Int_

The `!!` test operator returns 1 if the length of the list is not null. Otherwise, it returns 0.


_getAt(pIdx(Int)) →_ _Object_

If _pIdx <_ 0, _pIdx_ is set to _pIdx_ + _size_ (). If afterwards, _pIdx <_ 0 _∨_ _pIdx ≥_ _size_ (), an
exception is triggered. Otherwise, the element with index _pIdx_ is returned.


_setAt(pIdx(Int), pObject(Object))_

If _pIdx <_ 0, _pIdx_ is set to _pIdx_ + _size_ (). If afterwards, _pIdx <_ 0 _∨_ _pIdx ≥_ _size_ (), an
exception is triggered. Otherwise, _pObject_ is assigned the element with the _pIdx_ index.


39


_size() →_ _Int_

The number of elements of the list is returned.


_empty() →_ _Int_

If _size_ () = 0, 1 is returned;otherwise 0.


_front() →_ _Object_

If _size_ () = 0, an exception is triggered. Otherwise, the first element of the list is returned.


_back() →_ _Object_

If _size_ () = 0, an exception is triggered. Otherwise, the last element of the list is returned.


_pushFront(pObject(Object))_

As the first element, _pObject_ is appended to the list.


_pushBack(pObject(Object))_

As the last element, _pObject_ is appended to the list.


_popFront() →_ _Object_

If _size_ () = 0, an exception is triggered. Otherwise, the first element of the list is removed
from this and returned.


_popBack() →_ _Object_

If _size_ () = 0, an exception is triggered. Otherwise, the last element of the list is removed
from this and returned.


_insert(pPos(Int), pNum(Int) = 1, pObj(Object))_

If _pPos <_ 0 _∨_ _pPos > size_ () or _pNum <_ 0, an exception is triggered. Otherwise, _pObj_ is
inserted _pNum_ number of times at the _pPos_ index.


_erase(pBegin(Int), pEnd(Int) = pBegin + 1)_

If _pBegin <_ 0 _∨_ _pBegin > size_ () or _pEnd <_ 0 _∨_ _pEnd > size_ () or _pBegin > pEnd_, an
exception is triggered. Otherwise, if _pBegin < pEnd_, the elements indexed by _pBegin_ to
_pEnd −_ 1 are deleted from the list.


_swap(pList(List))_

If _pList_ is not an instance of the `List` type, an exception is triggered. Otherwise, the contents
of both lists are swapped.


_splice(pPos(Int), pList(List), pBegin(Int) = 0, pEnd(Int) = pBegin + 1)_

If _pPos <_ 0 _∨_ _pPos > size_ () or _pBegin <_ 0 _∨_ _pBegin > pList.size_ () or _pEnd <_ 0 _∨_ _pEnd >_
_pList.size_ () or _pBegin > pEnd_, an exception is triggered. Otherwise, if _pBegin < pEnd_,
the elements from _pList_ that are indexed by _pBegin_ to _pEnd −_ 1 are removed from _pList_
and inserted in the same order starting at _pPos_ .


_remove(pObj(Object))_

Compares each element of the list, starting with the first and ascending sequentially until
the last, using the `==` operator with _pObj_, whereby the list element appears on the left and
_pObj_ on the right of the relational operator. If an exception is triggered by the comparison,
the function turns back immediately. Otherwise, if the comparison resulted in _true_ ( _̸_ = 0), it
removes the current list element from the list.


40


_removeIf(pPred(Func))_

The _pPred_ argument has to be a function that expects an argument and returns either _true_
( _̸_ = 0) or _false_ (0) (unary predicate).


The _removeIf_ () method calls the _pPred_ function for every element of the list, starting with
the first and ascending sequentially until the last, whereby the list element is passed as an
argument to the function. If an exception is triggered by the function, _removeIf()_ turns back
immediately. If the return value of the function is not an `Int` type, an exception is triggered.
Otherwise, if the return value of the function is _true_ ( _̸_ = 0), it removes the current list element
from the list.


_unique()_

Removes from each sequence all identical, consecutive elements except the first one. To do
so, it compares each current element to its directly successive element using the `==` operator,
whereby the current element appears on the left and the successive element on the right of the
relational operator. If an exception is triggered by the relational operator, the function turns
back immediately. Otherwise, the successive element is deleted if the comparison results in
_true_, or the successive element is made the current element if the comparison results in _false_ .


_unique(pPred(Func))_

The _pPred_ argument has to be a function that expects two arguments and returns _true_ ( _̸_ = 0)
if both arguments are the same, or _false_ (0) if they are different (binary predicate). If the
return value is not an `Int` type, _unique_ () triggers an exception.


The _unique_ () method with _pPred_ as an argument behaves exactly as it does without an
argument except that, instead of the `==` operator, the _pPred_ function is called, to which the
current element is passed as the first argument and the successive element as the second.


_merge(pList(List))_

The _merge_ () method merges two lists, sorted in ascending order, into a single sorted list. It
uses the `<` operator, to the left side of which is passed an element from _pList_ and to the right
an element from _self_ . If an exception is triggered by the relational operator, _merge_ () turns
back immediately and the content of each list is undefined. The _pList_ argument list is empty
after _merge_ () comes back. If elements are equivalent in both lists, the elements from _self_
are placed before those from _pList_ in the result list. The order of elements in a list remains
unchanged in the result list.


_merge(pList(List), pPred(Func))_

The second _pPred_ argument has to be a function that expects two arguments and returns
_true_ ( _̸_ = 0) if the first argument is smaller than the second argument or _false_ (0) otherwise
(binary predicate). If the return value is not an `Int` type, _merge_ () triggers an exception.


The _merge_ () method with two arguments behaves exactly as it does with only one argument
except that, instead of the `<` operator, the _pPred_ function is called.


_sort()_

Sorts the list using the relational `<` operator, which can be called for any of the elements in
a list. If an exception is triggered by _sort_ () either directly or indirectly, the content of the
list is undefined. The order of same elements in the unsorted list remains intact in the sorted
list. The complexity of _sort_ () is approximately _size_ () _·_ log( _size_ ()) relational operations.


41


_sort(pPred(Func))_

The _pPred_ argument has to be a function that expects two arguments and returns _true_
( _̸_ = 0) if the first argument is smaller than the second argument or _false_ (0) otherwise (binary
predicate). If the return value is not an `Int` type, _sort_ () triggers an exception.


The _sort_ () method with one argument behaves exactly as it does without an argument except
that, instead of the `<` relational operator, the _pPred_ function is called.


_reverse()_

The _reverse_ () method reverses the order of the elements in the list.


**3.4.6** **Hash Tables**


The `Hash` type makes hash tables available. A hash table contains a set of entries in pairs. Each
entry consists of a key and a value. The key is used to access the value for read or write operations.


Values of the simple types, `Int`, `Float` and `Symbol`, as well as all reference types that define the
instance-oriented `getHashValue()` method, can be used as keys. The `getHashValue()` method
must return a value of the `Int` type, which is the same for two keys for which the equality operator,
`==`, when applied to them, yields _true_ .


Keys of different types can be used in a hash table. Two keys are considered the same if their
types are identical and the equality operator, `==`, when applied to both keys, yields _true_ .


The _Hash()_ constructor creates an empty hash table. The initial size of the hash table is dependent
on implementation. It grows with the amount of values stored in the hash table, whereby the time
for hash table enlargement is distributed to consecutive read or write accesses, while the additional
time taken for an access is on average independent of the size of the hash table.


The following operators (see Section 3.6) and methods (see Section 3.8.4) can be applied the `Hash`
type:


_operator[](pKey(Object)) →_ _Object_
_operator[](pKey(Object), pValue(Object))_


The index operator anticipates a key as an index value that meets the key requirements
listed above. If the hash table contains an entry with this key, the value stored for this entry
is returned. Otherwise, an exception is triggered.


If the index operator is used on the left side of the assignment operator, the result of the
expression on the right side of the assignment operator is stored as a value under the specified
key in the hash table. If no entry yet exists for this key, a new entry is created.


_operator!!() →_ _Int_

The test operator returns 1 if the hash table contains at least one entry. Otherwise, it returns
0.


_getAt(pKey(Object)) →_ _Object_

The _getAt_ () method anticipates as an argument a key that meets the above-mentioned requirements. If no entry exists for this key, an exception is triggered. Otherwise, the value of
the entry is returned.


42


_setAt(pKey(Object), pValue(Object))_

The _setAt_ () method anticipates as the first argument a key that meets the above-mentioned
requirements and, as the second, an object of any type. If no entry exists for this key, a new
one is created. Then, the value of the second argument is stored in this entry as a value.


_size() →_ _Int_

The number of entries in the hash table is returned.


_empty() →_ _Int_

If _size_ () =, 1 is returned, otherwise 0.


_hasKey(pKey(Object)) →_ _Int_

The _hasKey_ () method anticipates as the argument a key that meets the above-mentioned
requirements. It returns 1 if an entry with this key exists in the hash table; otherwise it
returns 0.


_keys() →_ _Vector_

The _keys_ () method returns a `Vector` whose individual elements are the keys of all entries in
the hash table.


_values() →_ _Vector_

The _values_ () method returns a `Vector` whose individual elements are the values of all entries
in the hash table.


_swap(pHash(Hash))_

If the argument is not of the `Hash` reference type, an exception is triggered. Otherwise, the
entries of both hash tables are swapped.


_remove(pKey(Object))_

The argument must be a key that meets the above-mentioned requirements. If no entry exists
for this key, an exception is triggered. Otherwise, the corresponding entry is deleted.


The identical order of the keys and values returned by the `keys()` and `values()` methods can only
be guaranteed if no other methods of the hash table, including index operators, are called between
the execution of the two methods.

#### **3.5 Statements**


The translation unit ( _translation-unit_ ) shapes the entry symbol of the OFML grammar (see Section3.1.3). Every translation unit consists of an optional package statement, a (potentially empty)
sequence of import statements ( _import-stmts_ ) and a (potentially empty) sequence of other statements ( _stmt-list_ ). The syntax and semantics of package and import statements are described in
Section 3.7.


_translation-unit:_
_package-stmtopt import-stmtsopt stmt-listopt_
_import-stmts:_
_import-stmtsopt import-stmt_
_stmt-list:_
_stmt-listopt stmt_


43


An OFML statement can contain the following: a definition ( _definition-stmt_ ), an expression ( _expr-_
_stmt_ ), a control statement ( _ctrl-stmt_ ) or a compound statement ( _compound-stmt_ ).


_stmt:_
_definition-stmt_
_expr-stmt_
_ctrl-stmt_
_compound-stmt_


Definitions are handled by the compiler. All other statements are executed in the order in which
they appear textually at runtime.


In some cases, either a semicolon or the end of file is expected at the end of a statement [8] .


_eox:_
`;` _|_ EOF


**3.5.1** **Definitions**


The following elements can be introduced by definitions: variables ( _var-def_ ), named functions
( _named-func-def_ ), classes ( _class-def_ ), the name of the package to which the translation unit belongs
( _package-stmt_ ) and the packages imported by the translation unit ( _import-stmt_ ). Package and
import statements are described in Section 3.7, class definition in Section 3.8.


_definition-stmt:_
_var-def_
_named-func-def_
_class-def_


**Variable Definitions**


A variable definition starts with an optional sequence of modifiers and the `var` keyword, followed
by one or more initialization expressions ( _init-expr_ ) separated by commas. The last expression
is ended with a semicolon or the end of file ( _eox_ ). Every initialization expression consists of an
identifier ( _ident_ ), optionally followed by an assignment operator and an expression ( _expr_ ) evaluated
in the value context (see Section 3.6.1). The latter is used to set the initial valued of variables. If
neither the assignment operator nor expression ( _expr_, see Section3.6) are present, the variable is
given the `NULL` value. The identifier becomes valid immediately after the initialization expression
contained within it.


8The end of file is allowed to terminate a statement so that the semicolon can be dropped in interactive mode.


44


_var-def:_
_global-modifiersopt_ `var` _init-expr-list eox_
_init-expr-list:_
_init-expr_
_init-expr-list_ `,` _init-expr_
_init-expr:_
_ident_
_ident_ `=` _expr_


Modifiers are described in Sections 3.7.6 and 3.8.


**Named Function Definitions**


The definition of a named function begins with an optional sequence of modifiers and the `func`
keyword, followed by the name of the function that, as such, becomes valid in the current namespace (see Section3.7). A pair of parentheses follows, which encloses any existing parameters and
compound statement, which represents the function body.


_named-func-def:_
_global-modifiersopt_ `func` _ident_ `(` _param-listopt_ `)` _compound-stmt_
_global-modifiersopt_ `func` _ident_ `(` _param-list_ `, ...` `)` _compound-stmt_
`native` _global-modifiersopt_ `func` _ident_ `( ) ;`
_param-list:_
_ident_
_param-list_ `,` _ident_


Modifiers are described in Sections 3.7.6 and 3.8.


The second form of the function definition, for which an ellipse ( `...` ), separated by it with a comma,
follows a non-empty parameter list, defines a function with a variable number of arguments. If
the function defined in this manner has _n_ parameters in its parameter list, it is to be called with
at least _n −_ 1 arguments. A vector, which receives all further arguments, is created for the _n_ th
parameter.


The third form of the function definition, which is introduced by the `native` keyword, does not
contain any parameter declarations [9] or any function body. Instead, its definition is closed with a
semicolon.


A function that is defined as `native` is implemented in platform-dependent code. This is usually
another programming language, such as C, C++ or Assembler.


9This does not mean that no arguments can be passed to a function defined as `native` . The parameters are not
declared, since it is the task of the platform-dependent code to verify the number of arguments (and their types).


45


**3.5.2** **Expressions as Statements**


Most statements in OFML consist of an expression ( _expr_, which is evaluated in secondary context
(see Section 3.6.1) and is closed with a semicolon or end of file.


_expr-stmt:_
_expropt eox_


If the expression is not present, the statement is an empty statement, which can be used in
situations where the syntax requires a statement but no action is desired (for example in the body
of an empty loop).


**3.5.3** **Control Statements**


Control statements are used to control the course of a program dynamically and are divided
roughly into three categories: selection statements ( _select-stmt_ ), loop statements ( _loop-stmt_ ), jump
statements ( _jump-stmt_ ) and exception statements _exception-stmt_ ). The latter are described in
Section 3.5.3.


_ctrl-stmt:_
_select-stmt_
_loop-stmt_
_jump-stmt_
_exception-stmt_


**Selection Statements**


Selection statements select one or more program sequences.


_select-stmt:_
`if (` _expr_ `)` _stmt_ 1
`if (` _expr_ `)` _stmt_ 1 `else` _stmt_ 2
_labelopt_ `switch (` _expr_ `) {` _switch-stmt-list_ `}`


For both forms of the `if` statement, the _expr_ expression is assessed in test context (see Section 3.6.1). If the expression yields _true_, the _stmt_ 1 statement is executed. In the second form,
_stmt_ 2 is executed if the expression yields _false_ . The syntactical ambiguity for `else` is resolved by
always assigning an `else` to the last occurring `if` without `else` on the same block nesting level.


The _stmt_ 1 and _stmt_ 2 statements of the `if` statement cannot be definitions ( _definition-stmt_ ).


The `switch` statement evaluates the `switch` expression _expr_ in value context and branches, depending on the result, to a label ( _switch-label_ ) within the subsequent statement list ( `switch-stmt-list` ),


46


which is enclosed in curly brackets. Optionally, it can include a label to which the `break` and
`continue` statements can refer within the `switch` statement list (see Section 3.5.3).


_switch-stmt-list:_
_switch-stmt-listopt switch-stmt_
_switch-stmt:_
_expr-stmt_
_ctrl-stmt_
_compound-stmt_
_switch-label_
_switch-label:_
`case` _expr_ `:`
```
     default :

```

Here, the expressions ( _expr_ ) of the `case` labels are evaluated in the order in which they occur and
compared for equality to the result of the `switch` expression, whereby the result of the `switch`
expression appears on the left of the relational operator. If the comparison yields _true_, processing continues with the statement ( _switch-stmt_ ) that directly follows the `case` label. Otherwise,
processing continues at the next `case` label.


If all `case` labels have been processed without the occurrence of equality, processing continues with
the statement following a `default` label if one is present. If none is present, no statement in the
statement list is processed.


No more than one `default` label may occur within the statement list of a `switch` statement.


Exceptions that have been triggered by the `switch` expression, the `case` expressions or the `==`
relational operator, which is applied to the results of both expressions, are not caught.


**Loop Statements**


Loop statements are used to repeat the execution of statements. Optionally, a loop statement can
include a label to which, within the body of the loop, the `break` and `continue` statements can refer
(see Section 3.5.3).


The _stmt_ statement that forms the body of the loop cannot be a definition ( _definition-stmt_ ).


_labeled-loop-stmt:_
_labelopt loop-stmt_
_label:_
_ident_ `:`
_loop-stmt:_
`while (` _expr_ `)` _stmt_
`do` _stmt_ `while (` _expr_ `)`
`for (` _expr_ 1 _opt; expr_ 2 _opt; expr_ 3 _opt_ `)` _stmt_
`foreach (` _name_ `;` _expr_ `)` _stmt_


47


The _expr_ expressions of the `while` or `do` - `while` statements and the second _expr_ 2 expression of the
`for` statement are evaluated in test context (see Section 3.6.1).


Using the `while` statement, the _stmt_ statement is repeated until the _expr_ expression yields _false_ .
The evaluation of the expression takes place **before** the first execution of the statement.


The `do` - `while` statement is similar to the `while` statement, except that the expression is evaluated
**after** the execution of the _stmt_ statement. In this case, the statement is executed at least once no
matter what.


With the `for` statement, the first expression ( _expr_ 1) is evaluated first in secondary context. It
is used (in general) to initialize the loop. The second expression ( _expr_ 2) is evaluated before each
processing of the loop body. If it yields _false_, the `for` loop is terminated. Otherwise, the body of
the loop is processed and then the third expression ( _expr_ 3), which (in general) is used to reinitialize
the loop, is processed in secondary context.


All three expressions of the `for` statement may be omitted. If the second expression ( _expr_ 2) is not
present, this is equivalent to the test result of _true_ .


If the statement does not contain a `continue` statement, the `for` statement is equivalent to:


_expr_ 1 _;_
`while (` _expr_ 2 `) {`
_stmt_
_expr_ 3 _;_
```
  }

```

The `foreach` statement is used for iterations through a sequence. In this case, the first expression
must be a (if necessary, qualified) name. The result of the second expression processed in value
context must meet the requirements for a sequence type (see Section 3.3.4). Otherwise, an exception
is triggered (potentially after one or more iterations).


The implementation must behave as if creating a temporary _idx_ variable that is assigned the `Int`
value of _−_ 1 before the loop is processed and is increased by 1 before each pass of the loop. The
second expression is evaluated once prior to processing the loop and its result is stored in the
temporary _seq_ variable. The loop is terminated if _idx_, after being increased by 1, is greater than
or equal to the current length of the sequence stored in _seq_ . Otherwise, the element of the sequence
indexed by _idx_ is assigned the value determined by the first expression. Then the body of the loop
is completely processed.


If the `foreach` statement does not contain a `continue` statement, it is equivalent to [10] :


`seq =` _expr_ `;`
```
  for (idx = 0; idx < seq.size(); idx++) {
```

_name_ `= seq[idx];`
_stmt_
```
  }

```

10The identifiers are selected only for demonstration; in principle, OFML generates internal variables that cannot
come into conflict with user-defined variables.


48


**Jump Statements**


Jump statements unconditionally continue processing of the program at another position.


_jump-stmt:_
_continue-stmt_
_break-stmt_
_return-stmt_


The `continue` statement may occur only within a `while`, `do` - `while`, `for` or `switch` statement.
For `while` and `do` - `while` loops, it continues program processing with the evaluation of the test
expression, for the `for` loop, with the evaluation of the reinitialization expression, and for the
`switch` statement, by restarting the entire `switch` statement.


_continue-stmt:_
```
     continue ;
```

`continue` _ident_ `;`


A `continue` statement without identifier passes control to the innermost of the statements listed
above. If such a statement is does not exist, a translation error occurs.


A `continue` statement with an _ident_ identifier passes control to the innermost of the statements
listed above that has the same identifier as a label. If such a statement is does not exist, a
translation error occurs.


The `break` statement may occur only within a `while`, `do` - `while`, `for` or `switch` statement.


_break-stmt:_
```
     break ;
```

`break` _ident_ `;`


A `break` statement without identifier continues program processing directly after the innermost of
the statements listed above. If such a statement is does not exist, a translation error occurs.


A `break` statement with an _ident_ identifier continues program processing directly after the innermost of the statements listed above that has the same identifier as a label. If such a statement is
does not exist, a translation error occurs.


The identifiers specified for `continue` and `break` statements and used as labels before `while`, `do` `while`, `for` and `switch` statements are located in a separate namespace, in which they can be
applied with any frequency.


The `return` statement ends the execution of a function. If the statement contains an expression
(optional), it is processed in value context (see Section 3.6.1) and its value is returned as a return
value of the function. If there is no expression or if the end of the function is reached without an
occurrence of the `return` statement, the function returns the `NULL` value.


49


A `return` statement outside of a function causes a translation error.


_return-stmt:_
```
     return ;
```

`return` _expr_ `;`


**Exceptions**


Exceptions can be triggered either by internal errors (such as errors while loading translation units
or division by zero) or by the explicit execution by the programmer of the `throw` statement. They
cause a nonlocal [11] pass of the program process from the position where the exception was triggered
to the position at which it is caught. The latter is determined during program runtime.


_exception-stmt:_
_try-stmt_
_throw-stmt_


The `try` statement allows exceptions to be handled in a user-defined manner. Using several optional
`catch` components, it is possible to handle various types of exceptions separately. Here _name_ is the
name of a type and _ident_ is the name of a local variable that contains the value of the exception
and that is valid only within the `catch` block.


_try-stmt:_
`try` _compound-stmt catch-stmtsopt_
_catch-stmts:_
_catch-stmt catch-stmtsopt_
_catch-stmt:_
`catch (` _name ident_ `)` _compound-stmt_


Only reference types are permitted as type names in the `catch` statement. Other types lead to a
translation error. Using a `catch` statement, all of the exceptions that are instances of the class
specified by the type names or one of the classes derived from this are caught.


If a `try` statement has several `catch` statements, the body of the first matching `catch` statement
is executed even if a subsequent `catch` statement of the same `try` statement would yield a more
exact match between the type of the `catch` parameter and the class of the exception.


A `try` statement without `catch` statement catches all exceptions. If no match between at least on
type of the `catch` parameter and the class of the exception can be found in a `try` statement with
at least one `catch` statement, the exception is not caught by this `try` statement.


The `throw` statement allows the programmer to trigger exceptions. Here, the value of the _expr_
expression processed in value context (see Section 3.6.1) is passed as the value of the exception.


11Nonlocal pass means that the catching `try` statement can be located in a direct or indirect caller of the exceptiontriggering function.


50


_throw-stmt:_
`throw` _expr eox_


The result of the expression of a `throw` statement must have a reference type. Otherwise, another
exception is triggered. The `throw` statement passes the program processing on to the `try` statement
that dynamically encloses it and that either contains one matching `catch` statement or none. If a
`try` statement of this sort is not present, the exception is, dependent on implementation, handled by
the runtime system, for example by outputting an error message and possibly terminated program
execution.


**Compound Statements**


Compound statements are used to insert sequences of several statements at positions where, syntactically, only one statement is permitted, such as in the body of a loop.


_compound-stmt:_
`{` _stmt-list_ `}`


Compound statements make a new namespace available where variables defined within the compound statement can be entered. When binding an identifier to a variable, a search is carried out
from inside to outside, one after the other, in the namespaces of the statically enclosing compound
statements.


Variables with identical identifier cannot be defined more than once in a compound statement.
Compound statements cannot contain any function or class definitions.

#### **3.6 Expressions**


The following section describes the operators of OFML, sorted by precedence. Precedence, associativity and evaluation order of operands are fixed conditions. Unless otherwise stated, operands
are evaluated from left to right, while the evaluation of one operand with all side-effects must
be completed before the evaluation of the next can be begun. This also applies to arguments of
functions and methods. With few exceptions, which are explicitly mentioned, all operands of an
operator are evaluated always.


The behavior of unary operators is oriented to the type of the result of the operand. For binary
operators, it is oriented to the type of the result of the left operand. If the corresponding result has
a predefined reference type, the exact behavior of each operator, if defined, is described in Section
3.4.


**3.6.1** **Value, Test and Secondary Context**


Expressions and subexpressions are processed in three different contexts:


51


**Wert–Kontext** The expression must supply a value of one of the simple types or a reference
type. If the result of the expression is a logical value, it is converted to the `Int` value of 1 if
it is _true_ and to the `Int` value of 0 if it is _false_ .


**Test–Kontext** The expression must deliver a logical value, i.e. either _true_ or _false_ . If the result
of the expression is a value of a reference type, the `operator!!()` operator function is called
up for it and then takes into account its return value. If the result now is not an `Int` or
`Float`, an exception is triggered. Otherwise, the result of the expression becomes _true_, if the
`Int` or `Float` does not equal null and, otherwise, _false_ .


**Nebenwirkungs–Kontext** The expression is evaluated to achieve a side effect. The result of the
expression that presents both a logical value and a value of a simple type or a reference type
is ignored.


If not otherwise stated, operators process their operands in value context.


**3.6.2** **Primary Expressions**


Primary expressions are identifiers (see Sections3.7.2 and 3.7.5), literal constants (see Section3.2.5),
special constructors for vectors (see Section3.4.4) and lists (see Section3.4.5) or bracketed expressions:


_primary-expr:_
_name_
_constant_
_special-ctor_
`(` _expr_ `)`
_special-ctor:_

`[` _arg-expr-listopt_ `]`
`@(` _arg-expr-listopt_ `)`
_arg-expr-list:_
_assign-expr_
_arg-expr-list_ `,` _assign-expr_


names ( _name_ ) are described in Section 3.7.2, constants ( _constant_ ) in Section 3.2.5 and special
constructors ( _special-ctor_ ) in Sections 3.4.4 and 3.4.5. The value of a bracketed expression is equal
to the value of the _expr_ expression within the brackets.


**3.6.3** **Postfix Expressions**


Postfix expressions are left-associative.


52


_postfix-expr:_
_primary-expr_
_postfix-expr_ `[` _expr_ `]`
_postfix-expr_ `[` _expr_ 1 _opt_ `:` _expr_ 2 _opt_ `]`
_postfix-expr_ `(` _arg-expr-listopt_ `)`
_postfix-expr_ `.` _ident_
_postfix-expr_ `++`
_postfix-expr_ `--`


The operator for accessing ”‘ `.` ”’ attributes is described in Section 3.8.


**Index Expressions**


The _postfix-expr_ in the index expression, _postfix-expr_ `[` _expr_ `]`, must deliver a reference type.
Otherwise, an exception is triggered.


For reference types, two operator methods can be defined, which are used to request and set an
object in a sequence based on an index:


`operator[](` _idx_ `)` is called for the result of the _postfix-expr_ if the indexed value is to be read. The
result of _expr_ is passed to the _idx_ parameter. The return value of the operator method is the value
of the index expression.


`operator[](` _idx_ `,` _value_ `)` is called for the result of the _postfix-expr_ if the indexed value is to be
written [12] . The result of _expr_ is then passed to the _idx_ parameter and the value to be written is
passed to the _value_ parameter. Any return value is ignored.


**Range Expressions**


The _postfix-expr_ in the range expression, _postfix-expr_ `[` _expr_ 1 _opt_ `:` _expr_ 2 _opt_ `]`, must deliver a reference type. Otherwise, an exception is triggered.


If _expr_ 1 is been specified, the `Int` value of 0 is passed to the range operator as the start of the
range. Similarly, if _expr_ 2 is not specified, the return value of the _size()_ method, applied to the
result of the _postfix-expr_, is passed to the range operator as the end of the range. An exception is
triggered if the _size()_ method does not exist.


For reference types, two operator methods can be defined, which are used to request and set a
range of a sequence based on a start and end index:


`operator[:](` _beginend_ `)` is called for the result of the _postfix-expr_ if the range is to be read. The
result of _expr_ 1 is passed to the _begin_ parameter and the result of _expr_ 2 to the _end_ parameter. The
return value of the operator method is the value of the range expression.


`operator[:](` _beginendvalue_ `)` is called for the result of the _postfix-expr_ if the range is to be written.
The result of _expr_ 1 is passed to the _begin_ parameter, the result of _expr_ 2 to the _end_ parameter and
the write value to the _value_ parameter. Any return value is ignored.


12This is the case, for example, if the index operator is used on the left side of the assignment operator. The
result of the expression on the right side of the assignment operator is then passed as _value_ on to the index operator.


53


**Function Calls**


For function calls, the first expression ( _postfix-expr_ ) has to deliver an object of a reference type
that implements the function call operator ( `operator()` ), such as the predefined function types,
`Func` and `CFunc` . An object of this sort is referred to as a function in the following.


For function calls, the two following cases can be distinguished in regard to the called function:


_•_ The called function is a common function or a class-oriented (static) method.


_•_ The called function is an instance-oriented method.


The difference when calling an instance-oriented method compared to calling a class-oriented
(static) method or a common function is that the object for which the method is called is implicitly passed to an instance-oriented method as a `self` parameter.


If the function to be called is an instance-oriented method and the expression delivering the method
is in the form of _postfix-expr_ 2 `.` _ident_, the result of _postfix-expr_ 2 is passed as a `self` parameter.
Otherwise, the caller must be an instance-oriented method and the `self` of the calling method is
passed as the `self` parameter of the instance-oriented method being called.


An exception is triggered if no object can be passed as `self` for an instance-oriented method [13] or
if the class of the object passed as `self` is not equal to the class or or one of its derived classes for
which the called instance-oriented method was defined.


The passing of arguments is analogous to the assignment as value for simple types ( _call by value_ )
and as reference for reference types ( _call by reference_ ). Exactly the same number of arguments as
specified in the function definition must be passed unless the function is defined as a function with
a variable number of arguments. In this case, the number of passed arguments may be no more
than one less that the number of declared parameters. All further arguments are assigned to the
last parameter in the form of a vector.


The return value of a function call is the value that was passed in the called function to the `return`
statement or `NULL` (see Section 3.5.3).


The function call operator can be defined for classes as follows:


`operator()(` _parameters_ `)` is called for the instance of a class if the instance is the result of the
_postfix-expr_ . The arguments of the function call are passed in the manner described above to the
parameters of the function call operator declared in the place of _parameters_ . The return value of
the function call operator method is the result of the function call.


**Postfix Incrementation and Decrementation**


The operand of a postfix increment or decrement operator must be a variable, an index expression
or a range expression. Otherwise, a translation error occurs.


The postfix increment and decrement operators, `++` and `--`, behave as follows depending on the
type of the operand:


13This is the case if the calling function is a common function or a class-oriented (static) method.


54


If the value of the operand is a simple type, it has to be an `Int` or `Float` . Otherwise, an exception
is triggered. Then, the following equivalence applies to the processing of the operator:


_expr⊕⊕≡_ `(` _tmp_ `=` _expr_ `,` _expr_ `=` _tmp ⊕_ `1,` _tmp_ `)`,


where _tmp_ is an unnamed variable created dynamically for the length of processing this subexpression and subexpressions of the _expr_ expression are only processed once. The addition or subtraction
follows the rules listed in Section 3.3.3.


If the value of the operand is a reference type, the `operator++(` _value_ `)` or `operator--(` _value_ `)`
operator method is called for the postfix increment or decrement operator for this reference type.
`NULL` is passed to the _dummy_ parameter. It is used only to distinguish from the corresponding
prefix increment or decrement operator. The return value of the operator method is the result of
the operator.


**3.6.4** **Unary Operators**


Unary expressions are right-associative.


_unary-expr:_
_postfix-expr_
_unary-op unary-expr_
_unary-op:_
`+` _|_ `-` _|_ `++` _|_ `--` _|_ `~` _|_ `!` _|_ `!!` _|_ `$`


**Unary Plus and Minus Operator**


The unary plus and minus operators, `+` and `-`, behave as follows depending on the type of the
operand:


If the value of the operand is a simple type, it has to be an `Int` or `Float` . Otherwise, an exception
is triggered. In the case of the plus operator, the value of the operand is equal to the result of
the operator. In the case of the minus operator (arithmetical negation operator), the result of the
operator is equal to the value of the operand multiplied by _−_ 1 [14] .


If the value of the operand is a reference type, the `operator+(` _value_ `)` or `operator-(` _value_ `)` operator
method is called for the unary plus or minus operator for this reference type. The value of the
operand is passed as a _value_ parameter to this method, the return value of which is the result of
the operator.


14Due to the use of the complement of two for representing integer numbers, the arithmetical negation of the
greatest-valued representable negative value is equal to this value.


55


**Prefix Incrementation and Decrementation**


The operand of a prefix increment or decrement operator must be a variable, an index expression
or a range expression. Otherwise, a translation error occurs.


The prefix increment and decrement operators, `++` and `--`, behave as follows depending on the
type of the operand:


If the value of the operand is a simple type, it has to be an `Int` or `Float` . Otherwise, an exception
is triggered. Then, the following equivalence applies to the processing of the operator:


_⊕⊕expr ≡_ `(` _expr_ `=` _tmp_ `=` _expr ⊕_ `1,` _tmp_ `)`,


where _tmp_ is an unnamed variable created dynamically for the length of processing this subexpression and subexpressions of the _expr_ expression are only processed once. The addition or subtraction
follows the rules listed in Section 3.3.3.


If the value of the operand is a reference type, the ) `operator++()` or ) `operator--()` operator
method is called for the prefix increment or decrement operator for this reference type. Any return
value from these methods is ignored. The value of the operand is equal to the result of the operator.


**Bitwise Negation**


The bitwise negation operator, `~`, behaves as follows depending on the type of the operand:


If the value of the operand is a simple type, it has to be an `Int` . Otherwise, an exception is
triggered. The result of the operator is then equal to the bitwise negation of the value of the
operand.


If the value of the operand is a reference type, the `operator~()` operator method is called for
bitwise negation for this reference type. The return value of this method is equal to the result of
the operator.


**Logical Negation**


The operand of the logical negation operator, `!`, is evaluated in test context. Its result is _true_ if
the value of the operand is _false_ and _false_ if the value of the operand is _true_ .


**The Test Operator**


The operand of the test operator, `!!`, is evaluated in test context. Its result is identical to the
value of the operand.


**The Symbol Resolution Operator**


The symbol resolution operator, `$`, requires an argument of the `Symbol` type. Otherwise, an
exception is triggered. It cannot be redefined for reference types.


The symbol resolution operator dynamically binds the symbol that its operands deliver to a variable. This takes place according to the rules for binding identifiers, which are specified in Section
3.7.5.


56


**3.6.5** **Multiplicative Operators**


Multiplicative expressions are left-associative.


_mul-expr:_
_unary-expr_
_mul-expr mul-op unary-expr_
_mul-op:_

`*` _|_ `/` _|_ `%`


The multiplicative operators, `*`, `/` and `%`, behave as follows depending on the type of the left
operand:


If the value of the left operand is a simple type, it has to be an `Int` or `Float` . The value of the
right operand, then, must also be an `Int` or `Float` . If either of these conditions are violated, an
exception is triggered. Otherwise, the operation takes place in `Int` if both operands are of the `Int`
type, or in `Float` if at least one of the operands are of the `Float` type. In the second case, an
operand of the `Int` type is converted to `Float` before the operation.


The `*` operator multiplies the two operands.


The `/` operator divides the two operands, where the left operand is the dividend and the right
operand, the divisor. If both operands are integers, the result is rounded towards 0.


The `%` operator determines the remainder of an implicit division for which the left operand is the
dividend and the right, the divisor.


If the remainder operation is carried out in `Int`, ( _a/b_ ) _∗b_ +( _a_ % _b_ ) = _a_ applies to the value calculated
by the remainder operator. It therefore follows, that the +/- sign of the remainder is the same
as the +/- sign of the dividends. Furthermore, the value of the remainder is always less than the
value of the divisor.


If the remainder operation is executed in `Float`, the result is the value of _a_ _−_ _i_ _∗_ _b_, where the integer
value of _i_ is selected so that the result carries the same +/- sign as _a_ and the value of the result
is less than the value of _b_ .


The calculations are carried out according to the rules listed in Section 3.3.3.


If the value of the left operand has a reference type, one of the following operator methods is called
for the left operand:


`operator*(` _rhs_ `)` (multiplication),


`operator/(` _rhs_ `)` (division)


`operator%(` _rhs_ `)` (remainder)


Here, the value of the right operand is passed as an _rhs_ parameter. The return value of the operator
method is the result of the operator.


57


**3.6.6** **Additive Operators**


Additive expressions are left-associative.


_add-expr:_
_mul-expr_
_add-expr add-op mul-expr_
_add-op:_
`+` _|_ `-`


The additive operators, `+` and `-`, behave as follows depending on the type of the left operand:


If the value of the left operand is a simple type, it has to be an `Int` or `Float` . The value of the
right operand, then, must also be an `Int` or `Float` . If either of these conditions are violated, an
exception is triggered. Otherwise, the operation takes place in `Int` if both operands are of the `Int`
type, or in `Float` if at least one of the operands are of the `Float` type. In the second case, an
operand of the `Int` type is converted to `Float` before the operation.


The `+` operator adds the two operands.


The `-` operator subtracts divides the two operands, where the left operand is the minuend and the
right operand, the subtrahend.


If the value of the left operand has a reference type, one of the following operator methods is called
for the left operand:


`operator+(` _rhs_ `)` (addition)


`operator-(` _rhs_ `)` (subtraction)


Here, the value of the right operand is passed as an _rhs_ parameter. The return value of the operator
method is the result of the operator.


**3.6.7** **Bitwise Shifts**


Expressions for bitwise shifts are left-associative.


_shift-expr:_
_add-expr_
_shift-expr shift-op add-expr_
_shift-op:_
`<<` _|_ `>>` _|_ `>>>`


The shift operators, `<<` (left shift), `>>` (signed right shift) and `>>>` (unsigned right shift) behave as
follows depending on the type of the left operand:


If the value of the operand is a simple type, both operands must be of the `Int` type and the
right operand must be nonnegative. If these conditions are not met, an exception is triggered.


58


Otherwise, the left operand is interpreted as a bit sequence, which is shifted by the number of
positions specified by the right operand either left ( `<<` ) or right ( `>>` and `>>>` ). The `<<` and `>>>`
operators fill vacated positions with 0-bits, while the `>>` operator fills vacated positions with the
value of the highest bit before the operation.


If the value of the left operand has a reference type, one of the following operator methods is called
for the left operand:


`operator<<(` _rhs_ `)` (left shift),


`operator>>(` _rhs_ `)` (signed right shift)


`operator>>>(` _rhs_ `)` (unsigned right shift)


Here, the value of the right operand is passed as an _rhs_ parameter. The return value of the operator
method is the result of the operator.


**3.6.8** **Relational Operators**


Relational expressions are left-associative [15] .


_comp-expr:_
_shift-expr_
_comp-expr comp-op shift-expr_
_comp-expr_ `instanceof` _shift-expr_
_comp-op:_
`<` _|_ `>` _|_ `<=` _|_ `>=`


The relational operators, `<` (less than), `<=` (less than or equal to), `>=` (greater than or equal to)
and `>` (greater than), behave as follows depending on the type of the left operand:


If the value of the left operand is a simple type, it has to be an `Int`, `Float` or `Symbol` . The value
of the right operand must be an `Int` or `Float` if the left operand is an `Int` or `Float`, or it must
be a `Symbol` if the left operand is a `Symbol` . If any of these conditions is violated, an exception is
triggered.


If none of the operands is `Symbol`, the relational operation takes place in `Int` if both operands are
of the `Int` type, or in `Float` if at least one of the operands are of the `Float` type. In the second
case, an operand of the `Int` type is converted to `Float` before the operation.


If the operands are of the `Symbol` type, a comparison of the internal representation of both symbols
takes place. The result of this comparison can only be guaranteed reproducible in an instance of
an OFML program.


The `<` operator yields _true_ if the value of the left operand is less than the value of the right operand.


The `<=` operator yields _true_ if the value of the left operand is less than or equal to the value of the
right operand.


15Note that consecutively written relational expressions do not follow common mathematical syntax: `0 < x < 5`
is interpreted as `(0 < x) < 5` and always returns the value of 1.


59


The `>=` operator yields _true_ if the value of the left operand is greater than or equal to the value of
the right operand.


The `>` operator yields _true_ if the value of the left operand is greater than the value of the right
operand.


If the relational operator does not yield _true_, it yields _false_ .


If the value of the left operand has a reference type, one of the following operator methods is called
for the left operand:


`operator<(` _rhs_ `)` (less than),


`operator<=(` _rhs_ `)` (less than or equal to),


`operator>=(` _rhs_ `)` (greater than or equal to),


`operator>(` _rhs_ `)` (greater than)


Here, the value of the right operand is passed as an _rhs_ parameter. The return value of the
operator method is interpreted in test context and converted to a logical value as described in
Section 3.6.1 [16] .


**Type Verification**


As the value of the right expressions, the `instanceof` operator expects a type derived from the
`Type` type (see Section 3.4.1). Otherwise, an exception is triggered. The result of the `instanceof`
operator is _true_ if


_•_ the left expression returns a value of a simple type whose type is identical to the type returned
by the right expression, or


_•_ the left expression returns a value of a reference type which is either identical to the type
returned by the right expression or has been derived from this.


Otherwise, the result of the operator is _false_ .


**3.6.9** **Equality Comparisons**


Equality comparisons are left-associative.


_equiv-expr:_
_comp-expr_
_equiv-expr equiv-op comp-expr_
_equiv-op:_
`==` _|_ `!=` _|_ `~=`


16Subsequently, the return value should though is not required to be an `Int` value.


60


If the value of the right operand of the relational operators, `==` (equality) and `!=` (inequality), is
`NULL`, both operands are considered equal if the value of the left operand is also `NULL` . The `==`
operator returns _true_ in case of equality, otherwise it returns _false_ . The `!=` operator returns _false_
in case of equality, otherwise _true_ .


Otherwise, the relational operators, `==`, `!=` and `~=` (pattern match), behave as follows depending
on the type of the left operand:


If the value of the left operand has a reference type, one of the following operator methods is called:


`operator==(` _rhs_ `)` (equality),


`operator!=(` _rhs_ `)` (inequality),


`operator =(` _rhs_ `)` (pattern match)


Here, the value of the right operand is passed as an _rhs_ parameter. The return value of the operator
method is interpreted in test context and converted to a logical value as described in Section 3.6.1.


If the value of the left operand has a simple type, an exception is triggered in the case of the `~=`
operator. In the case of the `!=` operator, the result is the logical negation of the result of the `==`
operator, applied to the same operands. The `==` operator behaves as follows:


If the value of the left operand is `NULL`, the result is _true_ if the value of the right operand is also
`NULL`, or _false_ if the value of the right operand is not `NULL` .


If the value of the left operand is of the `Symbol` type, the value of the right operand also has to
be of the `Symbol` type. Otherwise, an exception is triggered. The result is _true_ if both symbols
embody the same string, otherwise it is, _false_ .


If the value of the left operand is of the `Int` or `Float` type, the value of the right operand also
has to be of the `Int` or `Float` type. Otherwise, an exception is triggered. The comparison takes
place in `Int` if both operands are of the `Int` type, otherwise it takes place in `Float` . In the second
case, any `Int` type operand is converted to `Float` . The result is _true_ if both operands (also after
conversion, if one takes place) have the identical value, otherwise it is _false_ .


**3.6.10** **Minimum and Maximum**


Minimum and maximum operators are left-associative.


_minmax-expr:_
_equiv-expr_
_minmax-expr minmax-op equiv-expr_
_minmax-op:_
`<?` _|_ `>?`


For the `<?` (minimum) and `>?` (maximum) operators, the following equivalencies apply for processing the minimum and maximum operators:


_a_ `<?` _b ≡_ `(` _tmp_ 1 `=` _a_ `,` _tmp_ 2 `=` _b_ `,` _tmp_ 1 `<` _tmp_ 2 `?` _tmp_ 1 `:` _tmp_ 2 `)`

_a_ `>?` _b ≡_ `(` _tmp_ 1 `=` _a_ `,` _tmp_ 2 `=` _b_ `,` _tmp_ 1 `>` _tmp_ 2 `?` _tmp_ 1 `:` _tmp_ 2 `)`


Here, _tmp_ 1 and _tmp_ 2 are unnamed variables created dynamically for the length of processing this
subexpression.


61


**3.6.11** **Bitwise Links**


Expressions for bitwise links are left-associative.


_bit-and-expr:_
_minmax-expr_
_bit-and-expr_ `&` _minmax-expr_
_bit-xor-expr:_
_bit-and-expr_
_bit-xor-expr_ `^` _bit-and-expr_
_bit-or-expr:_
_bit-xor-expr_
_bit-or-expr_ `|` _bit-xor-expr_


The bitwise link operators, `&` (bitwise AND), `^` (bitwise exclusive OR) and `|` (bitwise OR), behave
as follows depending on the type of the left operand:


If the value of the operand is a simple type, both operands must be of the `Int` type. Otherwise,
an exception is triggered. The result is of the `Int` type.


If the value of the left operand has a reference type, one of the following operator methods is called
for the left operand:


`operator&(` _rhs_ `)` (bitwise AND),


`operator^(` _rhs_ `)` (bitwise exclusive OR),


`operator|(` _rhs_ `)` (bitwise OR)


Here, the value of the right operand is passed as an _rhs_ parameter. The return value of the operator
method is the result of the operator.


**3.6.12** **Logical Links**


Expressions for logical links are left-associative.


_logic-and-expr:_
_bit-or-expr_
_logic-and-expr_ `&&` _bit-or-expr_
_logic-or-expr:_
_logic-and-expr_
_logic-or-expr_ `||` _logic-and-expr_


The `&&` (logical AND) and `||` (logical OR) operators evaluate their left operands in test context.
If the value for the `&&` operator is _false_, the right operand is not evaluated and the result of the
operator is _false_ . Accordingly, the right operand is not evaluated and the result of the operator is
_true_ for the `||`, operator if the left operand yields _true_ . Otherwise, both operators evaluate their
right operands in test context and their result is equal to the value of the right operand.


Principally, the right operand is not evaluated if the result of the operator is determined by the
result of the left operand.


62


**3.6.13** **Conditional Expression**


Conditional expressions are right-associative.


_cond-expr:_
_logic-or-expr_
_logic-or-expr_ `?` _expr_ `:` _cond-expr_


For the conditional expression, the first operand ( _logic-or-expr_ ) is evaluated in test context. If the
evaluation yields _true_, the second operand ( _expr_ ) is evaluated and the result of the conditional
expression is equal to the value of the second operand. If the evaluation of the first operand yields
_false_, the third operand ( _cond-expr_ ) is evaluated and the result of the conditional expression is
equal to the value of the third operand.


Either the second or the third operand is evaluated, never both.


**3.6.14** **Assignment Operators**


All assignment operators are right-associative.


_assign-expr:_
_cond-expr_
_unary-expr assign-op assign-expr_
_assign-op:_
`=` _|_ `+=` _|_ `-=` _|_ `*=` _|_ `/=` _|_ `%=` _|_ `<<=` _|_ `>>=` _|_ `&=` _|_ `^=` _|_ `|=`


The left operand of an assignment must be a variable, an index expression or a range expression.
Otherwise, a translation error occurs.


If the left operand is a variable, the value of the right operand is calculated by the `=` assignment
operator and the variable is assigned. This value is the result of the assignment operator.


If the left operand is an index or range expression, the value of the right operand is calculated
first by the assignment operator. Then, the subexpressions (sequence, index or indices) of the left
operand are calculated and the index or range operator is called to set a value to the value of right
operand as an argument. The value of the right operand is the result of the assignment operator.


The combined assignment operators, `*=`, `/=`, `%=`, `+=`, `-=`, `<<=`, `>>=`, `>>>=`, `&=`, `^=` and `|=` first calculate
the value of the right operand. Then, the value of the left operand is calculated. Depending on its
type, processing continues as follows:


If the value of the left operand has a simple type, the following equivalence applies to the processing
of the combined assignment operator:


_lhs ⊕_ `=` _rhs ≡_ `(` _tmp_ 1 `=` _rhs_ `,` _lhs_ `=` _tmp_ 2 `=` _lhs ⊕_ _tmp_ 1 `,` _tmp_ 2 `)`


Here, _tmp_ 1 and _tmp_ 2 are unnamed variables created dynamically for the length of processing this
subexpression. Subexpressions of the left operand ( _a_ ) are calculated only once.


63


If the value of the left operand has a reference type, one of the following operator methods is called
for the left operand:


`operator*=(` _rhs_ `)` ( `*=` operator),


`operator/=(` _rhs_ `)` ( `/=` operator),


`operator%=(` _rhs_ `)` ( `\%=` operator),


`operator+=(` _rhs_ `)` ( `+=` operator),


`operator-=(` _rhs_ `)` ( `-=` operator),


`operator<<=(` _rhs_ `)` ( `<<=` operator),


`operator>>=(` _rhs_ `)` ( `>>=` operator),


`operator>>>=(` _rhs_ `)` ( `>>>=` operator),


`operator&=(` _rhs_ `)` ( `&=` operator),


`operator^=(` _rhs_ `)` ( `^=` operator),


`operator|=(` _rhs_ `)` ( `|=` operator)


Here, the value of the right operand is passed as an _rhs_ parameter. The return value of the operator
method is the result of the combined assignment operator.


**3.6.15** **The Comma Operator**


The comma operator is left-associative.


_expr:_
_assign-expr_
_expr_ `,` _assign-expr_


The left operand is evaluated in secondary context. Then, the right operand is evaluated. Its value
is the result of the comma operator. [17]


Table 3.1 summarizes once more the precedence and associativity for all operators. Here, the lowest
number represents highest precedence.

#### **3.7 Packages and Namespaces**


**3.7.1** **Module**


Every translation unit forms a module. A module belongs to a package, which is optionally specified
at the beginning of the module with the `package` statement (see Section 3.7.3).


17Note that, based on the grammar defined here, comma expressions in contexts in which the comma has another
syntactic meaning (such as in argument lists of function calls) must be placed within brackets in order to achieve
the targeted effect.


64


|Operators|Precedence|Associativity|
|---|---|---|
|`::`<br>`() @() [] .`<br>`! !! ~ ++ -- + - $`<br>`* / %`<br>`+ -`<br>`<< >> >>>`<br>`< <= > >= instanceof`<br>`== != ~=`<br>`>? <?`<br>`&`<br>`^`<br>`|`<br>`&&`<br>`||`<br>`?:`<br>`= *= /= %= += -= <<= >>= >>>= &= ^= |=`<br>`,`|1<br>2<br>3<br>4<br>5<br>6<br>7<br>8<br>9<br>10<br>11<br>12<br>13<br>14<br>15<br>16<br>17|left<br>left<br>right<br>left<br>left<br>left<br>left<br>left<br>left<br>left<br>left<br>left<br>left<br>left<br>right<br>right<br>left|


Table 3.1: Operators


A module forms a namespace, which implicitly contains all names defined within its package as
`public` or private to the package (i.e. without `public` or `private` ). In addition to these, other
names from other packages can be implicitly or explicitly imported (see Section 3.7.4), and new
names can be defined within a module.


Qualified access to names in the namespace is not possible.


If an attempt is made to explicitly reimport or redefine an explicitly imported name within a
module, a translation error occurs. Implicitly imported names can be imported implicitly more
than once and imported explicitly or defined no more than once.


An explicitly imported or defined name obscures all implicitly imported names of the same name.


If multiple identical names are implicitly imported from different packages, these implicitly Import
names are no longer visible.


A module is considered _loaded_ if it has been translated to the point where all the definitions on the
module and class levels in it have been processed and the corresponding names can be referenced
by the compiler while translating other modules. Compound statements do not yet have to have
been translated.


**3.7.2** **Packages and Namespaces**


The following namespaces exist In OFML: package (see Section 3.7.3), module (see Section 3.7.1),
class (see Section 3.8) and compound statement (see Section 3.5.3).


65


The namespaces of packages and classes from a hierarchy, where the individual components of a
name are separated by double colons `::` . If such a name begins with `::`, the search begins in the
root package, otherwise it begins in the package to which the translated module belongs. If the `::`
operator is located in the middle of a name, the identifier specified to the right of it is searched for
in the package or class specified to its left. Searching for names if `::` is not present is subject to
other rules (see Section 3.7.5). The syntax for names is:


_name:_
_ident_
_qualified-name_
_fully-qualified-name_
_fully-qualified-name:_
`::` _ident_
`::` _qualified-name_
_qualified-name:_
_name-qualifier_ `::` _ident_
_name-qualifier:_
_ident_
_name-qualifier_ `::` _ident_


If using a not fully qualified name, the first component must be a class that has been define in the
package to which the translated module belongs or a direct subpackage of this package.


For (qualified) access to a name that has not yet been loaded, an attempt is made to load it into the
corresponding package. For unqualified names, this is the package to which the accessing module
belongs. For qualified names, it is the package that is specified by the qualifier. For loading, the
name is mapped through an implementation-dependent mechanism to the name of a module, which
is then loaded.


**3.7.3** **Package Statement**


The syntax of a package statement is:


_package-stmt:_
`package` _fully-qualified-name_ `;`


The module is translated in the specified package. If a package statement is not present, the
module is translated in the root package (or default package).


During the translation of the package statement, all names of the package defined within the
specified package as `public` or private to the package (i.e. without `public` or `private` ) are imported
implicitly into the translated module. If a package statement is not present, the names of the root
package are imported implicitly.


66


**3.7.4** **Import Statement**


The syntax of the import statement is:


_import-stmt:_
`import` _fully-qualified-name_ `;`
```
     import :: * ;
```

`import` _fully-qualified-name_ `::` `* ;`


The first form of the import statement checks whether the specified name has already been defined.
If not, the module that defines the name is determined through an implementation-dependent
mechanism. This module is loaded. Then, the name in the namespace of the importing module
is carried over. It is an error to import names whose last components are identical or are defined
within the importing module using this form of the import statement.


The second and third forms of the import statement first check that all modules of the specified
package have been loaded. Then, all of the names of this package defined as `public` are copied
into the importing module.


Importing a name that belongs to the same package as the importing module is not possible.


Note that the import statement imports the names into the namespace of the module and not
into that of the package to which the module belongs. This is necessary to prevent modules from
influencing each other reciprocally via import statements.


**3.7.5** **Static and Dynamic Bindings**


In principle, names (both simple and (fully) qualified) are bound statically.


The point operator (see Sections 3.6 and 3.8.3) can be understood as a binary operator that
anticipates an object on the left and an identifier on the right. The identifier is bound to the
corresponding attribute of the object dynamically during runtime.


The definition of OFML allows the implementation to generate attributes dynamically during runtime. To check for undefined names, such attributes must be accessed via `self` (see Section3.8.3).


Binding a simple (unqualified) name takes place in the following order:


1. Within functions and methods, the name is searched for in the namespace of the innermost
compound statement. If the name is not found there, it is searched for from the inside
outwards until the namespace that continues the compound statement that forms the function
body is reached.


If the name cannot be found within a method (either instance-oriented or class-oriented), the
search continues in the namespace of the class to which the method belongs.


If the name cannot be found within a (common) function, the search continues within the
module in which the function was defined.


67


2. Within classes, the name is searched for in the namespace of the class. This contains all of
the names inherited from super-classes. If an instance-oriented method or variable from a
class-oriented method or a class-oriented initializer is found, a translation error is triggered.


If the name is not found within the namespace of the class, the search continues in the module
in which the class was defined.


3. The search in the module takes place only in the namespace of the module. This contains
all imported names [18] .


**3.7.6** **Visibility and Accessibility**


A simple name is visible if it can be bound according to the rules described in Section 3.7.5.


A qualified name is visible if it has not been defined as `private` or if a simple name consisting
only of the last component of the qualified name refers to the same definition and is visible.


A name to the right of the point operator is visible if it exists in the namespace of the class of the
object on the left side of the point operator and either has not been defined as `private` or access
takes place from within the same class.


The visibility of a simple name can be limited if it is covered by the same name in another
namespace, where the search is more likely to take place, as is described in Section 3.7.5.


A name is accessible if it is visible and access allows it. For unqualified access, every visible name
is also accessible. For qualified access or for access by means of the point operator, a visible name
may be inaccessible under some circumstances.


Accessibility and visibility are controlled by modifiers at the start of a definition. This section
describes only the modifiers for the definition of variables, functions and classes on the level of
modules. Modifiers for class attributes are described in Section 3.8.


_global-modifiers:_
_global-modifiersopt global-modifiers_
_global-modifier:_
`final` _|_ `public` _|_ `private`


The effect of the `final` keyword on variables is that no new values can be assigned to them after
the initialization requested within the variable definition. With OFML, however, variables as
well as functions and classes on the level of modules can be redefined by different modules or by


18To conserve memory, the implementation does not by requirement have to adopt every single name at the time
of translation. It is necessary, however, when removing a name, to search through all imported modules for the
name to exclude the possibility of ambiguity. For names for which this has already been done, an entry can be made
in the symbol table of the module to minimize the amount of processing necessary the next time the same name is
used.


The same applies to names that belong to the package of the module but which were not defined by the module.
Names that were defined by the module are located in the module’s symbol table anyway.


The search order for access to an unqualified name is as then as follows: 1. symbol table of the module, 2. symbol
table of the package, 3. all imported modules.


68


retranslating the (possibly modified) same module, in which case the value of the variables defined
with `final` can also change.


Classes defined as `final` cannot be used as super-classes of other classes.


For function definitions, `final` cannot be applied.


Variables in which functions and classes are stored are defined implicitly as `final` .


Variables, functions and classes defined as `public` are accessible in all modules, even in those of
other packages.


Variables, functions and classes defined as `private` are visible and accessible only within their
defining modules.


If a variable, function or class is defined as neither `public` nor `private`, it is handled as private to
the package. Names defined in this manner are generally visible, but accessible only from modules
belonging to the same package [19] .

#### **3.8 Classes**


**3.8.1** **Class Definitions**


Class definitions define new reference types and describe their implementation.


_class-def:_
_global-modifiersopt_ `class` _ident super-classopt class-body_


**3.8.2** **Super-classes**


_super-class:_
`:` _ident_


Optionally, the name of a super-class can be specified in a class definition. The specified super-class
cannot be defined as `final` . If a super-class is not specified, the class automatically inherits from
the `Object` root class (see Section 3.3.2).


If the super-class is defined within the same translation unit, its definition must appear before
the definition of the derived class. Furthermore, the constraint on using super-classes, which is
described in Section 3.1.2, must be observed.


A class inherits all attributes not defined as `private` from its super-class. These are placed in the
namespace of the subclass and are thus accessible in the subclass. Attributes of the super-class
defined as `private` are not visible in the subclass.


19Even though these names are visible to import statements for importing all the names of a package (in asterisk
form), these import statements do not try to access them (import them).


69


**3.8.3** **Attribute**


The body of a class definition consists of a sequence of attribute definitions and class-oriented
initializers.


_class-body:_
`{` _member-def-stmtsopt_ `}`
_member-def-stmts:_
_member-def-stmtsopt member-def-stmt_
_member-def-stmt:_
_field-def_
_method-def_
_static-initializer_


**3.8.4** **Data Fields**


_field-def:_
_field-modifiersopt_ `var` _init-expr-list_ `;`
_field-modifiers:_
_field-modifiersopt field-modifiers_
_field-modifier:_
`public` _|_ `protected` _|_ `private` _|_ `final` _|_ `static`


The syntax for defining data fields is the same as the syntax for defining variables (see Section
3.5.1) except that the `protected` and `static` modifiers are additionally allowed.


The initialization of class-oriented data fields takes place immediately after the module that contains the class definition is loaded. The initialization of instance-oriented data fields, including the
evaluation of the initialization expression, takes place in the order in which they occur immediately
after a new instance is created and before any `initialize()` method is called.


Data fields defined as `static` describe class-oriented variables that can exist only once per class.
Otherwise, they are instance-oriented variables, which are created new for every instance of the
class.


Data fields defined as `final` cannot be assigned new values after the initialization requested within
the variable definition.


Data fields defined as `public` are accessible from all modules, even those from other packages.


Data fields defined as `protected` are generally visible, but are accessible only from methods, classoriented initializers and initialization expressions from data fields of the same class or from classes
derived from this class.


Data fields defined as `private` are visible and accessible only in methods, class-oriented initializers
and initialization expressions from data fields of the same class or from classes derived from this
class.


70


Data field defined as private to the package (i.e. without one of the keywords, `public`, `protected`
or `private` ), are generally visible, but are accessible only from methods, class-oriented initializers
and initialization expressions from data fields of the same class or from classes derived from this
class as well as from all modules belonging to the same package.


**Methods**


_method-def:_
_method-modifiersopt_ `func` _method-name_ `(` _param-listopt_ `)` _compound-stmt_
_method-modifiersopt_ `func` _method-name_ `(` _param-list_ `, ...` `)` _compound-stmt_
`native` _method-modifiersopt_ `func` _method-name_ `( ) ;`
_method-modifiers:_
_method-modifier method-modifiersopt_
_method-modifier:_
`public` _|_ `protected` _|_ `private` _|_ `final` _|_ `static`
_method-name:_
_ident_
`operator` _method-operator_
_method-operator:_
`++` _|_ `--` _|_ `!!` _|_ `~` _|_ `*` _|_ `/` _|_ `%` _|_ `+` _|_ `-` _|_ `<<` _|_ `>>` _|_ `>>>`
`<` _|_ `<=` _|_ `>=` _|_ `>` _|_ `==` _|_ `!=` _|_ `~=` _|_ `&` _|_ `^` _|_ `|`
`*=` _|_ `/=` _|_ `%=` _|_ `+=` _|_ `-=` _|_ `<<=` _|_ `>>=` _|_ `>>>=` _|_ `&=` _|_ `^=` _|_ `|=`


The syntax for defining methods is the same as the syntax for defining named functions (see Section
3.5.1) except that modifiers and special names for redefining operators are additionally allowed.


Methods defined as `static` describe class-oriented methods that cannot access instance-oriented
variables. Class-oriented methods are called in conjunction with the type (see below). Otherwise,
it is an instance-oriented method, which is called in reference to a specific object that is an instance
of the class or an instance of a class derived from this class.


Methods defined as `final` cannot be redefined in subclasses.


The modifiers for controlling access have the same meaning as for data fields (see above).


**Redefinition of Operators**


Most of the operators supported by the OFML grammar can be redefined for reference types. To
do so, instance-oriented methods having names combined from the `operator` keyword, followed by
the operator being redefined, must be defined in the corresponding class definition. The number
of parameters to be declared for operator methods is defined in Section 3.6.


If an operator method is defined as `static` (class-oriented) or defined with an unallowed number
of parameters, a translation error occurs.


71


**Constructors**


Constructors are never defined explicitly, but instead are always created automatically. Userdefined operations for initializing instances can take place within the special `initialize()` instance
method, which is passed to the arguments that are passed to the constructor. If an `initialize()`
method is defined, it is called in the constructor automatically. If an `initialize()` method is
not defined, no arguments may be passed to the constructor. `initialize()` methods from superclasses are **not** called automatically. Such calls must take place explicitly in the `initialize()`
methods of their corresponding subclasses!


**Class-oriented Initializers**


_static-initializer:_
`static` _compound-stmt_


Class-oriented initializers consist of the `static` keyword, followed by a compound statement. Classoriented initializers are processed in the order in which they occur within the module together with
other executable statements on the module level and initializations of class-oriented variables after
the module that contains the class definition is loaded.


The following example illustrates this concept:

```
public class MyInt {
 private var value = 0; // instance variable
 private static var num; // class variable
 public func initialize() { // initialization method
  numInts++;
 }
 public func getValue() { // normal instance method
  return (value);
 }
 public func incr(i) {
  value += i;
 }
 public static func getNum() { // class method
  return (num);
 }
 static { // class-oriented initializer
  num = 0;
 }
}

```

**Access to Attributes**


Within instance methods of the same class, access to attributes is in general direct, meaning it
takes place within the current namespace. Alternatively, instance variables and methods can be


72


accessed using the special `self` keyword and of the ”‘ `.` ”’ access operator. If instance variables are
created dynamically by the implementation, as is the case with child objects in _OFML_, `self` must
be used to access them. Accordingly, in the example above, `return (self.value);` could have
been written instead of `return (value);` .


Access to instance methods and variables takes place via the ”‘ `.` ”’ operator. In this case, the
left operand is any expression, the type of which must be shared by the attribute, and the right
operand is the name of the attribute, e.g. `i.getValue()` .


Access to class methods and variables takes place via the ”‘ `::` ”’ operator according to the rules for
qualified names (see Section3.7.2), where the class name is used as a qualifier, e.g. `MyInt::getNum()` .

#### **3.9 Predefined Functions**


**3.9.1** **Standard Functions**


Standard functions are defined in the `::cobra::lang` package.


_typeOf(pObject(Object)) →_ _Type_

The _typeOf_ () function anticipates any value of a simple type or reference type as an argument
and returns the type of the argument.


**3.9.2** **Numerical Standard Functions**


All predefined numerical standard functions are defined in the `::cobra::math` package.


**Error Handling**


**Range Errors:** If an argument of a numerical standard function is outside the definition range
of the function, an exception is triggered.


**Overflow and Underflow Errors:** An overflow or underflow error occurs if the result of a
function cannot be represented as `Float` . If an overflow occurs (meaning the amount of the results is
so great that it cannot be represented in a `Float` ), the function returns the value `Float::HUGE_VAL`
with the same +/- sign as the correct value of the function (except in the case of `tan()` ). In the
case of an underflow, the result is `0` .


**Argument Conversion**


If an `Int` value is passed to one of the numerical standard functions instead of a `Float` value, the
`Int` value is converted implicitly by the function into a `Float` value.


73


**Trigonometrical Functions**


_acos(x(Float)) →_ _Float_

The _acos_ () function computes the arc cosine of _x_ in radians. An exception is triggered if _x_
is not in the interval [ _−_ 1 _,_ +1]. The result is in the interval [0 _, π_ ].


_asin(x(Float)) →_ _Float_

The _asin_ () function computes the arc sine of _x_ in radians. An exception is triggered if _x_ is
not in the interval [ _−_ 1 _,_ +1]. The result is in the interval [ _−π/_ 2 _,_ + _π/_ 2].


_atan(x(Float)) →_ _Float_

The _atan_ () function computes the arc tangent of _x_ in radians. The result is in the interval
( _−π/_ 2 _,_ + _π/_ 2).


_atan2(y(Float), x(Float)) →_ _Float_

The _atan_ 2() function computes the arc tangent of _y/x_ in radians. It uses the sign of both
arguments to compute the quadrants of the return value. If both arguments are 0, an
exception is triggered. The result is in the interval [ _−π,_ + _π_ ].


_cos(x(Float)) →_ _Float_

The _cos_ () function computes the cosine of _x_ (specified in radians).


_sin(x(Float)) →_ _Float_

The _sin_ () function computes the sine of _x_ (specified in radians).


_tan(x(Float)) →_ _Float_

The _tan_ () function computes the tangent of _x_ (specified in radians).


_acosh(x(Float)) →_ _Float_

The _acosh_ () function computes the hyperbolic arc cosine of _x_ . If _x_ is not in the interval

[1 _, ∞_ ), an exception is triggered.


_asinh(x(Float)) →_ _Float_

The _asinh_ () function computes the hyperbolic arc sine of _x_ .


_atanh(x(Float)) →_ _Float_

The _atanh_ () function computes the hyperbolic tangent of _x_ . If _x_ is not in the interval
( _−_ 1 _,_ +1), an exception is triggered.


_cosh(x(Float)) →_ _Float_

The _cosh_ () function computes the hyperbolic cosine of _x_ .


_sinh(x(Float)) →_ _Float_

The _sinh_ () function computes the hyperbolic sine of _x_ .


_tanh(x(Float)) →_ _Float_

The _tanh_ () function computes the hyperbolic tangent of _x_ .


74


**Exponential Functions and Logarithmic Functions**


_exp(x(Float)) →_ _Float_

The _exp_ () function computes the exponential function of _x_ (i.e. _e_ _[x]_ ).


_frexp(x(Float)) →_ _[Float, Int_ ]
The _frexp_ () function breaks a floating-point number into a normalized fractions ( _frac_ ) and
an integral power of 2 ( _exp_ ), where _x_ = _frac ·_ 2 _[exp]_ . Both values are returned as vector

[ _frac, exp_ ].


If _x_ is 0, both parts of the result are 0.


_ldexp(x(Float), exp(Int)) →_ _Float_

The _ldexp_ () function multiplies the floating-point number _x_ by the integral power _exp_ of 2.


_log(x(Float)) →_ _Float_

The _log_ () function computes the natural logarithm of _x_ . If the argument is negative, an
exception is triggered. If it is 0, the result is _−Float_ :: _HUGE_ ~~_V_~~ _AL_ .


_log10(x(Float)) →_ _Float_

The _log_ 10() function computes the base 10 logarithm of _x_ . If the argument is negative, an
exception is triggered. If it is 0, the result is _−Float_ :: _HUGE_ ~~_V_~~ _AL_ .


_modf(x(Float)) →_ _[Float, Float_ ]
The _modf_ () function breaks the argument into an integer part ( _int_ ) and a fractional part
( _frac_ ), of which both have the same sign as the argument. Both values are returned as vector

[ _int, frac_ ].


**Exponential Functions**


_pow(x(Float), y(Float)) →_ _Float_

The _pow_ () function computes _x_ to the power of _y_ . An exception is triggered if _x_ is negative
and _y_ is not an integer, or _x_ is 0 and _y_ is negative. The result is 1 _._ 0 if both _x_ and _y_ are 0.


_sqrt(x(Float)) →_ _Float_

The _sqrt_ () function computes the nonnegative square root of _x_ . If _x_ is negative, an exception
is triggered.


**Rounding, Absolute Value and Remainder**


_ceil(x(Float)) →_ _Float_

The _ceil_ () function computes the smallest integer not smaller than _x_ .


_fabs(x(Float)) →_ _Float_

The _fabs_ () function computes the absolute value of _x_ .


_floor(x(Float)) →_ _Float_

The _floor_ () function computes the largest integer not larger than _x_ .


75


_fmod(x(Float), y(Float)) →_ _Float_

For _y_ is not equal to 0, the _fmod_ () function computes the value _x −_ _i · y_ so that the result
for an integer _i_ has the same sign as _x_ and a magnitude less than the magnitude of _y_ . If _y_
is 0, an exception is triggered.


76


## **Chapter 4**

# **Basic Interfaces**

The basic interfaces described below implement fundamental concepts on which the actual types
of the OFML standard are based. Such a type implements one or several of these basic interfaces.


Each interface is assigned an interface category (Appendix H). By means of the general method
for determining the category association of a type or an instance described below, it is possible to
determine as an alternative to determining the type identity whether a type implements a specific
interface or whether an instance of the type provides the functionality of the interface.

#### **4.1 MObject**


The _MObject_ interface defines the fundamental interfaces of all OFML types. Consequently, every
OFML type implements at least this interface.


**4.1.1** **Type Identity and Category Association**


_• getType() →_ _Type_


The function provides the direct type of the implicit instance.


_• getClass() →_ _String_


The function provides the name of the direct type of the implicit instance.


**Note:** Equivalent to `String(getType().getName())` .


_• isA(pType(Type)) →_ _Int_


The function verifies the association to a transferred type _pType_ . _isA()_ furnishes 1 if _pType_
is the direct type of the implicit instance or a super type of it. Otherwise, the result is 0.


77


_• isCat(pCat(Symbol)) →_ _Int_


The function furnishes 1 if the implicit instance belongs to the transferred category.


**Note:** As a rule, a type inherits the association to categories from its direct super type. For this

reason, attention should generally be paid in overwriting the function that the inherited implemen
tation of the function is called for the transfer of a category that is not defined by the concrete type

itself.


**4.1.2** **Instance Identity and Hierarchy**


_• final getName() →_ _String_


The function returns the absolute name of the implicit instance.


_• final getFather() →_ _MObject_


The function provides a reference to the father object. If the implicit instance does not have
a father, the result is _NULL_ .


_• final getRoot() →_ _MObject_


The function furnishes a reference to the root instance of the hierarchy in which the implicit
instance is located.


_• final getChildren() →_ _MObject[]_


The function returns a list of object references that represent the direct children of the
implicit instance. If no children are available, an empty list is returned.


_• final getElements() →_ _MObject[]_


The function returns a list of object references that represent those direct children of the
implicit instance that are also elements. If no elements are available, an empty list is returned.


_• final add(pType(Type) ...) →_ _MObject_


The function creates a child of the implicit instance of type _pType_ and continues to register
it as element. The local name of the child is selected automatically. Should a type require
additional parameter for its instantiation, they must be specified with the _add()_ call after
_pType_ .


The return value of the function is a reference to the created object or _NULL_ .


_• final remove(pChild(MObject)) →_ _self_


The function removes the specified object, which is a child of the implicit instance, from the
list of children of the implicit instance. If it is an element at the same time, it is also removed
from the list of elements.


78


#### **4.2 Base**

As an extension of the _MObject_ interface, the _Base_ interface also represents a fundamental interface
of the OFML types that is implemented by most of the OFML types.


Every type that implements the _Base_ interface **also** implements the _MObject_ interface.


**4.2.1** **Instance Variables**


_• mIsCutable(Int)_


The variable specifies the independence of the instance with respect to the cut operation of
the clipboard ( _setCutable()_ and _isCutable()_ functions in section 4.2.2).


_• [static] eps(Float) = 0.005_


The static variable _eps_ must be used for geometric relation operations due to the limited
presentation accuracy of floating point numbers in OFML. Neither this variable nor the
following ones may be redefined. Non-redefinable variables can be designated with _final_ .


_• [static] sPi4(Float) =_ _[π]_ 4

_• [static] sPi2(Float) =_ _[π]_ 2


_• [static] sPi(Float) = π_


_• [static] s2Pi(Float) =_ 2 _π_


**4.2.2** **Selectability**


_• final selectable() →_ _self_


The function allows the selection of the implicit instance.


_• final notSelectable() →_ _self_


The function prohibits the selection of the implicit instance. In the case of an attempted
selection of the implicit instance, the first instance that is selectable within the scope of an
upward traversing is selected.


_• final hierSelectable() →_ _self_


The function allows the selection of all entities of the subhierarchy whose root object is the
implicit instance. Whether a single instance can actually be selected is determined by the
status that was set via _selectable()_ or _notSelectable()_ .


_• final notHierSelectable() →_ _self_


The function prohibits the selection of all entities of the subhierarchy whose root object is
the implicit instance. The prohibition applies to all entities of the subhierarchy, even if the
selection of an individual instance via _selectable()_ is allowed in principle. Thus, _notHierSe-_
_lectable()_ in reference to a single instance takes precedence over _selectable()_ .


79


_• final isSelectable() →_ _Int_


The function returns True if the implicit instance can be selected. This is the case if _se-_
_lectable()_ was called for the instance and was not called for an object in the hierarchy via
_notHierSelectable()_ instance.


Entities can be selected initially.


**4.2.3** **Cuttability**


_• setCutable(pMode(Int)) →_ _Void_


The function determines the independence of the implicit instance with respect to the cut
operation of the clipboard and saves the transferred mode in the _mIsCutable_ instance variable.
Possible values are:


-1 In general, the implicit instance may not be deleted.


0 The implicit instance itself may not be deleted, but it can be deleted within the scope
of a higher-level instance. In the case of an attempted cut operation of the implicit
instance, the operation is applied to the first instance in the course of an upward
traversing for which _isCutable()_ furnishes 1.


1 The implicit instance can be deleted and copied to the clipboard. This is the initial
state.


2 The implicit instance can be deleted, but it may not be copied to the clipboard.


**Example:** Mode 2 is used for objects such as cornice profiles that are constructed with regard to

a set of base objects and, consequently cannot readily be copied to another spatial or topological

position.


_• isCutable() →_ _Int_


The function queries the independence of the implicit instance with respect to the cut operation of the clipboard. It furnishes the value of the _mIsCutable_ instance variable that can be
described using the _setCutable()_ function.


_• removeValid() →_ _Int_


The function returns True if the implicit instance may be deleted.


Entities can be deleted initially.


In contrast to the _isCutable()_ function, which specifies the fundamental ability of deleting an
object and is called by the application prior to a cut operation on the selected object, the
_removeValid()_ function is used for modeling dynamic aspects of the ability of deleting and is
called by father entities within the scope of REMOVE ~~E~~ LEMENT rules.


80


**4.2.4** **Visibility**


_• final hide() →_ _self_


The function hides the implicit instance, including its children and grandchildren. Hiding
entities does not have any influence on collision recognition.


_• final show() →_ _self_


The function makes the implicit instance visible again if it was hidden.


_• final isHidden() →_ _Int_


The function indicates through its return value whether the implicit instance is visible (0) or
hidden (1).


**4.2.5** **Resolution**


The following functions can be used to set or query the object space resolution of an object. In
general, this applies to the mapping of an analytical or parametric primitive to a piece by piece
linear approximation. The direct conversion is done for geometric elementary types only (Chapter
7). All other types or entities merely pass on the resolution. Imported polygonal data records are
not effected.


Normally, the resolution should only be set directly for the root of an object hierarchy. However,
direct setting of the object space resolution for a non-root object is allowed.


The resolution is indicated by a floating point number _r_ in the range 0 _._ 0 _≤_ _r ≤_ 1 _._ 0. Where
0 _._ 0 represents the minimum resolution and 1 _._ 0 the maximum resolution. If the resolution 0 _._ 0 is
specified for a parametric primitive, its representation corresponds to the polygonal body that
results from the corresponding connection of the defining vertices. The initial resolution is 0 _._ 1.


_• final setResolution(pRes(Float)) →_ _self_


The function sets the object space resolution for the subtree preset by the implicit instance.
This resolution continues to be inherited in the subtree. If an ancestor already contains an
explicitly assigned object space resolution, the recursive inheritance is ended at this position
and for this path of the subtree.


_• final getResolution() →_ _Float_


The function returns the valid object space resolution for the implicit instance.


**4.2.6** **Change Status**


_• final setChanged() →_ _self_


The function explicitly marks the implicit instance as changed with respect to the instant
immediately after executing the initialization. An explicit call of _setChanged()_ is necessary
if instance variables are directly written without other changing operations being performed
(e.g., creation of children, move). The change status is evaluated to enable an efficient storing
of instance hierarchies which is applied to clipboard and persistence operations.


81


_• final setUnchanged() →_ _self_


The function resets the change status of the implicit instance to the status immediately
after executing the initialization. That is, the instance is now considered as unchanged with
respect to the instant immediately after the initialization.


**4.2.7** **Collision Detection**


_• final disableCD() →_ _Void_


The function deactivates the collision detection for the implicit instance. Afterwards, the
implicit instance, including its children, are ignored by the collision detection.


_• final enableCD() →_ _Void_


The function (re-)activates the collision detection for the implicit instance.


_• final isEnabledCD() →_ _Int_


The function furnishes 0, if the implicit instance is excluded from collision detection, otherwise
it furnishes 1.


**4.2.8** **Dimensioning**


_• measure(pMode(Symbol)) →_ _Void_


The function activates the dimensioning of the implicit instance. If necessary, different types
of dimensioning can be selected using the implementation-dependent value _pMode_ . If only
one type of dimensioning exists, the parameter may be ignored.


The following symbols are predefined for the dimensioning:


**–** _@ISO_ The dimensioning is done in meter.


**–** _@INCH_ The dimensioning is done in inch.


_• unMeasure() →_ _Void_


The function deactivates the dimensioning of the implicit instance.


**4.2.9** **Spatial Modeling**


_• final setPosition(pPosition(Float[3])) →_ _self_


The function unconditionally sets the local position [1] of the implicit instance, i.e., no rules
are called and the degrees of translation freedom are ignored to the position _pPosition_ . At
the same time, this position represents the move of the implicit instance compared with the
father of the implicit instance. Initially, an instance does not have a move with respect to its
father. If no father exists, the world coordinate system serves as reference.


The function is used for the explicit positioning within functions.


1Actually, the local coordinate system is moved to the respective position relative to the father.


82


_• final getPosition() →_ _Float[3]_


The function furnishes the current move of the implicit instance with respect to its father,
provided that he exists, or with respect to the world coordinate system.


_• final translate(pVector(Float[3])) →_ _self_


The function conditionally moves the implicit instance by the vector _pVector_ defined in the
world coordinate system. The conditionality of the move results from a possible rasterization
and snapping functionality, provided that it is supported by the OFML runtime environment,
translational degrees of freedom ( _setTrAxis()_ ), as well as through the presence of _TRANS-_
_LATE_ rules of the reason (Chapter 5). If _TRANSLATE_ rules are defined for the implicit
instance, they are directly called after executing the translation.


The function is used for interactive positioning via direct manipulation or user interface. The
_pVector_ vector is transformed from the world coordinate system to the local coordinate system
of the implicit instance under consideration of the current inherited and local modeling of
the implicit instance. This ensures an intuitive modeling via the _translate()_ function.


_• final moveTo(pPosition(Float[3])) →_ _self_


The function conditionally moves the implicit instance to the _pPosition_ position defined in
the world coordinate system. The semantics of the function completely corresponds to the
call of _translate()_ with the _pPosition_   - _getWorldPosition()_ vector.


_• final setTrAxis(pAxis(Int)) →_ _self_


The function permits or prohibits the movability of the implicit instance for individual axes
of the local coordinate system. The _pAxis_ parameter results from the addition of allowed
axes, whereby x, y and z-axis are represented by 1, 2 and 4. If _pAxis_ features the value 0,
the object cannot be moved.


_• final getTrAxis() →_ _Int_


The function furnishes the current movability of the implicit instance.


_• final rotate(pAxis(Symbol), pArc(Float)) →_ _self_


The function conditionally rotates the implicit instance by the _pArc_ angle [2] defined in the
radiant measure with respect to the _pAxis_ local coordinate axis. The conditionality of the
rotation results from a possible rasterization and snapping functionality, provided that it is
supported by the OFML runtime environment, rotational degrees of freedom ( _setRtAxis()_ ),
as well as through the presence of _ROTATE_ rules of the reason (Chapter 5). If _ROTATE_ rules
are defined for the implicit instance, they are directly called after executing the rotation.


_pAxis_ is either _@PX_, _@PY_ or _@PZ_ as long as a rotation about the (positive) x, y or zaxis occurs. Alternatively, a rotation about an opposite axis may be carried out. The
respective symbols are _@NX_, _@NY_ and _@NZ_ . The rotation about random axes is achieved
by corresponding consecutive rotations about the elementary axes.


In contrast to the translation, there is no function for unconditional setting for the rotation.
The setting of a certain orientation is carried out either initially, i.e., if no rotation compared
to the father has taken place, or through a subtraction of the actual orientation from the
orientation to be set. However, this does not invalidate the conditionality described above.


2Actually, the local coordinate system is rotated accordingly.


83


Within the scope of rules, the correction of orientations is carried out through a new application of the _rotate()_ function. In this case, the OFML runtime system must ensure that
_ROTATE_ rules are not called again.


A general issue concerning the rotation about cartesian axes consists of the overlay of the
three elementary rotations. For this reason and to ensure the correct functioning of the
rotations, it is recommended to release only one rotational axis at a time ( _setRtAxis()_ ).


The function is used for the interactive positioning via direct manipulation or user interface
as well as for the explicit positioning within functions.


_• final getRotation(pAxis(Symbol)) →_ _Float_


The function furnishes the current rotation in the radiant measure by the rotational axis
specified through _pAxis_ .


**Attention:** If an instance was rotated about more than one axis, _getRotation()_ could furnish
unexpected results. This is due to the principal problem of overlay of the three elementary
cartesian rotations.


_• final setRtAxis(pAxis(Int)) →_ _self_


The function permits or prohibits the ability of rotation of the implicit instance for individual
axes of the local coordinate system. The _pAxis_ parameter results from the addition of allowed
axes, whereby x, y and z-axis are represented by 1, 2 and 4. If _pAxis_ features the value 0,
the object cannot be rotated.


It should be possible to rotate entities about a maximum of one rotation axis. However, this
axis may be changed over time.


_• final getRtAxis() →_ _Int_


The function furnishes the current ability of rotation of the implicit instance.


_• final getLocalBounds() →_ _Float[2][3]_


The function furnishes the minimum axis-orthogonal delimiting volume of the implicit instance in reference to its local coordinate system. The delimiting volume includes children
and the origin of the local coordinate system.


The return value is a vector consisting of two elements. The first element is the minimum coordinate within the local delimiting volume. The second element is the maximum coordinate
within the local delimiting volume.


The OFML runtime environment must ensure that the local delimiting volume is always in
a consistent state.


_• final getLocalGeoBounds() →_ _Float[2][3]_


The function furnishes the minimum axis-orthogonal delimiting volume of the implicit instance with reference to its local coordinate system. In contrast to _getLocalBounds()_, the
delimiting volume does not include the origin of the local coordinate system and children
with empty geometry.


_• final getWorldBounds() →_ _Float[2][3]_


84


The function furnishes the minimum axis-orthogonal delimiting volume of the implicit instance in reference to the world coordinate system. The delimiting volume includes the
children.


The return value is a vector consisting of two elements. The first element is the minimum coordinate within the global delimiting volume. The second element is the maximum coordinate
within the global delimiting volume.


The OFML runtime environment must ensure that the global delimiting volume is always in
a consistent state.


_• final getWorldGeoBounds() →_ _Float[2][3]_


The function furnishes the minimum axis-orthogonal delimiting volume of the implicit instance in reference to the world coordinate system. In contrast to _getWorldBounds()_, the
delimiting volume does not include children with empty geometry.


_• final getDistance(pDirection(Symbol)) →_ _Float_


The function determines the shortest distance of the implicit instance along one of six directions, starting with the local delimiting volume to another instance in the scene. The
direction indicated by _pDirection_ features one of the following values: _@NX_, _@PX_, _@NY_,
_@PY_, _@NZ_, _@PZ_ .


The return value is the distance, provided that another instance could be determined, or _−_ 1.


**4.2.10** **Rule Call**


_• final callRules(pReason(Symbol), pArg(Any)) →_ _Int_


The function triggers the execution of the rules defined for the reason _pReason_ . _pReason_ is
either a predefined rule (Chapter 5) or a user-defined rule.


The explicit call of a predefined rule reason can be used, for example, to explicitly request
a snapping behavior that is implemented by means of a _TRANSLATE_ rule, following the
initial positioning of the instance. If a predefined rule reason is called, _pArg_ must correspond
to the specification in Chapter 5.


The explicit call of a user-defined rule reason via _callRules()_ is the only possibility to bring
the corresponding rules to be executed. A principal application of user-defined rule reasons
consist of enabling a communication between entities that is more flexible and robust than
the communication via the functions of types. In this case, the necessity for checking type
compatibility is not required; if no rules are defined in a type for a certain reason, no error
will occur. However, calling a function for a type that does not define this function, will
always result in an error.


**Example:** Spotlights can be planned for a system, but they can normally not be moved. However,

as children of a very few types they can be moved in a specific way. In the _TRANSLATE_ rule of

the spotlight, the move is reset so that the spotlight is not moved. This is followed by a call of

the father with _callRules()_, whereby the user-defined reason _MOVE_ ~~_S_~~ _POT_, the desired new position

of the spotlight, and the spotlight itself are transferred. If the father permits a movement of the

spotlight, it can control it with a corresponding _MOVE_ ~~_S_~~ _POT_ rule. Otherwise, the spotlight remains

unchanged.


85


The return value is _−_ 1 if a rule of the reason _pReason_ failed. Otherwise, the return value is
0.


**4.2.11** **Dynamic Properties**


Features of an instance whose current characteristics are stored in a corresponding instance variable
(and that can be assigned or queried using corresponding set and get functions), are referred to
as _static_ features or properties of the instance. In contrast, it is sometimes necessary to assign
dynamic properties and values to an instance for the duration of its existence. For this purpose,
the _Base_ interface manages an internal hash table for each instance, in which such properties can
be set up. A property is defined and addressed via its unique key of the _Symbol_ type. The value
for the key entered in the table can come from a simple type or from the reference types _String_,
_Vector_, _List_, and _Hash_ .


_• getDynamicProps() →_ _Hash_


The function furnishes the (reference to the) hash table for dynamic properties.


**4.2.12** **2D Representation and ODB**


Objects that implement the _Base_ interface can be equipped with a 2D representation. It is created
by means of the _getOdbInfo()_, _getPictureInfo()_, _invalidatePicture()_ methods and the methods for
creating primitive 2D objects, as described in Appendix B.


2D symbols created via _getOdbInfo()_ and _getPictureInfo()_ cannot be used simultaneously. If a hash
table is returned by _getOdbInfo()_, a symbol that may be specified by means of getPictureInfo()
will not be represented.


_• getOdbInfo() →_ _Hash_


The function is called by the core system at random times to query the current ODB information that is required for the creation of a 2D symbol or the 3D geometries. The function
returns the ODB information in form of a hash table or _NULL_ if no ODB information is
available for the object. The use of ODB is described in [ODB].


_• getPictureInfo() →_ _Vector_


The function is called by the core system at random times to query information about the
2D symbol that is to be used for this object. The return value is a vector consisting of three
elements:


**–** The first element is either _NULL_, in which case this object does not feature a 2D symbol,
or the fully qualified name of the symbol. The name is used to search for a corresponding
EGM, DMP, or FIG symbol. The first symbol to be found is used for the representation
in 2D mode. If no symbol can be found, a symbol is created automatically based on the
3D geometries for the object and all its current child objects.


86


**–** The second element is either _@TRAVERSAL_ ~~_S_~~ _TOP_ or _@TRAVERSAL_ ~~_C_~~ _ONT_ . It determines whether possibly available symbols of child objects should be represented
( _@TRAVERSAL_ _CONT_ ) or not ( _@TRAVERSAL_ ~~_S_~~ _TOP_ ) for the representation in 2D
mode. If a symbol is automatically generated for the object, this value should always
be set to _@TRAVERSAL_ ~~_S_~~ _TOP_ .

**–** The third element is either _@SHARE_ ~~_O_~~ _N_ or _@SHARE_ _OFF_ . It determines whether symbols with identical names from different objects should be used jointly ( _@SHARE_ ~~_O_~~ _N_ )
or whether the symbol for each object is loaded and generated again ( _@SHARE_ ~~_O_~~ _FF_ ).
In general, _@SHARE_ _ON_ should be specified in this case for symbols that are loaded
from files. For objects whose 2D symbol is automatically generated, _@SHARE_ _ON_ can
be specified if different entities with identical symbol name are always equipped with the
same 3D geometry, otherwise _@SHARE_ _OFF_ should be used. But it should be observed
that child objects (e.g., accessories) become part of the symbol, so that for the joint use
of automatically generated symbols for objects, that may possibly contain additional
children, these children are visible either for all objects or for none.


By default, _getPictureInfo()_ returns the type of class of the implicit instance as symbol name,
allows the traversing of child objects with representation in 2D mode, and prevents the joint
utilization of symbols.


**Note:** A change in the child objects does not automatically cause a matching of an automatically
generated symbol.


If possible, the automatic generation of 2D symbols should be abandoned since it can lead to no
ticeable delays, especially with repeated application, and the result is usually unsatisfactory for an

effective planning in 2D mode.


_• invalidatePicture() →_ _Void_


The function must be called after properties of the object that affect the 2D or 3D geometry
have changed. The core system discards all saved information (return values of _getOdbInfo()_
and _getPictureInfo()_ as well as 2D symbols (ODB, EGM, DMP, FIG, and generated ones)).
If required, this information is queried again and the 2D symbols are generated.


_• createOdbObjects(pUpdate(Int)) →_ _Void_


The function generates child objects according to the specification in the ODB. If the _pUpdate_
parameter is 0 (false), all currently existing child objects generated by the ODB are deleted
and then recreated. If the _pUpdate_ parameter is 1 (true), a matching of the existing child
objects generated by the ODB is carried out.


**Note:** In the current OFML implementation of EasternGraphics GmbH, all child objects generated

by the ODB are deleted independent of the Parameter _pUpdate_ parameter, and then recreated by

the ODB.

#### **4.3 Material**


The _Material_ interface defines the functions for processing material properties (surface properties)
on the basis of material categories. All types whose entities can be assigned material properties


87


must implement this interface.


All furniture or furniture components whose materials should be processed similarly due to functional and/or aesthetic viewpoints are combined in a material category. An instance may belong
to one or several material categories. Material categories are designated through symbols.


Predefined material categories are listed in Appendix H.


**Example:** Typical material categories are _corpus_, _front_, _base_, _tabletop_ . Entities of a cabinet type with

doors and/or drawers would then belong to the categories _corpus_, _front_ and _base_ while the child entities,

for example, that implement the corpus belong only to the _corpus_ category. The corresponding OFML

material categories could be: _@CORPUS_, _@FRONT_, _@BASE_ and _@TOP_ .


**Note:** The universally valid _@ANY_ material category is predefined (function _setCMaterial()_ ) and may

not be used in a different capacity.


For each material category, a limited set of possible materials is specified that is also designated by
symbols ( _getMatCategories()_ function). Material designators are unique across all material categories. The visual properties of a material are specified in a separate file whose format is described
in Appendix D.2. Each material designator must be assigned a material name ( _getMatName()_
function) to be able to read the corresponding material description file during runtime by using
the name.


**Example:** The materials ”gray laminate” and ”light beech veneer” are intended for the _corpus_ category,

and only the ”light beech veneer” material for the _front_ category. Possible designators for these materials
are _@LGray_ or _@VBlight_ [3] . Corresponding material names would be ”gray laminate” or ”light beech” and
the corresponding material description files `graylaminate.mat` or `lightbeech.mat` .


**4.3.1** **Material Categories**


_• getMatCategories() →_ _Symbol[]_


It furnishes the list of material categories that are currently defined for the implicit instance.
An instance for which no material categories are defined furnishes either an empty list or
_Void_ . In the latter case, the defined material categories for the father instance should be
used for the implicit instance.


The number of material categories defined for the implicit instance can change dynamically
and, therefore, differentiate themselves from the set of all potentially possible material categories ( _getAllMatCats()_ function).


_• isMatCat(pCat(Symbol)) →_ _Int_


It returns 1 if the transferred material category belongs to the material categories currently
defined for the implicit instance, otherwise 0.


3The material codes already in place in manufacturing companies are ideal for use as symbolic material designators.


88


_• getAllMatCats() →_ _Symbol[]_


It furnishes the list of all material categories that are potentially definable for the implicit
instance (see also _getMatCategories()_ function).


_• getCMaterials(pCat(Symbol)) →_ _Symbol[]_


It furnishes the list of all materials that are applicable within the transferred material category
for the implicit instance. The return value is of type _Void_ if the transferred material category
does not belong to the material categories currently defined for the implicit instance.


**4.3.2** **Materials**


_• setCMaterial(pCat(Symbol), pMat(Symbol)) →_ _Int_


It assigns the specified material to the implicit instance in the transferred material category.
The operation is recursively applied to all children and grandchildren. The function is without
effect if neither the implicit instance nor one of the children belongs to the transferred material
category. The return value is 1 if the material could be assigned to the implicit instance or
at least to one of its ancestors (child, grandchild, etc.) 0.


The predefined universally valid material category _@ANY_ can be used of explicitly assigning
a material without considering the association of the implicit instance to a concrete material
category.


_• getCMaterial(pCat(Symbol)) →_ _Symbol_


The function furnishes the material currently assigned to the implicit instance in the transferred material category or a value of the _Void_ type if the implicit instance does not currently
belong to the transferred material category.


_• getMatName(pMat(Symbol)) →_ _String_


The function furnishes the material name to the transferred material or a value of the _Void_
type for the implicit instance if the material is unknown. The standard implementation calls
the function of the father of the same name.

#### **4.4 Property**


_Properties_ are object features that can be changed interactively by the system user with the help
of suitable dialogs (property editors).The _Property_ interface defines the functions for handling
properties. Properties can be associated with features in product databases (Chapter 9).


**4.4.1** **Specifying Properties**


_• setupProperty(pKey(Symbol), pDef(Any[5]), pPos(Int)) →_ _Void_


The function creates a property with the specified key (identifier) and the transferred specification. If a property with the specified key is already registered, its specification is overwritten
by the parameter values.


The definition of a property ( _pDef_ parameter) is a vector made up of five values:


89


_pName(String)_ the name of the property (appears in the property editor). This can be a
wildcard that is resolved via an external resource file (Appendix D).


_pMin(Any)_ lower (inclusive) limit of the value range


_pMax(Any)_ upper (inclusive) limit of the value range or maximum length fir String
properties


_pFmt(String)_ desired special input/output format (syntax and meaning according to
Appendix E.1)


_pType(String)_ the type of property:


_b_ boolean value

_i_ integer

_f_ real number

_s_ string

_ch_ choice list ( _choice list_ )
The type specification is followed by a space and then by the list
of choice values. Each choice value is either a string ID designated
(language-neutral) by a preceding @ character, or by a pair made
up of string ID separated by spaces and language-dependent designation (Appendix D). The choice values are separated by spaces. If
no language-dependent designation is specified for a value, it is read
from language-dependent designation files by means of the string
ID.

_chf_ Choice list via function
The type information is followed by the name of a function which,
when called for the implicit instance, furnishes the list of choice
values in the same form as the explicit information in a property
of type _ch_ .

u Special type ( _user defined_ )
The type information is followed by a space and then by the ID
of the required special editor and (after an additional space) additional information for the special editor, if required.


**Note:** It is not guaranteed that the special editor is implemented in the

OFML runtime environment used at the time.


Besides the actual property definition, the desired position in the property list can be specified
in the _pPos_ parameter. The same specification applies to the setting of the position as of the
_setPropPosOnly()_ function which can be used to individually set the position for an existing
property.


The value range limits, format, and position are optional. Missing information are designated
by a parameter of type _Void_ .


In the type of the implicit instance, a _set_ and a _get_ method can be defined for each property:


**–** _set<Key>(pValue(Any)) →_ _Void_

The function is called if the value of the _<_ Key _>_ property was changed.


90


**Note:** Generally, an assignment of the value to a corresponding instance variable is performed

in this function. Any additional semantics, such as the regeneration of geometry or correspond
ing collision tests, is reserved for the _propsChanged()_ function.


**–** _get<Key>() →_ _Any_

The function furnishes the value for the _<_ Key _>_ property currently stored in the implicit
instance.


**Note:** Generally, the function furnishes the contents of a corresponding instance variable.

A return value of type _Void_ designates a non-specified property, e.g., with optional features.


_• setPropPosOnly(pKey(Symbol), pPos(Int)) →_ _Int_


The function specifies the desired position in the property list for the property with the
specified key. If no property with the specified key is defined for the implicit instance, the
function is without effect and the return value is of type _Void_ . If a property with the specified
key is defined for the implicit instance, the old position information is overwritten. If _pPos_ is
an integer greater than or equal to 0 and the desired position was already assigned to another
property, then this and all the following properties in the position list are moved back by
one position. If _pPos_ is of type _Void_ or features the value _−_ 1, no special position is required
for the property. It is then filed in the property list according to the properties for which a
position was explicitly requested. The new position of the property is the return value or _−_ 1
if no special position is required.


_• setExtPropOffset(pOffset(Int)) →_ _Void_


This function is used to assign an offset to the implicit instance for positions of externally
defined properties, i.e., of properties that are defined for the implicit instance by another
instance besides the implicit instance. The offset indicates the smallest position number that
may be used for externally defined properties.


**Example:** A typical example of externally defined properties are those that are defined for the

representation of product features from the product database for the implicit instance by a global

product data manager instance (Section 9.1).


_• removeProperty(pKey(Symbol)) →_ _Void_


The function removes the property specified by the indicated key from the property list. If
no property with the indicated key is defined for the implicit instance, the function is without
effect.


_• clearProperties() →_ _Void_


The function removes all properties from the property list.


**4.4.2** **Querying Properties**


_• hasProperties() →_ _Int_


The function furnishes 1 if properties are defined for the implicit instance, otherwise it returns
0.


91


_• hasProperty(pKey(Symbol)) →_ _Int_


The function furnishes 1 if a property with indicated key is defined for the implicit instance,
otherwise it returns 0.


_• getPropertyDef(pKey(Symbol)) →_ _Any[]_


The function furnishes the definition of the property with indicated key. The structure of
the returned vector corresponds to the structure of the _pDef_ parameter that was transferred
as property definition to the _setupProperty()_ function. If no property with the indicated key
is defined for the implicit instance, the return value is of type _Void_ .


_• getPropertyPos(pKey(Symbol)) →_ _Int_


The function furnishes the position of the property with indicated key. If no special position
was requested for the property, the return value is _−_ 1. If no property with the indicated key
is defined for the implicit instance, the return value is of type _Void_ .


_• getExtPropOffset() →_ _Int_


This function is used to furnish the for positions of externally defined properties, i.e., of
properties that are defined for the implicit instance by another instance besides the implicit
instance. The offset indicates the smallest position number that may be used for externally
defined properties. This offset should be called by an external instance before the definition of
a property for the implicit instance and should be taken into consideration for the assignment
of explicit positions.


If no other value was assigned using _setExtPropOffset()_, the default return value is equal to
0.


_• getPropertyKeys() →_ _Symbol[]_


The function furnishes a list of the keys of all properties currently defined for the implicit
instance.


At the same time, the properties are sorted in ascending order according to their explicit positions. The properties without explicit position appear at the end of the list in an undefined
order.


_• getProperties() →_ _String_


The function furnishes a description of all properties currently defined for the implicit instance. The format of this description is explained in Appendix E.2.


_• getPropTitle() →_ _String_


The function furnishes a brief description of the instance for use in the header line of property
editors.


**Note:** The two functions described beforehand are used by the property editors to build up a dialog

window.


92


**4.4.3** **Property Values**


_• getPropValue(pKey(Symbol)) →_ _Any_


The function furnishes the value currently stored in the implicit instance for the property
with the indicated key. If no property with the indicated key is defined for the implicit
instance, the return value is of type _Void_


**Note:** The function utilizes the get method of the property (see _setupProperty()_ function). If the

type of the implicit instance does not feature such a method, the value is determined from the hash

table of the dynamic properties (see _getDynamicProps()_ function at the _Base_ interface).


_• setPropValue(pKey(Symbol), pValue(Any)) →_ _Int_


The function assigns the implicit instance a new value for the property with the indicated
key.


If the property is associated with a feature in a product database, the global product data
manager (Chapter 9) evaluates relationships between properties and property values next
(consequently, other properties or their values may change). Next, the _propsChanged()_ function (see below) for performing special processings is called. True is transferred for the
_pDoChecks_ parameter. If the value assignment of the product manager or _propsChanged()_
was rejected, all properties are reset to the state saved at the start of the function and the
_propsChanged()_ function is called again, whereby False is now transferred for the _pDoChecks_
parameter.


The return value of the function is True if the definition of one or several properties changed
or if properties were added or removed.


**Note:** The function uses the set method of the property (see _setupProperty()_ function) for the

actual assignment of the new value to the corresponding instance variable. If the type of the implicit

instance does not feature such a method, the value under the key of the property is written in the

hash table of the dynamic properties (see _getDynamicProps()_ function at the _Base_ interface).


_• propsChanged(pPKeys(Symbol[]), pDoChecks(Int)) →_ _Int_


The function performs special processings and checks after property values were changed.
The properties whose values changed are specified by their keys. The _pDoChecks_ parameter
indicates whether checks need to be performed or whether it is only necessary to respond to
the change of property values, e.g., through geometry matching. The return value is 1 if the
new property values are valid, otherwise it is 0.


**Note:** The function is called at the end of the _setPropValue()_ function. In general, matchings of

the geometry or the material properties of the implicit instance are carried in the function.


_• changedPropList() →_ _Symbol[]_


The function delivers the reference to the list of properties whose values changed during the
processing of the _setPropValue()_ function. The properties are recorded in the list based on
their keys.


93


**Note:** In general, the function is used only by product data managers (Chapter 9) during the
evaluation of knowledge on product data relationships within the _setPropValue()_ function.


The list is emptied at the start of each execution of _setPropValue()_ .


**4.4.4** **Activation Status**


A property can be _active_ or not. For an active property, its value can be changed interactively.
For non-active properties, only their current values are displayed and they cannot be changed
interactively. The initial state following the definition of a property is ”active.”


_• setPropState(pKey(Symbol), pState(Int)) →_ _Void_


The function sets the activation status of the property with the indicated key for the implicit
instance to the transferred value. If no property with the indicated key is defined for the
implicit instance, the function is without effect.


_• getPropState(pKey(Symbol)) →_ _Int_


The function furnishes 1 if the implicit instance features a property with the indicated key
and if it is active. The function furnishes 0 if the implicit instance features a property with
the indicated key and if it is not active. If no property with the indicated key is defined for
the implicit instance, the return value is -1.


**4.4.5** **Information about Properties and Property Values**


_• getPropInfo(pKey(Symbol), pPropValue(Any), pInfoType(Symbol)) →_ _Any_


The function furnishes the information of the requested type for the specified property value
for the implicit instance. The return value is of type _Void_ if the instance does not feature
the specified property or if no information of the requested type is available.


Default implementations of this function delegate the call to the _getPropInfo4Obj()_ method
of the OiProgInfo instance (if available) responsible for the instance, see Chapter 8.


The following standard information types are predefined:


_@Picture_

Name of the graphics file that represents the property value (String)

_@Text_

text description (String, can be text resource)

_@HTML_

URL of the HTML description (String)

#### **4.5 Complex**


The _Complex_ interface describes the necessary functionality of complex objects, i.e., of objects that
are composed of one or several accessible subobjects (children). In principle, this applies to all
types whose entities can be combined, expanded or disassembled at runtime.


94


**4.5.1** **Spatial Model**


On the one hand, the functions of this group serve the more effective access to the spatial dimensions of objects that would otherwise have to be determined by the more time-consuming
_getLocalBounds()_ function of the _Base_ interface. On the other hand, they allow for using dimensions that deviate from the exact geometric dimensions according to _getLocalBounds()_ .


_• getWidth() →_ _Float_


The function furnishes the width of the implicit instance.


_• getHeight() →_ _Float_


The function furnishes the height of the implicit instance.


_• getDepth() →_ _Float_


The function furnishes the depth of the implicit instance.


**4.5.2** **Dynamic Creation and Management of Children**


_• checkAdd((pType(Type), pObj(MObject), pPosRot(Any[2]), pParams(Any)) →_ _Float[3]_


The function checks whether an instance of the indicated type can be attached to the implicit
instance as child and, if positive, furnishes a valid position for the child instance (in the local
coordinate system of the implicit instance). If no instance of the indicated type can be
attached as child or if no open valid position can be determined, the function returns a value
of type _Void_ .


If the _pObj_ argument is not of type _Void_, it specifies an already existing instance that should
be enlisted to locate a position. If the _pPosRot_ argument is not of type _Void_, it specifies a
suggested position and rotation with respect to the local coordinate system of the implicit
instance. The first element of the parameter vector contains the suggested position (Float[3])
and the second element the suggested rotation with respect to the positive Y axis. If the
_pParams_ argument is not of type _Void_, it contains additional parameters for the initialization
function of the type _pType_ .


To check whether an instance of the transferred type should be added, it may be necessary to
generate a temporary instance of the type during the execution of the function, e.g., to be able
to make statements about the child to be generated by using function calls on this instance.
The way in which such a temporary child instance is generated, is controlled by the so-called
_Paste Mode_ which is assigned by means of the _setPasteMode()_ function before _checkAdd()_ is
called by the client. If the instance to be inserted represents an article (see _Article_ interface),
a simple instantiation of the transferred type may sometimes not be sufficient; instead, the
temporary child instance must also accept the configuration of the article to be inserted.
For this purpose, the desired article specification is transferred by the client by calling the
_setTempArticleSpec()_ function before calling _checkAdd()_ of the implicit instance.


So far as the type to be inserted defines planning categories (Appendix H), they can be taken
into consideration during the implementation of _checkAdd()_ functions.


95


**Note:** In general, this function is called by the runtime environment if the user has entered the

command for inserting an object of a selected type in the scene or in a selected object. If the

function furnishes a valid position, the runtime environment generates an instance of the indicated

type in the next step and places it at the determined position. If the new object cannot be inserted

into the selected object, an attempt is made to insert it in its father instance, etc.


_• setPasteMode(pMode(Symbol)) →_ _Void_


The function sets the _Paste_ mode for inserting temporary child entities into the implicit
instance. The following modes are possible:


`@CR` The child instance must be re-generated as instance of the type that was transferred to
the _checkAdd()_ function. This is the default setting.


`@PA` The child instance should be created as a copy of an already existing object whose representation can be found on the clipboard of the application. In this case, the child instance is generated by means of evaluating the clipboard using the global _oiApplPaste()_
function.


_• getPasteMode() →_ _Symbol_


The function furnishes the current _Paste_ mode for inserting temporary child entities into the
implicit instance.


_• setTempArticleSpec(pArticle(Vector[2])) →_ _Void_


The function assigns the article specification to the implicit instance which must be assigned
to the temporary child instance after its creation (see _setXArticleSpec()_ function of the _Article_
interface, Section 4.6). The _pArticle_ parameter contains a vector whose first element lists the
base article number, while the second specifies the variant code of the article.


_• getTempArticleSpec() →_ _Vector[2]_


The function returns the article specification for the temporary child instance that was assigned with the _setTempArticleSpec()_ function.


_• setMethod(pMethod(String)) →_ _Void_


The function sets the method call, including the parameters according to the basic syntax
(Chapter 3), which should be executed after generating and initially positioning a child
instance following an execution of the _checkAdd()_ function for this child instance.


_• getMethod() →_ _String_


The function provides the code according to the basic syntax (Chapter 3), which should be
executed after generating and initially positioning a child instance following an execution of
the _checkAdd()_ function for this child instance. If no method is to be executed, an empty
string is returned.


**Note:** The method call to be executed is provided by the _checkAdd()_ function which is executed

beforehand. It contains actions that go beyond the positioning of the child instance, e.g., rotating

the child instance by a required angle.


96


_• clearMethod() →_ _Void_


After generating a child instance, the function resets a method call to be executed for the
child instance, if necessary. In this case, an empty string is set as method call.


_• addPart(pType(Type), pParams(Any)) →_ _MObject_


The function adds an instance of the specified type as a child to the implicit instance, if
possible. If the _pParams_ argument is not of type _Void_, it contains additional parameters for
the initialization function of the type _pType_ . If no instance of the specified type can be added
as child, the function returns a value of type _Void_ .


**Note:** The function utilizes the _checkAdd()_ function for determining a valid position and upon the

return of such a position after the initial positioning performs the code specified by the _getMethod()_

function, if necessary.


_• checkElPos(pEl(MObject), pOldPos(Float[3])) →_ _Int_


The function checks the validity of the current local position of the transferred child instance.
The function furnishes 1 if the current position is allowed, otherwise it furnishes 0.


**Note:** The function is used primarily for checking the new position of a child instance after a

translation or rotation of the instance. Generally, a collision check is performed. Additional, type
dependent checks are possible, e.g., monitoring for compliance of a specified grid. If necessary, a

correction of the position may be performed before the transformation using the position transferred

in the _pOldPos_ parameter, e.g., a setting to the next grid position.


**4.5.3** **Collision Check**


_• disableChildCD() →_ _Void_


The function deactivates the collision detection for children of the implicit instance which is
performed via _checkChildColl()_ .


_• enableChildCD() →_ _Void_


The function (re-)activates the collision detection for children of the implicit instance which
is performed via _checkChildColl()_ .


_• isEnabledChildCD() →_ _Int_


The function furnishes 1, if the collision detection for the implicit instance is activated,
otherwise it furnishes 0.


_• isValidForCollCheck(pObj(MObject)) →_ _Int_


The function furnishes 1 if the specified (child) instance should be considered during the
collision check, otherwise it furnishes 0.


**Note:** The function is a hook function which is called by the _checkChildColl()_ function. Standard

implementations of this function always deliver 1.


97


_• checkChildColl(pObj(MObject), pExclObj(MObject[])) →_ _MObject_


The function checks whether a collision of the transferred (child) instance with other objects
is present. If the _pExclObj_ argument contains a non-empty set of objects, they are excluded
from the collision check.


The function first checks for collision with the children of the implicit instance. The check
only takes place if the following conditions are met:


**–** _isEnabledChildCD()_ of the implicit instance delivers True


**–** _isValidForCollCheck()_ of the implicit instance delivers True for the transferred (child)
instance


**–** _isEnabledCD()_ of the transferred (child) instance delivers True


The following children are excluded from the collision check:


**–** children for which the _isValidForCollCheck()_ function of the implicit instance delivers
False


**–** children whose _isEnabledCD()_ function delivers False


**–** children that are listed in the _pExclObj_ argument


If _isEnabledCD()_ of the implicit instance delivers True, the function of the father instance of
the same name is called next (if it exists and if its type implements the _Complex_ interface).


The return value is the first located object with which the transferred instance collides or a
value of type _Void_ if no collision was detected or if the collision detection is deactivated.

#### **4.6 Article**


The _Article_ interface includes a set of functions that provide the necessary information about a
planning object from a commercial point of view.


**4.6.1** **Program Access**


_• getProgram() →_ _Symbol_


The function delivers the ID of the program (Appendix I) to which the implicit instance
belongs.


**4.6.2** **Structure of Order Lists**


_• setOrderID(pID(Symbol)) →_ _Void_


The function assigns a unique ID to the implicit article instance that is used in structures of
order lists for assigning an article item of the order list to the instance that represents the
article in planning.


98


The order ID is assigned to the article instance immediately following its generation and
is not changed as long as the article instance exists. If the position of the article in the
planning hierarchy changes (e.g., in grouping actions), the order ID is transferred from the
destroyed instance to the newly generated clone instance in the cut-and-paste operation that
takes place.


_• getOrderID() →_ _Symbol_


The function delivers the unique order ID of the implicit article instance.


**4.6.3** **Product Data**


_• getArticleSpec() →_ _String_


The function delivers the name of the article (base article number) to which the implicit
instance corresponds or a value of type _Void_ if no article specification is available for the
implicit instance.


If the result of the function is a value of type _Void_, no entry is generated for the instance in
the order lists.


_• getXArticleSpec(pType(Symbol)) →_ _String_


The function delivers the specification of the requested type for the article to which the
implicit instance corresponds or a value of type _Void_ if no article specification of the required
type is available for the implicit instance.


The following specification types are predefined:

```
   @Base
```

base article number, designates the model of the article without reference to a concrete
implementation/configuration (corresponds to the return value of _getArticleSpec()_ )

```
   @VarCode
```

variant code, describes the concrete implementation/configuration of the article with
respect to the base article number.

```
   @Final
```

final article number, designates the model of the article and describes its concrete implementation/configuration


**Note:** Usually, the final article number consists of the base article number and the variant code.

However, this depends upon the underlying product data system. If it does not allow for such a

strict definition, variant code and final article number are identical.


_• setArticleSpec(pSpec(String)) →_ _Void_


The function assigns a new base article number to the implicit instance.


**Note:** The function applies only for types whose entities can represent different article (numbers).

Assigning a new article number generally leads to a change of certain properties of the instance and,

if necessary, also to a new geometric representation.


99


_• setXArticleSpec(pType(Symbol), pSpec(String)) →_ _Void_


The function assigns a new article specification of the specified type to the implicit instance.


The possible specification types are described under the _getXArticleSpec()_ function. With a
transfer of an article specification of type @Base, the function behaves like the _setArticle-_
_Spec()_ function above.


**Note:** Assigning a new final article number or a new variant code (specification types @Final or

@VarCode) generally leads to a change of certain properties of the instance and, if necessary, to a

new geometric representation.


_• getArticleParams() →_ _Any_


The function furnishes the parameters of the implicit instance that should be used for determining the article number (see _getArticleSpec()_ function) in addition to the type of the
instance. The return value is a vector with the parameter values or a string that already
contains the parameter values that were converted into the respective storage format. If no
parameters are required for determining the article number, the function furnishes a value of
type _Void_ .


_• getArticlePrice(pLanguage(String), ...) →_ _Any[]_


The function delivers price information for the implicit instance in the specified language.
If an additional optional parameter is given, it specifies the desired currency. However, the
price information does not have to be furnished in this currency by the function (if, for
example, the underlying product database cannot supply prices in this currency). In this
case, the client of the function must perform a conversion into the desired currency by means
of conversion rates.


The return value is a list that contains the individual price components. Every list entry is
a vector consisting of three elements:


1. a description ( _String_ ) that specifies the type or existential reason of the price component,
e.g. the reason for a surcharge.


2. the selling price of the price component ( _Float_ )


3. the purchase price of the price component ( _Float_ )


The first entry represents an exception since it contains the applied currency ( _String_ ) instead
of the prices. The last entry of the list specifies the (accumulated) final price. The optional
entries in between specify the individual price components (base price, extra charges, discounts, etc.). If such a price component contains the designator `"@baseprice"`, then it is
explicitly designated as base price.


**Note:** The explicit designation of the base price component can be used by the respective application

to treat the base price differently for the presentation of order lists.


The function furnishes a value of type _Void_, if no price information is available for the implicit
instance.


_• getArticleText(pLanguage(String), pForm(Symbol)) →_ _String[]_


100


The function furnishes a text description of the desired form in the specified language for the
article that is represented by the implicit instance.


The _pForm_ parameter may take on the following values:


**–** @ `s` short description


**–** @ `l` long description


The return value is a list of strings that contain the individual lines of the description or a
value of type _Void_ if no article description is available for the implicit instance.


**Note:** The article description furnished by this function contains (typically in long form) only

information about the fixed features of the article. A description of the concrete current implemen
tations of the changeable/configurable features of the article is furnished by the _getArticleFeatures()_

function.


_• getArticleFeatures(pLanguage(String)) →_ _Any_


The function furnishes a description in the specified language for the article represented by
the implicit instance, of the current implementation of the product properties that can be
changed/configured for the article.


The return value is a list of two-digit vectors whose first element ( _String_ ) labels the feature,
while the second element contains the current value (as character string) of the feature. If
the _pLanguage_ parameter contains a value of type _Void_, language-independent designators
are furnished for feature and value. The function furnishes a value of type _Void_, if no feature
description is available for the implicit instance.


Calls of the function immediately following each other with different parameters for the
language furnish lists of identical length and contain the features in the same order. If no
language-independent designator is available for a value with a language parameter of type
_Void_ for a feature, the corresponding entry in the return list is not a vector, but a value of
type _Void_ .


**Note:** The language-independent designators (codes) furnished by the function with a language

parameter of type _Void_ are generally used by export routines of the application to generate a complete

description of an article that can be exported to an external PPS, e.g., for order processing.


**4.6.4** **Consistency Check**


_• checkConsistency() →_ _Int_


The function checks the consistency and completeness of the planning element. If necessary,
corrections or additions are performed or error messages are generated.


If the higher-order instance that initiated the consistency check of the implicit instance,
created an _error log_, the error messages should be written into this error log; otherwise
they can be issued directly to the user by means of _oiOutput()_ . The error log to be used
must be called using the _getErrorLog()_ function of the global planning instance ( _OiPlanning_
type, Section 8.1). The data structure of the error log specified for _checkConsistency()_ is a


101


hash table, in which the corresponding messages for each article instance are entered as a
code under their order ID (see _getOrderID()_ function). The value for this code is a list of
three-digit vectors:


1. the error message (String)


2. the name of the object that reported the error (String)


3. the name of the method by which the error was detected (String)


**Note:** If required, detailed reports for error analyses can be generated with the last two entries.


102


## **Chapter 5**

# **Predefined Rule Reasons**

This chapter contains a description of predefined rule reasons. The properties of the predefined
rule reasons are:


_•_ They correspond to the fundamental basic interactions in their entirety, such as selecting,
moving, copying, inserting, etc.


_•_ They are called automatically by the runtime environment, if a corresponding action occurred
(implicit call).


_•_ They can also be called explicitly.


In addition, there may be user-defined rule reasons. The properties of user-defined rule reasons
are:


_•_ They are always called explicitly.


_•_ The definition of user-defined rule reasons does not violate the compatibility of OFML data.

#### **5.1 Element Rules**


**CREATE** ~~**E**~~ **LEMENT**


The rules of the _CREATE_ ~~_E_~~ _LEMENT_ reason are called for an _O_ object _before_ an _E_ object of _TE_
type is generated as element of _O_ . The corresponding interaction is the generation of objects in
general, e.g., by inserting an object from the clipboard. The parameter of the rules is the _TE_ type.
Rules of this reason can be used to control the aggregation dynamically and dependent upon the
state of the _O_ object. Reasons for the failure of such rules can be:


_•_ Entities of the _TE_ type cannot be aggregated in _O_ .


**Example:** Tabletop lamps cannot be planned in a carcass cabinet.


103


_•_ Entities of the _TE_ type can be aggregated only in _O_ if certain (geometric) rules are adhered
to, e.g., a linear dependency between the width of _O_ and the width of the instance of _TE_ . If
such a condition is violated, the rule will necessarily fail.


**Example:** In general, only shelves with the corresponding width can be planned for a carcass

cabinet of a certain width.


_•_ On principle, entities of the _TE_ type can be aggregated in _O_ ; however, an insertion would
create a conflict with already existing children.


**Example:** No more shelves can be planned for a carcass cabinet that already contains a shelf at

every grid position.


In these cases further processing of the list of rules is interrupted, and no instance of _TE_ as element
of _O_ is generated.


**NEW** ~~**E**~~ **LEMENT**


The rules of the _NEW_ ~~_E_~~ _LEMENT_ reason are called for an _O_ object _before_ an _E_ child of _O_ is
accepted in the list of elements of _O_ . According to Chapter 2, an element is a special child in
so far as elements from outside are accessible by _O_, i.e., they can be generated or deleted. The
corresponding interaction is the generation of objects in general, e.g., by inserting an object from
the clipboard. The rules of the _NEW_ _ELEMENT_ reason are called after the rules of the _CRE-_
_ATE_ ~~_E_~~ _LEMENT_ reason have been called. The generation of an instance cannot be prevented by a
_NEW_ _ELEMENT_ rule. Instead, the _NEW_ ~~_E_~~ _LEMENT_ reason offers expanded possibilities for the
derivation of functionality within the corresponding rules compared to the _CREATE_ _ELEMENT_
reason. Since an actual instance is transferred as a parameter instead of a type, queries can be
implemented that go beyond comparing types, e.g., the query of type compatibility to abstract
super types, the query of geometric parameters, and other (type-dependent) queries.


The rule parameter is an already existing child of _O_ that is to be incorporated as element of _O_ . If
a rule fails, _E_ is not incorporated.


**Example:** The automatic generation of components such as mounting rails that are required for fastening

add-on parts, can be implemented via _NEW_ ~~_E_~~ _LEMENT_ or _CREATE_ ~~_E_~~ _LEMENT_ .


**REMOVE** ~~**E**~~ **LEMENT**


The rules of the REMOVE ~~E~~ LEMENT reason are called for an _O_ object _before_ an _E_ element is
deleted. The corresponding interaction is the removal of objects in general, e.g., by operations
such as cutting or deleting.


The rule parameter is a reference to the already existing element _E_ of _O_ that is to be deleted. The
rule can fail if other elements in _O_ depend upon _E_ . If a rule fails, _E_ is not deleted.


**Example:** A mattress box as an element of a bed cannot be deleted as long as it contains bed frame,

mattresses, head sections, back panels, etc.


104


#### **5.2 Selection Rules**

**PICK**


The rules of the PICK reason are called _after_ an object was chosen or selected. The corresponding
interaction is the selection of an object in general, e.g., in a direct-manipulative way (2D/3D
interaction) or via a graphical user interface. The rule parameter is of type _Float[3]_ und indicates
the local coordinates at which the object was selected.


PICK rules can be defined to generate a special feedback, e.g., the change in material characteristics
or the geometry. Such a feedback is independent from the general feedback which is provided by
the OFML runtime environment. In addition, random actions can be triggered by a PICK rule,
e.g., the display of object properties within the graphical user interface or the change of the global
state.


**UNPICK**


The rules of the UNPICK reason are called _after_ an object was deselected, e.g., by selecting another
object. The rule parameter is not defined.


UNPICK rules are generally a reversal of the corresponding PICK rules. For example, the feedback
generated by the PICK rule can be reset.

#### **5.3 Move Rules**


**TRANSLATE**


The rules of the _TRANSLATE_ reason are called _after_ an _O_ object was moved. The corresponding interaction is the translatory move of objects via direct or indirect manipulation. The rule
parameter is the local position of _O_ before the move.


The translatory move of an object can be controlled at random through the definition of the
_TRANSLATE_ rules, e.g.:


_•_ homogenous or inhomogenous rasterization,


_•_ limitation to a range,


_•_ initiation of a collision detection with corresponding correction of the position,


_•_ snapping to objects or positions.


The different possibilities can be combined within a single rule, for example, to enable multidimensional moves. In addition, the father can be called within the rule and the rule functionality
can be delegated to it.


105


Moreover, _O_ can adapt itself to the new position at random. This can refer to local properties such
as geometry or the properties of children, e.g., the position of a child relative to its _O_ father. The
vector that results from the new and old position can be used to derive directional information
and, if required, applied.


**Example:** The shelves of a carcass cabinet can be moved in a grid of 32 mm, starting at a height of 80

mm. The maximum fitting height results from the inner height of the carcass cabinet minus 80 mm.


**ROTATE**


The rules of the _ROTATE_ reason are called _after_ an _O_ object was rotated. The corresponding
interaction is the rotary move of objects via direct or indirect manipulation. The rule parameter
is the local orientation of _O_ along the employed rotary axis before the rotation.


The rotary move of an object can be controlled at random through the definition of the _ROTATE_
rules, e.g.:


_•_ homogenous or inhomogenous rasterization,


_•_ limitation to a range,


_•_ initiation of a collision detection and corresponding correction of the orientation,


_•_ snapping to objects or positions.


The different possibilities can be combined within a single rule, for example, to enable a rasterized
rotation within a certain range. In addition, the father can be called within the rule and the rule
functionality can be delegated to it.


Analogous to the _TRANSLATE_ rule, an object can adapt itself at random to the new orientation.


**Example:** The door of a carcass cabinet can be opened at an angle from 0 to 90 degrees. At angles of

10 degrees or less, a snapping to 0 degrees is performed automatically. At angles of 80 degrees or more,

a snapping to 90 degrees is performed automatically. The snapping behavior can be used to simulate the

latching at the end positions.


**SPATIAL** ~~**M**~~ **ODELING**


The rules of the SPATIAL ~~M~~ ODELING reason are called _after_ an _O_ object was moved indirectly,
i.e., shifted or rotated. An indirect move takes place if an ancestor (father, grandfather, etc.) was
panned or rotated. A match of _O_ can take place again. The rule parameter is undefined.


**Example:** The door handles of a construction kit within a shelf plan are placed dependent upon the

fitting height of the construction kit. At a height of less than 1.40 m, it is placed at the top end of the

door. Otherwise, it is placed at the bottom end. This adjustment can be implemented automatically by

using a _SPATIAL_ ~~_M_~~ _ODELING_ rule.


106


#### **5.4 Persistence Rules**

The persistence rules serve for the conversion of the instance variables from a representation that
is used at runtime to a persistent representation and vice versa. This includes, above all, the
conversion of object references to values such as _String_ or _Int_ that can be stored and restored.
Furthermore, especially the _*_ ~~_E_~~ _VAL_ rules can be used for adapting stored scenes by initializing
instance variables accordingly that did not exist so far.


The definition of persistence rules is required only in exceptional cases.


The rule parameter of persistence rules is undefined.


**START** ~~**D**~~ **UMP**


The rules of the START ~~D~~ UMP reason are called _before_ the generation of a persistent representation of the _O_ object, e.g., within the scope of scenes/object saving or a clipboard operation (e.g.,
cutting, copying). After processing the rules, the instance variables must be available in a storable
representation.


**FINISH** ~~**D**~~ **UMP**


The rules of the FINISH DUMP reason are called _after_ the generation of a persistent representation
of the _O_ object and its children. After processing the rules, the instance variables must be available
again in the representation that is required for the normal operating mode.


**START** ~~**E**~~ **VAL**


The rules of the START ~~E~~ VAL reason are called _before_ the processing of a persistent representation
of the _O_ object, e.g., within the scope of loading a scenes/object saving or a clipboard operation
(e.g., inserting). The call is performed immediately following the generation of the _O_ object and
before the assignment of attributes, children, etc.


**FINISH** ~~**E**~~ **VAL**


The rules of the FINISH ~~E~~ VAL reason are called _after_ the generation of a persistent representation
of the _O_ object and its children. After processing the rules, the instance variables must be available
in the representation that is required for the normal operating mode.


**Example:** With a certain roll-container type, the new version can be used to optionally configure an

espagnolette. Consequently, this type defines an additional instance variable in the new version that

describes by means of a symbol whether the espagnolette is desired or not. A _FINISH_ ~~_E_~~ _VAL_ rule can be

used to ensure that saves of the old version can be post-initialized in this connection.


107


#### **5.5 Other Rules**

**SENSOR**


The rules of the _SENSOR_ reason are called if any _M_ object was moved directly. The rule parameter
is a reference to _M_ .


Sensory objects, i.e., objects with at least a _SENSOR_ rule, can autonomously respond to changes
of the environment.


**Example:** The door of a room opens automatically if an object is located in a circumcircle of 5 m.


**TIMER**


The rules of the _TIMER_ reason are called if the time interval defined in the respective rule signature
expired at least once. The number of passed intervals (typically 1 for time intervals that are not
too small) is passed on to the rule(s) as parameter. An instance (or a type) with at least a _TIMER_
rule is time-dependent. By generating and removing time-dependent children, a dynamic indirect
time dependence of an object can be implemented.


**Example:** An instance of type clock shows the current time. A _TIMER_ rule is used for updating.


**INTERACTOR**


The rules of the _INTERACTOR_ reason are called for the father of the interactor if an attempt to
select an interactor is made. They typically serve to activate the interactor (Section F.1).


The selected interactor is transferred as reference as the rule parameter.


**Example:** Designs can be mounted to an organizational wall at different positions. If interactors are

defined for these positions, the user can interactively select the desired mounting point.


108


## **Chapter 6**

# **Global functions**

#### **6.1 Formatted Output**

Some of the functions described below use special character strings to control the formatting. The
format character string contains two types of components: regular characters that are accepted
in the output without change, and formatting sequences that control the conversion of one of the
following arguments in each case. Every formatting sequence begins with the character `%` and ends
with a formatting character. The following optional characters can be used between the character
`%` and the conversion character in the sequence indicated here:


_•_ Control characters (in random order) that modify the conversion:


`-` The converted argument is left-aligned.


`+` The number is always indicated with a sign.


_space_ If the first character is not a sign, a space is used as prefix.


`0` Numbers are filled with zeros up to the width of the field.


`\#` Generates an alternative form of the conversion, dependent upon the formatting
character (see below). For `o`, the first character is a zero. For `x` or `X`, `0x` or `0X` are
prefixed for a result different than zero. For `e`, `E`, `f`, `g`, and `G`, the output always
contains a decimal point; for `g` and `G`, zeros at the end are not suppressed.


_•_ A number that specifies the minimum field width. The number of characters output is at
least equal to the number of characters indicated, and more if required (i.e., characters will
never be cut off). If the converted argument is shorter, it is filled up to the width of the field.
Alignment and fill characters are dependent upon the formatting and control characters.


_•_ A period that separates the field width and the accuracy.


_•_ A number with the following meaning: For `e`, `E` or `f` the number of places behind the decimal
point. For `g` or `G` the number of significant digits. For integer values, the minimum number
of digits to be output. In the remaining cases, the number indicates the maximum number
of characters that are output by a character string.


109


In each case, `*` can be indicated as field width or accuracy, so that the value is determined by the
next or the next two arguments that must be of type `Int` .


Table 6.1 explains the formatting characters. A character that follows `%` and is not a formatting
character, represents an error.

|Character|Argument type|Formatting|
|---|---|---|
|`d`,` i`<br>`o`<br>`x`,` X`<br>`u`<br>`c`<br>`s`<br>`f`<br>`e`,` E`<br>`g`,` G`<br>`%`|`Int`<br>`Int`<br>`Int`<br>`Int`<br>`Int`<br>`String`<br>`Float`<br>`Float`<br>`Float`<br>-|decimal with sign<br>octal without sign, leading zero optional<br>hexadecimal without sign,` 0x`,` 0X` optional, for` abcdef`<br>for` x` or` ABCDEF` for` X`<br>decimal without sign<br>single character (Section 3.2.1)<br>character string<br>decimal as` [-]`_mmm.ddd_, accuracy determines the number<br>of_ d_, default: 6, no decimal point for 0<br>decimal as` [-]`_mmm.ddd_`e`_±xx_ or` [-]`_mmm.ddd_`E`_±xx_<br>accuracy determines the number of_ d_, default: 6,<br>no decimal point for 0<br>corresponds to` %e`,` %E` if exponent is smaller than_ −_4 or not<br>smaller than accuracy, otherwise` %f`. zero and decimal point<br>are not issued at the end.<br>issues` %`|



Table 6.1: Formatting Character

#### **6.2 oiApplPaste()**


_• oiApplPaste(pFather(MObject), pName(Symbol)) →_ _Int_


The function evaluates the clipboard of the application and generates a new object as child
of _pFather_ . The local name of the new object is specified by _pName_ . If an object with the
resulting global name already exists or if the clipboard of the application is empty, a runtime
error occurs. If _NULL_ is set for _pName_, a valid name is automatically selected. The return
value of the function is 1 if an object could be generated, otherwise 0.


The state of the clipboard does not change.


**Note:** The clipboard of the application is implemented by the runtime environment and does not

have any reference to the global OFML clipboard which can be manipulated or evaluated using the

_oiCopy()_, _oiCut()_, and _oiPaste()_ function.

#### **6.3 oiClone()**


_• oiClone(pSrc(MObject), pDest(String)) →_ _MObject_


110


The function generates an identical copy of the _pSrc_ object under the global name _pDest_ and
returns the corresponding object reference. If an immediately preceding object under the
name _pDest_ exists, it causes a runtime error.


The state of the OFML clipboard does not change.

#### **6.4 oiCollision()**


_• oiCollision(pObject1(MObject), pObject2(MObject)) →_ _Int_


The function checks the collision between two objects _pObject1_ and _pObject2_ . The polygons
of the geometric basic primitives are the atomic element of the collision check. In the case
of the parametric primitives _OiRotation_, _OiSweep_ and _OiSurface_, these polygons result from
the definition coordinates or areas. That is, the actual mapping to a piece by piece linear
approximation is not taken into account.


In the case of a collision, 1 is returned, otherwise 0.


The function always delivers 1 if _pObject1_ is an ancestor (father, grandfather, etc.) or a
successor (child, grandchild, etc.) of _pObject2_ and vice versa.

#### **6.5 oiCopy()**


_• oiCopy(pObject(MObject)) →_ _Void_


The function writes an adequate description of _pObject_ to the global OFML clipboard.


The existing state of the clipboard is lost.


Since the OFML clipboard is a global data structure, corresponding operations must follow
each other directly. Otherwise, the correctness of the operations cannot be guaranteed.

#### **6.6 oiCut()**


_• oiCut(pObject(MObject)) →_ _Void_


The function writes an adequate description of _pObject_ to the global OFML clipboard and
then deletes object _pObject_ .


The existing state of the clipboard is lost.


Since the OFML clipboard is a global data structure, corresponding operations must follow
each other directly. Otherwise, the correctness of the operations cannot be guaranteed.

#### **6.7 oiDialog()**


_• oiDialog(pDialog(Symbol), pIcon(Symbol), pMessage(String)) →_ _Symbol_


111


This function causes the reaction of the user to a modal dialog that is generated by the
OFMLruntime environment.


The _pDialog_ parameter specifies the dialog through one of the following symbols. The possible
return values are listed in parentheses.


**–** _@OK_    - Confirmation ( _@OK_ ).


**–** _@OK_ ~~_C_~~ _AN_    - Confirmation or Cancel ( _@OK_, _@CANCEL_ ).


**–** _@ABT_ _IGN_    - Abort or Ignore ( _@ABORT_, _@IGNORE_ ).


**–** _@YES_ _NO_ ~~_C_~~ _AN_    - Yes or No or Cancel ( _@YES_, _@NO_, _@CANCEL_ ).


**–** _@YES_ _NO_    - Yes or No ( _@YES_, _@NO_ ).


If no valid value is transferred for _pDialog_, no dialog is started and the return value is
_@INVALID_ ~~_D_~~ _IALOG_ .


In addition, the _pIcon_ parameter specifies the visual representation of the dialog. It can be
executed through a corresponding icon and is always binding, independent of the value of
_pDialog_ . The value range of _pIcon_ is specified as follows:


**–** _@NONE_    - No display of a special character.


**–** _@STOP_    - Display of a stop character.


**–** _@QUESTION_    - Display of a question mark.


**–** _@WARNING_    - Display of an exclamation mark.


**–** _@INFO_    - Display of an information sign (a small _i_ in a circle).


If no valid value is transferred for _pIcon_, no dialog is started and the return value is _@IN-_
_VALID_ _ICON_ .


The _pMessage_ parameter specifies the message to be output. If the first character of the
_pMessage_ string is a `@`, the string is considered a reference that is triggered by an access to
an external database (Appendix D).


_pMessage_ must be either a valid string according to the basic syntax (Chapter 3) or a vector.
In the first case, umlauts are not permitted. Specifying a vector is used for the formatted
output. In this case, the first element is the format character string (Section 6.1), the
remaining elements are the arguments to be formatted. If the first element of the vector
starts with `@`, the format character string is read from the external database, as indicated
above. If no valid value is transferred for _pMessage_, an empty character string is output in
the dialog.


The return value is a symbol that describes the selected answer in accordance with the
aforementioned alternatives.

#### **6.8 oiDump2String()**


_• oiDump2String(pObj(MObject)) →_ _String_


The function delivers the (implementation-dependent) dump representation of the transferred
instance.


112


It can be used together with the _oiReplace()_ function to store and restore object states which
may be used, for example, in problem cases for implementing undo-capable operations.

#### **6.9 oiExists()**


_• oiExists(pName(String)) →_ _Int_


The function checks the existence of the object whose absolute name is transferred as string
in the _pName_ parameter. If the object exists, the return value is 1, otherwise 0. The existence
check may be necessary since accessing a non-existing object will cause a runtime error.

#### **6.10 oiGetDistance()**


_• oiGetDistance(pPosition(Float[3]), pDirection(Float[3])) →_ _Float_


The function determines the first point of intersection of a beam whose origin lies in the
world coordinate point _pPosition_ and runs alongside the normed vector _pDirection_, with the
objects of the scene. The return value is the distance along the beam to the first intersection
or _−_ 1 if no intersection is found.

#### **6.11 oiGetNearestObject()**


_• oiGetNearestObject(pPosition(Float[3]), pDirection(Float[3])) →_ _MObject_


The function determines the first encountered object while tracing a beam whose origin lies
in the world coordinate point _pPosition_ and runs alongside the normed vector _pDirection_ .
The return value is a reference to the first encountered object or _NULL_ if no object was
found.

#### **6.12 oiGetRoots()**


_• oiGetRoots() →_ _MObject[]_


The function determines the root objects available in the scene.

#### **6.13 oiGetStringResource()**


_• oiGetStringResource(pStr(String), pLanguage(String), ...) →_ _String_


The function delivers the text stored in an external resource file for the transferred text
resource in the specified language or the text resource if no text could be found for the
resource or an invalid value was transferred for the language.


If an additional optional parameter is given, it specifies an instance. The text is searched in
the name space of this instance if the text resource is not fully qualified (see Appendix D).


113


#### **6.14 oiLink()**


_• oiLink(pURL(String)) →_ _Void_


The function loads the file specified by the string _pURL_ . The current scene can be replaced
by a new scene or another document in the result.

#### **6.15 oiOutput()**


_• oiOutput(pLevel(Symbol), pMessage(String)) →_ _Void_


This function causes the output of a text message through the OFML runtime environment.
The output should be implemented through a modal dialog. The _pLevel_ symbol describes
the category of the output as follows:


**–** _@MESSAGE_    - Output of a message.


**–** _@WARNING_    - Output of a warning.


**–** _@ERROR_    - Output of an error message.


**–** _@FATAL_    - Output of an error message. After quitting the modal dialog, the runtime
environment must terminate.


If the first character of the _pMessage_ string is a `@`, the string is considered a reference that is
triggered by an access to an external database (Appendix D).


_pMessage_ must be either a valid string according to the basic syntax (Chapter 3) or a vector.
In the first case, umlauts are not permitted. Specifying a vector is used for the formatted
output. In this case, the first element is the format character string (Section 6.1), the
remaining elements are the arguments to be formatted. If the first element of the vector
starts with `@`, the format character string is read from the external database, as indicated
above.


If `"::ofml::app::@none"` is transferred as the message, the application does not output a
message.


**Note:** It can be used, for example, to indicate an ”error condition” via `oiOutput(@ERROR, "::ofml::app::@none")`
of the application for which the OFML already performed a dialog ( _oiDialog()_ function) (e.g., a Can
cel dialog during _checkAdd()_, see _Complex_ interface), and no additional message is desired.

#### **6.16 oiPaste()**


_• oiPaste(pFather(MObject), pName(Symbol)) →_ _MObject_


The function evaluates the global OFML clipboard and generates a new object as child of
_pFather_ . The local name of the new object is specified by _pName_ . If an object with the
resulting global name already exists, a runtime error occurs. If _NULL_ is set for _pName_, a


114


valid name is automatically selected. The return value of the function is a reference to the
created object.


The state of the clipboard does not change.


Since the OFML clipboard is a global data structure, corresponding operations must follow
each other directly. Otherwise, the correctness of the operations cannot be guaranteed.

#### **6.17 oiReplace()**


_• oiReplace(pObj(MObject), pDump(String)) →_ _Void_


The function replaces the transferred instance by an object whose dump representation is
contained in the transferred buffer.


An (implementation-dependent) dump representation can be created with the _oiDump2String()_
function.

#### **6.18 oiSetCheckString()**


_• oiSetCheckString(pString(String) →_ _Void_


The function sets a string that must be verified by the respective OFML runtime environment.
This string, which is usually set in persistent OFML scene representations, can be used for
checking the consistency or validity of a scene representation. An incorrect string in this sense
must result in the cancellation of the read operation of the persistent scene representation.

#### **6.19 oiTable()**


_• oiTable(pRequest(Symbol), pArgs(List)) →_ _List_


The _oiTable_ function implements the read access to data from an external relational database
(Appendix D).


The desired table operation is specified via _pRequest_ parameter and the corresponding arguments via _pArgs_ . The following listing shows the possible operations and corresponding
arguments:

```
   @openTbl List of TableEntry
   @closeTbl List of TableID
   @readTE List of TableEntry

```

A _TableEntry_ is transferred as `[tableID, attributeList]` vector, where _tableID_ is indicated as string and _attributeList_ as a list of _TableAttributen_ .


A _TableAttribute_ is transferred as `[name, isPrimKey, isKey, type, value, format]` vector, where _name_ is indicated as string, _isPrimKey_ and _isKey_ as (boolean) Int, _type_ as symbol,
_value_ as object according to _type_ and _format_ as string.


The following attribute types and corresponding format strings are defined:


115


```
Int: type = @i, format = maximum number of places
Float: type = @f, format = total number of places.number of places
                           behind decimal point
String: type = @s, format = maximum length

```

A TableID string consists of three components separated by spaces:


**–** the designator for the type of database which is always `"FTXT"` (text file with fixed field
length) in OFML,


**–** the localization path for the database to be used, and


**–** the actual name of the concerned table.


A table must be opened before the first access. The `@openTbl` operation opens a table for
reading where its structure is defined via the list of attributes. The _isPrimKey_ and _isKey_
attributes of _TableAttribute_ are evaluated only during the table definition in `@openTbl` .


The `@closeTbl` operation requests a list of TableIDs of the tables to be closed as parameter.
All system resources used for the management of these tables are released; afterwards, an
access is no longer possible.


The `@readTbl` operation is used for reading table rows. The TableEntries transferred in the
list specify which rows should be read. `@readTbl` is the only operation where the transferred
TableEntries do not have to feature a complete row description according to the table definition. Rather, only those TableAttributes must be given that should serve as key for the
access. All rows whose values in the specified columns correspond to the specified values in the
transferred TableAttributes are delivered for a transferred TableEntry. Thus, the `@readTbl`
operation presents a simple, table-specific search function. Only if the TableEntry contains
a TableAttribute marked as primary key during the table definition can a unique result be
expected. If several TableEntry objects are transferred to `@readTbl`, the corresponding query
is performed for each object and the results are linked in the returned list.


116


## **Chapter 7**

# **Geometric types**

This chapter describes the hierarchy of the geometrically oriented types. A geometric instance
can be viewed directly through its geometry and, if required, through its children. The entities of
geometric types are generally located at the lowest level in hierarchical product models.

#### **7.1 OiGeometry**


**Description**


_•_ The abstract type _OiGeometry_ is the base type for the geometrically-oriented types described
below. _OiGeometry_ may not be instantiated directly. The derivation of application-specific
types of _OiGeometry_ is allowed. An implementation of an application-specific derived type
is carried out here through parameterization and aggregation of one or several _OiGeometry_    compatible entities.


The entities of the _OiGeometry_ type can feature a child with the local name _geo_ for the
implementation of the geometry. This name may not be assigned elsewhere. In addition, the
potential existence of _geo_ should be observed with iterations on the list of children.


_•_ **Interface(s):** Base, material


**Initialization**


_• OiGeometry(pFather(MObject), pName(Symbol))_


The function initializes an indirect instance of _OiGeometry_ type. Initially, the selection
option is deactivated. The initial material category is _@ANY_ . In the normal case, it must be
changed accordingly via _setMatCat()_ . The initial alignment is not defined uniformly and is
determined by the respective primitive.


117


**Methods**


_• setMatCat(pCat(Symbol)) →_ _Void_


The function overwrites the initial material category _@ANY_ or a category previously set with
the value of _pCat_ .


_• setAlignment(pAlignment(Symbol[3])) →_ _Void_


The function allows for the alignment of the geometry with respect to the local axes. The
following symbols for element of _pAlignment_ are supported (in each case with respect to one
of the three axes):


**–** _@C_    - The origin of the object is located in the middle of the local delimiting volume.


**–** _@I_    - The origin of the object is located in the minimum of the local delimiting volume.


**–** _@A_    - The origin of the object is located in the maximum of the local delimiting volume.


A subsequent change of the geometry does not lead to adaptation in accordance with the
specified alignment. Children that may exist can lead to unexpected results when the alignment is set.

#### **7.2 OiBlock**


h


|Col1|Col2|Col3|
|---|---|---|
||||
|||d|
||||



z



w



Figure 7.1: The geometric type _OiBlock_


**Description**


_• OiBlock_ represents an orthogonal quboid that begins in the origin of the local coordinate
system and expands accordingly along the positive axes of the local coordinate system. The
dimensions of the quboid can be changed after its generation.


_•_ **Super type:** _OiGeometry_


118


**Initialization**


_• OiBlock(pFather(MObject), pName(Symbol), pDimensions(Float[3]))_


The function initializes an instance of the _OiBlock_ type. The initial dimensions of the quboid
are indicated by a vector of three positive numbers.


**Methods**


_• setDimensions(pDimensions(Float[3])) →_ _Void_


The function sets the dimensions of the quboid. _pDimensions_ must be a vector of three
positive numbers.


If required, an adaptation of the alignment must be performed afterwards ( _setAlignment()_ ).


_• getDimensions() →_ _Float[3]_


The function delivers the current dimensions of the quboid.

#### **7.3 OiCylinder**


r



z





Figure 7.2: The Geometric Type _OiCylinder_


**Description**


_• OiCylinder_ represents a closed homogenous cylinder that begins in the origin of the local
coordinate system and expands centered along the positive y-axis of the local coordinate
system. The dimensions of the cylinder can be changed after its generation.


_•_ **Super type:** _OiGeometry_


119


**Initialization**


_• OiCylinder(pFather(MObject), pName(Symbol), pLength(Float), pRadius(Float))_


The function initializes an instance of the _OiCylinder_ type. The initial dimensions of the
cylinder are indicated by the parameters length and radius. Only positive numbers are
allowed.


**Methods**


_• setLength(pLength(Float)) →_ _Void_


The function sets the length of the cylinder. _pLength_ must be a positive number.


If required, an adaptation of the alignment must be performed afterwards ( _setAlignment()_ ).


_• getLength() →_ _Float_


The function delivers the current length of the cylinder.


_• setRadius(pRadius(Float)) →_ _Void_


The function sets the radius of the cylinder. _pRadius_ must be a positive number.


If required, an adaptation of the alignment must be performed afterwards ( _setAlignment()_ ).


_• getRadius() →_ _Float_


The function delivers the current radius of the cylinder.

#### **7.4 OiEllipsoid**









z





Figure 7.3: Der geometric type _OiElliposid_


120


**Description**


_• OiEllipsoid_ represents a homogenous ellipsoid whose center is located in the origin of the local
coordinate system and expands accordingly to all six sides of the local coordinate system.
The dimensions of the ellipsoid can be changed after its generation.


_•_ **Super type:** _OiGeometry_


**Initialization**


_• OiEllipsoid(pFather(MObject), pName(Symbol), pDimensions(Float[3]))_


The function initializes an instance of the _OiEllipsoid_ type. The initial dimensions of the
ellipsoid are indicated by a vector of three positive numbers.


**Methods**


_• setDimensions(pDimensions(Float[3])) →_ _Void_


The function sets the dimensions of the ellipsoid. _pDimensions_ must be a vector of three
positive numbers.


If required, an adaptation of the alignment must be performed afterwards ( _setAlignment()_ ).


_• getDimensions() →_ _Float[3]_


The function delivers the current dimensions of the ellipsoid.

#### **7.5 OiFrame**


th


h





d



w


|Col1|Col2|Col3|Col4|Col5|
|---|---|---|---|---|
||||||
||||||
||||||
||||||



z





Figure 7.4: Der geometric type _OiFrame_


121


**Description**


_• OiFrame_ represents a frame that begins in the origin of the local coordinate system and
expands accordingly along the positive axes of the local coordinate system. An orthogonal
volume is subtracted from the body in the local x-y plane. The thickness of the frame in x
and y direction is identical. The following must always apply to the dimensions _w_, _h_ in x
and y direction and the x/y thickness _th_ : _w_, _h >_ 2 _∗_ _th_ . The dimensions of the frame can be
changed after its generation.


_•_ **Super type:** _OiGeometry_


**Initialization**


_• OiFrame(pFather(MObject), pName(Symbol), pDimensions(Float[3]), pThickness(Float))_


The function initializes an instance of the _OiFrame_ type. The initial outer dimensions of the
frame are indicated by a vector of three positive numbers. The initial thickness of the frame
in the local x and y direction is indicated by a positive number.


**Methods**


_• setDimensions(pDimensions(Float[3])) →_ _Void_


The function sets the outer dimensions of the frame. _pDimensions_ must be a vector of three
positive numbers.


If required, an adaptation of the alignment must be performed afterwards ( _setAlignment()_ ).


_• getDimensions() →_ _Float[3]_


The function delivers the current outer dimensions of the frame.


_• setThickness(pThickness(Float)) →_ _Void_


The function sets the frame thickness in the local x and y direction. _pThickness_ must be a
positive number.


_• getThickness() →_ _Float_


The function delivers the actual frame thickness in the local x and y direction.

#### **7.6 OiHole**


**Description**


_• OiHole_ implements circular or rectangular openings in circular or rectangular areas. This
allows for simulating boolean operations, especially the subtraction in special cases. However,
no actual subtraction in the sense of a boolean operation takes place. The real purpose of
_OiHole_ consists of generating the areas for the combination of circular outlines rectangular
hole and rectangular outline circular hole. _OiHole_ does not implement outside areas along


122


**Loch**



z





@RECTANGLE


_**@CIRCLE**_



**Außenlinie**


_**@RECTANGLE**_

@CIRCLE



Figure 7.5: The geometric type _OiHole_


the outline in the local z direction. _OiHole_ entities begin in the origin of the local coordinate
system and expand according to the outer dimensions along the positive x-, y- and z-axis.


_•_ **Super type:** _OiGeometry_


**Initialization**


_• OiHole(pFather(MObject), pName(Symbol), pOMode(Symbol), pODim(Float[3]), pBack(Int),_
_pHMode(Symbol), pHDim(Float[3]), pHOffset(Float[2])_


The function initializes an instance of the _OiHole_ type. The following specific parameters
must be supplied:


**–** The _pOMode_ parameter indicates the mode of the outline. Permissible implementations
for _pOMode_ are:


_∗_ _@RECTANGLE_      - The outline corresponds to a rectangle.
_∗_ _@CIRCLE_      - The outline corresponds to a circle.


**–** The _pODim_ parameter determines the outer dimensions of the body, consisting of width
_w_, height _h_ and depth _d_ . All dimensions must be positive numbers. In the case of a
circular outline, the width also determines the height.


**–** The _pBack_ parameter indicates whether the outer back plane is generated ( _pBack ==_
_1_ ) or not ( _pBack == 0_ ).


**–** The _pHMode_ parameter indicates the mode of the hole. Permissible implementations
for _pHMode_ are:


_∗_ _@RECTANGLE_      - A rectangular hole is generated.


123


_∗_ _@CIRCLE_      - A circular hole is generated.


**–** The _pHDim_ determines the dimensions of the hole consisting of width _wh_, height _hh_
and depth _dh_ . All dimensions must be positive numbers. In the case of a circular hole,
the width also determines the height. The hole width _wh_ must be smaller than total
width _w_ . The hole height _hh_ must be smaller than the total height _h_ . The hole depth _dh_
may not be larger than the total depth _d_ . If it is smaller, a hole back area is generated
automatically.


**–** The _pHOffset_ parameter defines the offset from the center of the hole to the local origin
of the primitive. The hole may not go beyond the area of the outer volume.

#### **7.7 OiHPolygon**


V1 V2









**V0** V


Figure 7.6: The geometric type _OiHPolygon_



3



**Description**


_• OiHPolygon_ represents a one-sided, simple, planar and convex polygon from which a number
of simple, planar and convex polygons can be cut out.


_•_ **Super type:** _OiGeometry_


**Initialization**


_• OiHPolygon(pFather(MObject), pName(Symbol), pMosaic(Int), pOutline(Float[3][]),_
_pHoles(Float[3][][]))_


The function initializes an instance of the _OiHPolygon_ type. The _pMosaic_ parameter controls
the tessalation of the resulting polygon net. If _pMosaic_ obtains the value 0, it results in a
triangulation. Otherwise, the number of internal polygons is minimized. _pOutline_ describes
the outer polygon in clockwise direction. _pHoles_ is an optional empty vector of polygons that
each describes a cutout. These polygons must be defined in counterclockwise direction.


124


Figure 7.7: The geometric type _OiImport_

#### **7.8 OiImport**


**Description**


_• OiImport_ imports an external file in a geometric format. Provided that it does not contain
any materials, a material can be set via the _Material_ interface.


_OiImport_ optionally supports exactly one heavily resolution-reduced geometry next to the actual geometry. If it is present, it can be used for the speed-optimized presentation. However,
its use is dependent upon the respective presentation software.


_•_ **Super type:** _OiGeometry_


**Initialization**


_• OiImport(pFather(MObject), pName(Symbol), pMode(Symbol), pGeometry(String))_


The function initializes an instance of the _OiImport_ type. _pGeometry_ describes the name
of a geometry file in form of a simple string without path and extension information, e.g.,
”wheel.” The file type is determined by the _pMode_ parameter. Permissible implementations
for _pMode_ are:


**–** _@OFF_    - The geometry features the Object File Format.


125


**–** _@G3DS_    - The geometry features the 3D Studio format.


The optional resolution-reduced geometry file also contains an underscore character at the
beginning of the name, e.g., ” ~~w~~ heel.”


The data record is loaded in accordance with the definitions for external data (Chapter D)
and must be fully qualified.


**Methods**


_• setScale(pFactor(Float[3])) →_ _Void_


The function allows for the scaling of _OiImport_ objects. The elements of the vector _pFactor_
must be real, positive numbers. The initial scaling is 1 _._ 0 in all three dimensions.


If required, an adaptation of the alignment must be performed afterwards ( _setAlignment()_ ).


_• getScale() →_ _Float[3]_


The function furnishes the current scaling of the implicit instance.

#### **7.9 OiPolygon**


V3 V2



V0 V


Figure 7.8: The geometric type _OiPolygon_



1



**Description**


_• OiPolygon_ represents a one-sided, simple, planar and convex polygon. This primitive should
be used in exceptional cases only since a number of _OiPolygonen_ is extremely inefficient
compared to other polygon sets (e.g., on the basis of _OiImport_ ). In addition, a singular
_OiPolygon_ does not describe a body which contradicts the general intention of OFML.


_•_ **Super type:** _OiGeometry_


126


**Initialization**


_• OiPolygon(pFather(MObject), pName(Symbol), pPoints(Float[3][]))_


The function initializes an instance of the _OiPolygon_ type. The _pPoints_ parameter defines
a one-sided, simple, planar and convex polygon. The last and first point are automatically
connected. The visibility results by means of the right-hand rule. If the curvature of the
right hand follows the vertex line, the thumb of the right hand indicates the visible side.

#### **7.10 OiRotation**









V0


V2


V3





V0


V2


V3









z





Figure 7.9: The geometric type _OiRotation_


**Description**


_• OiRotation_ describes a solid body that is defined by the planar rotation of a three-dimensional
planar curve around an axis. The curve must be defined as follows, according to the righthand rule: If the thumb of the right hand points towards the direction of rotation, the
remaining fingers of the right hand indicate the orientation. Otherwise, an inversion must
take place.


_•_ **Super type** _OiGeometry_


127


**Initialization**


_• OiRotation(pFather(MObject), pName(Symbol), pMode(Symbol), pAxis(Float[3]),_
_pPoints(Float[3][]), pArc(Float), pUWMode(Symbol[2]), pCMode(Symbol[2]), pFlip(Int))_


The function initializes an instance of the _OiRotation_ type. This requires indicating the
following specific parameters:


**–** _pMode_ specifies whether the body along the definition curve should be smooth
( _@SMOOTH_ ) or not ( _@LINEAR_ ).


**–** _pAxis_ defines the rotation axis with respect to the local coordinate system.


**–** _pPoints_ describes the definition curve. However, points on the rotation axis are not
allowed.


**–** _pArc_ sets the angle of rotation of the definition curve. _pArc_ must be positive and smaller
than or equal to 2 _π_ .


**–** _pUWMode_ defines the openness ( _@OPEN_ ) or compactness ( _@CLOSED_ ) of the body
along two curves. _pUWMode[0]_ specifies whether the body along the rotation axis is
closed. In general, this is the case for bodies with _pArg_ = 2 _π_ . _pUWMode[1]_ specifies
whether a compactness of the body results with respect to the definition curve (by
joining the first and last point). In general, this is not the case.


**–** _pCMode_ defines the openness ( _@OPEN_ ) or compactness ( _@CLOSED_ ) of the body with
respect to two areas. _pCMode[0]_ specifies whether possibly resulting interfaces of the
body should be closed. This is only necessary for bodies with _pArg_ = 2 _π_ . _pCMode[1]_
specifies whether the cap areas of the body should be generated or not.


**–** _pFlip_ forces an inversion of the sequence in _pPoints_ if it features the value 1. Otherwise,
the value must be 0.

#### **7.11 OiSphere**







z





Figure 7.10: The geometric type _OiSphere_


128


**Description**


_• OiSphere_ represents a homogenous sphere that is centered about the origin of the local
coordinate system. The radius of the sphere can be changed after its generation.


_•_ **Super type** _OiGeometry_


**Initialization**


_• OiSphere(pFather(MObject), pName(Symbol), pRadius(Float))_


The function initializes an instance of the _OiSphere_ type. The initial radius of the sphere is
indicated by the positive number _pRadius_ .


**Methods**


_• setRadius(pRadius(Float)) →_ _Void_


The function sets the radius of the sphere. _pRadius_ must be a positive number.


If required, an adaptation of the alignment must be performed afterwards ( _setAlignment()_ ).


_• getRadius() →_ _Float_


The function delivers the current radius of the sphere.

#### **7.12 OiSweep**


V1



Vn-1

V1


Vn-1



l



V0


V0



V3


V3



V0


z



V1
V2


Vn-1







Figure 7.11: The geometric type _OiSweep_


129


**Description**


_• OiSweep_ describes a solid body that is defined by the planar move of a three-dimensional
planar curve along an axis. The curve must be defined as follows, according to the righthand rule: If the thumb of the right hand points towards the direction of move, the remaining
fingers of the right hand indicate the orientation. Otherwise, an inversion must take place.


_•_ **Super type:** _OiGeometry_


**Initialization**


_• OiSweep(pFather(MObject), pName(Symbol), pMode(Symbol), pAxis(Float[3]), pLength(Float),_
_pPoints(Float[3][]), pUMode(Symbol), pCMode(Symbol[2]), pFlip(Int))_


The function initializes an instance of the _OiSweep_ type. This requires indicating the following
specific parameters:


**–** _pMode_ specifies whether the body along the definition curve should be smooth
( _@SMOOTH_ ) or not ( _@LINEAR_ ).


**–** _pAxis_ defines the move axis with respect to the local coordinate system.


**–** _pLength_ sets the length of the body along the move axis. _pLength_ must be a positive
number.


**–** _pPoints_ describes the definition curve.


**–** _pUMode_ defines the openness ( _@OPEN_ ) or compactness ( _@CLOSED_ ) of the body along
the definition curve. If _pUMode_ = _@OPEN_, end point and start point of _pPoints_ are
connected by a straight line. Otherwise, an appropriate soft connection occurs.


**–** _pCMode_ defines the openness ( _@OPEN_ ) or compactness ( _@CLOSED_ ) of the body with
respect to two areas. _pCMode[0]_ specifies whether the side faces of the body should be
closed or not. _pCMode[1]_ specifies whether the connection of the last point with the
first point should be closed or not.


**–** _pFlip_ forces an inversion of the sequence in _pPoints_ if it features the value 1. Otherwise,
the value must be 0.


**Methods**


_• setLength(pLength(Float)) →_ _Void_


The function sets the length of the object along the move axis. _pLength_ must be a positive
number.


If required, an adaptation of the alignment must be performed afterwards ( _setAlignment()_ ).


_• getLength() →_ _Float_


The function delivers the current length of the object along the move axis.


130


Vm-1,n-1





V0,0















z



Figure 7.12: The geometric type _OiSurface_

#### **7.13 OiSurface**


**Description**


_• OiSurface_ describes a primitive that is defined by a two-dimensional net of three-dimensional
supporting points. Here, _u_ and _w_ are the dimensions of the net.


_•_ **Super type:** _OiGeometry_


**Initialization**


_• OiSurface(pFather(MObject), pName(Symbol), pUDim(Int), pWDim(Int),_
_pPoints(Float[3][pUDim(pWDim]), pUWMode(Symbol[2])_


The function initializes an instance of the _OiSurface_ type. This requires the following specific
parameters:


**–** _pUDim_ defines the u dimension of the net.


**–** _pWDim_ defines the w dimension of the net.


**–** _pPoints_ describes an array with the definition points. Within a patch, the right-hand
rule indicates the orientation, i.e., if the thumb of the right hand sits on the patch at a
right angle, the remaining fingers of the hand indicate the orientation.


**–** _pUWMode_ defines the openness ( _@OPEN_ ) or compactness ( _@CLOSED_ ) of the primitive
along the u and w dimension. If _pUWMode[0]_ = _@OPEN_, no connection of the net in
the u direction is made. If _pUWMode[1]_ = _@OPEN_, no connection of the net in the w
direction is made.


131


## **Chapter 8**

# **Global Planning Types**

This chapter describes global, higher-level planning base types. These base types are independent
of concrete planning elements (pieces of furniture) and, therefore, also independent of concrete
geometric implementations.


The types described here are based on the conceptual model shown in Figure 8.1.







Figure 8.1: Conceptual model of the global planning types

#### **8.1 OiPlanning**


**Description**


_•_ An instance of this type functions as root object of a complete planning hierarchy and
implements global planning logics for the elements of the planning ( _OiPlElement_ type).


_•_ Additional tasks of the global planning object include:


132


**–** The definition of a _planning limit_ that specifies the space within which the planning
elements can be placed.


**–** The monitoring and handling of the transformation of planning elements for the purpose
of avoiding collisions and exceeding planning limits.


**–** The management and utilization of information about characteristics and requirements
of the (furniture) programs to which the elements belong that are represented in the
planning ( _OiProgInfo_ type).


**–** Use of a product data manager for accessing product data (see also Chapter 9).


_•_ **Interface(s):** Base, Complex, Property, Material


**Initialization**


_• OiPlanning(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiPlanning_ type. Initially, the selection option is
deactivated. The initial planning limit is infinite.


**Methods**


**General Methods**


_• setLanguage(pLang(String)) →_ _Void_


The function specifies the language to be used for subsequent messages and labels. The
_pLang_ parameter describes the national language through a string in accordance with ISO
639 guidelines.


**Examples:**


**–** `"de"`    - German


**–** `"en"`    - English


**–** `"nl"`    - Dutch


_• getLanguage() →_ _String_


The function delivers the language that is currently used for messages and labels.


_• setRegion(pRegion(String)) →_ _Void_


The function specifies the sales region, i.e., generally a country, for which the current planning
is created. The _pRegion_ parameter describes the sales region through a string in accordance
with ISO 3166 guidelines (ISO Code 2) or extensions in this connection for the designation
of federal states, etc.


**Examples:**


**–** `"DE"`    - Germany


133


**–** `"UK"`    - United Kingdom


**–** `"NL"`    - Netherlands


_• getRegion() →_ _String_


The function delivers the current sales region. If no sales region was specified, the return
value is of type _Void_ .


_• setProgram(pProgr(Symbol)) →_ _Void_


The function specifies the currently relevant program.


**Note:** Programs are distinguished based on their ID (identification symbol). The program ID must
be unique across all manufacturers. For this reason, it starts with a two-digit manufacturer code,
followed by the code for the actual program.


The program ID is used with certain operations for the delegation of functionality to program-specific

information or similar objects. The currently relevant program can be determined externally through

the runtime environment, or even internally out of the certain context, e.g. out of the association of

the currently processed planning element to a program.


_• delegationDone() →_ _Void_


The function signals the implicit planning instance the successful execution of a functionality
delegated to another instance ( _Delegat_ ).


**Note:** The function is called by the delegation instance upon successful execution of the delegated

functionality.


**Error Log**


Complex test algorithms that are executed on an object structure can lead to error messages
concerning various objects of the structure. Instead of issuing an error message from every test
method for the corresponding objects, it is generally desirable to collect these messages and view
them together in a dialog. For this purpose, the global planning instance manages a so-called _error_
_log_ . The instance or method that initiates a global testing process, generates the data structure
to be used for the log of the testing process and passes it on to the _setErrorLog()_ function before
the execution of the test is delegated to another instance or inherited implementations are called.
Implementations of the testing algorithm must first be checked with the _getErrorLog()_ function
whether a higher-level log was created, in which case the generated messages must be entered in
this log and no separate dialog for display may be started. If no higher-level log exists, it must be
created before possible delegations can take place, and a dialog for displaying the messages from
the log must be started at the end of the testing process. The data structure used for the log can
be defined separately for each testing algorithm.


**Note:** An application example of an error log is located in the _Article_ interface under the performance of

consistency checks.


134


_• setErrorLog(pLog(Any)) →_ _Void_


The function assigns the implicit planning instance a new data structure for the error log.


_• getErrorLog() →_ _Any_


The function returns the reference to the last data structure for the error log that was assigned
by means of the _setErrorLog()_ function.


**Instance Hierarchy**


_• getEnvironment() →_ _MObject_


The function delivers the root object of the hierarchy of the planning environment. The
function furnishes a value of the type _Void_ if no planning environment exists.


_• getPlElementUp(pObj(MObject)) →_ _MObject_


The function traverses the instance hierarchy, beginning with the transferred instance and
upward to the root object and delivers the first instance which is of the _OiPlElement_ type.
If no such instance was located on the traversing path, the function delivers a value of the
type _Void_ .


_• getTopPlElement(pObj(MObject)) →_ _MObject_


The function traverses the instance hierarchy, beginning with the transferred instance and
upward to the root object and delivers the top instance which is of the _OiPlElement_ type.
If no such instance was located on the traversing path, the function delivers a value of the
type _Void_ .


_• getPropObj(pObj(MObject)) →_ _MObject_


The function traverses the instance hierarchy, beginning with the transferred instance and
upward to the root object and delivers the first instance that features properties. If no such
instance was located on the traversing path, the function delivers a value of the type _Void_ .


**Planning Limit**


_• setBorder(pBorder(Float[2][3]) →_ _Void_


The function assigns the implicit planning instance a new value for the (axis-orthogonal)
planning limit volume.
The _pBorder_ parameter is a vector consisting of two vectors with three _Float_ values each. The
first _Float_ vector specifies the origin of the acceptable planning volume in world coordinates.
The second _Float_ vector determines the maximum expansion of the acceptable planning
volume along the x-, y- and z-axes. If the spatial location of the planning is not important,
a value of the type _Void_ may also be transferred instead of the first vector.


_• getBorder() →_ _Float[2][3]_


The function delivers the specification for the planning limit volume that is currently used
in the implicit planning instance. Structure and semantics of the return value correspond to
the parameter for the _setBorder()_ function.


135


_• checkBorder() →_ _String_


The function checks whether the planning limit is maintained by the planning elements
currently contained in the implicit planning instance.
In the case of a limit violation, the function delivers a string that contains the corresponding
error message. If the limit is maintained, the function delivers a value of the type _Void_ .


**Management of Program Information**


_• addInfoObj(pType(Type), pID(Symbol)) →_ _Void_


The function adds an instance of the indicated type to the set of program information objects
and registers them under the indicated program ID. If a program information object with
the indicated program ID already exists, it is removed before the new object is inserted.


**Note:** The program information objects are inserted into the instance hierarchy of the planning as

non-graphical (thus, not visible) objects. After storing and reloading the planning, these information

objects are available immediately.


_• delInfoObj(pID(Symbol)) →_ _Void_


The function removes the program information object with the indicated program ID.


_• clearInfoObjs() →_ _Void_


The function removes all program information objects.


_• getInfoIDs() →_ _Symbol[]_


The function delivers the program IDs of all registered program information objects.


_• getInfo(pID(Symbol)) →_ _MObject_


The function delivers the program information object with the indicated program ID or a
value of the type _Void_ if no program information object is registered under the indicated
program ID.


**Materials**


_• Material::getMatCategories() →_ _Symbol[]_


_• Material::getCMaterials(pCat(Symbol)) →_ _Symbol[]_


_• Material::getCMaterial(pCat(Symbol)) →_ _Symbol_


_• Material::setCMaterial(pCat(Symbol), pMat(Symbol)) →_ _Int_


_• Material::getMatName(pMat(Symbol)) →_ _String_


The standard implementation performs a string conversion of the symbol.


These functions implement the corresponding functions of the _Material_ interface by means of
delegation to the functions of the program information object under the same name ( _OiProgInfo_
type) of the currently relevant program ( _setProgram()_ function).


136


**Element Management and Collision Detection**


_• Complex::checkAdd((pType(Type), pObj(MObject), pPos(Float[3]), pParams(Any)) →_ _Float[3]_


The function checks whether an instance of the indicated type can be inserted as element
into the planning and, if positive, delivers a valid position for the element.
(For more information about the semantics of the function or its parameter, see the _Complex_
interface.)
First, the function calls the function of the program information object under the same name
( _OiProgInfo_ type) for the program to which the instance belongs that was transferred in the
_pObj_ parameter. Afterwards, a program-independent check is performed in accordance with
a global planning logic, if required. This requires a call to the _doCheckAdd()_ hook function.


_• doCheckAdd(pType(Type), pObj(MObject), pPos(Float[3]), pParams (Any)) →_ _Float[3]_


The function checks independent of concrete furniture programs whether an instance of the
indicated type can be inserted as element into the planning and, if positive, delivers a valid
position for the element. The semantics of the parameters corresponds to the _checkAdd()_
function. The standard implementation achieves an attaching of the new element to the right
of the existing planning.


**Note:** The function is called by _checkAdd()_ and, in contrast to _checkAdd()_, can be redefined in

subtypes where the function of the same name of the immediate super type should be called in the

case of non-applicability of the special planning logic that is implemented by the subtype.


_• Complex::checkChildColl(pObj(MObject), pExclObj(MObject)) →_ _MObject_


The function checks whether a collision of the transferred (child) instance with other objects
is present. If the _pExclObj_ argument contains a non-empty set of objects, they are excluded
from the collision check. The function first checks for collision with the children of the implicit
instance. Objects for which the _isValidForCollCheck()_ hook function delivers the value 0
are excluded from the collision check. Before and after this check, the _startCollCheck()_ or
_finishCollCheck()_ functions of the program information object are called for the program to
which instance belongs that is transferred in the _pObj_ parameter. Afterwards, the function
of the same name of the root object of the hierarchy of the planning environment is called (if
it exists and its type implements the _Complex_ interface). The return value is the first located
object with which the transferred instance collides or a value of type _Void_ if no collision was
detected or if the collision detection is deactivated.


_• Complex::isValidForCollCheck(pObj(MObject)) →_ _Int_


This function implements the corresponding function of the _Complex_ interface by means of
delegation to the function of the same name of the program information object ( _OiProgInfo_
type) for the program to which the instance belongs that is transferred in the _pObj_ parameter.


_• Complex::checkElPos(pEl(MObject), pOldPos(Float[3])) →_ _Int_


The function implements the corresponding function of the _Complex_ interface by means
of collision detection and planning limit monitoring ( _checkChildColl()_ and _checkBorder()_
functions).


137


**Element Transformations**


_• elemTranslation(pEl(MObject), pOldPos(Float[3])) →_ _Void_


The function handles an (already performed) translation of the indicated planning element
in the following way.


**–** First, the general acceptability of the translation of the planning element is checked (see
also _translateValid()_ function of the _OiPlElement_ type).


**–** If the translation is acceptable on principle, the _translated()_ function of the transferred
planning element is now called (see also the _OiPlElement_ type).


**–** If the _translated()_ function returned the value 0, the implicit instance now checks the
validity of the current position of the planning element (collision detection, adherence to
planning limit, and others). If necessary, a correction of the current position may occur
before the translation with the aid of the position of the planning element transferred
in the _pOldPos_ parameter.


If the indicated object is not an instance of the _OiPlElement_ type, the function is without
effect.


**Note:** The function is called from the _TRANSLATE_ rule of the transferred planning element

( _OiPlElement_ type).


_• elemRotation(pEl(MObject), pOldRot(Float)) →_ _Void_


The function handles the rotation of the indicated planning element in the following way.


**–** First, the general acceptability of the rotation of the planning element is checked (see
also _rotateValid()_ function of the _OiPlElement_ type).


**–** If the rotation is acceptable on principle, the _rotated()_ function of the transferred planning element is now called (see also the _OiPlElement_ type).


**–** If the _rotated()_ function returned the value 0, the implicit instance now checks the validity of the current rotary angle of the planning element (collision detection, adherence
to planning limit, and others). If necessary, a correction of the current angle may occur
before the rotation with the aid of the rotary angle of the planning element transferred
in the _pOldRot_ parameter.


If the indicated object is not an instance of the _OiPlElement_ type, the function is without
effect.


**Note:** The function is called from the _ROTATE_ rule of the transferred planning element ( _OiProgInfo_

type).


_• checkPosition(pEl(MObject), pPos(Float[3]), pAngles(Float[3])) →_ _Float[2][3]_


The function checks whether the indicated position and the indicated rotary angle (per axis)
are allowed for the transferred planning element.


The return value is a vector consisting of two vectors of three _Float_ values each. The first
vector specifies an acceptable position, the second the rotary angle (per axis). The returned


138


values may deviate from the desired values transferred in the parameters to a certain extent
to prevent collisions and other conflicts. If the planning element cannot be placed at the
desired position on principle (or in its vicinity), the return vector contains a value of the
_Void_ type instead of a position information.


**Note:** The function is called by the runtime environment during a dialog for explicit positioning of

a planning element.


**Product Data Management**


_• setPDManager(pType(Type)) →_ _Void_


The function generates an instance of the indicated type to be used as global product data
manager ( _OiPDManager_ type). If a product data manager instance already exists, it is
removed first.


_• getPDManager() →_ _MObject_


The function delivers the global product data manager instance or a value of the type _Void_
if such an instance is not registered.


_• article2Class(pArticle(String)) →_ _String_


The function delivers the name of the type that models the article which was specified based
on its article number, or a value of the type _Void_ if no assignment could be found for the
article. If a global product data manager instance is registered, the query to this instance is
delegated.


_• addProductDB(pType(Type), pID(Symbol), pPath(String), pProgList(Symbol[]) →_ _MObject_


The function generates an instance of the transferred type (subtype of _OiProductDB_ ) and
registers it with the global product data manager under the indicated ID. The file system
path of the directory that contains the files of the product database is transferred in the
_pPath_ parameter (relative to the root directory of the runtime environment). The additional
_pProgList_ parameter specifies the programs (IDs) that are represented in the database. If
a product database is already registered under the indicated ID, the list of programs of the
product database is expanded, if required.


The return value is the reference to the (generated) product database instance.


**Note:** The function achieves the same effect as the function of the _OiPDManager_ type of the same

name.


**Miscellaneous**


_• checkConsistency() →_ _Int_


The function checks the consistence and completeness of the planning. If required, corrections
or additions are performed or error messages are generated. The function delivers True if the
planning is consistent, otherwise False.


139


First, the function calls the function of the same name on all registered entities of _OiProgInfo_
and then on all children of the _OiPlElement_ type. The result of the check is False if the check
was not successful for at least one instance. The check uses the error log written with the
_checkConsistency()_ function in the _Article_ interface.


**Note:** The function is usually called by the runtime environment before the creation of an order

list.


_• checkObjConsistency(pObj(MObject) →_ _Int_


The function performs a consistence check on the transferred instance of the _OiPlElement_ or
_OiPart_ type. Besides the call of the _checkConsistency()_ method on the transferred instance,
the function can perform additional actions, e.g., displaying or removing a visual feedback
with incorrect articles or adding or removing an entry in the global error log.


_• doSpecial(pPID(Symbol), pOp(Symbol), pArgs(Any)) →_ _Any_


Using the transferred arguments, the function performs the indicated operation concerning
the program specified by the transferred ID. If a program information object is registered for
the program, the function is delegated to it ( _OiProgInfo_ type). The return value is dependent
upon the operation.


**Note:** The function can be used for expanding the functionality of a planning system without having

to expand the interface between runtime environment and global planning instance.


**Rules**


_• REMOVE_ _ELEMENT(pValue(Symbol)) →_ _Int_


The rule prevents the removal of planning elements whose _removeValid()_ function delivers
the value 0.

#### **8.2 OiProgInfo**


**Description**


_•_ Entities of this type manage information about a (furniture) program (Appendix I) or implement program-specific functions if requested by the global planning instance ( _OiPlanning_
type).


_•_ **Interface(s):** _MObject_, _Property_


**Initialization**


_• OiProgInfo(pFather(MObject), pName(Symbol), pPID(Symbol))_


The function initializes an instance of the _OiProgInfo_ type with the indicated program ID.
The program ID cannot be changed later.


140


**Methods**


**General Methods**


_• getID() →_ _Symbol_


The function delivers the ID of the program for which the implicit instance responsible is.


_• getPlanning() →_ _MObject_


The function delivers the root object of the planning hierarchy ( _t_ ) if this is an instance of
the _OiPlanning_ type, otherwise a value of the type _Void_ .


_• checkConsistency() →_ _Int_


The function performs a program-specific consistence check. It is called by the global planning
instance of the _OiPlanning_ type with a global consistence check before the check on the
planning elements is performed.


_• doSpecial(pOp(Symbol), pArgs(Any)) →_ _Any_


Using the transferred arguments, the function performs the indicated operation (see also the
function of the same name of the _OiPlanning_ type).


**Materials**


_• getMatCategories() →_ _Symbol[]_


_• getCMaterials(pCat(Symbol)) →_ _Symbol[]_


_• setCMaterial(pCat(Symbol), pMat(Symbol)) →_ _Int_


_• getCMaterial(pCat(Symbol)) →_ _Symbol_


_• getMatName(pMat(Symbol)) →_ _String_


The standard implementation performs a string conversion of the symbol.


These functions represent program-specific versions of the corresponding functions of the _Mate-_
_rial_ interface and are called by the functions of the same name of the global planning instance
( _OiPlanning_ type).


**Element Management and Collision Detection**


_• Complex::checkAdd((pType(Type), pObj(MObject), pPos(Float[3]), pParams(Any)) →_ _Float[3]_


The function checks whether an instance of the indicated type can be inserted as neighboring
element of the program element that is transferred in the _pObj_ parameter, into the planning
and, if positive, delivers a valid position for the element.
The semantics of the function or its parameter correspond to the function of the same name
of the global planning instance ( _OiPlanning_ type) and is called by it.


141


_• isValidForCollCheck(pObj(MObject)) →_ _Int_


The function delivers 1 if the indicated program element should be considered in the collision
check, otherwise 0. The function is called by the function of the same name of the global
planning instance ( _OiPlanning_ type).


_• startCollCheck(pObj(MObject)) →_ _Void_


The function performs required actions before the indicated program element is checked for
collision with other planning elements.
It is called by the _checkChildObj()_ function of the global planning instance ( _OiPlanning_ type).
The standard implementation of the function does not perform any actions.


_• finishCollCheck(pObj(MObject)) →_ _Void_


The function performs required actions after the indicated program element was checked for
collision with other planning elements.
It is called by the _checkChildObj()_ function of the global planning instance ( _OiPlanning_ type).
The standard implementation of the function does not perform any actions.

#### **8.3 OiPlElement**


**Description**


_•_ Entities of the _OiPlElement_ type represent independent elements of a planning.


_•_ Planning elements cooperate in a defined way with the global planning instance ( _OiPlanning_
type).


_•_ **Interface(s):** Base, Complex, Material, Property, Article


**Initialization**


_• OiPlElement(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiPlElement_ type.


The initialization function of concrete subtypes must define the properties of the planning
element. This is accomplished either by means of the _setupProperty()_ function of the _Prop-_
_erty_ interface or through delegation to the _setupProps()_ function of the global product data
manager ( _OiPDManager_ type) if such an instance exists.


**Methods**


**General Methods**


_• getPlanning() →_ _MObject_


The function delivers the root object of the planning hierarchy ( _t_ ) if this is an instance of
_OiPlanning_, otherwise a value of the type _Void_ .


142


_• setPlProgram() →_ _Void_


The function assigns the inherent program specified through the _getProgram()_ function as
the currently relevant program by means of the _setProgram()_ function ( _OiPlanning_ type) to
the global planning instance.


**Spatial Model**


_• setWidth(pWidth(Float)) →_ _Void_


The function assigns an explicit value for the width expansion to the implicit instance.


_• Complex::getWidth() →_ _Float_


The function furnishes the width of the implicit instance. If a value was assigned for the
width during or after the initialization by means of the _setWidth()_ method, it is returned,
otherwise the width of the delimiting volume determined by the _getLocalBounds()_ method
( _Base_ interface).


_• setHeight(pHeight(Float)) →_ _Void_


The function assigns an explicit value for the height expansion to the implicit instance.


_• Complex::getHeight() →_ _Float_


The function furnishes the height of the implicit instance. If a value was assigned for the
height during or after the initialization by means of the _setHeight()_ method, it is returned,
otherwise the height of the delimiting volume determined by the _getLocalBounds()_ method
( _Base_ interface).


_• setDepth(pDepth(Float)) →_ _Void_


The function assigns an explicit value for the depth expansion to the implicit instance.


_• Complex::getDepth() →_ _Float_


The function furnishes the depth of the implicit instance. If a value was assigned for the
depth during or after the initialization by means of the _setDepth()_ method, it is returned,
otherwise the depth of the delimiting volume determined by the _getLocalBounds()_ method
( _Base_ interface).


_• setOrigin(pOrigin(Float[3])) →_ _Void_


The function assigns an offset of the reference origin with respect to the minimum of the
local delimiting volume to the implicit instance.


_• getOrigin() →_ _Float[3]_


The function delivers the offset of the reference origin of the implicit instance with respect
to the minimum of the local delimiting volume. If a value was assigned for the offset during
or after the initialization by means of the _setOrigin()_ method, it is returned, otherwise it is
determined with the help of the _getLocalBounds()_ method of the _Base_ interface.


143


**Materials**


In each of the following functions, a call of _setPlProgram()_ is performed at the beginning.


_• Material::getMatCategories() →_ _Symbol[]_


It delivers the list of material categories currently defined for the implicit instance (for detailed
specifications see the _Material_ interface). The standard implementation delivers a value of
type _Void_ .


_• Material::getAllMatCats() →_ _Symbol[]_


It furnishes the list of _all_ material categories that are potentially definable for the implicit
instance. The standard implementation delivers the return value of the _getMatCategories()_
function.


_• Material::getCMaterials(pCat(Symbol)) →_ _Symbol[]_


It delivers the list of all materials that are applicable within the transferred material category
for the implicit instance (for detailed specifications see the _Material_ interface). The standard
implementation delivers the return value of the function of the same name of the global
planning instance if its type is _OiPlanning_, otherwise a value of type _Void_ .


_• Material::getCMaterial(pCat(Symbol)) →_ _Symbol_


The function furnishes the material currently assigned to the implicit instance in the transferred material category or a value of the _Void_ type if the implicit instance does not currently
belong to the transferred material category. The standard implementation delivers the return value of the function of the same name of the global planning instance if its type is
_OiPlanning_, otherwise a value of type _Void_ .


**Note:** Concrete subclasses must overwrite this method in such a way that the material currently set

in the object for this category is delivered. The standard implementation (call of the father object)

must be performed only if a material in this category has not been assigned with explicit assignment

to the object.


_• Material::getMatName(pMat(Symbol)) →_ _String_


The function furnishes the material name to the transferred material or a value of the _Void_
type for the implicit instance if the material is unknown. The standard implementation
delivers the return value of the function of the same name of the global planning instance if
its type is _OiPlanning_, otherwise the return value of the function of the same name of the
father instance if its type implements the _Material_ interface.


**Element Generation**


_• isElemCatValid(pCat(Symbol)) →_ _Int_


The function delivers 1 if instances of the indicated category can be added to the implicit
instance as elements, otherwise 0.


The standard implementation delivers 0.


144


**Example:** The _isElemCatValid()_ function of a type of table on which instances of the _@TOP_ ~~_E_~~ _LEM_

category can be placed, must deliver 1 for this category.


**Note:** After checking for special categories during an overwriting of the function in derived types,

the inherited function must be called so that 1 is also delivered for the categories which are allowed

by super types.


_• Complex::checkAdd((pType(Type), pObj(MObject), pPos(Float[3]), pParams(Any)) →_ _Float[3]_


The function checks whether an instance of the indicated type can be inserted as element
into the planning and, if positive, delivers a valid position for the element.


The standard implementation implements the placement of elements of the _@TOP_ ~~_E_~~ _LEM_
category if the _isElemCatValid()_ function delivers 1 for this category. The _getWidth()_, _getH-_
_eight()_, _getDepth()_, and _getOrigin()_ functions are used for this purpose.


_• getPDistance() →_ _Float_


The function delivers the desired initial distance to the previous element.
The standard implementation delivers the minimum x-value of the local delimiting volume.


**Note:** The function can be used within the _checkAdd()_ function of the father instance.

The value delivered by the function can be queried by the user through a dialog before the new

element is inserted. The subtypes must make a corresponding set function available for this purpose.


_• getWallOffset() →_ _Float_


The function delivers the desired initial distance to a wall element in front of which the
implicit instance should be placed.
The standard implementation delivers 0.01 minus the minimum z-value of the local delimiting
volume.


**Note:** The function can be used within the _checkAdd()_ function of the father instance.

The value delivered by the function can be queried by the user through a dialog before the new

element is inserted. The subtypes must make a corresponding set function available for this purpose.


_• onCreate(pRot(Float), pObj(MObject), pParams(Any)) →_ _Void_


The function can be called after the generation of the implicit instance and ends the overall
process of interactively inserting the instance into the planning. The requested rotation with
respect to the y-axis in positive direction, the neighboring element to which the implicit
instance was added, and an additional random parameter are transferred. The standard
implementation implements the required rotation with respect to the y-axis in positive direction.


**Note:** The function is used for setting object properties that cannot be performed during the object

generation (in the _initialize()_ function) for lack of knowledge of the planning context. The function is


145


usually set together with the appropriate arguments during the _checkAdd()_ of _OiPlanning_ by means

of calling _setMethod()_ ( _Complex_ interface).


**Element Control**


_• elRemoveValid(pObj(MObject)) →_ _Int_


The function returns True if the transferred child instance can be removed. The function is
called in REMOVE ~~E~~ LEMENT rules in addition to the _removeValid()_ function of the _Base_
interface for the instance that is to be removed.


**Example:** A cupboard unit subplanning can use this, for example, to remove elements from the

left and right side only.


The standard implementation delivers True.


_• isElOrderSubPos(pObj(MObject)) →_ _Int_


The function delivers True if the transferred child instance may not appear as a subitem in
an order list.


**Example:** The function can be used in algorithms for generating order lists to move certain items

to specific positions in the order list.


The standard implementation delivers True.


**Product Data**


_• Article::getArticleSpec() →_ _String_


The standard implementation of the function delegates the query to the _class2Article()_ function of the global product data manager ( _OiPDManager_ type), if such an instance exists.


_• Article::setArticleSpec(pSpec(String)) →_ _Void_


The standard implementation does not perform any actions.


_• Article::getArticleParams() →_ _Any_


The standard implementation delivers a value of type _Void_ .


_• Article::getArticlePrice(pLanguage(String)) →_ _Any[]_


The standard implementation of the function delegates the query to the function of the same
name of the global product data manager ( _OiPDManager_ type), if such an instance exists.


_• Article::getArticleText(pLanguage(String), pForm(Symbol)) →_ _String[]_


The standard implementation of the function delegates the query to the function of the same
name of the global product data manager ( _OiPDManager_ type), if such an instance exists.


_• Article::getArticleFeatures(pLanguage(String)) →_ _Any_


The standard implementation of the function delegates the query to the function of the same
name of the global product data manager ( _OiPDManager_ type), if such an instance exists.


146


**Consistence Check**


_• Article::checkConsistency() →_ _Int_


The function checks the consistence and completeness of the planning element and is called
by _OiPlanning::checkConsistency()_ . If required, corrections or additions are performed or
error messages are generated.


The standard implementation delegates to the function of the same name of the global
product data manager ( _OiPDManager_ type).


**Child Transformations**


_• elemTranslation(pEl(MObject), pOldPos(Float[3])) →_ _Void_


The function handles a (completed) translation of the transferred child instance of the
_OiPlElement_ or _OiPart_ type.


For instances of the _OiPlElement_ type, this is accomplished in the same way as the _OiPlanning_
function of the same name. For instances of the _OiPart_ type, the _onTranslate()_ function is
called.


**Note:** The function is called from the _TRANSLATE_ rule of the transferred child instance.


_• elemRotation(pEl(MObject), pOldRot(Float)) →_ _Void_


The function handles the (completed) rotation of the transferred child instance of the _OiPlEle-_
_ment_ or _OiPart_ type.


For instances of the _OiPlElement_ type, this is accomplished in the same way as the _OiPlanning_
function of the same name. For instances of the _OiPart_ type, the _onRotate()_ function is called.


**Note:** The function is called from the _ROTATE_ rule of the transferred child instance.


**Translation and Rotation**


_• translateValid(pOldPos(Float[3])) →_ _Int_


The function delivers 1 if the planning element can be moved from the transferred old position
to the new current position, otherwise it delivers 0.
The standard implementation delivers 1.


**Note:** The function is used within the _elemTranslation()_ function of the global planning instance

( _OiPlanning_ type).


_• translated(pOldPos(Float[3])) →_ _Int_


The function is called by the _elemTranslation()_ function of the global planning instance to
enable the planning element to individually react to its translation. The return value is 1 if
the function has already checked the validity of the new position, otherwise it is 0.


147


_• rotateValid(pOldPos(Float)) →_ _Int_


The function delivers 1 if the planning element can be rotated from the transferred old rotary
angle to the new current rotary angle, otherwise it is 0.
The standard implementation delivers 1.


**Note:** The function is used within the _elemRotation()_ function of the global planning instance

( _OiPlanning_ type).


_• rotated(pOldPos(Float)) →_ _Int_


The function is called by the _elemRotation()_ function of the global planning instance to
enable the planning element to individually react to its rotation. The return value is 1 if the
function has already checked the validity of the new rotary angle, otherwise it is 0.


**Rules**


_• REMOVE_ _ELEMENT(pValue(Symbol)) →_ _Int_


The rule prevents the removal of child instances whose _removeValid()_ function delivers False
or for which the _elRemoveValid()_ function delivers False.


_• TRANSLATE(pValue(Float[3])) →_ _Int_


The rule delegates the handling of the translation to the _elemTranslation()_ function of the
global planning instance ( _OiPlanning_ type).


_• ROTATE(pValue(Float)) →_ _Int_


The rule delegates the handling of the rotation to the _elemRotation()_ function of the global
planning instance ( _OiPlanning_ type).

#### **8.4 OiPart**


**Description**


_•_ The _OiPart_ type is the basic type for functional base types that are used as components in
planning elements ( _OiPlElement_ class).


_•_ **Interface(s):** Base, Complex, Material, Property, Article


**Initialization**


_• OiPart(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiPart_ type.


148


**Methods**


**General Methods**


_• getPlanning() →_ _MObject_


The function delivers the root object of the planning hierarchy ( _t_ ) if this is an instance of
_OiPlanning_, otherwise a value of the type _Void_ .


**Spatial Model**


_• setWidth(pWidth(Float)) →_ _Void_


The function assigns an explicit value for the width expansion to the implicit instance.


_• Complex::getWidth() →_ _Float_


The function furnishes the width of the implicit instance. If a value was assigned for the
width during or after the initialization by means of the _setWidth()_ method, it is returned,
otherwise the width of the delimiting volume determined by the _getLocalBounds()_ method
( _Base_ interface).


_• setHeight(pHeight(Float)) →_ _Void_


The function assigns an explicit value for the height expansion to the implicit instance.


_• Complex::getHeight() →_ _Float_


The function furnishes the height of the implicit instance. If a value was assigned for the
height during or after the initialization by means of the _setHeight()_ method, it is returned,
otherwise the height of the delimiting volume determined by the _getLocalBounds()_ method
( _Base_ interface).


_• setDepth(pDepth(Float)) →_ _Void_


The function assigns an explicit value for the depth expansion to the implicit instance.


_• Complex::getDepth() →_ _Float_


The function furnishes the depth of the implicit instance. If a value was assigned for the
depth during or after the initialization by means of the _setDepth()_ method, it is returned,
otherwise the depth of the delimiting volume determined by the _getLocalBounds()_ method
( _Base_ interface).


_• setOrigin(pOrigin(Float[3])) →_ _Void_


The function assigns an offset of the reference origin with respect to the minimum of the
local delimiting volume to the implicit instance.


_• getOrigin() →_ _Float[3]_


The function delivers the offset of the reference origin of the implicit instance with respect
to the minimum of the local delimiting volume. If a value was assigned for the offset during
or after the initialization by means of the _setOrigin()_ method, it is returned, otherwise it is
determined with the help of the _getLocalBounds()_ method of the _Base_ interface.


149


**Materials**


_• Material::getMatCategories() →_ _Symbol[]_


It delivers the list of material categories currently defined for the implicit instance (for detailed
specifications see the _Material_ interface). The standard implementation delivers a value of
type _Void_ .


_• Material::getAllMatCats() →_ _Symbol[]_


It furnishes the list of _all_ material categories that are potentially definable for the implicit
instance. The standard implementation delivers the return value of the _getMatCategories()_
function.


_• Material::getCMaterials(pCat(Symbol)) →_ _Symbol[]_


It delivers the list of all materials that are applicable within the transferred material category
for the implicit instance (for detailed specifications see the _Material_ interface). The standard
implementation delivers the return value of the function of the same name of the father
instance if its type implements the _Material_ interface, otherwise a value of type _Void_ .


_• Material::getCMaterial(pCat(Symbol)) →_ _Symbol_


The function furnishes the material currently assigned to the implicit instance in the transferred material category or a value of the _Void_ type if the implicit instance does not currently
belong to the transferred material category. The standard implementation delivers the return
value that was delivered by the function of the same name from the father instance.


_• Material::getMatName(pMat(Symbol)) →_ _String_


The function furnishes the material name to the transferred material or a value of the _Void_
type for the implicit instance if the material is unknown. The standard implementation
delivers the return value of the function of the same name of the global planning instance if
its type is _OiPlanning_, otherwise the return value of the function of the same name of the
father instance if its type implements the _Material_ interface.


**Element Generation**


_• isElemCatValid(pCat(Symbol)) →_ _Int_


The function delivers 1 if instances of the indicated category can be added to the implicit
instance as elements, otherwise 0.


The standard implementation delivers 0.


**Example:** The _isElemCatValid()_ function of a type of table on which instances of the _@TOP_ ~~_E_~~ _LEM_

category can be placed, must deliver 1 for this category.


**Note:** After checking for special categories during an overwriting of the function in derived types,

the inherited function must be called so that 1 is also delivered for the categories which are allowed

by super types.


150


_• Complex::checkAdd((pType(Type), pObj(MObject), pPos(Float[3]), pParams(Any)) →_ _Float[3]_


The function checks whether an instance of the indicated type can be inserted as element
into the planning and, if positive, delivers a valid position for the element.


The standard implementation implements the placement of elements of the _@TOP_ ~~_E_~~ _LEM_
category if the _isElemCatValid()_ function delivers 1 for this category. The _getWidth()_, _getH-_
_eight()_, _getDepth()_, and _getOrigin()_ functions are used for this purpose.


**Element Control**


_• elRemoveValid(pObj(MObject)) →_ _Int_


The function returns True if the transferred child instance can be removed. The function is
called in REMOVE ~~E~~ LEMENT rules in addition to the _removeValid()_ function of the _Base_
interface for the instance that is to be removed.


**Example:** A cupboard unit subplanning can use this, for example, to remove elements from the

left and right side only.


The standard implementation delivers True.


_• isElOrderSubPos(pObj(MObject)) →_ _Int_


The function delivers True if the transferred child instance may not appear as a subitem in
an order list.


**Example:** The function can be used in algorithms for generating order lists to move certain items

to specific positions in the order list.


The standard implementation delivers True.


**Product Data**


_• Article::getArticleSpec() →_ _String_


The standard implementation of the function delegates the query to the _class2Article()_ function of the global product data manager ( _OiPDManager_ type), if such an instance exists.


_• Article::setArticleSpec(pSpec(String)) →_ _Void_


The standard implementation does not perform any actions.


_• Article::getArticleParams() →_ _Any_


The standard implementation delivers a value of type _Void_ .


_• Article::getArticlePrice(pLanguage(String)) →_ _Any[]_


The standard implementation of the function delegates the query to the function of the same
name of the global product data manager ( _OiPDManager_ type), if such an instance exists.


151


_• Article::getArticleText(pLanguage(String), pForm(Symbol)) →_ _String[]_


The standard implementation of the function delegates the query to the function of the same
name of the global product data manager ( _OiPDManager_ type), if such an instance exists.


_• Article::getArticleFeatures(pLanguage(String)) →_ _Any_


The standard implementation of the function delegates the query to the function of the same
name of the global product data manager ( _OiPDManager_ type), if such an instance exists.


**Consistence Check**


_• Article::checkConsistency() →_ _Int_


The function checks the consistence and completeness of the planning element and is called
by _OiPlanning::checkConsistency()_ . If required, corrections or additions are performed or
error messages are generated.


The standard implementation delegates to the function of the same name of the global
product data manager ( _OiPDManager_ type).


**Translation and Rotation**


_• onTranslate(pOldPos(Float[3])) →_ _Void_


The function is called by the translation rule and is used in derived classes to implement a
specific behavior for a move.


_• onRotate(pOldRot) →_ _Void_


The function is called by the rotation rule and is used in derived classes to implement a
specific behavior for a rotation.


**Rules**


_• REMOVE_ _ELEMENT(pValue(Symbol)) →_ _Int_


The rule prevents the removal of child instances whose _removeValid()_ function delivers False
or for which the _elRemoveValid()_ function delivers False.


_• TRANSLATE(pValue(Float[3])) →_ _Int_


If the father instance is of _OiPlElement_ type, the handling of the translation is delegated
to its _elemTranslation()_ function, otherwise to the _onTranslate()_ function of the implicit
instance.


_• ROTATE(pValue(Float)) →_ _Int_


If the father instance is of _OiPlElement_ type, the handling of the rotation is delegated to its
_elemRotation()_ function, otherwise to the _onRotate()_ function of the implicit instance.


152


#### **8.5 OiUtility**

**Description**


_•_ The _OiUtility_ type is the basic type for types that are used for specific tasks, e.g., for the
representation and storage of the global data of a program.


_•_ **Interface(s):** MObject


**Initialization**


_• OiUtility(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiUtility_ type.

#### **8.6 OiPropertyObj**


**Description**


_•_ The _OiPropertyObj_ type is the basic type for types that are used for specific tasks and feature
properties.


_•_ **Super type:** OiUtility


_•_ **Interface(s):** MObject (inherited), Property


**Initialization**


_• OiPropertyObj(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiPropertyObj_ type.


**Methods**


**General Methods**


_• getPlanning() →_ _MObject_


The function delivers the root object of the planning hierarchy ( _t_ ) if this is an instance of
_OiPlanning_, otherwise a value of the type _Void_ .


_• isCutable() →_ _Int_


See the function of the same name of the _Base_ interface.


_• removeValid() →_ _Int_


See the function of the same name of the _Base_ interface.


153


#### **8.7 OiOdbPlElement**

**Description**


_•_ The _OiOdbPlElement_ type is the basic type for planning elements whose geometries are
generated by the ODB.


_•_ **Super type:** OiPlElement


_•_ **Interface(s):** Base, Complex, Material, Property, Article


_•_ The most important function of the _OiOdbPlElement_ class consists of providing the ODB
information in form of a hash table returned by _getOdbInfo()_ . It contains an entry for the
ODB name and an additional entry for each property, where the property key is also used
as key in the hash table. Thus, the values of the properties are available in the ODB for the
parameterization of the geometries.


**Initialization**


_• OiOdbPlElement(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiOdbPlElement_ type analogous to _OiPlElement_ .
In addition, the translation in x- and z-direction is enabled and blocked in y-direction.


**Methods**


**General Methods**


_• setOdbType(pArticle(String)) →_ _Void_ (protected)


The function sets the ODB name on the basis of the transferred article. The ODB name is
determined by calling the _article2ODBType()_ method on the PD manager. If this method
does not deliver an ODB name, a name is generated by default. This name consists of the
series of the article and the article designation, where all characters in the article designation except for letters, numbers and underscore sign are replaced by ~~`X`~~ `X` . `XX` represents the
hexadecimal representation of the code of the respective character. The underscore sign is
replaced by two succeeding underscores.


_• setArticleSpec(pArticle(String)) →_ _Void_


The function assigns a new base article number to the implicit instance. This causes the
initialization of the ODB information and the subsequent generation of the geometries by
means of calling _createOdbChildren(@NEW)_ .


_• getArticleSpec() →_ _String_


The function delivers the name of the article (base article number) to which the implicit
instance corresponds or a value of type _Void_ if no article specification is available for the
implicit instance.


The name of the article is determined by means of the ODB name.


154


_• propsChanged(pPKeys(Symbol[]), pDoChecks(Int)) →_ _Int_


If the list of property keys _pPKeys_ is not empty, the _createOdbChildren(@INCR)_ function
is called to regenerate the geometries for this article. In the current implementation, the
_pDoChecks_ parameter is ignored and the return value is always 1.


_• setPropValue(pKey(Symbol), pValue(Any)) →_ _Int_


First, the _setPropValue()_ method of the _OiPlElement_ top class is called. Next, an iteration
is performed on all direct children of the implicit instance and the _setPropValue()_ method is
called for every child that is either an _OiPlElement_ or an _OiPart_ .


_• getOdbInfo() →_ _Hash_


The function returns a hash table with the ODB parameters. It contains the ODB name
determined from the base article number and the current property values.


_• createOdbChildren(pVal(Symbol)) →_ _Void_


The function controls the generation of the child objects via ODB. Dependent upon the
_pVal_ parameter, the child objects are either generated completely new ( _@NEW_ and _@RULE_ )
or adapted to the new ODB information ( _@INCR_ ). Usually, either _@NEW_ or _@INCR_ is
transferred as argument. The _@RULE_ argument is intended for use in the _FINISH_ ~~_E_~~ _VAL_
rule.


**Note:** The current implementation always performs a complete regeneration of the child objects.


With a regeneration of the child objects, those child objects that are not part of the article, such as

accessories, are deleted and not displayed again. Since the regeneration of child objects can lead to

random changes in the geometry of the article, a general discussion of this problem is not possible.


**Translation and Rotation**


_• translated(pOldPos(Float[3])) →_ _Int_


The function is called following every translation and checks whether the object at the current
position causes a collision. If this is the case, the function attempts to determine a new
position on the line between old and current position that is as close as possible to the
current position and on which the object does not collide.


155


## **Chapter 9**

# **Types for Product Data** **Management**

On principle, _OFML_ allows the complete description of logics and dependencies of types without
external data records. Still, a specification of product properties via external data records could
be desirable for various reasons, e.g., to be able to use an existing data array directly in _OFML_ .


For this purpose, _OFML_ defines a powerful, generic product data management interface. The
concept of a product data management (see also Figure 9.1) conceives that there are a number of
external product databases (possibly in different data formats), but they are managed by a global
product data manager and communicate with this manager via a uniform generic interface. For
each concrete data format (but not for each external product database), a special interface type
must be implemented (subtypes of _OiProductDB_ ) that takes over the interpretation of the data
format on the _OFML_ level.


Figure 9.1: Conceptual model of the product data management types


**Example:** A concrete example is the data format that is generated from a SAP/R3 system while main
taining the physical basic format. The data is distributed over several tables that are interlinked. The

relational knowledge is stored in expressions of the ABAP/4 language. Using the implementation of a

respective subtype of _OiProductDB_, this format can now be read in on the _OFML_ level. This includes the

implementation of an ABAP/4 parser.


156


#### **9.1 OiPDManager**

**Description**


_•_ An instance of the _OiPDManager_ type manages a set of external product databases ( _OiPro-_
_ductDB_ type) and allows access to the product data stored in these databases.


_•_ Exactly one instance of this type exists for each planning. This instance is referred to as
product data manager. It is generated by means of the _setPDManager()_ function of the
_OiPlanning_ type.


_•_ The product data manager also manages the assignment of types to articles and vice versa.


_•_ **Interface(s):** _MObject_


**Initialization**


_• OiPDManager(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiPDManager_ type.


**Methods**


**Management of the Product Databases**


_• addProductDB(pType(Type), pID(Symbol), pPath(String), pProgList(Symbol[]) →_ _MObject_


The function generates an instance of the transferred type (subtype of _OiProductDB_ ) and
registers it under the indicated ID. The file system path of the directory that contains the files
of the product database is transferred in the _pPath_ parameter (relative to the root directory
of the runtime environment). The additional _pProgList_ parameter specifies the programs
(IDs) that are represented in the database. If a product database is already registered under
the indicated ID, the list of programs of the product database is expanded, if required.


The return value is the reference to the (generated) product database instance.


_• delProductDB(pID(Symbol)) →_ _Void_


The function deletes and removes the product database with the indicated ID from the set
of registered product databases.


_• clearProductDBs() →_ _Void_


The function deletes and removes all registered product databases.


_• getPDB_ _IDs() →_ _Symbol[]_


The function delivers the IDs of all registered product databases.


_• getProductDB(pID(Symbol)) →_ _MObject_


The function delivers the product database instance that is registered under the indicated
ID or a value of type _Void_, if such an ID is not available.


157


_• getPDBFor(pObj(MObject)) →_ _MObject_


The function delivers the product database instance that is responsible for the transferred
planning element or a value of type _Void_, if such a product database is not found. The
responsibility results from the program association of the planning element.


_• getProgPDB(pPID(Symbol)) →_ _MObject_


The function delivers the product database instance that is responsible for the program
specified by the transferred ID or a value of type _Void_, if such a product database is not
found.


**Assignment of Types to Articles**


_• article2Class(pArticle(String)) →_ _String_


The function delivers the name of the type that models the article which was specified based
on its article number, or a value of the type _Void_ if no assignment could be found.


_• article2Params(pArticle(String)) →_ _String_


The function delivers the parameter values for the type that models the article specified by its
article number. The return value is a string that contains the presentation of the parameter
values stored in the product data. The function delivers a value of type _Void_ if no entry for
the article was found in the product data.


_• object2Article(pObj(MObject)) →_ _String_


The function delivers the name of the article (article number) to which the transferred planning element corresponds.


The assignment is composed of the program association, the immediate type, and the relevant
parameters (see _OiPlElement::getArticleParams()_ function) of the planning element.


_• class2Articles(pObj(MObject)) →_ _String[]_


The function delivers the list of article (numbers) that are represented by the class of transferred planning elements. The return value is a value of type _Void_ if no article assignment
for the class exists.


**Properties**


_• setupProps(pObj(MObject)) →_ _Void_


The function defines the initial properties for the indicated planning element based on the
product data for the article that corresponds to the type of the planning element and its
program association. The language currently selected in the global planning instance (see
_OiPlanning_ type) is used for designations (labels, values).


_• evalPropValue(pObj(MObject), pPKey(Symbol), pValue(Any), pOldValue(Any), pOldArti-_
_cle(String)) →_ _Int_


The function evaluates the relational knowledge in the product data after the property of
the indicated planning element that was specified by its key was set to the transferred new


158


value. In addition, the old property value and the base article number are transferred before
the value assignment. The evaluation of the value assignment can lead to changes of the
definition (value ranges) or current values of other properties of the planning element. In
this case, the function delivers 0, otherwise 1.


**Note:** The function is called from the _setPropValue()_ function of the _Property_ interface.


_• checkConsistency(pObj(MObject)) →_ _Int_


The function checks the correctness of the product data of the article that is represented by
the transferred instance. The global error log is used for error messages (see the function of the
same name of the _Article_ interface). The standard implementation delegates to the function
of the same name of the product database that is responsible for the article ( _OiProductDB_
type).


**Article Information**


_• getXArticleSpec(pObj(MObject), pType(Symbol)) →_ _String_


The function delivers the specification of the requested type for the article that is represented
by the transferred instance or a value of type _Void_ if no article specification of the required
type is available for the implicit instance. Semantics and return value of the function correspond to the function of the same name of the _Article_ interface, where only the @VarCode
and @Final specification types are allowed. The standard implementation of the function delegates the query with the @VarCode specification type to the _getVarCode()_ function and with
the @Final specification type to the _getFinalArticleSpec()_ function of the product database
that is responsible for the article instance ( _OiProductDB_ type), if such an instance exists.
Instead of the article instance, its base article number and a list of its current property values
are transferred.


_• setXArticleSpec(pObj(MObject), pType(Symbol), pSpec(String)) →_ _Void_


The function assigns a new article specification of the specified type to the transferred article
instance. Semantics and return value of the function correspond to the function of the same
name of the _Article_ interface, where only the @VarCode specification type is allowed. The
standard implementation of the function uses the _varCode2PValues()_ function of the product
database that is responsible for the article instance ( _OiProductDB_ type) to determine the
product properties that match the transferred variant code. If the obtained values differ
from the current values of the respective properties, they will be reassigned by means of the
_setPropValue()_ function ( _Property_ interface) of the transferred article instance.


_• getArticlePrice(pObj(MObject), pLanguage(String), ...) →_ _Any[]_


The function delivers price information for the transferred planning element in the specified
language. Semantics and return value of the function correspond to the function of the same
name of the _Article_ interface. The standard implementation of the function delegates the
query to the function of the same name of the product database that is responsible for the
planning element ( _OiProductDB_ type), if such an instance exists. Instead of the planning
element, its base article number and a list of its current property values are transferred.


159


_• getArticleText(pObj(MObject), pLanguage(String), pForm(Symbol)) →_ _String[]_


The function delivers describing article information for the transferred planning element
in the specified language and in the specified form. Semantics and return value of the
function correspond to the function of the same name of the _Article_ interface. The standard
implementation of the function delegates the query to the function of the same name of the
product database that is responsible for the planning element ( _OiProductDB_ type), if such
an instance exists. Instead of the planning element, its base article number is transferred.


_• getArticleFeatures(pObj(MObject), pLanguage(String)) →_ _Any_


The function delivers a description of the configurable product properties for the transferred
planning element in the specified language. Semantics and return value of the function
correspond to the function of the same name of the _Article_ interface. The standard implementation of the function delegates the query to the _getPropDescription()_ function of the
product database that is responsible for the planning element ( _OiProductDB_ type), if such
an instance exists. Instead of the planning element, its base article number and a list of its
current property values are transferred.

#### **9.2 OiProductDB**


**Description**


_•_ An instance of the _OiProductDB_ type manages exactly one product database and offers
services for access and evaluation of information about articles and their properties.


_•_ **Interface(s):** _MObject_


**Initialization**


_• OiProductDB(pFather(MObject), pName(Symbol), pID(Symbol))_


The function initializes an instance of the _OiProductDB_ type with the indicated ID. The ID
cannot be changed later.


**Methods**


**Article Configuration**


Some of the functions described below expect a _pPValues_ parameter that contains the current
article configuration. This parameter is a list that contains a vector made up of the following
elements for each product property:


1. the feature class ( _String_ or _Void_, unless relevant)


2. the (language-independent) designator of the feature ( _String_ )


160


3. the value of the feature ( _Any_ )


4. the list of the currently possible values ( _List_ or _Void_, unless relevant)


5. the activation state of the property that is assigned to the feature ( _Int_ )


**General Methods**


_• getID() →_ _Symbol_


The function delivers the ID of the product database.


_• setPrograms(pProgList(Symbol[])) →_ _Void_


The function assigns the number of programs (IDs) that are represented in the product
database to the implicit instance.


_• getPrograms() →_ _Symbol[]_


The function delivers the number of programs (IDs) that are represented in the product
database.


_• setDataBasePath(pDir(String)) →_ _Void_


The function assigns the root directory of the product data to the implicit instance.


_• getDataBasePath() →_ _String_


The function delivers the root directory of the product data.


_• getPDManager() →_ _MObject_


The function delivers the reference to the global product data manager.


**Features and Relational Knowledge**


_• hasProductKnowledge() →_ _Int_


The functions delivers True if the product database contains relational knowledge which must
be evaluated with a change of feature values.


_• getArticlePropClasses(pArticle(String)) →_ _Any_


The function delivers a list with feature classes to which the indicated article (base article
number) is assigned.


_• getPropDefs(pArticle(String), pPropOffset(Int), pLanguage(String), pChangedProp(Any[]),_
_pPValues(Vector[])) →_ _Any_


The function delivers the property definitions for all features of the transferred article (base
article number).


The _pPropOffset_ parameter specifies the number at which positions can be assigned for the
properties. The specified language (ISO code) is used for designations (label, values). If no
language is specified ( _Void_ ), English is used. If the _pChangedProp_ parameter is not a value
of type _Void_, it specifies a feature whose value was changed so that the function is called.


161


In this case the parameter contains a three-digit vector consisting of (language-independent)
designator of the feature, new and old feature value. The _pPValues_ parameter describes the
current article configuration (see above) or is a value of type _Void_ if the function is called for
an article that has not yet been initialized.


The return value is a list of seven-digit vectors. Each vector describes a feature and consists
of the following:


1. Feature class ( _String_ or _Void_, unless relevant).


2. (Language-independent) designation of the feature ( _String_ ).


3. Specification of the associated property ( _Any[5]_ ) in accordance with the _setupProperty()_
function of the _Property_ interface.


4. (Initial) value of the feature or _Void_ if no value is (pre)defined.


5. List of all possible values in so far as several values ( _List_ or _Void_ are defined for the
feature, unless relevant).
The entries are two-digit vectors that contain the value and the language-independent
description of the value. For optional features, the list must contain the value ”not
selected” which must be specified as `[` @ `VOID, "` @ `VOID"]` .


6. Position in the property list ( _Int_ ).


7. Activation status for the property ( _Int_ ).


_• checkConsistency(pArticle(String), pPValues(Vector[]), pLanguage(String), pErrorList(String[]))_
_→_ _Int_


The function delivers True if the transferred article configuration for the indicated article
(base article number) is correct from a product point of view. Error messages are attached
to the list that is transferred in the _pErrorList_ parameter. Here, the language specified in
the _pLanguage_ parameter is used.


**Article Information**


_• getVarCode(pArticle(String), pPValues(Any[]), ...) →_ _String_


The function delivers the variant code for the transferred article (base article number) and
the transferred article configuration. If an additional optional parameter is indicated, it
specifies whether the feature values contained in the article configuration are OFML values
of the associated property (True) or whether they are given in the form used by the product
database (False). Without any information, True is assumed.


_• varCode2PValues(pArticle(String), pVarcode(String)) →_ _Any[]_


The function delivers the feature values to the transferred variant code for the indicated
article (base article number).


The return value is a list that contains a vector consisting of the following elements for each
product feature:


1. the feature class ( _String_ or _Void_, unless relevant)


2. the (language-independent) designator of the feature ( _String_ )


162


3. the value of the feature ( _Any_ )


_• getFinalArticleSpec(pArticle(String), pPValues(Any[])) →_ _String_


The function delivers the final article number for the transferred article (base article number)
and the transferred article configuration.


_• getArticlePrice(pArticle(String), pPValues(Any[]), pLanguage(String), ...) →_ _Any[]_


The function delivers price information for the transferred article (base article number) and
the transferred article configuration in the specified language. If an additional optional
parameter is given, it specifies the desired currency.


The return value corresponds to the function of the same name of the _Article_ interface.


_• getArticleText(pArticle(String), pLanguage(String), pForm(Symbol)) →_ _String[]_


The function delivers the article description for the transferred article (base article number)
in the specified language and in the specified form. The _pForm_ parameter may take on the
following values:


**–** @ `short` short description


**–** @ `long` long description


The return value is a list of strings that contain the individual lines of the description or a
value of type _Void_ if no article description is available for the implicit instance.


_• getPropDescription(pArticle(String), pPValues(Any[]), pNeedSymbols(Int), pLanguage(String))_
_→_ _Any_


The function delivers a description of the transferred article configuration for the specified
article (base article number) in the specified language.


The return value is a list of two-digit vectors whose first element ( _String_ ) labels the feature,
while the second element contains the current value (as character string) of the feature. If
the _pLanguage_ parameter contains a value of type _Void_, language-independent designators
are furnished for feature and value.


If the _pNeedSymbols_ parameter has the value 1, the list entries consist of four-digit vectors
with the following fields in the indicated order:


1. language-independent symbol of the feature


2. language-independent designation of the feature


3. language-independent symbol of the current value of the feature


4. language-independent designation of the current value of the feature


The function delivers a value of type _Void_ if no descriptions for the features are available.


163


## **Chapter 10**

# **Types of the Planning** **Environment**

#### **10.1 The Wall Interface**

_Wall_ defines the interface of a wall or some of its parts (e.g., sides) for furniture planning.


_• getWallParams()) →_ _[Float, Float, Float[3]]_


The function delivers the geometric parameter to be able to place furniture at the wall in
the course of the planning process. The return value is a vector with three elements.


1. Width.


2. Rotary angle (in positive orientation about the y-axis).


3. Position (origin of the local coordinate system).

#### **10.2 OiLevel**


**Description**


_• OiLevel_ models one story of a building that can consist of one or several rooms.


_•_ **Interface(s):** Base, Complex


**Initialization**


_• OiLevel(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiLevel_ type.


164


**Methods**


_• setDefaultHeight(pHeight(Float)) →_ _Void_


The function sets a default for the height of walls to be created. This value is effective only
if there are no walls on the story, otherwise the height of the planning wall (see below) is
used as default.


_• Complex::getHeight() →_ _Float_


The function delivers the maximum wall height within a story or, if no walls exist, the
specified height.


_• setPlanningWall(pWall(MObject)) →_ _MObject_


The function selects a wall to which furniture is to be added in the following. _pWall_ must be
an object that implements the _Wall_ interface (Section 10.1). _NULL_ is allowed as a special
value for _pWall_ . In this case, a possibly existing setting is deleted. The function delivers the
new planning wall as return value.


_• getPlanningWall() →_ _MObject_


The function delivers the specified planning wall (see below). If no planning wall was explicitly set, the wall generated last is used.


_• setPlanningMode(pMode(Int)) →_ _Void_


The function sets the planning mode. As a minimum requirement, the values 0 (activates furniture planning) and 1 (switches to the base mode of floor space planning) must be detected.
Values _>_ 1 are acceptable depending upon the implementation.


_• Complex::checkAdd(pType(Type), pObj(MObject), pPos(Float[3]), pParams(Float[])) →_ _Float[3]_


The function checks the insertion of a new wall. _pType_ must be a subtype of OiWall. _pObj_
must be instance of a subtype of _OiWall_ to which the new wall should be attached. If _NULL_
is transferred for _pObj_, attaching is performed at the preset planning wall. (Section 10.2).
_pPos_ is ignored and may be _NULL_ . _pParams_ is _NULL_ or contains an optional list of default
parameters. If available, these parameters are interpreted as follows:


1. Width.


2. Attaching angle.


3. Thickness.


If the given parameters can be inserted, the attaching position is returned, otherwise _NULL_ .


_• objInLevel(pObj(MObject)) →_ _Int_


The function delivers 1 if the _pObj_ object is located within the story, otherwise 0. Simplified
tests (e.g., limitation to the surrounding rectangle or bounding box) are possible, and collision
must not be observed.


165


#### **10.3 OiWall**

**Description**


_• OiWall_ represents a wall as a component of a story. This may be an outside wall or a dividing
wall. Windows, doors, etc. can be inserted in a wall as children.


_•_ **Interface(s):** Base, Complex, Properties, Material, Wall


**Initialization**


_• OiWall(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiWall_ type.

#### **10.4 OiWallSide**


**Description**


_•_ A single side of a wall at which furniture can be placed in the course of the planning process.


_•_ **Interface(s):** Base, Properties, Material, Wall


**Initialization**


_• OiWallSide(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _OiWallSide_ type.


166


## **Appendix A**

# **Product Data Model**

This appendix describes the underlying data model for the types of the product data management
(Chapter 9). Figure A.1 shows a graphic representation of the model which illustrates the major
concepts and terms [1] .

















|Feature<br>features Name rel.knowledge<br>Article class Feature class Simple Configurable<br>feature feature<br>Name<br>rel.knowledge rel.knowledge<br>Article number<br>Kind price condition value Feature value values<br>Value default<br>value<br>rel.knowledge<br>Set<br>order list Relation<br>Kind<br>Price condition Coding|Feature|rel.knowledge|
|---|---|---|
|**Set**<br>Name<br>features<br>Value<br>Name<br>**Feature value**<br>value<br>value<br>Kind<br>**Feature class**<br>order list<br>**Article**<br>**Feature**<br>price condition<br>rel.knowledge<br>rel.knowledge<br>class<br>Article number<br>default<br>rel.knowledge<br>rel.knowledge<br>values<br>**Configurable**<br>**feature**<br>**Simple**<br>**feature**<br>**Price condition**<br>**Relation**<br>Kind<br>Coding|Name|Name|
|**Price condition**|**Price condition**|**Price condition**|


Figure A.1: Product data model













1The notational conventions used here is explained in Appendix G.


167


**Additional remarks and explanations**


Each article is assigned to a certain article type that specifies which actions are allowed for the
article or which meaning certain model properties have. The major article types are ”configurable
article,” ”assembly unit” and ”commercial article.”


Features describe the properties of articles and are combined to feature classes. A feature in a
class can be another class. Each article is assigned one or several feature classes.


Price terms contain the definitions for the base price as well as extra charges and discounts for
configurable articles via variant terms. Relational knowledge must be used to establish the relation
to the corresponding features or feature values.


Relational knowledge is shown by means of five types of relations:


_• Conditions_
determine whether a feature may be evaluated or whether a feature value may be set.


_• Selection criteria_
specify that a feature must be evaluated or that a parts list position must be selected.


_• Actions_ and _Procedures_
serve for derivation of feature values and are executed if a feature value is selected or a feature
is evaluated. For this purpose, actions have declarative character and are independent of the
order of the evaluation. Procedures, on the other hand, implement more complex algorithms
and are executed only at certain times.


_• Constraints_
serve for monitoring the consistence of a configuration and, therefore, can only be bound to
a configurable article via the configuration profile.


168


## **Appendix B**

# **The 2D Interface**

#### **B.1 Introduction**

The 2D interface described in this chapter allows for programming of 2D objects. Altogether, the
generation of 2D objects in OFML can be accomplished in the following ways:


_•_ through generation based on a (3D) OFML geometry,


_•_ through description via the OFML database [ODB],


_•_ through import of an external 2D vector data record (Chapter C), and


_•_ through programming.


A specialty of this 2D programming interface is the fact that the generated 2D objects cannot be
stored persistently. Thus, they must be restored in the appropriate rules (Chapter 5), if required.

#### **B.2 The 2D Object Hierarchy**


The 2D objects are generally arranged in a tree where the nodes of the tree are of the `G2DCompound`
type and the leaves are consequently of a type derived from `G2DPrimitive` . The root of the tree
is always bound to an OFML object that supports the _MObject_ interface so that each 2D object
can directly or indirectly be assigned to an _MObject_ object.


From OFML, the 2D objects are referenced via integer ID’s. An _MObject_ object is not assigned
two 2D objects with the same ID, that is, the ID’s below an _MObject_ object are unique. Assigned
ID’s do not grow monotonously, that is, a new object can receive the ID of an old object that was
deleted.


The object with the ID 0 always exists [1] and is of the `G2DCompound` type.


1In fact, it is generated if required.


169


#### **B.3 Coordinates**

All coordinates are indicated in the rectangular X/Y coordinate system, where the positive X
axis points to the right and the positive Y axis up. In principle, angular dimensions are radiant
measures and mathematically positive (counterclockwise). The zero angle shows in the direction
of the positive X axis.

#### **B.4 Methods**


The manipulation of 2D objects is carried out via the methods listed in the following subsections.


**B.4.1** `new2DObj`


`t.new2DObj(` _parent_ _id_ `,` _object_ ~~_t_~~ _ype_ `, ...)`


All 2D objects are generated with the `new2DObj` method. Their first argument is the ID of the
father object which must be of the `G2DCompound` type. The second argument is a symbol which
determines the type of 2D object to be generated. The remaining arguments are dependent upon
the type of the object to be generated. The return value is the ID of the newly generated object.


The exact form of the calls of `new2DObj` is described in the section B.5 in the respective objectspecific subsections.


**B.4.2** `delete2DObj`


`t.delete2DObj(` _obj_ ~~_i_~~ _d_ `)`


The `delete2DObj` method removes the object with the indicated ID and, if required, recursively
all existing child objects of this object.


**B.4.3** `set2DObjAttr`


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `,` _attr_ ~~_t_~~ _ype_ `, ...)`


The `set2DObjAttr` method sets the attributes of existing objects. The first argument is the ID of
the object of which an attribute is to be set. The second argument is a symbol which determines
the type of the attribute to be set. The remaining attributes are dependent upon the attribute
type.


The exact form of the calls of `set2DObjAttr` is described in the B.6 section in the respective
attribute-specific subsections.


170


**B.4.4** `translate2DObj`


`t.translate2DObj(` _obj_ ~~_i_~~ _d_ `, [` _x_ `,` _y_ `])`


The `translate2DObj` method moves the `G2DCompound` object with the ID _obj_ ~~_i_~~ _d_ relative to the
current position in the coordinate system of the father object by _x_ ; _y_ .


**B.4.5** `rotate2DObj`


`t.rotate2DObj(` _obj_ ~~_i_~~ _d_ `,` _angle_ `)`


The `rotate2DObj` method rotates the `G2DCompound` object with the ID _obj_ ~~_i_~~ _d_ relative to the current
rotation by the angle _angle_ . The rotation is carried out around the origin of the coordinate system
of the father object.

#### **B.5 Object Types**


The following subsections list the available 2D object types with their attributes. Besides the
specified attributes, every type has the _Pickable_ and _Snapable_ attribute.


**B.5.1** `G2DCompound`


A `G2DCompound` object differs from all other objects that are derived from `G2DPrimitive` in that
it


_•_ can have additional 2D objects as children and


_•_ can be translated, rotated and scaled.


A new `G2DCompound` object is generated with the method


`t.new2DObj(` _parent_ _id_ `, @COMPOUND)`


**B.5.2** `G2DPoints`


An object of the `G2DPoints` type consists of a list o f X/Y coordinates that describe the center of
the individual points. In addition, it features the attributes _Color_, _PointSize_, and _PointSmooth_ .


A new `G2DPoints` object with _n_ points is generated with the method


`t.new2DObj(` _parent_ _id_ `, @POINTS, [[` _x_ 0 `,` _y_ 0 `], ..., [` _xn−_ 1 `,` _yn−_ 1 `]])`


171


**B.5.3** `G2DLines`


An object of the `G2DLines` type consists of individual line segments that are defined in the X/Y coordinate system by means of their start and end points. It features the attributes _Color_, _LineWidth_,
and _LineStyle_ .


A new `G2DLines` object with _n_ lines is generated with the method


`t.new2DObj(` _parent_ _id_ `, @LINES, [[` _x_ 0 `,` _y_ 0 `], ..., [` _x_ 2 _n−_ 1 `,` _y_ 2 _n−_ 1 `]])`


where _x_ 2 _i_ ; _y_ 2 _i_ specifies the start point and _x_ 2 _i_ +1; _y_ 2 _i_ +1 the end point of a line.


**B.5.4** `G2DLineStrip`


An object of the `G2DLineStrip` type consists of a series of at least two points that are connected
with each other by means of individual line segments in the specified order where, in contrast with
`G2DLineLoop`, the last point is not connected with the first point. It features the attributes _Color_,
_LineWidth_, and _LineStyle_ .


A new `G2DLineStrip` object with _n_ points is generated with the method


`t.new2DObj(` _parent_ _id_ `, @LINE` ~~`S`~~ `TRIP, [[` _x_ 0 `,` _y_ 0 `], ..., [` _xn−_ 1 `,` _yn−_ 1 `]])`


**B.5.5** `G2DLineLoop`


An object of the `G2DLineLoop` type consists of a series of at least two points that are connected
with each other by means of individual line segments in the specified order where, in contrast
with `G2DLineStrip`, the last point is also connected with the first point. It features the attributes
_Color_, _LineWidth_, and _LineStyle_ .


A new `G2DLineLoop` object with _n_ points is generated with the method


`t.new2DObj(` _parent_ _id_ `, @LINE` ~~`L`~~ `OOP, [[` _x_ 0 `,` _y_ 0 `], ..., [` _xn−_ 1 `,` _yn−_ 1 `]])`


**B.5.6** `G2DConvexPolygon`


An object of the `G2DConvexPolygon` type is described by a series of at least three points that must
result in a convex polygon when connected with each other, including the last point with the first
point. It features the attributes _Color_, _PointSize_, _PointSmooth_, _LineWidth_, _LineStyle_, _FillStyle_,
and _PolygonMode_ . Depending upon _PolygonMode_, only one subset of the attributes is used in each
case.


A new `G2DConvexPolygon` object with _n_ corner points is generated with the method


`t.new2DObj(` _parent_ _id_ `, @POLYGON, [[` _x_ 0 `,` _y_ 0 `], ..., [` _xn−_ 1 `,` _yn−_ 1 `]])`


172


**B.5.7** `G2DRectangle`


An object of the `G2DRectangle` type is described by two cornerpoints lying diagonally opposite each
other. It features the attributes _Color_, _PointSize_, _PointSmooth_, _LineWidth_, _LineStyle_, _FillStyle_,
and _PolygonMode_ . Depending upon _PolygonMode_, only one subset of the attributes is used in each
case.


A new `G2DRectangle` object is generated with the method


`t.new2DObj(` _parent_ _id_ `, @RECTANGLE, [[` _x_ 0 `,` _y_ 0 `], [` _x_ 1 `,` _y_ 1 `]])`


where the rectangle has the four cornerpoints _x_ 0; _y_ 0, _x_ 0; _y_ 1, _x_ 1; _y_ 0 and _x_ 1; _y_ 1.


**B.5.8** `G2DText`


An object of the `G2DText` type consists of an ASCII text positioned relative to a reference point.
It features the attributes _Color_, _Text_, _Position_, _Height_, _AspectRatio_, and _Alignment_ .


A new `G2DText` object is generated with the method


`t.new2DObj(` _parent_ _id_ `, @TEXT, [` _x_ `,` _y_ `],` _text_ `)`


where _x_ ; _y_ is the position and _text_ a character string with the text to be represented.


**B.5.9** `G2DArc`


An object of the `G2DArc` type represents the arc of a circle that is described by means of its center,
radius, start and end angle. The circle segment is drawn in mathematically positive direction of
rotation from start to end angle. `G2DArc` objects feature the attributes _Color_, _LineWidth_, and
_LineStyle_ .


A new `G2DArc` object is generated with the method


`t.new2DObj(` _parent_ _id_ `, @ARC, [` _xcenter_ `,` _ycenter_ `],` _radius_ `,` _start_ `,` _end_ `)`


Using the method


`t.new2DObj(` _parent_ _id_ `, @CIRCLE, [` _xcenter_ `,` _ycenter_ `],` _radius_ `)`


generates the special case of an arc of a circle where _start_ = 0 and _end_ = 2 _π_ .


**B.5.10** `G2DEllipse`


An object of the `G2DEllipse` type represents an ellipse that is described by means of its center,
radius in x and y direction, and rotary angle about the center.


A new `G2DEllipse` object is generated with the method


`t.new2DObj(` _parent_ _id_ `, @ELLIPSE, [` _xcenter_ `,` _ycenter_ `], [` _xradius_ `,` _yradius_ `],` _angle_ `)`


173


#### **B.6 Attributes**

The following subsections describe the attributes supported for 2D objects.


**B.6.1** _**Color**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @COLOR, [` _red_ `,` _green_ `,` _blue_ `])`


sets the color of the object specified by the ID _obj_ ~~_i_~~ _d_ to the RGB value _red_ ; _green_ ; _blue_, where
the three color components must be given as floating point values in the interval [0 _._ 0 _,_ 1 _._ 0].


**B.6.2** _**PointSize**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @POINT` ~~`S`~~ `IZE,` _point_ _size_ `)`


sets the size of a point to the floating point value _point_ ~~_s_~~ _ize_ . For the screen display, a value of
1 _._ 0 corresponds to a point size of one pixel. For printout, 1 _._ 0 should correspond to a size of 1pt
(1 _/_ 72in).


The standard value of the point size is 1 _._ 0


**B.6.3** _**PointSmooth**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @POINT` ~~`S`~~ `MOOTH,` _smooth_ `)`


specifies for points whose size is not 1 _._ 0 whether the point should be represented as a square
( _smooth_ = 0) or as a filled circle with a smooth edge ( _smooth ̸_ = 0). This setting is only relevant
for screen display with OpenGL. The screen display with other drivers or the printout can ignore
the _PointSmooth_ flag if the point is always represented as a filled circle.


The standard value for the _PointSmooth_ flag is 0.


**B.6.4** _**LineWidth**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @LINE` ~~`W`~~ `IDTH,` _line_ ~~_w_~~ _idth_ `)`


sets the line width to _line_ _width_ pixels (screen display) or points (1pt = 1 _/_ 72in) (print). _line_ ~~_w_~~ _idth_
is given as floating point value.


The standard value for the line width is 1 _._ 0.


174


**B.6.5** _**LineStyle**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @LINE` ~~`S`~~ `TYLE,` _factor_ `,` _pattern_ `)`


sets the line style. In this case, _pattern_ is a symbol that can accept the values listed in table B.1.

|Sample|Description|
|---|---|
|`@DEFAULT`|The preset line style is used.|
|`@SOLID`|A continuous line is drawn.|
|`@DASHED`|A dashed line is drawn. The_ factor_ factor determines the length of<br>the line segments displayed and not displayed.|
|`@DOTTED`|A dotted line is drawn. The_ factor_ factor determines the distance<br>between the centers of two neighboring points.|
|`@DASH`~~` D`~~`OTTED`|A dot-dash line is drawn. The_ factor_ factor determines the length of<br>the displayed line segment and half the length of the non-displayed<br>line segments.|
|`@DASH`~~` D`~~`OUBLE` `DOTTED`|A dash double-dotted line is drawn. The_ factor_ factor determines<br>the length of the displayed line segment and one-third the length of<br>the non-displayed segments.|
|`@DASH`~~` T`~~`RIPLE` `DOTTED`|A dash triple-dotted line is drawn. The_ factor_ factor determines the<br>length of the displayed line segment and on-fourth the length of the<br>non-displayed segments.|



Table B.1: Line styles


The _factor_ factor is given as floating point value. Its unit is one pixel for screen display and a dot
(1pt = 1 _/_ 72in) for the printout.


The standard value for _factor_ is 4 _._ 0 and for _pattern_ `@DEFAULT` .


**B.6.6** _**FillStyle**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @FILL` ~~`S`~~ `TYLE,` _style_ `)`


sets the fill pattern for polygons. In this case, _style_ is a symbol that can accept the values listed
in table B.2.


The horizontal distance between diagonal or vertical lines in the fill pattern or the vertical distance
between horizontal lines should measure eight pixels for the screen display and eight points (1pt =
1 _/_ 72in) for the printout.


By default, a polygon is shown completely filled.


175


|Fill pattern|Description|
|---|---|
|`@LEFT`~~` 3`~~`0`|diagonal lines from top left to bottom right at an angle of 30 degrees<br>to the horizontal|
|`@RIGHT`~~` 3`~~`0`|diagonal lines from bottom left to top right at an angle of 30 degrees<br>to the horizontal|
|`@CROSS`~~` 3`~~`0`|crossing diagonal lines from top left to bottom right and from bottom<br>left to top right, both at an angle of 30 degrees to the horizontal|
|`@LEFT`~~` 4`~~`5`|diagonal lines from top left to bottom right at an angle of 45 degrees<br>to the horizontal|
|`@RIGHT`~~` 4`~~`5`|diagonal lines from bottom left to top right at an angle of 45 degrees<br>to the horizontal|
|`@CROSS`~~` 4`~~`5`|crossing diagonal lines from top left to bottom right and from bottom<br>left to top right, both at an angle of 45 degrees to the horizontal|
|`@H`~~` L`~~`INES`|horizontal lines|
|`@V`~~` L`~~`INES`|vertical lines|
|`@CROSS`|crossing horizontal and vertical lines|


Table B.2: Fill pattern


**B.6.7** _**PolygonMode**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @POLYGON` ~~`M`~~ `ODE,` _mode_ `)`


and the _mode_ parameter specifies for polygons whether the corner points ( _mode_ = `@POINT` ), the
boundary lines ( _mode_ = `@LINE` ) or the area ( _mode_ = `@FILL` ) should be displayed. The attributes
used dependent on _PolygonMode_ are listed in table B.3.

|PolygonMode|attributes used|
|---|---|
|`@POINT`|_Color, PointSize, PointSmooth_|
|`@LINE`|_Color, LineWidth, LineStyle_|
|`@FILL`|_Color, FillStyle_|



Table B.3: Attributes used dependent on _PolygonMode_


The standard value for _PolygonMode_ is `@FILL` .


**B.6.8** _**Text**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @TEXT,` _text_ `)`


sets the _text_ text to be displayed for an object of the `G2DText` type. The _text_ argument must be
a character string whose characters are from the ASCII character set.


176


**B.6.9** _**Position**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @POSITION, [` _x_ `,` _y_ `])`


sets the position (the reference point) for an object of the `G2DText` type.


**B.6.10** _**Height**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @HEIGHT,` _height_ `)`


sets the height of a capital letter without descender for an object of the `G2DText` type.


The standard value for the text height is 0 _._ 1.


**B.6.11** _**AspectRatio**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @ASPECT` ~~`R`~~ `ATIO,` _ratio_ `)`


determines the extension or compression in x direction for an object of the `G2DText` type. The
floating point value of _ratio_ must be greater than 0 _._ 0. With the default value of 1 _._ 0, the text
appears within normal proportions that are predefined by the font used. With values greater
than 1 _._ 0, it is extended in the x direction, with values smaller than 1 _._ 0 and greater than 0 _._ 0 it is
compressed in the x direction.


**B.6.12** _**Alignment**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @ALIGNMENT,` _align_ `)`


determines the alignment of the text relative to the reference point for an object of the `G2DText`
type.


If _width_ is the width of the text, the move ∆ _x_ of the left side of the first letter on the base line
relative to the reference point is determined as follows:


∆ _x_ = _−_ ( _alignment_ + 1 _._ 0) _×_ ( _width/_ 2 _._ 0)


The standard value for _align_ is _−_ 1 _._ 0, which left-aligns the text.


177


**B.6.13** _**Pickable**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @PICKABLE,` _pickable_ `)`


can specify for each object whether it should be considered in 2D mode in determining the object
that is to be selected with the mouse. If the integer argument _pickable_ is zero, the corresponding
2D object is not considered; if it is not zero, it is considered.


If the _pickable_ flag of an object of the `G2DCompound` type is not set, none of the child objects is
taken into account. If it is set, then the _pickable_ flag of the respective child object is decisive.


The standard value for _pickable_ is 1.


**B.6.14** _**Snapable**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @SNAPABLE,` _snapable_ `)`


can determine for each object whether it should be taken into account in determining a trap point.
If the integer argument _snapable_ is zero, the corresponding 2D object is not taken into account. If
it is not zero, it is taken into account.


If the _snapable_ flag of an object of the `G2DCompound` type is not set, none of the child objects is
taken into account. If it is set, then the _snapable_ flag of the respective child object is decisive.


The standard value for _snapable_ is 1.


**B.6.15** _**Exportable**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @EXPORTABLE,` _exportable_ `)`


can determine for each object whether it should be exported during the export to a 2D vector
format [2] . If the integer argument _exportable_ is zero, the corresponding 2D object is not exported.
If it is not zero, it is exported.


If the _exportable_ flag of an object of the `G2DCompound` type is not set, none of the child objects is
exported. If it is set, then the _exportable_ flag of the respective child object is decisive.


The standard value for _exportable_ is 1.


2Since the print command also uses the export to a 2D vector format, the _exportable_ flag also influences the
objects appearing during printout.


178


**B.6.16** _**Layer**_


Using the call


`t.set2DObjAttr(` _obj_ ~~_i_~~ _d_ `, @LAYER,` _layer_ `)`


can determine the layer for each object which is used to control its visibility. Here, the _layer_
argument is a symbol that should solely consist of letters, numbers and underscore sign.


If the layer was set for an object of the `G2DCompound` type, this layer is used for all direct and
indirect child objects for which a layer was not explicitly set.


179


## **Appendix C**

# **The 2D vector file format**

#### **C.1 Introduction**

The EasternGraphics Metafile (EGM) is an expandable file format for saving graphic and nongraphic data. Due to its expandability it allows the integration of data in other file formats.


The EGM format is structured in such a way that an EGM parser doesn’t have to be able to
interpret all EGM elements to read until the end of the file. It should be able to generally ignore
unknown elements.


The EGM format supports hierarchical saving of data. This makes it possible, for example, to save
graphic 2D symbols on the top level as well as 2D symbols embedded in a scene element within
an EGM. The first case is used when EGM describes only a single 2D symbol for use in the GF.
The second case may become interesting when a whole scene is described in EGM format which,
among other things, also contains user-defined 2D symbols.


EGM is specified in binary as well as text format. Binary format helps to save data efficiently
while text format may be used primarily during the developmental phase.

#### **C.2 data types**


This section describes the data types used in EGM, in particular their representation in EGM
binary and text format.


Both formats – binary and text – are defined to be platform-independent making easy data exchange between different computer platforms possible.


All numeric values are saved in binary format so that the byte with the highest value comes first,
followed by all other bytes in descending value order. This is also known as ”Network Byte Order”
or ”Big-Endian.”


The text format character set must be an aggregate of the ASCII character set. This is the case
for most, if not all, ISO–8859–x character sets. To enable easy conversion between text format and
binary format, the same limitation also applies to the binary format _String_ data type.


180


**C.2.1** **Simple Data Types**


**Byte**


A _Byte_ is an integer 8 bit value that can be interpreted either as an unsigned value in the 0 to
2 [8] _−_ 1 ([0 _,_ 255]) range or as a two’s complement signed value in the _−_ 2 [7] to 2 [7] _−_ 1 ([ _−_ 128 _,_ 127])
range. Type _Byte_ values are saved in EGM binary format as a single byte, as shown in figure C.1,
with _I_ 7 being the highest value bit and _I_ 0 the lowest value bit.


Byte 1 _I_ 7 _I_ 0


Figure C.1: Type _Byte_ values


In text format, a _Byte_ is displayed as a decimal, octal, or hexadecimal number, which can optionally
be preceded by a minus sign. An octal number is designated by a leading `0` and a hexadecimal
number by a leading `0x` or `0X`, with only digits ranging from `0` to `7` being permitted for octal
numbers and digits ranging from `0` to `9`, `A` to `F`, and `a` to `f` being permitted for hexadecimal
numbers.


In this EGM specification, the UINT8 identifier is used as a type specification for unsigned _Byte_
values; for signed _Byte_ values, the INT8 identifier is used.


**Word**


A _Word_ is an integer 16 bit value that is either interpreted as an unsigned value in the 0 to 2 [16] _−_ 1
([0 _,_ 65535]) range or as a two’s complement signed value in the _−_ 2 [15] to 2 [15] _−_ 1 ([ _−_ 32768 _,_ 32767])
range. Type _Word_ values are saved in EGM binary format as two successive bytes as shown in
figure C.2, with _I_ 15 being the highest value bit and _I_ 0 the lowest value bit.



_I_ 8



Byte 1 _I_ 15 _I_ 8
Byte 2 _I_ 7 _I_ 0



_I_ 7



Figure C.2: Type _Word_ values


In text format, a _Word_ is displayed as a decimal, octal or hexadecimal number, which can optionally
be preceded by a minus sign. An octal number is designated by a leading `0` and a hexadecimal
number by a leading `0x` or `0X`, with only digits ranging from `0` to `7` being permitted for octal
numbers and digits ranging from `0` to `9`, `A` to `F`, and `a` to `f` being permitted for hexadecimal
numbers.


In this EGM specification, the UINT16 identifier is used as a type specification for unsigned _Word_
values and the INT16 identifier is used for signed _Word_ values.


181


**Double Word**


A _Double Word_ is an integer 32 bit value that is interpreted as an unsigned value in the 0 to
2 [32] _−_ 1 ([0 _,_ 4294967295]) range or as a two’s complement signed value in the _−_ 2 [31] to 2 [31] _−_ 1
([ _−_ 2147483648 _,_ 2147483647]) range. Type _Double Word_ values are saved in EGM binary format as
four successive bytes as shown in figure C.3, with _I_ 31 being the highest value bit and _I_ 0 the lowest
value bit.



_I_ 31 _I_ 24

_I_ 23 _I_ 16

_I_ 15 _I_ 8



Byte 1 _I_ 31
Byte 2 _I_ 23
Byte 3 _I_ 15
Byte 4 _I_ 7



_I_ 0



Figure C.3: Type _Double Word_ values


In text format, a _Double Word_ is displayed as a decimal, octal or hexadecimal number that
is optionally preceded by a minus sign. An octal number is designated by a leading `0` and a
hexadecimal number by a leading `0x` or `0X`, with only digits ranging from `0` to `7` being are permitted
for octal numbers and digits ranging from `0` to `9`, `A` to `F`, and `a` to `f` being permitted for hexadecimal
numbers.


In this EGM specification, the UINT32 identifier is used as a type specification for unsigned _Double_
_Word_ values and the INT32 identifier is used for signed _Double Word_ values.


**Single Precision Floating Point**


Type _Single Precision Floating Point_ values are displayed according to the IEEE 754 standard.
The absolute value lies in the range between 1 _._ 17549435 _×_ 10 _[−]_ [38] and 3 _._ 40282347 _×_ 10 [38] with a
minimum of 6 significant decimals in the mantissa. In EGM binary format, single precision floating
point values are saved as four successive bytes as displayed in figure C.4.



Byte 1

Byte 2

Byte 3

Byte 4








|S|E E<br>7 1|
|---|---|
|_E_0|_F_1<br>_F_7|
|_F_8<br>_F_15|_F_8<br>_F_15|
|_F_16<br>_F_23<br><br>|_F_16<br>_F_23<br><br>|



Figure C.4: Single precision floating point number


The value is 0 _._ 0, if the exponent and the mantissa are 0. Otherwise it is calculated according to
( _−_ 1) _[s]_ _×_ 1 _.f ×_ 2 _[e][−]_ [127] . _S_ is the sign bit, _f_ is the mantissa ( _F_ 1 _. . . F_ 23 with _F_ 1 being the highest
value bit) and _e_ is the exponent ( _E_ 7 _. . . E_ 0 with _E_ 7 being the highest value bit).


In text format, a single precision floating point number consists of an optional leading minus or
plus sign, an integer decimal quantity, a decimal point, a fractional quantity, and an optional


182


exponent. The exponent consists of one of the numbers `e` or `E`, followed by an optional minus
or plus sign which in turn is followed by an integer decimal . The integer or fractional quantity
can be dropped, but both cannot. The decimal point can be dropped, if the fractional quantity is
dropped with an exponent present.


In this EGM specification, the FLOAT32 identifier is used as a type specification for single precision
floating point values.


**Double Precision Floating Point**


Type _Double Precision Floating Point_ values are displayed according to the IEEE 754 standard.
The absolute value lies in the range between 2 _._ 2250738585072014 _×_ 10 _[−]_ [308] and 1 _._ 7976931348623157 _×_
10 [308] with a minimum of 15 significant decimals in the mantissa. In EGM binary format, double
precision floating point values are saved as eight successive bytes as displayed in figure C.5.



Byte 1

Byte 2

Byte 3

Byte 4







Byte 5

Byte 6

Byte 7

Byte 8



_F_ 21 _F_ 28
_F_ 29 _F_ 36
_F_ 37 _F_ 44



_F_ 45





_F_ 52


|S|E E<br>10 4|Col3|
|---|---|---|
|_E_0<br>_E_3|_E_0<br>_E_3|_F_1<br><br>_F_4|
|_F_5<br>_F_12|_F_5<br>_F_12|_F_5<br>_F_12|
|_F_13<br>_F_20|_F_13<br>_F_20|_F_13<br>_F_20|



Figure C.5: double precision floating point


The value is 0 _._ 0, if the exponent and the mantissa are 0. Otherwise it is calculated according to
( _−_ 1) _[s]_ _×_ 1 _.f ×_ 2 _[e][−]_ [1023] . _S_ is the sign bit, _f_ is the mantissa ( _F_ 1 _. . . F_ 52 with _F_ 1 being the highest
value bit) and _e_ is the exponent ( _E_ 10 _. . . E_ 0 with _E_ 10 being the highest value bit).


The display of a double precision floating point number in text format corresponds to the display
of a single precision floating point number as described above.


In this EGM specification, the FLOAT64 identifier is used as a type specification for double precision floating point values.


**Symbol**


A symbol is a series of characters and is saved in EGM binary format, as shown in figure C.6. The
length ( _U_ 15 _. . . U_ 0 with _U_ 15 being the highest value bit) that is saved in the first two bytes as an
unsigned value neither includes itself nor the NUL character terminating the symbol.


In text format, a symbol consists of a series of ASCII letters, digits and underscores; the first
character must not be a digit. This symbol is case sensitive.


In this EGM specification, the SYMBOL identifier is used as a type specification for symbols.


**String**


A string is saved in EGM binary format, as shown in figure C.7. The length ( _U_ 15 _. . . U_ 0 with _U_ 15
being the highest value bit) that is saved in the first two bytes as an unsigned value neither includes
itself or the NUL character terminating the string.


183


Byte 1 _U_ 15 _U_ 8
Byte 2 _U_ 7 _U_ 0

Byte 3 _C_ 7 _C_ 0



Byte 3 + _n_



0 0 0 0 0 0 0 0


Figure C.6: Symbol



Byte 1 _U_ 15 _U_ 8
Byte 2 _U_ 7 _U_ 0

Byte 3 _C_ 7 _C_ 0



Byte 3 + _n_



0 0 0 0 0 0 0 0



Figure C.7: Character string


In EGM text format, character strings are displayed as a result of any number of characters
(including none) that are surrounded by quotation marks. Non-printable characters are represented
by escape sequences that can be used in the applications listed in table C.1. Please note that the
octal coding represented in parentheses can vary between platforms. For example, in MacOS(R)

the coding of `\n` is `\r` exchanged.


`\a` Klingelzeichen ( `\7` ) `\\` backward slash ( `\` )
`\b` backspace ( `\8` ) `\?` question mark ( `?` )
`\f` form feed ( `\14` ) `\’` apostrophe ( `’` )
`\n` line separator ( `\12` ) `\"` quotation marks ( `"` )
`\r` carriage return ( `\15` ) `\` _ooo_ octal number
`\t` tab character ( `\9` ) `\x` _hh_ hexadecimal number
`\v` vertical tabulation character ( `\13` )


Table C.1: escape sequences for character strings


In the escape sequence `\` _ooo ooo_ stands for a series consisting of one to three octal digits (0 _. . ._ 7) and
in `\x` _hh hh_ stands for a series consisting of one or more hexadecimal digit (0 _. . ._ 9 _,_ A _. . ._ F _,_ a _. . ._ f).
You should preferably use the format `\` _ooo_ with three octal digits, since only then a correct coding
of the character can be ensured without any consideration for the subsequent character.


In this EGM specification, the STRING identifier is used as a type specification for character
strings.


184


**C.2.2** **Structured data types**


Structured data types consist of a series of simple data types. Each structured data type is
described using a combination consisting of type class and object type. The type class is used for
classifying object types; the object types are used for defining the structure and the meaning of
structured data types. For example, a type class can combine all object types defined for describing
graphic 2D primitives, with each object type describing the structure of the respective 2D object
within the EGM format.


In the binary format, every structured data type consists of the structure header and the structure
body. The structure header is eight bytes long and contains information regarding the type of
structure and its length. The structure body contains the actual data. Within the structure body,
every piece of data is oriented to a multiple of its own size relative to the structure beginning, with
the exception of strings that are oriented to a multiple of two.



Byte 1

Byte 2

Byte 3

Byte 4







Byte 5



_F_ 7 _F_ 0



_L_ 23





_T_ 8 Byte 7 _L_ 15

_T_ 0 Byte 8 _L_ 7



Byte 6 _L_ 23 _L_ 16
Byte 7 _L_ 15 _L_ 8
Byte 8 _L_ 7 _L_ 0



_L_ 15 _L_ 8



_L_ 0


|R R<br>7 0|Col2|Col3|Col4|
|---|---|---|---|
|_C_7<br>_C_0|_C_7<br>_C_0|_C_7<br>_C_0|_C_7<br>_C_0|
|_B_|_E_|_S_|_T_12<br>_T_8|
|_T_0<br>_T_7<br>|_T_0<br>_T_7<br>|_T_0<br>_T_7<br>|_T_0<br>_T_7<br>|



Figure C.8: Structure header


Figure C.8 shows the organization of the structure header.


Bits _R_ 7 _. . . R_ 0 are reserved for future use and should be set to 0.


Bits _C_ 7 _. . . C_ 0 contain the type class, bits _T_ 12 _. . . T_ 0 contain the object type. The object types must
be unique within the type class.


The _B_ - and _E_ –bits are used to label the beginning and end of a compound object as described in
section C.2.3.


The _S_ bit indicates whether single precision floating point parameters ( _S_ –Bit is set) or double
precision floating point parameters ( _S_ –Bit is reset) are present [1] . Not every object type that has
floating point parameters must support single and double precision.


Bits _F_ 7 _. . . F_ 0 do not have a predefined meaning and, depending on object type, can be used for
flag bits.


Finally, bits _L_ 23 _. . . L_ 0 contain the entire length of the structure.


The structural header itself is always oriented to a multiple of eight relative to the beginning
of the file and the entire length of the structure also is a multiple of eight. This requires the
end of the structure to be filled with null bytes, if necessary. This ensures that the EGM can
be mapped directly to memory and that no orientation problems arise when individual data are
directly accessed, which could cause a bus error.


1For type attributes that use either FLOAT32 or FLOAT64 depending on _S_ bit, only FLOAT is written.


185


In text format, every structured data type consists of one or more lines, with all but the last line
having to have a backward slash ( `\` ) directly before the line end character(s). These lines are
combined into a data record. The backward slashes and the subsequent line end are removed.


Every single line cannot have more than 2047 characters, including the line end character. The
number of characters per line without the line end character should not exceed 2045, since two
characters are used for designating the end of a line on some platforms. Theoretically, the length
of the entire data record is unlimited.


Lines are ended using either `\x0A`, `\x0D`, or `\x0D\x0A` .


The individual single data within a data record are delimited by one or more separators, using the
blank space ( `\40` ) and the tab character ( `\t` ) as separator.


At the beginning of the data record, there are type class and object type, delimited by one or several
separators and followed by flags _F_ 0 to _F_ 7 indicated as unsigned decimal, octal, or hexadecimal
number in the range between 0 and 255, with _F_ 7 being the highest value bit. Type class as well as
object type may be indicated as identifier that is case sensitive or as decimal, octal, or hexadecimal
numbers. The structural remainder of the data record is defined by the object type whose identifier
and coding only must be unique within the type class.


An octal number is marked by a leading `0` and a hexadecimal number by a leading `0x` or `0X`, with
only digits ranging from `0` to `7` being permitted for octal numbers and digits ranging form `0` to `9`,
`A` to `F`, and `a` to `f` being permitted for hexadecimal numbers.


**C.2.3** **Compound types**


Compound types consist of a series of structured data types that are enclosed by a _Begin_ and a
_End_ object of the same structured data type. These _Begin_ and _End_ objects must always occur in
pairs.


In EGM binary format, the _Begin_ object is marked by a set _B_ bit in the structure header; the _End_
object is marked by a set _E_ bit.


In EGM text format, the data record of the _Begin_ object starts with the `begin` identifier, which
is followed by the type class, delimited by one or several separators. The data record of the _End_
object starts analogically with the `end` identifier, followed by one or several separators and the
type class. Instead of `begin`, there can also be a single plus sign ( `+` ), and instead of `end`, there can
be a single minus sign ( `-` ).

#### **C.3 File header**


The EGM binary format starts with the following structure:


_major_ is the main version number and _minor_ is the sub version number. The version described
in this document of EGM is 1.0 ( _major_ = 1; _minor_ = 0).


In EGM text format, the first line contains the `EGM` and `version` identifiers, with `EGM` being directly
at the beginning of the line and `version` being delimited from `EGM` by a blank space. This is followed


186


|31 24|16|8|0|
|---|---|---|---|
|||||
|0x45|0x47|0x4D|0x00|
|_major_|_major_|_minor_|_minor_|


Figure C.9: binary EGM header


by the main version number and the sub version number, delimited by the ”usual” separators. The
first EGM line in version 1.0 is as follows:

```
EGM version 1 0

#### **C.4 General structured data types**

```

This section describes general structured data types that have no relation to concrete type classes.
These types have the type class 1 with the `common` identifier.


**C.4.1** **Comment**


Type class: 1 / `common`


Object type: 1 / `comment`

|1 / comment|Col2|Col3|
|---|---|---|
|Oﬀset|Type|Parameter|
|8|STRING|_comment_|



roundup(11 + _len,_ 8) End of structure


Comments are ignored during reading.


In a special case, comments can consist of a data record in the text format that starts with a
pound sign ( `#` ) followed directly by the actual comment and delimited from the pound sign by one
or more separators, if necessary.


**C.4.2** **EGM type**


Type class: 1 / `common`


Object type: 2 / `egm_type`

|2 / egm_type|Col2|Col3|
|---|---|---|
|Oﬀset|Type|Parameter|
|8|STRING|_egmtype_|



roundup(11 + _len,_ 8) End of structure


The file header described in section C.3 can be followed directly by a type _EGMType_ object. The
_egmtype_ character string describes the EGM type. Currently, the types listed in table C.2 have
been defined.


187


|Identifier|Description|
|---|---|
|`x2DSYMBOL`|The EGM describes a 2D symbol. It should only contain objects of type classes<br>`common` and` gr2dobj`. Objects of a diﬀerent type class are ignored while they<br>are being read .<br>Type` x2DSYMBOL` EGM ﬁles contain exactly one 2D object. If this object is made<br>up of several primitive 2D objects, they must be encapsulated by a_ Compound_<br>object.|


Table C.2: EGM types

#### **C.5 Graphic 2D objects**


The graphic 2D objects are combined into one type class carrying the number 2 and the `gr2dobj`
identifier .


The 2D objects are described using a x/y-coordinate system, with the x-axis pointing to the right
and the y-axis pointing up. Angle information is given by radian measure, they are mathematically
positive (counterclockwise) and are relative to the positive x-axis, if not otherwise specified.


All coordinates are given as single or double precision floating point values. The precision used is
specified in each individual 2D object by the _S_ bit of the structure header.


**C.5.1** **Compound**


Graphic 2D objects are nested by the _Compound_ compound type. The nesting can also be applied
recursively.


A _Compound_ does not possess a geometric representation. Ordinarily, it includes at least one
object of type class `gr2dobj` . Other included objects are not permitted.


As an option, the _Compound_ object can contain a geometric transformation that is to be applied
to the enclosed objects. The transformation is specified either as a rotation of the enclosed objects
with subsequent translation or as a 3 _×_ 3 transformation matrix plus optional inverse transformation
matrix.


188


Type class: 2 / `gr2dobj`


Object type: 1 / `compound`


Flags: _F_ 1 _F_ 0 = 00: no transformation
_F_ 1 _F_ 0 = 01: Rotation and translation
_F_ 1 _F_ 0 = 10: Transformation matrix
_F_ 1 _F_ 0 = 11: Transformation matrix and inverse transformation matrix


Parameter: Offset ( _S_ = 1) Offset ( _S_ = 0) Type Parameter

with _F_ 1 _F_ 0 = 01:

|8<br>12|8<br>16|FLOAT<br>FLOAT|α<br>x, y<br>offs offs|
|---|---|---|---|
|24|32|End of structure|End of structure|



with _F_ 1 _F_ 0 = 10:

|8|8|FLOAT|mat<br>2×3|
|---|---|---|---|
|32|56|End of structure|End of structure|



with _F_ 1 _F_ 0 = 11:

|8<br>32|8<br>56|FLOAT<br>FLOAT|mat<br>2×3<br>mat−1<br>2×3|
|---|---|---|---|
|56|104|End of structure|End of structure|



If the transformation is entered using rotation and translation, the coordinates of the encapsulated
in the coordinate system of the _Compound_ object are calculated as follows:


_x_ _[′]_ = _x_ cos _α −_ _y_ sin _α_ + _xoffs_
_y_ _[′]_ = _x_ sin _α_ + _y_ cos _α_ + _yoffs_


If the transformation is entered using the transformation matrix, the coordinates of the encapsulated objects in the coordinate system of the _Compound_ object are calculated as follows:


_x_ _[′]_ = _m_ 11 _x_ + _m_ 12 _y_ + _m_ 13
_y_ _[′]_ = _m_ 21 _x_ + _m_ 22 _y_ + _m_ 23


The transformation matrices as well as their inverse are saved in EGM format line by line, beginning
with the element in the left upper corner ( _m_ 11). The third line is not saved, since it always reads
0 _._ 0 _,_ 0 _._ 0 _,_ 1 _._ 0.


The inverse matrix may be included in the EGM format in order to eliminate the need to determine
it while reading the EGM .


**C.5.2** **Graphic primitives**


This section describes all the 2D objects with a graphic representation.


The object definitions contain only the geometric information, such as point coordinates and angles.
Attributes, such as color, are saved in a _Attribut–Set_ and are set using special attribute objects
within it. The graphic primitives use the attributes that are relevant to them at the time of their
occurrence in the attribute set. For each individual graphic primitive, these attributes are listed
under ”Attributes:”


189


**Lines**


Type class: 2 / `gr2dobj`


Object type: 256 / `lines`

|Color, LineWidth,|LineStyle|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8<br>12<br>12 + (_n −_1)_ ×_ 16|8<br>16<br>16 + (_n −_1)_ ×_ 32|UINT32<br>FLOAT<br>FLOAT|_n_<br>_x_10_, y_10_, x_20_, y_20<br>_x_1_n−_1_, y_1_n−_1_,_<br>_x_2_n−_1_, y_2_n−_1|



16 + _n ×_ 16 16 + _n ×_ 32 End of structure


Type _Lines_ objects represent one or several separated line segments. The _n_ parameter specifies the
number of line segments. Its value must be greater than or equal to 1. Every single line segment
starts at _x_ 1 _i, y_ 1 _i_ and ends at _x_ 2 _i, y_ 2 _i_ .


**Polyline**


Type class: 2 / `gr2dobj`


Object type: 257 / `polyline`


Attributes: _Color, LineWidth, LineStyle_


Flags: _F_ 0 = 0: open

|F0 = 0: open F0 = 1: closed|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8<br>12<br>12 + (_n −_1)_ ×_ 8|8<br>16<br>16 + (_n −_1)_ ×_ 16|UINT32<br>FLOAT<br>FLOAT|_n_<br>_x_0_, y_0<br>_xn−_1_, yn−_1|



16 + _n ×_ 8 16 + _n ×_ 16 End of structure


Depending on flag _F_ 0, type _Polyline_ objects represent an open or closed line. The _n_ parameter is
bigger than the number of line segments by 1. It must be greater than or equal to 2. If flag _F_ 0 is
set, the last point ( _xn−_ 1 _, yn−_ 1) is connected to the first point ( _x_ 0 _, y_ 0).


**Points**


Type class: 2 / `gr2dobj`


Object type: 258 / `points`

|Color, PointSize,|PointType|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8<br>12<br>12 + (_n −_1)_ ×_ 8|8<br>16<br>16 + (_n −_1)_ ×_ 16|UINT32<br>FLOAT<br>FLOAT|_n_<br>_x_0_, y_0<br>_xn−_1_, yn−_1|



16 + _n ×_ 8 16 + _n ×_ 16 End of structure


190


Type _Points_ objects represent one or several points. The parameter _n_ specifies the number of
points. Its value must be greater than or equal to 1.


**Circle**


Type class: 2 / `gr2dobj`


Object type: 259 / `circle`


Attributes: _Color, LineWidth, LineStyle_

|Offset (S = 1)|Offset (S = 0)|Type|Parameter|
|---|---|---|---|
|8<br>16|8<br>24|FLOAT<br>FLOAT|_xcenter, ycenter_<br>_r_|



24 32 End of structure


_Circle_ type objects represent a circle with a radius of _r_, whose center is determined by _xcenter, ycenter_ .
The radius _r_ must be greater than 0 _._ 0.


**Arc**


Type class: 2 / `gr2dobj`


Object type: 260 / `arc`


Attributes: _Color, LineWidth, LineStyle_

|Offset (S = 1)|Offset (S = 0)|Type|Parameter|
|---|---|---|---|
|8<br>16<br>20|8<br>24<br>32|FLOAT<br>FLOAT<br>FLOAT|_xcenter, ycenter_<br>_r_<br>_αstart, αend_|



24 48 End of structure


_Arc_ type objects represent an arc with a radius of _r_, whose center is determined by _xcenter, ycenter_ .
The arc is drawn from angle _αstart_ to angle _αend_ in mathematically positive direction.


**Ellipsis**


Type class: 2 / `gr2dobj`


Object type: 261 / `ellipse`


Attributes: _Color, LineWidth, LineStyle_

|Offset (S = 1)|Offset (S = 0)|Type|Parameter|
|---|---|---|---|
|8<br>16<br>24|8<br>24<br>40|FLOAT<br>FLOAT<br>FLOAT|_xcenter, ycenter_<br>_xradius, yradius_<br>_α_|



32 48 End of structure


Type _Ellipse_ objects represent an ellipsis whose center is determined by _xcenter, ycenter_ . The radius
of the not rotated ellipsis in the direction of the x-axis is _xradius_, the radius in the direction of the
y-axis is _yradius_ . The rotation angle of the ellipsis around its center is _α_ .


191


**Text**


Type class: 2 / `gr2dobj`


Object type: 262 / `text`


Attributes: _Color, Font, FontHeight, FontAspectRatio, FontAlignment_

|Offset (S = 1)|Offset (S = 0)|Type|Parameter|
|---|---|---|---|
|8<br>16<br>20<br>20|8<br>24<br>32<br>32|FLOAT<br>FLOAT<br>FLOAT<br>STRING|_xorigin, yorigin_<br>_α_<br>_width_<br>_text_|



roundup(23 + _len,_ 8) roundup(35 + _len,_ 8) End of structure


Type _Text_ objects represent the _text_ text whose baseline goes through the reference point _xorigin, yorigin_
and around which the reference point is rotated by the angle _α_ . The _width_ parameter indicates
the width of the text that was not rotated. The position of the left side of the first letter on the
baseline is determined as described in section C.5.3.


A system that uses the same fonts as EGR GF can ignore the _width_ parameter when reading, since
the width of the text is determined by the _FontAspectRatio_ attribute. Other systems must ignore
the _FontAspectRatio_ attribute and must instead modify the text to match the width specified in
the _width_ parameter.


**Convex Polygon**


Type class: 2 / `gr2dobj`


Object type: 263 / `cvx_polygon`


Attributes: _Color, FillStyle_

|Color, FillStyle|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8<br>12<br>12 + (_n −_1)_ ×_ 8|8<br>16<br>16 + (_n −_1)_ ×_ 16|UINT32<br>FLOAT<br>FLOAT|_n_<br>_x_0_, y_0<br>_xn−_1_, yn−_1|



16 + _n ×_ 8 16 + _n ×_ 16 End of structure


Type _Convex Polygon_ objects represent a convex polygon. In a convex polygon, no edges intersect
and all interior angles are smaller than or equal to _π_ .


The _n_ parameter containing the number of corner points of the polygon must be greater than 2.


**C.5.3** **Attributes**


As described in section C.5.2, the graphic 2D primitives contain no attributes, but only pure
geometry information. They use the attributes relevant to them that are specified in the current
attribute set at the time of their occurrence in the EGM format.


Since the EGM permits the hierarchical structuring of data in tree form, it must also support a
hierarchy of attribute sets. This hierarchy is built in tree form when the EGM is read, with all


192


nodes of the tree that are located on the path from the root to the current leaf existing. This is
implemented by a stack of Attribute sets.


To keep the stack of the attribute sets consistent with the EGM hierarchy, every compound type
_End_ object removes the necessary number of attribute sets from the stack, until the number of
attribute sets on the stack is the same as the number that the corresponding _Begin_ object found
on the stack. Similarly, an error occurs when an attribute set is removed from the stack, so that
the number of attribute sets on the stack becomes smaller than the number of the attribute sets
that the most interior compound type _Begin_ object found on the stack.


**Attribute values**


The coordinate values given with the graphic 2D primitives generally (i.e. if there was no scaling)
can be interpreted as meters. The dimensions of the display on screen, on the printer or the plotter
depend on the scale used.


Compared with this, many attribute values for the graphic 2D object are given independently from
the scale, since on the one hand this complies with the capabilities of common output devices and
on the other hand scaling is oftentimes not desired. The basic unit in this case is the point. The
following interrelations are valid for the size of one point:


1pt = 0 _._ 03527cm 1pt = 0 _._ 3527mm 1pt = 0 _._ 0138in
1cm = 28 _._ 346457pt 1mm = 2 _._ 8346457pt 1in = 72pt


This definition of a point is compatible with the definition of a PostScript point, but somewhat
differs from the definition used during letterpress printing. This definition stated 1in = 72 _._ 27pt
and 1in = 72bp, where bp stands for _Big Point_ .


When outputting the point on a printer or plotter, the size of the point should be observed exactly.
When outputting on the screen it is acceptable to display a point as a pixel using the customary
resolution of 75 to 100 dpi.


**Push Attrib**


Type class: 2 / `gr2dobj`


Object type: 512 / `push_attr`


Parameter: Offset ( _S_ = 1) Offset ( _S_ = 0) Type Parameter

8 8 End of structure


When an _PushAttrib_ object is being read, the current status of the attribute set is put on the
attribute stack.


**Pop Attrib**


Type class: 2 / `gr2dobj`


Object type: 513 / `pop_attr`


Parameter: Offset ( _S_ = 1) Offset ( _S_ = 0) Type Parameter

8 8 End of structure


193


When a _PopAttrib_ object is being read, the top attribute set is copied into the current attribute
set and is removed from the stack. An error occurs when the number of the attribute sets on the
stack is subsequently smaller than when the most interior compound type _Begin_ object was read.


**Init Attrib**


Type class: 2 / `gr2dobj`


Object type: 514 / `init_attr`


Parameter: Offset ( _S_ = 1) Offset ( _S_ = 0) Type Parameter

8 8 End of structure


When a _InitAttrib_ object is read, the current attribute set is reset to the default values. The
default values are specified with the individual attributes.


**Color**


Type class: 2 / `gr2dobj`


Object type: 515 / `color`


Default: _r/g/b_ = 0 _/_ 0 _/_ 0

|Offset|Type|Parameter|
|---|---|---|
|8<br>10<br>12|UINT16<br>UINT16<br>UINT16|_red_<br>_green_<br>_blue_|



16 End of structure


The specification of numbers is accomplished in the RGB system. Here, an unsigned integer value
ranging from 0 to 65535 is specified for each color component. 0 Corresponds to minimum intensity
and 65535 corresponds to maximum intensity.


**Line Width**


Type class: 2 / `gr2dobj`


Object type: 516 / `line_width`


Default: 1.0

|1.0|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8|8|FLOAT|_linewidth_|



16 16 End of structure


Type _LineWidth_ objects set the line width attribute in the current attribute set. The line width is
specified in points (pt). The value must be greater than 0 _._ 0. Invalid values are interpreted as 1 _._ 0.


194


**Line Style**


Type class: 2 / `gr2dobj`


Object type: 517 / `line_style`


Default: -1

|-1|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8<br>12|8<br>16|UINT32<br>FLOAT|_linestyle_<br>_factor_|



16 24 End of structure


Type _LineStyle_ objects set the line type in the current attribute set.


The following values are predefined for the _linestyle_ parameter:

|Constant|line type|
|---|---|
|_−_1<br>0<br>1<br>2<br>3<br>4<br>5|default value<br>solid line<br>dashed line<br>dotted line<br>dash–point line<br>dash–point–point line<br>dash–point–point–point line|



Table C.3: Predefined line types


If the _LineStyle_ is _−_ 1, the _LineStyle_ set by the parent object will apply.


The parameter is specified in points (pt) and determines how the line is stretched. Depending on
the line type, the parameter affects the display of the line as follows:

|Line type|meaning of factor factor|
|---|---|
|dashed|length of the displayed and hidden segments|
|dotted|distance between the center points of two neighboring points|
|dash–point|length of the displayed line segment and half length of the hidden<br>line segments|
|dash–2–point|length of the line segment and a third of the hidden line segments|
|dash–3–point|length of the line segment and a fourth of the hidden line segments|



Table C.4: Effects of the factor on the line type


195


**Point Size**


Type class: 2 / `gr2dobj`


Object type: 518 / `point_size`


Default : 0.1

|0.1|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8|8|FLOAT|_pointsize_|



16 16 End of structure


In the current attribute set, type _PointSize_ objects set the size of a point. The point size is used
only if the set point type is a vector point. In this case, the vertices of the point are calculated as
described in table C.6, with the _d_ variable being the point size set using _PointSize_ .


**Point Type**


Type class: 2 / `gr2dobj`


Object type: 522 / `point_style`


Default: 0xf **f** f

|0xffffffff|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8|8|UINT32|_pointstyle_|



16 16 End of structure


Points can be differentiated into bitmap points and vector points. The orientation of bitmap points
is always the same and they are always the same size irrespective of the scaling and the set point
size. The size and orientation of vector points is determined by the set point size as well as the
transformation of a parent compound object, if necessary.


The specification of points is done using a bit mask so that different bitmap points as well as
different vector points can be combined with each other [2] . Vector points may be combined in any
way; the combination of bitmap points is limited to point types of different classes.


Tables C.5 and C.6 contain the constants for the specification of bitmap and vector points. Table
C.6 contains the provision for calculating the vertices of the points, with the _d_ variable being the
point size set using _PointSize_ .


**Font**


Currently, different fonts are not supported.


2i.e., that bitmap and vector points cannot be combined.


196


|Constant|diameter|constant|diameter|
|---|---|---|---|
|_−_1|default value|||
|ﬁlled circle:|ﬁlled circle:|cross:|cross:|
|`0x40000001`|1 pixel|`0x40000008`|5 pixels|
|`0x40000002`|3 pixels|`0x40000010`|10 pixels|
|`0x40000003`|5 pixels|`0x40000018`|15 pixels|
|`0x40000004`|7 pixels|`0x40000020`|20 pixels|
|`0x40000005`|9 pixels|`0x40000028`|25 pixels|
|`0x40000006`|11 pixels|`0x40000030`|30 pixels|
|`0x40000007`|13 pixels|`0x40000038`|40 pixels|
|diagonal cross:|diagonal cross:|circle:|circle:|
|`0x40000040`|5 pixels|`0x40000200`|5 pixels|
|`0x40000080`|10 pixels|`0x40000400`|10 pixels|
|`0x400000c0`|15 pixels|`0x40000600`|15 pixels|
|`0x40000100`|20 pixels|`0x40000800`|20 pixels|
|`0x40000140`|25 pixels|`0x40000a00`|25 pixels|
|`0x40000180`|30 pixels|`0x40000c00`|30 pixels|
|`0x400001c0`|40 Pixel|`0x40000e00`|40 Pixel|
|square:|square:|square:|square:|
|`0x40001000`|5 pixels|`0x40002000`|10 pixel|
|`0x40003000`|15 pixels|`0x40004000`|20 pixel|
|`0x40005000`|25 pixels|`0x40006000`|30 pixel|
|`0x40007000`|40 pixels|||


Table C.5: Bitmap point types and their constants


**Font Height**


Type class: 2 / `gr2dobj`


Object type: 519 / `font_height`


Default: 0.1

|0.1|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8|8|FLOAT|_fontheight_|



16 16 End of structure


In the current attribute set, type _FontHeight_ objects set the font height, measured from the baseline
to the upper edge of normal capital letters. Thus, the height specification does not take into
consideration ascenders and descenders.


The font height is **not** given in points.


197


|Constant|Type|
|---|---|
|`0x00000001`|small cross:<br>[(_−_0_._5_ × d,_ 0)_,_ (0_._5_ × d,_ 0)]_,_ [(0_, −_0_._5_ × d_)_,_ (0_,_ 0_._5_ × d_)]|
|`0x00000002`|large cross:<br>[(_−_<br>_√_<br>0_._5_ × d,_ 0)_,_ (<br>_√_<br>0_._5_ × d,_ 0)]_,_ [(0_, −_<br>_√_<br>0_._5_ × d_)_,_ (0_,_<br>_√_<br>0_._5_ × d_)]<br>|
|`0x00000004`|small cross rotated by 45~~_◦_~~:<br>[(_−_<br>_√_<br>0_._125_ × d, −_<br>_√_<br>0_._125_ × d_)_,_ (<br>_√_<br>0_._125_ × d,_<br>_√_<br>0_._125_ × d_)]_,_ [(_−_<br>_√_<br>0_._125_ ×_<br>_d,_<br>_√_<br>0_._125_ × d_)_,_ (<br>_√_<br>0_._125_ × d, −_<br>_√_<br>0_._125_ × d_)]<br>|
|`0x00000008`|large cross rotated by 45~~_◦_~~:<br>[(_−_0_._5_ × d, −_0_._5_ × d_)_,_ (0_._5_ × d,_ 0_._5_ × d_)]_,_ [(_−_0_._5_ × d,_ 0_._5_ × d_)_,_ (0_._5_ ×_<br>_d, −_0_._5_ × d_)]|
|`0x00000010`|circle, diameter is_ d_, center is at (0_,_ 0)|
|`0x00000020`|square:<br>[(0_._5_×d,_ 0_._5_×d_)_,_ (_−_0_._5_×d,_ 0_._5_×d_)_,_ (_−_0_._5_×d, −_0_._5_×d_)_,_ (0_._5_×d, −_0_._5_×_<br>_d_)]<br>|
|`0x00000040`|square rotated by 45~~_◦_~~:<br>[(0_._5_ × d,_ 0)_,_ (0_,_ 0_._5_ × d_)_,_ (_−_0_._5_ × d,_ 0)_,_ (0_, −_0_._5_ × d_)]|
|`0x00000080`|isosceles triangle, vertex to the right:<br>_c_ = (0_._5_ −_<br>~~q~~<br>3<br>4)_ × d,_<br>[(0_._5_ × d,_ 0)_,_ (_c,_ 0_._5_ × d_)_,_ (_c, −_0_._5_ × d_)]|
|`0x00000100`|isosceles triangle, vertex up:<br>_c_ = (0_._5_ −_<br>~~q~~<br>3<br>4)_ × d,_<br>[(0_,_ 0_._5_ × d_)_,_ (_−_0_._5_ × d, c_)_,_ (0_._5_ × d, c_)]|
|`0x00000200`|isosceles triangle, vertex to the left:<br>_c_ = (0_._5_ −_<br>~~q~~<br>3<br>4)_ × d,_<br>[(_−_0_._5_ × d,_ 0)_,_ (_c, −_0_._5_ × d_)_,_ (_c,_ 0_._5_ × d_)]|
|`0x00000400`|isosceles triangle, vertex down:<br>_c_ = (0_._5_ −_<br>~~q~~<br>3<br>4)_ × d,_<br>[(0_, −_0_._5_ × d_)_,_ (0_._5_ × d, c_)_,_ (_−_0_._5_ × d, c_)]|



Table C.6: Vector point types and their constants


**Font Aspect Ratio**


Type class: 2 / `gr2dobj`


Object type: 520 / `font_aspect_ratio`


Default: 1.0

|1.0|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8|8|FLOAT|_aspectratio_|



16 16 End of structure


In the current attribute, type _FontAspectRatio_ objects set the font aspect ratio. This ratio determines whether the font should appear flattened, ( _aspectratio <_ 1 _._ 0), stretched ( _aspectratio >_ 1 _._ 0),
or normal ( _aspectratio_ = 1 _._ 0).


198


**Font Alignment**


Type class: 2 / `gr2dobj`


Object type: 521 / `font_alignment`


Default: _−_ 1 _._ 0

|−1.0|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8|8|FLOAT|_alignment_|



16 16 End of structure


In the current attribute set, type _FontAlignment_ objects set the horizontal orientation of the text
in relation to its origin.


If _width_ is the width of the text, the ∆ _x_ offset of the left side of the first letter on the baseline in
relation to the reference point is determined as follows:


∆ _x_ = _−_ ( _alignment_ + 1 _._ 0) _×_ ( _width/_ 2 _._ 0)


**Layer**


Type class: 2 / `gr2dobj`


Object type: 523 / `layer`


Default: `DEFAULT`

|DEFAULT|Col2|Col3|Col4|
|---|---|---|---|
|Oﬀset (_S_ = 1)|Oﬀset (_S_ = 0)|Type|Parameter|
|8|8|SYMBOL|_layer_|



roundup(11 + _len,_ 8) roundup(11 + _len,_ 8) End of structure


In the current attribute set, type _Layer_ objects set the layer. The name of the layer must be
made up exclusively of ASCII letters, digits, and underscore and must not start with a digit. This
symbol is case sensitive.


The layer is used to control the visibility of objects. Generally it is possible to show and hide all
graphic objects belonging to a layer at the same time. The attributes are not allocated to objects
using the layer.


In a hierarchical structure of objects, objects belonging to the default layer [3] inherit the layer from
the parent object which in turn can be the default layer. If this is the case, the respective object
is always displayed [4] .


3the name of the default layer is `DEFAULT` .
4The default layer cannot be hidden.


199


## **Appendix D**

# **External data formats**

_OFML_ defines the external data formats described below. The corresponding files are located in
a library directory or library archive. Global material definitions are located in a global directory
with a relative path of _data/material_ . The predefined fonts are located in a global directory with
a relative path of _data/font_ .


External data have to be qualified completely. For example, the taken text resource, _@collision_,
must be qualified from the _::ofml::xoi_ packet as follows when called:


”::ofml::xoi::@collision”


If a text resource is not qualified, the resource file is first looked for in the package of the immediate
type of the instance for which the text resource is to be triggered [1] . If the resource file cannot be
found in this package, the search continues in the supertype packages.

#### **D.1 Geometries**


_•_ Geometry description files define polygon geometries, which can be loaded into _OFML_ directly.


_•_ **Name assignment:** The name of the geometric definition file results from the name of the
geometry as it is applied in _OiImport_ but without path or extension. Only ASCII characters
may be used. However, spaces are not allowed. The extension depends on the individual file.
Allowed extensions are:


**–** _geo_    - polygonal geometries (OFF format)

In this case, polygons must be defined simple, planar, convex and clockwise.


**–** _ipc_    - optional polygon colors (OFF format)

If polygon colors are defined, a material can be allocated on the _OFML_ level, but not
visualized.


1for example, the instance that outputs a message using _oiOutput()_, or the instance that is sent to the _oiGet-_
_StringResource()_ function


200


**–** _vnm_    - optional vertex normals (OFF format)

If no vertex normals are defined, they are generated.

**–** _3ds_    - polygonal geometries (3DS format)

Only geometries and materials (including textures) are accepted. If polygon colors are
defined, a material can be allocated on the _OFML_ level, but not visualized.


_•_ **Format:** The formats correspond to the individual definitions of the 3DS format and the
OFF format.

#### **D.2 Materials**


_•_ Material definition files substitute String identifiers from _OFML_ with a corresponding set of
material parameters.


_•_ **Name assignment:** The name of a material definition file results from the name of the
material in lower case. Only ASCII characters may be used. If a material name consists of
more than one word, the words are joined together. In doing so, spaces are eliminated. The
file extension is _mat_ .


**Example:** The ”ashnature.mat” file contains the definition of the material _Ash Nature_ .


_•_ **Format:** Material definition files are constructed line-by-line and consist of the name of the
material and any number of material parameter specifications. A material parameter specification overwrites the initial value of the corresponding material parameter. The following
specifications are permitted:


**–** _amb Red(Float) Green(Float) Blue(Float)_

The _amb_ key specifies the ambient color of the material. The components are floatingpoint numbers in the range of 0 _≤_ _z ≤_ 1. The initial ambient color is white (1 _._ 0 1 _._ 0
1 _._ 0).

**–** _dif Red(Float) Green(Float) Blue(Float)_

The _dif_ key specifies the diffuse color of the material. The components are floating-point
numbers in the range of 0 _≤_ _z ≤_ 1. The initial diffuse color is white (1 _._ 0 1 _._ 0 1 _._ 0). The
ambient and diffuse colors are usually the same.

**–** _spe Red(Float) Green(Float) Blue(Float)_

The _spe_ key specifies the specular color of the material. The components are floatingpoint numbers in the range of 0 _≤_ _z ≤_ 1. The initial specular color is black (0 _._ 0 0 _._ 0
0 _._ 0).

**–** _shi Shininess(Float)_

The _shi_ key specifies the specular exponent using a positive floating-point number. The
higher the exponent is, the lower the spread of the specular highlights. The initial
specular exponent has the value of 30 _._ 0.

**–** _tra Transparency(Float)_

The _tra_ key specifies the transparency using a nonnegative floating-point number that is
less than or equal to 1. The value of 0 _._ 0 stands for complete impermeability; the value
of 1 means complete transparency. The initial transparency is 0 _._ 0.


201


**–** _ref Refraction(Float)_

The _ref_ key specifies the refraction using a positive floating-point number. The initial
refraction has a value of 1 _._ 0 and is equivalent to the refraction in a vacuum.


**–** _tex image Format(String) Name(String)_

The _tex_ key specifies an image map texture. Initially, no texture is applied in the scope
of the material being defined. The supported formats are Targa ( _tga_ ), BMP ( _bmp_ ),
JPEG ( _jpg_ ) and SGI RGB ( _rgb_ ). The _Name_ parameter specifies the name of the image
without path or extension.


**–** _scale X(Float) Y(Float) Z(Float)_

If a texture has been defined using the _tex_ key, the _scale_ key specifies the scaling of the
texture. This is done using a positive scalar for each dimension. Each initial value is
1 _._ 0, meaning the image, regardless of its resolution, is scaled to a size of 1x1m.


**–** _rotate AngleX(Float) AngleY(Float) AngleZ(Float)_

If a texture has been defined using the _tex_ key, the _rotate_ key specifies the rotation of
the texture by the angle specified in degrees to the corresponding axis. The initial value
is 0 _._ 00 _._ 00 _._ 0.


**–** _prjx_

If an image mapping has been defined using the _tex_ key, the _prjx_ key specifies the
projection of the image on the x-axis.


**–** _prjy_

If an image mapping has been defined using the _tex_ key, the _prjy_ key specifies the
projection of the image on the y-axis.


**–** _prjz_

If an image mapping has been defined using the _tex_ key, the _prjz_ key specifies the
projection of the image on the z-axis.


**–** _prj X(Float) Y(FLoat) Z(Float)_

If an image mapping has been defined using the _tex_ key, the _prj_ key specifies the projection of the image on the axis specified by _X_, _Y_ and _Z_ .


**–** _circ R(Float)_

If an image mapping has been defined using the _tex_ key, the _circ_ key specifies the
mapping of the image on a circle with the radius of _R_


**–** _sph R(Float)_

If an image mapping has been defined using the _tex_ key, the _sph_ key specifies the mapping
of the image on a sphere with the radius of _R_


**–** _cyl R(Float) H(Float)_

If an image mapping has been defined using the _tex_ key, the _cyl_ key specifies the mapping
of the image on a cylinder with the radius of _R_ and height of _H_ .


**–** _cone R1(Float) R2(Float) H(Float)_

If an image mapping has been defined using the _tex_ key, the _cone_ key specifies the
mapping of the image on a cone with radii of _R1_ and _R2_ and height of _H_ .


202


**–** _quad X1(Float) Y1(Float) Z1(Float) X2(Float) Y2(Float) Z2(Float) X3(Float) Y3(Float)_
_Z3(Float) X4(Float) Y4(Float) Z4(Float)_

If an image mapping has been defined using the _tex_ key, the _quad_ key specifies the
mapping of the image on a common quadrilateral surface with corresponding coefficients.


**–** _interp Mode(Int)_

If an image mapping has been defined using the _tex_ key, the _interp_ key specifies whether
interpolation takes place (1) or not (0). The initial value is 1.


**–** _once Mode(Int)_

If an image mapping has been defined using the _tex_ key, the _once_ key specifies whether
a repeated mapping of the image takes place (1) or not (0). The initial value is 1.


The use of the material parameter depends on the applied display method. As long as objects
already define their own colors and materials, possibly concerning _OiImport_, the materials defined
here are not accepted.


In special cases, materials can be specified alternatively without external files. In such cases, the
parameter specifications can be entered directly in place of the material name. A material defined
in this manner must begin with a ’$’ sign. Furthermore, semicolons are used in place of the line
ends. Using the _mat_ key is not permitted in this case.


**Example:** The ”$ amb 1.0 0.0 0.0; dif 1.0 0.0 0.0” string sets the color of red as a material without the

use of an external material definition file.

#### **D.3 Fonts**


_•_ The fonts supported in _OFML_ are based on the fonts created by Dr. A. V. Hershey (U.S.
National Bureau of Standards). These are vector fonts, which describe continuous lines.


The following fonts are to be prepared by an _OFML_ -conforming runtime environment:


**–** _default_


**–** _cyrillic_


**–** _cursive_


**–** _timesg_


**–** _timesi_


**–** _timesib_


**–** _timesr_


**–** _timesrb_


_•_ **Name assignment:** The name of a font results from an identifier (the name of the font),
in which all letters are lower case. The font name does not have an extension.


_•_ **Format:** The format corresponds to the definition of the Hershey font format.


203


#### **D.4 External Tables**


_•_ External tables, such as product databases, are saved in a simple text format. Data records
are separated by line breaks. The individual fields of a data record have fixed lengths; there
are no field separators. Fields that are shorter than their corresponding lengths are filled in
to achieve their fixed lengths.


This generic table format can be read within _OFML_ using the global _oiTable()_ function
(Chapter 6).


_•_ **Name assignment:** The name of a product database can be chosen freely.


_•_ **Format:** The following field types are understood:


**–** Character strings. These are left-justified and, as necessary, filled with spaces, except
the last field in a data record. In the later case, the character string is closed with the
line end.

If the last field is the empty string, it can be omitted completely. In this case, the field
before the last is handled according to the rules above. If the field before the last is
empty as well, the rules can be applied again.


**–** Integers are right-justified and are filled with zeros.


**–** Fixed point numbers are right-justified and are filled with zeros; the decimal point is
left out.


**–** Fields that serve as the first key for access must be sorted in ascending order.

#### **D.5 Text Resources**


_•_ Text resources substitute a symbol identifier from _OFML_ with a corresponding text from
an external file. This might find use, for example, for property names, description texts or
output texts.


_•_ **Name assignment:** The name of a resource file is made up a link of the library names and
the corresponding ISO country abbreviation, separated by an underscore. The file extension
is _sr_ . All letters are lower case.


**Example:** The ”room ~~d~~ e.sr” file contains the German text resources for the _Room_ library.


_•_ **Format:** The relevant lines are formatted as follows:

```
   @SYMBOL=<Text>

```

Here, the left expression is the assignment of a valid symbol as understood in _OFML_ . The
right expression is a text in any 8-bit character format. For Western Europe, the ISO-Latin
1 (ISO 8859-1) character set is applied. Another valid character format, for example, is
UTF 8. For use with formatted output, the text can be a format character string (Section
6.1). All other lines are ignored and can be used for structuring and comments. Based on
convention, a single pound sign indicates a comment. Two pound signs leads to structuring
of the resource file. By convention, the following structurings are established:


204


**–** _## messages:_    - Character strings that follow stand for messages, warnings etc.


**–** _## properties:_    - Character strings that follow stand for property titles.

#### **D.6 Archives**


_•_ Archives represent containers, each of which usually contains all of the files belonging to a
library. The archive structure corresponds to the format used by the UNIX SVR4 _ar_ utility
program.


_•_ **Name assignment:** The name of an archive is lower case. The extension is _alb_ . No other
standards apply.


_•_ **Format:** All archives begin with the string, `!<arch>\n` . The rest of the archive is made up
of objects, each of which consists of a header and the actual content of the file.


The header consists of six, fixed-length fields of ASCII characters. With two exceptions (see
below, these fields contain the file names (16 characters), the most recent time the file was
modified (12 characters), the user and group numbers of the file owner (6 characters each),
the access mode (8 characters) and the size of the file in bytes. All numerical fields are
decimal, except the access mode, which is specified in octal. The header is closed with the
`‘\n` string.


File names that are longer than 16 characters are treated differently. If at least one such
file exists in the archive, the first object in the archive is not a file, but a table named `//`,
which contains the long file names. In place of the file name in the header of each file is the
character, `/`, followed by a number that indicates the offset of the file name in regard to the
table.


A line break is appended to files having an uneven number of bytes, which, however, has
no effect on the size specified in the header. This ensures that every object starts on an
even-numbered address.


¿From the _OFML_ runtime environment, each archive becomes a special file called `__attrib`,
which contains the attributes of the archive. Each attribute claims one line and contains a
key and value pair, the elements of which are separated by a space. The following attributes
are standardized:


`version` The version of the archive, consisting of two numbers
separated by a period.
`valid_span` The validity range of the archive, consisting of
two date entries separated by an underscore.
`pwdcheck` A password for checking encryption.
`md5sum` The MD5 checksum of the archive.


All attributes are optional. Any number of attributes can be added.


205


## **Appendix E**

# **Format Specifications**

#### **E.1 Format Specifications for Properties**

This section describes syntax and meaning of format specifications for properties. Format specifications can be entered during the setting of properties in the _setupProperty()_ function of the
_Property_ interface (Section 4.4).


The format specification has one of the following forms:


_property-format:_
`"` _@L_ `"`
`"` _@A_ `"`
`"` _%[-][width][.prec]type_ `"`


The first two formats can be used with properties of the `"f"` base type and indicate that the
property value is a length or angle measurement and that the unit of measure set by the user
should be used for the presentation of entry of the value. The property editor must then perform
a conversion between the user-defined unit of measure and the unit of measure used in OFML for
length and angle measurements (m or rad).


The third form is used if an OFML object intends to force a special format for the presentation or
entry of property values. The format character string of this form begins with a % sign. Afterwards,
the following specifiers in the respective sequence are allowed:


_•_ an optional left-align indicator – `"` _[-]_ `"`


_•_ an optional width specifier – _[width]_


_•_ an optional precision specifier – _[.prec]_


_•_ a required type specifier – _type_


The following discrete value range is predefined for the type specifier:


206


**–** Decimal number ( _Int_ ) – _d_

The argument must be an _Int_ value. The value is converted to a character string that
contains the decimal places. If the format specification contains a precision specifier,
the specifier indicates that the resulting character string contains at least the specified
number of places. If the value features fewer places, it is filled with zeros dependent
upon the optional left-align indicator. If the left-align indicator is given, zeros are filled
in on the right side. Otherwise, zeros are filled in on the left side.

If the width specifier is used, it indicates the maximum number of places that the
resulting character string may possess. If width and precision specifier are used, then
the following applies: _width ≥_ _prec_ .


**–** Floating point number ( _Float_ ) – _f_

The argument must be a floating point number. The value is converted to a character
string of the form `"` _-ddd.ddd..._ `"` . The resulting character string starts with a minus sign
if the number is negative. The number of places after the decimal point is indicated by
the precision specifier. If no precision specifier is given, 2 is assumed as the number of
decimal places after the period.

If a width specifier is used, it indicates the exact width of the resulting character string.
Here, the minus sign is counted, but the decimal point is not. If the value has fewer
digits, zeros are filled in on the left side. The left-align indicator is ignored, if present.
If the value has more digits, the leading places are suppressed.


**–** Character string ( _String_ ) – _s_

The argument must be a character string. It is inserted instead of the format specifier. If
the precision specifier is indicated, it defines the maximum length of the resulting character string. If the length of the argument exceeds the maximum length, the character
string is cut off accordingly.

If the format specification contains a width specifier, the specifier indicates the minimum
number of characters of the resulting character string. If the character string features
fewer characters, the resulting character string is filled with spaces on the left side
(without set left-align indicator) or on the right side (with set left-align indicator).

#### **E.2 Definition Format for Properties**


This section describes the format of a property definition description that describes all properties
of an instance and is delivered as the result of the _getProperties()_ function in the _Property_ interface
(Section 4.4).


The following rules apply to the format of the definition of properties:


_•_ The description of all properties consists of the descriptions for each individual property
separated from each other by semicolons.


_•_ Each property definition reflects the data that are transferred to the _Property::setupProperty()_
function and consists of a set of required and optional specifications that are separated by
semicolons.


207


_•_ A semicolon can be followed by a random number of spaces.


_•_ The first specification of a property definition is the key specifier:


**–** _k <str>_   - key of the specified property.


_•_ The last specifier of a property specification is the type specifier. It must have one of the
following values:


**–** _b_   - a boolean type (0 or 1).


**–** _i_   - a decimal type.


**–** _f_   - a floating point number type.


**–** _s_   - a character string type.


**–** _ch <str>*n_   - a selection list with _n, n >_ 0 character strings for use with character string
entry.


**–** _chf <str>_   - a selection list whose possible character strings are delivered by the listed
function.


**–** _u_   - a user-defined type with a given editor identification.


_•_ Additional optional specifiers between key and type specifier are:


**–** _n <str>*n_   - the name of the property.


**–** _d <str>*n_   - the initial value of the property.


**–** _mn <str>_   - minimum value of a decimal or floating point number or minimum number
of characters in a character string property.


**–** _mx <str>_   - maximum value of a decimal or floating point number or maximum number
of characters in a character string property.


**–** _fmt <str>_   - C-type format specifier (Section E.1)


208


## **Appendix F**

# **Additional Types**

The following defined types are not a direct component of OFML, but they can be used in OFMLconform libraries. The _Base_ interface is implemented, but with a few specific limitations in each
case.

#### **F.1 Interactor**


**Description**


_• Interactor_ implements the base class for interactors.


_•_ **Interface(s):** _Base_ with limitations:


The functions _isCat()_, _hide()_, _show()_, _isHidden()_, _selectable()_, _notSelectable()_, _isSelectable()_,
_setCutable()_, _isCutable()_, _enableCD()_, _disableCD()_, _isEnabledCD()_, _measure()_, and _unMea-_
_sure()_ are not available. The instance variable _mIsCutable_ is not available.


**Initialization**


_• Interactor(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _Interactor_ type.


**Methods**


_• final makeVisible(pType(Type) ...) →_ _Void_


The function generates an instance of the indicated type as element which represents the
geometry of the interactor. After the type argument, additional constructor arguments may
follow. If the interactor is already visible, the function is without effect. The transfer of
ZERO makes the interactor visible.


209


_• final isVisible() →_ _Int_


The function delivers 1 if the interactor is visible, otherwise 0.


_• final getState() →_ _Symbol_


The function delivers the state of the interactor which is described by one of the symbols
_@ENABLED_, _@DISABLED_ or _@ACTIVE_ .


_• final enable() →_ _Int_


It sets the interactor in the state ”free” ( _@ENABLED_ ) and always delivers 1 (success).


_• final disable() →_ _Int_


It sets the interactor in the state ”blocked” ( _@DISABLED_ ) and always delivers 1 (success).


_• final activate() →_ _Int_


It sets the interactor in the state ”active” ( _@ACTIVE_ ) where it must already be in the state
_@ENABLED_ . It delivers 1 with success and 0 if the interactor was blocked.

#### **F.2 Light**


**Description**


_• Light_ is a globally acting active light source that is, however, integrated in an instance
hierarchy. The following procedure applies for converting the light source in a local lighting
model: If the light source features children, it is a directional point light source. It is located
in the local origin and lights along the local negative y-axis. The aperture of the cone of
light results from the arcus tangent of the relationship of the maximum z-value of the local
delimiting volume of the light source to the negative minimum y-coordinate of the local
delimiting volume.


If the light source does not have any children or the minimum y-coordinate of the local
delimiting volume is equal to 0 _._ 0, it is a nondirectional point light source.


In a global lighting model, this explicit differentiation is unnecessary.


_•_ The _Light_ type may **not** be derived.


_•_ **Interface(s):** _Base_ with limitations:


The functions _getType()_, _isCat()_, _setCutable()_, _isCutable()_, _enableCD()_, _disableCD()_, _isEn-_
_abledCD()_, _measure()_, and _unMeasure()_ are not available. The instance variable _mIsCutable_
is not available.


**Initialization**


_• Light(pFather(MObject), pName(Symbol))_


The function initializes an instance of the _Light_ type.


210


**Methods**


_• final setColor(pColor(Float[3])) →_ _self_


The function sets the color of the light source. The elements of the _pColor_ vector must be
real numbers in the interval from 0 _._ 0 to 1 _._ 0 where the interval boundaries are acceptable
values. The vector elements are interpreted as amplitudes of the wavelengths red, green, and
blue. Their linear combination results in the actual color. The initial light color is white.


_• final getColor() →_ _Float[3]_


The function furnishes the current light color of the implicit instance.


_• final on() →_ _self_


The function activates the light source.


_• final off() →_ _self_


The function deactivates the light source.


_• final isOn() →_ _Int_


The function signals the status of the light source via its return value: activated (1) or
deactivated (0).

#### **F.3 MLine**


**Description**


_• MLine_ implements an automatic dimensioning primitive that automatically dimensions the
higher-order object in the hierarchy.


The line thickness measures 1 in the smallest representation unit of the image space in each
case, e.g., 1 pixel.


_•_ The _MLine_ type may **not** be derived.


_•_ **Interface(s):** _Base_ with limitations:


The functions _getType()_, _isCat()_, _hide()_, _show()_, _isHidden()_, _selectable()_, _notSelectable()_,
_isSelectable()_, _setCutable()_, _isCutable()_, _enableCD()_, _disableCD()_, _isEnabledCD()_, _measure()_,
and _unMeasure()_ are not available. The instance variable _mIsCutable_ is not available.


**Initialization**


_• MLine(pFather(MObject), pName(Symbol), pDirection(Symbol))_


The function initializes an instance of the _MLine_ type. The _pDirection_ parameter defines
how the topologically higher-order primitive is dimensioned. Either the width, the height, or
the depth of the local delimiting volume of the father is dimensioned. The following symbols
are allowed:


211


@NX The width is dimensioned at the bottom rear. The dimensioning lies in the local x-yplane of the father and can be read from the front.

@NXG The width is dimensioned at the bottom rear. The dimensioning lies in the local x-zplane of the father and can be read from the front and the top.

@NXT The width is dimensioned at the bottom rear. The dimensioning lies in the local x-zplane of the father and can be read from the rear and the top.

@PX The width is dimensioned at the top rear. The dimensioning lies in the local x-y-plane
of the father and can be read from the front.

@PXT The width is dimensioned at the bottom front. The dimensioning lies in the local
x-z-plane of the father and can be read from the front and the top.

@NY The height is dimensioned from the left rear. The dimensioning lies in the local x-y-plane
of the father, can be read from the front, and is aligned from bottom to top.

@PY The height is dimensioned from the right rear. The dimensioning lies in the local x-yplane of the father, can be read from the front, and is aligned from bottom to top.

@NZ The depth is dimensioned from the bottom left. The dimensioning lies in the local
y-z-plane of the father and can be read from the left.

@NZT The depth is dimensioned from the bottom left. The dimensioning lies in the local
x-z-plane of the father and can be read from the left and the top.

@PZ The depth is dimensioned from the bottom right. The dimensioning lies in the local
y-z-plane of the father and can be read from the right.

@PZT The depth is dimensioned from the bottom right. The dimensioning lies in the local
x-z-plane of the father and can be read from the right and the top.


**Methods**


_• final setMaterial(pMaterial(String)) →_ _self_


The specified material is assigned. The ambient component of the material is assigned as
color during the display. The presentation should be done without considering the lighting
and tint.


_• final getMaterial() →_ _String_


The function delivers the currently valid material of the implicit instance.


_• final setOffset(pOffset(Float)) →_ _self_


This function sets the offset of the dimension line with respect to the edge to be dimensioned.
The initial offset measures 0 _._ 1.


_• final getOffset() →_ _Float_


The function furnishes the current offset of the implicit instance.


_• final setText(pText(String)) →_ _self_


Initially, entities of _MLine_ automatically dimension the respective edge of the delimiting
volume of the father object and automatically adjust to the dimensional changes. However,
by using this function the text can be set explicitly. In this case, the _pText_ parameter presents
the text to be displayed by means of an ASCII character string.


212


_• final getText() →_ _String_


The function delivers the currently displayed text.

#### **F.4 MSymbol**


**Description**


_• MSymbol_ implements a polymorphic dimensioning primitive. All variants are generated in
the local x-y-plane. The z-coordinate is always 0. Coordinates with respect to this plane are
represented by a vector with 2 elements (x- and y-value, in this order).


The line thickness measures 1 in the smallest representation unit of the image space in each
case, e.g., 1 pixel.


_•_ The _MSymbol_ type may **not** be derived.


_•_ **Interface(s):** _Base_ with limitations:


The functions _getType()_, _isCat()_, _hide()_, _show()_, _isHidden()_, _selectable()_, _notSelectable()_,
_isSelectable()_, _setCutable()_, _isCutable()_, _enableCD()_, _disableCD()_, _isEnabledCD()_, _measure()_,
and _unMeasure()_ are not available. The instance variable _mIsCutable_ is not available.


**Initialization**


_• MSymbol(pFather(MObject), pName(Symbol), pMode(Symbol), pValues(Float[][2]))_


The function initializes an instance of the _MSymbol_ type. In this context, the _pMode_ parameter together with the variable _pValues_ parameter specifies the implementation of _MSymbol_ .
The evaluation of _pValues_ is dependent upon the assignment of _pMode_ . The following symbols
may be used for _pMode_ :


_@ARCLINE_ No contour of a segment of a circle is generated. The origin of the corresponding circle is
indicated by _pValues[0]_ . _pValues[1][0]_ defines the radius of the circle through a positive
number. _pValues[1][1]_ defines the length of the line in the radian measure through a
non-negative number. If the length is positive, the line starts at


_(pValues[0][0], pValues[0][1]+pValues[1][0])_


in clockwise direction. If it is negative, it starts at the same point, but runs in counterclockwise direction.

_@CIRCLE_ A filled circle is generated in the local origin. _pValues[0][0]_ defines the radius of the
circle through a positive number.

_@POLYLINE_ A continuous line is generated that connects the given points in the respective order.
The last and first point are not connected. The orientation is not taken into account.

_@RECTANGLE_ A filled rectangle is generated. _pValues[0]_ describes the lower left corner. _pValues[1]_
describes the upper right corner.

_@X_ _CIRCLE_ A circle is generated. _pValues[0]_ defines the origin of the circle with respect to the
local coordinate system. _pValues[1][0]_ defines the radius of the circle through a positive
number. If _pValues[1][1]_ equals 0 _._ 0, only the contour is shown. Otherwise, a filled circle
is drawn.


213


**Methods**


_• final setMaterial(pMaterial(String)) →_ _self_


The specified material is assigned. The ambient component of the material is assigned as
color during the display. The presentation should be done without considering the lighting
and tint.


_• final getMaterial() →_ _String_


The function delivers the currently valid material of the implicit instance.

#### **F.5 MText**


**Description**


_• MText_ implements a vector-text-primitive.


The line thickness measures 1 in the smallest representation unit of the image space in each
case, e.g., 1 pixel.


_•_ The _MText_ type may **not** be derived.


_•_ **Interface(s):** _Base_ with limitations:


The functions _getType()_, _isCat()_, _hide()_, _show()_, _isHidden()_, _selectable()_, _notSelectable()_,
_isSelectable()_, _setCutable()_, _isCutable()_, _enableCD()_, _disableCD()_, _isEnabledCD()_, _measure()_,
and _unMeasure()_ are not available. The instance variable _mIsCutable_ is not available.


**Initialization**


_• MText(pFather(MObject), pName(Symbol), pText(String))_


The function initializes an instance of the _MText_ type. The _pText_ parameter specifies the
text to be displayed in form of an ASCII character string.


**Methods**


_• final setMaterial(pMaterial(String)) →_ _self_


The specified material is assigned. The ambient component of the material is assigned as
color during the display. The presentation should be done without considering the lighting
and tint.


_• final getMaterial() →_ _String_


The function delivers the currently valid material of the implicit instance.


_• final setFont(pFont(String)) →_ _self_


The specified font is assigned. _pFont_ specifies the font through a corresponding font name
without path or extension information in accordance with Chapter D.


214


_• final getFont() →_ _String_


The function furnishes the current font of the implicit instance.


_• final setText(pText(String)) →_ _self_


The test to be displayed is set anew through the ASCII character string _pText_ .


_• final getText() →_ _String_


The function furnishes the current text of the implicit instance.


_• final setScale(pScale(Float)) →_ _self_


The positive _pScale_ parameter sets the scaling of the text. The initial scaling measures 0 _._ 05.


_• final getScale() →_ _Float_


The function furnishes the current scaling of the implicit instance.


_• final setAlignment(pAlignment(Symbol)) →_ _self_


The _pAlignment_ parameter determines the horizontal alignment of the text. The following
symbols can be used here:


_@LEFT_ The text is left-aligned with respect to the local reference point.


_@CENTER_ The text is centered with respect to the local reference point.


_@RIGHT_ The text is right-aligned with respect to the local reference point.


The initial alignment is _@CENTER_ .


_• final getAlignment() →_ _Symbol_


The function furnishes the current alignment of the implicit instance.


_• final setMode(pMode(Symbol)) →_ _self_


The _pMode_ parameter sets the presentation mode of the text. The following symbols are
allowed here:


_@NORMAL_ The text is shown in normal mode.


_@UNDERLINE_ The text is highlighted through underlining.


_@BOX_ The text is highlighted by a box.


The initial display mode is _@NORMAL_ .


_• final getMode() →_ _Symbol_


The function furnishes the current display mode of the implicit instance.


215


## **Appendix G**

# **Applied Notation**

#### **G.1 Class Diagrams based on Rumbaugh**

The notation used in this document for class diagrams is a modified form of the notation by
_Rumbaugh_ [Rumb91].




















|specific<br>subclass2|Col2|Col3|Col4|
|---|---|---|---|
|Attribute|Attribute|Attribute|Attribute|
|Method|Method|implementation<br>in pseudo code||
|Method||||



Figure G.1: Modified Rumbaugh Notation for Class Diagrams


In object-oriented software engineering, class diagrams are used to visualize the properties (attributes, methods) of classes and relationships between classes. Principally, there are three **types**
**of relationship** :


_• Vererbung_ (Inheritance).
A subclass inherits the properties of its super-class(es).


216


_• Aggregation_ (Consists Of).
An instance of a class (an object) contains (consists of) one or more object(s) of another
class.


_• Assoziation bzw. Bekanntschaft_ (Acquaintance).
An object of a class ”knows”’ an object of another class or is associated with it.


In abstract models, the attribute and/or method part of a class can be omitted.


217


## **Appendix H**

# **Categories**

The categories outlined below are predefined according to the definition. The use of these categories
is optional; the applicability and readability of data acquired in OFML increases accordingly if
these categories are used.

#### **H.1 Interface Categories**


For each OFML interface, a predefined corresponding category exists whose symbolic designator
is formed by the prefix ”‘IF ~~”~~ ’ [1] and the name of the interface, e.g., _@IF_ _Article_ . In this way, every
instance of an OFML type can be queried using the _isCat()_ function whether it implements a
special interface.

#### **H.2 Material Categories**


The following categories are predefined to designate the assignment of a geometric object or a
complex object to a certain material category:


_• @FRONT_  - The object belongs to the front of a complex object or represents it.


_• @GRIFF_  - The object belongs to the handle of a complex object or represents it.


_• @KORPUS_  - The object belongs to the corpus of a complex object or represents it.


_• @KRANZ_  - The object belongs to the border of a complex object or represents it.


_• @RUECK_  - The object belongs to the back of a complex object or represents it.


_• @SOCKEL_  - The object belongs to the base of a complex object or represents it.


1acronym for interface


218


_• @S_ ~~_F_~~ _USS_  - The object belongs to the foot of a chair or represents it.


_• @S_ ~~_L_~~ _EHNE_  - The object belongs to the back rest of a chair or represents it.


_• @S_ ~~_S_~~ _ITZ_  - The object belongs to the seat of a chair or represents it.


_• @T_ ~~_F_~~ _USS_  - The object belongs to the foot of a table or represents it.


_• @T_ ~~_G_~~ _ESTELL_  - The object belongs to the stand of a table or represents it.


_• @T_ ~~_G_~~ _EST_ _ABDECK_  - The object belongs to a lateral stand cover or represents it.


_• @T_ ~~_K_~~ _ANTE_  - The object belongs to the edge of a table top or represents it.


_• @T_ ~~_P_~~ _LATTE_  - The object belongs to a table top or represents it.

#### **H.3 Planning Categories**


The following categories are predefined to designate the ability of adding sections of an object:


_• @CEILING_ _ELEM_  - The object (e.g., a ceiling lamp) can be planned below an object.


_• @TOP_ _ELEM_  - The object (e.g., a desk lamp) can be planned on the surface of an object.


_• @WALL_ ~~_E_~~ _LEM_  - The object (e.g., an electrical outlet) can be planned at the surface of an
object.


219


## **Appendix I**

# **Terms**


_•_ **Bounding box**


**–** A bounding box is a rectangular volume that minimally encloses a body.


**–** The definition of bounding boxes makes reference to the local coordinate system of an
object or to the common coordinate system of all objects (world or global coordinate
system).


_•_ **Category**


**–** A category is a classification of _→_ types or _→_ entities that results from a certain viewing
perspective.


**–** Categories represent an expansion of the concept of types.


_•_ **Clipboard**


**–** A clipboard is a buffer storage in which objects can be placed. Objects can be written
to the clipboard using operations such as Cut and Copy. They can be read out again
from the clipboard using the Paste operation.


_•_ **Coordinate system**


**–** A coordinate system is an orthogonal space defined by three axes (x, y, z) to which
position and direction information are referenced.


**–** In a specific case, the z- and x-axis span a plane on which the y-axis is located at a right
angle.


_•_ **Father object**


**–** A father object is an _→_ object from which properties are inherited, e.g., a name space,
the spatial modeling, the material, etc.


220


_•_ **Identity**


**–** The identity of an _→_ instance results from a _→_ name in the hierarchical name space
that exists only once and uniquely describes the position in an instance hierarchy.


_•_ **Instance**


**–** An instance is a concrete implementation of a _→_ type. It differs from other entities
through a local copy of _→_ attributes, especially through a unique _→_ identity.


**–** Synonyms for instance are _→_ object and entity.


_•_ **Interface**


**–** An interface is the collection of a number of methods and member variables that a _→_
type must define or implement for interface compatibility.


_•_ **Lighting model**


**–** A lighting model uses great simplification to simulate the lighting of bodies (3D objects).


**–** For a **local** lighting model, only the lighted object and the light source (distance, materials, etc.) are viewed.


**–** For a **global** lighting model, other objects of the _→_ scene that cause shadows or reflections are also included.


_•_ **Name**


**–** The (absolute) name of an _→_ instance uniquely describes the topological position of the
instance. Alternatively, an instance can also be referenced through a _→_ symbol relating
to the respective context or through a variable.


_•_ **Object**


**–** From a programmer’s view, an object is a synonym for the _→_ instance of a _→_ type.
From a user’s view, an object represents a certain unit that can be generated, selected,
modified, and deleted as a whole.


_•_ **Program**


**–** A set of products combined by a manufacturer for functional and/or aesthetic points of
view.


**–** Synonyms: collection, product line


_•_ **Property (Feature)**


**–** A property is a feature of an instance, e.g., a geometric measurement or the designation
of an execution that may be changed interactively by the system user with the help of
appropriate dialogs (property editors).


221


_•_ **Symbol**


**–** A symbol is a string-like value that is used primarily to designate constants and instance
_→_ names.


_•_ **Root object**


**–** A root object is an _→_ object that is located at the root of an object hierarchy. Consequently, a root object has no _→_ father object. All objects that are directly located in a
_→_ scene are root objects.


_•_ **Scene**


**–** A scene is the collection of a number of 3D objects, in the context of OFML also called
_→_ entities.


_•_ **Type**


**–** A type combines a number of homogenous _→_ entities and defines structure and behavior
for them.


**–** A type implements one or several _→_ interfaces.


**–** A type features no more than one direct super type; its characteristics are inherited
from the super type.


**–** Class is a synonym for type.


_•_ **Units**


**–** If no other specific definition exists, units of length implicitly feature the unit _meter_ .


**–** If no other specific definition exists, units of angle implicitly feature the unit _radiant_ .


222


# **Index**

Change Status, 81
Spatial Modeling, 82
Spatial model, 95
2D Representation, 86
2D interface, 169
3DS file, 125, 201


ABAP/4, 156
Action, 168
Activation Status of a Property, 94
add()
MObject, 78
addInfoObj()
OiPlanning, 136
addPart()
Complex, 97
addProductDB()
OiPDManager, 157
OiPlanning, 139
Archive, 205
Article
Article, 98
Information, 99, 100, 158, 162
information, 14
Information, general, 99
article2Class()
OiPDManager, 158
OiPlanning, 139
article2Params()
OiPDManager, 158


Base, 79
Basic interfaces, 77
Block, 118
Bounding Box
global, 84
global, geometric, 85
local, 84
local, geometric, 84



callRules()
Base, 85
Category, 16
changedPropList()
Property, 93
Check String, 115
checkAdd()
Complex, 95
OiLevel, 165
OiPart, 151
OiPlanning, 137
OiPlElement, 145
OiProgInfo, 141
checkBorder()
OiPlanning, 136
checkChildColl()
Complex, 98
OiPlanning, 137
checkConsistency()
Article, 101
OiPart, 152
OiPDManager, 159
OiPlanning, 139
OiPlElement, 147
OiProductDB, 162
OiProgInfo, 141
checkElPos()
Complex, 97
OiPlanning, 137
checkObjConsistency()
OiPlanning, 140
checkPosition()
OiPlanning, 138
Child, 12, 78
and instance variable, 12
Creation and management, 95
Transformation, 147
Class, 10


223


class2Articles()
OiPDManager, 158
clearInfoObjs()
OiPlanning, 136
clearMethod()
Complex, 97
clearProductDBs()
OiPDManager, 157
Clipboard, 80, 96, 110, 114
Collision Detection, 82
Collision detection, 111, 141
for children, 97
Complex, 94
Condition, 168
Consistency check, 101
Constraint, 168
CREATE ~~E~~ LEMENT, 103
createOdbChildren()
OiOdbPlElement, 155
createOdbObjects()
Base, 87
Cylinder, 119
Cuttability, 80


Database, 115
delegationDone()
OiPlanning, 134
delInfoObj()
OiPlanning, 136
delProductDB()
OiPDManager, 157
Diagram, 216
Dimensioning, 82
disableCD()
Base, 82
disableChildCD()
Complex, 97
Dissolving text resources, 113
Distance measurement, 113
doCheckAdd()
OiPlanning, 137
doSpecial()
OiPlanning, 140
OiProgInfo, 141
Dynamic Properties, 86


EasternGraphics Metafile, 180
EGM, 180



Element, 12, 78
Transformation, 138
elemRotation()
OiPlanning, 138
OiPlElement, 147
elemTranslation()
OiPlanning, 138
OiPlElement, 147
Ellipsoid, 120
elRemoveValid()
OiPart, 151
OiPlElement, 146
enableCD()
Base, 82
enableChildCD()
Complex, 97
Environment, 135
Epsilon, eps, 79
Error log, 134
evalPropValue()
OiPDManager, 158
Existence check, 113
external data, 200
external geometry (ODB)
2D, 180
Extrusion body, 129


Father, 12
Father-child-relation, 12
Feature, 89
FINISH ~~D~~ UMP, 107
FINISH ~~E~~ VAL, 107
finishCollCheck()
OiProgInfo, 142
Font, 203
Format specifications, 206
Frame, 121


Generating a dump representation, 112
geometric object, 117
Geometry, 117, 200
getAllMatCats()
Material, 89
OiPart, 150
OiPlElement, 144
getArticleFeatures()
Article, 101
OiPart, 152


224


OiPDManager, 160
OiPlElement, 146
getArticleParams()
Article, 100
OiPart, 151
OiPlElement, 146
getArticlePrice()
Article, 100
OiPart, 151
OiPDManager, 159
OiPlElement, 146
OiProductDB, 163
getArticleSpec()
Article, 99
OiOdbPlElement, 154
OiPart, 151
OiPlElement, 146
getArticleText()
Article, 100
OiPart, 152
OiPDManager, 160
OiPlElement, 146
OiProductDB, 163
getBorder()
OiPlanning, 135
getChildren()
MObject, 78
getClass()
MObject, 77
getCMaterial()
Material, 89
OiPart, 150
OiPlanning, 136
OiPlElement, 144
OiProgInfo, 141
getCMaterials()
Material, 89
OiPart, 150
OiPlanning, 136
OiPlElement, 144
OiProgInfo, 141
getDataBasePath()
OiProductDB, 161
getDepth()
Complex, 95
OiPart, 149
OiPlElement, 143



getDistance()
Base, 85
getDynamicProps()
Base, 86
getElements()
MObject, 78
getEnvironment()
OiPlanning, 135
getErrorLog()
OiPlanning, 135
getExtPropOffset()
Property, 92
getFather()
MObject, 78
getFinalArticleSpec()
OiProductDB, 163
getHeight()
Complex, 95
OiLevel, 165
OiPart, 149
OiPlElement, 143
getID()
OiProductDB, 161
OiProgInfo, 141
getInfo()
OiPlanning, 136
getInfoIDs()
OiPlanning, 136
getLanguage()
OiPlanning, 133
getLocalBounds()
Base, 84
getLocalGeoBounds()
Base, 84
getMatCategories()
Material, 88
OiPart, 150
OiPlanning, 136
OiPlElement, 144
OiProgInfo, 141
getMatName()
Material, 89
OiPart, 150
OiPlanning, 136
OiPlElement, 144
OiProgInfo, 141
getMethod()


225


Complex, 96
getName()
MObject, 78
getOdbInfo()
Base, 86
OiOdbPlElement, 155
getOrderID()
Article, 99
getOrigin()
OiPart, 149
OiPlElement, 143
getPasteMode()
Complex, 96
getPDB ~~I~~ Ds()
OiPDManager, 157
getPDBFor()
OiPDManager, 158
getPDistance()
OiPlElement, 145
getPDManager()
OiPlanning, 139
OiProductDB, 161
getPictureInfo()
Base, 86
getPlanning()
OiPart, 149
OiPlElement, 142
OiProgInfo, 141
OiPropertyObj, 153
getPlanningMode()
OiLevel, 165
getPlanningWall()
OiLevel, 165
getPlElementUp()
OiPlanning, 135
getPosition()
Base, 83
getProductDB()
OiPDManager, 157
getProgPDB()
OiPDManager, 158
getProgram()
Article, 98
getPrograms()
OiProductDB, 161
getPropDefs()
OiProductDB, 161



getPropDescription()
OiProductDB, 163
getProperties()
Property, 92
getPropertyDef()
Property, 92
getPropertyKeys()
Property, 92
getPropertyPos()
Property, 92
getPropInfo()
Property, 94
getPropObj()
OiPlanning, 135
getPropState()
Property, 94
getPropTitle()
Property, 92
getPropValue()
Property, 93
getRegion()
OiPlanning, 134
getResolution()
Base, 81
getRoot()
MObject, 78
getRotation()
Base, 84
getRtAxis()
Base, 84
getTempArticleSpec()
Complex, 96
getTopPlElement()
OiPlanning, 135
getTrAxis()
Base, 83
getType()
MObject, 77
getVarCode()
OiProductDB, 162
getWallOffset()
OiPlElement, 145
getWallParams()
Wall, 164
getWidth()
Complex, 95
OiPart, 149


226


OiPlElement, 143
getWorldBounds()
Base, 84
getWorldGeoBounds()
Base, 85
getXArticleSpec()
Article, 99
OiPDManager, 159
Global planning object, 132
GO types, 8


hasProductKnowledge()
OiProductDB, 161
hasProperties()
Property, 91
hasProperty()
Property, 92
hide()
Base, 81
hierSelectable()
Base, 79
Hole, 122
Hyperlink, 114


Import of Geometries, 125
Information object, 136
Inheritance of features, 12
Initialization, 16
Instance, 10–12
Identity, 13
identity, 78
Initialization, 16
name, 13
variable, 11, 14
INTERACTOR, 108
Interactor, 17, 209
Interface, 11, 14
Interfaces
Basic interfaces, 77
Categories, 218
invalidatePicture()
Base, 87
isA()
MObject, 77
isCat()
MObject, 78
isCutable()
Base, 80



OiPropertyObj, 153
isElemCatValid()
OiPart, 150
OiPlElement, 144
isElOrderSubPos()
OiPart, 151
OiPlElement, 146
isEnabledCD()
Base, 82
isEnabledChildCD()
Complex, 97
isHidden()
Base, 81
isMatCat()
Material, 88
isSelectable()
Base, 80
isValidForCollCheck()
Complex, 97
OiPlanning, 137
OiProgInfo, 142


Language Selection, 133
Light, 210
Light source, 210
Link, 114


Material, 87
-definition, 201
Categories, 88, 218
measure()
Base, 82
Measurement line, 211
Measurement symbol, 213
Measurement text, 214
Metafile, 180
Method, 10, 14
Difference compared to rule, 15
MLine, 211
MObject, 77
Modal dialog, 111
Module, 10
moveTo()
Base, 83
MSymbol, 213
MText, 214


Name space


227


hierarchical, 13
Names
for entities, 13
of methods, 15
predefined, 13
reserved, 13
NEW ~~E~~ LEMENT, 104
Notation, 216
notHierSelectable()
Base, 79
notSelectable()
Base, 79


OAM, 8
OAS, 8
Object, 11
Object model, 8
object2Article()
OiPDManager, 158
objInLevel()
OiLevel, 165
OCD, 8
ODB, 8
2D Representation and ODB, 86
OEX, 8
OFF file, 200
OFML
Concepts, 10
Features, 7
Overview, 8
OFML database, 8
oiApplPaste(), 110
OiBlock, 118
oiClone(), 110
oiCollision(), 111
oiCopy(), 111
oiCut(), 111
OiCylinder, 119
oiDialog(), 111
oiDump2String(), 112
OiEllipsoid, 120
oiExists(), 113
OiFrame, 121
oiGetDistance(), 113
oiGetNearestObject(), 113
oiGetRoots(), 113
oiGetStringResource(), 113



OiHole, 122
OiHPolygon, 124
OiImport, 125
OiLevel, 164
oiLink(), 114
OiOdbPlElement, 154
oiOutput(), 114
OiPart, 148
oiPaste(), 114
OiPDManager, 157
OiPlanning, 132
OiPlElement, 142
OiPolygon, 126
OiProductDB, 160
OiProgInfo, 140
OiPropertyObj, 153
oiReplace(), 115
OiRotation, 127
oiSetCheckString(), 115
OiSphere, 128
OiSurface, 131
OiSweep, 129
oiTable(), 115
OiUtility, 153
OiWall, 166
OiWallSide, 166
onCreate()
OiPlElement, 145
onRotate()
OiPart, 152
onTranslate()
OiPart, 152
Open-form areas, 131


Pi, 79
PICK, 105
Planning check, 101
Planning element, 142
Planning environment, 135, 164
Planning hierarchy, 132
Planning limit, 133, 135
Planning mode, 165
Polygon, 124, 126
Price, 100
Primitive, 117
Procedure, 168
Product Data, 99


228


Product Data Management, 156
Product data management, 139
Product data model, 167
Product database, 204
Program Access, 98
Program Information, 140
Program information, 136
Property, 14, 89
Definition format, 207
Property Information, 94
propsChanged()
OiOdbPlElement, 155
Property, 93


Quboid, 118


Reference Types, predefined
CFunc, 31
Func, 31
Hash, 42
List, 38
String, 31
Type, 30
Vector, 36
Relational database, 115
remove()
MObject, 78
REMOVE ~~E~~ LEMENT, 104
removeProperty()
Property, 91
removeValid()
Base, 80
OiPropertyObj, 153
Resolution, 81
Resource, 204
Restoring an instance from a dump representation, 115
Root object, 113
ROTATE, 106
rotate()
Base, 83
rotated()
OiPlElement, 148
rotateValid()
OiPlElement, 148
Rotation, 83
Rotational body, 127
Rule, 10, 15, 85, 103



Difference compared to method, 15
explicit call, 85
predefined, 103
user-defined, 103


Sales region, 133
SAP/R3, 156
Scaling of geometries, 126
Scene, 12
Selectability, 79
selectable()
Base, 79
Selection criterion, 168
SENSOR, 108
setAlignment()
OiGeometry, 118
setArticleSpec()
Article, 99
OiOdbPlElement, 154
OiPart, 151
OiPlElement, 146
setBorder()
OiPlanning, 135
setChanged()
Base, 81
setCMaterial()
Material, 89
OiPlanning, 136
OiProgInfo, 141
setCutable()
Base, 80
setDataBasePath()
OiProductDB, 161
setDefaultHeight()
OiLevel, 165
setDepth()
OiPart, 149
OiPlElement, 143
setErrorLog()
OiPlanning, 135
setExtPropOffset()
Property, 91
setHeight()
OiPart, 149
OiPlElement, 143
setLanguage()
OiPlanning, 133



229


setMatCat()
OiGeometry, 118
setMethod()
Complex, 96
setOdbType()
OiOdbPlElement, 154
setOrderID()
Article, 98
setOrigin()
OiPart, 149
OiPlElement, 143
setPasteMode()
Complex, 96
setPDManager()
OiPlanning, 139
setPlanningWall()
OiLevel, 165
setPlProgram()
OiPlElement, 143
setPosition()
Base, 82
setProgram()
OiPlanning, 134
setPrograms()
OiProductDB, 161
setPropPosOnly()
Property, 91
setPropState()
Property, 94
setPropValue()
OiOdbPlElement, 155
Property, 93
setRegion()
OiPlanning, 133
setResolution()
Base, 81
setRtAxis()
Base, 84
setTempArticleSpec()
Complex, 96
setTrAxis()
Base, 83
setUnchanged()
Base, 82
setupProperty()
Property, 89
setupProps()



OiPDManager, 158
setWidth()
OiPart, 149
OiPlElement, 143
setXArticleSpec()
Article, 100
OiPDManager, 159
show()
Base, 81
SPATIAL ~~M~~ ODELING, 106
Sphere, 128
START ~~D~~ UMP, 107
START ~~E~~ VAL, 107
startCollCheck()
OiProgInfo, 142
Structure of Order Lists, 98


Table, external, 204
Text output, 114
Text resource, 204
TIMER, 108
Topology
Name space, 13
Scene, 12
topological independence, 11
TRANSLATE, 105
translate()
Base, 83
translated()
OiOdbPlElement, 155
OiPlElement, 147
translateValid()
OiPlElement, 147
Translation, 83
Type, 10
abstract, 10
Uniqueness, 10
Type identity, 77


unMeasure()
Base, 82
UNPICK, 105


varCode2PValues()
OiProductDB, 162
Visibility, 81


Wall, 164


230


