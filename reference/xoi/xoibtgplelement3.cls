// === ofml/xoi/xoibtgplelement3.cls ===
package ::ofml::xoi;
import ::ofml::oi::*;


































































public class xOiBTGPlElement3 : xOiBTGPlElement2
{
    
    static var sMListTE3 = @(["params",    0, 0+0, @s, ""+"",   "0"],
			     ["dwgname",   0, 0+0, @s, ""+"",   "0"],
			     ["variant",   0, 0+0, @s, ""+"",   "0"],
			     ["article",   0, 0+0, @s, ""+"",   "0"],
      			     ["series",    0, 0+0, @s, ""+"",   "0"],
      			     ["width",     0, 0+0, @f, 0.0+0.0, "0.0"],
      			     ["depth",     0, 0+0, @f, 0.0+0.0, "0.0"],
      			     ["height",    0, 0+0, @f, 0.0+0.0, "0.0"],
      			     ["ins_angle", 0, 0+0, @f, 0.0+0.0, "0.0"],
			     ["attr_flgs", 0, 0+0, @i, 0+0,     "0"],
			     ["sattr",     0, 0+0, @s, ""+"",   "0"]);

    
    static var sMListTE4 = @(["params",    0, 0+0, @s, ""+"", "0"],
			     ["dwgname",   0, 0+0, @s, ""+"", "0"],
			     ["variant",   0, 0+0, @s, ""+"", "0"],
			     ["article",   0, 0+0, @s, ""+"", "0"],
      			     ["series",    0, 0+0, @s, ""+"", "0"],
      			     ["width",     0, 0+0, @s, ""+"", "0.0"],
      			     ["depth",     0, 0+0, @s, ""+"", "0.0"],
      			     ["height",    0, 0+0, @s, ""+"", "0.0"],
      			     ["ins_angle", 0, 0+0, @s, ""+"", "0.0"],
			     ["attr_flgs", 0, 0+0, @i, 0+0,   "0"],
			     ["sattr",     0, 0+0, @s, ""+"", "0"]);

    
    static var sVarTblStruct = @(["varkey",  0, 0+0, @s, ""+"", "0"],
				 ["option",  0, 0+0, @s, ""+"", "0"],
				 ["value",   0, 0+0, @s, ""+"", "0"],
				 ["weight",  0, 0+0, @s, ""+"", "0"],
				 ["negate",  0, 0+0, @s, ""+"", "0"]);

    
    
    
    
    
    
    
    public var mXoiFixDimensions;

    
    
    
    
    
    
    public var mXoiUseVarKeys;

    
    
    
    
    
    public var mXoiUseOAM;

    
    public func initialize(pFa, pNa)
    {
	xOiBTGPlElement2::initialize(pFa, pNa);

	mXoiFixDimensions = 1;
	mXoiUseVarKeys = 0;
	mXoiUseOAM = 0;

	var tTbl = openDataTbl();
	if (tTbl == NULL) return;

	

	var tRes = tTbl.read1EntryFor(@(["type", "@UseVarKeys"]),
				      @("args", "value"), 0);
	if (tRes != NULL) {
	    var tUseVarKeys;
	    try {
		tUseVarKeys = Int(tRes[1]);
		mXoiUseVarKeys = tUseVarKeys;
	    }
	    catch(&e: Error) {
		xOiDebugPrint(@EvalErr, 1,
			      "could not evaluate value for UseVarKeys: " +
			      e.what);
	    }
	}
	tRes = tTbl.read1EntryFor(@(["type", "@FixDimensions"]),
				  @("args", "value"), 0);
	if (tRes != NULL) {
	    var tFixDimensions;
	    try {
		tFixDimensions = Int(tRes[1]);
		mXoiFixDimensions = tFixDimensions;
	    }
	    catch(&e: Error) {
		xOiDebugPrint(@EvalErr, 1,
			      "could not evaluate value for FixDimensions: " +
			      e.what);
	    }
	}

	readUseOAM(tTbl);

	tTbl.close(); 
    }

    private func readUseOAM(pTbl)
    {
	var tRes = pTbl.read1EntryFor(@(["type", "@UseOAM"]),
				      @("args", "value"), 0);
	if (tRes != NULL) {
	    var tUseOAM;
	    try {
		tUseOAM = Int(tRes[1]);
		mXoiUseOAM = tUseOAM;
	    }
	    catch(&e: Error) {
		xOiDebugPrint(@EvalErr, 1,
			      "could not evaluate value for UseOAM: " + e.what);
	    }
	}
    }

