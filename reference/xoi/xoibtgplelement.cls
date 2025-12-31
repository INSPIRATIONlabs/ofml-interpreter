// === ofml/xoi/xoibtgplelement.cls ===
package ::ofml::xoi;
import ::ofml::oi::*;

import ::stdio::*;






















































public class xOiBTGPlElement: OiOdbPlElement
{
    
    static var sMListTE = @(["params", 0, 0+0, @s, ""+"", "0"],
			    ["dwgname", 0, 0+0, @s, ""+"", "0"],
			    ["variant", 0, 0+0, @s, ""+"", "0"],
			    ["article", 0, 0+0, @s, ""+"", "0"]);
    static var sMaxVariantLen = 40;

    
    static var sMatTE = @(["series", 0, 0+0, @s, ""+"", "0"],
			  ["article", 0, 0+0, @s, ""+"", "0"],
			  ["property", 0, 0+0, @s, ""+"", "0"],
			  ["prop_val", 0, 0+0, @s, ""+"", "0"],
			  ["number", 0, 0+0, @i, 0+0, "0"],
			  ["layer", 0, 0+0, @s, ""+"", "0"],
			  ["material", 0, 0+0, @s, ""+"", "0"]);

    
    
    static var sMListInfo  = NULL; 
    static var sBestEndArt = NULL; 
    static var sEndArt     = NULL;

    
    
    static var sIsTmpObj   = 0;

    
    
    static var sChLPValTxtHashKey = @_XOIBTGPLEL_CHLPVALTXT;

    var mArticleSet;
    var mArticle;
    var mVarPrefix;

    var mXoiProgram;
    var mXoiAddOdbParams;

    
    static var sxOiBTGPlElementSetGetMembers =
    @(@ChfPropValues, @VarPrefix, @AddOdbParams, @MatTableID, @BTGMListTableID, 
      @CadVarValue);

    
    public func initialize(pFa, pNa)
    {
	OiOdbPlElement::initialize(pFa, pNa);
    }

    public func forceDynamicProp(pPKey)
    {
	if (xOiBTGPlElement.sxOiBTGPlElementSetGetMembers.find(pPKey) >= 0) 
	    return(1);

	return(OiOdbPlElement::forceDynamicProp(pPKey));
    }

    public func manualSetMembers(pMembers)
    {
	var tRes = OiOdbPlElement::manualSetMembers(pMembers);

	if (!pMembers.hasKey(@mArticleSet))
	    tRes[1].pushBack(@mArticleSet);
	if (!pMembers.hasKey(@mXoiProgram))
	    tRes[1].pushBack(@mXoiProgram);
	if (!pMembers.hasKey(@mXoiAddOdbParams))
	    tRes[1].pushBack(@mXoiAddOdbParams);

	return (tRes);
    }

    
    public func setProgram(pPID)
    {
	OiOdbPlElement::setProgram(pPID);
	mXoiProgram = pPID;
    }

    
    
    
    
    public func getProgram()
    {
	if (mXoiProgram != NULL) return(mXoiProgram);

	return(OiOdbPlElement::getProgram());
    }

    
    
    
    
    
    
    
    
    public func getMetaInfo(pInfoType, pArgs)
    {
	return(xOiGetMetaInfo(self, pInfoType, pArgs));
    }

    
    
    

    
    
    protected func needMListCheck()
    {
	return(1);
    }

    
    
    
    