    public func useOAM()
    {
	return(mXoiUseOAM);
    }

    
    
    
    
    
    public func getAddOdbParams(pArticle, pVariant, pInclMats, pIsTmpObj)
    {
	xOiDebugStartFunc("xOiBTGPlElement3::getAddOdbParams",
			  [pArticle, pVariant, pInclMats, pIsTmpObj]);

	var tParams;

	if (useOAM()) {
	    var tPDM = getPDManager();
	    if ((!pIsTmpObj || pInclMats) && tPDM.isA(xOiPDManager))
		tPDM.startTransaction(self, pArticle);

	    var tPMgr = oiGetOdbParamManager();
	    tParams = tPMgr.getAddOdbParams(self, pInclMats);

	    if ((!pIsTmpObj || pInclMats) && tPDM.isA(xOiPDManager))
		tPDM.finishTransaction(self);
	}
	else
	    tParams = xOiBTGPlElement2::getAddOdbParams(pArticle, pVariant, 
							pInclMats, pIsTmpObj);
	xOiDebugFinishFunc(tParams);
	return(tParams);
    }

    

    
    
    
    protected func needMListCheck()
    {
	return(!useOAM());
    }

    
    
    protected func prepareVarCode(pSpec)
    {
	xOiDebugStartFunc2("xOiBTGPlElement3::prepareVarCode", pSpec);

	if (!mXoiUseVarKeys) {
	    xOiDebugPrint(@Info, 1, "no handling for varkeys");
	    var tRes = xOiBTGPlElement2::prepareVarCode(pSpec);
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tPDB = getPDManager().getPDBFor(self);

	var tSpec = getArticleSpec();  
	var tVarcode = pSpec;
	var tAt = pSpec.find("@");
	
	
	if (tAt < 0) {
	    xOiDebugPrint(@Info, 1, "no special varcode");
	    var tRes = xOiBTGPlElement2::prepareVarCode(pSpec);
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tSep = "";
	var tIsAEPDFProductDB = 
	    (tPDB != NULL &&
	     (tPDB.isA(xOiEPDFProductDB) || tPDB.isA(xOiNativeOCDProductDB)));
	if (tIsAEPDFProductDB)
            tSep = tPDB.getSpecSeparator(tSpec);
	if (!tIsAEPDFProductDB || tPDB.getVarCodeType(tSpec) != @UserDefined)
	    tSpec += tSep + pSpec.substr(0, tAt); 
	else
	    tSpec = pSpec.substr(0, tAt);
	tVarcode = pSpec.substr(tAt+1);

	if (!tVarcode.empty() && tSpec != tVarcode) {
	    var tVarKeys = xOiParseString2List(tVarcode, ',');
	    xOiDebugPrint(@Info, 2, ["varkeys: ", tVarKeys]);
	    var tVarKey;
	    var t1st = 1;
	    var tNoEntries = 1;
	    foreach(tVarKey; tVarKeys) {
		var tVarTblEntries = readVarTableEntries(tVarKey);
		if (tVarTblEntries == NULL) continue;
		tNoEntries = 0;
		var tUsedProps = @();
		var tVarTblEntry;
		foreach(tVarTblEntry; tVarTblEntries) {
		    if (tUsedProps.find(tVarTblEntry[0]) < 0) {
			tSpec += (t1st ? "@" : ";") + 
			         tVarTblEntry[0] + "=" + tVarTblEntry[1];
			if (t1st) t1st = 0;
			tUsedProps.pushBack(tVarTblEntry[0]);
		    }
		    else
			xOiDebugPrint(@EvalErr, 1, 
				      ["multiple entries for property ", 
				       tVarTblEntry[0], " under varkey ", 
				       tVarKey, " (using first)!"]);
		}
	    }
	    if (tNoEntries) {
	        xOiDebugPrint(@Info, 1, 
			      "no entries in var.csv found for varcode");
		
		tSpec = xOiBTGPlElement2::prepareVarCode(pSpec);
	    }
	}

	xOiDebugFinishFunc(tSpec);
	return(tSpec);
    }

    
    
    
    protected func readMListInfo(pBaseArt, pEndArt, pPDB, pPValues)
    {
	xOiDebugStartFunc2("xOiBTGPlElement3::readMListInfo",
			   [pBaseArt, pEndArt, pPDB, pPValues]);

	if (!mXoiUseVarKeys) {
	    xOiDebugPrint(@Info, 1, "no handling for varkeys");
	    var tRes = xOiBTGPlElement2::readMListInfo(pBaseArt, pEndArt,
						       pPDB, pPValues);
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tRes = readMListInfoEntry(pBaseArt, pEndArt, pPDB, pPValues);

	if (tRes == NULL) {
	    xOiDebugPrint(@Warn, 1, ["no data in btgmlist for ", pBaseArt]);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	if (pPDB == NULL || !pPDB.hasMember(@getOFMLValue)) {
	    xOiDebugPrint(@Warn, 1, ["no (valid) PDB for ", self]);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tBestMatchInfo;
	var tBestMatchWeight = 0;
	var tR;
	foreach(tR; tRes) {
	    var tVariant = tR[2];
	    var tWeight = 0;
	    if (!tVariant.empty()) {
		var tVarKeys = xOiParseString2List(tVariant, ',');
		xOiDebugPrint(@Info, 2, ["varkeys: ", tVarKeys]);
		var tVarKey;
		foreach(tVarKey; tVarKeys) {
		    var tVTEL = readVarTableEntries(tVarKey);
		    if (tVTEL == NULL) {
			xOiDebugFinishFunc(NULL);
			return(NULL);
		    }
		    var tVTE;
		    foreach(tVTE; tVTEL)
			tWeight += _getWeight(pBaseArt, tVTE, pPValues, pPDB);
		}
	    }
	    if (tWeight > tBestMatchWeight || tBestMatchInfo == NULL) {
		tBestMatchInfo = tR;
		tBestMatchWeight = tWeight;
	    }
	}

	xOiDebugFinishFunc(tBestMatchInfo);
	return(tBestMatchInfo);
    }

    
    protected func readMListInfoEntry(pBaseArt, pEndArt, pPDB, pPValues)
    {
	var tTblID = getBTGMListTableID();

	if (tTblID == NULL) return(NULL); 

	var tTable = xOiTable2(tTblID, 
			       mXoiFixDimensions ? sMListTE3 : sMListTE4);

	xOiDebugPrint(@Info, 2, ["table ID: ", tTblID]);

	if (!tTable.open()) return(NULL);

	
	var tRes = tTable.readNEntriesFor(@(["article", pBaseArt]),
	  				  @("params", "dwgname", "variant", 
					    "series", "ins_angle",
					    "width", "depth", "height",
					    "attr_flgs", "sattr"), 0);
	tTable.close();

	if (tRes == NULL || mXoiFixDimensions) return(tRes);

	

	var tNewRes = @();
	var tEntry;
	foreach(tEntry; tRes) {
	    tNewRes.pushBack(
		[tEntry[0], tEntry[1], tEntry[2], tEntry[3],
		 
		 _getFloatVal(tEntry[4], pBaseArt, pEndArt, pPDB, pPValues),
		 
		 _getFloatVal(tEntry[5], pBaseArt, pEndArt, pPDB, pPValues),
		 
		 _getFloatVal(tEntry[6], pBaseArt, pEndArt, pPDB, pPValues),
		 
		 _getFloatVal(tEntry[7], pBaseArt, pEndArt, pPDB, pPValues),
		 tEntry[8], tEntry[9]]);
	}
	return(tNewRes);
    }

    
    
    protected func addAdditionalMListInfo(pMListInfo, pHash)
    {
	xOiDebugStartFunc2("xOiBTGPlElement3::addAdditionalMListInfo",
			   [pMListInfo, pHash]);

	xOiBTGPlElement2::addAdditionalMListInfo(pMListInfo, pHash);

	var tSAttrStr = pMListInfo[9];
	if (tSAttrStr.empty()) {
	    xOiDebugPrint(@Info, 1, "no attributes given");
	    xOiDebugFinishFunc(NULL);
	    return;
	}
	var tSAttrVec;
	try { 
	    tSAttrVec = eval(tSAttrStr);
	}
	if (!(tSAttrVec instanceof Vector)) {
	    xOiDebugPrint(@ExplWarn, 1, 
			  ["invalid attributes field: '", tSAttrStr, "'!"]);
	    xOiDebugFinishFunc(NULL);
	    return;
	} 
	var tIdx = 0, tLast = tSAttrVec.size() - 1;
	while (tIdx < tLast) {
	    var tSAttrName = tSAttrVec[tIdx++];
	    var tSAttrValue = tSAttrVec[tIdx++];
	    tSAttrName = Symbol("FOS_SATTR_" + String(tSAttrName));
	    pHash[tSAttrName] = tSAttrValue;
	}
	xOiDebugFinishFunc(NULL);
	return;
    }

    

    private func _getVarTableID()
    {
	var tLibPath = getPlanning().getLibPath(getProgram());

	if (tLibPath != NULL) 
	    return("EBASE " + tLibPath + "/odb.ebase var");

	xOiDebugPrint(@ExplWarn, 1, ["no lib path for program ", getProgram()]);

	return(NULL);
    }

    private func readVarTableEntries(pVarKey)
    {
	var tTblID = _getVarTableID();

	if (tTblID == NULL) return(NULL); 

	var tTable = xOiTable2(tTblID, sVarTblStruct);

	xOiDebugPrint(@Info, 2, ["table ID: ", tTblID]);

	if (!tTable.open()) return(NULL);

	
	var tRes = tTable.readNEntriesFor(@(["varkey", pVarKey]),
	  				  @("option", "value", 
					    "weight", "negate"), 0);
	tTable.close();

	if (tRes == NULL)
	    xOiDebugPrint(@EvalErr, 1, 
			  ["error reading table entry for varkey ", pVarKey,
			   " from ", tTblID]);
	return(tRes);
    }

    
    
    private func _getWeight(pBaseArt, pVarTableEntry, pPValues, pPDB)
    {
	xOiDebugStartFunc2("xOiBTGPlElement3::_getWeight", 
			   [pBaseArt, pVarTableEntry, pPValues, pPDB]);

	var tOption = pVarTableEntry[0];
	var tWeight = pVarTableEntry[2];
	if (tWeight.empty())
	    tWeight = 1;
	else
	    tWeight = Int(tWeight);

	var tNegate = 0;
	if (!pVarTableEntry[3].empty() && pVarTableEntry[3][0] == '#')
	    tNegate = 1;

	var tRes = 0;
	var tP;
	foreach(tP; pPValues)
	    if (tP[1] == tOption) {
		var tPVal1 = tP[2];
		var tPVal2 = pPDB.getOFMLValue(pBaseArt, tP[0], tP[1], 
					       pVarTableEntry[1]);
		if ((!tNegate && tPVal1 == tPVal2) ||
		    ( tNegate && tPVal1 != tPVal2)) tRes = tWeight;
		break;
	    }

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    private func _getFloatVal(pValStr, pBaseArt, pEndArt, pPDB, pPValues)
    {
	var tFloat = 0.0;
	var tFloat2 = NULL;

	if (pValStr.empty()) return(tFloat);

	if (pValStr[0] != '@') {
	    try { tFloat2 = Float(pValStr);}
	    if (tFloat2 != NULL)
		tFloat = tFloat2;
	    else
	        xOiDebugPrint(@EvalErr, 1, ["could not convert btgmlist value ",
					     pValStr, " to Float!"]);
	    return(tFloat);
	}

	if (pEndArt == NULL) return(tFloat);

	var tValStr = getCadVarValue(pBaseArt, pEndArt, pValStr.substr(1),
				     pPDB, pPValues);

	if (tValStr != NULL)
	    try { tFloat2 = Float(tValStr); }
	if (tFloat2 != NULL)
	    tFloat = tFloat2;
	else
	    xOiDebugPrint(@ExplWarn, 1, ["got no Float value for CAD variable ",
					 pValStr.substr(1)]);
	return(tFloat);
    }

    rule START_EVAL(pArg)
    {
	
	if (mXoiFixDimensions == NULL)
	    mXoiFixDimensions = 1;
	if (mXoiUseVarKeys == NULL)
	    mXoiUseVarKeys = 0;
	if (mXoiUseOAM == NULL)
	    mXoiUseOAM = 0;

	return(0);
    }

    rule FINISH_EVAL(pArg)
    {
	var tTbl = openDataTbl();
	if (tTbl != NULL) {
	    readUseOAM(tTbl);
	    tTbl.close(); 
	}

	return(0);
    }

    public func manualSetMembers(pMembers)
    {
        var tRes = xOiBTGPlElement2::manualSetMembers(pMembers);

        if (!pMembers.hasKey(@mXoiFixDimensions))
            tRes[1].pushBack(@mXoiFixDimensions);
        if (!pMembers.hasKey(@mXoiUseVarKeys))
            tRes[1].pushBack(@mXoiUseVarKeys);
        if (!pMembers.hasKey(@mXoiUseOAM))
            tRes[1].pushBack(@mXoiUseOAM);

        return (tRes);
    }
}