    public func propsChanged(pPKeys, pDoChecks)
    {
	xOiDebugStartFunc("xOiBTGPlElement::propsChanged", 
			  [pPKeys, pDoChecks]);
	xOiDebugPrint(@Info, 1, ["self is ", self]);

	var tRet = 0;

	if (!needMListCheck()) {
	    
	    
	    
	    invalidatePicture();
	    createOdbChildren(@INCR);
	    tRet = 1;
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tPDM = getPDManager();
	var tPDB = tPDM.getPDBFor(self);

	tPDM.startTransaction(self, mArticle);

	var tPValues = tPDM.buildPropValList(self, tPDB, mArticle);
	sEndArt = getXArticleSpec(@Final);

	tPDM.finishTransaction(self);

	sMListInfo = readMListInfo(mArticle, sEndArt, tPDB, tPValues);

	var tUpdateGeo = 1;

	if (pDoChecks) {
	    if (sMListInfo == NULL) {
	    	oiOutput(@WARNING, "::ofml::xoi::@nobtgmlist");
		xOiDebugPrint(@Warn, 1, 
			      oiGetStringResource(
				"::ofml::xoi::@nobtgmlist", "de"));
		tUpdateGeo = 0;
	    }
	    else 
	    if (sMListInfo.empty()) {
	    	oiOutput(@WARNING, "::ofml::xoi::@nodwgname");
		xOiDebugPrint(@Warn, 1, 
			      oiGetStringResource(
				"::ofml::xoi::@nodwgname", "de"));
		tUpdateGeo = 0;
	    }
	}

	if (tUpdateGeo) {
	    
	    
	    invalidatePicture();
	    createOdbChildren(@INCR);
	    tRet = 1;
	}

	sMListInfo  = NULL;
	sBestEndArt = NULL;
	sEndArt	    = NULL;

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    

    
    
    
    
    
    
    
    
    public func setArticleSpec(pArticle)
    {
	var tPlan = getPlanning();
	if (tPlan != NULL && tPlan.isAtEval()) return;

	xOiDebugStartFunc("xOiBTGPlElement::setArticleSpec", pArticle);

	if (pArticle != NULL) {
	    mArticleSet = (pArticle != mArticle);
	    mArticle = pArticle;
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    public func getArticleSpec()
    {
	return(mArticle);
    }

    
    
    
    
    protected func prepareVarCode(pSpec)
    {
	xOiDebugStartFunc("xOiBTGPlElement::prepareVarCode", pSpec);

	var tSpec = xOiPrepareVarCode(self, pSpec);

	xOiDebugFinishFunc(tSpec);
	return(tSpec);
    }

    
    
    
    
    public func setXArticleSpec(pMode, pSpec)
    {
	var tPlan = getPlanning();
	if (tPlan != NULL && tPlan.isAtEval()) return;

	xOiDebugStartFunc("xOiBTGPlElement::setXArticleSpec", [pMode, pSpec]);

	if (!(pMode == @VarCode || pMode == @OFMLVarCode || pMode == @Final)) {
	    OiOdbPlElement::setXArticleSpec(pMode, pSpec);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tAfterSetArt = (mArticleSet != NULL && mArticleSet);

	if (tAfterSetArt)
	    xOiDebugPrint(@Info, 1, "changed base article");

	var tPDM = getPDManager();
	var tPDB = tPDM.getPDBFor(self);

	var tSpec = pSpec;

	if (pMode != @OFMLVarCode && pSpec.substr(0, 2) == "XX") {
	    var tArticle = mArticle;
	    if (pSpec.size() >= tArticle.size() + 3 &&
		pSpec.substr(3, tArticle.size()) == tArticle) {
	    	mVarPrefix = pSpec.substr(0, 3);
	    	tSpec = pSpec.substr(3);
	    }
	    else
		mVarPrefix = NULL;
	}
	else {
	    mVarPrefix = NULL;
	}

	xOiDebugPrint(@Info, 2, ["variant prefix: ", mVarPrefix]);

	if (pMode == @VarCode)
	    if (tSpec.empty())
		tSpec = mArticle;
	    else
		tSpec = prepareVarCode(tSpec);

	xOiDebugPrint(@Info, 2, ["(partially defined) article code: ", tSpec]);

	if (pMode != @OFMLVarCode && (tSpec == NULL || tSpec.empty())) {
	    xOiDebugPrint(@Warn, 1, "no article spec!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	tPDM.startTransaction(self, mArticle);

	setODBType(mArticle);
	assignArticleParams(mArticle);

	if (pMode != @OFMLVarCode)
	    tPDM.setupProps(self, tSpec);
	else
	    tPDM.setupProps(self, mArticle, pSpec, pMode);

	tAfterSetArt = 0;

	invalidatePicture();

	createOdbChildren(@NEW);

	tPDM.finishTransaction(self);

	sEndArt	    = NULL;

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    public func getVarPrefix()
    {
	return(mVarPrefix);
    }

    
    
    
    
    
    
    public func getXArticleSpec(pType)
    {
	xOiDebugStartFunc("xOiBTGPlElement::getXArticleSpec", pType);

	var tRet = OiOdbPlElement::getXArticleSpec(pType);

	xOiDebugPrint(@Info, 2, ["inherited spec: ", tRet]);

	if (pType != @VarCode && pType != @Final) {
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	if (pType == @VarCode && tRet != NULL && !tRet.empty() &&
	    tRet[0] == '@' && mVarPrefix != NULL)
	    tRet = mVarPrefix+mArticle+tRet;
	else
	if (pType != @Base && mVarPrefix != NULL &&  
	    tRet != NULL && !tRet.empty() &&
	    tRet.size() >= mVarPrefix.size() &&
	    tRet.substr(0, mVarPrefix.size()) != mVarPrefix)
	    tRet = mVarPrefix+tRet;

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getAddStateCode(pDomain)
    {
	xOiDebugStartFunc("xOiBTGPlElement::getAddStateCode", pDomain);

	var tRes = "";

	if (pDomain == NULL || pDomain != @xOiBTGPlElement)
	    tRes = OiOdbPlElement::getAddStateCode(pDomain);

	if (pDomain != NULL && pDomain != @xOiBTGPlElement) {
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	 }

	var tMyCode = "";
	if (mVarPrefix != NULL)
	    tMyCode = "VarPrefix='" + mVarPrefix +"'";

	if (pDomain == @xOiBTGPlElement)
	    tRes = tMyCode;
	else
	if (pDomain == NULL && !tMyCode.empty())
	    tRes.pushBack([@xOiBTGPlElement, tMyCode]);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setAddStateCode(pDomain, pCode)
    {
	xOiDebugStartFunc("xOiBTGPlElement::setAddStateCode", [pDomain, pCode]);

	if (pDomain == NULL && !(pCode instanceof Vector)) {
	    xOiDebugPrint(@Warn, 1, "invalid code!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tMyCode, tBaseCode = Vector(0);

	if (pDomain == NULL) {
	    var tSection;
	    foreach(tSection; pCode) {
		var tDomain = tSection[0];
		var tCode   = tSection[1];
		if (tDomain == @xOiBTGPlElement)
		    tMyCode = tCode;
		else
		    tBaseCode.pushBack([tDomain, tCode]);
	    }
	}
	else {
	    if (pDomain != @xOiBTGPlElement)
		tBaseCode = pCode;
	    else
		tMyCode = pCode;
	}

	if (pDomain == NULL || pDomain != @xOiBTGPlElement)
	    OiOdbPlElement::setAddStateCode(pDomain, tBaseCode);

	if (tMyCode == NULL || tMyCode.empty()) {
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tCodes = xOiList2Hash(oiParseOAMParams(tMyCode));

	xOiDebugPrint(@Info, 2, ["codes: ", tCodes]);
 
	if (tCodes != NULL && tCodes.hasKey(@VarPrefix)) {
	    mVarPrefix = tCodes[@VarPrefix];
	    xOiDebugPrint(@Info, 1, ["assigend varprefix: ", mVarPrefix]);
	    invalidatePicture();
	    createOdbChildren(@INCR);
	}

	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    
    
    

    
    
    
    
    public func getOdbInfo()
    {
	xOiDebugStartFunc("xOiBTGPlElement::getOdbInfo", NULL);
	xOiDebugPrint(@Info, 1, ["self is ", self]);

	var tPlan = self.getPlanning();
	var tAtPaste = 0;
	if (tPlan != NULL && tPlan.hasMember(@atObjectPaste))
	    tAtPaste = tPlan.atObjectPaste();

	var tAInfo = getBasicOdbInfo();

	xOiDebugPrint(@Info, 3, ["base odb info: ", tAInfo]);

	var tP;

	if (tAtPaste && mXoiAddOdbParams != NULL) {
	    xOiDebugPrint(@Info, 1, "using saved additional parameters");
	    foreach(tP; mXoiAddOdbParams) tAInfo[tP[0]] = tP[1];
	    xOiDebugFinishFunc(tAInfo);
	    return(tAInfo);
	}

	var tBaseArt = mArticle;

	
	var tEndArt  = sEndArt;
	if (tEndArt == NULL)
	    tEndArt  = getXArticleSpec(@Final);

	xOiDebugPrint(@Info, 2, ["base article: ", tBaseArt]);
	xOiDebugPrint(@Info, 2, ["end article: ", tEndArt]);

	if (tBaseArt == NULL) {
	    xOiDebugPrint(@Warn, 1, 
			  "NULL base article! "+
			  "(missing call of setArticleSpec()?)");
	    xOiDebugFinishFunc(tAInfo);
	    return(tAInfo);
	}

	mXoiAddOdbParams = getAddOdbParams(tBaseArt, tEndArt, 1, 0);
	foreach(tP; mXoiAddOdbParams) tAInfo[tP[0]] = tP[1];

	try { applyOdbParams(tAInfo); }

	xOiDebugFinishFunc(tAInfo);
	return(tAInfo);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func getAddOdbParams(pArticle, pVariant, pInclMats, pIsTmpObj)
    {
	xOiDebugStartFunc("xOiBTGPlElement::getAddOdbParams", 
			  [pArticle, pVariant, pInclMats, pIsTmpObj]);

	sIsTmpObj = pIsTmpObj;

	var tAInfo = Hash();

	var tPDM = getPDManager();
	if ((!pIsTmpObj || pInclMats) && tPDM.isA(xOiPDManager)) 
	    tPDM.startTransaction(self, pArticle);

	
	addMListInfo(tAInfo, pArticle, pVariant);

	
	if (pInclMats) addMaterials(tAInfo, pArticle, pVariant);

	if ((!pIsTmpObj || pInclMats) && tPDM.isA(xOiPDManager)) 
	    tPDM.finishTransaction(self);

	var tParams = Vector(0);
	var tK;
	foreach(tK; tAInfo.keys()) {
	    var tKey = tK;
	    tParams.pushBack([tKey, tAInfo[tK]]);
	}

	xOiDebugFinishFunc(tParams);
	return(tParams);
    }

    
    protected func getMatTableID()
    {
	var tLibPath = getPlanning().getLibPath(getProgram());

	if (tLibPath != NULL) 
	    return("EBASE " + tLibPath + "/odb.ebase mat");

	xOiDebugPrint(@ExplWarn, 1, ["no lib path for program ", getProgram()]);

	return(NULL);
    }

    
    
    
    
    
    
    
    
    public func readMatLayers(pBaseArt, pEndArt)
    {
	xOiDebugStartFunc2("xOiBTGPlElement::readMatLayers", 
			   [pBaseArt, pEndArt]);

	var tTblID = getMatTableID();

	if (tTblID == NULL) {
	    xOiDebugPrint(@Warn, 1, "no lib path 1?");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tTable = xOiTable2(tTblID, sMatTE);

	xOiDebugPrint(@Info, 2, ["table ID: ", tTblID]);

	if (!tTable.open()) {
	    xOiDebugPrint(@Warn, 1, ["cannot open table : ", tTblID]);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tSeries = xOiParseString2List(xOiGetAppSeriesID(getProgram()), ';');
	if (tSeries == NULL)
	    tSeries = @(xOiProgID2Series(getProgram()).toUpper());

	xOiDebugPrint(@Info, 1, ["series: ", tSeries]); 

	
	var tEntries = @();
	var tS;
	foreach(tS; tSeries) {
	    var tRes = tTable.
		       readNEntriesFor(@(["series", tS], ["article", pBaseArt]),
	  			       @("property", "prop_val", "number", 
				         "layer", "material"), 0);
	    if (tRes != NULL) xOiCopyAggr(tRes, tEntries, 1);
	    
	    tRes = tTable.readNEntriesFor(@(["series", tS], ["article", "*"]),
					  @("property", "prop_val", "number",
					    "layer", "material"), 0);
	    if (tRes != NULL) xOiCopyAggr(tRes, tEntries, 1);
	}
	tTable.close();

	if (tEntries.empty()) {
	    xOiDebugPrint(@Warn, 1, 
			  ["no entries in mat table for article : ", 
			   pBaseArt]);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	xOiDebugPrint(@Info, 2, ["entries: ", tEntries]); 

	var tRet = Hash();
	var tEntry;
	foreach(tEntry; tEntries) {
	    var tProp  = tEntry[0];
	    var tPVal  = tEntry[1];
	    var tNum   = tEntry[2];
	    var tLayer = tEntry[3];
	    var tMat   = tEntry[4];

	    if (!tRet.hasKey(tProp)) tRet[tProp] = Hash();
	    var tPropHash = tRet[tProp];

	    if (!tPropHash.hasKey(tPVal)) tPropHash[tPVal] = Vector(0);
	    var tPValInfo = tPropHash[tPVal];
	    tPValInfo.pushBack([tLayer, tMat]);
	}

	xOiDebugFinishFunc(tRet);
	return (tRet);
    }

    
    
    

    
    protected func getBTGMListTableID()
    {
	var tLibPath = getPlanning().getLibPath(getProgram());

	if (tLibPath != NULL) 
	    return("EBASE " + tLibPath + "/odb.ebase btgmlist");

	xOiDebugPrint(@ExplWarn, 1, ["no lib path for program ", getProgram()]);

	return(NULL);
    }

    
    
    
    
    
    
    
    protected func readMListInfo(pBaseArt, pEndArt, pPDB, pPValues)
    {
	xOiDebugStartFunc2("xOiBTGPlElement::readMListInfo", 
			   [pBaseArt, pEndArt, pPDB, pPValues]);

	var tRes = readMListInfoEntry(pBaseArt, pEndArt, pPDB, pPValues);

	if (tRes == NULL) {
	    xOiDebugPrint(@Warn, 1, ["no data in btgmlist for ", pBaseArt]);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tEndArt;
	if (mVarPrefix != NULL && pEndArt != NULL && 
	    pEndArt.size() >= mVarPrefix.size() && 
	    pEndArt.substr(0, mVarPrefix.size()) != mVarPrefix)
	    tEndArt = mVarPrefix + pEndArt;
	else
	    tEndArt = (pEndArt == NULL) ? "" : pEndArt;

	var tMListInfo = []; 

	if (tRes.size() == 1 && tRes[0][2] == "") {
	    
	    
	    tMListInfo = tRes[0];
	    xOiDebugFinishFunc(tMListInfo);
	    return(tMListInfo);
	}

	if (tEndArt.size() > sMaxVariantLen) tEndArt.resize(sMaxVariantLen);

	
	var tDiffs = (pEndArt == NULL) ? 0 : pEndArt.size();
	var tExactMatches = 0;
	var tFound = 0;
	var tR;
	foreach(tR; tRes) {
	    var tInfo = tR;
	    var tVariant = tInfo[2];

	    var tVariantSize = variantSize(tVariant);
	    var tEndArtSize = tEndArt.size();
	    for (; tVariantSize < tEndArtSize; ++tVariantSize)
		tVariant = tVariant + "?";
	    for (; tEndArtSize < tVariantSize; ++tEndArtSize)
		tEndArt = tEndArt + " ";

	    xOiDebugPrint(@Info, 2, 
			  ["compare ", tVariant, "[", tVariant.size(), "]"]);
	    xOiDebugPrint(@Info, 2, 
			  ["with    ", tEndArt]);

	    if (tVariant.fnmatch(tEndArt, 0)) {
		
		var tEM = exactMatches(tInfo[2], tEndArt);
		if (tEM > tExactMatches) {
		    tExactMatches = tEM;
		    tMListInfo = tInfo;
		    tFound = 1;
		}
	    }
	    else 
	    if (!tFound) {
		var tD = diffCharCount(tVariant, tEndArt);
		if (tD[0] < tDiffs) {
		    tDiffs = tD[0];
		    sBestEndArt = tD[1];
		}
	    }
	}
	if (tMListInfo.empty() && sIsTmpObj) {
	    xOiDebugPrint(@Info, 1, "using first entry for base article");
	    tMListInfo = tRes[0];
	}
	xOiDebugFinishFunc(tMListInfo);
	return(tMListInfo);
    }

    
    
    protected func readMListInfoEntry(pBaseArt, pEndArt, pPDB, pPValues)
    {
	var tTblID = getBTGMListTableID();

	if (tTblID == NULL) return(NULL); 

	var tTable = xOiTable2(tTblID, sMListTE);

	xOiDebugPrint(@Info, 2, ["table ID: ", tTblID]);

	if (!tTable.open()) {
	    xOiDebugPrint(@Warn, 1, ["cannot open table : ", tTblID]);
	    return(NULL);
	}

	
	var tRes = tTable.readNEntriesFor(@(["article", pBaseArt]),
	  				  @("params", "dwgname", "variant"), 0);
	tTable.close();

	return (tRes);
    }

    
    
    
    
    
    
    
    
    
    protected func addMListInfo(pHash, pBaseArt, pEndArt)
    {
	xOiDebugStartFunc2("xOiBTGPlElement::addMListInfo", 
			   [pBaseArt, pEndArt]);

	var tPDM = getPDManager();
	var tPDB = tPDM.getPDBFor(self);

	tPDM.startTransaction(self, pBaseArt);
	var tPValues = tPDM.buildPropValList(self, tPDB, pBaseArt);
	tPDM.finishTransaction(self);

	var tMListInfo;
	if (sMListInfo != NULL) 
	    tMListInfo = sMListInfo;
	else
	    tMListInfo = readMListInfo(pBaseArt, pEndArt, tPDB, tPValues);

	xOiDebugPrint(@Info, 2, ["btgmlist entry: ", tMListInfo]);

	if (tMListInfo == NULL) {
	    
	    
	    xOiDebugPrint(@ExplWarn, 1, 
			  [oiGetStringResource(
				"::ofml::xoi::@nobtgmlist", "de"), 
			   " Article ", pBaseArt]);
	    
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	if (tMListInfo.empty()) {
	    
	    xOiDebugPrint(@ExplWarn, 1, 
			  [oiGetStringResource(
				"::ofml::xoi::@nodwgname", "de"),
			   " (End)Article ", pEndArt]);
	    pHash[@DWGNAME] = "";
	    
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	pHash[@DWGNAME] = tMListInfo[1];

	var tParams = NULL;
	try {
	    tParams = eval(tMListInfo[0]);
	}
	catch(&e: Error) {
	    xOiDebugPrint(@EvalErr, 1, 
			  ["error during evaluation of btgmlist parameter ",
			   "field '", tMListInfo[0], "' for article ", pBaseArt,
			   " (", self.getProgram(), ") :", e.what]);
	    
	}

	if (tParams == NULL || tParams.empty()) {
	    addAdditionalMListInfo(tMListInfo, pHash);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var i = 0;
	while (i < tParams.size()-1) {
	    var tPName = tParams[i++];
	    var tPValue = tParams[i++];
	    if (tPValue instanceof String && tPValue.size() > 0 &&
		tPValue[0] == '@' && !sIsTmpObj) {
		
		
		
		
		
		
		
		var tVal = getCadVarValue(pBaseArt, pEndArt, tPValue.substr(1),
					  tPDB, tPValues);
		if (tVal != NULL) tPValue = tVal;
		var tName2 = String(tPName);
		if (tName2.substr(0, 2) == "_F")
		    tPValue = Float(tVal);
		else {
		    
		    
		    var tFloat = NULL;
		    try { tFloat = Float(tVal); }
		    if (tFloat != NULL) {
			tName2[1] = 'F';
	    		pHash[Symbol(tName2)] = Float(tVal);
		    }
		}
	    }
	    pHash[tPName] = tPValue;
	}

	addAdditionalMListInfo(tMListInfo, pHash);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    protected func addAdditionalMListInfo(pMListInfo, pHash)
    {
    }

    
    
    
    
    
    
    protected func getCadVarValue(pBaseArt, pEndArt, pVariant, pPDB, pPValues)
    {
	xOiDebugStartFunc2("xOiBTGPlElement::getCadVarValue", 
			   [pBaseArt, pEndArt, pVariant, pPDB, pPValues]);

	var tVal = NULL;

	var tPDM  = getPDManager();
	var tPKey = Symbol(pVariant);
	xOiDebugPrint(@Info, 1, 
		      ["using property ", tPKey, " for resolving CAD variant"]);

	if (self.hasProperty(tPKey)) {
	    tVal = self.getPropValue(tPKey);
	    xOiDebugPrint(@Info, 2, ["property value: ", tVal]);
	    var tPName = NULL;
	    if (tPDM.isA(xOiPDManager))
		tPName = tPDM.propKey2PropName(self, tPKey);
	    if (tPName != NULL && pPDB != NULL && 
		pPDB.hasMember(@getNativeValue)) {
	        xOiDebugPrint(@Info, 2, "getting native value from PDB");
		tVal = pPDB.getNativeValue(pBaseArt, "", tPName, tVal);
	    }
	}
	else
	    xOiDebugPrint(@Warn, 1, 
			  ["self ", self, " has no property ", tPKey]);

	xOiDebugFinishFunc(tVal);
	return(tVal);
    }

    
    
    
    
    
    
    
    
    
    
    
    protected func addMaterials(pHash, pBaseArt, pEndArt)
    {
	var tPDM = getPDManager();
	var tPDB = tPDM.getPDBFor(self);

	if (tPDB == NULL) return;

	if (!tPDB.hasMember(@getMatLayers)) {
	    xOiDebugPrint(@EvalErr, 1, 
			  ["product database ", tPDB, 
			   " does not implement method getMatLayers()!"]);
	    return;
	}

	var tPckg = getMatPackage();
	var tMats = tPDB.getMatLayers(pBaseArt, pEndArt, self);

	if (tMats == NULL) tMats = @();

	var tM;
	foreach(tM; tMats) pHash[_xOiString2Symbol(tM[0], NULL)] = tPckg+tM[1];
    }

    

    private func variantSize(pVariant)
    {
	var tRes = 0, tIdx, tSize = pVariant.size();
	for (tIdx = 0; tIdx < tSize; ++tIdx) {
	    if (pVariant[tIdx] == '\\')
		++tIdx;
	    ++tRes;
	}
	return (tRes);
    }

    private func diffCharCount(pVariant, pEndArt)
    {
	var tEndArt = pEndArt;
	var tDiffs  = 0;

	var i, j, tSize = pVariant.size(), tQuoted;
	for (i = j = 0; i < tSize; ++i, ++j) {
	    if ((tQuoted = pVariant[i] == '\\') && ++i == tSize)
		break;
	    if ((pVariant[i] != '?' || tQuoted) && pVariant[i] != pEndArt[j]) {
		tDiffs++;
		tEndArt[j] = pVariant[i];
	    }
	}

	return([tDiffs, tEndArt]);
    }

    private func exactMatches(pVariant, pEndArt)
    {
	var tExactMatches = 0;
	var i, j, tSize = pVariant.size();
	for (i = j = 0; i < tSize; ++i, ++j) {
	    if (pVariant[i] == '\\' && ++i == tSize)
		break;
	    if (pVariant[i] == pEndArt[j])
		tExactMatches++;
	}
	return (tExactMatches);
    }
}

