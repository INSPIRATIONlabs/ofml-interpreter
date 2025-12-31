// === ofml/xoi/xoifuncs.cls ===
package ::ofml::xoi;
import ::ofml::oi::*;
import ::egr::found::*;

import ::stdio::*;





















public class xOiFuncs
{
    var mUpperStrings = Hash();

    static var sProgressDialog = NULL;
    static var sConfigTraversalContext = NULL;

    static var sInvArticles4CommonPropsMsgPrinted = 0; 

    static var sCHFProp2CHPropCache; 

    static var sCommonPropPrefix = "__COMMON_PROP_";
    static var sCommonPValPrefix = "__COMMON_PVAL";

    static var sHasOfmlAppFuncGetDataSearchPath = -1;
    static var sHasOfmlAppFuncGetPrimaryCurrency = -1;
    static var sHasOfmlAppFuncGetSecondaryCurrency = -1;
    static var sHasOfmlAppFuncGetManufacturerID = -1;
    static var sHasOfmlAppFuncGetManufacturerName = -1;
    static var sHasOfmlAppFuncGetSeriesID = -1;
    static var sHasOfmlAppFuncGetProgID = -1;
    static var sHasOfmlAppFuncGetProgInfoType = -1;
    static var sHasOfmlAppFuncGetPDBType = -1;
    static var sHasOfmlAppFuncGetProductDBPath = -1;
    static var sHasOfmlAppFuncGetOAMPath = -1;
    static var sHasOfmlAppFuncGetLibPath = -1;
    static var sHasOfmlAppFuncRegisterPackage = -1;
    static var sHasOfmlAppFuncGetProgInfoDBPath = -1;
    static var sHasOfmlAppFuncGetConfigProfileTableID = -1;
    static var sHasOfmlAppFuncGetUpdateMode = -1;
    static var sHasOfmlAppFuncGetMetaType = -1;
    static var sHasOfmlAppFuncGetMetaSeries = -1;
    static var sHasOfmlAppFuncIsPropTypeSupported = -1;
    static var sHasOfmlAppFuncHandleNotUpdatedArticles = -1;
    static var sHasOfmlAppFuncCreateArticle = -1;
    static var sHasOfmlAppFuncCopyObject = -1;
    static var sHasOfmlAppFuncGetRegistryKey = -1;

    static var sHasEgrAppFuncNeedUserDefinedOrderData = -1;
    static var sHasEgrAppFuncGetRegistryKey = -1;
    static var sHasEgrAppFuncGetEPLPath = -1;
    static var sHasEgrAppFuncGetDebugInfo = -1;
    static var sHasEgrAppFuncGetDebugClasses = -1;
    static var sHasEgrAppFuncGetProductDBPath = -1;
    static var sHasEgrAppFuncGetLibPath = -1;
    static var sHasEgrAppFuncApplyPriceProfiles = -1;
    static var sHasEgrAppFuncApplyPriceProfiles2 = -1;
    static var sHasEgrAppFuncProgressEvent = -1;
    static var sHasEgrAppFuncShowHint = -1;
    static var sHasEgrAppFuncIsLicensedProg = -1;
    static var sHasEgrAppFuncConvertCurrency = -1;

    static var sHasEgrAppBasketFuncAddItem = -1;
    static var sHasEgrAppBasketFuncRemoveItem = -1;
    static var sHasEgrAppBasketFuncChangedItem = -1;

    static var sHasEgrAppObkFuncLockObjects = -1;
    static var sHasEgrAppObkFuncUnlockAllObjects = -1;
    static var sHasEgrAppObkFuncGetArticleData = -1;
    static var sHasEgrAppObkFuncDeleteBasketItems = -1;
    static var sHasEgrAppObkFuncCreateObject = -1;
    static var sHasEgrAppObkFuncSetDefaultPriceDate = -1;
    static var sHasEgrAppObkFuncGetDefaultPriceDate = -1;

    
    static var sAppLibPath = Hash();
    static var sAppRegionVersionPath = Hash(); 
    static var sAppIndexpricesPath = Hash(); 
    static var sAppOAMPath = Hash();
    static var sAppProductDBPath = Hash();
    static var sAppProgInfoDBPath = Hash();
    static var sApppMetaSeries = Hash();
    static var sAppPDBType = Hash();
    static var sAppIsPropTypeSupported = Hash();
    static var sLicensedPrograms = Hash();

    static var sDecomposedChoiceLists;

    
    static var sAddAttPtDefsTblName = "addattptdefs";
    static var sAddAttPtDefsTblStruct = 
	@(["article",    0, 0+0, @s, ""+"", "0"],
	  ["number",     0, 0+0, @i, 0+0,   "0"],
	  ["attpt_key",  0, 0+0, @s, ""+"", "0"],
	  ["condition",  0, 0+0, @s, ""+"", "0"],
	  ["attpt_pos",  0, 0+0, @s, ""+"", "0"],
	  ["attpt_yrot", 0, 0+0, @s, ""+"", "0"],
	  ["attpt_dir",  0, 0+0, @s, ""+"", "0"],
	  ["attpt_mode", 0, 0+0, @s, ""+"", "0"]);

    static var sHasAddAttPtDefsTbl = Hash(); 
    static var sAddAttPtDefsCache = Hash(); 

    static var sOppositeAttPtsTblName = "oppositeattpts";
    static var sOppositeAttPtsTblStruct = 
	@(["attpt_key",  0, 0+0, @s, ""+"", "0"],
	  ["direction",  0, 0+0, @s, ""+"", "0"],
	  ["opposites",  0, 0+0, @s, ""+"", "0"]);

    static var sHasOppositeAttPtsTbl = Hash(); 
    static var sOppositeAttPtsCache = Hash(); 

    
    static var sControlDataTblStruct = 
	@(["type",  0, 0+0, @s, ""+"", "0"],
	  ["args",  0, 0+0, @s, ""+"", "0"],
	  ["value", 0, 0+0, @s, ""+"", "0"]);

    
    static var sDataTables = Hash();

    static var sOfmlKeywords = @("abstract", "break", "case", "catch", "class", 
				 "continue", "default", "do", "else", "final", 
				 "finally", "for", "foreach", "func", "goto", 
				 "if", "import", "instanceof", "native", 
				 "operator", "package", "private", "protected", 
				 "public", "return", "rule", "self", "static", 
				 "super", "switch", "throw", "transient", 
				 "try", "var", "while", "ODB_NAME");
}

var gxOiFuncs = xOiFuncs();








func xOiAskYesNo(pMsg, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiAskYesNo", pMsg);

    var tPlanning = xOiBasePlanning.getPlanning();

    var tRet;
    var tMsg = pMsg[0];
    var tType = @QUESTION;
    if (pMsg.size() > 1)
	tType = pMsg[1];
    var tLanguage = tPlanning.getLanguage();
    if (pMsg.size() > 2)
	tLanguage = pMsg[2];
    
    if (!xOiBasePlanning.sUseGUIDialogs) {
	xOiDebugPrint(@Info, 1, "using standard file I/O");
	tMsg = oiGetStringResource(tMsg, tLanguage);
	tMsg = xOiConcatStrings(xOiString2List(tMsg, 1));
	stdout.printf("%s [y/n] ", tMsg);
	var tInpStr = "";
	stdin.gets(tInpStr);
	tRet = (tInpStr.empty() || tInpStr[0] != 'n');
    }
    else {
	var tRes = oiDialog(@YES_NO, tType, tMsg);
	xOiDebugPrint(@Info, 2, ["oiDialog result: ", tRes]); 
	tRet = (tRes == @YES);
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}










func xOiFooBarDummy()
{
}






public func xOiGetClasses(pObj)
{
    xOiDebugStartFunc2("xOiFuncs::xOiGetClasses", pObj);

    var tCL = @();

    if (pObj.hasMember(@isCat) && pObj.isCat(@IF_RecoveredObject)) {
        xOiDebugPrint(@Info, 2, "recovered object!");
	tCL = pObj.getOriginalClasses();
	xOiDebugFinishFunc(tCL);
	return(tCL);
    }

    var tType = typeOf(pObj);
    while (tType != NULL) {
	tCL.pushBack(tType.getFullName());
	tType = tType.getSuperClass();
	
	if (tType != NULL && tType.getName() == @Measurement) tType = NULL;
    }

    xOiDebugFinishFunc(tCL);
    return(tCL);
}



public func xOiGetOdbPlElementUp(pObj)
{
    var tF, tObj = pObj;
    while ((tF = tObj.getFather()) != NULL) {
	tObj = tF;
	if (tObj.isA(OiOdbPlElement)) return(tObj);
    }
    return(NULL);
}





public func xOiIsAComplex(pObj)
{       
    return(pObj.isA(OiPlanning) || pObj.isA(OiPlElement) || pObj.isA(OiPart) ||
	   pObj.isA(OiLevel) || pObj.isA(OiWall));
}










public func xOiIsABasicGeometry(pArg)
{
    xOiDebugStartFunc("xOiFuncs::xOiIsABasicGeometry", pArg);

    var tType = typeOf(pArg);
    var tIsObj = (tType.subClassOf(::egr::afml::Primitive));
    var tIsType = (tType.getFullName() == "::cobra::lang::ClassType");
    if (!tIsObj && !tIsType)  {
	xOiDebugPrint(@ExplWarn, 1, 
		      "wrong argument for xOiIsABasicGeometry()!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }
    if (tIsType) tType = pArg;

    var tTName = tType.getFullName();
    var tClass = String(tType.getName());
    var tClPos = tTName.find(tClass, 0);
    var tPckg  = tTName.substr(0, tClPos-2);

    var tRet = (tPckg == "::egr::afml" || 
		(tPckg == "::ofml::oi" && 
		 tType.subClassOf(::ofml::oi::OiGeometry)));
    xOiDebugFinishFunc(tRet);
    return(tRet);
}











public func xOiCompareObjs(pObj1, pObj2)
{
    var tType1 = typeOf(pObj1);
    var tType2 = typeOf(pObj2);

    if (tType1 == tType2) {
	var tEqual = 0;
	if (tType1 == Vector || tType1 == List) {
	    
	    try {
	    	tEqual = (pObj1 == pObj2);
	    }
	}
	else {
	    try {
	        tEqual = (pObj1 == pObj2);
	    }
	    if (!tEqual && tType1 == Float)
		
		tEqual = (fabs(pObj1-pObj2) < 1e-6);
	}
	return(tEqual);
    }

    var tInt, tFloat;
    if (tType1 == Int && tType2 == Float) {
	tInt = pObj1;
	tFloat = pObj2;
    }
    else
    if (tType1 == Float && tType2 == Int) {
	tInt = pObj2;
	tFloat = pObj1;
    }
    else 
	return(0);

    var tFloatInt = Int(tFloat);
    var tFloatFrac = fabs(tFloat - tFloatInt);

    if (tFloatFrac > 0.0001) return(0);

    return(tInt == tFloatInt);
}









public func xOiGetCatalogID(pObj)
{
    var tCatInfo = NULL;

    if (!pObj.isCat(@IF_Article) || (tCatInfo = pObj.getCatalogInfo()) == NULL)
	return(NULL);

    var tCatInfoItem;
    var tCatID = NULL;
    foreach (tCatInfoItem; tCatInfo)
	if (tCatInfoItem[0] == @CatID) {
	    tCatID = tCatInfoItem[1];
	    break;
	}

    return(tCatID);
}







public func xOiGetPackage(pObj)
{
    var tType = typeOf(pObj);
    var tIsObj = (tType.subClassOf(::egr::afml::Primitive));
    var tIsType = (tType.getFullName() == "::cobra::lang::ClassType");
    if (!tIsObj && !tIsType)  {
	xOiDebugPrint(@ExplWarn, 1, "wrong argument for xOiGetPackage()!");
	return(NULL);
    }
    if (tIsType) tType = pObj;
    var tSpec  = tType.getFullName();
    var tClass = String(tType.getName());

    var tPos   = tSpec.find(tClass, 0);

    return(tSpec.substr(0, tPos-2));
}










public func xOiPackage2Path(pPackage, ...)
{
    var tPackage = pPackage[0];
    var tPathPrefix = "data/";
    if (pPackage.size() > 1)
	tPathPrefix = pPackage[1];

    var tPath = String(tPackage);

    tPath.replace(0, 2, tPathPrefix);

    var tPos = 0;
    while ((tPos = tPath.find("::", tPos)) > -1) tPath.replace(tPos, 2, "/");

    return(tPath);
}







public func xOiPackage2ProgID(pPackage)
{
    if (!(pPackage instanceof String)) return(NULL);

    var tPos1 = pPackage.find("::", 2);
    if (tPos1 < 0) return(NULL);

    var tMan = pPackage.substr(2, tPos1-2);

    var tPos2 = pPackage.find("::", tPos1+2);
    if (tPos2 < 0) tPos2 = pPackage.size();

    var tSeries = pPackage.substr(tPos1+2, tPos2-tPos1-2);

    return(Symbol(tMan+"_"+tSeries));
}









public func xOiProgID2Package(pPID)
{
    if (pPID == NULL || !(pPID instanceof Symbol)) return("");

    var tPID = String(pPID);
    var tUS = tPID.find('_');

    if (tUS < 0) return("");

    return("::"+tPID.substr(0, tUS)+"::"+tPID.substr(tUS+1)+"::");
}












public func xOiProgID2Path(pPID)
{
    if (pPID == NULL || !(pPID instanceof Symbol)) return("");

    var tPath = "data";
    var tPID  = String(pPID);
    var tLen  = tPID.size();
    var tPos1 = 0, tUS;
    do {
	tUS = tPID.find('_', tPos1);
	if (tUS < 0) tUS = tLen;
	tPath += "/" + tPID.substr(tPos1, tUS-tPos1);
	tPos1 = tUS+1;
    } while (tUS < tLen);

    return(tPath);
}










public func xOiProgID2Manufacturer(pPID)
{
    if (pPID == NULL || !(pPID instanceof Symbol)) return("");

    var tPID = String(pPID);
    var tUS = tPID.find('_');

    if (tUS < 0) return(tPID);

    return(tPID.substr(0, tUS));
}










public func xOiProgID2Series(pPID)
{
    if (pPID == NULL || !(pPID instanceof Symbol)) return("");

    var tPID = String(pPID);
    var tUS = tPID.find('_');

    if (tUS < 0) return(tPID);

    return(tPID.substr(tUS+1));
}






public func xOiIsManufacturerProg(pPID, pManufacturers)
{
    xOiDebugStartFunc("xOiFuncs::xOiIsManufacturerProg", 
		      [pPID, pManufacturers]);

    var tRet = 0;

    if (!(pPID instanceof Symbol) ||
	!(pManufacturers instanceof List || pManufacturers instanceof Vector)) {
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tProgMan = xOiProgID2Manufacturer(pPID);
    if (tProgMan.empty()) {
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tM;
    foreach(tM; pManufacturers) {
	if (!(tM instanceof String || tM instanceof Symbol)) continue;
	var tMan = tM;
	if (tM instanceof Symbol)
	    tMan = String(tM);
	if (tMan == tProgMan) {
	    tRet = 1;
	    break;
	}
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}















public func xOiMapFloat2Str(pV, pN)
{       
    return(String(Int(pV*pow(10, pN)+0.5)));
}   












public func xOiFloat2Str(pVal, pDigits, ...)
{
    var tDigits = pDigits[0];

    var tFill = NULL; 
    if (pDigits.size() > 1)
	tFill = pDigits[1];

    var tS   = String(pVal);
    var tDot = tS.find(".", 0);

    if (tDot >= 0) {
	var tMustLen = tDot + tDigits + 1;
	tS = tS.substr(0, tMustLen);
	if (tFill != NULL && tFill && tS.size() < tMustLen)
	    tS.resize(tMustLen, '0');
    }
    else { 
	if (tFill == NULL || tFill)
	    tS += "." + String().resize(tDigits, '0');
	else
	    tS += ".0";
    }

    return(tS);
}





































public func xOiParam2Str(pParam, ...)
{
    var tParam = pParam[0];
    var tFullClassInfo = 0;
    if (pParam.size() > 1)
	tFullClassInfo = pParam[1]; 
    var t4Eval = 0;
    if (pParam.size() > 2)
	t4Eval = pParam[2];

    if (tParam == NULL) return("NULL");
    if (tParam instanceof String) {
	var tStr = tParam;
	if (t4Eval) {
	    tStr = xOiSubstString(tStr, "\\", "\\\\");
	    tStr = xOiSubstString(tStr, "\"", "\\\"");
	}
	return("\""+tStr+"\"");
    }

    var tRes = "";
    var tType = typeOf(tParam);

    var tIsSimple = 0;
    if (tParam instanceof Int || tParam instanceof Float ||
	tParam instanceof Symbol) tIsSimple = 1;
    var tIsVector = 0;
    if (tParam instanceof Vector) tIsVector = 1;
    var tIsList = 0;
    if (tParam instanceof List) tIsList = 1;
    var tIsHash = 0;
    if (tParam instanceof Hash) tIsHash = 1;
    var tIsObj = 0;
    if (tType.subClassOf(::egr::afml::Primitive)) tIsObj = 1;

    if (tIsVector || tIsList || tIsHash) {
        if (tIsVector) 
	    tRes = "[";
        else
        if (tIsList) 
	    tRes = "@(";
        else
        if (tIsHash) 
	    tRes = "Hash(\n";
	var tParam2 = tParam;
	if (tIsHash) {
	    tParam2 = @();
	    xOiCopyAggr(tParam.keys(), tParam2, 0);
	    try {
		tParam2.sort();
	    }
	    catch (&e: Error) {
		tParam2 = tParam.keys();
	    }
	}
        var tP;
        var t1st = 1;
        foreach(tP; tParam2) {
    	    if (t1st) 
		t1st = 0;
    	    else
		if (tIsHash)
		    tRes = tRes + "\n";
		else
		    tRes = tRes + ", ";
	    if (tIsHash)
    	    	tRes = tRes + "* " + 
		       xOiParam2Str(tP, tFullClassInfo, t4Eval) + "=" + 
		       xOiParam2Str(tParam[tP], tFullClassInfo, t4Eval);
	    else
    	    	tRes = tRes + xOiParam2Str(tP, tFullClassInfo, t4Eval);
        }
        if (tIsVector)
	    tRes = tRes + "]";
        else
	    tRes = tRes + ")";
    }
    else
    if (tIsObj) {
	var tName = NULL;
	try { tName = tParam.getName(); }
	if (tName != NULL) {
	    var tClassInfo = tParam.getClass();
	    if (!t4Eval && tFullClassInfo) {
		var tCL = xOiGetClasses(tParam);
		tClassInfo = "";
		var tClass;
		foreach(tClass; tCL) {
		    if (!tClassInfo.empty()) tClassInfo += ",";
		    tClassInfo += tClass;
		    if (tClass.size() > 11 &&
			(tClass.substr(0, 11) == "::ofml::oi:" ||
			 tClass.substr(0, 11) == "::egr::afml")) break;
		}
	    }
	    if (t4Eval)
		tRes = tName;
	    else
	        tRes = "object: " + tName + " (" + tClassInfo + ")";
	}
	else {
	    if (t4Eval)
		tRes = "NULL";
	    else
	        tRes = "object: <invalid reference!>";
	}
    }
    else
    if (tIsSimple) {
        tRes = String(tParam);
        if (tParam instanceof Symbol) {
	    if (t4Eval) {
		tRes = xOiSubstString(tRes, "\\", "\\\\");
		tRes = xOiSubstString(tRes, "\"", "\\\"");
		tRes = "Symbol(\"" + tRes+ "\")";
	    }
	    else
	        tRes = "@" + tRes;
	}
	else
	if (tParam instanceof Float && tRes.find('.') < 0 &&
	    tRes.find('e') < 0 && tRes.find('E') < 0)
	    tRes = tRes + ".0";
    }
    else {
	var tTypeName = tType.getFullName();
	if (tTypeName == "::cobra::lang::ClassType") {
	    if (t4Eval) tRes += "class ";
	    tRes += tParam.getFullName();
	}
	else {
	    
	    if (t4Eval)
		tRes = "NULL";
	    else
	        tRes = "<" + tTypeName +">";
	}
    }

    return(tRes);
}








































public func xOiPatternMatch(pSubject, pPattern, ...)
{
    var tPattern = pPattern[0];
    var tMode = 0;
    if (pPattern.size() > 1)
	tMode = pPattern[1];

    xOiDebugStartFunc("xOiFuncs::xOiPatternMatch", [pSubject, tPattern, tMode]);

    var tRet = 0;

    if (!(pSubject instanceof String && 
	  tPattern instanceof String &&
	  tMode instanceof Int && tMode >= 0 && tMode <= 7))
	xOiDebugPrint(@Warn, 1, "invalid parameter(s)!");
    else
	tRet = tPattern.fnmatch(pSubject, tMode);

    xOiDebugFinishFunc(tRet);
    return(tRet);
}




public func xOiIsOfmlKeyword(pStr)
{
    return(xOiFuncs.sOfmlKeywords.find(pStr) >= 0);
}




public func xOiIsalpha(pChar)
{
    var c = pChar;
    return (c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_');
}




public func xOiIsnum(pChar)
{
    var c = pChar;
    return(c >= '0' && c <= '9');
}




public func xOiIsalnum(pChar)
{
    var c = pChar;
    return (xOiIsalpha(c) || xOiIsnum(c));
}





public func xOiIsValidDate(pDate)
{
    xOiDebugStartFunc("xOiFuncs::xOiIsValidDate", pDate);

    var tRet = oiIsValidDate(pDate);

    xOiDebugFinishFunc(tRet);
    return(tRet);
}






public func xOiToupper(pVal)
{
    var tRes = NULL;

    if (pVal instanceof String)
	tRes = pVal.toUpper();
    else
    if (pVal instanceof Int) {
	tRes = pVal;
	if (pVal >= 'a' && pVal <= 'z') tRes = pVal - 'a' + 'A';
    }
    return(tRes);
}






public func xOiTolower(pVal)
{
    var tRes = NULL;

    if (pVal instanceof String) {
	tRes = "";
	var tLen = pVal.size();
	var i = 0;
	for (i; i<tLen; i++) tRes.append(1, xOiTolower(pVal[i]));
    }
    else
    if (pVal instanceof Int) {
	tRes = pVal;
	if (pVal >= 'A' && pVal <= 'Z') tRes = pVal + 'a' - 'A';
    }
    return(tRes);
}











public func xOiStripPadChar(pStr, pPadChar, ...)
{
    var tPads = pPadChar[0];
    if (tPads instanceof Int)
	tPads = @(pPadChar[0]);
    var tBothSides = 0;
    if (pPadChar.size() > 1)
	tBothSides = pPadChar[1];

    var tL = pStr.size();
    var tPos = tL; 
    var i;
    for(i=tPos-1; i >= 0; i--)
	if (tPads.find(pStr[i]) >= 0)
	    tPos = i;
	else
	   break;

    var tStr = pStr.substr(0, tPos);

    if (!tBothSides || tStr.empty()) return(tStr);

    for(i=0; i < tStr.size(); i++)
	if (tPads.find(tStr[i]) < 0) {
	    tPos = i;
	    break;
	}
    return(tStr.substr(tPos));
}







public func xOiSubstString(pStr, pWhat, pBy)
{
    var tStr = String(pStr);
    var tPos = 0;
    var tWhatLen = pWhat.size();
    var tByLen   = pBy.size();

    if (tWhatLen == 0 || tStr.size() == 0)
	return(tStr);

    while ((tPos = tStr.find(pWhat, tPos)) >= 0) {
	tStr.replace(tPos, tWhatLen, pBy);
	tPos += tByLen;
    }

    return(tStr);
}









public func xOiString2List(pStr, ...)
{
    var tStr = pStr[0];
    var tQuoted = 0;
    if (pStr.size() > 1)
	tQuoted = pStr[1];

    if (tStr == NULL || !(tStr instanceof String)) return(NULL);

    var tRes = @();

    if (tStr.empty()) return(tRes);

    var tSearchStr = "\n";
    if (tQuoted)
	tSearchStr = "\\n";
    var tSkipChars = tSearchStr.size();
    var tLen = tStr.size();
    var tPos1 = 0;
    var tPos2;
    while ((tPos2 = tStr.find(tSearchStr, tPos1)) >= 0) {
	tRes.pushBack(tStr.substr(tPos1, tPos2-tPos1));
	tPos1 = tPos2 + tSkipChars;
    }
    tRes.pushBack(tStr.substr(tPos1, tLen-tPos1));

    return(tRes);
}








public func xOiConcatStrings(pStrings, ...)
{
    if (pStrings == NULL || 
	!(pStrings instanceof List || pStrings instanceof Vector)) return("");

    var tStrings = pStrings[0];
    var tSeparator = "\n";
    if (pStrings.size() > 1)
	tSeparator = pStrings[1];

    var tRes = "";
    var t1st = 1;
    var tStr;
    foreach(tStr; tStrings) {
	if (t1st) t1st = 0;
	else      tRes.append(tSeparator);
	tRes.append(tStr);
    }
    return(tRes);
}


















public func xOiParseString2List(pStr, pSeparator, ...)
{
    if (pStr == NULL || !(pStr instanceof String)) return(NULL);

    var tSeparator = pSeparator[0];

    if (!(tSeparator instanceof Int || tSeparator instanceof String ||
	  tSeparator instanceof Vector)) 
	return(NULL);
    if (tSeparator instanceof Vector && 
	(tSeparator.size() != 2 || 
	 !(tSeparator[0] instanceof Int || tSeparator[0] instanceof String) || 
	 !(tSeparator[1] instanceof Int || tSeparator[1] instanceof String)))
	return(NULL);

    var tStripPads = 0;
    if (pSeparator.size() > 1)
	tStripPads = pSeparator[1];

    var tRes = @();

    var tSepPos, tPos1 = 0, tPos2;

    if (!(tSeparator instanceof Vector)) {
	var tLen = 1;
	if (tSeparator instanceof String)
	    tLen = tSeparator.size();
	do {
	    tSepPos = pStr.find(tSeparator, tPos1);
	    tPos2 = tSepPos;
	    if (tSepPos < 0) tPos2 = pStr.size();
	    tRes.pushBack(pStr.substr(tPos1, tPos2-tPos1));
	    tPos1 = tPos2+tLen;
	} while (tSepPos >= 0 && tPos1 < pStr.size());
    }
    else {
	var tSep1 = tSeparator[0];
	var tLen1 = 1;
	if (tSep1 instanceof String)
	    tLen1 = tSep1.size();
	var tSep2 = tSeparator[1];
	var tLen2 = 1;
	if (tSep2 instanceof String)
	    tLen2 = tSep2.size();
	while (1) {
	    tPos1 = pStr.find(tSep1, tPos1);
	    if (tPos1 < 0) break;
	    tPos2 = pStr.find(tSep2, tPos1+tLen1);
	    if (tPos2 < 0) break;
	    tRes.pushBack(pStr.substr(tPos1+tLen1, tPos2-tPos1-tLen1));
	    tPos1 = tPos2+tLen2;
	}
    }

    if (!tStripPads) return(tRes);

    var tRes2 = @();
    var tStr;
    foreach(tStr; tRes) tRes2.pushBack(xOiStripPadChar(tStr, ' ', 1));

    return(tRes2);
}







public func xOiStripPClassesFromVarCode(pVarCode)
{
    xOiDebugStartFunc("xOiFuncs::xOiStripPClassesFromVarCode", pVarCode);

    var tProps = xOiParseString2List(pVarCode, ';'); 

    if (tProps == NULL || tProps.empty()) {
	xOiDebugPrint(@Warn, 1, "varcode parsing error!"); 
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tStripped = "";
    var tFirst = 1;

    var tProp;
    foreach(tProp; tProps) {
	var tPCV  = xOiParseString2List(tProp, '=');
	if (tPCV == NULL || tPCV.size() != 2) {
	    xOiDebugPrint(@Warn, 1, "varcode parsing error!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}
	var tPC = xOiParseString2List(tPCV[0], '.');
	if (tPC == NULL || tPC.size() != 2) {
	    xOiDebugPrint(@Warn, 1, "varcode parsing error!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}
	if (tFirst)
	    tFirst = 0;
	else
	    tStripped += ";";
	tStripped += tPC[1]+"="+tPCV[1];
    }

    xOiDebugFinishFunc(tStripped);
    return(tStripped);
}





public func xOiGetValuesFromVarcode(pVarCode)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetValuesFromVarcode", pVarCode);

    var tRes = @();
    var tPos = 0;

    do {
	var tAssign = pVarCode.find('=', tPos);
	var tSemicolon = pVarCode.find(';', tPos);
	var tEnd = tSemicolon;
	if (tSemicolon == -1) tEnd = pVarCode.size();
	if (tAssign != -1) {
	    var tProp = pVarCode.substr(tPos, tAssign-tPos);
	    var tPVal = pVarCode.substr(tAssign+1, tEnd-tAssign-1);
	    tRes.pushBack(["", tProp, tPVal]);
	}
	else
		xOiDebugPrint(@Warn, 1, "invalid VarCode");
	tPos = tSemicolon + 1;
    } while(tPos);

    xOiDebugFinishFunc(tRes);
    return(tRes);
}





public func xOiChar2HexStr(pChar)
{
    var tChar;
    var tRes = "";

    tChar = (pChar >> 4) & 0x0f;
    tChar += tChar < 10 ? '0' : ('A' - 10);
    tRes.append(1, tChar);
    tChar = pChar & 0x0f;
    tChar += tChar < 10 ? '0' : ('A' - 10);
    tRes.append(1, tChar);

    return(tRes);
}

private func _xOiHexDigit2Decimal(pChar)
{
    var tAlphaUpperChars = @('A', 'B', 'C', 'D', 'E', 'F');
    var tAlphaLowerChars = @('a', 'b', 'c', 'd', 'e', 'f');
    var tDigitChars = @('0', '1', '2', '3', '4', '5', '6', '7', '8', '9');

    if (tAlphaUpperChars.find(pChar) >= 0)
	return(10 + (pChar - 'A'));
    if (tAlphaLowerChars.find(pChar) >= 0)
	return(10 + (pChar - 'a'));
    if (tDigitChars.find(pChar) >= 0)
    	return(0 + (pChar - '0'));
    return(NULL);
}






public func xOiHexStr2Char(pHexStr)
{
    if (pHexStr == NULL || !(pHexStr instanceof String) || pHexStr.size() != 2)
	return(0);
    var tD0 = _xOiHexDigit2Decimal(pHexStr[1]);
    var tD1 = _xOiHexDigit2Decimal(pHexStr[0]);
    if (tD0 == NULL || tD1 == NULL)
	return(0);
    return(tD1*16 + tD0);
}









public func xOiSpecialChars2Hex(pInStr, ...)
{
    var tInStr = pInStr[0];
    var tSpecials = NULL;
    if (pInStr.size() > 1)
	tSpecials = pInStr[1];

    var tOutStr = "";
    var i;
    for (i = 0; i < tInStr.size(); i++) {
	var tCh = tInStr[i];
	if ((tSpecials instanceof List && tSpecials.find(tCh) >= 0) ||
	    !xOiIsalnum(tCh)) {
	    tOutStr += "_" + xOiChar2HexStr(tCh);
	}
	else {
	    tOutStr.append(1, tCh);
	    if (tCh == '_') tOutStr.append(1, '_');
	}
    }
    return(tOutStr);
}









func xOiHex2SpecialChars(pHexStr)
{
    

    var tOutStr = "";
    var i, tLen = pHexStr.size();
    for (i = 0; i < tLen; i++) {
	var tCh  = pHexStr[i];
	var tStr = pHexStr.substr(i, 1);

	if (tCh == '_' && i < tLen-1) {
	    var tCh1 = pHexStr[++i];
	    if (tCh1 != '_') {
		if (i < tLen-1) {
		    var tHexStr = pHexStr.substr(i, 2);
		    tCh = xOiHexStr2Char(tHexStr);
		    i++;
		    if (tCh)
			tStr[0] = tCh;
		    else
			tStr.append(tHexStr);
		}
		else
		    tStr.append(1, tCh1);
	    }
	}
	tOutStr.append(tStr);
    }
    
    return(tOutStr);
}








public func xOiString2Symbol(pInStr)
{
    return(_xOiString2Symbol(pInStr, "S"));
}



func _xOiString2Symbol(pInStr, pAddPrefix)
{
    xOiDebugStartFunc3("xOiFuncs::_xOiString2Symbol", [pInStr, pAddPrefix]);

    if (pInStr == NULL || !(pInStr instanceof String)) {
	xOiDebugPrint(@Warn, 1, "no String given!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tOutStr = "";
    var tPrefix = "S";
    var tAddPrefix = 0;
    if (pAddPrefix != NULL && pAddPrefix instanceof String) {
	tPrefix = pAddPrefix;
	tAddPrefix = 1;
    }
    if (tAddPrefix || xOiIsnum(pInStr[0])) tOutStr = tPrefix;

    var tRet = Symbol(tOutStr+xOiSpecialChars2Hex(pInStr));

    xOiDebugFinishFunc(tRet);
    return(tRet);
}








public func xOiSymbol2String(pSym)
{
    return(_xOiSymbol2String(pSym, "S"));
}



func _xOiSymbol2String(pSym, pStripPrefix)
{
    if (pSym == NULL || !(pSym instanceof Symbol)) return(NULL);

    var tInStr = String(pSym);

    var tPrefix = "S";
    var tStripPrefix = 0;
    if (pStripPrefix != NULL && pStripPrefix instanceof String) {
	tPrefix = pStripPrefix;
	tStripPrefix = 1;
    }
    var tPrefixLen = tPrefix.size();

    if (tStripPrefix ||
	(tInStr.size() > tPrefixLen+1 && 
	 tInStr.substr(0, tPrefixLen) == tPrefix &&
	 xOiIsnum(tInStr[tPrefixLen])))
	tInStr = tInStr.substr(tPrefixLen);

    return(xOiHex2SpecialChars(tInStr));
}





public func xOiEscapeQuotes(pString)
{
    if (pString == NULL || !(pString instanceof String)) return(NULL);

    var tOutStr = "";
    var i, tLen = pString.size();

    for (i = 0; i < tLen; i++) {
	var tCh = pString[i];
	if (tCh == '\"') 	tOutStr += "\\\"";
	else
			   	tOutStr.append(1, tCh);
    }
    return(tOutStr);
}







public func xOiEscapeCharacters(pString, pChars)
{
    if (!(pString instanceof String)) return(NULL);
    if (!(pChars  instanceof List))   return(String(pString));

    var tOutStr = "";
    var i;

    for (i = 0; i < pString.size(); i++) {
	var tCh = pString[i];
	if (pChars.find(tCh) >= 0) 
	    tOutStr.append(1, '\\');

	tOutStr.append(1, tCh);
    }
    return(tOutStr);
}







public func xOiConvertUmlauts(pString)
{
    if (pString == NULL || !(pString instanceof String)) return(NULL);

    

    var tOutStr = "";
    var i;

    for (i = 0; i < pString.size(); i++) {
	var tCh = pString[i];
	if (tCh == 'ä') 	tOutStr += "ae";
	else
	if (tCh == 'Ä') 	tOutStr += "Ae";
	else
	if (tCh == 'Ü') 	tOutStr += "Ue";
	else
	if (tCh == 'ü') 	tOutStr += "ue";
	else
	if (tCh == 'ö') 	tOutStr += "oe";
	else
	if (tCh == 'Ö') 	tOutStr += "Oe";
	else
	if (tCh == 'ß') 	tOutStr += "ss";
	else
	if (tCh == 'á' || tCh == 'à' || tCh == 'â') 
				tOutStr += "a";
	else
	if (tCh == 'Á' || tCh == 'À' || tCh == 'Â') 
				tOutStr += "A";
	else
	if (tCh == 'é' || tCh == 'è' || tCh == 'ê' || tCh == 'ë') 
				tOutStr += "e";
	else
	if (tCh == 'É' || tCh == 'È' || tCh == 'Ê' || tCh == 'Ë') 
				tOutStr += "E";
	else
	if (tCh == 'í' || tCh == 'ì' || tCh == 'î' || tCh == 'ï') 
				tOutStr += "i";
	else
	if (tCh == 'Í' || tCh == 'Ì' || tCh == 'Î' || tCh == 'Ï') 
				tOutStr += "I";
	else
	if (tCh == 'ó' || tCh == 'ò' || tCh == 'ô') 
				tOutStr += "o";
	else
	if (tCh == 'Ó' || tCh == 'Ò' || tCh == 'Ô') 
				tOutStr += "O";
	else
	if (tCh == 'ú' || tCh == 'ù' || tCh == 'û') 
				tOutStr += "u";
	else
	if (tCh == 'Ú' || tCh == 'Ù' || tCh == 'Û') 
				tOutStr += "U";
	else
	if (tCh == 'ç') 	tOutStr += "c";
	else
	if (tCh == 'ñ') 	tOutStr += "n";
	else
			   	tOutStr.append(1, tCh);
    }

    
    return(tOutStr);
}
















public func xOiTransformObjCoords(pObj, pCoords, pFather)
{
    var tObj = pObj;
    var tCoords = pCoords;

    while (tObj != NULL && tObj != pFather) {
	var tObjPos = tObj.getPosition();
	var tPos = xOiTransformXZCoords(tObjPos, tObj.getRotation(@PY),
				        tCoords[0], tCoords[2]);
	tCoords = [tPos[0], tObjPos[1]+tCoords[1], tPos[2]];
	tObj = tObj.getFather();
	if (tObj == NULL)
	    
	    tCoords = NULL;
    }
    return(tCoords);
}











public func xOiGetBoundsFor(pObjects, pExclObj)
{
    if (pObjects.empty()) return([[0, 0, 0], [0, 0, 0]]);

    
    var tExclObj = @();
    if (pExclObj != NULL &&
	((pExclObj instanceof List) || (pExclObj instanceof Vector)))
	tExclObj = pExclObj;

    var tObj, tBB, tP1, tP2;
    var tEps = 0.0001;
    var tFirstObj = NULL;

    
    foreach(tObj; pObjects) {
        if (tExclObj.find(tObj) >= 0 || tObj.isA(xOiBBox)) continue;
        tBB = tObj.getWorldBounds();
        tP1 = tBB[0];
        tP2 = tBB[1];
        if (fabs(tBB[1][0]) < tEps && fabs(tBB[1][1]) < tEps &&
    	    fabs(tBB[1][2]) < tEps) continue;
        tFirstObj = tObj;
        break;
    }

    if (tFirstObj == NULL) return([[0, 0, 0], [0, 0, 0]]);

    var minX = tP1[0];
    var minY = tP1[1]; 
    var minZ = tP1[2];
    var maxX = tP2[0];
    var maxY = tP2[1]; 
    var maxZ = tP2[2];

    foreach(tObj; pObjects) {
        if (tObj == tFirstObj || tExclObj.find(tObj) >= 0 || 
	    tObj.isA(xOiBBox)) continue;
        tBB = tObj.getWorldBounds();
        tP1 = tBB[0];
        tP2 = tBB[1];
        if (fabs(tBB[1][0]) < tEps && fabs(tBB[1][1]) < tEps &&
    	    fabs(tBB[1][2]) < tEps) continue;
        if (tP1[0] < minX) minX = tP1[0];
        if (tP1[1] < minY) minY = tP1[1];
        if (tP1[2] < minZ) minZ = tP1[2];
        if (tP2[0] > maxX) maxX = tP2[0]; 
        if (tP2[1] > maxY) maxY = tP2[1];
        if (tP2[2] > maxZ) maxZ = tP2[2];
    }
    return([[minX, minY, minZ], [maxX, maxY, maxZ]]);
}







public func xOiGetMaxCoord(pObjList, pAxis)
{
    var tS = pObjList.size();
    
    if (tS == 0) return(0);

    var tBBMax  = pObjList[0].getWorldBounds()[1];
    var tMax = tBBMax[pAxis];
    var i;
    for (i = 1; i < tS; i++) {
        tBBMax  = pObjList[i].getWorldBounds()[1];
        if (tBBMax[pAxis] > tMax) tMax = tBBMax[pAxis];
    }
    return(tMax);
}







public func xOiGetMinCoord(pObjList, pAxis)
{
    var tS = pObjList.size();
    
    if (tS == 0) return(0);

    var tBBMin  = pObjList[0].getWorldBounds()[0];
    var tMin = tBBMin[pAxis];
    var i, tC;
    for (i = 1; i < tS; i++) {
        tBBMin  = pObjList[i].getWorldBounds()[0];
        if (tBBMin[pAxis] < tMin) tMin = tBBMin[pAxis];
    }
    return(tMin);
}








public func xOiGetObjectsGE(pObjList, pAxis, pTresh)
{
    var tOL = @();
    var tLEl, tP, tBB;
    foreach (tLEl; pObjList) {
        tP = tLEl.getPosition()[pAxis];
        tBB = tLEl.getLocalBounds()[1];
        if (tP+tBB[pAxis] > pTresh-xOiEps) tOL.pushBack(tLEl);
    }
    return(tOL);
}







public func xOiGetObjectsLE(pObjList, pAxis, pTresh)
{
    var tOL = @();
    var tLEl;
    foreach (tLEl; pObjList)
        if (tLEl.getPosition()[pAxis] < pTresh+xOiEps) tOL.pushBack(tLEl);
    return(tOL);
}













public func xOiHash2List(pHash)
{
    if (pHash == NULL || !(pHash instanceof Hash)) return(NULL);

    var tList = @();
    var tK;
    foreach(tK; pHash.keys()) {
	var tKey = tK; 
	var tVal = pHash[tKey];
	if (tVal != NULL && tVal instanceof Hash)
	    tList.pushBack([tKey, xOiHash2List(tVal), @Hash]);
	else
	    tList.pushBack([tKey, tVal]);
    }
    return(tList);
}








public func xOiList2Hash(pList)
{
    if (pList == NULL || 
	(!(pList instanceof List) && !(pList instanceof Vector))) return(NULL);

    var tHash = Hash();

    var tEntry;
    foreach(tEntry; pList) {
	if (tEntry == NULL || !(tEntry instanceof Vector) || 
	    tEntry.size() > 3 || (tEntry.size() == 3 && tEntry[2] != @Hash))
	    return(NULL);
	if (tEntry.size() == 3 && tEntry[2] == @Hash)
	    tHash[tEntry[0]] = xOiList2Hash(tEntry[1]);
	else {
	    var tValue = tEntry[1];
	    if (tEntry[1] instanceof Vector) {
		
		tValue = Vector(0);
		xOiCopyAggr(tEntry[1], tValue, 1);
	    }
	    tHash[tEntry[0]] = tValue;
	}
    }

    return(tHash);
}













public func xOiList2HashX(pList)
{
    if (pList == NULL || 
	(!(pList instanceof List) && !(pList instanceof Vector))) return(NULL);

    var tHash = Hash();

    var tEntry;
    foreach(tEntry; pList) {
	if (tEntry == NULL || !(tEntry instanceof Vector) || tEntry.size() < 2)
	    return(NULL);
	var tKey = tEntry[0];
	var tVal = tEntry[1];
	if (tEntry.size() > 2) {
	    tVal = Vector(0);
	    xOiCopyAggr(tEntry, tVal, 1);
	    tVal.erase(0);
	}    
	tHash[tKey] = tVal;
    }

    return(tHash);
}







public func xOiGetListValueOrder(pList)
{
    if (pList == NULL || pList.empty()) return(NULL);

    var tLen = pList.size();
    var tRes = Vector(0);
    var tSVV = Vector(0);	
    var i;
    for(i = 0; i < tLen; i++) {
	var tV = pList[i];
	if (tV == NULL || 
	    !(tV instanceof Int || tV instanceof Float)) return(NULL);
	tRes.pushBack(i);
	tSVV.pushBack(tV);
    }

    for(i = 1; i < tLen; i++) {
	var j;
	for(j = 0; j < i; j++) {
	    var tE1 = tSVV[j];
	    var tE2 = tSVV[i];
	    if (tE1 > tE2) {
		
		var idx = tRes[i];
		tRes[i] = tRes[j];
		tRes[j] = idx;
		tSVV[j] = tE2;
		tSVV[i] = tE1;
	    }
	}
    }

    return(tRes);
}






public func xOiMinInList(pList)
{
    if (pList == NULL || pList.empty()) return(NULL);

    var tMin, tE;
    foreach(tE; pList) {
	if (tE == NULL) continue;
	if (!(tE instanceof Int) && !(tE instanceof Float)) return(NULL);
        if (tMin == NULL)
    	    tMin = tE;
        else
    	    if (tE < tMin) tMin = tE;
    }

    return(tMin);
}






public func xOiMinIdxInList(pList)
{
    if (pList == NULL || pList.empty()) return(NULL);

    var i = -1;
    var tMin, tMinIdx, tE;
    foreach(tE; pList) {
	i++;
	if (tE == NULL) continue;
	if (!(tE instanceof Int) && !(tE instanceof Float)) return(NULL);
        if (tMin == NULL) {
    	    tMin = tE;
	    tMinIdx = i;
	}
        else
    	    if (tE < tMin) {
		tMin = tE;
		tMinIdx = i;
	    }
    }

    return(tMinIdx);
}






public func xOiMaxInList(pList)
{
    if (pList == NULL || pList.empty()) return(NULL);

    var tMax, tE;
    foreach(tE; pList) {
	if (tE == NULL) continue;
	if (!(tE instanceof Int) && !(tE instanceof Float)) return(NULL);
        if (tMax == NULL)
    	    tMax = tE;
        else
    	    if (tE > tMax) tMax = tE;
    }

    return(tMax);
}






public func xOiMaxIdxInList(pList)
{
    if (pList == NULL || pList.empty()) return(NULL);

    var i = -1;
    var tMax, tMaxIdx, tE;
    foreach(tE; pList) {
	i++;
	if (tE == NULL) continue;
	if (!(tE instanceof Int) && !(tE instanceof Float)) return(NULL);
        if (tMax == NULL) {
    	    tMax = tE;
	    tMaxIdx = i;
	}
        else
    	    if (tE > tMax) {
		tMax = tE;
		tMaxIdx = i;
	    }
    }

    return(tMaxIdx);
}





















public func xOiCopyAggr(pFrom, pTo, pDeep)
{
    
    if (pFrom == NULL || 
	!(pFrom instanceof List || pFrom instanceof Vector || 
	  pFrom instanceof Hash))
	return;
    if (pTo != NULL &&
	!(pTo instanceof List || pTo instanceof Vector || pTo instanceof Hash))
	return;
    if ((pFrom instanceof Hash && pTo != NULL && !(pTo instanceof Hash)) ||
	(pTo != NULL && pTo instanceof Hash && !(pFrom instanceof Hash)))
	return;

    if (0 && pTo == NULL && !(pFrom instanceof Hash)) {
	
	
	
	if (pDeep)
	    return (pFrom.copy(99)); 
	return (pFrom.copy());
    }

    var tTo = pTo;
    if (pTo == NULL) {
	if (pFrom instanceof List)
	    tTo = @();
	else
	if (pFrom instanceof Vector)
	    tTo = Vector(0);
	else
	    tTo = Hash();
    }

    var tIsList, tIsVec, tIsHash;

    if (pFrom instanceof Hash)  {
	var tKey;
	foreach(tKey; pFrom.keys()) {
	    var tVal = pFrom[tKey];
	    if (pDeep && 
		((tIsList = (tVal instanceof List)) || 
		 (tIsVec = (tVal instanceof Vector)) || 
		 (tIsHash = (tVal instanceof Hash)))) {
		var tVal2 = @();
		if (!tIsList) {
		    if (tIsVec)
			tVal2 = Vector(0);
		    else
		    if (tIsHash)
			tVal2 = Hash();
		}
		xOiCopyAggr(tVal, tVal2, pDeep);
		tVal = tVal2;
	    }
	    tTo[tKey] = tVal;
	}
	return(pTo == NULL ? tTo : NULL);
    }

    var tE;
    foreach(tE; pFrom) {
        if (pDeep && 
	    ((tIsList = (tE instanceof List)) || 
	     (tIsVec = (tE instanceof Vector)) || 
	     (tIsHash = (tE instanceof Hash)))) {
    	    var tAggr = @();
	    if (!tIsList) {
		if (tIsVec)  
		    tAggr = Vector(0);
		else
		if (tIsHash)
		   tAggr = Hash();
	    }
    	    xOiCopyAggr(tE, tAggr, pDeep);
    	    tTo.pushBack(tAggr);
        }
        else {
	    
    	    tTo.pushBack(tE);
	}
    }
    return(pTo == NULL ? tTo : NULL);
}















public func xOiCompareAggr(pAggr1, pAggr2, pDeep)
{
    if (pAggr1 == NULL || pAggr2 == NULL) return(-1);
    if (!(pAggr1 instanceof List || pAggr1 instanceof Vector)) return(-1);
    if (!(pAggr2 instanceof List || pAggr2 instanceof Vector)) return(-1);

    var tSize1 = pAggr1.size();
    var tSize2 = pAggr2.size();
    if (tSize1 != tSize2) return(0);

    var i;
    for(i = 0; i < tSize1; i++) {
        var tE1 = pAggr1[i];
        var tE2 = pAggr2[i];
        var tIsAggr1 = (tE1 instanceof List || tE1 instanceof Vector);
        var tIsAggr2 = (tE2 instanceof List || tE2 instanceof Vector);

        if ((!tIsAggr1 && tIsAggr2) || (tIsAggr1 && !tIsAggr2)) return(0);

        if ((!tIsAggr1 && !tIsAggr2) ||
    	    (tIsAggr1 && tIsAggr2 && !pDeep)) {
    	    
	    if (xOiCompareObjs(tE1, tE2)) 
		continue;
    	    else
		return(0);
        }

        

        var tRes = xOiCompareAggr(tE1, tE2, pDeep);
        if (tRes == 1) continue;
        else	       return(0);
    }
    return(1);
}





public func xOiClearAggr(pAggr)
{
    if (pAggr == NULL) return;
    if (!(pAggr instanceof List || pAggr instanceof Vector)) return;

    while (pAggr) pAggr.popBack();
}
























public func xOiObjInList(pObj, pList)
{
    if (!(pList instanceof List || pList instanceof Vector)) return(0);

    var tLE;
    foreach(tLE; pList)
	if (xOiCompareObjs(tLE, pObj)) return(1);

    return(0);
}





public func xOiObjPosInList(pObj, pList)
{
    var tLE;
    var i = 0;
    foreach(tLE; pList)
        if (tLE == pObj)
    	    return(i);
        else
    	    i++;
    return(-1);
}





public func xOiGetObjectsByType(pList, pType, pObj)
{
    var tC;
    foreach(tC; pObj.getChildren())
        if (tC.isA(pType)) 
	    pList.pushBack(tC);
        else 
	    xOiGetObjectsByType(pList, pType, tC);
}
















public func xOiGetAppRegistryKey(pProgID, pKey, pLang, pParam)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetAppRegistryKey",
		      [pProgID, pKey, pLang, pParam]);

    if (xOiFuncs.sHasEgrAppFuncGetRegistryKey == 0 &&
	xOiFuncs.sHasOfmlAppFuncGetRegistryKey == 0) {
	xOiDebugPrint(@Warn, 1, "not supported by application");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    

    var tArgs = xOiParam2Str(pProgID) + "," + xOiParam2Str(pKey) + "," +
		xOiParam2Str(pLang) + "," + xOiParam2Str(pParam);

    xOiDebugPrint(@Info, 2, ["args: ", tArgs]);

    var tRes = NULL;

    if (xOiFuncs.sHasEgrAppFuncGetRegistryKey == 1) {
	tRes = ::egr::app::getRegistryKey(tArgs);
	if (tRes == "NULL" || tRes.empty()) tRes = NULL;
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }
    if (xOiFuncs.sHasOfmlAppFuncGetRegistryKey == 1) {
	tRes = ::ofml::app::getRegistryKey(tArgs);
	if (tRes == "NULL" || tRes.empty()) tRes = NULL;
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    xOiFuncs.sHasEgrAppFuncGetRegistryKey = 1;
    try {
	tRes = ::egr::app::getRegistryKey(tArgs);
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncGetRegistryKey = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::getRegistryKey(): " + e.what);
    }
    if (xOiFuncs.sHasEgrAppFuncGetRegistryKey == 1) {
        if (tRes != NULL && (tRes == "NULL" || tRes.empty())) tRes = NULL;
        xOiDebugFinishFunc(tRes);
        return(tRes);
    }

    xOiFuncs.sHasOfmlAppFuncGetRegistryKey = 1;
    try {
	tRes = ::ofml::app::getRegistryKey(tArgs);
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetRegistryKey = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getRegistryKey(): " + e.what);
    }
    if (tRes != NULL && (tRes == "NULL" || tRes.empty())) tRes = NULL;

    xOiDebugFinishFunc(tRes);
    return(tRes);
}










public func xOiGetAppManufacturerName(pPID, pLanguage, ...)
{
    var tLanguage = pLanguage[0];
    var tType = "Normal";
    if (pLanguage.size() > 1)
	tType = pLanguage[1];
    var tArg = xOiParam2Str(pPID) + ", " + tLanguage + ", " + tType;

    if (xOiFuncs.sHasOfmlAppFuncGetManufacturerName == 0) return(NULL);
    if (xOiFuncs.sHasOfmlAppFuncGetManufacturerName == 1)
	return(::ofml::app::getManufacturerName(tArg));

    xOiFuncs.sHasOfmlAppFuncGetManufacturerName = 1;
    var tMName = NULL;
    try {
	tMName = ::ofml::app::getManufacturerName(tArg);
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetManufacturerName = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getManufacturerName(): " + e.what);
    }

    return(tMName);
}











public func xOiGetAppManufacturerID(pPID, ...)
{
    var tPID = pPID[0];
    var tType = @PD;
    if (pPID.size() > 1)
	tType = pPID[1];
    var tArg = xOiParam2Str(tPID) + ", " + xOiParam2Str(tType);

    var tMID = NULL;

    if (xOiFuncs.sHasOfmlAppFuncGetManufacturerID == 0) return(tMID);
    if (xOiFuncs.sHasOfmlAppFuncGetManufacturerID == 1) {
	tMID = ::ofml::app::getManufacturerID(tArg);
	if (tMID == "NULL") tMID = NULL;
	return(tMID);
    }

    xOiFuncs.sHasOfmlAppFuncGetManufacturerID = 1;
    try {
	tMID = ::ofml::app::getManufacturerID(tArg);
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetManufacturerID = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getManufacturerID(): " + e.what);
    }
    if (tMID == "NULL") tMID = NULL;
    return(tMID);
}








public func xOiGetAppCommercialManID(pOfmlID)
{
    var tMID = NULL;

    try {
	tMID = ::ofml::app::getCommercialManID(pOfmlID);
    }
    catch (&e: Error) {
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getCommercialManID(): " + e.what);
    }
    if (!(tMID instanceof String && tMID != "NULL")) tMID = NULL;
    return(tMID);
}












public func xOiGetAppSeriesID(pPID, ...)
{
    var tPID = pPID[0];
    var tType = @PD;
    if (pPID.size() > 1)
	tType = pPID[1];
    var tArg = xOiParam2Str(tPID) + ", " + xOiParam2Str(tType);

    var tSID = NULL;

    if (xOiFuncs.sHasOfmlAppFuncGetSeriesID == 0) return(tSID);
    if (xOiFuncs.sHasOfmlAppFuncGetSeriesID == 1) {
	tSID = ::ofml::app::getSeriesID(tArg);
	if (tSID == "NULL") tSID = NULL;
	return(tSID);
    }

    xOiFuncs.sHasOfmlAppFuncGetSeriesID = 1;
    try {
	tSID = ::ofml::app::getSeriesID(tArg);
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetSeriesID = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getSeriesID(): " + e.what);
    }
    if (tSID == "NULL") tSID = NULL;
    return(tSID);
}










public func xOiGetAppMetaType(pPID)
{
    if (xOiFuncs.sHasOfmlAppFuncGetMetaType == 0) return(NULL);

    var tMT = NULL;

    if (xOiFuncs.sHasOfmlAppFuncGetMetaType == 1) {
	tMT = ::ofml::app::getMetaType(xOiParam2Str(pPID));
	if (tMT == "NULL") tMT = NULL;
	return(tMT);
    }

    xOiFuncs.sHasOfmlAppFuncGetMetaType = 1;
    try {
	tMT = ::ofml::app::getMetaType(xOiParam2Str(pPID));
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetMetaType = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getMetaType(): " + e.what);
    }
    if (tMT == "NULL") tMT = NULL;
    return(tMT);
}















public func xOiGetAppMetaSeries(pMID, ...)
{
    var tMID = pMID[0];
    if (pMID.size() > 1 && pMID[1])
	tMID = xOiGetAppCommercialManID(pMID[0]);

    
    

    if (xOiFuncs.sApppMetaSeries.hasKey(tMID)) 
	return(xOiFuncs.sApppMetaSeries[tMID]);

    var tSID = NULL;

    if (xOiFuncs.sHasOfmlAppFuncGetMetaSeries == 0) return(tSID);
    if (xOiFuncs.sHasOfmlAppFuncGetMetaSeries == 1) {
	tSID = ::ofml::app::getMetaSeries(tMID);
	if (tSID == "NULL") tSID = NULL;
	xOiFuncs.sApppMetaSeries[tMID] = tSID;
	return(tSID);
    }

    xOiFuncs.sHasOfmlAppFuncGetMetaSeries = 1;
    try {
	tSID = ::ofml::app::getMetaSeries(tMID);
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetMetaSeries = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getMetaSeries(): " + e.what);
    }
    if (tSID == "NULL") tSID = NULL;

    xOiFuncs.sApppMetaSeries[tMID] = tSID;

    return(tSID);
}










public func xOiGetAppProgID(pManufacturerID, pSeriesID)
{
    if (xOiFuncs.sHasOfmlAppFuncGetProgID == 0) return(NULL);

    var tArgs = pManufacturerID + ", " + pSeriesID;

    if (xOiFuncs.sHasOfmlAppFuncGetProgID == 1)
	return(eval(::ofml::app::getProgID(tArgs)));

    xOiFuncs.sHasOfmlAppFuncGetProgID = 1;
    var tPID = NULL;
    try {
	tPID = eval(::ofml::app::getProgID(tArgs));
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetProgID = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getProgID(): " + e.what);
    }
    return(tPID);
}








public func xOiGetAppProgInfoType(pPID)
{
    if (xOiFuncs.sHasOfmlAppFuncGetProgInfoType == 0) return(NULL);
    if (xOiFuncs.sHasOfmlAppFuncGetProgInfoType == 1)
	return(eval(::ofml::app::getProgInfoType(xOiParam2Str(pPID))));

    xOiFuncs.sHasOfmlAppFuncGetProgInfoType = 1;
    var tPIT = NULL;
    try {
	tPIT = eval(::ofml::app::getProgInfoType(xOiParam2Str(pPID)));
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetProgInfoType = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getProgInfoType(): " + e.what);
    }
    return(tPIT);
}








public func xOiGetAppPDBType(pPID)
{
    
    

    if (xOiFuncs.sAppPDBType.hasKey(pPID)) return(xOiFuncs.sAppPDBType[pPID]);

    var tPDBT = NULL;

    if (xOiFuncs.sHasOfmlAppFuncGetPDBType == 0) {
	xOiFuncs.sAppPDBType[pPID] = NULL;
	return(NULL);
    }

    if (xOiFuncs.sHasOfmlAppFuncGetPDBType == 1) {
	try { tPDBT = eval(::ofml::app::getPDBType(xOiParam2Str(pPID))); }
	xOiFuncs.sAppPDBType[pPID] = tPDBT;
	return(tPDBT);
    }

    xOiFuncs.sHasOfmlAppFuncGetPDBType = 1;
    try {
	try { tPDBT = eval(::ofml::app::getPDBType(xOiParam2Str(pPID))); }
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetPDBType = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getPDBType(): " + e.what);
    }
    xOiFuncs.sAppPDBType[pPID] = tPDBT;
    return(tPDBT);
}







public func xOiGetAppProductDBPath(pPID)
{
    
    

    if (xOiFuncs.sAppProductDBPath.hasKey(pPID))
	return(xOiFuncs.sAppProductDBPath[pPID]);

    var tPath = NULL;

    

    if (xOiFuncs.sHasOfmlAppFuncGetProductDBPath == 0 &&
	xOiFuncs.sHasEgrAppFuncGetProductDBPath == 0) {
	xOiFuncs.sAppProductDBPath[pPID] = tPath;
	return(tPath);
    }

    if (xOiFuncs.sHasOfmlAppFuncGetProductDBPath == 1) {
	tPath = ::ofml::app::getProductDBPath(xOiParam2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
	xOiFuncs.sAppProductDBPath[pPID] = tPath;
	return(tPath);
    }
    if (xOiFuncs.sHasEgrAppFuncGetProductDBPath == 1) {
	tPath = ::egr::app::getProductDBPath(xOiParam2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
	xOiFuncs.sAppProductDBPath[pPID] = tPath;
	return(tPath);
    }

    xOiFuncs.sHasOfmlAppFuncGetProductDBPath = 1;
    try {
	tPath = ::ofml::app::getProductDBPath(xOiParam2Str(pPID));
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetProductDBPath = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getProductDBPath(): " + e.what);
    }
    if (xOiFuncs.sHasOfmlAppFuncGetProductDBPath == 1) {
	if (tPath == "NULL") tPath = NULL;
	xOiFuncs.sAppProductDBPath[pPID] = tPath;
	return(tPath);
    }

    xOiFuncs.sHasEgrAppFuncGetProductDBPath = 1;
    try {
	tPath = ::egr::app::getProductDBPath(xOiParam2Str(pPID));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncGetProductDBPath = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::getProductDBPath():" + e.what);
    }
    if (tPath == "NULL") tPath = NULL;

    xOiFuncs.sAppProductDBPath[pPID] = tPath;
    return(tPath);
}











public func xOiGetAppRegionVersionPath(pPID)
{
    var tPath = NULL;

    
    

    
    

    if (xOiFuncs.sAppRegionVersionPath.hasKey(pPID))
	return(xOiFuncs.sAppRegionVersionPath[pPID]);

    var tManufacturer = xOiGetAppRegistryKey(pPID, "manufacturer", NULL, NULL);
    var tProgram = xOiGetAppRegistryKey(pPID, "program", NULL, NULL);
    var tRegion = xOiGetAppRegistryKey(pPID, "distribution_region", NULL, NULL);
    var tVersion = xOiGetAppRegistryKey(pPID, "version", NULL, NULL);

    if (tManufacturer == NULL || tProgram == NULL || tRegion == NULL ||
	tVersion == NULL) {
	var tPDBPath = xOiGetAppProductDBPath(pPID);
	if (tPDBPath != NULL) {
	    var tLastSlash = tPDBPath.findLastOf('/');
	    if (tLastSlash >= 0)
		tPath = tPDBPath.substr(0, tLastSlash);
	}
    }
    else {
	
	
	tManufacturer = xOiStripPadChar(tManufacturer, '\"', 1);
	tProgram =      xOiStripPadChar(tProgram, '\"', 1);
	tRegion =       xOiStripPadChar(tRegion, '\"', 1);
	tVersion =      xOiStripPadChar(tVersion, '\"', 1);

	tPath = tManufacturer + "/" + tProgram + "/" + tRegion + "/" + tVersion;
    }

    xOiFuncs.sAppRegionVersionPath[pPID] = tPath;

    return(tPath);
}
















public func xOiGetAppIndexpricesPath(pPID)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetAppIndexpricesPath", pPID);

    var tPath = NULL;

    
    

    if (xOiFuncs.sAppIndexpricesPath.hasKey(pPID)) {
	tPath = xOiFuncs.sAppIndexpricesPath[pPID];
	xOiDebugFinishFunc(tPath);
	return(tPath);
    }

    tPath = xOiGetAppRegistryKey(pPID, "indexprices_path", NULL, NULL);

    if (tPath != NULL) {
	xOiDebugPrint(@Info, 1, "using registry key 'indexprices_path'");
	xOiFuncs.sAppIndexpricesPath[pPID] = tPath;
	xOiDebugFinishFunc(tPath);
	return(tPath);
    }

    var tManufacturer = xOiGetAppRegistryKey(pPID, "manufacturer", NULL, NULL);
    var tRegion = xOiGetAppRegistryKey(pPID, "distribution_region", NULL, NULL);
    var tVersion = xOiGetAppRegistryKey(pPID, "version", NULL, NULL);

    if (tManufacturer == NULL || tRegion == NULL || tVersion == NULL) {
	xOiDebugPrint(@Warn, 1, "incomplete registry info!");
    }
    else {
	
	
	tManufacturer = xOiStripPadChar(tManufacturer, '\"', 1);
	tRegion =       xOiStripPadChar(tRegion, '\"', 1);
	tVersion =      xOiStripPadChar(tVersion, '\"', 1);

	tPath = tManufacturer + "/indexprices/" + tRegion + "/" + tVersion;
    }

    xOiFuncs.sAppIndexpricesPath[pPID] = tPath;
    xOiDebugFinishFunc(tPath);
    return(tPath);
}












public func xOiGetAppOAMPath(pPID)
{
    
    

    if (xOiFuncs.sAppOAMPath.hasKey(pPID)) return(xOiFuncs.sAppOAMPath[pPID]);

    var tPath = NULL;

    if (xOiFuncs.sHasOfmlAppFuncGetOAMPath == 1) {
	tPath = ::ofml::app::getOAMPath(xOiParam2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
    }
    else
    if (xOiFuncs.sHasOfmlAppFuncGetOAMPath == -1) {
        xOiFuncs.sHasOfmlAppFuncGetOAMPath = 1;
        try {
	    tPath = ::ofml::app::getOAMPath(xOiParam2Str(pPID));
            if (tPath == "NULL") tPath = NULL;
        }
        catch (&e: Error) {
	    xOiFuncs.sHasOfmlAppFuncGetOAMPath = 0;
	    xOiDebugPrint(@ExplWarn, 1, 
		          "Error during evaluation of function " +
		          "::ofml::app::getOAMPath(): " + e.what);
        }
    }

    if (tPath == NULL && (tPath = xOiGetAppProductDBPath(pPID)) != NULL) {
	
	var tPos = tPath.rfind("/");
	if (tPos < 0)
	    tPath = NULL;
	else
	    tPath = tPath.substr(0,tPos+1) + "oam";
    }
    xOiFuncs.sAppOAMPath[pPID] = tPath;
    return(tPath);
}







public func xOiGetAppProgInfoDBPath(pPID)
{
    
    

    if (xOiFuncs.sAppProgInfoDBPath.hasKey(pPID))
	return(xOiFuncs.sAppProgInfoDBPath[pPID]);

    if (xOiFuncs.sHasOfmlAppFuncGetProgInfoDBPath == 0) {
	xOiFuncs.sAppProgInfoDBPath[pPID] = NULL;
	return(NULL);
    }

    if (xOiFuncs.sHasOfmlAppFuncGetProgInfoDBPath == 1) {
        var tPath = ::ofml::app::getProginfoDBPath(xOiParam2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
	xOiFuncs.sAppProgInfoDBPath[pPID] = tPath;
	return(tPath);
    }

    var tPath = NULL;

    xOiFuncs.sHasOfmlAppFuncGetProgInfoDBPath = 1;
    try {
        tPath = ::ofml::app::getProginfoDBPath(xOiParam2Str(pPID));
        if (tPath == "NULL") tPath = NULL;
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetProgInfoDBPath = 0;
	xOiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function getProginfoDBPath()!\n");
    }
    xOiFuncs.sAppProgInfoDBPath[pPID] = tPath;
    return(tPath);
}

func _xOiGetOfmlAppLibPath(pPID)
{
    return(::ofml::app::getLibPath(pPID));
}

func _xOiGetEgrAppLibPath(pPID)
{
    return(::egr::app::getLibPath(pPID));
}







public func xOiGetAppLibPath(pPID)
{
    xOiDebugStartFunc3("xOiFuncs::xOiGetAppLibPath", pPID);

    var tPath = NULL;

    
    

    if (xOiFuncs.sAppLibPath.hasKey(pPID)) {
	xOiDebugPrint(@Info, 1, "using cache");
	tPath = xOiFuncs.sAppLibPath[pPID];
	xOiDebugFinishFunc(tPath);
	return(tPath);
    }

    

    if (xOiFuncs.sHasOfmlAppFuncGetLibPath == 0 &&
	xOiFuncs.sHasEgrAppFuncGetLibPath == 0) {
	xOiDebugPrint(@Warn, 1, "not supported by application");
	xOiFuncs.sAppLibPath[pPID] = NULL;
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }
    if (xOiFuncs.sHasOfmlAppFuncGetLibPath == 1) {
	
	tPath = _xOiGetOfmlAppLibPath(xOiParam2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
	xOiFuncs.sAppLibPath[pPID] = tPath;
	xOiDebugFinishFunc(tPath);
	return(tPath);
    }
    if (xOiFuncs.sHasEgrAppFuncGetLibPath == 1) {
	
	tPath = _xOiGetEgrAppLibPath(xOiParam2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
	xOiFuncs.sAppLibPath[pPID] = tPath;
	xOiDebugFinishFunc(tPath);
	return(tPath);
    }

    xOiFuncs.sHasOfmlAppFuncGetLibPath = 1;
    try {
	
	tPath = _xOiGetOfmlAppLibPath(xOiParam2Str(pPID));
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetLibPath = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getLibPath(): " + e.what);
    }
    if (xOiFuncs.sHasOfmlAppFuncGetLibPath == 1) {
	if (tPath == "NULL") tPath = NULL;
	xOiFuncs.sAppLibPath[pPID] = tPath;
	xOiDebugFinishFunc(tPath);
	return(tPath);
    }

    xOiFuncs.sHasEgrAppFuncGetLibPath = 1;
    try {
	
	tPath = _xOiGetEgrAppLibPath(xOiParam2Str(pPID));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncGetLibPath = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::getLibPath(): " + e.what);
    }
    if (tPath == "NULL") tPath = NULL;

    xOiFuncs.sAppLibPath[pPID] = tPath;
    xOiDebugFinishFunc(tPath);
    return(tPath);
}














public func xOiGetAppLanguage()
{
    return(oiGetAppLanguage());
}






public func xOiGetAppPrimaryCurrency()
{
    if (xOiFuncs.sHasOfmlAppFuncGetPrimaryCurrency == 0) return(NULL);
    if (xOiFuncs.sHasOfmlAppFuncGetPrimaryCurrency == 1)
	return(::ofml::app::getPrimaryCurrency(""));

    xOiFuncs.sHasOfmlAppFuncGetPrimaryCurrency = 1;
    var tCurr = NULL;
    try {
	tCurr = ::ofml::app::getPrimaryCurrency("");
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetPrimaryCurrency = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getPrimaryCurrency(): " + e.what);
    }

    return(tCurr);
}






public func xOiGetAppSecondaryCurrency()
{
    if (xOiFuncs.sHasOfmlAppFuncGetSecondaryCurrency == 0) return(NULL);
    if (xOiFuncs.sHasOfmlAppFuncGetSecondaryCurrency == 1)
	return(::ofml::app::getSecondaryCurrency(""));

    xOiFuncs.sHasOfmlAppFuncGetSecondaryCurrency = 1;
    var tCurr = NULL;
    try {
	tCurr = ::ofml::app::getSecondaryCurrency("");
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetSecondaryCurrency = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getSecondaryCurrency(): " + e.what);
    }

    return(tCurr);
}








public func xOiGetAppConfigProfileTableID()
{
    var tTID = NULL;

    if (xOiFuncs.sHasOfmlAppFuncGetConfigProfileTableID == 0) return(tTID);
    if (xOiFuncs.sHasOfmlAppFuncGetConfigProfileTableID == 1) {
	tTID = ::ofml::app::getConfigProfileTableID("");
	if (tTID == "NULL" || tTID.empty()) tTID = NULL;
	return(tTID);
    }

    xOiFuncs.sHasOfmlAppFuncGetConfigProfileTableID = 1;
    try {
	tTID = ::ofml::app::getConfigProfileTableID("");
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetConfigProfileTableID = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getConfigProfileTableID(): " + e.what);
    }
    if (tTID != NULL && (tTID == "NULL" || tTID.empty())) tTID = NULL;
    return(tTID);
}









public func xOiGetAppUpdateMode()
{
    xOiDebugStartFunc("xOiFuncs::xOiGetAppUpdateMode", NULL);

    var tValidModes = @("@ImplicitUpdate", "@UserUpdate");
    var tMode = NULL;

    if (xOiFuncs.sHasOfmlAppFuncGetUpdateMode == 0)  {
	xOiDebugFinishFunc(tMode);
	return(tMode);
    }
    if (xOiFuncs.sHasOfmlAppFuncGetUpdateMode == 1) {
	tMode = ::ofml::app::getUpdateMode("");
	xOiDebugPrint(@Info, 2, ["result from app-call: ", tMode]);
	if (tValidModes.find(tMode) < 0) 
	    tMode = NULL;
	else
	    tMode = eval(tMode);
	xOiDebugFinishFunc(tMode);
	return(tMode);
    }

    xOiFuncs.sHasOfmlAppFuncGetUpdateMode = 1;
    try {
	tMode = ::ofml::app::getUpdateMode("");
	xOiDebugPrint(@Info, 2, ["result from app-call: ", tMode]);
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncGetUpdateMode = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getUpdateMode(): " + e.what);
    }
    if (tMode != NULL) 
	if (tValidModes.find(tMode) < 0)
	    tMode = NULL;
	else
	    tMode = eval(tMode);
    xOiDebugFinishFunc(tMode);
    return(tMode);
}
































public func xOiAppCreateArticle(pFather, pRefObj, pPos, pYRot, pPID, 
				pArticle, pVarCode, pUseCat, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiAppCreateArticle",
		      [pFather, pRefObj, pPos, pYRot, pPID, pArticle, pVarCode, 
		       pUseCat]);

    var tUseCat = pUseCat[0];
    var tMode = NULL;
    if (pUseCat.size() > 1)
	tMode = pUseCat[1];

    var tRet = NULL;

    if (xOiFuncs.sHasOfmlAppFuncCreateArticle == 0) {
	xOiDebugPrint(@Warn, 1, "::ofml::app::createArticle() not supported");
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tFName = "NULL";
    if (pFather != NULL)
	tFName = "\""+pFather.getName()+"\"";
    var tRName = "NULL";
    if (pRefObj != NULL)
	tRName = "\""+pRefObj.getName()+"\"";
    var tModeStr = "";
    if (tMode != NULL)
	tModeStr = "," + xOiParam2Str(tMode);

    var tArgs = "["+tFName+","+tRName+","+xOiParam2Str(pPos)+","+
		xOiParam2Str(pYRot)+","+xOiParam2Str(pPID)+",\""+pArticle+
		"\",\""+pVarCode+"\","+xOiParam2Str(tUseCat)+tModeStr+"]";

    xOiDebugPrint(@Info, 2, ["args: ", tArgs]);

    if (xOiFuncs.sHasOfmlAppFuncCreateArticle == 1) {
	tRet = eval(::ofml::app::createArticle(tArgs));
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    xOiFuncs.sHasOfmlAppFuncCreateArticle = 1;
    try {
	tRet = eval(::ofml::app::createArticle(tArgs));
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncCreateArticle = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::createArticle(): " + e.what);
    }
    xOiDebugFinishFunc(tRet);
    return(tRet);
}























public func xOiAppCopyObject(pOrigObj, pFather, pRefObj, pPos, pYRot, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiAppCopyObject",
		      [pOrigObj, pFather, pRefObj, pPos, pYRot]);

    var tYRot = pYRot[0];
    var tMode = NULL;
    if (pYRot.size() > 1)
	tMode = pYRot[1];

    var tRet = NULL;

    if (xOiFuncs.sHasOfmlAppFuncCopyObject == 0) {
	xOiDebugPrint(@Warn, 1, "::ofml::app::copyObject() not supported");
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tOName = "\""+pOrigObj.getName()+"\"";
    var tFName = "NULL";
    if (pFather != NULL)
	tFName = "\""+pFather.getName()+"\"";
    var tRName = "NULL";
    if (pRefObj != NULL)
	tRName = "\""+pRefObj.getName()+"\"";

    var tModeStr = "";
    if (tMode != NULL)
	tModeStr = "," + xOiParam2Str(tMode);

    var tArgs = "["+tFName+","+tRName+","+xOiParam2Str(pPos)+","+
		xOiParam2Str(pYRot)+","+tOName+tModeStr+"]";

    xOiDebugPrint(@Info, 2, ["args: ", tArgs]);

    if (xOiFuncs.sHasOfmlAppFuncCopyObject == 1) {
	tRet = eval(::ofml::app::copyObject(tArgs));
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    xOiFuncs.sHasOfmlAppFuncCopyObject = 1;
    try {
	tRet = eval(::ofml::app::copyObject(tArgs));
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncCopyObject = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::copyObject(): " + e.what);
    }
    xOiDebugFinishFunc(tRet);
    return(tRet);
}
















public func xOiRegisterPackage(pPID)
{
    return(oiRegisterPackage(pPID));
}







public func xOiIsPropTypeSupported(pPType)
{
    var tRet = NULL;

    if (xOiFuncs.sHasOfmlAppFuncIsPropTypeSupported == 0) return(tRet);

    if (xOiFuncs.sAppIsPropTypeSupported.hasKey(pPType)) 
	return(xOiFuncs.sAppIsPropTypeSupported[pPType]);

    if (xOiFuncs.sHasOfmlAppFuncIsPropTypeSupported == 1) {
        var tRes = ::ofml::app::isPropTypeSupported(xOiParam2Str(pPType));
	try {
	    tRet = Int(tRes);
	}
	xOiFuncs.sAppIsPropTypeSupported[pPType] = tRet;
	return(tRet);
    }

    xOiFuncs.sHasOfmlAppFuncIsPropTypeSupported = 1;
    try {
        var tRes = ::ofml::app::isPropTypeSupported(xOiParam2Str(pPType));
	try {
	    tRet = Int(tRes);
	}
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncIsPropTypeSupported = 0;
	xOiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function isPropTypeSupported()!\n");
    }
    xOiFuncs.sAppIsPropTypeSupported[pPType] = tRet;
    return(tRet);
}












public func xOiHandleNotUpdatedArticlesByApp()
{
    var tRet = NULL;

    if (xOiFuncs.sHasOfmlAppFuncHandleNotUpdatedArticles == 0)
	return(tRet);
    if (xOiFuncs.sHasOfmlAppFuncHandleNotUpdatedArticles == 1) {
        var tRes = ::ofml::app::handleNotUpdatedArticles("");
	try {
	    tRet = Int(tRes);
	    if (tRet != 1 && tRet != 2) tRet = 0;
	}
	return(tRet);
    }

    xOiFuncs.sHasOfmlAppFuncHandleNotUpdatedArticles = 1;
    try {
        var tRes = ::ofml::app::handleNotUpdatedArticles("");
	try {
	    tRet = Int(tRes);
	    if (tRet != 1 && tRet != 2) tRet = 0;
	}
    }
    catch (&e: Error) {
	xOiFuncs.sHasOfmlAppFuncHandleNotUpdatedArticles = 0;
	xOiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function handleNotUpdatedArticles()!");
    }
    return(tRet);
}














public func xOiNeedAppUserDefinedOrderData()
{
    if (xOiFuncs.sHasEgrAppFuncNeedUserDefinedOrderData == 0) return(0);
    if (xOiFuncs.sHasEgrAppFuncNeedUserDefinedOrderData == 1) {
	var tAppRes = eval(::egr::app::needUserDefinedOrderData(""));
	return(tAppRes != NULL && tAppRes);
    }

    xOiFuncs.sHasEgrAppFuncNeedUserDefinedOrderData = 1;
    var tRes = 0;
    try {
	var tAppRes = eval(::egr::app::needUserDefinedOrderData(""));
	tRes = (tAppRes != NULL && tAppRes);
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncNeedUserDefinedOrderData = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::needUserDefinedOrderData(): " + e.what);
    }

    return(tRes);
}








public func xOiGetAppDebugInfo()
{
    if (xOiFuncs.sHasEgrAppFuncGetDebugInfo == 0) return(NULL);
    if (xOiFuncs.sHasEgrAppFuncGetDebugInfo == 1)
	return(eval(::egr::app::getDebugInfo("")));

    xOiFuncs.sHasEgrAppFuncGetDebugInfo = 1;
    var tInfo = NULL;
    try {
	tInfo = eval(::egr::app::getDebugInfo(""));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncGetDebugInfo = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::getDebugInfo(): " + e.what);
    }

    return(tInfo);
}








public func xOiGetAppDebugClasses()
{
    if (xOiFuncs.sHasEgrAppFuncGetDebugClasses == 0) return(NULL);
    if (xOiFuncs.sHasEgrAppFuncGetDebugClasses == 1)
	return(eval(::egr::app::getDebugClasses("")));

    xOiFuncs.sHasEgrAppFuncGetDebugClasses = 1;
    var tClasses = NULL;
    try {
	tClasses = eval(::egr::app::getDebugClasses(""));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncGetDebugClasses = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::getDebugClasses(): " + e.what);
    }

    return(tClasses);
}























public func xOiApplyAppPriceProfiles(pManID, pSeriesID, pArticle, 
				     pVarCode, pPrice)
{
    xOiDebugStartFunc("xOiFuncs::xOiApplyAppPriceProfiles",
		      [pManID, pSeriesID, pArticle, pVarCode, pPrice]);

    if (xOiFuncs.sHasEgrAppFuncApplyPriceProfiles  == 0 &&
	xOiFuncs.sHasEgrAppFuncApplyPriceProfiles2 == 0)  {
	xOiDebugPrint(@Warn, 1, "no application function(s)!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tPriceHasVarConds = (pPrice != NULL && !pPrice.empty() && 
			     pPrice.front().size() >= 4);

    var tArgs =  xOiParam2Str([pManID, pSeriesID, pArticle, pVarCode, pPrice]);

    if (tPriceHasVarConds) {
	if (xOiFuncs.sHasEgrAppFuncApplyPriceProfiles2 == 1) {
	    var tRet = eval(::egr::app::applyPriceProfiles2(tArgs));
	    xOiDebugPrint(@Info, 2, "used applyPriceProfiles2()");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}
        if (xOiFuncs.sHasEgrAppFuncApplyPriceProfiles2 == 0 &&
	    xOiFuncs.sHasEgrAppFuncApplyPriceProfiles == 1) {
	    var tStrippedPrice = @();
	    var tP;
	    foreach(tP; pPrice) tStrippedPrice.pushBack([tP[0], tP[1], tP[2]]);
	    xOiDebugPrint(@Info, 2, 
			  ["stripped price for applyPriceProfiles(): ",
			   tStrippedPrice]);
	    tArgs =  xOiParam2Str([pManID, pSeriesID, pArticle, 
			           pVarCode, tStrippedPrice]);
	    var tRet = eval(::egr::app::applyPriceProfiles(tArgs));
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}
    }
    else
    if (xOiFuncs.sHasEgrAppFuncApplyPriceProfiles == 1) {
	var tRet = eval(::egr::app::applyPriceProfiles(tArgs));
	xOiDebugPrint(@Info, 2, "used applyPriceProfiles()");
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tNewPrice = NULL;

    if (tPriceHasVarConds) {
	var tOk = 0;
        xOiFuncs.sHasEgrAppFuncApplyPriceProfiles2 = 1;
        try {
	    tNewPrice = eval(::egr::app::applyPriceProfiles2(tArgs));
	    tOk = 1;
        }
        catch (&e: Error) {
	    xOiFuncs.sHasEgrAppFuncApplyPriceProfiles2 = 0;
	    xOiDebugPrint(@ExplWarn, 1, 
		          "Error during evaluation of function " +
		          "::egr::app::applyPriceProfiles2(): " + e.what);
	}
	if (tOk) {
	    xOiDebugPrint(@Info, 2, "used applyPriceProfiles2()");
	    xOiDebugFinishFunc(tNewPrice);
	    return(tNewPrice);
	}
	else {
	    var tStrippedPrice = @();
	    var tP;
	    foreach(tP; pPrice) tStrippedPrice.pushBack([tP[0], tP[1], tP[2]]);
	    xOiDebugPrint(@Info, 2, 
			  ["stripped price for applyPriceProfiles(): ",
			   tStrippedPrice]);
	    tArgs =  xOiParam2Str([pManID, pSeriesID, pArticle,
				   pVarCode, tStrippedPrice]);
	    if (xOiFuncs.sHasEgrAppFuncApplyPriceProfiles == 1) {
		var tRet = eval(::egr::app::applyPriceProfiles(tArgs));
		xOiDebugPrint(@Info, 2, "used applyPriceProfiles2()");
		xOiDebugFinishFunc(tRet);
		return(tRet);
	    }
	}
    }

    xOiFuncs.sHasEgrAppFuncApplyPriceProfiles = 1;
    try {
	tNewPrice = eval(::egr::app::applyPriceProfiles(tArgs));
	xOiDebugPrint(@Info, 2, "used applyPriceProfiles()");
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncApplyPriceProfiles = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::applyPriceProfiles(): " + e.what);
    }

    xOiDebugFinishFunc(tNewPrice);
    return(tNewPrice);
}

























public func xOiConvert2AppCurrency(pManID, pSeriesID, pBaseArtNr, pFinalArtNr, 
				   pOFMLVarCode, pCurrency, pPurchasePrice, 
				   pSalesPrice, pPriceLevel)
{
    xOiDebugStartFunc("xOiFuncs::xOiConvert2AppCurrency",
		      [pManID, pSeriesID, pBaseArtNr, pFinalArtNr, pOFMLVarCode,
		       pCurrency, pPurchasePrice, pSalesPrice, pPriceLevel]);

    var tRet = NULL;

    if (xOiFuncs.sHasEgrAppFuncConvertCurrency == 0) {
        xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    if (xOiFuncs.sHasEgrAppFuncConvertCurrency == 1) {
        tRet = ::egr::app::convertCurrency(pManID, pSeriesID, pBaseArtNr, 
					   pFinalArtNr, pOFMLVarCode, pCurrency,
					   pPurchasePrice, pSalesPrice, 
					   pPriceLevel);
        xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    xOiFuncs.sHasEgrAppFuncConvertCurrency = 1;
    try {
        tRet = ::egr::app::convertCurrency(pManID, pSeriesID, pBaseArtNr, 
					   pFinalArtNr, pOFMLVarCode, pCurrency,
					   pPurchasePrice, pSalesPrice, 
					   pPriceLevel);
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncConvertCurrency = 0;
	xOiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::egr::app or that\n" +
	    "package does not implement function convertCurrency()!");
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}





















public func xOiSendProgressEvent2App(pEventType, pProgress, pText, pMode)
{
    xOiDebugStartFunc("xOiFuncs::xOiSendProgressEvent2App",
		      [pEventType, pProgress, pText, pMode]);

    xOiDebugPrint(@Progress, 1, 
		  ["progress event: ", [pEventType, pProgress, pText, pMode]]);

    if (xOiFuncs.sHasEgrAppFuncProgressEvent == 0) {
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tArgs = xOiParam2Str(pEventType) + ", " + xOiParam2Str(pProgress) + 
		", " + xOiParam2Str(pText) + ", " + xOiParam2Str(pMode);

    if (xOiFuncs.sHasEgrAppFuncProgressEvent == 1) {
	xOiDebugPrint(@Info, 2, ["call args: ", tArgs]);
	::egr::app::progressEvent(tArgs);
	xOiDebugFinishFunc(1);
	return(1);
    }

    xOiFuncs.sHasEgrAppFuncProgressEvent = 1;
    var tRet = 1;

    xOiDebugPrint(@Info, 2, ["call args: ", tArgs]);

    try {
	::egr::app::progressEvent(tArgs);
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncProgressEvent = 0;
	tRet = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::progressEvent(): " + e.what);
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}
















public func xOiShowAppHint(pText, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiShowAppHint", pText);

    var tText = pText[0];
    var tOiOutput = 0;
    if (pText.size() > 1)
	tOiOutput = pText[1];

    tText = oiGetStringResource(tText, oiGetPlanning().getLanguage());

    if (xOiFuncs.sHasEgrAppFuncShowHint == 0) {
	if (tOiOutput) oiOutput(@MESSAGE, tText);
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tArgs = "[" + xOiParam2Str(tText) + "]";

    if (xOiFuncs.sHasEgrAppFuncShowHint == 1) {
	::egr::app::showHint(tArgs);
	xOiDebugFinishFunc(1);
	return(1);
    }

    xOiFuncs.sHasEgrAppFuncShowHint = 1;
    try {
	::egr::app::showHint(tArgs);
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncShowHint = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::showHint(): " + e.what);
	if (tOiOutput) oiOutput(@MESSAGE, tText);
    }
    xOiDebugFinishFunc(xOiFuncs.sHasEgrAppFuncShowHint);
    return(xOiFuncs.sHasEgrAppFuncShowHint);
}







public func xOiIsAppLicensedProg(pPID)
{
    xOiDebugStartFunc("xOiFuncs::xOiIsAppLicensedProg", pPID);

    
    

    if (xOiFuncs.sLicensedPrograms.hasKey(pPID)) {
	var tRet = xOiFuncs.sLicensedPrograms[pPID];
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tRet = 1;

    if (xOiFuncs.sHasEgrAppFuncIsLicensedProg == 0) {
	xOiDebugPrint(@Warn, 1, "not supported by application");
	xOiFuncs.sLicensedPrograms[pPID] = tRet;
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tValidValues = @("0", "1");
    var tRes;

    xOiFuncs.sHasEgrAppFuncIsLicensedProg = 1;
    try {
	tRes = ::egr::app::isLicensedProg(xOiParam2Str(pPID));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppFuncIsLicensedProg = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::isLicensedProg(): " + e.what);
    }
    if (tRes instanceof String && tValidValues.find(tRes) >= 0)
	tRet = Int(tRes);
    else
	xOiDebugPrint(@Warn, 1, 
		      ["invalid return value from app function: ", tRes]);

    xOiFuncs.sLicensedPrograms[pPID] = tRet;
    xOiDebugFinishFunc(tRet);
    return(tRet);
}








public func xOiSetAppMultiFMLProcessState(pKey, pValue)
{
    xOiDebugStartFunc("xOiFuncs::xOiSetAppMultiFMLProcessState", 
		      [pKey, pValue]);

    var tArgs = xOiParam2Str(pKey) + ", " + xOiParam2Str(pValue);
    var tRet = 1;
    try {
	::egr::app::setMultiFMLProcessState(tArgs);
    }
    catch (&e: Error) {
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::setMultiFMLProcessState(): " + e.what);
	tRet = 0;
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}









public func xOiGetAppMultiFMLProcessState(pKey)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetAppMultiFMLProcessState", pKey);

    var tRes = NULL;
    try {
	tRes = ::egr::app::getMultiFMLProcessState(xOiParam2Str(pKey));
    }
    catch (&e: Error) {
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::getMultiFMLProcessState(): " + e.what);
    }
    if (tRes == NULL) {
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tRet = NULL;
    try {
	tRet = eval(tRes);
    }
    catch (&e: Error) {
	xOiDebugPrint(@Warn, 1, 
		      ["invalid return value from app function: ", tRes]);
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}













public func xOiAppBasketAddItem(pType, pOID, pFatherOID)
{
    if (xOiFuncs.sHasEgrAppBasketFuncAddItem == 0) return(NULL);

    var tArg = xOiParam2Str([pType, pOID, pFatherOID]);

    if (xOiFuncs.sHasEgrAppBasketFuncAddItem == 1) 
	return(::egr::app::basket::addItem(tArg));

    xOiFuncs.sHasEgrAppBasketFuncAddItem = 1;
    var tRes = NULL;
    try {
	tRes = ::egr::app::basket::addItem(tArg);
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppBasketFuncAddItem = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::basket::addItem(): " + e.what);
    }
    return(tRes);
}






public func xOiAppBasketRemoveItem(pOID)
{
    if (xOiFuncs.sHasEgrAppBasketFuncRemoveItem == 0) return(NULL);

    if (xOiFuncs.sHasEgrAppBasketFuncRemoveItem == 1) 
	return(::egr::app::basket::removeItem(xOiParam2Str(pOID)));

    xOiFuncs.sHasEgrAppBasketFuncRemoveItem = 1;
    var tRes = NULL;
    try {
	tRes = ::egr::app::basket::removeItem(xOiParam2Str(pOID));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppBasketFuncRemoveItem = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::basket::removeItem(): " + e.what);
    }
    return(tRes);
}






public func xOiAppBasketChangedItem(pOID)
{
    if (xOiFuncs.sHasEgrAppBasketFuncChangedItem == 0) return(NULL);

    if (xOiFuncs.sHasEgrAppBasketFuncChangedItem == 1) 
	return(::egr::app::basket::changedItem(xOiParam2Str(pOID)));

    xOiFuncs.sHasEgrAppBasketFuncChangedItem = 1;
    var tRes = NULL;
    try {
	tRes = ::egr::app::basket::changedItem(xOiParam2Str(pOID));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppBasketFuncChangedItem = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::basket::changedItem(): " + e.what);
    }
    return(tRes);
}

















































public func xOiAppObkLockObjects(pMode, pArgs)
{
    if (xOiFuncs.sHasEgrAppObkFuncLockObjects == 0) return(NULL);

    xOiDebugStartFunc("xOiFuncs::xOiAppObkLockObjects", [pMode, pArgs]);

    var tArg = "[" + xOiParam2Str(pMode) + "," + xOiParam2Str(pArgs) + "]";
    var tRes = NULL;

    if (xOiFuncs.sHasEgrAppObkFuncLockObjects == 1) {
	tRes = eval(::egr::app::mt::lockObjects(tArg));
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    xOiFuncs.sHasEgrAppObkFuncLockObjects = 1;
    try {
	tRes = eval(::egr::app::mt::lockObjects(tArg));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppObkFuncLockObjects = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::mt::lockObjects(): " + e.what);
    }

    xOiDebugFinishFunc(tRes);
    return(tRes);
}






public func xOiAppObkUnlockAllObjects()
{
    if (xOiFuncs.sHasEgrAppObkFuncUnlockAllObjects == 0) return(NULL);

    xOiDebugStartFunc("xOiFuncs::xOiAppObkUnlockAllObjects", NULL);

    if (xOiFuncs.sHasEgrAppObkFuncUnlockAllObjects == 1) {
	::egr::app::mt::unlockAllObjects(tArg);
	xOiDebugFinishFunc(NULL);
    }

    xOiFuncs.sHasEgrAppObkFuncUnlockAllObjects = 1;
    try {
	::egr::app::mt::unlockAllObjects(tArg);
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppObkFuncUnlockAllObjects = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::mt::unlockAllObjects(): " + e.what);
    }

    xOiDebugFinishFunc(NULL);
}

































public func xOiAppObkGetArticleData(pId, pKeys)
{
    if (xOiFuncs.sHasEgrAppObkFuncGetArticleData == 0) return(NULL);

    xOiDebugStartFunc("xOiFuncs::xOiAppObkGetArticleData", [pId, pKeys]);

    var tArg = "[" + xOiParam2Str(pId) + "," + xOiParam2Str(pKeys) + "]";
    var tRes = NULL;

    if (xOiFuncs.sHasEgrAppObkFuncGetArticleData == 1) {
	tRes = eval(::egr::app::mt::getArticleData(tArg));
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    xOiFuncs.sHasEgrAppObkFuncGetArticleData = 1;
    try {
	tRes = eval(::egr::app::mt::getArticleData(tArg));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppObkFuncGetArticleData = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::mt::getArticleData(): " + e.what);
    }

    xOiDebugFinishFunc(tRes);
    return(tRes);
}







public func xOiAppObkDeleteBasketItems(pIds)
{
    if (xOiFuncs.sHasEgrAppObkFuncDeleteBasketItems == 0) return(NULL);

    xOiDebugStartFunc("xOiFuncs::xOiAppObkDeleteBasketItems", pIds);

    var tArg = "[" + xOiParam2Str(pIds) + "]";
    var tRes = NULL;

    if (xOiFuncs.sHasEgrAppObkFuncDeleteBasketItems == 1) {
	tRes = eval(::egr::app::mt::deleteBasketItems(tArg));
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    xOiFuncs.sHasEgrAppObkFuncDeleteBasketItems = 1;
    try {
	tRes = eval(::egr::app::mt::deleteBasketItems(tArg));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppObkFuncDeleteBasketItems = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::mt::deleteBasketItems(): " + e.what);
    }

    xOiDebugFinishFunc(tRes);
    return(tRes);
}



























































public func xOiAppObkCreateObject(pMode, pPkg, pArtNo, pVarCode, pArgs, 
				  pPCContext, pWCPos, pWCRot)
{
    if (xOiFuncs.sHasEgrAppObkFuncCreateObject == 0) return(NULL);

    xOiDebugStartFunc("xOiFuncs::xOiAppObkCreateObject", 
		      [pMode, pPkg, pArtNo, pVarCode, pArgs, pPCContext, 
		       pWCPos, pWCRot]);

    var tArg = "[" + xOiParam2Str(pMode) + "," + 
		     xOiParam2Str(pPkg) + "," +
		     xOiParam2Str(pArtNo) + "," +
		     xOiParam2Str(pVarCode) + "," +
		     (pArgs == NULL ? "\"\"" : xOiParam2Str(pArgs)) + "," +
		     xOiParam2Str(pPCContext) + "," +
		     xOiParam2Str(pWCPos) + "," +
		     xOiParam2Str(pWCRot[1]) + "]";

    var tRes = NULL;

    if (xOiFuncs.sHasEgrAppObkFuncCreateObject == 1) {
	tRes = eval(::egr::app::mt::createObject(tArg));
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    xOiFuncs.sHasEgrAppObkFuncCreateObject = 1;
    try {
	tRes = eval(::egr::app::mt::createObject(tArg));
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppObkFuncCreateObject = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::mt::createObject(): " + e.what);
    }

    xOiDebugFinishFunc(tRes);
    return(tRes);
}








public func xOiAppObkSetDefaultPriceDate(pDate)
{
    if (xOiFuncs.sHasEgrAppObkFuncSetDefaultPriceDate == 0) return(NULL);

    xOiDebugStartFunc("xOiFuncs::xOiAppObkSetDefaultPriceDate", pDate);

    if (xOiFuncs.sHasEgrAppObkFuncSetDefaultPriceDate == 1) {
	::egr::app::mt::setDefaultPriceDate(pDate);
	xOiDebugFinishFunc(NULL);
	return;
    }

    xOiFuncs.sHasEgrAppObkFuncSetDefaultPriceDate = 1;
    try {
	::egr::app::mt::setDefaultPriceDate(pDate);
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppObkFuncSetDefaultPriceDate = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::mt::setDefaultPriceDate(): " + e.what);
    }

    xOiDebugFinishFunc(NULL);
}





public func xOiAppObkGetDefaultPriceDate()
{
    if (xOiFuncs.sHasEgrAppObkFuncGetDefaultPriceDate == 0) return(NULL);

    xOiDebugStartFunc("xOiFuncs::xOiAppObkGetDefaultPriceDate", NULL);

    var tRes = NULL;

    if (xOiFuncs.sHasEgrAppObkFuncGetDefaultPriceDate == 1) {
	tRes = ::egr::app::mt::getDefaultPriceDate("");
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    xOiFuncs.sHasEgrAppObkFuncGetDefaultPriceDate = 1;
    try {
	tRes = ::egr::app::mt::getDefaultPriceDate("");
    }
    catch (&e: Error) {
	xOiFuncs.sHasEgrAppObkFuncGetDefaultPriceDate = 0;
	xOiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::mt::getDefaultPriceDate(): " + e.what);
    }

    xOiDebugFinishFunc(tRes);
    return(tRes);
}
















































































public func xOiCollectProperties(pFather, pChildren, pUnion, 
				 pCheckManufacturer, pStartPos, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiCollectProperties", 
		      [pFather, pChildren, pUnion, pCheckManufacturer, 
		       pStartPos]);

    if (pChildren == NULL || !(pChildren instanceof List)) {
	xOiDebugPrint(@Warn, 1, "wrong arg pChildren");
	xOiDebugFinishFunc(0);
	return(0);
    }

    if (pChildren.empty()) {
	xOiDebugPrint(@Info, 1, "no children specified");
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tPlanning = oiGetPlanning();

    var tStartPos = pStartPos[0];
    var tAddChildren = 1;
    if (pStartPos.size() > 1)
	tAddChildren = pStartPos[1];
    var tCheckInvalidArticles = 2;
    if (pStartPos.size() > 2)
	tCheckInvalidArticles = pStartPos[2];

    
    
    var tChildren = @();
    xOiCopyAggr(pChildren, tChildren, 0);
    if (tAddChildren) {
	var tCh;
	foreach(tCh; pChildren)
	    if (tCh.hasMember(@addObjects4CommonProps)) {
		var tAddChL = 
			tCh.addObjects4CommonProps(pCheckManufacturer, NULL);
		xOiDebugPrint(@Info, 2, 
			      ["additional objects for ", tCh, ": ", tAddChL]);
		if (tAddChL != NULL) {
		    
		    
		    var i;
		    for(i=0; i < tAddChL.size(); i++) {
			var tCh = tAddChL[i];
			if (!tCh.isCat(@ACCESSORY)) tChildren.pushBack(tCh);
		    }
		    xOiDebugPrint(@Info, 2, 
				  ["filtered additional objects : ", tAddChL]);
		}
	    }
    }

    
    xOiDebugPrint(@Info, 2, 
		  ["check invalid articles? ", tCheckInvalidArticles]);
    var t2BeRemoved = @();
    var tMsgPrinted = 0;
    var tMsg = "::ofml::xoi::@NoInvalidArticlesForCommonProps";
    var tCh;
    foreach(tCh; tChildren) {
        xOiDebugPrint(@Info, 3, ["check ", tCh]);
	if (!tCh.hasMember(@isCat) || !tCh.isCat(@IF_Property)) {
	    xOiDebugPrint(@Warn, 1, [tCh, "has no Property interface!"]);
	    var tObj = tCh; 
	    t2BeRemoved.pushBack(tObj);
	}
	if (tCheckInvalidArticles && 
	    tCh.isCat(@IF_Article) && !tCh.isUp2Date()) {
	    if (tCheckInvalidArticles == 2 && !tMsgPrinted) {
		if (!xOiFuncs.sInvArticles4CommonPropsMsgPrinted) {
		    oiOutput(@MESSAGE, tMsg);
		    
		}
		else
		    xOiShowAppHint(tMsg, 1);
		tMsgPrinted = 1;
	    }
	    var tObj = tCh;
	    t2BeRemoved.pushBack(tObj);
	}
    }

    xOiDebugPrint(@Info, 2, ["to be removed: ", t2BeRemoved]);

    foreach(tCh; t2BeRemoved) tChildren.remove(tCh);

    xOiDebugPrint(@Info, 2, ["effective targets: ", tChildren]);

    if (tChildren.empty()) {
	xOiDebugPrint(@Warn, 1, "no valid children specified");
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tUnion = pUnion;

    if (pCheckManufacturer) {
	xOiDebugPrint(@Info, 1, "checking manufacturers ...");
	var tChildren2 = @();
	var tMID = NULL;
	var tDiffMan = 0;
    	var tCh;
	foreach(tCh; tChildren) {
	    xOiDebugPrint(@Info, 2, ["check: ", tCh]);
	    if (!tCh.isCat(@IF_Article)) {
		xOiDebugPrint(@Warn, 1, 
			      [tCh, " does not implement interface Article!"]);
		continue;
	    }
	    var tCh2 = tCh;
	    tChildren2.pushBack(tCh2);

	    if (tMID == NULL) tMID = xOiProgID2Manufacturer(tCh.getProgram());
	    else 
	    if (!tDiffMan && xOiProgID2Manufacturer(tCh.getProgram()) != tMID)
		tDiffMan = 1;
	}
	if (tChildren2.empty()) {
	    xOiDebugPrint(@Warn, 1, "no articles!?");
	    xOiDebugFinishFunc(0);
	    return(0);
	}
	tChildren = tChildren2;
	xOiDebugPrint(@Info, 2, ["effective targets after manufacturer check: ",
				 tChildren]);
	if (tDiffMan) {
	    xOiDebugPrint(@Warn, 1, 
			  "Selected elements belong to different manufacturers"+
			  " -> enforcing union of choice lists!");
	    tUnion = 1;
	}
    }

    var tLanguage = "de";
    if (tPlanning != NULL) 
	tLanguage = tPlanning.getLanguage();

    var tCommonProps = List();	
	    			
				
				
				
				
    var tPropObjs = Hash();	
				
				

    var tCnt = 0; 

    try {

    
    
    
    

    xOiFuncs.sCHFProp2CHPropCache = Hash();

    
    _xOiCollectProperties(pFather, tChildren, tCommonProps, tPropObjs, 
			  tPlanning, tLanguage);

    
    _xOiCollectPropValues(tCommonProps, tPropObjs, tUnion, tLanguage, 
			  tPlanning);

    xOiFuncs.sCHFProp2CHPropCache = NULL;

    var tDynProps = pFather.getDynamicProps();
    var tPPos = tStartPos;

    var tOldValues = NULL;
    if (tDynProps.hasKey(@OLD_COMMON_PVALS))
	tOldValues = tDynProps[@OLD_COMMON_PVALS];
    xOiDebugPrint(@Info, 2, ["old values: ", tOldValues]);

    if (!tCommonProps.empty()) {
	
	
	var tPName = oiGetStringResource("::ofml::xoi::@PropValUnion", 
					 tLanguage);
	var tPDef = [tPName, NULL, NULL, NULL, "b"];
	pFather.setupProperty(@__PVAL_UNION, tPDef, tPPos++);
	if (tOldValues != NULL && tOldValues.hasKey("__PVAL_UNION"))
	    tUnion = tOldValues["__PVAL_UNION"][0];
	xOiCallPropSetMethod(pFather, @__PVAL_UNION, tUnion);

	tDynProps[@COMMON_PROPS] = tPropObjs;
	tDynProps[@CHECK_PID_FOR_CP] = pCheckManufacturer;
	
	var tObj;
	var tObjects = @();
	foreach(tObj; tChildren) tObjects.pushBack(tObj.getName());
	tDynProps[@ELEMENTS_FOR_CP] = tObjects;
    }

    

    var tProcessed  = @();
    var tMultiples1 = @();
    var tMultiples2 = Hash(); 	
				
				
    var tCP;
    foreach(tCP; tCommonProps) { 
	var tPName = tCP[1];
	if (tProcessed.find(tPName) < 0)
	    tProcessed.pushBack(tPName);
	else
	    tMultiples1.pushBack(tPName);
    }
    foreach(tCP; tCommonProps) { 
	var tPName  = tCP[1];
	if (tMultiples1.find(tPName) < 0) continue;
	var tMID    = tCP[2];
	var tPType  = tCP[4];
	var tPClass = tCP[9];
	if (!tMultiples2.hasKey(tPName))
	    tMultiples2[tPName] = [tMID, tPType, tPClass];
	else {
	    if (tMultiples2[tPName][0] != NULL &&
		tMultiples2[tPName][0] != tMID)    tMID = NULL;
	    if (tMultiples2[tPName][1] != NULL &&
		tMultiples2[tPName][1] != tPType)  tPType = NULL;
	    if (tMultiples2[tPName][2] != NULL &&
		tMultiples2[tPName][2] != tPClass) tPClass = NULL;
	    tMultiples2[tPName] = [tMID, tPType, tPClass];
	}
    }

    

    var i = 0;

    for(i; i<tCommonProps.size(); i++) {
	var tCProp       = tCommonProps[i];
	var tPKey        = tCProp[0];
	var tPName       = tCProp[1];
	var tMID         = tCProp[2];
	var tObjects     = tCProp[3];
	var tPType       = tCProp[4];
	var tPMin        = tCProp[5];
	var tPMax        = tCProp[6];
	var tPVals       = tCProp[7];
	var tPVal        = tCProp[8];
	var tPClassDescr = tCProp[10];
	var tPFmt        = tCProp[11];

	if (tMultiples2.hasKey(tPName)) {
	    var tAdd = "";
	    if (tMultiples2[tPName][0] == NULL)
		tAdd += " (" + tMID.toUpper() + ")";
	    if (tMultiples2[tPName][1] == NULL)   
		tAdd += " (" + String(tPType) + ")";
	    if (tMultiples2[tPName][2] == NULL)
	        tAdd += " (" + tPClassDescr + ")";
	    tPName += tAdd;
 	}

	var tIsEditFloatChL   = (tPType == "u chf.edit");
	var tIsEditIntChL     = (tPType == "u chi.edit");

	if (tPType == "ch string" || tPType == "ch symbol")
	    tPType = "ch";
	else
	if (tPType == "u chi" || tPType == "u chf")
	    tPType = "ch";

	if (xOiPropIsCH(tPType) && tPVals == NULL) tPVals = @();

	var tState = 1;
	if (tPVals != NULL && tPVals.size() < 2 && !xOiPropIsEditable(tPType)) 
	    tState = 0;

	var tFormat = NULL;
	if (tIsEditFloatChL && (tPFmt == "@L" || tPFmt == "@L"))
	    tFormat = tPFmt;

	var tPVals2 = @();

	if (xOiPropIsCH(tPType)) {
	    var tValList = @();
	    var tPVal;
	    foreach(tPVal; tPVals) {
		tValList.pushBack([tPVal[0], tPVal[2]]);
		tPVals2.pushBack(tPVal[0]);
	    }
	    tPType = xOiBuildChoiceList2(tPType, tValList);
	}

	xOiDebugPrint(@Info, 2, ["current values: ", tPVals2]);

	if (tPType == NULL) {
	    xOiDebugPrint(@Warn, 1, 
			  ["no setting for ", tPKey, 
			   " due to unkown prop type or empty choice list"]);
	    if (pFather.hasProperty(tPKey)) {
		
		xOiDebugPrint(@Info, 1, ["removing left common property ", 
					 tPKey, " from father"]);
		pFather.removeProperty(tPKey);
	    }
	    continue;
	}

	var tPDef  = [tPName, tPMin, tPMax, tFormat, tPType];

	xOiDebugPrint(@Info, 2, 
		      ["setting up property ", tPKey, " with def ", tPDef]);
	pFather.setupProperty(tPKey, tPDef, tPPos++);
	tCnt++;

	
	
	
	
	if (tOldValues != NULL && tOldValues.hasKey(tPName)) {
	    
	    var tOldVal = tOldValues[tPName];
	    var tOldOrigVal = NULL;
	    if (tOldVal instanceof Vector) {
		tOldOrigVal = tOldVal[1];
		tOldVal = tOldVal[0];
	    }
	    var tNewOrigVal = xOiCommon2OriginalProp(pFather, tPKey, tOldVal);
	    if (tPVals == NULL || 
		(tPVals2.find(tOldVal) >= 0 && 
		 (tOldOrigVal == NULL ||
		  xOiCompareObjs(tNewOrigVal[1], tOldOrigVal)))) {
	    	xOiDebugPrint(@Info, 1, ["assign old value ", tOldVal]);
		tPVal = tOldVal;
	    }
	}
	xOiCallPropSetMethod(pFather, tPKey, tPVal);
	pFather.setPropState(tPKey, tState);
    }

    } catch(&e: Error) { xOiDebugPrint(@EvalErr, 1, e.where + ": " + e.what); }

    xOiDebugFinishFunc(tCnt);
    return(tCnt);
}





public func xOiIsCommonProp(pObj, pPKey)
{
    var tPKeyStr = String(pPKey);
    var tIsCommonPrefix = (tPKeyStr.size() > 13 && 
		           tPKeyStr.substr(0, 13) == "__COMMON_PROP");
    if (!tIsCommonPrefix) return(0);

    var tDynProps = pObj.getDynamicProps();
    if (!tDynProps.hasKey(@COMMON_PROPS)) return(0);

    var tCommonProps = tDynProps[@COMMON_PROPS];
    return(tCommonProps.hasKey(pPKey)); 
}

private func _xOiProgID2Descr(pPID)
{
    if (pPID == NULL || !(pPID instanceof Symbol)) return("");

    var tPID = String(pPID);
    var tUS = tPID.find('_');

    if (tUS > 0) {
	tPID.toUpper(0, tUS);
	tPID[tUS] = ' ';
	tPID.toUpper(tUS+1, 1);
    }

    return(tPID);
}










public func xOiRemoveCommonProps(pFather, pKeepValues)
{
    xOiDebugStartFunc("xOiFuncs::xOiRemoveCommonProps", [pFather, pKeepValues]);

    var tDynProps = pFather.getDynamicProps();
    if (tDynProps instanceof List) {
	xOiDebugPrint(@ExplWarn, 1, 
		      ["Cannot remove common props during dump of ", pFather]);
    	xOiDebugFinishFunc(NULL);
	return;
    }
    var tPKey;
    var tOldValues = Hash();
    var tPropPrefix = xOiFuncs.sCommonPropPrefix;
    foreach(tPKey; pFather.getPropertyKeys()) {
        xOiDebugPrint(@Info, 3, ["key: ", tPKey]);
	var tKeyStr = String(tPKey);
	if (tPKey == @__PVAL_UNION || 
	    (tKeyStr.size() > tPropPrefix.size() && 
	     tKeyStr.substr(0, tPropPrefix.size()) == tPropPrefix)) {
	    var tPName = pFather.getPropertyDef(tPKey)[0];
	    var tOldVal = pFather.getPropValue(tPKey);
	    xOiDebugPrint(@Info, 2, ["removing common prop ", tPKey]);
	    pFather.removeProperty(tPKey);
	    
	    if (tDynProps.hasKey(tPKey)) {
	    	xOiDebugPrint(@Info, 2, 
			     ["removing value for common prop ", tPKey]);
		tDynProps.remove(tPKey);
	    }
	    if (pKeepValues) {
		var tOrigVal = NULL;
		if (tPKey != @__PVAL_UNION) {
		    tOrigVal = xOiCommon2OriginalProp(pFather, tPKey, tOldVal);
		    if (tOrigVal != NULL)
		        tOrigVal = tOrigVal[1];
		}
		else
		    tPName = "__PVAL_UNION";
		tOldValues[tPName] = [tOldVal, tOrigVal];
	    }
	}
    }
    xOiDebugPrint(@Info, 3, ["dynamic props: ", tDynProps]);

    if (tDynProps.hasKey(@COMMON_PROPS))
	tDynProps.remove(@COMMON_PROPS);
    if (tDynProps.hasKey(@CHECK_PID_FOR_CP))
	tDynProps.remove(@CHECK_PID_FOR_CP);
    if (tDynProps.hasKey(@ELEMENTS_FOR_CP))
	tDynProps.remove(@ELEMENTS_FOR_CP);
    if (tDynProps.hasKey(@OLD_COMMON_PVALS))
	tDynProps.remove(@OLD_COMMON_PVALS);

    if (pKeepValues) {
	xOiDebugPrint(@Info, 1, 
		      "adding old values under key @OLD_COMMON_PVALS");
	tDynProps[@OLD_COMMON_PVALS] = tOldValues;
        xOiDebugPrint(@Info, 2, ["old values: ", tDynProps[@OLD_COMMON_PVALS]]);
    }

    xOiDebugFinishFunc(NULL);
}

















public func xOiCommonPropsChanged(pFather, pProps, ...)
{
    var tProps = pProps[0];
    var tObjects;
    if (pProps.size() > 1)
	tObjects = pProps[1];
    if (tObjects == NULL)
	tObjects = @();
    var tObjectsOnly = 0;
    if (pProps.size() > 2)
	tObjectsOnly = pProps[2];

    xOiDebugStartFunc("xOiFuncs::xOiCommonPropsChanged", 
		      [pFather, tProps, tObjects]);
 
    var tDynProps = pFather.getDynamicProps();
    if (!tDynProps.hasKey(@COMMON_PROPS)) {
    	xOiDebugPrint(@Info, 1, ["no common properties defined for ", pFather]);
    	xOiDebugFinishFunc(0);
    	return(0);
    }

    var tOldValues = NULL;
    if (tDynProps.hasKey(@OLD_COMMON_PVALS))
	tOldValues = tDynProps[@OLD_COMMON_PVALS];

    var tCommonProps = tDynProps[@COMMON_PROPS];
    xOiDebugPrint(@Info, 2, ["common props: ", tCommonProps]);

    var tDelegations = 0;
    var tNotAffected = 0;

    var tP;
    foreach(tP; tProps) {
	var tPVal = pFather.getPropValue(tP);
	xOiDebugPrint(@Info, 2, ["check ", tP, "=", tPVal]);

	if (tP == @__PVAL_UNION) {
	    
	    
	    var tObjects = pFather.getDynamicProps()[@ELEMENTS_FOR_CP];
	    var tChildren = @();
	    var tObj;
	    foreach(tObj; tObjects)
	        if (oiExists(tObj)) tChildren.pushBack(eval(tObj));
	    xOiDebugPrint(@Info, 3, ["elements: ", tChildren]);

	    
	    if (tOldValues != NULL && tOldValues.hasKey("__PVAL_UNION"))
		tOldValues.remove("__PVAL_UNION");

	    var tCheckProgID = pFather.getDynamicProps()[@CHECK_PID_FOR_CP];
	    xOiCollectProperties(pFather, tChildren, tPVal, tCheckProgID,
				 pFather.getPropertyPos(@__PVAL_UNION), 0);
	    if (tProps.size() > 1)
		
		
		xOiDebugPrint(@ExplWarn, 1, 
			      "It's not possible to handle change of " +
			      "property @__PVAL_UNION\ntogether with " +
			      "changes of other properties!");
	    break;
	}

	if (tPVal != NULL && tPVal instanceof Symbol && 
	    (tPVal == @VOID || tPVal == @UNSPECIFIED))
	    
	    
	    continue;

	if (!tCommonProps.hasKey(tP)) 
	    
	    continue;

	var tCPType = xOiPropType2ID(pFather.getPropertyDef(tP)[4]);
	xOiDebugPrint(@Info, 2, ["common prop type: ", tCPType]);

	var tIsEditableUD = (tCPType == "u chf.edit" || 
			     tCPType == "u chi.edit");

	var tPropObjs = tCommonProps[tP];
	var tPropObj;
	foreach(tPropObj; tPropObjs) {
	    xOiDebugPrint(@Info, 2, ["check obj props ", tPropObj]);
	    var tObjName = tPropObj[0];
	    var tObj = NULL;
	    if (!oiExists(tObjName)) {
		xOiDebugPrint(@Warn, 1, 
			      ["object ", tObjName, " does not exist!"]);
		continue;
	    }

	    tObj = eval(tObjName);
	    var tPKey = tPropObj[1];
	    var tPState = tObj.getPropState(tPKey);
	    var tPDef = tObj.getPropertyDef(tPKey);

	    if (tPDef == NULL) {
		xOiDebugPrint(@Warn, 1, ["no definition for property ", tPKey]);
		tNotAffected++;
		continue;
	    }
	    var tPType = tObj.getPropertyDef(tPKey)[4];
	    xOiDebugPrint(@Info, 2, 
			  [tObj, " prop ", tPKey, " state=", tPState]);
	    if ((tObjectsOnly && tObjects.find(tObj) < 0) ||
		(!tObjectsOnly && tObj.getPropState(tPKey) != 1 && 
		 tObjects.find(tObj) < 0)) {
		tNotAffected++;
		continue;
	    }

	    
	    
	    
	    
	    var tObjPVals = tPropObj[2];
	    if (!tIsEditableUD &&
		tObjPVals != NULL && tObjPVals instanceof List) {
		var tObjPVal;
		var tAssigned = 0;
		foreach(tObjPVal; tObjPVals) {
	    	    xOiDebugPrint(@Info, 2, ["check obj val ", tObjPVal[0], 
					     " vs. ", tPVal]);
		    if (tObjPVal[0] == tPVal) {
			var tNewVal = tObjPVal[1]; 
			if (xOiPropIsCHF(tPType))
			    tPType = xOiCHFProp2CHProp(tObj, tPType);
			var tPVals = xOiGetValuesFromChoiceList(tPType);
			if (tPVals == NULL || tPVals.find(tNewVal) < 0)
			    
			    tNewVal = tObjPVal[0];
			var tOk = xOiCheckPropertyValue(tObj, tPKey, tNewVal);
			if (tOk) {
	    	    	    xOiDebugPrint(@Info, 2, ["assign value ", 
						     tNewVal, " to obj"]);
	    		    tDelegations++;
			    tAssigned = 1;
			    if (tOk == 1) tObj.setPropValue(tPKey, tNewVal);
			}
			break;
		    }
		}
		if (!tAssigned) tNotAffected++;
	    }
	    else {
		var tOk = xOiCheckPropertyValue(tObj, tPKey, tPVal);
		if (tOk) {
	    	    xOiDebugPrint(@Info, 2, 
			          ["assign value ", tPVal, " to obj"]);
	    	    tDelegations++;
		    if (tOk == 1) tObj.setPropValue(tPKey, tPVal);
		}
		else
		    tNotAffected++;
	    }
	}
    }

    xOiDebugPrint(@Info, 2, ["not effected objects: ", tNotAffected]);
    if (tNotAffected) 
	xOiShowAppHint("::ofml::xoi::@CommonPropChangeNot4All", 1);

    xOiDebugFinishFunc(tDelegations);
    return(tDelegations);
}















public func xOiCheckCommonPropsPaste(pFather)
{
    xOiDebugStartFunc("xOiFuncs::xOiCheckCommonPropsPaste", pFather);
 
    if (!pFather.getDynamicProps().hasKey(@ELEMENTS_FOR_CP)) {
	xOiDebugPrint(@Info, 1, "obj has no common properties");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tCommonEL = pFather.getDynamicProps()[@ELEMENTS_FOR_CP];
    var t1stEl = tCommonEL.front();
    var t1stDot = t1stEl.find('.');
    var t2ndDot = t1stEl.find('.', t1stDot+1);
    var tOrigName = t1stEl.substr(0, t2ndDot);
    var tFatherName = pFather.getName();
    if (tOrigName == tFatherName) {
	xOiDebugPrint(@Info, 1, "same father");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    
    var tNewL = @();
    var tCommonEl;
    foreach(tCommonEl; tCommonEL) {
	tCommonEl.replace(0, tOrigName.size(), tFatherName);
	tNewL.pushBack(tCommonEl);
    }
    pFather.getDynamicProps()[@ELEMENTS_FOR_CP] = tNewL;
    xOiDebugPrint(@Info, 2, ["new el names: ", 
			     pFather.getDynamicProps()[@ELEMENTS_FOR_CP]]);

    

    var tP;
    var tCommonProps = pFather.getDynamicProps()[@COMMON_PROPS];
    foreach(tP; tCommonProps.keys()) {
	var tNewSpecs = @();
	var tObjSpec;
	foreach(tObjSpec; tCommonProps[tP]) {
	    var tObjName = tObjSpec[0];
	    tObjName.replace(0, tOrigName.size(), tFatherName);
	    tNewSpecs.pushBack([tObjName, tObjSpec[1], tObjSpec[2]]);
	}
	tCommonProps[tP] = tNewSpecs;
    }

    xOiDebugFinishFunc(tOrigName);
    return(tOrigName);
}




















public func xOiPropChanges2Parts(pObj, pProps, pCommonPropsOnly, ...)
{
    var tCommonPropsOnly = pCommonPropsOnly[0];
    var tIgnStateObjs;
    if (pCommonPropsOnly.size() > 1)
	tIgnStateObjs = pCommonPropsOnly[1];
    if (tIgnStateObjs == NULL)
	tIgnStateObjs = @();

    xOiDebugStartFunc("xOiFuncs::xOiPropChanges2Parts", 
		      [pObj, pProps, tCommonPropsOnly, tIgnStateObjs]);
 
    var tPID = pObj.getProgram();
    var tDelegations = 0;
    var tProps = pProps;

    if (tCommonPropsOnly) {
        var tProgInfo = pObj.getPlanning().getInfo(tPID);;
	if (tProgInfo == NULL || 
	    (!tProgInfo.hasMember(@isCommonProperty) &&
	     !tProgInfo.hasMember(@isCommonProperty2))) {
	    xOiDebugPrint(@Info, 1, 
			  ["no prog info for ", tPID, 
			   " or method isCommonProperty()!"]);
	    xOiDebugFinishFunc(tDelegations);
	    return(tDelegations);
	}
	var tUseFunc2 = tProgInfo.hasMember(@isCommonProperty2);
	tProps = @();
	var tProp;
	foreach(tProp; pProps)
	    if ((!tUseFunc2 && tProgInfo.isCommonProperty(tProp)) ||
		(tUseFunc2 && tProgInfo.isCommonProperty2(pObj, tProp)))
		tProps.pushBack(tProp);
    }
    if (tProps.empty()) {
	xOiDebugPrint(@Info, 1, "no properties for delegation");
	xOiDebugFinishFunc(tDelegations);
	return(tDelegations);
    }
    xOiDebugPrint(@Info, 2, ["check props: ", tProps]);

    var tE;
    foreach(tE; pObj.getElements()) {
	if (!(tE.isA(OiPlElement) || tE.isA(OiPart)) || tPID != tE.getProgram())
	    continue;
	var tProp;
        xOiDebugPrint(@Info, 2, ["check ", tE]);
	foreach(tProp; tProps) {
	    if (tE.hasProperty(tProp) &&
		(tE.getPropState(tProp) || tIgnStateObjs.find(tE) >= 0)) {
        	xOiDebugPrint(@Info, 1, 
			      ["delegate change of ", tProp, " to ", tE]);
	    	tE.setPropValue(tProp, pObj.getPropValue(tProp));
	    	tDelegations++;
	    }
	}
	tDelegations += xOiPropChanges2Parts(tE, pProps, tCommonPropsOnly);
    }

    xOiDebugFinishFunc(tDelegations);
    return(tDelegations);
}














public func xOiCommon2OriginalProp(pObj, pProp, pPVal)
{
    xOiDebugStartFunc("xOiFuncs::xOiCommon2OriginalProp", [pObj, pProp, pPVal]);
 
    var tDynProps = pObj.getDynamicProps();
    if (!tDynProps.hasKey(@COMMON_PROPS)) {
    	xOiDebugPrint(@Info, 1, ["no common properties defined for ", pObj]);
    	xOiDebugFinishFunc(NULL);
    	return(NULL);
    }

    var tCommonProps = pObj.getDynamicProps()[@COMMON_PROPS];
    if (!tCommonProps.hasKey(pProp)) {
    	xOiDebugPrint(@Info, 1, [pProp, " is not a common property"]);
    	xOiDebugFinishFunc(NULL);
    	return(NULL);
    }

    var tRes = Vector(2);
    var tCommonPropInfo = tCommonProps[pProp];
    xOiDebugPrint(@Info, 2, ["common prop info: ", tCommonPropInfo]);

    var tPropPrefix = xOiFuncs.sCommonPropPrefix;
    var t1stObj = NULL;
    var tOrigPKey;
    var tP;
    foreach(tP; tCommonPropInfo) {
	t1stObj = tP[0];
	tOrigPKey = tP[1];
	var tPKeyStr = String(tOrigPKey);
	if (tPKeyStr.size() <= tPropPrefix.size() ||
	    tPKeyStr.substr(0, tPropPrefix.size()) != tPropPrefix) {
	    
	    
	    if (pPVal == NULL) {
		tRes[0] = tOrigPKey;
		break;
	    }
	    var tPValues = tP[2];
	    if (tPValues == NULL) {
		
		
	    	tRes[0] = tOrigPKey;
		tRes[1] = pPVal;
		break;
	    }
	    var tPVal;
	    foreach(tPVal; tPValues)
	    	
		
	    	if (xOiCompareObjs(tPVal[0], pPVal)) {
	    	    tRes[0] = tOrigPKey;
		    tRes[1] = tPVal[1];
		    break;
		}
	}
    }

    if (tRes[0] == NULL && t1stObj != NULL && oiExists(t1stObj)) {
	
	
	
	var tRes2 = xOiCommon2OriginalProp(eval(t1stObj), tOrigPKey, pPVal);
	if (tRes2 != NULL) tRes = tRes2;
    }

    xOiDebugFinishFunc(tRes);
    return(tRes);
}

private func _xOiCollectProperties(pFather, pChildren, pCommonProps, pPropObjs,
				   pPlanning, pLanguage)
{
    var tCheckFather = (pFather.hasMember(@isChildPropValid4Common));
    var tPCount = 0;
    var tCh;
    foreach(tCh; pChildren) {
	var tPKeys = tCh.getPropertyKeys();
	xOiDebugPrint(@Info, 2, ["check ", tCh, ", prop keys: ", tPKeys]);
	var tCommonProps = Hash();
        if (tCh.getDynamicProps().hasKey(@COMMON_PROPS))
    	    tCommonProps = tCh.getDynamicProps()[@COMMON_PROPS];
	var tPID = tCh.getProgram();
	var tMID = xOiProgID2Manufacturer(tPID);
	var tPI = NULL;
	if (pPlanning != NULL)
	    tPI = pPlanning.getInfo(tPID);
	xOiDebugPrint(@Info, 2, ["planning ", pPlanning, ", prog info ", tPI]);
	var tCheckPI = (tPI != NULL && 
			(tPI.hasMember(@isCommonProperty) ||
			 tPI.hasMember(@isCommonProperty2)));
	xOiDebugPrint(@Info, 2, ["check prog info: ", tCheckPI]);
	var tUseFunc2 = (tPI != NULL && tPI.hasMember(@isCommonProperty2));
	var tIgnorePClass = (tPI != NULL && 
			     tPI.hasMember(@ignorePClass4CommonProps) &&
			     tPI.ignorePClass4CommonProps(tCh));

	var tPClasses = xOiList2HashX(tCh.getPropClassDescriptions(pLanguage));

	var tPKey;
	foreach(tPKey; tPKeys) {
	    var tPKey2 = tPKey;
	    var tState = tCh.getPropState(tPKey2);
	    var tPDef = tCh.getPropertyDef(tPKey2);
	    var tPTID = xOiPropType2ID(tPDef[4]);
	    var tPFmt = tPDef[3];

	    if (xOiPropIsMO(tPTID)) {
		xOiDebugPrint(@Info, 2, 
			      ["ignoring multi option property ", tPKey2]);
		continue;
	    }

	    if (tState == -1) {
		xOiDebugPrint(@Info, 2, 
			      ["ignoring invisible property ", tPKey2,
			       " of ", tCh]);
		continue;
	    }

	    
	    if (tCommonProps.hasKey(tPKey)) {
		tPKey2 = tCommonProps[tPKey].front()[1];
		xOiDebugPrint(@Info, 2, 
			      ["checking ", tPKey2, " instead of ", tPKey, 
			       " for common prop"]);
	    }
	    if (!tCheckPI || 
		(!tUseFunc2 && !tPI.isCommonProperty(tPKey2)) ||
		(tUseFunc2 && !tPI.isCommonProperty2(tCh, tPKey2))) {
		xOiDebugPrint(@Info, 2, 
			      ["property ", tPKey2, " not common in ", tPID]);
		continue;
	    }

	    if (tCheckFather && !pFather.isChildPropValid4Common(tCh,tPKey2)) {
		xOiDebugPrint(@Info, 2, ["father says no for prop ", tPKey2, 
				         " of ", tCh]);
		continue;
	    }

	    var tPClass = tCh.getPropClass(tPKey2);
	    var tPClassDescr = "NULL";
	    if (tPClass != NULL && tPClasses != NULL && 
		tPClasses.hasKey(tPClass))
		tPClassDescr = tPClasses[tPClass][0];
	    if (tPClass == NULL)
		tPClass = "NULL";

	    if (xOiPropIsCHF(tPTID))
		tPTID = "ch";
		
	        
	    xOiDebugPrint(@Info, 3, [tPKey, ": ", tPDef, " type: ", tPTID]);

	    if (tPTID == "ch")
		
		tPTID += " " + xOiChPropSubType(tCh, tPDef[4]);

	    var tPName = tPDef[0];
	    var tAtPos;
	    if (!tPName.empty() && (tAtPos = tPName.find('@')) >= 0)
		if (tAtPos)
		    
		    tPName = oiGetStringResource(tPName, pLanguage);
		else 
		    tPName = oiGetStringResource(tPName, pLanguage, tCh);

	    
	    
	    
	    var tPinProps = NULL;
	    var tCP;
	    foreach(tCP; pCommonProps)
		if (tCP[1] == tPName && tCP[2] == tMID && tCP[4] == tPTID  && 
		    (tCP[10] == tPClassDescr || 
		     (tIgnorePClass || tCP[9] == tPClass))) {
		    tPinProps = tCP;
		    break;
		}
	    if (tPinProps == NULL) {
		tPinProps = Vector(12);
		tPinProps[0]  = Symbol(xOiFuncs.sCommonPropPrefix +
				       String(++tPCount));
		tPinProps[1]  = tPName;
		tPinProps[2]  = tMID;
		tPinProps[3]  = List(); 
		tPinProps[4]  = tPTID;
		tPinProps[9]  = tPClass;
		tPinProps[10] = tPClassDescr;
		tPinProps[11] = tPFmt;
		pCommonProps.pushBack(tPinProps);
		xOiDebugPrint(@Info, 3, ["added common prop: ", tPinProps]);
	    }

	    
	    var tCPKKey  = tPinProps[0];
	    var tObjName = tCh.getName();
	    if (!pPropObjs.hasKey(tCPKKey)) pPropObjs[tCPKKey] = @();
	    var tObjProps = pPropObjs[tCPKKey];
	    var tObjFound = 0;
	    var tObjProp;
	    foreach(tObjProp; tObjProps)
		if (tObjProp[0] == tObjName) {
		    tObjFound = 1;
		    break;
		}
	    if (tObjFound) 
		xOiDebugPrint(@ExplWarn, 1, 
			      ["ignoring multiple property '", tPName, 
			       "' of ", tCh, " for common prop ", tCPKKey]);
	    else {
	        tPinProps[3].pushBack(tCh);
	        tObjProp = Vector(3);
	    	tObjProp[0] = tObjName;
	    	tObjProp[1] = tPKey; 	
	    	tObjProp[2] = NULL;	
	    	tObjProps.pushBack(tObjProp);
	    	xOiDebugPrint(@Info, 2, ["added obj prop: ", tObjProp]);
	    }
	}
    }
    xOiDebugPrint(@Info, 2, ["common props (1.step): ", pCommonProps]);
    xOiDebugPrint(@Info, 2, ["prop objects (1.step): ", pPropObjs]);
}

private func _xOiCollectPropValues(pCommonProps, pPropObjs, pUnion, pLanguage,
				   pPlanning)
{
    xOiDebugStartFunc2("xOiFuncs::_xOiCollectPropValues", [pUnion, pLanguage]);
    var tP;
    foreach(tP; pCommonProps) {
	var tCommonPKey = tP[0];
	xOiDebugPrint(@Info, 2, ["processing ", tCommonPKey]);
	var tObjects = tP[3];
	var t1stObj = 1;
	var tObj;
	foreach(tObj; tObjects) {
	    var tObjProps = pPropObjs[tCommonPKey];

	    xOiDebugPrint(@Info, 2, ["object props: ", tObjProps]);

	    
	    var tObjProp, tPVal;
	    foreach(tObjProp; tObjProps)
		if (tObjProp[0] == tObj.getName()) {
	    	    tPVal = xOiCombinePropDefs(tP, tObj, tObjProp, 
				               (t1stObj || pUnion), 
					       pLanguage, pPlanning);
		    break;
		}
	    xOiDebugPrint(@Info, 3, ["combined defs: ", tP]);

	    
	    var tPVals = tP[7];
	    if (tPVals != NULL) {
		
		var tPV;
		foreach (tPV; tPVals)
		    if (tPV[1] == tPVal) {
			tPVal = tPV[0];
			break;
		    }
	    }
	    if (t1stObj) {
		tP[8] = tPVal;
		t1stObj = 0;
	    }
	    else
	        
	        if (!xOiCompareObjs(tP[8], tPVal)) tP[8] = @UNDEF;
	}
    }

    if (!pUnion) {
	
	xOiDebugPrint(@Info, 3, 
		      ["no union -> removing not common values using\n" +
		       "common props: ", pCommonProps, " and\n" +
		       "prop objects: ", pPropObjs]);
        var tP;
        foreach(tP; pCommonProps) {
	    var tCommonPKey  = tP[0];
	    var tCommonPVals = tP[7];
	    var tObjects     = tP[3];

	    if (tCommonPVals == NULL) 
		
		continue;

	    var tCommonVals = @();
	    var tCommonP;
	    foreach(tCommonP; tCommonPVals) tCommonVals.pushBack(tCommonP[0]);

	    var tObj;
	    foreach(tObj; tObjects) {
	    	var tObjProps = pPropObjs[tCommonPKey];
	    	var tObjProp;
	    	foreach(tObjProp; tObjProps) {
		    var tObjPVals = tObjProp[2];
		    var tRmOPVals = @();
		    var tObjPVal;
		    foreach(tObjPVal; tObjPVals)
		    	if (tCommonVals.find(tObjPVal[0]) < 0)
			    tRmOPVals.pushBack(tObjPVal);
		    foreach(tObjPVal; tRmOPVals) tObjPVals.remove(tObjPVal);
		}
	    }
	}
    }

    xOiDebugPrint(@Info, 2, ["common props (2.step): ", pCommonProps]);
    xOiDebugPrint(@Info, 2, ["prop objects (2.step): ", pPropObjs]);

    xOiDebugFinishFunc(NULL);
}













public func xOiGetCommonPropInfo(pObj, pEL, pPropKey, pPropValue, pInfoType)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetCommonPropInfo", 
		      [pObj, pEL, pPropKey, pPropValue, pInfoType]);

    var tInfo = NULL;

    if (!xOiIsCommonProp(pObj, pPropKey) || 
	(pPropValue != NULL && pPropValue instanceof Symbol && 
	 pPropValue == @UNDEF)) {
        xOiDebugFinishFunc(tInfo);
        return(tInfo);
    }

    var tOrigKeys = xOiCommon2OriginalProp(pObj, pPropKey, pPropValue);

    var tPropKey, tPropValue;
    if (tOrigKeys != NULL) { 
	tPropKey   = tOrigKeys[0];
	tPropValue = tOrigKeys[1];
    }

    if (tPropKey == NULL && tPropValue == NULL) {
	
	xOiDebugPrint(@Warn, 1, "no original keys found (wrong element list?)");
        xOiDebugFinishFunc(tInfo);
        return(tInfo);
    }

    var tPlanning = pObj.getPlanning();
    var tPI = NULL;
    var tE;
    foreach(tE; pEL) {
	xOiDebugPrint(@Info, 3, tE);
	if (!tE.isCat(@IF_Article) || tE.isA(xOiFreePlElement) ||
	    !tE.hasProperty(tPropKey)) continue;
	tPI  = tPlanning.getInfo(tE.getProgram());
	if (tPI != NULL) break;
    }
    if (tPI != NULL) {
        xOiDebugPrint(@Info, 1, ["delegating to prog info ", tPI, " for ", tE]);
	tInfo = tPI.getPropInfo4Obj(tE, tPropKey, tPropValue, pInfoType);
    }

    xOiDebugFinishFunc(tInfo);
    return(tInfo);
}










































































public func xOiInitConfigTraversal(pObj, pExclProps, pExclMode)
{
    xOiDebugStartFunc("xOiFuncs::xOiInitConfigTraversal", 
		      [pObj, pExclProps, pExclMode]);

    if (xOiFuncs.sConfigTraversalContext != NULL) {
	xOiDebugPrint(@Warn, 1, "there is an ongoing configuration traversal!");
	if (xOiFuncs.sConfigTraversalContext[3].empty()) 
	    xOiDebugPrint(@Warn, 1, "assuming missing call of " +
				    "xOiFinishConfigTraversal()!");
	else {
	    xOiDebugFinishFunc(0);
	    return(0);
	}
    }

    var tPDM = pObj.getPDManager();
    if (tPDM == NULL || !tPDM.isA(xOiPDManager)) {
	
	xOiDebugPrint(@Warn, 1, "no xOiPDManager!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    
    if (pExclProps != NULL &&
	!(pExclProps instanceof List || pExclProps instanceof Vector)) {
	xOiDebugPrint(@Warn, 1, "invalid parameter pExclProps!");
	xOiDebugFinishFunc(0);
	return(0);
    }
    var tValidModes = @(@explicit, @exclude);
    if (pExclMode != NULL && tValidModes.find(pExclMode) < 0) {
	xOiDebugPrint(@Warn, 1, "invalid parameter pExclMode!");
	xOiDebugFinishFunc(0);
	return(0);
    }
    var tExclMode = pExclMode;
    if (pExclMode == NULL)
	tExclMode = @explicit;

    
    
    var tExclProps = NULL;
    if (pExclProps != NULL) {
	tExclProps = Hash(); 
	var tP;
	foreach(tP; pExclProps) {
	    var tPName = tP;
	    var tPVals = NULL;
	    if (tP instanceof Vector) {
		if (tExclMode == @exclude) {
		    xOiDebugPrint(@Warn, 1, ["ignoring pExclProps element ", 
					     tP, " in mode @exclude"]);
		    continue;
		}
		tPName = tP[0];
		tPVals = tP[1];
	    }
	    if (tPName instanceof Symbol) {
		if (tPVals == NULL)
		    tPName = tPDM.propKey2PropName(pObj, tPName);
		else {
		    var tPVals2 = Vector(0); 
		    xOiCopyAggr(tPVals, tPVals2, 0);
		    var tNativeKeys = 
			tPDM.getNativePropKeys(pObj, tPName, tPVals2);
		    if (tNativeKeys == NULL) {
	    	        xOiDebugPrint(@Warn, 1, 
			              ["no mapping to product data for ", tP, 
			   	       " -> using Symbol 2 String conversion"]);
	    	        tPName = String(tPName);
		    }
		    else {
		        tPName = tNativeKeys[1];
		        tPVals = @();
		        xOiCopyAggr(tNativeKeys[2], tPVals, 0); 
		    }
		}
	    }
	    if (tPName instanceof String)
		tExclProps[tPName] = tPVals;
	    else
		xOiDebugPrint(@Warn, 1, 
			      ["ignoring invalid pExclProps element ", tP]);
	}
    }

    xOiDebugPrint(@Info, 2, ["exclusive props: ", tExclProps]);

    var tPStack = @(); 	
			

    var tPropList = pObj.getPropertyKeys(); 

    if (!_xOiPushNextProp2ConfigStack(tPDM, pObj, tExclProps, tExclMode, 
				      tPStack, tPropList)) {
	xOiDebugPrint(@Warn, 1, "no properties for configuration!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    xOiFuncs.sConfigTraversalContext = [pObj, tExclProps, tExclMode, 
					tPStack, tPropList];

    xOiDebugFinishFunc(1);
    return(1);
}


















public func xOiSetupNextConfiguration(pArg)
{
    xOiDebugStartFunc("xOiFuncs::xOiSetupNextConfiguration", pArg);

    if (xOiFuncs.sConfigTraversalContext == NULL) {
	xOiDebugPrint(@Warn, 1, "configuration traversal not initialized!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tObj       = xOiFuncs.sConfigTraversalContext[0];
    var tExclProps = xOiFuncs.sConfigTraversalContext[1];
    var tExclMode  = xOiFuncs.sConfigTraversalContext[2];
    var tPropStack = xOiFuncs.sConfigTraversalContext[3];
    var tPropList  = xOiFuncs.sConfigTraversalContext[4];

    if (tPropStack.empty()) {
	xOiDebugPrint(@Warn, 1, "configuration traversal already completed!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tPDM = tObj.getPDManager();
    if (tPDM == NULL || !tPDM.isA(xOiPDManager)) {
	
	xOiDebugPrint(@Warn, 1, "no xOiPDManager!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    while (1) {
        xOiDebugPrint(@Info, 2, ["current stack: ", tPropStack]);
	var tTopProp  = tPropStack.back();
	var tProp     = tTopProp[0];
	var tValues   = tTopProp[1];
	var tNextVIdx = tTopProp[2] + 1;

	if (tNextVIdx == tValues.size()) {
	    
	    tPropStack.popBack();
	    if (tPropStack.empty()) {
		
		xOiDebugFinishFunc(0);
		return(0);
	    }
	    continue;
	}

	tTopProp[2] = tNextVIdx;
	xOiDebugPrint(@Info, 1, ["setPropValue(", tProp, ",", 
				 tValues[tNextVIdx],") ..."]);
	tObj.setPropValue(tProp, tValues[tNextVIdx]);
	xOiDebugPrint(@Info, 1, "... done.");

	tPropList = _xOiUpdatePropList(tObj, tProp, tPropList);

	var tPushed = _xOiPushNextProp2ConfigStack(tPDM, tObj, tExclProps, 
					      	   tExclMode, tPropStack,
						   tPropList);
	if (!tPushed) {
	    
	    
	    
	    if (tExclProps == NULL || tExclMode != @explicit ||
	        _xOiCheckConfiguration(tPDM, tObj, tExclProps))
	        break;
	}
    }

    xOiFuncs.sConfigTraversalContext[4] = tPropList;

    xOiDebugFinishFunc(1);
    return(1);
}


private func _xOiCheckConfiguration(pPDM, pObj, pExclProps)
{
    xOiDebugStartFunc("xOiFuncs::_xOiCheckConfiguration", pObj);

    xOiDebugPrint(@Info, 3, ["exclusive props: ", pExclProps]);

    var tObj = pObj;
    if (pObj.hasMember(@isCat) && pObj.isCat(@IF_Article))
	tObj = pObj.getArticleObj();

    xOiDebugPrint(@Info, 2, ["effective ", tObj]);

    if (tObj == NULL || !tObj.hasMember(@isCat) || !tObj.isCat(@IF_Article)) {
	xOiDebugPrint(@Warn, 1, "invalid object!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tPName;
    foreach(tPName; pExclProps.keys()) {
	var tAllowedVals = pExclProps[tPName];
	if (tAllowedVals == NULL) continue;

        xOiDebugPrint(@Info, 2, ["check ", tPName, " -> ", tAllowedVals]);

	var tCheckNative = 1;
	var tPKey = pPDM.propName2PropKey(tObj, NULL, tPName)[0];
	if (tPKey == NULL)  {
	    xOiDebugPrint(Info, 1, "non PD property");
            tObj = pObj;
	    tPKey = Symbol(tPName);
	    tCheckNative = 0;
	}

	if (!tObj.hasProperty(tPKey)) {
	    xOiDebugPrint(@Info, 2, ["currently invalid property ", tPKey]);
	    continue;
	}

	var tCurrPVal = tObj.getPropValue(tPKey);
        xOiDebugPrint(@Info, 2, [tPKey, " = ", tCurrPVal]);

	if (tCheckNative) {
	    var tNKs = pPDM.getNativePropKeys(tObj, tPKey, tCurrPVal);
	    if (tNKs == NULL)
		
		xOiDebugPrint(@Warn, 1, "could not determine native value");
	    else
	        tCurrPVal = tNKs[2];
            xOiDebugPrint(@Info, 2, ["native val: ", tCurrPVal]);
	}
	if (tAllowedVals.find(tCurrPVal) < 0) {
	    xOiDebugFinishFunc(0);
	    return(0);
	}
    }

    xOiDebugFinishFunc(1);
    return(1);
}











private func _xOiUpdatePropList(pObj, pChangedProp, pOldList)
{
    xOiDebugStartFunc("xOiFuncs::_xOiUpdatePropList", 
		      [pObj, pChangedProp, pOldList]);

    var tPropList = @();
    var tNewList = pObj.getPropertyKeys();
    var tChangedIdx = tNewList.find(pChangedProp);

    xOiDebugPrint(@Info, 2, 
		  ["index of changed prop in new list: ", tChangedIdx]);

    if (tChangedIdx < 0) {
	
	xOiDebugPrint(@Warn, 1, "internal error in _xOiUpdatePropList()!");
	xOiCopyAggr(pOldList, tPropList, 0); 
	xOiDebugFinishFunc(tPropList);
	return(tPropList);
    }

    var tP, tPred;

    try {

    
    
    foreach(tP; pOldList) {
	var tPKey = tP;
	if (tNewList.find(tP) < 0)
	    xOiDebugPrint(@Info, 1, ["removed property ", tP]);
	else {
	    xOiDebugPrint(@Info, 2, ["restoring existing property ", tP]);
	    tPropList.pushBack(tPKey);
	}
    }

    
    var tIdx = 0;
    foreach(tP; tNewList) {
	var tPKey = tP;
	if (tPropList.find(tP) < 0) {
	    xOiDebugPrint(@Info, 2, ["check new property ", tP]);
	    if (tIdx < tChangedIdx) 
		tPropList.pushBack(tPKey);
	    else {
		
		if (tPred == NULL) 
		    xOiDebugPrint(@Warn, 1, ["no predecessor for ", tP, "!?"]);
		else {
		    var tPredIdx = tPropList.find(tPred);
		    if (tPredIdx < 0) 
			xOiDebugPrint(@Warn, 1, ["predecessor ", tPred, 
						 "not yet in tPropList !?"]);
		    else
			tPropList.insert(tPredIdx+1, 1, tPKey);
		}
	    }
	}
	tPred = tPKey;
	tIdx++;
    }

    } catch(&e: Error) {
	xOiDebugPrint(@Warn, 1, e.where + ": " + e.what);
    }

    xOiDebugFinishFunc(tPropList);
    return(tPropList);
}

private func _xOiPushNextProp2ConfigStack(pPDM, pObj, pExclProps, pExclMode, 
					  pPropStack, pPropList)
{
    xOiDebugStartFunc("xOiFuncs::_xOiPushNextProp2ConfigStack", 
		       [pObj, pExclMode]);

    var tLastProp = NULL;
    if (!pPropStack.empty())
	tLastProp = pPropStack.back()[0];

    xOiDebugPrint(@Info, 2, ["exclusive props: ", pExclProps]);
    xOiDebugPrint(@Info, 2, ["(effective prop list: ", pPropList]);

    var tNextProp = tLastProp; 
    var tPType, tPVals = @();
    var tOk = 0;

    while (!tOk) {

        tNextProp = xOiGetNextProperty(pObj, tNextProp, @active, pPropList);

        xOiDebugPrint(@Info, 2, ["next property: ", tNextProp]);

        if (tNextProp == NULL) {
            xOiDebugFinishFunc(0);
            return(0);
        }

	var tPDef = pObj.getPropertyDef(tNextProp);
        xOiDebugPrint(@Info, 2, ["prop def: ", tPDef]);

	if (tPDef == NULL || tPDef[4] == NULL) {
	    
	    xOiDebugPrint(@Warn, 1, ["no property type for ", tNextProp]);
	    continue; 
	}
	tPType = tPDef[4];
	if (!xOiPropIsCH(tPType)) {
	    xOiDebugPrint(@Info, 1, 
			  ["ignoring non-choicelist property ", tNextProp]);
	    continue;
	}
        if (xOiPropIsCHF(tPType))
	    tPType = xOiCHFProp2CHProp(pObj, tPType);

        var tChLVals = xOiGetValuesFromChoiceList(tPType);

        xOiDebugPrint(@Info, 2, ["choice list vals: ", tChLVals]);

	if (tChLVals.empty()) {
	    xOiDebugPrint(@Info, 1, ["ignoring property ", tNextProp, 
				     " with empty choicelist"]);
	    continue;
	}

	var tChLVals2 = Vector(0); 
	xOiCopyAggr(tChLVals, tChLVals2, 0);
	var tNativeKeys = pPDM.getNativePropKeys(pObj, tNextProp, tChLVals2);
	var tPropName, tAllVals;
	if (tNativeKeys == NULL) {
	    xOiDebugPrint(@Info, 2, 
			  ["no mapping to product data for ", tNextProp, 
			   " -> using Symbol 2 String conversion"]);
	    tPropName = String(tNextProp);
	    tAllVals  = tChLVals;
	}
	else {
	    tPropName = tNativeKeys[1];
	    tAllVals  = tNativeKeys[2];
	}
        if (pExclProps != NULL &&
	    ((pExclMode == @explicit && !pExclProps.hasKey(tPropName)) ||
	     (pExclMode == @exclude  && pExclProps.hasKey(tPropName)))) {
	    xOiDebugPrint(@Info, 1, ["excluding property ", tNextProp]);
	    continue;
        }
	tOk = 1;

	var tUsableVals = NULL;
	if (pExclProps != NULL && pExclMode == @explicit)
	    tUsableVals = pExclProps[tPropName];

        xOiDebugPrint(@Info, 2, ["all vals: ", tAllVals]);
        xOiDebugPrint(@Info, 2, ["usable vals: ", tUsableVals]);

	if (tUsableVals == NULL)
	    xOiCopyAggr(tChLVals, tPVals, 0);
	else {
            
	    var i;
	    for(i = 0; i < tChLVals.size(); i++) {
		var tChLVal = tChLVals[i];
		var tNVal   = tAllVals[i]; 
		if (tUsableVals.find(tNVal) >= 0)
		    tPVals.pushBack(tChLVal);
	    }
	}
    }

    var tPInfo = Vector(3);

    tPInfo[0] = tNextProp;
    tPInfo[1] = tPVals;
    tPInfo[2] = -1;

    pPropStack.pushBack(tPInfo);

    xOiDebugPrint(@Info, 2, ["new top of stack: ", tPInfo]);

    xOiDebugFinishFunc(1);
    return(1);
}










public func xOiFinishConfigTraversal(pArg)
{
    xOiDebugStartFunc("xOiFuncs::xOiFinishConfigTraversal", pArg);

    xOiFuncs.sConfigTraversalContext = NULL;

    xOiDebugFinishFunc(NULL);
    return(NULL);
}

























public func xOiProcessConfigurations(pObj, pAction, pExclProps, pExclMode)
{
    xOiDebugStartFunc("xOiFuncs::xOiProcessConfigurations", 
		      [pObj, pAction, pExclProps, pExclMode]);

    if (pAction == NULL || !pAction.hasMember(@processConfiguration)) {
	xOiDebugPrint(@Warn, 1, "invalid action object!");
        xOiDebugFinishFunc(0);
        return(0);
    }

    if (!xOiInitConfigTraversal(pObj, pExclProps, pExclMode)) {
	xOiDebugPrint(@Warn, 1, 
		      "configuration traversal could not be started!");
        xOiDebugFinishFunc(0);
        return(0);
    }

    while (xOiSetupNextConfiguration(NULL))
	pAction.processConfiguration(pObj);

    xOiFinishConfigTraversal(NULL);

    xOiDebugFinishFunc(1);
    return(1);
}


































public func xOiGetSubArticleIDs(pObj, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetSubArticleIDs", pObj);

    var tObj = pObj[0];
    var tCallArtObj = 0;
    if (pObj.size() > 1 && pObj[1] instanceof Int)
	tCallArtObj = pObj[1];

    var tRet = xOiGetSubArticleIDs2(tObj, NULL, NULL, tCallArtObj);

    xOiDebugFinishFunc(tRet);
    return(tRet);
}




































public func xOiGetSubArticleIDs2(pObj, pElements, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetSubArticleIDs2", [pObj, pElements]);

    var tElements = pElements[0];
    var tInvalidArticles = @();
    if (pElements.size() > 1 && pElements[1] != NULL) {
	if (pElements[1] instanceof Vector || pElements[1] instanceof List)
	    tInvalidArticles = pElements[1];
	else
	    xOiDebugPrint(@Warn, 1, 
			  "nedd a Vector or List for 1st optional parameter!");
    }
    var tCallArtObj = 1;
    if (pElements.size() > 2 && pElements[2] instanceof Int)
	tCallArtObj = pElements[2];

    var tRet = Vector(0);

    if (!pObj.isCat(@IF_CompositeArticle)) {
	xOiDebugPrint(@Warn, 1, "given instance is not a composite article!");
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }
    var tSelfLen = pObj.getName().size();

    if (!(tElements instanceof Vector || tElements instanceof List)) {
	xOiDebugPrint(@Info, 1, "no element list given -> using getElements()");
	tElements = pObj.getElements();
    }

    var tEl;
    foreach(tEl; tElements) {
	if (!tEl.hasMember(@isCat) || !tEl.isCat(@IF_Article))
	    continue;
	var tArtObj = tEl;
	if (tCallArtObj)
	    tArtObj = tEl.getArticleObj();
	var tArticle = tArtObj.getArticleSpec();
	if (tArtObj.getArticleSpec() == NULL ||
	    tInvalidArticles.find(tArticle) >= 0) {
	    xOiDebugPrint(@Info, 1, ["ignoring ", tEl]);
	    continue;
	}
	var tID = tEl.getName().substr(tSelfLen+1);
	tRet.pushBack(tID);
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}














public func xOiGetSubArticle(pObj, pID)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetSubArticle", [pObj, pID]);

    if (!pObj.isCat(@IF_CompositeArticle)) {
	xOiDebugPrint(@Warn, 1, "given instance is not a composite article!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tObj = NULL;

    try {
	tObj = eval(pObj.getName()+"."+pID);
    }
    catch (&e: Error) {
	xOiDebugPrint(@Warn, 1, ["invalid ID (", e.where, ": ", e.what, ")"]);
    }

    xOiDebugFinishFunc(tObj);
    return(tObj);
}













public func xOiUpdatePropsDeep(pObj, pDepth)
{
    xOiUpdatePropsRecursive(pObj, pDepth, 0);
}

private func xOiUpdatePropsRecursive(pObj, pDepth, pLevel)
{
    xOiDebugStartFunc("xOiFuncs::xOiUpdatePropsRecursive", 
		      [pObj, pDepth, pLevel]);

    pObj.updateProperties();

    if (pLevel >= pDepth) {
	xOiDebugFinishFunc(NULL);
	return;
    }

    if (!pObj.isCat(@IF_CompositeArticle)) {
	xOiDebugPrint(@Info, 1, [pObj, " is not a composite article"]);
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tSubIDs = pObj.getSubArticleIDs();

    if (tSubIDs.empty()) {
	xOiDebugPrint(@Info, 1, "no sub articles");
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tSubID;
    foreach (tSubID; tSubIDs) {
	var tObj = pObj.getSubArticle(tSubID);
	if (tObj == NULL)
	    xOiDebugPrint(@Warn, 1,
			  ["no object for sub article ID ", tSubID, "?!"]);
	else
	    xOiUpdatePropsRecursive(tObj, pDepth, pLevel+1);
    }

    xOiDebugFinishFunc(NULL);
}






























public func xOiUpdateCompositeArticle(pObj)
{
    xOiDebugStartFunc("xOiFuncs::xOiUpdateCompositeArticle", pObj);

    

    if (pObj == NULL || !pObj.hasMember(@isCat) || 
	!(pObj.isCat(@IF_CompositeArticle) || pObj.isA(xOiGroup))) {
	xOiDebugPrint(@Warn, 1, "given object is not a composite article!");
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tUpdateState = pObj.getObjState(@OI_UpdateState);

    var tStates2Handle = @(@Undefined, @Updatable, @Migratable);

    if (tStates2Handle.find(tUpdateState) < 0) {
	xOiDebugPrint(@Warn, 1, 
		      ["update state is not one of ", tStates2Handle]);
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tPlan = pObj.getPlanning();
    var tPDM = pObj.getPDManager();

    if (tPlan == NULL || tPDM == NULL) {
	
	xOiDebugPrint(@ExplWarn, 1, 
		      "xOiUpdateCompositeArticle(): no PD manager!");
	
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tSubArticle, tSubArticles = @(); 

    if (pObj.isA(xOiGroup))
	tSubArticles = pObj.getElements();
    else
    if (pObj.isA(xOiAggregate))
	
        
	tSubArticles = pObj.getParts();
    else 
	tSubArticles = xOiGetCompositeSubArticles(pObj, 1);

    xOiDebugPrint(@Info, 2, ["sub articles: ", tSubArticles]);

    

    if (tUpdateState == @Undefined) {
        var tInvalid = 0;	
				
        var tMigratable = 0;	
				
        if (pObj.isA(xOiGroup))
	    pObj.setObjState(@OI_UpdateState, @Up2Date);
	else {
	    var tCodeType = @OFMLVarCode;
	    if (pObj.getArticleObj().getXArticleSpec(@OFMLVarCode) == NULL)
		
		
		tCodeType = @VarCode;
	    pObj.setObjState(@OI_UpdateState, 
			     tUpdateState = pObj.checkUpdatability(tCodeType));
	}
	
	

	if (tUpdateState == @Invalid)
	    tInvalid = 1;
	else
	if (tUpdateState == @Migratable)
            tMigratable = 1;

	if (tUpdateState != @Invalid) {
	    
	    foreach(tSubArticle; tSubArticles) {
		var tCodeType = @OFMLVarCode;
		var tArtObj = tSubArticle.getArticleObj();
		if (tArtObj == NULL)
		    tArtObj = tSubArticle;
		if (tArtObj.getXArticleSpec(@OFMLVarCode) == NULL)
		    
		    tCodeType = @VarCode;

		
		oiRegisterPackage(tSubArticle.getProgram());

	        tUpdateState = tSubArticle.checkUpdatability(tCodeType);
		tSubArticle.setObjState(@OI_UpdateState, tUpdateState);
	        if (tUpdateState == @Invalid) {
		    tInvalid = 1;
		    break; 
		    
		}
	        if (tUpdateState == @Migratable) 
		    tMigratable = 1;
		    
		    
	    }
	}
	xOiDebugPrint(@Info, 2, ["invalid?    ", tInvalid]);
	xOiDebugPrint(@Info, 2, ["migratable? ", tMigratable]);
	if (tInvalid) 
	    tPlan.issueUpdateMessage(pObj, @InvalidArticle, NULL);

	if (tInvalid || tMigratable) {
	    
	    var tState = @Invalid;
	    if (!tInvalid)
		tState = @Migratable;
	    pObj.setObjState(@OI_UpdateState, tState);
	    foreach(tSubArticle; tSubArticles)
	        tSubArticle.setObjState(@OI_UpdateState, tState);
	    xOiDebugFinishFunc(NULL);
	}

	if (tInvalid ||
	    (tMigratable && !tPlan.confirmArticleMigration(pObj))) {
	    
	    xOiDebugPrint(@Info, 1, "no update/migration!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}
    }
    else
    if (tUpdateState == @Migratable) {
	if (!tPlan.confirmArticleMigration(pObj)) {
	    xOiDebugPrint(@Info, 1, "no update/migration!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}
    }

    var tAssignPriceDate2Old = (tPlan.getPriceDateMode() == @AnySave);
    var tAssigned = 0;
    var tPriceDate = oiGetCurrentDate();

    
    
    if (!pObj.isA(xOiGroup)) {
	xgPH.restorePropertyStates(pObj, 
				   OiFuncs.sSavedMigratablePropStatesDynProp);
        tPDM.updateConfiguration(pObj);
	var tObj = pObj;
	if (pObj.isCat(@IF_MetaType))
	    tObj = pObj.getMainChild();
	if (tObj != NULL && tObj.hasMember(@updateGeometry)) 
	    tObj.updateGeometry();

	if (tAssignPriceDate2Old) {
	    var tObjPriceDate = pObj.getPriceDate();
	    if (tObjPriceDate == NULL) {
		oiDebugPrint(@Info, 2, ["assigning price date ", tPriceDate]);
		tPlan.issueUpdateMessage(pObj, @NoPriceDate, NULL);
		pObj.setPriceDate(tPriceDate);
		tAssigned = 1;
	    }
	}
        pObj.setObjState(@OI_UpdateState, @Up2Date);
    }
    foreach(tSubArticle; tSubArticles) {
        xOiDebugPrint(@Info, 2, ["updating part ", tSubArticle]);
	xgPH.restorePropertyStates(tSubArticle, 
				   OiFuncs.sSavedMigratablePropStatesDynProp);
	tPDM.updateConfiguration(tSubArticle);
	if (tSubArticle.hasMember(@updateGeometry)) 
	    tSubArticle.updateGeometry();

	var tObjPriceDate = tSubArticle.getPriceDate();
	if (tObjPriceDate != NULL && tAssigned) {
	    
	    
            xOiDebugPrint(@Warn, 1, 
			  ["part ", tSubArticle, 
			   " was stored with price date but composite not!?"]);
	}
	if (tObjPriceDate == NULL && tAssignPriceDate2Old)
	    tSubArticle.setPriceDate(tPriceDate);

	tSubArticle.setObjState(@OI_UpdateState, @Up2Date);
    }

    xOiDebugFinishFunc(NULL);
}

func xOiGetCompositeSubArticles(pComposite, pRecursive)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetCompositeSubArticles", 
		      [pComposite, pRecursive]);

    var tSubArticles = @();

    if (pComposite == NULL || !pComposite.hasMember(@isCat) ||
	!pComposite.isCat(@IF_CompositeArticle)) {
	xOiDebugPrint(@Warn, 1, "given object is not a composite article!");
	xOiDebugFinishFunc(tSubArticles);
	return(tSubArticles);
    }

    if (pComposite.hasMember(@getSubArticleIDs)) {
        var tSubIDs = pComposite.getSubArticleIDs();
        var tSubID;
        foreach(tSubID; tSubIDs) 
	    tSubArticles.pushBack(pComposite.getSubArticle(tSubID));
    }
    else 
    if (pComposite.hasMember(@getPropSubArticleIDs)) {
	xOiDebugPrint(@Warn, 1, 
		      [pComposite.getClass(), 
		       " implements old style composite interface!" ]);
        var tSubIDs = pComposite.getPropSubArticleIDs();
        var tSubID;
        foreach(tSubID; tSubIDs) 
	    tSubArticles.pushBack(pComposite.getPropSubArticle(tSubID));
    }
    else
	xOiDebugPrint(@ExplWarn, 1, 
		      [pComposite.getClass(), 
		       " doesn't implement getSubArticleIDs()!" ]);

    if (pRecursive) {
	var tSubArticles2 = @();
	var tSubArticle;
	foreach(tSubArticle; tSubArticles) {
	    if (!tSubArticle.isCat(@IF_CompositeArticle)) continue;
	    xOiCopyAggr(xOiGetCompositeSubArticles(tSubArticle, 1), 
			tSubArticles2, 0);
	}
	xOiCopyAggr(tSubArticles2, tSubArticles, 0);
    }

    xOiDebugFinishFunc(tSubArticles);
    return(tSubArticles);
}












































public func xOiGetAddStateCode(pObj, pDomain, pInherited, pObjDomain, 
			       pMembers, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetAddStateCode", 
		      [pObj, pDomain, pInherited, pObjDomain, pMembers]);

    var tRes = pInherited;

    if (pDomain != NULL && pDomain != pObjDomain) {
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    var tMembers = pMembers[0];
    if (tMembers == NULL)
	tMembers = Vector(0);
    if (!(tMembers instanceof Vector)) {
	xOiDebugPrint(@Warn, 1, "invalid parameter pMembers!");
	tMembers = Vector(0);
    }
    var tSaveSubs = 1;
    if (pMembers.size() > 1)
	tSaveSubs = pMembers[1];

    var tSavedMembers = Vector(0);
    var tM;
    foreach(tM; tMembers) {
	if (!(tM instanceof String) || tM.empty() ||
	    !pObj.hasMember(Symbol(tM))) {
	    xOiDebugPrint(@Warn, 1, ["invalid member: ", tM]);
	    continue;
	}
	var tMember = tM;  
	var tValue;
	try {
	    tValue = eval(pObj.getName()+"."+tMember);
	    if (tValue instanceof Hash)
		tValue = [@xOiHash2List, xOiHash2List(tValue)];
	}
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 1, e.where+": "+e.what);
	    continue;
	}
	tSavedMembers.pushBack([Symbol(tMember), tValue]);
    }

    var tElements = @();
    if (pObj.isCat(@IF_CompositeArticle) && tSaveSubs) {
	var tSelfLen = pObj.getName().size();
	
	var tEl;
	foreach(tEl; pObj.getElements()) {
	    if (!tEl.isCat(@IF_Article) || 
		tEl.getArticleObj().getArticleSpec() == NULL) {
		xOiDebugPrint(@Info, 1, ["ignoring non-article element ", tEl]);
		continue;
	    }
	    var tSubID   = tEl.getName().substr(tSelfLen+1);
	    var tPID     = tEl.getProgram();
	    var tArticle = tEl.getArticleObj().getArticleSpec();
	    var tPos     = tEl.getPosition();
	    var tRot     = tEl.getRotation(@PY);
	    tElements.pushBack([tSubID, tPID, tArticle, tPos, tRot]);
	}
    }

    var tAddState = [@By_xOiGetAddStateCode, @V1, tSavedMembers, tElements];

    var tMyCode = xOiParam2Str(tAddState, 0, 1);

    if (pDomain == pObjDomain)
	tRes = tMyCode;
    else
    if (pDomain == NULL && !tMyCode.empty() &&  tRes instanceof Vector)
	tRes.pushBack([pObjDomain, tMyCode]);

    xOiDebugFinishFunc(tRes);
    return(tRes);
}





















public func xOiGetAddStateCode2(pObj, pDomain, pInherited, pObjDomain, pMembers)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetAddStateCode2", 
		      [pObj, pDomain, pInherited, pObjDomain, pMembers]);

    
    

    var tRes = pInherited;

    if (pDomain != NULL && pDomain != pObjDomain) {
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    var tSavedMembers = Vector(0);
    var tM;
    foreach(tM; pMembers) {
	if (!(tM instanceof String) || tM.empty() ||
	    !pObj.hasMember(Symbol(tM))) {
	    xOiDebugPrint(@Warn, 1, ["invalid member: ", tM]);
	    continue;
	}
	var tMember = tM;  
	var tValue;
	try {
	    tValue = eval(pObj.getName()+"."+tMember);
	    if (tValue instanceof Hash)
		tValue = [@xOiHash2List, xOiHash2List(tValue)];
	}
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 1, e.where+": "+e.what);
	    continue;
	}
	tSavedMembers.pushBack([Symbol(tMember), tValue]);
    }

    var tElements = @();
    if (pObj.isCat(@IF_CompositeArticle)) {
	var tSelfLen = pObj.getName().size();
	
	var tEl;
	foreach(tEl; pObj.getElements()) {
	    if (!tEl.isCat(@IF_Article) || 
		tEl.getArticleObj().getArticleSpec() == NULL) {
		xOiDebugPrint(@Info, 1, ["ignoring non-article element ", tEl]);
		continue;
	    }
	    var tSubID      = tEl.getName().substr(tSelfLen+1);
	    var tPID        = tEl.getProgram();
	    var tArticle    = tEl.getArticleObj().getArticleSpec();
	    var tMatrix     = tEl._getMatrix();
	    var tSelectable = tEl.isSelectable();
	    var tCutable    = tEl.isCutable();
	    var tTrAxis     = tEl.getTrAxis();
	    var tRtAxis     = tEl.getRtAxis();
	    tElements.pushBack([tSubID, tPID, tArticle, tMatrix,
				tSelectable, tCutable, tTrAxis, tRtAxis]);
	}
    }

    var tAddState = [@By_xOiGetAddStateCode, @V2, tSavedMembers, tElements];

    var tMyCode = xOiParam2Str(tAddState, 0, 1);

    if (pDomain == pObjDomain)
	tRes = tMyCode;
    else
    if (pDomain == NULL && !tMyCode.empty() &&  tRes instanceof Vector)
	tRes.pushBack([pObjDomain, tMyCode]);

    xOiDebugFinishFunc(tRes);
    return(tRes);
}





































public func xOiParseAddStateCode(pObj, pDomain, pCode, pObjDomain)
{
    xOiDebugStartFunc("xOiFuncs::xOiParseAddStateCode", 
		      [pObj, pDomain, pCode, pObjDomain]);

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
	    if (tDomain == pObjDomain)
		tMyCode = tCode;
	    else
		tBaseCode.pushBack([tDomain, tCode]);
	}
    }
    else {
	if (pDomain != pObjDomain)
	    tBaseCode = pCode;
	else
	    tMyCode = pCode;
    }

    if (tMyCode instanceof String && tMyCode.empty())
	tMyCode = NULL;

    var tRes = [tBaseCode, tMyCode];

    xOiDebugFinishFunc(tRes);
    return(tRes);
}













public func xOiProcessAddStateCode(pObj, pCode)
{
    xOiDebugStartFunc("xOiFuncs::xOiProcessAddStateCode", [pObj, pCode]);

    if (pCode == NULL || pCode.empty()) {
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tAddState = NULL;

    try {
	tAddState = eval(pCode);
    }
    catch (&e: Error) {
	xOiDebugPrint(@Warn, 1, ["invalid code (", e.what, ")"]);
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    xOiDebugPrint(@Info, 2, ["add state: ", tAddState]);

    if (!(tAddState instanceof Vector && tAddState[0] instanceof Symbol &&
	  tAddState[0] == @By_xOiGetAddStateCode)) {
	xOiDebugPrint(@Warn, 1, ["wrong code: ", tAddState]);
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tVersion = tAddState[1];

    var tMembers = tAddState[2];
    var tM;
    foreach(tM; tMembers) {
	xOiDebugPrint(@Info, 2, ["member ", tM]);
	var tMember = tM[0];
	if (!pObj.hasMember(tMember)) {
	    xOiDebugPrint(@Warn, 1, 
			  ["removed member ", tMember, " in ", pObj, "?!"]);
	    continue;
	}
	var tValue = tM[1];
	if (tValue instanceof Vector && tValue.size() == 2 &&
	    tValue[0] instanceof Symbol && tValue[0] == @xOiHash2List)
	    tValue = xOiList2Hash(tValue[1]);
	if (tValue instanceof Vector || tValue instanceof List)
	    
	    
	    
	    
	    tValue = xOiCopyAggr(tValue, NULL, 1);
	try {
	    eval(pObj.getName()+"."+String(tMember)) = tValue;
	}
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 1,  e.where+": "+e.what);
	}
    } 

    var tElements = tAddState[3];

    if (tElements.empty()) {
	xOiDebugPrint(@Info, 1, "no elements encoded");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tPlan = oiGetPlanning();

    var tE;
    foreach(tE; tElements) {
	if (!(tE instanceof Vector && tE.size() >= 5) ||
	    !(tE[0] instanceof String && tE[2] instanceof String) ||
	    !(tE[1] instanceof Symbol)) {
	    xOiDebugPrint(@Warn, 1, ["invalid element: ", tE]);
	    continue;
	}
	xOiDebugPrint(@Info, 2, ["element: ", tE]);

	var tSubID   = tE[0];
	var tPID     = tE[1];
	var tArticle = tE[2];

	if (!oiRegisterPackage(tPID)) {
	    xOiDebugPrint(@Warn, 1, ["couldn't register package ", tPID]);
	    continue;
	}

	var tType, tSubArt;
	try {
	    if (tPlan.isA(xOiBasePlanning))
		tType = tPlan.article2Class(tArticle, 1);
	    else
		tType = tPlan.article2Class(tArticle);
	    tSubArt = eval(tType+"(pObj, @"+tSubID+")");
	    tSubArt.beAnElement();
	}
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 1, ["couldn't create element ", tSubID,
				     " with article numbber ", tArticle,
				      "' (", e.where, ": ", e.what, ")"]);
	    continue;
	}

	
	
	

	if (tVersion == @V1) {
	    tSubArt.setPosition(tE[3]);
	    xOiSetYRotation(tSubArt, tE[4]);
	}
	else 
	if (tVersion == @V2) {
	    tSubArt._setMatrix(tE[3]);
	    if (tE[4])
		tSubArt.selectable();
	    else
		tSubArt.notSelectable();
	    tSubArt.setCutable(tE[5]);
	    tSubArt.setTrAxis(tE[6]);
	    tSubArt.setRtAxis(tE[7]);
	}
    }

    tPlan.setProgram(pObj.getProgram());

    xOiDebugFinishFunc(NULL);
}


























public func xOiGetPropSpec4AddStateCode(pObj, pPKey)
{
    
    
    
    

    if (!pObj.hasProperty(pPKey)) return(NULL);

    var tSpec  = pObj.getPropSpec(pPKey);
    var tChL   = (tSpec[3] == @None ? NULL : pObj.getPropChoiceList(pPKey, 0)); 
    var tClass = tSpec[12];

    return([tSpec[0:9], tChL, tClass]);
}

















public func xOiRestorePropFromAddStateCode(pObj, pPKey, pSpec, pDynProps)
{
    xOiDebugStartFunc("xOiFuncs::xOiRestorePropFromAddStateCode", 
		      [pObj, pPKey, pSpec]);

    if (pSpec == NULL) {
	xOiDebugPrint(@Warn, 1, "invalid specification parameter!");
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tSpec = pSpec[0];
    var tPDef = tSpec[0:4];

    pObj.setupProperty2(pPKey, tPDef, tSpec[4], tSpec[5], tSpec[6]);
    if (tPDef[3] != @None)
	pObj.setPropChoiceList(pPKey, pSpec[1]);
    if (tSpec[7] != NULL)
	pObj.setPropRanges(pPKey, tSpec[7]);
    if (pSpec[2] != NULL)
	pObj.setPropClass(pPKey, pSpec[2]);

    var tDynProps = (pDynProps instanceof Hash ? 
		     pDynProps : pObj.getDynamicProps());
    tDynProps[pPKey] = tSpec[8];

    xOiDebugFinishFunc(NULL);
}


public func xOiRestorePropSpecFromAddStateCode(pObj, pPKey, pSpec, pDynProps)
{
    xOiRestorePropFromAddStateCode(pObj, pPKey, pSpec, pDynProps);
}















public func xOiGetPropertyByName(pObj, pPropName)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetPropertyByName", [pObj, pPropName]);

    if (pObj == NULL || !(pPropName instanceof String)) {
	xOiDebugPrint(@Info, 1, "invalid parameter(s)");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tPKeys  = pObj.getPropertyKeys();
    var tResKey = NULL;
    var tPKey;

    foreach(tPKey; tPKeys) {
	var tPKey2 = tPKey; 
	var tPDef  = pObj.getPropertyDef(tPKey);
	if (tPDef[0] == pPropName) {
	    if (tResKey == NULL)
		tResKey = tPKey2;
	    else {
		tResKey = NULL;
		break; 
	    }
	}
    }

    xOiDebugFinishFunc(tResKey);
    return(tResKey);
}

























public func xOiGetNextProperty(pObj, pPropKey, pStates, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetNextProperty", 
		      [pObj, pPropKey, pStates]);

    if (pObj == NULL || 
	(pPropKey != NULL && !pObj.hasProperty(pPropKey))) {
	xOiDebugPrint(@Info, 1, "no property with given key");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tValidStateModes = @(@all, @visible, @active);
    var tStates = pStates[0];
    if (pStates == NULL || tValidStateModes.find(pStates) < 0)
	tStates = @visible;

    var tPKeys = NULL;
    if (pStates.size() > 1 && pStates[1] instanceof List)
	tPKeys = pStates[1];
    else
	tPKeys = pObj.getPropertyKeys();

    xOiDebugPrint(@Info, 2, ["property list: ", tPKeys]);

    var tPos   = -1;

    if (pPropKey != NULL && (tPos = tPKeys.find(pPropKey)) < 0) {
	
	xOiDebugPrint(@ExplWarn, 1, "internal error in xOiGetNextProperty()!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tPCount = tPKeys.size();

    if (tPos+1 == tPKeys.size())  {
	xOiDebugPrint(@Info, 1, "is last property");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tResKey = NULL;

    for (++tPos; tPos < tPCount; ++tPos) {
	var tState = pObj.getPropState(tPKeys[tPos]);
	if (tStates == @all  ||
	    (tStates == @visible && tState != -1) ||
	    (tStates == @active  && tState == 1)) {
	    tResKey = tPKeys[tPos];
	    break;
	}
    }

    xOiDebugFinishFunc(tResKey);
    return(tResKey);
}







public func xOiCallPropSetMethod(pObj, pPKey, pValue)
{
    xOiDebugStartFunc2("xOiFuncs::xOiCallPropSetMethod", 
			[pObj, pPKey, pValue]);

    if (!pObj.hasProperty(pPKey)) {
	xOiDebugPrint(@Warn, 1, "no property with given key");
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tPDef = pObj.getPropertyDef(pPKey);

    if (tPDef == NULL) {
	
	xOiDebugPrint(@Warn, 1, "no property definition!?");
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tPTyp  = tPDef[4];
    var tIsNum = xOiPropIsNum(tPTyp);
    var tIsMO  = xOiPropIsMO(tPTyp);

    if (tIsNum == NULL &&
	!(pValue instanceof Void || 
	  pValue instanceof Symbol || pValue instanceof String ||
	  (tIsMO && pValue instanceof Vector))) {
	xOiDebugPrint(@Warn, 1, "invalid value");
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tValS = "(NULL)";
    if (pValue != NULL)
	if (tIsNum != NULL)
	    if (pValue instanceof Symbol)
		tValS = "(Symbol(\""+String(pValue)+"\"))";
	    else
		tValS = "("+String(pValue)+")";
	else
	    if (pValue instanceof Symbol)
		tValS = "(Symbol(\""+String(pValue)+"\"))";
	    else
	    if (pValue instanceof Vector)
		tValS = "("+xOiParam2Str(pValue)+")";
	    else
	    if (!(pValue instanceof String)) {
		xOiDebugPrint(@ExplWarn, 1,
			      ["xOiCallPropSetMethod(): property ", pPKey,
			       " of ", pObj, " has non-String value (",
			       pValue, ") !"]);
		xOiDebugFinishFunc(0);
		return(0);
	    }
	    else
		tValS = "(\""+pValue+"\")";

    var tMethod = "set"+String(pPKey);

    var tRet = 1;

    if (!pObj.forceDynamicProp(pPKey) && pObj.hasMember(Symbol(tMethod)))
    	try {
	    eval("pObj."+tMethod+tValS);
    	}
    	catch (&e: Error) {
	    var tErr = "error while evaluating: '" + 
		       "pObj." + tMethod + tValS + "'\n" +
		   	e.where + ": " + e.what;
	    oiOutput(@WARNING, tErr);
	    tRet = 0;
    	}
    else
	
	pObj.getDynamicProps()[pPKey] = pValue;

    if (tRet) pObj.setChanged();

    xOiDebugFinishFunc(tRet);
    return(tRet);
}








func xOiPropIsNum(pType)
{
    var tType = pType;
    if (pType instanceof Vector)
	tType = pType[0].getPropertyDef(pType[1])[4];

    var tIsFloat = 0;
    var tIsInt = 0;
    var tRes = NULL;
    if (tType.substr(0, 4) == "u ch") {
        var tT5 = tType[4];
        if (tT5 == 'f') tIsFloat = 1;
        else
        if (tT5 == 'i') tIsInt = 1;
    }
    else {
        var tT1 = tType[0];
        if (tT1 == 'f') tIsFloat = 1;
        else
        if (tT1 == 'i' || tT1 == 'b') tIsInt = 1;
    }
    if (tIsFloat || tIsInt) tRes = [tIsInt, tIsFloat];

    return(tRes);
}






func xOiPropIsMO(pType)
{
    var tType = pType;
    if (pType instanceof Vector)
	tType = pType[0].getPropertyDef(pType[1])[4];

    return(tType.size() > 2 && tType.substr(0, 3) == "mch");
}






func xOiPropIsCH(pType)
{
    var tType = pType;
    if (pType instanceof Vector)
	tType = pType[0].getPropertyDef(pType[1])[4];

    return((tType.size() > 1 && tType.substr(0, 2) == "ch") || 
	   (tType.size() > 2 && tType.substr(0, 3) == "mch") ||
	   (tType.size() > 3 && tType.substr(0, 4) == "u ch"));
}






func xOiPropIsCHF(pType)
{
    var tType = pType;
    if (pType instanceof Vector)
	tType = pType[0].getPropertyDef(pType[1])[4];

    return(tType.size() > 2 && tType.substr(0, 3) == "chf");
}






func xOiPropIsUD(pType)
{
    var tType = pType;
    if (pType instanceof Vector)
	tType = pType[0].getPropertyDef(pType[1])[4];

    return(tType.size() > 0 && tType.substr(0, 1) == "u");
}






func xOiPropIsStandardUD(pType)
{
    var tType = pType;
    if (pType instanceof Vector)
	tType = pType[0].getPropertyDef(pType[1])[4];

    return(xOiPropIsUD(tType) && (
	   (tType.size() > 4 && tType.substr(0,  5) == "u chi") ||
           (tType.size() > 9 && tType.substr(0, 10) == "u chi.edit") ||
           (tType.size() > 4 && tType.substr(0,  5) == "u chf") ||
           (tType.size() > 9 && tType.substr(0, 10) == "u chf.edit") ||
	   (tType.size() > 6 && tType.substr(0, 7)  == "u chmat") ||
	   (tType.size() > 5 && tType.substr(0, 6)  == "u fmat") ||
	   (tType.size() > 8 && tType.substr(0, 9)  == "u ch.fmat")));
}






func xOiPropIsEditable(pType)
{
    var tType = pType;
    if (pType instanceof Vector)
	tType = pType[0].getPropertyDef(pType[1])[4];

    return(tType.find(".edit", 0) >= 0);
}




func xOiPropType2ID(pPType)
{
    
    var tSpace = pPType.find(" ", 0);
    if (tSpace < 0) return(pPType);

    if (pPType[0] == 'u') {
	tSpace = pPType.find(" ", tSpace+1);
	if (tSpace < 0) return(pPType);
    }
    return(pPType.substr(0, tSpace));
}





func xOiCHFProp2CHProp(pObj, pPDef)
{
    xOiDebugStartFunc2("xOiFuncs::xOiCHFProp2CHProp", [pObj, pPDef]);

    var tObjName = pObj.getName();
    var tCacheKey = tObjName + " " + pPDef;

    var tChL = NULL;

    if (xOiFuncs.sCHFProp2CHPropCache != NULL && 
	xOiFuncs.sCHFProp2CHPropCache.hasKey(tCacheKey)) {
	xOiDebugPrint(@Info, 1, "using cache");
	tChL = xOiFuncs.sCHFProp2CHPropCache[tCacheKey];
	xOiDebugFinishFunc(tChL);
	return(tChL);
    }

    var tPType = xOiPropType2ID(pPDef);
    var tFunc = tObjName + "." + pPDef.substr(tPType.size()+1);
    xOiDebugPrint(@Info, 2, ["func: ", tFunc]);
    try {
	tChL = eval(tFunc);
    }
    catch (&e: Error) {
	xOiDebugPrint(@EvalErr, 1, e.where + ": " + e.what);
    }

    if (tChL != NULL) tChL = "ch " + tChL;

    if (xOiFuncs.sCHFProp2CHPropCache != NULL) {
	xOiDebugPrint(@Info, 1, "adding to cache");
	xOiFuncs.sCHFProp2CHPropCache[tCacheKey] = tChL;
    }

    xOiDebugFinishFunc(tChL);
    return(tChL);
}











public func xOiGetChoiceListValues(pObj, pProp)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetChoiceListValues", [pObj, pProp]);

    var tRet = @();

    var tChL = pObj.getPropChoiceList(pProp);

    if (tChL != NULL) {
	var tChoice;
	foreach(tChoice; tChL) tRet.pushBack(tChoice[0]);
    }
    else
	xOiDebugPrint(@Info, 1, "invalid property or no choice list");

    xOiDebugFinishFunc(tRet);
    return(tRet);
}









public func xOiValueInChoiceList(pPVal, pChoiceList)
{
    xOiDebugStartFunc("xOiFuncs::xOiValueInChoiceList", 
		      [pPVal, pChoiceList]);

    if (!(pChoiceList instanceof List)) {
	xOiDebugPrint(@Warn, 1, "invalid choicelist parameter!");
        xOiDebugFinishFunc(0);
	return(0);
    }

    var tRet = 0;
    var tV;
    foreach(tV; pChoiceList) {
	if (xOiCompareObjs(pPVal, tV[0])) {
	    tRet = 1;
	    break;
	}
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}

































public func xOiCheckPropertyValue(pObj, pPKey, pPVal)
{
    xOiDebugStartFunc("xOiFuncs::xOiCheckPropertyValue", [pObj, pPKey, pPVal]);

    

    var tPDef;

    if (!pObj.hasProperty(pPKey) || 
	(tPDef = pObj.getPropertyDef(pPKey)) == NULL) {
	xOiDebugPrint(@Warn, 1, "invalid object/property!");
        xOiDebugFinishFunc(0);
	return(0);
    }

    if (xOiCompareObjs(pObj.getPropValue(pPKey), pPVal)) {
	xOiDebugPrint(@Info, 1, "unchanged value");
	xOiDebugFinishFunc(2);
	return(2);
    }

    xOiDebugPrint(@Info, 2, ["prop def: ", tPDef]);

    var tMin   = tPDef[1];
    var tMax   = tPDef[2];
    var tPType = tPDef[4];
    if (xOiPropIsCHF(tPType))
        tPType = xOiCHFProp2CHProp(pObj, tPType);

    var tIsMO = xOiPropIsMO(tPType);
    var tEditable = (tPType.find(".edit") >= 0);

    if (tIsMO && !(pPVal instanceof Vector)) {
	xOiDebugPrint(@ExplWarn, 1,
		      ["no Vector given for multi option property ", 
		       pPKey, "!"]);
	xOiDebugFinishFunc(0);
	return(0);
    }

    if (tIsMO) {
	
	var tPVals = xOiGetValuesFromChoiceList(tPType);
	var tV;
	foreach(tV; pPVal)
	    if (tPVals.find(tV) < 0) {
		xOiDebugPrint(@Warn, 1, ["value ", tV, " not in ", tPVals]);
		xOiDebugFinishFunc(0);
		return(0);
	    }
    }

    if (xOiPropIsCH(tPType)) {
	if (tEditable && tMin == NULL && tMax == NULL) {
	    xOiDebugPrint(@Info, 1, 
			  "editable choice list property without interval");
	    xOiDebugFinishFunc(1);
	    return(1);
	}
	if (!tEditable) {
	    var tPVals = xOiGetValuesFromChoiceList(tPType);
	    if (tPVals == NULL) {
		
		xOiDebugPrint(@Warn, 1, "no choice list !");
		xOiDebugFinishFunc(0);
		return(0);
	    }
	    
	    
	    if (tPVals.find(pPVal) < 0 &&
	        (!(pPVal instanceof Symbol && 
		   (pPVal == @VOID || pPVal == @UNSPECIFIED)) ||
		 tPVals.find(NULL) < 0)) {
		xOiDebugPrint(@Warn, 1, ["new value not in ", tPVals]);
		xOiDebugFinishFunc(0);
		return(0);
	    }
	}
    }

    

    if (xOiPropIsNum(tPType) == NULL || (tMin == NULL && tMax == NULL)) {
        xOiDebugPrint(@Info, 1, "default return value");
        xOiDebugFinishFunc(1);
        return(1);
    }

    if (!(pPVal == NULL || pPVal instanceof Int || pPVal instanceof Float)) {
	
	xOiDebugPrint(@Warn, 1, ["value ", pPVal, " is not numeric!"]);
	xOiDebugFinishFunc(0);
	return(0);
    }

    
    

    if (tMin != NULL) {
	xOiDebugPrint(@Info, 2, ["min: ", tMin]);
	
	if (!(tMin instanceof Int || tMin instanceof Float)) {
	    
	    xOiDebugPrint(@Warn, 1, ["min ", tMin, " is not numeric!"]);
	    xOiDebugFinishFunc(0);  
	    return(0);
	}
	if (pPVal == NULL || pPVal < tMin) {
	    xOiDebugPrint(@Info, 1, ["new value violates min: ", tMin]);
	    xOiDebugFinishFunc(0);
	    return(0);
	}
    }

    if (tMax != NULL) {
	xOiDebugPrint(@Info, 2, ["max: ", tMax]);
	
	if (!(tMax instanceof Int || tMax instanceof Float)) {
	    
	    xOiDebugPrint(@Warn, 1, ["max ", tMax, " is not numeric!"]);
	    xOiDebugFinishFunc(0);  
	    return(0);
	}
	if (pPVal == NULL || pPVal > tMax) {
	    xOiDebugPrint(@Info, 1, ["new value violates max: ", tMax]);
	    xOiDebugFinishFunc(0);
	    return(0);
	}
    }

    xOiDebugPrint(@Info, 1, "default return value");
    xOiDebugFinishFunc(1);
    return(1);
}







func xOiCheckValue4ChfProp(pObj, pPKey, pPVal)
{
    xOiDebugStartFunc2("xOiFuncs::xOiCheckValue4ChfProp", [pObj, pPKey, pPVal]);

    var tPDef;

    if (!pObj.hasProperty(pPKey) || 
	(tPDef = pObj.getPropertyDef(pPKey)) == NULL) {
	xOiDebugPrint(@Warn, 1, "invalid object/property!");
        xOiDebugFinishFunc(0);
	return(0);
    }

    var tPType = tPDef[4];
    if (xOiPropIsCHF(tPType))
        tPType = xOiCHFProp2CHProp(pObj, tPType);

    var tPVals = xOiGetValuesFromChoiceList(tPType);

    var tRet = (tPVals != NULL && tPVals.find(pPVal) > -1);

    xOiDebugFinishFunc(tRet);
    return(tRet);
}








func xOiChPropSubType(pObj, pPType)
{
    xOiDebugStartFunc2("xOiFuncs::xOiChPropSubType", [pObj, pPType]);

    var tRet = "undef";
    var tPType = pPType;

    if (!xOiPropIsCH(tPType)) {
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    if (xOiPropIsCHF(pPType)) {
	tPType = xOiCHFProp2CHProp(pObj, pPType);
	if (tPType == NULL) {
	    
	    xOiDebugPrint(@Info, 1, "eval error during chf-function!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}
    }
    var tPVals = _xOiDecomposeChoiceList(tPType, 1);
    if (tPVals.size() > 0) {
	var t1stVal = tPVals.front()[0];
	if (t1stVal instanceof String)
	    tRet = "string";
	else
	if (t1stVal instanceof Symbol)
	    tRet = "symbol";
    }
    xOiDebugFinishFunc(tRet);
    return(tRet);
}








private func xOiCombinePropDefs(pCommonProp, pObj, pObjProp, pUnion, pLanguage,
				pPlanning)
{
    xOiDebugStartFunc2("xOiFuncs::xOiCombinePropDefs",
		       [pCommonProp, pObj, pObjProp, pUnion, pLanguage]);

    var tObjPKey   = pObjProp[1];
    var tPVal      = pObj.getPropValue(tObjPKey);
    var tCPropType = pCommonProp[4];

    if (tCPropType == "b") {
	xOiDebugFinishFunc("NULL (boolean value)");
	return(tPVal);
    }

    var tObjPDef = pObj.getPropertyDef(tObjPKey);

    var tObjPKeyStr = String(tObjPKey);
    var tIsCommonPr = (tObjPKeyStr.size() > 13 && 
		       tObjPKeyStr.substr(0, 13) == "__COMMON_PROP");

    
    
    
    var tCommonMin = pCommonProp[5];
    var tCommonMax = pCommonProp[6];
    var tObjMin = tObjPDef[1];
    var tObjMax = tObjPDef[2];

    if (tObjMin != NULL && (tCommonMin == NULL || tObjMin > tCommonMin))
	tCommonMin = tObjMin;
    if (tObjMax != NULL && (tCommonMax == NULL || tObjMax < tCommonMax))
	tCommonMax = tObjMax;

    pCommonProp[5] = tCommonMin;
    pCommonProp[6] = tCommonMax;

    if (!xOiPropIsCH(tCPropType)) {
	xOiDebugFinishFunc("NULL (no choice list)");
	return(tPVal);
    }

    
    var tObjPType = tObjPDef[4];
    if (xOiPropIsCHF(tObjPType)) {
	tObjPType = xOiCHFProp2CHProp(pObj, tObjPType);
	if (tObjPType == NULL) {
	    
	    xOiDebugFinishFunc("NULL (eval error in chf-function)");
	    return(tPVal);
	}
    }

    

    if (pObjProp[2] == NULL) pObjProp[2] = @();
    var t1stObj = 0;
    if (pCommonProp[7] == NULL) {
	pCommonProp[7] = @();
	t1stObj = 1;
    }

    var tObjPVals = pObjProp[2];
    var tCommonVals = pCommonProp[7];	
					
					
    var tVCount = tCommonVals.size();
    var tObjVals = _xOiDecomposeChoiceList(tObjPType, 1);

    
    
    
    
    
    

    var tIsEditFloatChL   = (tCPropType == "u chf.edit");
    var tIsEditIntChL     = (tCPropType == "u chi.edit");
    var tNeedCommonSymbol = (!tIsEditFloatChL && !tIsEditIntChL);

    var tLanguage = pLanguage;
    if (pPlanning != NULL)
	tLanguage = pPlanning.getPDLanguage(pObj);

    if (!pUnion) {
	
	var tRmCVals = @();
	var tCommonVal;
	foreach(tCommonVal; tCommonVals) {
	    var tOrigPVal  = tCommonVal[1];
	    var tOrigDescr = tCommonVal[3];
	    var tFound = 0;
    	    var tObjVal;
    	    foreach(tObjVal; tObjVals) {
		var tOPVal   = tObjVal[0];

		if (tOrigPVal != tOPVal) continue;

		if (!tNeedCommonSymbol) {
		    tFound = 1;
		    break;
		}

		var tOPDescr = tObjVal[1];
		if (tOPDescr == NULL) {
		    if (tOPVal instanceof String)
			tOPDescr = tOPVal; 
		    else
			tOPDescr = xOiParam2Str(tOPVal);
		}
		tOPDescr = oiGetStringResource(tOPDescr, tLanguage, pObj);
		if (tOPVal instanceof Symbol && tOPVal == @VOID && 
		    tOPDescr.empty())
		    tOPDescr = oiGetStringResource("::ofml::xoi::@VOID", 
						   tLanguage);
	        if (tOrigDescr == tOPDescr) {
		    tFound = 1;
		    break;
		}
	    }
	    if (!tFound) tRmCVals.pushBack(tCommonVal);
	}
	var tRmCVal;
	foreach(tRmCVal; tRmCVals) tCommonVals.remove(tRmCVal);

	
	foreach(tCommonVal; tCommonVals) 
	    tObjPVals.pushBack([tCommonVal[0], tCommonVal[1]]);
	
	xOiDebugFinishFunc("NULL (intersection)");
	return(tPVal);
    }

    var tObjVal;
    foreach(tObjVal; tObjVals) {
	xOiDebugPrint(@Info, 2, ["check object val ", tObjVal]);
	var tOPVal   = tObjVal[0];
	
	var tRealOPVal = xOiCommon2OriginalProp(pObj, tObjPKey, tOPVal);
	if (tRealOPVal != NULL && tRealOPVal[1] != tOPVal) {
	    xOiDebugPrint(@Info, 2, ["using ", tRealOPVal[1], " instead of ", 
	                             tOPVal, " as value for comparing"]);
	    tOPVal = tRealOPVal[1];
	}

	var tOPDescr = tObjVal[1];
	if (tOPDescr == NULL) {
	    if (tOPVal instanceof String)
		tOPDescr = tOPVal; 
	    else
	        tOPDescr = xOiParam2Str(tOPVal);
	}

	tOPDescr = oiGetStringResource(tOPDescr, tLanguage, pObj);

	if (tOPVal instanceof Symbol && tOPVal == @VOID && tOPDescr.empty())
	    tOPDescr = oiGetStringResource("::ofml::xoi::@VOID", tLanguage);

	var tCommonVal;
	var tAddNewCVal = 1;
	var tPosForNew = tCommonVals.size();
	var tAppendix = 0;
	var i = 0;
	for(i; i<tCommonVals.size(); i++) {
	    if (t1stObj) break;

	    tCommonVal = tCommonVals[i];
	    xOiDebugPrint(@Info, 2, ["check common val ", tCommonVal]);

	    var tOrigPVal  = tCommonVal[1];
	    var tCPDescr   = tCommonVal[2];
	    var tOrigDescr = tCommonVal[3];

	    var tNextCommonVal = NULL;
	    if (i+1 < tCommonVals.size())
		tNextCommonVal = tCommonVals[i+1];

	    var tNextOrigPVal, tNextCPDescr, tNextOrigDescr;
	    if (tNextCommonVal != NULL) {
		tNextOrigPVal  = tNextCommonVal[1];
		tNextCPDescr   = tNextCommonVal[2];
		tNextOrigDescr = tNextCommonVal[3];
	    }

	    xOiDebugPrint(@Info, 3, 
			  ["orig val ", tOrigPVal, " vs. object val ", tOPVal]);
	    xOiDebugPrint(@Info, 3, ["orig descr '", tOrigDescr, 
				     "' vs. object descr '", tOPDescr, "'"]);
	    if (tOrigPVal == tOPVal && 
		(!tNeedCommonSymbol || tOrigDescr == tOPDescr)) {
		
		tAddNewCVal = 0;
	        xOiDebugPrint(@Info, 3, "found value match");
		break;
	    }
	    if (tOrigPVal != tOPVal && tNeedCommonSymbol &&
		tOrigDescr == tOPDescr) {
		
		
		
		
	        xOiDebugPrint(@Info, 3, ["next orig val ", tNextOrigPVal, 
					 " vs. object val ", tOPVal]);
	        xOiDebugPrint(@Info, 3, ["next orig descr '", tNextOrigDescr, 
				     "' vs. object descr '", tOPDescr, "'"]);
	    	if (tNextCommonVal != NULL &&
		    !(tNextOrigPVal != tOPVal && tNextOrigDescr != tOPDescr))
		    continue;
		tPosForNew = i+1;

		
		
		
		
		
		
		if (!tOrigDescr.empty()) tAppendix = 1;
		if (tCPDescr == tOrigDescr && tAppendix)
		    tCommonVal[2] = tCPDescr + " (" + 
				    xOiParam2Str(tOrigPVal) + ")";
		break;
	    }
	}
	if (tAddNewCVal) {
	    tCommonVal = Vector(4);
	    tCommonVal[0] = tOPVal;
	    if (tNeedCommonSymbol)
	        tCommonVal[0] = Symbol(xOiFuncs.sCommonPValPrefix+
				       String(++tVCount));
	    tCommonVal[1] = tOPVal;
	    tCommonVal[2] = tOPDescr;
	    tCommonVal[3] = tOPDescr;
	    if (tAppendix)
		tCommonVal[2] = tOPDescr + " (" + xOiParam2Str(tOPVal) + ")";
	    tCommonVals.insert(tPosForNew, 1, tCommonVal);
	}
	tObjPVals.pushBack([tCommonVal[0], tOPVal]);
    }
 
    xOiDebugFinishFunc(tPVal);
    return(tPVal);
}






func xOiBuildChoiceList(pValues)
{
    if (pValues == NULL || pValues.empty()) return(NULL);

    var tChoices = "ch";

    var tV;
    foreach(tV; pValues) tChoices += " \"" + String(tV) + "\"";

    return(tChoices);
}











func xOiBuildChoiceList2(pType, pValues, ...)
{
    xOiDebugStartFunc2("xOiFuncs::xOiBuildChoiceList2", [pType, pValues]);

    var tValues = pValues[0];
    var tObj = NULL;
    if (pValues.size() > 1)
	tObj = pValues[1];

    var tChoices = _xOiBuildChoiceList(pType, tValues, tObj);

    xOiDebugFinishFunc(tChoices);
    return(tChoices);
}

func _xOiBuildChoiceList(pType, pValues, pObj)
{
    if (!xOiPropIsCH(pType) || pValues == NULL || pValues.empty()) return(NULL);

    xOiDebugStartFunc3("xOiFuncs::_xOiBuildChoiceList", [pType, pValues, pObj]);

    var tLanguage = "de";
    if (pObj != NULL) {
	var tPlanning = pObj.getPlanning();
	if (tPlanning != NULL) tLanguage = tPlanning.getLanguage();
    }

    var tChoices = pType;

    var tV;
    foreach(tV; pValues) {
        if (tV[1] != NULL && tV[1] == "@VOID") {
    	    tChoices = tChoices + " " + tV[1] + " \"" +
		       oiGetStringResource("@VOID", tLanguage, pObj) + "\"";
    	    continue;
        }
        if (tV[1] != NULL && tV[1] == "@UNSPECIFIED") {
    	    tChoices = tChoices + " " + tV[1] + " \"" + 
		       oiGetStringResource("@UNSPECIFIED", tLanguage, pObj) + "\"";
    	    continue;
        }
	var tValStr = NULL;
        if (xOiPropIsNum(pType) != NULL && !(tV[0] instanceof Symbol))
    	    tValStr = String(tV[0]);
        else
    	if (tV[0] instanceof Symbol) {
	    var tSpecials = @(' ');
	    
	    
	    
	    
    	    
    	    tValStr = "@" + String(tV[0]);
	}
    	else
    	if (tV[0] instanceof String && tV[1] == NULL)
    	    tValStr = "\"" + tV[0] + "\"";

        var tDescr = "";
        if (tV[1] != NULL)
	    tDescr = " \"" + xOiEscapeQuotes(String(tV[1])) + "\"";
	    
	    
	else
	    if (tV[0] instanceof Symbol && pObj != NULL) {
		
		var tRes = oiGetStringResource(tValStr, tLanguage, pObj);
		if (tRes != tValStr) tDescr = " \"" + tRes + "\"";
		xOiDebugPrint(@Info, 2, ["val: ", tValStr, ", descr: ", tDescr]);
	    }

        if (tValStr != NULL)
    	    tChoices = tChoices + " " + tValStr + tDescr;
    }

    xOiDebugFinishFunc(tChoices);
    return(tChoices);
}




func _xOiGetPropValueFromString(pPType, pValStr)
{
    

    if (pPType == NULL || pValStr == NULL) {
	
	
	return(NULL);
    }

    
    var tIsFloat = 0;
    var tIsInt = 0;
    var tIsStr = 0;
    var tType = xOiPropIsNum(pPType);
    if (tType == NULL)
        tIsStr = 1;
    else {
        tIsInt = tType[0];
        tIsFloat = tType[1];
    }

    

    var tVal = pValStr;

    if (tIsInt) {
	
	if (tVal == "@VOID" || tVal == "@UNSPECIFIED")
	    tVal = NULL;
	else
            tVal = Int(tVal);
    }
    else
    if (tIsFloat) 
        tVal = Float(tVal);
    else
    if (tVal[0] == '@') 
	
        
        tVal = Symbol(tVal.substr(1, tVal.size()-1));

    
    return(tVal);
}





func xOiGetValuesFromChoiceList(pChList)
{
    

    if (pChList == NULL || !xOiPropIsCH(pChList) || xOiPropIsCHF(pChList)) {
	
	return(NULL);
    }

    if (xOiPropIsUD(pChList) && !xOiPropIsStandardUD(pChList)) {
	
	return(NULL);
    }

    var tLe = pChList.size();
    var tValues = @();

    
    var tP1 = xOiPropType2ID(pChList).size();
    if (tP1+1 >= tLe) {
	
	return(tValues);
    }
    var tVL = _xOiDecomposeChoiceList(pChList, 0);
    var tV;
    foreach(tV; tVL)
	tValues.pushBack(_xOiGetPropValueFromString(pChList, tV[0]));

    
    return(tValues);
}







public func xOiGetPValTxtFromChL(pObj, pChLProp, pPVal)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetPValTxtFromChL", 
		      [pObj, pChLProp, pPVal]);

    if (!pObj.hasProperty(pChLProp)) {
	xOiDebugPrint(@Warn, 1, "invalid property key");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tChList = pObj.getPropertyDef(pChLProp)[4];

    if (xOiPropIsCHF(tChList)) 
	tChList = xOiCHFProp2CHProp(pObj, tChList);

    if (tChList == NULL || !xOiPropIsCH(tChList)) {
	xOiDebugPrint(@Warn, 1, "not a standard choice list");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    if (xOiPropIsUD(tChList) && !xOiPropIsStandardUD(tChList)) {
	xOiDebugPrint(@Warn, 1, "not a valid user definded type");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tVL = _xOiDecomposeChoiceList(tChList, 1);

    var tTxt = NULL;
    var tV;
    foreach(tV; tVL) 
	if (tV[0] == pPVal) {
	    tTxt = tV[1];
	    break;
	}

    xOiDebugFinishFunc(tTxt);
    return(tTxt);
}






private func _xOiDecomposeChoiceList(pChList, pConvert)
{
    xOiDebugStartFunc3("xOiFuncs::_xOiDecomposeChoiceList", 
		       [pChList, pConvert]);

    var tVL = @(); 

    var tTyp = xOiPropType2ID(pChList);
    if (tTyp.size()+1 >= pChList.size()) {
	xOiDebugFinishFunc(tVL);
	return(tVL);
    }

    var tChList = pChList.substr(tTyp.size()+1);

    if (xOiFuncs.sDecomposedChoiceLists == NULL)
        xOiFuncs.sDecomposedChoiceLists = Hash();

    var tCacheKey = tChList + "+" + String(pConvert);

    if (xOiFuncs.sDecomposedChoiceLists.hasKey(tCacheKey)) {
        xOiDebugPrint(@Info, 1, "using cache");
        tVL = xOiFuncs.sDecomposedChoiceLists[tCacheKey];
        xOiDebugFinishFunc(tVL);
        return(tVL);
    }

    var tLe = tChList.size();

    var tEOS = 0; 
    var tEOL = 0; 

    
    while (!tEOS) {
	
    	var tP1 = tChList.findFirstNotOf(' ', tEOL);
    	if (tP1 < 0) {
	    tEOS = 1;
	    break;
	}
    	var tP2;
	if (tChList[tP1] == '\"') {
	    
	    tP2 = tChList.find('\"', tP1+1);
	    while (tP2 > 0 && tChList[tP2-1] == '\\')
		tP2 = tChList.find('\"', tP2+1);
	    if (tP2 == -1) {
		xOiDebugPrint(@Warn, 1, ["missing matching '\"' in ", tChList]);
		xOiFuncs.sDecomposedChoiceLists[tCacheKey] = @();
		xOiDebugFinishFunc(@());
		return(@());
	    }
	    tP2++;
	}
	else {
    	    tP2 = tChList.find(" ", tP1);
    	    if (tP2 == -1) {
		tP2 = tLe;
		tEOS = 1;
	    }
	}
        var tVal = tChList.substr(tP1, tP2-tP1);

	var tDescr = NULL;

	if (tVal[0] == '\"')
	    
	    
	    tVal = xOiSubstString(tVal.substr(1, tVal.size()-2), "\\\"", "\"");
	else {
    	    
            if (!tEOS && (tP1 = tChList.find('\"', tP2)) > 0) {
	    	tP2 = tChList.find('\"', tP1+1);
		while (tP2 > 0 && tChList[tP2-1] == '\\')
		    tP2 = tChList.find('\"', tP2+1);
		if (tP2 == -1) {
		    xOiDebugPrint(@Warn, 1, 
				  ["missing matching '\"' in ", tChList]);
		    xOiFuncs.sDecomposedChoiceLists[tCacheKey] = @();
		    xOiDebugFinishFunc(@());
		    return(@());
		}
	    	tDescr = xOiSubstString(tChList.substr(tP1+1, tP2-tP1-1), "\\", "");
		
	    	
	    	tP2++;
	    }
	}
	if (pConvert) tVal = _xOiGetPropValueFromString(tTyp, tVal);
	tVL.pushBack([tVal, tDescr]);
	tEOL = tP2;
    }
    xOiFuncs.sDecomposedChoiceLists[tCacheKey] = tVL;
    xOiDebugFinishFunc(tVL);
    return(tVL);
}













public func xOiIsObjChildOf(pObj, pFather)
{
    if (pObj == NULL || pFather == NULL) return(0);
    if (pObj == pFather) return(1);

    var tF = pObj;
    while ((tF = tF.getFather()) != NULL)
	if (tF == pFather) return(1);

    return(0);
}








public func xOiIsObjChildOf2(pObj, pFathers)
{
    if (pObj == NULL || pFathers == NULL) return(0);
    if (!(pFathers instanceof Vector || pFathers instanceof List)) return(0);

    var tF;
    foreach(tF; pFathers) {
	if (tF == pObj) continue;
	if (xOiIsObjChildOf(pObj, tF)) return(tF);
    }

   return(NULL);
}







public func xOiGetSelectableFather(pObj)
{
    var tFather = pObj.getFather();
    while (tFather != NULL && !tFather.isSelectable())
	tFather = tFather.getFather();

    return(tFather);
}










public func xOiGetNextAggregateUp(pObj)
{
    var tF   = pObj;
    var tObj = pObj;

    while (!tObj.isA(xOiAggregate) && (tF = tObj.getFather()) != NULL)
	tObj = tF;

    return(tF == NULL ? NULL : tObj);
}







public func xOiGetNextPlGroupUp(pObj)
{
    var tFather = pObj.getFather();
    while (tFather != NULL && !tFather.isA(xOiPlGroup))
	tFather = tFather.getFather();

    return(tFather);
}








public func xOiGetTopObjects(pObjs)
{
    var tRetL = @();
    var tO, tO2;
    foreach(tO; pObjs) {
	if (!tO.hasMember(@isCat) || !tO.isCat(@IF_MObject)) return(NULL);
	var tFather = tO.getFather();
	var tIsChild = 0;
	foreach(tO2; pObjs) {
	    if (tO == tO2) continue;
	    if (tO2 == tFather) {
		tIsChild = 1;
		break;
	    }
	}
	if (!tIsChild) tRetL.pushBack(tO);
    }
    return(tRetL);
}

































public func xOiTraverseHierarchy(pObj, ...)
{
    var tObj = pObj[0];
    var tAction = @PrintObjects;
    if (pObj.size() > 1)
	tAction = pObj[1];

    xOiDebugStartFunc2("xOiFuncs::xOiTraverseHierarchy", [tObj, tAction]);
    var tRet = NULL;

    var tPrintObjects = (tAction == @PrintObjects);
    var tScanMatCats = (tAction == @ScanMatCats);
    var tScanMatCats2 = (tAction == @ScanMatCats2);
    var tCheckNonBasicGeos = (tAction == @HasNonBasicGeos);
    var tCheckOdbLayers = (tAction == @HasOdb3DLayers);
    var tCheckRecovered = (tAction == @HasRecovered);
    var tSetAnyMatCat = (tAction == @SetAnyMatCat);
    var tUpdateGeo = (tAction == @UpdateGeo);

    var tObjects = "";
    var tMatCats = @();
    var tMatCats2 = Hash();
    var tHasNonBasicGeos = 0;
    var tHasOdbLayers = 0;
    var tHasRecovered = 0;

    if (tPrintObjects)
	tObjects += xOiParam2Str(tObj, 1).substr(8);
    else
    if (tScanMatCats && tObj.hasMember(@getMatCategories))
	
	xOiCopyAggr(tObj.getMatCategories(), tMatCats, 1);
    else
    if (tScanMatCats2 && tObj.hasMember(@getMatCategories)) {
	var tMatCats = tObj.getMatCategories();
	if (tMatCats == NULL)
	    tMatCats = @();
	var i;
	for (i=0; i<tMatCats.size(); i++) {
	    var tMatCat  = tMatCats[i];
	    var tCMat    = tObj.getCMaterial(tMatCat);
	    if (tCMat != NULL)
		tMatCats2[tMatCat] = @(tObj.getMatName(tCMat));
	}
    }
    else
    if (tSetAnyMatCat && tObj.isA(OiGeometry))
	tObj.setMatCat(@ANY);
    else
    if (tUpdateGeo && tObj.hasMember(@updateGeo))
	tObj.updateGeo();

    var tCh;
    foreach(tCh; tObj.getChildren()) {
	var tOdbLayer;
	if (tCheckOdbLayers && (tOdbLayer = tCh.get3DLayer()) != NULL &&
	    !tOdbLayer.empty()) {
	    tHasOdbLayers = 1;
	    break;
	}
	else
	if (tCheckNonBasicGeos && !xOiIsABasicGeometry(tCh)) {
	    tHasNonBasicGeos = 1;
	    break;
	}
	else
	if (tCheckRecovered && tCh.hasMember(@isCat) && 
	    tCh.isCat(@IF_RecoveredObject)) {
	    tHasRecovered = 1;
	    break;
	}

	var tRes = xOiTraverseHierarchy(tCh, tAction);

        if (tPrintObjects)
	    tObjects += "\n" + tRes;
	else
	if (tScanMatCats)
	    xOiCopyAggr(tRes, tMatCats, 1);
	else
	if (tScanMatCats2) {
	    var tHKeys = tRes.keys();
	    var i;
	    for (i=0; i<tHKeys.size(); i++) {
		var tMatCat   = tHKeys[i];
		var tMatNames = tRes[tMatCat];
		if (!tMatCats2.hasKey(tMatCat))
		    tMatCats2[tMatCat] = tMatNames;
		else
		    xOiCopyAggr(tMatNames, tMatCats2[tMatCat], 0);
	    }
	}
	else
	if (tCheckNonBasicGeos && tRes) {
	    tHasNonBasicGeos = 1;
	    break;
	}
	else
	if (tCheckRecovered && tRes) {
	    tHasRecovered = 1;
	    break;
	}
    }

    if (tPrintObjects)
	tRet = tObjects;
    else
    if (tScanMatCats) {
	tMatCats.sort();
	tMatCats.unique();
	tRet = tMatCats;
    }
    else
    if (tScanMatCats2) {
	var tHKeys = tMatCats2.keys();
	var i;
	for (i=0; i<tHKeys.size(); i++) {
	    var tMatCat   = tHKeys[i];
	    var tMatNames = tMatCats2[tMatCat];
	    tMatNames.sort();
	    tMatNames.unique();
	    tMatCats2[tMatCat] = tMatNames;
	}
	tRet = tMatCats2;
    }
    else
    if (tCheckNonBasicGeos)
	tRet = tHasNonBasicGeos;
    else
    if (tCheckOdbLayers)
	tRet = tHasOdbLayers;
    else
    if (tCheckRecovered)
	tRet = tHasRecovered;

    xOiDebugFinishFunc(tRet);
    return(tRet);
}






























public func xOiDisintegrateGroup(pGroup, pDoFs, pCutable, pSelectable, 
				 pActivateProps)
{
    xOiDebugStartFunc("xOiFuncs::xOiDisintegrateGroup", pGroup);

    var tFather = pGroup.getFather();

    if (tFather == NULL) {
	xOiDebugPrint(@Warn, 1, "cannot disintegrate hierarchy root object");
        xOiDebugFinishFunc(NULL);
    }

    xOiReparentObjects(pGroup.getChildren(), tFather, pDoFs, pCutable, 
		       pSelectable, pActivateProps);

    xOiDebugFinishFunc(NULL);
}




































public func xOiReparentObjects(pObjects, pNewFather, pDoFs, pCutable, 
			       pSelectable, pPropActivation)
{
    xOiDebugStartFunc("xOiFuncs::xOiReparentObjects", [pObjects, pNewFather]);

    if (pNewFather == NULL || pObjects == NULL || 
	!(pObjects instanceof List || pObjects instanceof Vector)) {
	xOiDebugPrint(@Warn, 1, "wrong parameter pObjects and/or pNewFather");
        xOiDebugFinishFunc(NULL);
	return;
    }

    var tDoFs = Hash();
    if (pDoFs instanceof Hash) {
	tDoFs = pDoFs;
	xOiDebugPrint(@Info, 1, ["degrees of freedom: ", tDoFs]);
    }
    xOiDebugPrint(@Info, 1, ["cutable state(s): ", pCutable]);
    xOiDebugPrint(@Info, 1, ["selectable state(s): ", pSelectable]);
    xOiDebugPrint(@Info, 1, ["property activation info: ", pPropActivation]);

    var tNewObjects = @();
    var i;
    for(i=0; i<pObjects.size(); i++) {
	var tObj = pObjects[i];
	var tFather = tObj.getFather();
	if (!tObj.isA(OiPlElement) && !tObj.isA(OiPart)) {
	    xOiDebugPrint(@Info, 2, ["ignoring ", tObj, 
				    " not being a planning element!"]);
	    continue;
	}
	if (!(xOiIsObjChildOf(tObj, pNewFather) && pNewFather != tFather)) {
	    xOiDebugPrint(@Info, 2, ["ignoring ", tObj, 
			             " not being a grand child of new father ",
			   	     pNewFather]);
	    continue;
	}
	var tObjName = tObj.getName();
	xOiDebugPrint(@Info, 2, ["ungrouping/reparenting ", tObj]);

	
	var tDoF = NULL;
	if (tDoFs.hasKey(tObjName))
	    tDoF = tDoFs[tObjName];
	xOiDebugPrint(@Info, 2, ["degrees of freedom: ", tDoF]);

	
	var tGrChDoFs = @();
	var tName;
	foreach(tName; tDoFs.keys()) {
	    if (tName.size() > tObjName.size()+1 &&
		tName.substr(0, tObjName.size()) == tObjName)
		tGrChDoFs.pushBack([tName.substr(tObjName.size()+1),
				    tDoFs[tName]]);
	}
	xOiDebugPrint(@Info, 2, 
		      ["degrees of freedom of grand children: ", tGrChDoFs]);

	
	var tActivateProps = @();
	if (pPropActivation instanceof Hash &&
	    pPropActivation.hasKey(tObjName))
	    tActivateProps = pPropActivation[tObjName];
	xOiDebugPrint(@Info, 2, ["reactivate props: ", tActivateProps]);

	
	var tGrChActProps = @();
	if (pPropActivation instanceof Hash) {
	    foreach(tName; pPropActivation.keys()) {
	        if (tName.size() > tObjName.size()+1 &&
		    tName.substr(0, tObjName.size()) == tObjName)
		    tGrChActProps.pushBack([tName.substr(tObjName.size()+1),
				        pPropActivation[tName]]);
	    }
	}
	xOiDebugPrint(@Info, 2, 
		      ["reactivate props of grand children: ", tGrChActProps]);

        
	

	var tPos = tObj.getPosition();
        
        
	
	
	var tNewPos = xOiTransformObjCoords(tFather, tPos, pNewFather);
	tObj.setPosition([tNewPos[0], tNewPos[1], tNewPos[2]]);

	
        
        
        
        
        
	var tNewRot = oiGetWorldRotation(tObj, @PY) -
		      oiGetWorldRotation(pNewFather, @PY);
	xOiSetYRotation(tObj, tNewRot);

        
	var tNewObj  = oiReparent(pNewFather, tObj);
	var tNewName = tNewObj.getName();

	tNewObjects.pushBack(tNewObj);

	
	if (tDoF != NULL) {
	    tNewObj.setTrAxis(tDoF[0]);
	    tNewObj.setRtAxis(tDoF[1]);
	}
	var tGrChDoF;
	foreach(tGrChDoF; tGrChDoFs) {
	    var tChName = tNewName+"."+tGrChDoF[0];
	    if (oiExists(tChName)) {
		var tCh = eval(tChName);
		tCh.setTrAxis(tGrChDoF[1][0]);
		tCh.setRtAxis(tGrChDoF[1][1]);
	    }
	    else
		xOiDebugPrint(@Warn, 1, ["missing child ", tChName, 
					 " after reparenting"]);
	}

	
	if (pCutable instanceof Int)
	    tNewObj.setCutable(pCutable);

	
	if (pSelectable instanceof Int) {
	    if (pSelectable)
	        tNewObj.selectable();
	    else
		tNewObj.notSelectable();
	}

	
	if (pPropActivation == NULL) continue;

	if (pPropActivation instanceof Int && pPropActivation == 0) {
	    tNewObj.invalidateProperties();
	    continue;
	}

	var tPKey;
	foreach(tPKey; tActivateProps) {
	    if (tNewObj.hasProperty(tPKey))
		tNewObj.setPropState(tPKey, 1);
	    else
		xOiDebugPrint(@Warn, 1, ["missing property ", tPKey,
					 " after reparenting"]);
	}
	var tGrChP;
	foreach(tGrChP; tGrChActProps) {
	    var tChName = tNewName+"."+tGrChActProps[0];
	    if (!oiExists(tChName)) {
		xOiDebugPrint(@Warn, 1, ["missing child ", tChName, 
					 " after reparenting"]);
		continue;
	    }
	    var tCh = eval(tChName);
	    foreach(tPKey; tGrChActProps[1]) {
	        if (tCh.hasProperty(tPKey))
		    tCh.setPropState(1);
	        else
		    xOiDebugPrint(@Warn, 1, 
				  ["missing property ", tPKey, " for child ",
				   tChName, " after reparenting"]);
	    }
	}
    }

    xOiDebugFinishFunc(tNewObjects);
    return(tNewObjects);
}






public func xOiGetLocalObjName(pFather, pObj)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetLocalObjName", [pFather, pObj]); 

    var tFName = pFather.getName();
    var tFSize = tFName.size();
    var tObjName = pObj.getName();

    var tRet = NULL;

    if (tObjName.size() > tFSize && tObjName.substr(0, tFSize) == tFName)
	tRet = tObjName.substr(tFSize+1);

    xOiDebugFinishFunc(tRet);
    return(tRet);
}






public func xOiGetObj4LocalName(pFather, pName)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetObj4LocalName", [pFather, pName]); 

    var tObj = NULL;

    try {
	tObj = eval(pFather.getName()+"."+pName);
    }
    catch (&e: Error) {
	xOiDebugPrint(@ExplWarn, 1,
		      ["invalid local name (", e.where, ": ", e.what, ")"]);
    }

    xOiDebugFinishFunc(tObj);
    return(tObj);
}








public func xOiGetHighestEnameNum(pObj)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetHighestEnameNum", pObj);

    var tONameLen = pObj.getName().size();
    var tChildren = pObj.getChildren();

    xOiDebugPrint(@Info, 2, ["children: ", tChildren]);

    var tCh, tHighestNum = 0;
    foreach(tCh; tChildren) {
	var tName = tCh.getName().substr(tONameLen+1);
	if (tName[0] != 'e' || tName.size() == 1 || !xOiIsnum(tName[1]))
	    continue;
	var tNum = NULL;
	try {
	    tNum = Int(tName.substr(1));
	}
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 1, e.where + ": " + e.what);
	}
	if (tNum != NULL && tNum > tHighestNum)
	    tHighestNum = tNum;
    }

    xOiDebugFinishFunc(tHighestNum);
    return(tHighestNum);
}




















public func xOiCheckWallCrossing(pObj, pNewPos, pOldPos, pCorrectPos)
{
    xOiDebugStartFunc("xOiFuncs::xOiCheckWallCrossing", 
		      [pObj, pNewPos, pOldPos, pCorrectPos]);

    var tRoot = pObj.getRoot();
    var tPlanning = pObj.getPlanning();
    if (tPlanning == NULL || tPlanning != tRoot || 
	!tPlanning.isA(xOiPlanning)) {
	
	xOiDebugPrint(@Info, 1, "foreign hierarchy");
	xOiDebugFinishFunc(1);
	return(1);
    }

    var tEnv = tPlanning.getEnvironment();
    if (tEnv == NULL) {
	xOiDebugPrint(@Info, 1, "no environment");
	xOiDebugFinishFunc(1);
	return(1); 
    }

    var tViewMode = tPlanning.getAppMode(@View);
    var tCheck = tPlanning.getAppMode(@CheckWallCrossing);

    xOiDebugPrint(@Info, 2, ["view mode: ", tViewMode]);
    xOiDebugPrint(@Info, 2, ["check mode: ", tCheck]);

    if (tViewMode == NULL ||  
	(tViewMode == @_3D && !(tCheck & 1)) ||
	(tViewMode == @_2D && !(tCheck & 2))) {
	xOiDebugPrint(@Info, 1, "no check required");
	xOiDebugFinishFunc(1);
	return(1);
    }

    var tNewWPos = xOiGetWorldPosition(pObj, pNewPos);
    var tOldWPos = xOiGetWorldPosition(pObj, pOldPos);
    var tDist = xOiGetDistance([tNewWPos[0], tNewWPos[2]], 
			       [tOldWPos[0], tOldWPos[2]]);
    var tAngle = xOiGetPYRot(tOldWPos, tNewWPos);

    xOiDebugPrint(@Info, 2, ["distance: ", tDist]);
    xOiDebugPrint(@Info, 2, ["angle: ", tAngle]);
    xOiDebugPrint(@Info, 2, ["old pos: ", tOldWPos]);

    var tThreshold = 0.15; 
    if (tDist < tThreshold) {
	xOiDebugPrint(@Info, 1, "distance below threshold");
	xOiDebugFinishFunc(1);
	return(1);
    }

    var tDepth = 0.001;
    var tHeight = 2.0;
    var tTmpTop = OiGeometry(tPlanning, @_egr_found_tmp);
    var tBlock = OiBlock(tTmpTop, @_block, [tDist-tDepth, tHeight, tDepth]);
    tBlock.setPosition([tDepth/2, 0.0, -tDepth/2]);
    tTmpTop.setPosition(tOldWPos);
    tTmpTop.rotate(@PY, tAngle);

    xOiDebugPrint(@Info, 2, ["bounds of temporary check object: ",
			     tTmpTop.getWorldBounds()]);
    var tCollision = 0;
    var tEnvObj;
    foreach(tEnvObj; tEnv.getChildren())
	if (tEnvObj.hasMember(@isCat) && tEnvObj.isCat(@IF_Wall) && 
	    oiCollision(tEnvObj, tTmpTop)) {
    	    xOiDebugPrint(@Info, 2, ["collision with ", tEnvObj, ", bounds: ",
				     tEnvObj.getLocalBounds()]);
	    tCollision = 1;
	    break;
	}
    tPlanning.remove(tPlanning._egr_found_tmp);

    if (tCollision && pCorrectPos)
	
	xOiTranslateAsFarAsPossible(pObj, pOldPos, pNewPos);

    xOiDebugFinishFunc(!tCollision);
    return(!tCollision);
}









public func xOiTranslateAlongAxes(pObj, pOld)
{
    xOiDebugStartFunc("xOiFuncs::xOiTranslateAlongAxes", [pObj, pOld]);

    var tF    = pObj.getFather();
    var tPos  = pObj.getPosition();
    var tDiff = [tPos[0]-pOld[0], tPos[1]-pOld[1], tPos[2]-pOld[2]];

    var tP, tPValid = pOld;

    
    if (fabs(tDiff[0]) > 0.0001) {
	tP = [tPValid[0] + tDiff[0], tPValid[1], tPValid[2]];
	pObj.setPosition(tP);
	if (tF.checkElPos(pObj, tPValid)) tPValid = tP;
    }

    
    if (fabs(tDiff[2]) > 0.0001) {
	tP = [tPValid[0], tPValid[1], tPValid[2] + tDiff[2]];
	pObj.setPosition(tP);
	if (tF.checkElPos(pObj, tPValid)) tPValid = tP;
    }

    
    if (fabs(tDiff[1]) > 0.0001) {
	tP = [tPValid[0], tPValid[1] + tDiff[1], tPValid[2]];
	pObj.setPosition(tP);
	if (tF.checkElPos(pObj, tPValid)) tPValid = tP;
    }

    pObj.setPosition(tPValid);

    xOiDebugFinishFunc(tPValid);
    return(tPValid);
}











public func xOiTranslateAsFarAsPossible(pObj, pOld, pNew)
{
    xOiDebugStartFunc("xOiFuncs::xOiTranslateAsFarAsPossible", 
		      [pObj, pOld, pNew]);

    var tF    = pObj.getFather();
    var tPos  = pNew; 
    var tMV   = [tPos[0]-pOld[0], tPos[1]-pOld[1], tPos[2]-pOld[2]];
    var tDist = sqrt(tMV[0]*tMV[0]+tMV[1]*tMV[1]+tMV[2]*tMV[2]);

    xOiDebugPrint(@Info, 2, ["distance: ", tDist]);

    if (fabs(tDist) < 0.0001) {
        xOiDebugFinishFunc(pNew);
	return(pNew);
    }

    var tMinD, tMaxD, tS, tD, tP;

    tMV   = [tMV[0]/tDist, tMV[1]/tDist, tMV[2]/tDist];
					
    tMinD = 0.0;			
    tMaxD = tDist;			
    tS    = -1; 
    tD    = tMaxD;
	    
    while (tMaxD-tMinD >= 0.0001) {
	tD = tD + tS*(tMaxD-tMinD)/2;
	tP = [pOld[0] + tD*tMV[0], pOld[1] + tD*tMV[1], pOld[2] + tD*tMV[2]];
	pObj.setPosition(tP);
	if (tF.checkElPos(pObj, pOld) && 
	    xOiCheckWallCrossing(pObj, tP, pOld, 0)) {
	    tMinD = tD;
	    tS = 1;
	}
	else {
	    tMaxD = tD;
	    tS = -1;
	}
    }
    if (tS == -1)
	tP = [pOld[0] + tMinD*tMV[0], 
	      pOld[1] + tMinD*tMV[1], 
	      pOld[2] + tMinD*tMV[2]];
    pObj.setPosition(tP);

    xOiDebugFinishFunc(tP);
    return(tP);
}











public func xOiCheckElPos(pObj, pEl, pOldPos)
{
    xOiDebugStartFunc("xOiFuncs::xOiCheckElPos", [pObj, pEl, pOldPos]);

    var tPlan = pObj.getPlanning();
    var tCheckTopPos = 0;
    if (tPlan != NULL && tPlan.hasMember(@getTopPosCheckMode))
	tCheckTopPos = tPlan.getTopPosCheckMode();

    var tRet;

    if (pObj.isElemCatValid(@TOP_ELEM) && pEl.isCat(@TOP_ELEM) &&
	tCheckTopPos && !xOiCheckTopPos(pEl, pOldPos, @old)) 
	tRet = 0;
    else
        tRet = (pObj.checkChildColl(pEl, NULL) == NULL);

    xOiDebugFinishFunc(tRet);
    return(tRet);
}
































public func xOiGetTopPos(pObj, ...)
{
    var tObj = pObj[0];
    var tPos = NULL;
    if (pObj.size() > 1)
	tPos = pObj[1];

    xOiDebugStartFunc("xOiFuncs::xOiGetTopPos", [tObj, tPos]);

    var tF  = tObj.getFather();
    var tFW = tF.getWidth();
    var tFH = tF.getHeight();
    var tFD = tF.getDepth();
    var tFO = tF.getOrigin();
    var tOW = tObj.getWidth();
    var tOH = tObj.getHeight();
    var tOD = tObj.getDepth();
    var tOO = tObj.getOrigin();

    var tPlan = tObj.getPlanning();
    var tCheckTopPos = 0;
    if (tPlan != NULL && tPlan.hasMember(@getTopPosCheckMode))
	tCheckTopPos = tPlan.getTopPosCheckMode();

    xOiDebugPrint(@Info, 2, ["father ", tF]);
    xOiDebugPrint(@Info, 2, ["father data: ", [tFO, [tFW, tFH, tFD]]]);
    xOiDebugPrint(@Info, 2, ["child data:  ", [tOO, [tOW, tOH, tOD]]]);
    xOiDebugPrint(@Info, 2, ["check top pos? ", tCheckTopPos]);

    var tRet = [NULL, NULL, "::ofml::xoi::@coll"];

    
    
    

    if (tPos != NULL) {
	var tA = 0.0;
	if (tPos[0] instanceof Vector) {
	    if (tPos.size() > 1) 
		tA  = tPos[1];
	    tPos = tPos[0];
	}

	
	
	var tOrigYPos = tPos[1] - tFO[1];
	var tCorrYPos = tFH - tFO[1];

	tPos = [tPos[0], tCorrYPos, tPos[2]];

	tObj.setPosition(tPos);
	xOiSetYRotation(tObj, tA);

	xOiDebugPrint(@Info, 2, ["try explicit pos ", tPos,
				 " with rotation ", tA*180/xOiPi]);

	if (tF.checkElPos(tObj, [0, tCorrYPos, 0])) {
	    tRet = [tPos, tA, NULL];
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	if (tCorrYPos > tOrigYPos) {
	    
	    
	    
	    
	    tPos = [tPos[0], tOrigYPos, tPos[2]];
	    tObj.setPosition(tPos);
	    xOiDebugPrint(@Info, 2, ["try original explicit y-pos ", tPos]);
	    if (tF.checkElPos(tObj, [0, tOrigYPos, 0])) {
	        tRet = [tPos, tA, NULL];
	        xOiDebugFinishFunc(tRet);
	        return(tRet);
	    }
	}

	if (xOiAskYesNo("::ofml::xoi::@invexpltoppos")) {
	    tRet = [tPos, tA, NULL];
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}
	    
	tRet = [NULL, NULL, "::ofml::app::@none"];
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tY = tFH - tFO[1] + tOO[1];

    var tA;
    var tAngles = @(0.0, xOiPi2, xOiPi4, -xOiPi4);

    var tModes = [@Center, @Raster];
    if (tObj.hasMember(@getTopPosModes))
	tModes = tObj.getTopPosModes();

    xOiDebugPrint(@Info, 2, ["modes: ", tModes]);

    var tLastCollPos = NULL;
    var tEps = 0.0001;

    
    if (xOiObjInList(@Center, tModes)) {
	xOiDebugPrint(@Info, 1, "trying center ...");
    	var tCP = [tFW/2 - tFO[0], tY, tFD/2 - tFO[2]];
    	var tX  = tCP[0] - tOW/2 + tOO[0];
    	var tZ  = tCP[2] - tOD/2 + tOO[2];

    	foreach(tA; tAngles) {
	    xOiDebugPrint(@Info, 2, ["try center ", tCP, 
				     " with rotation ", tA*180/xOiPi]);
	    var tPos = xOiGetRotPt3D(tCP, [tX, tY, tZ], tA, @P);

	    xOiDebugPrint(@Info, 2, ["check pos: ", tPos]);

	    if (tCheckTopPos) {
		if (!xOiCheckTopPos(tObj, tPos, @new)) continue;

	        var tPos2 = tObj.getPosition();

	        if (xOiCompareAggr(tPos, tPos2, 0) != 1) {
		    tPos = tPos2;
		    xOiDebugPrint(@Info, 2, ["corrected pos: ", tPos]);
	        }
	    }
	    else
	        tObj.setPosition(tPos);

	    xOiSetYRotation(tObj, tA);

	    var tOk = (tF.checkChildColl(tObj, NULL) == NULL);

	    xOiDebugPrint(@Info, 2, ["OK? ", tOk]);

	    if (tOk) {
		xOiDebugFinishFunc([tPos, tA, NULL]);
		return([tPos, tA, NULL]);
	    }
	    if (tLastCollPos == NULL)
		tLastCollPos = [tPos[0], tY, tPos[2]];
	}
    }

    tA = 0.0;

    
    if (xOiObjInList(@Raster, tModes)) {
	xOiDebugPrint(@Info, 1, "trying raster ...");
    	xOiSetYRotation(tObj, tA);

    	var tX0 = -tFO[0] - tOO[0];
    	var tZ0 = -tFO[0] - tOO[2];
	
    	var tDX = tOW;
    	var tDZ = tOD;
	var tDMin = 0.01;
	var tX, tZ;
	while (tDX >= tDMin && tDZ >= tDMin) {
	    for (tZ = tZ0; tZ < tFD; tZ += tDZ)
		for (tX = tX0; tX < tFW; tX += tDX) {
		    var tPos = [tX, tY, tZ];
		    xOiDebugPrint(@Info, 2, ["try pos: ", tPos]);
		    if (tCheckTopPos) {
			var tOk = xOiCheckTopPos(tObj, tPos, @new);
			if (!tOk) continue;
			var tPos2 = tObj.getPosition();
			if (xOiCompareAggr(tPos, tPos2, 0) != 1) {
			    tPos = tPos2;
			    xOiDebugPrint(@Info, 2, ["corrected pos: ", tPos]);
			}
		    }
		    else
			tObj.setPosition(tPos);
		    var tOk = (tF.checkChildColl(tObj, NULL) == NULL);
		    xOiDebugPrint(@Info, 3, ["pos ok? ", tOk]);
		    if (tOk) {
			xOiDebugFinishFunc([tPos, tA, NULL]);
			return([tPos, tA, NULL]);
		    }
		    else
			tLastCollPos = tPos; 
		}
	    tDX = tDX/4;
	    tDZ = tDZ/4;
	}
    }

    xOiDebugPrint(@Info, 2, ["last coll pos: ", tLastCollPos]);

    var tError = "::ofml::app::@none";

    var tErrRes = "::ofml::xoi::@continue_checkAdd";
    var tErrRes2 = "::ofml::xoi::@invtoppos2a";

    if (tF.getFather() == tF.getScene()) {
	tErrRes = "::ofml::xoi::@ask_place_beside";
	tErrRes2 = "::ofml::xoi::@invtoppos2b";
    }

    if (tLastCollPos != NULL) {
	if (xOiAskYesNo("::ofml::xoi::@invtoppos")) {
	    xOiDebugFinishFunc([tLastCollPos, tA, NULL]);
	    return([tLastCollPos, tA, NULL]);
	}
	if (xOiAskYesNo(tErrRes))
	    tError = NULL;
    }
    else
	if (xOiAskYesNo(tErrRes2))
	    tError = NULL;

    xOiDebugFinishFunc([NULL, NULL, tError]);
    return([NULL, NULL, tError]);
}


























public func xOiCheckTopPos(pObj, pPos, pMode)
{
    xOiDebugStartFunc("xOiFuncs::xOiCheckTopPos", [pObj, pPos, pMode]);

    var tBase = pObj.getFather();

    
    
    var tBO = tBase.getOrigin();
    var tBW = tBase.getWidth();
    var tBH = tBase.getHeight();
    var tBD = tBase.getDepth();

    xOiDebugPrint(@Info, 2, ["father ", tBase]);
    xOiDebugPrint(@Info, 2, ["father's origin:    ", tBO]);
    xOiDebugPrint(@Info, 2, ["father's dimension: ", [tBW, tBH, tBD]]);

    var tTB = pObj.getLocalBounds();
    var tTO = tTB[0];
    var tTW = tTB[1][0] - tTO[0];
    var tTH = tTB[1][1] - tTO[1];
    var tTD = tTB[1][2] - tTO[2];

    var tTPos = pObj.getPosition();
    var tTRot = pObj.getRotation(@PY);
    var tCPos = tTPos;	

    if (pPos != NULL && (pMode == @new || pMode == @pos)) tCPos = pPos;

    var tNeedCorrect = (pPos != NULL && pMode == @old);

    var eps = 0.0001;

    var tCenter = xOiTransformXZCoords(tCPos, tTRot, 
				       tTO[0]+tTW/2, tTO[2]+tTD/2);
    var tTX = tCenter[0];
    var tTZ = tCenter[2];

    xOiDebugPrint(@Info, 2, ["pos of object's center: ", [tTX, tTZ]]);

    

    var tDX = 0.0;	
    var tDZ = 0.0;	

    var tDL  = tTX + tBO[0];		
    if (tDL < -eps)
	if (tNeedCorrect) 
	    tDX -= tDL;
	else {
	    xOiDebugPrint(@Info, 1, "invalid left side");
	    xOiDebugFinishFunc(0);
	    return(0);
	}
    var tDR  = tTX - (tBW - tBO[0]);	
    if (tDR > eps)
	if (tNeedCorrect) 
	    tDX -= tDR;
	else {
	    xOiDebugPrint(@Info, 1, "invalid right side");
	    xOiDebugFinishFunc(0);
	    return(0);
	}
    var tDB  = tTZ + tBO[2];		
    if (tDB < -eps)
	if (tNeedCorrect) 
	    tDZ -= tDB;
	else {
	    xOiDebugPrint(@Info, 1, "invalid back side");
	    xOiDebugFinishFunc(0);
	    return(0);
	}
    var tDF  = tTZ - (tBD - tBO[2]);	
    if (tDF > eps)
	if (tNeedCorrect) 
	    tDZ -= tDF;
	else {
	    xOiDebugPrint(@Info, 1, "invalid front side");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

    if (tNeedCorrect) {
	xOiDebugPrint(@Info, 2, ["correction vector: ", [tDX, tDZ]]);
	tCPos[0]   += tDX; tCPos[2]   += tDZ;
	tCenter[0] += tDX; tCenter[2] += tDZ;
    }

    
    
    
    
    
    

    var tOffset = 0.01;
    pObj.setPosition([tCPos[0], tCPos[1]+tOffset, tCPos[2]]);
    var tCPY  = pObj.getWorldBounds()[0][1];

    
    
    
    var tBPos = tBase.getPosition();
    var tBRot = tBase.getRotation(@PY);
    var tCPoint = xOiTransformXZCoords(tBPos, tBRot, tCenter[0], tCenter[2]);
    var tBF = tBase.getFather();
    while (tBF.getFather() != NULL) {
	tCPoint = xOiTransformXZCoords(tBF.getPosition(), tBF.getRotation(@PY),
				       tCPoint[0], tCPoint[2]);
	tBF = tBF.getFather();
    }
    tCPoint = [tCPoint[0], tCPY, tCPoint[2]];

    xOiDebugPrint(@Info, 2, ["check point: ", tCPoint]);

    var tOk = 1;
    var tObj  = oiGetNearestObject(tCPoint, [0, -1, 0]);

    xOiDebugPrint(@Info, 2, ["nearest ", tObj]);

    if (tObj == NULL || !xOiIsObjChildOf(tObj, tBase))
	tOk = 0;
    else {
	var tDist = oiGetDistance(tCPoint, [0, -1, 0]);
	if (fabs(tOffset-tDist) > eps) {
	    xOiDebugPrint(@Info, 2, ["distance: ", tDist]);
	    if (pPos != NULL && pMode == @new) {
		xOiDebugPrint(@Info, 3, ["current check pos: ", tCPos]);
		tCPos = [tCPos[0], tCPos[1]-tDist+tOffset, tCPos[2]];
		xOiDebugPrint(@Info, 2, ["corrected check pos: ", tCPos]);
	    }
	    else
		tOk = 0;
	}
    }

    pObj.setPosition(tTPos); 

    if (tOk) {
	if (pPos != NULL && pMode == @new) pObj.setPosition(tCPos);
	xOiDebugFinishFunc(1);
	return(1);
    }

    if (!tNeedCorrect) {
	xOiDebugFinishFunc(0);
	return(0);
    }

    
    
    
    
    
    
    
    

    xOiDebugPrint(@Info, 1, "correcting position ...");

    tDX  = tTPos[0] - pPos[0];
    tDZ  = tTPos[2] - pPos[2];
    var tDXS  = xOiSign(tDX);
    var tDZS  = xOiSign(tDZ);
    var tDist = sqrt(tDX*tDX + tDZ*tDZ);
    var tANP  = xOiGetPYRot(pPos, tTPos); 

    var tDir1 = [tDXS, 0];
    var tDir2 = [0, tDZS];

    var tDirs = Vector(0);
    var tStartCPs = Vector(0);

    pObj.setPosition([tTPos[0], tTPos[1]+tOffset, tTPos[2]]);
    var tDir, tCP0, tCPUp;
    foreach(tDir; [tDir1, tDir2]) {
	if (fabs(tDir[0]*tDX) < eps && fabs(tDir[1]*tDZ) < eps) continue;

	tCP0 = [pPos[0] + tDir[0]*fabs(tDX), pPos[1], 
		pPos[2] + tDir[1]*fabs(tDZ)];
	tCPUp = xOiTransformXZCoords(tBPos, tBRot, tCP0[0], tCP0[2]);
	tObj = oiGetNearestObject(tCPUp, [0, -1, 0]);
	if (tObj != NULL && xOiIsObjChildOf(tObj, tBase) &&
	    fabs(tOffset-oiGetDistance(tCPUp, [0, -1, 0])) < eps) {
	    tDirs.pushBack(tDir);
	    tStartCPs.pushBack(tCP0);
	}
    }
    pObj.setPosition(tTPos);

    if (tStartCPs.empty()) {
        xOiDebugPrint(@Warn, 1, "no start points for correction");
	xOiDebugFinishFunc(0);
	return(0);
    }

    var tValidPos = NULL;

    

    xOiDebugPrint(@Info, 2, "trying rotation ...");
    var i, tACP, tADiff, tPrevValid, tRotDir;
    for(i=0; i<tStartCPs.size(); i++) {
	tCP = tStartCPs[i];
	xOiDebugPrint(@Info, 2, ["check start point ", tCP]);

	if (!xOiCheckTopPos(pObj, tCP, @pos)) continue;

	tValidPos = tCP;
	tPrevValid = 1;

	tACP = xOiGetPYRot(pPos, tCP); 
	if (tANP < tACP)
	    tRotDir = @N;
	else
	    tRotDir = @P;
	tADiff = fabs(tANP-tACP);
	if (tADiff > xOiPi) {
	    tADiff = xOi2Pi - tADiff;
	    tRotDir = tRotDir == @P? @N : @P;
	}
	while((tADiff = tADiff/2) > 0.05) { 
	    tCP = xOiGetRotPt3D(pPos, tCP, tADiff, tRotDir);
	    if (xOiCheckTopPos(pObj, tCP, @pos)) {
		tValidPos = tCP;
		if (!tPrevValid) tRotDir = tRotDir == @P? @N : @P;
		tPrevValid = 1;
	    }
	    else {
		if (tPrevValid) tRotDir = tRotDir == @P? @N : @P;
		tPrevValid = 0;
	    }
	}
	xOiDebugPrint(@Info, 2, ["valid pos: ", tValidPos]);

	if (tValidPos != NULL) {
	    pObj.setPosition(tValidPos);
	    xOiDebugFinishFunc(1);
	    return(1);
	}
    }

    

    xOiDebugPrint(@Info, 2, "trying axes ...");
    var tP1, tP2, tPDiff, tFX, tFZ;
    for(i=0; i<tStartCPs.size()-1; i++) {
	tFX = tDirs[i][0];
	tFZ = tDirs[i][1];
	tP1 = pPos;		
	tP2 = tStartCPs[i]; 
	xOiDebugPrint(@Info, 2, ["check start point ", tP2]);
	tPDiff = fabs(tFX*tDX + tFZ*tDZ);
	while((tPDiff = tPDiff/2) > 0.001) {
	    tCP = [tP1[0] + tFX*tPDiff, tP1[1], tP1[2] + tFZ*tPDiff];
	    if (xOiCheckTopPos(pObj, tCP, @pos)) {
		tValidPos = tCP;
		tP1 = tCP;
	    }
	    else
		tP2 = tCP;
	}
	xOiDebugPrint(@Info, 2, ["valid pos: ", tValidPos]);

	if (tValidPos != NULL) {
	    pObj.setPosition(tValidPos);
	    xOiDebugFinishFunc(1);
	    return(1);
	}
    }

    xOiDebugFinishFunc(0);
    return(0);
}
 
public func xOiGetTopPos2(pFather, pBounds, pPos)
{
    var tFW = pFather.getWidth();
    var tFH = pFather.getHeight();
    var tFD = pFather.getDepth();
    var tFO = pFather.getOrigin();
    var tOW = pBounds[1][0]-pBounds[0][0];
    var tOH = pBounds[1][1]-pBounds[0][1];
    var tOD = pBounds[1][2]-pBounds[0][2];
    var tOO = [-pBounds[0][0], -pBounds[0][1], -pBounds[0][2]];

    
    

    if (pPos != NULL && pPos instanceof Vector) {
	var tPos = pPos;
	var tA = 0.0;
	if (pPos.size() == 2) {
	    tA   = pPos[1];
	    tPos = pPos[0];
	}
	
	
	tPos = [pPos[0], tFH - tFO[1], pPos[2]];

	return([tPos, tA]);
    }

    var tY = tFH - tFO[1] + tOO[1];

    

    var tCP = [tFW/2 - tFO[0], tY, tFD/2 - tFO[2]];
    var tX  = tCP[0] - tOW/2 + tOO[0];
    var tZ  = tCP[2] - tOD/2 + tOO[2];

    var tPos = xOiGetRotPt3D(tCP, [tX, tY, tZ], 0.0, @P);

    return([tPos, tA]);
}







public func xOiRelRotateObj(pObj, pRefP, pAxis, pAngle)
{
    var tPos = pObj.getPosition();
    if (xOiGetDistance3(tPos, pRefP) < xOiEps) {
        
        pObj.rotate(pAxis, pAngle);
        return;
    }

    var tOC1, tOC2;	
    var tRC1, tRC2;	
    var tDir;

    if (pAxis == @PX || pAxis == @NX) {
        tOC1 = 1; tOC2 = 2;
        tRC1 = 1; tRC2 = 2;
    }
    else
    if (pAxis == @PY || pAxis == @NY) {
        tOC1 = 0; tOC2 = 2;
        tRC1 = 0; tRC2 = 2;
    }
    else
    if (pAxis == @PZ || pAxis == @NZ) {
        tOC1 = 1; tOC2 = 0;
        tRC1 = 1; tRC2 = 0;
    }
    if (pAxis == @PX || pAxis == @PY || pAxis == @PZ) 
        tDir = @P;
    else
        tDir = @N;
    var tRotP = xOiGetRotPt([pRefP[tRC1], pRefP[tRC2]],
    			    [tPos[tRC1], tPos[tRC2]],
    			    pAngle, tDir);
    
    tPos[tRC1] = tRotP[0];
    tPos[tRC2] = tRotP[1];
    pObj.setPosition(tPos);
    pObj.rotate(pAxis, pAngle);
}










public func xOiRotationMode2Axis(pMode)
{
    xOiDebugStartFunc("xOiFuncs::xOiRotationMode2Axis", pMode);

    if (pMode == NULL  ||
        !(pMode instanceof Int || 
	  typeOf(pMode).subClassOf(::egr::afml::Primitive))) {
	xOiDebugPrint(@Warn, 1, ["invalid parameter: ", pMode]);
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tMode = pMode;
    if (!(pMode instanceof Int))
	tMode = pMode.getRtAxis();

    var tRtMode2Axis = Hash();
    tRtMode2Axis[1] = @PX;
    tRtMode2Axis[2] = @PY;
    tRtMode2Axis[4] = @PZ;

    var tAxis = NULL;

    if (tRtMode2Axis.hasKey(tMode))
	tAxis = tRtMode2Axis[tMode];

    xOiDebugFinishFunc(tAxis);
    return(tAxis);
}












public func xOiSetRotation(pObj, pAxis, pAngle)
{
    xOiDebugStartFunc("xOiFuncs::xOiSetRotation", [pObj, pAxis, pAngle]);

    var tRtAxis = pObj.getRtAxis(); 

    var tCD = pObj.isEnabledCD();

    xOiDebugPrint(@Info, 2, ["collision detection enabled ? ", tCD]);

    if (tCD) pObj.disableCD();

    var tAxis = 1;
    if (pAxis == @PY || pAxis == @NY) 
	tAxis = 2;
    else
    if (pAxis == @PZ || pAxis == @NZ) 
	tAxis = 4;

    pObj.setRtAxis(tAxis);
    pObj.setRecursionState(1);

    try {
         pObj.rotate(pAxis, pAngle-pObj.getRotation(pAxis));
    }
    catch(&e: Error) {
	xOiDebugPrint(@Warn, 1, e.where + ": " + e.what);
    }

    pObj.setRecursionState(0);
    if (tCD) pObj.enableCD();

    pObj.setRtAxis(tRtAxis);

    xOiDebugFinishFunc(NULL);
}







public func xOiSetYRotation(pObj, pAngle)
{
    
    

    xOiDebugStartFunc("xOiFuncs::xOiSetYRotation", [pObj, pAngle]);

    xOiSetRotation(pObj, @PY, pAngle);

    xOiDebugFinishFunc(NULL);
}








public func xOiGetWorldPosition(pObj, ...)
{
    var tObj = pObj[0];
    var tPos = tObj.getPosition();
    if (pObj.size() > 1)
	tPos = pObj[1];

    var tF = tObj;
    while ((tF = tF.getFather()) != NULL) {
	var tFPos = tF.getPosition();
	tPos = [tPos[0]+tFPos[0], tPos[1]+tFPos[1], tPos[2]+tFPos[2]];
    }
    return(tPos);
}












public func xOiGetWorldRotation(pObj, pAxis)
{
    var tRot = pObj.getRotation(pAxis);
    var tObj = pObj.getFather();
    if (tObj == NULL) return(0.0);

    var tF;
    while ((tF = tObj.getFather()) != NULL) {
	tRot += tObj.getRotation(pAxis);
	if (tRot > xOi2Pi)
	    tRot -= xOi2Pi;
	else
	if (tRot < -xOi2Pi)
	    tRot += xOi2Pi;
	tObj = tF;
    }
    return(tRot);
}















public func xOiFileExists(pFileName)
{
    xOiDebugStartFunc("xOiFuncs::xOiFileExists", pFileName);

    var tStream = IFStream(pFileName, "r");
    var tRet = 1;
    if (tStream.fail())
	tRet = 0;
    else
	tStream.close();

    xOiDebugFinishFunc(tRet);
    return(tRet);
}












public func xOiReadFile(pFileName, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiReadFile", pFileName);

    var tFileName = pFileName[0];

    if (tFileName == NULL || tFileName.empty()) {
	xOiDebugPrint(@Warn, 1, "no file name given!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tFile = IFStream(tFileName, "r");

    if (!tFile && tFileName[0] != '/' && tFileName.find(":") < 0 &&
	tFileName.substr(0, 5) != "data/") {
	xOiDebugPrint(@Warn, 1, ["could not open ", tFileName]);
	tFileName = "data/" + tFileName;
	xOiDebugPrint(@Info, 1, ["instead, trying to open ", tFileName]);
	tFile = IFStream(tFileName, "r");
    }
    if (!tFile) {
	xOiDebugPrint(@Warn, 1, ["could not open ", tFileName]);
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }
    var tRet = @();
    while (!tFile.eof()) {
	if (!tFile) {
	    xOiDebugPrint(@Warn, 1, ["Error reading file ", tFileName]);
	    break;
	}
	var tS;
	tFile.gets(tS);
	var tS2 = xOiSubstString(tS, "\r", "\\r");
	xOiDebugPrint(@Info, 2, 
		      ["read line: ", xOiSubstString(tS2, "\n", "\\n")]);
	var tLen = tS.size();
	if (tLen > 0 && tS[tLen-1] == '\r')
	    tS = tS.substr(0, tLen-1);
	if (tLen > 0 && tS[tLen-1] == '\n')
	    tS = tS.substr(0, tLen-1);
	if (tS.find("\r") >= 0)
	    xOiCopyAggr(xOiParseString2List(tS, "\r"), tRet, 1);
	else
	    tRet.pushBack(tS);
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}


















func xOiReadTableEntries(pTable, ...)
{
    var tTable  = pTable[0];
    var tFields = NULL;
    if (pTable.size() > 1)
	tFields = pTable[1];

    var tRet = @();
    var tR = tTable.readFirstEntry();
    while (tR != NULL) {
	var tEntry = tR;
	if (tFields instanceof Int)
	    tEntry = tR[tFields];
	else
	if (tFields instanceof Vector) {
	    tEntry = Vector(0);
	    var tIdx;
	    foreach(tIdx; tFields) tEntry.pushBack(tR[tIdx]);
	}
	tRet.pushBack(tEntry);
	tR = tTable.readNextEntry();
    }
    return(tRet);
}

























public func xOiOpenDataTbl(pName, pPID, pAddPath)
{
    return(xOiOpenDataTbl2(pName, pPID, pAddPath, 
			   xOiFuncs.sControlDataTblStruct));
}























public func xOiOpenDataTbl2(pName, pPID, pAddPath, pTblStruct, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiOpenDataTbl2", [pName, pPID, pAddPath]);

    var tTblStruct = pTblStruct[0];
    var tEbaseName = "ofml";
    if (pTblStruct.size() > 1 &&
	(pTblStruct[1] == NULL || pTblStruct[1] instanceof String))
	tEbaseName = pTblStruct[1];

    var tValidTblTypes = @("FTXT", "CSV");
    var tTblType   = "CSV";
    if (pTblStruct.size() > 2 && pTblStruct[2] instanceof String &&
	tValidTblTypes.find(pTblStruct[2]) >= 0)
	tTblType   = pTblStruct[2];

    var tPlan = xOiPlanning.getPlanning();
    var tLibPath = NULL;
    if (tPlan != NULL && tPlan.hasMember(@getLibPath))
	tLibPath = tPlan.getLibPath(pPID);

    if (tLibPath == NULL) {
	xOiDebugPrint(@Warn, 1, ["no lib path in planning for program ", pPID]);
	tLibPath = xOiGetAppLibPath(pPID);
	if (tLibPath == NULL) {
	    xOiDebugPrint(@Warn, 1, 
			  ["no lib path in registry for program ", pPID]);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}
    }

    var tTblID;
    if (tEbaseName == NULL)
	tTblID = tTblType + " " + tLibPath + " " + pName;
    else
	tTblID = "VAR EBASE," + tTblType + " " + tLibPath + " " + 
		 tEbaseName + " " + pName;

    xOiDebugPrint(@Info, 2, ["table ID: ", tTblID]);
    xOiDebugPrint(@Info, 2, ["table structure: ", tTblStruct]);

    var tTable = xOiTable2(tTblID, tTblStruct);

    if (tTable.open()) {
	var tID = tTable.getID();
        if (xOiFuncs.sDataTables.hasKey(tID)) {
	    tTable = xOiFuncs.sDataTables[tID][0];
	    xOiFuncs.sDataTables[tID][1]++; 
	    xOiDebugPrint(@Info, 1, "using existing handle");
	}
	else
	    xOiFuncs.sDataTables[tID] = [tTable, 1+0];
	xOiDebugFinishFunc([tTable, tTblID]);
	return(tTable);
    }

    xOiDebugPrint(@Warn, 1, ["cannot open table : ", tTblID]);

    
    if (pAddPath != NULL) {
	if (tEbaseName == NULL)
	    tTblID = tTblType + " " + pAddPath + " " + pName;
	else
	    tTblID = "VAR EBASE," + tTblType + " " + pAddPath + " " + 
		     tEbaseName + " " + pName;
	tTable = xOiTable2(tTblID, tTblStruct);
	if (!tTable.open()) {
	    xOiDebugPrint(@Warn, 1, ["cannot open table : ", tTblID]);
	    tTable = NULL;
	}
	else {
	    var tID = tTable.getID();
            if (xOiFuncs.sDataTables.hasKey(tID)) {
	        tTable = xOiFuncs.sDataTables[tID][0];
	        xOiFuncs.sDataTables[tID][1]++; 
	        xOiDebugPrint(@Info, 1, "using existing handle");
	    }
	    else
	        xOiFuncs.sDataTables[tID] = [tTable, 1+0];
	}
    }
    else
	tTable = NULL;

    xOiDebugFinishFunc(tTable == NULL ? NULL : [tTable, tTblID]);

    return(tTable);
}









public func xOiCloseDataTbl(pTable)
{
    xOiDebugStartFunc("xOiFuncs::xOiCloseDataTbl", pTable);

    if (pTable == NULL) {
	xOiDebugFinishFunc(NULL);
	return;
    }

    xOiDebugPrint(@Info, 2, ["data tables: ", xOiFuncs.sDataTables]);

    var tID = pTable.getID();

    xOiDebugPrint(@Info, 2, ["table ID: ", tID]);

    if (xOiFuncs.sDataTables.hasKey(tID)) {
	if (--(xOiFuncs.sDataTables[tID][1]) == 0) {
	    xOiDebugPrint(@Info, 1, ["closing data table ", tID]);
    	    pTable.close();
	    xOiFuncs.sDataTables.remove(tID);
	}
    }
    else
        pTable.close();

    xOiDebugFinishFunc(NULL);
    return;
}




























































































public func xOiReadAddAttachPtDefs(pPID, pArticle)
{
    xOiDebugStartFunc("xOiFuncs::xOiReadAddAttachPtDefs", [pPID, pArticle]);

    var tTbl = NULL;

    if (!xOiFuncs.sHasAddAttPtDefsTbl.hasKey(pPID)) {
	tTbl = xOiOpenDataTbl2(xOiFuncs.sAddAttPtDefsTblName, pPID,
			       xOiGetAppProgInfoDBPath(pPID),
			       xOiFuncs.sAddAttPtDefsTblStruct);
	xOiFuncs.sHasAddAttPtDefsTbl[pPID] = (tTbl != NULL); 
    }

    if (!xOiFuncs.sHasAddAttPtDefsTbl[pPID]) {
	xOiDebugPrint(@Info, 1, "no table for given program");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tCacheKey = String(pPID)+"_"+pArticle;
    xOiDebugPrint(@Info, 3, ["cache key: ", tCacheKey]);

    if (xOiFuncs.sAddAttPtDefsCache.hasKey(tCacheKey)) {
	xOiDebugPrint(@Info, 1, "using cache");
	var tRet = xOiFuncs.sAddAttPtDefsCache[tCacheKey];
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    if (tTbl == NULL)
	
	tTbl = xOiOpenDataTbl2(xOiFuncs.sAddAttPtDefsTblName, pPID,
			       xOiGetAppProgInfoDBPath(pPID),
			       xOiFuncs.sAddAttPtDefsTblStruct);
    if (tTbl == NULL) {
	
	xOiDebugPrint(@Warn, 1, "internal error!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tEntries = 
    tTbl.readNEntriesFor(@(["article", pArticle]),
			 @("number", "attpt_key", "condition", "attpt_pos", 
			   "attpt_yrot", "attpt_dir", "attpt_mode"), 0);
    xOiCloseDataTbl(tTbl);

    if (tEntries == NULL) {
	xOiDebugPrint(@Info, 1, "no table entries for given article");
	xOiFuncs.sAddAttPtDefsCache[tCacheKey] = NULL;
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    
    var tSortFunc = func(pE1, pE2) { return(pE1[0] < pE2[0]); };
    tEntries.sort(tSortFunc);

    var tValidDirs = @(@R, @L, @B, @F, @T);
    var tValidModes = @(@Sibling, @Child);

    var tRet = @();
    var tE;
    foreach(tE; tEntries) {
	xOiDebugPrint(@Info, 2, ["processing entry ", tE]);

	var tAPKey     = xOiStripPadChar(tE[1], ' ', 1);
	var tCondition = xOiStripPadChar(tE[2], ' ', 1);
	var tAPPos     = xOiStripPadChar(tE[3], ' ', 1);
	var tAPRot     = xOiStripPadChar(tE[4], ' ', 1);
	var tAPDir     = xOiStripPadChar(tE[5], ' ', 1);
	var tAPMode    = xOiStripPadChar(tE[6], ' ', 1);

	if (tAPKey.empty() || tAPPos.empty() || tAPRot.empty() ||
	    tAPDir.empty() || tAPMode.empty()) {
	    xOiDebugPrint(@Warn, 1, 
			  "at least one of the mandatory fields is empty!");
	    continue;
	}
	tAPKey = Symbol(tAPKey);
	tAPDir = Symbol(tAPDir);
	if (tValidDirs.find(tAPDir) < 0) {
	    xOiDebugPrint(@Warn, 1, "invalid direction!");
	    continue;
	}
	tAPMode = Symbol(tAPMode);
	if (tValidModes.find(tAPMode) < 0) {
	    xOiDebugPrint(@Warn, 1, "invalid mode!");
	    continue;
	}
	tRet.pushBack([tAPKey, tCondition, tAPPos, tAPRot, tAPDir, tAPMode]);
    }

    xOiFuncs.sAddAttPtDefsCache[tCacheKey] = tRet;

    xOiDebugFinishFunc(tRet);
    return(tRet);
}



















public func xOiEvalAttPtExpression(pObj, pExpr)
{
    xOiDebugStartFunc("xOiFuncs::xOiEvalAttPtExpression", [pObj, pExpr]);

    var tRes = NULL;
    if (!pExpr.empty()) {
	var tExpr = xOiSubstString(pExpr, "self.", pObj.getName()+".");
	xOiDebugPrint(@Info, 2, ["substituted expression: '", tExpr, "'"]);
	try {
	    tRes = eval(tExpr);
	}
	catch(&e: Error) {
	    xOiDebugPrint(@Warn, 1, ["couldn't evaluate expression '", 
				     pExpr, "' for ", pObj, ": ", e.what]);
	}
    }

    xOiDebugFinishFunc(tRes);
    return(tRes);
}













public func xOiIsAttPtCondValid(pObj, pCondition)
{
    xOiDebugStartFunc("xOiFuncs::xOiIsAttPtCondValid", [pObj, pCondition]);

    var tValid = 1;
    if (!pCondition.empty()) {
        var tRes = xOiEvalAttPtExpression(pObj, pCondition);
	tValid = (tRes instanceof Int ? tRes : 0);
    }

    xOiDebugFinishFunc(tValid);
    return(tValid);
}


































































public func xOiReadOppositeAttPts(pPID, pAttPt)
{
    xOiDebugStartFunc("xOiFuncs::xOiReadOppositeAttPts", [pPID, pAttPt]);

    var tTbl = NULL;

    if (!xOiFuncs.sHasOppositeAttPtsTbl.hasKey(pPID)) {
	tTbl = xOiOpenDataTbl2(xOiFuncs.sOppositeAttPtsTblName, pPID,
			       xOiGetAppProgInfoDBPath(pPID),
			       xOiFuncs.sOppositeAttPtsTblStruct);
	xOiFuncs.sHasOppositeAttPtsTbl[pPID] = (tTbl != NULL); 
    }

    if (!xOiFuncs.sHasOppositeAttPtsTbl[pPID]) {
	xOiDebugPrint(@Info, 1, "no table for given program");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tCacheKey = String(pPID)+"_"+String(pAttPt);
    xOiDebugPrint(@Info, 3, ["cache key: ", tCacheKey]);

    if (xOiFuncs.sOppositeAttPtsCache.hasKey(tCacheKey)) {
	xOiDebugPrint(@Info, 1, "using cache");
	var tRet = xOiFuncs.sOppositeAttPtsCache[tCacheKey];
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    if (tTbl == NULL)
	
	tTbl = xOiOpenDataTbl2(xOiFuncs.sOppositeAttPtsTblName, pPID,
			       xOiGetAppProgInfoDBPath(pPID),
			       xOiFuncs.sOppositeAttPtsTblStruct);
    if (tTbl == NULL) {
	
	xOiDebugPrint(@Warn, 1, "internal error!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tEntries = 
    tTbl.readNEntriesFor(@(["attpt_key", String(pAttPt)]),
			 @("direction", "opposites"), 0);
    xOiCloseDataTbl(tTbl);

    if (tEntries == NULL) {
	xOiDebugPrint(@Info, 1, "no table entries for given attach point");
	xOiFuncs.sOppositeAttPtsCache[tCacheKey] = NULL;
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tValidDirs = @(@R, @L, @B, @F, @T);

    var tRet = Vector(0);
    var tE;
    foreach(tE; tEntries) {
	xOiDebugPrint(@Info, 2, ["processing entry ", tE]);

	var tDirField = xOiStripPadChar(tE[0], ' ', 1);
	var tDir = NULL;
	if (!tDirField.empty()) {
	    tDir = Symbol(tDirField);
	    if (tValidDirs.find(tDir) < 0) {
		xOiDebugPrint(@Warn, 1, "invalid direction!");
		continue;
	    }
	}
	var tOppField = xOiStripPadChar(tE[1], ' ', 1);
	if (tOppField.empty()) {
	    xOiDebugPrint(@Warn, 1, "opposites field is empty!");
	    continue;
	}
	var tOpposites = NULL;
	try {
	    tOpposites = eval(tOppField);
	}
	catch(&e: Error) {
	    xOiDebugPrint(@Warn, 1,
			  ["couldn't evaluate opposites field: ", e.what]);
	}
	if (!(tOpposites instanceof List)) {
	    xOiDebugPrint(@Warn, 1, 
			  ["invalid opposites field '", tOppField, "'"]);
	    continue;
	}
	tRet.pushBack([tDir, tOpposites]);
    }

    xOiFuncs.sOppositeAttPtsCache[tCacheKey] = tRet;

    xOiDebugFinishFunc(tRet);
    return(tRet);
}


































public func xOiGetOppositeAttPts(pPID, pAttPt, pDir)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetOppositeAttPts", [pPID, pAttPt, pDir]);

    var tEntries = xOiReadOppositeAttPts(pPID, pAttPt);

    if (tEntries == NULL) {
	xOiDebugPrint(@Info, 1, "no table for given program or no (valid) " +
				"entries for specfied attach point");
	xOiDebugFinishFunc(@());
	return(@());
    }

    xOiDebugPrint(@Info, 2, ["table entries: ", tEntries]);

    var tRet = @();

    if (tEntries.size() == 1) {
	var tE = tEntries[0];
	if (tE[0] == NULL || pDir == NULL ||
	    (tE[0] != NULL && pDir != NULL && tE[0] ==  pDir))
	    tRet = tE[1];
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tE;

    var tNULLEntry = NULL;

    if (pDir == NULL) {
	
        
	var tOtherEntry = NULL;
        foreach(tE; tEntries) {
	    if (tE[0] == NULL) {
		tNULLEntry = tE[1];
		break;
	    }
	    if (tOtherEntry == NULL)
		tOtherEntry = tE[1];
	}
	if (tNULLEntry != NULL)
	    tRet = tNULLEntry;
	else
	if (tOtherEntry != NULL)
	    tRet = tOtherEntry;
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    var tMatchEntry = NULL;
    foreach(tE; tEntries) {
	if (tE[0] != NULL && tE[0] == pDir) {
	    tMatchEntry = tE[1];
	    break;
	}
	if (tE[0] == NULL) {
	    if (tNULLEntry == NULL)
		tNULLEntry = tE[1];
	}
    }
    if (tMatchEntry != NULL)
	tRet = tMatchEntry;
    else
    if (tNULLEntry != NULL)
	tRet = tNULLEntry;

    xOiDebugFinishFunc(tRet);
    return(tRet);
}



























public func xOiInitProgressDialog(pMsg, pNumber, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiInitProgressDialog", [pMsg, pNumber]);

    var tNumber = pNumber[0];
    var tMode = 0;
    if (pNumber.size() > 1)
	
	tMode = pNumber[1];

    if (!(pMsg == NULL || pMsg instanceof String) || 
	!(tNumber instanceof Int)) {
	xOiDebugPrint(@Warn, 1, "wrong parameters!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    
    if (tNumber < 10) {
	xOiDebugPrint(@Info, 1, "less then 10 process elements");
	xOiDebugFinishFunc(0);
	return(0);
    }

    if (xOiFuncs.sProgressDialog != NULL) {
	
	xOiDebugPrint(@Warn, 1, "there is an ongoing progress dialog!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    if (!xOiSendProgressEvent2App(@START, 0, pMsg, tMode)) {
	xOiDebugPrint(@Warn, 1, "application doesn't support progress events!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    
    
    
    var tDialogStepAmount = 1;
    if (tNumber < 100)
	tDialogStepAmount = Int(ceil(100/Float(tNumber)));

    xOiDebugPrint(@Info, 2, 
		  ["dialog step amount (in percent): ", tDialogStepAmount]);

    xOiFuncs.sProgressDialog =	[pMsg, tNumber, tMode, tDialogStepAmount, 
				 0+0, 	
				 0+0];	

    xOiDebugFinishFunc(1);
    return(1);
}












public func xOiProcessProgressStep()
{
    if (xOiFuncs.sProgressDialog == NULL)
	
	return;

    xOiDebugStartFunc("xOiFuncs::xOiProcessProgressStep", NULL);

    var tMsg        = xOiFuncs.sProgressDialog[0];
    var tMode       = xOiFuncs.sProgressDialog[2];

    var tProcessed  = ++xOiFuncs.sProgressDialog[4];
    var tPercent    = Float(tProcessed) / xOiFuncs.sProgressDialog[1] * 100;
    var tStepAmount = xOiFuncs.sProgressDialog[3];
    var tLastStep   = xOiFuncs.sProgressDialog[5];

    if (tPercent >= tLastStep + tStepAmount) {
	xOiFuncs.sProgressDialog[5] = tLastStep += tStepAmount;
	xOiSendProgressEvent2App(@SHOW, tLastStep, tMsg, tMode);
    }

    xOiDebugFinishFunc(NULL);
}









public func xOiFinishProgressDialog()
{
    if (xOiFuncs.sProgressDialog == NULL)
	
	return;

    xOiDebugStartFunc("xOiFuncs::xOiFinishProgressDialog", NULL);

    xOiSendProgressEvent2App(@STOP, 100, NULL, xOiFuncs.sProgressDialog[2]);

    xOiFuncs.sProgressDialog = NULL;

    xOiDebugFinishFunc(NULL);
}







func _xOiCreateArticle(pVariant, pFather, pRefObj, pAttPt, pPID, pArticle, 
		       pVarCode, pCodeType, pMode, pChAttPt, pName, pReplace,
		       pApplyMetaType, pProperties)
{
    xOiDebugStartFunc2("xOiFuncs::_xOiCreateArticle", 
		       [pVariant, pFather, pRefObj, pAttPt, pPID, pArticle, 
			pVarCode, pCodeType, pMode, pChAttPt, pName, pReplace,
			pApplyMetaType, pProperties]);

    if (pFather == NULL) {
	xOiDebugPrint(@Warn, 1, "father object required!");
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tPlan = oiGetPlanning();

    if (tPlan == NULL || !tPlan.isA(xOiBasePlanning)) {
	
	xOiDebugPrint(@Warn, 1, "no global planning instance!?");
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tValidCodeTypes = @(@VarCode, @OFMLVarCode, @Final);
    if (!(pCodeType instanceof Symbol) ||
	tValidCodeTypes.find(pCodeType) < 0) {
	xOiDebugPrint(@Warn, 1, "invalid code type!");
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tValidModes = @(@extended, @lite, @att_pts);
    var tMode = pMode;
    if (!(pMode instanceof Symbol) || tValidModes.find(pMode) < 0) {
	tMode = @extended; 
	xOiDebugPrint(@Warn, 1, "invalid mode! (using @extended)");
    }
    if (tMode == @att_pts) {
	var tError = 0;
	if (pRefObj == NULL || pAttPt == NULL) {
	    xOiDebugPrint(@Warn, 1, "reference object and attach point must " +
				    "be specified for mode @att_pts!");
	    tError = 1;
	}
	else
	if (pRefObj.getFather() != pFather) {
	    xOiDebugPrint(@Warn, 1, "reference object must be an immediate " +
				    "child of pFather in mode @att_pts!");
	    tError = 1;
	}
	else 
	if (!pRefObj.isCat(@IF_AttachPts)) {
	    xOiDebugPrint(@Warn, 1, "class of reference object must implement" +
				    " interface AttachPts for mode @att_pts!");
	    tError = 1;
	}
	if (tError) {
	    xOiDebugFinishFunc(NULL);
	    return;
	}
    }

    var tChAttPt = pChAttPt;
    if (!(pChAttPt == NULL || pChAttPt instanceof Symbol)) {
	tChAttPt = NULL;
	xOiDebugPrint(@Warn, 1, "ignoring invalid child attach point!");
    }

    var tName = pName;
    if (!(pName == NULL || pName instanceof String)) {
	tName = NULL;
	xOiDebugPrint(@Warn, 1, 
		      "ignoring invalid element name (using add())!");
    }

    if (tName != NULL) {
	var tChildName = pFather.getName()+"."+tName;
	if (oiExists(tChildName)) {
	    if (pReplace) {
		xOiDebugPrint(@Info, 1, 
			      ["removing existing child ", tChildName]);
		var tChild = NULL;
		try {
		    tChild = eval(tChildName);
		}
		catch (&e: Error) {
		    
		    xOiDebugPrint(@Warn, 1, "internal error: "+e.what);
		    tName = NULL;
		}
		if (tChild != NULL) {
		    pFather.remove(tChild);
		    if (oiExists(tChildName)) {
			xOiDebugPrint(@Warn, 1, 
				      ["couldn't remove ", tChildName]);
			tName = NULL;
		    }
		}
	    }
	    else {
		xOiDebugPrint(@Warn, 1, 
			      ["child ", tChildName, " already exists!"]);
		tName = NULL;
	    }
	    if (tName == NULL) {
		xOiDebugFinishFunc(NULL);
		return;
	    }
	}
    }
    else
    if (tMode == @att_pts)
	
	
	tName = "e" + String(xOiGetHighestEnameNum(pFather)+1);

    var tPlanPID = tPlan.getProgram();

    xOiDebugPrint(@Info, 1, ["current planning program: ", tPlanPID]);

    if (tPlanPID != pPID) 
	oiRegisterPackage(pPID);

    var tClass = tPlan.article2Class(pArticle, pApplyMetaType);

    if (tClass == NULL) {
	xOiDebugPrint(@Warn, 1, ["no class specified for article ", pArticle]);
	if (tPlanPID != pPID) tPlan.setProgram(tPlanPID);
	xOiDebugFinishFunc(NULL);
	return;
    }

    xOiDebugPrint(@Info, 2, ["article class: ", tClass]); 

    var tType;
    try {
	tType = eval(tClass);
    }
    catch (&e: Error) {
	xOiDebugPrint(@Warn, 1, ["couldn't evaluate type for class '", 
				 tClass, "': "+e.what+"!"]); 
	if (tPlanPID != pPID) tPlan.setProgram(tPlanPID);
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tNewPos;

    if (tMode == @extended) {
	pFather.setPasteMode(@CR);
	pFather.sChDone = 0;
	pFather.sError = NULL;
	pFather.setTempArticleSpec([pArticle, pVarCode, pCodeType]);

	if (pRefObj != NULL && pAttPt != NULL)
	    pRefObj.setActiveAttPt(pAttPt);

	tNewPos = pFather.checkAdd(tType, pRefObj, NULL, 
				   pFather.isA(xOiPlGroup) ? tChAttPt : NULL);

	xOiDebugPrint(@Info, 2, ["checkAdd() pos: ", tNewPos]);

	if (pRefObj != NULL && pAttPt != NULL)
	    pRefObj.setActiveAttPt(NULL); 

	if (tNewPos == NULL) {
	    
	    
	    
	    if (tPlanPID != pPID) tPlan.setProgram(tPlanPID);
	    xOiDebugPrint(@Warn, 1, "insertion not allowed/possible");
	    xOiDebugFinishFunc(NULL);
	    return;
	}
    }

    var tFatherHasPrepare4NewElement = pFather.hasMember(@prepare4NewElement);

    if (tMode == @lite && tFatherHasPrepare4NewElement)
	pFather.prepare4NewElement([NULL, NULL, NULL, NULL]);

    var tEl = NULL;

    if (tName == NULL)
	tEl = pFather.add(tType);
    else
    try {
	tEl = eval(tClass+"(pFather, @"+tName+")");
	if (tMode != @att_pts)
	    
	    tEl.beAnElement();
    }
    catch (&e: Error) {
	xOiDebugPrint(@Warn, 1, ["couldn't create instance of class '", 
				 tClass, "': "+e.what+"!"]); 
	if (tPlanPID != pPID) tPlan.setProgram(tPlanPID);
	xOiDebugFinishFunc(NULL);
	return;
    }

    if (tMode == @att_pts && !tEl.isCat(@IF_AttachPts)) {
	xOiDebugPrint(@Warn, 1, "class of added object must implement " +
				"interface AttachPts for mode @att_pts!");
	pFather.remove(tEl);
	if (tPlanPID != pPID) tPlan.setProgram(tPlanPID);
	xOiDebugFinishFunc(NULL);
	return;
    }

    if (tMode == @extended) {
	tEl.setPosition(tNewPos);

	var tMethod = pFather.getMethod(); 
	if (tMethod != NULL) {
	    try {
		eval("tEl."+tMethod);
	    }
	    catch (&e: Error) {
		xOiDebugPrint(@Warn, 1,  e.where + ": " + e.what);
	    }
	    pFather.clearMethod();
	}
	tEl.callRules(@PICK, tNewPos);
    }

    tEl.setXArticleSpec(@Base, pArticle);
    tEl.setXArticleSpec(pCodeType, pVarCode);

    if (tMode == @att_pts) {
	var tCurrActAttPt = pRefObj.getActiveAttPt();
	pRefObj.setActiveAttPt(pAttPt);

	
	
	
	
	if (pProperties != NULL) {
	    xOiDebugPrint(@Info, 1, "assigning properties... ");
	    var tP;
	    foreach(tP; pProperties) tEl.setPropValue(tP[0], tP[1]);
	}

	var tPR = oiGetPosRot4AttachPts(pRefObj, tEl, tChAttPt, 0);

	xOiDebugPrint(@Info, 2, ["position/rotation: ", tPR]);

	if (tPR != NULL && tPR[0] != NULL && tPR[2] == NULL) {
	    tEl.setPosition(tPR[0]);
	    xOiSetYRotation(tEl, tPR[1]);
	    
	    
	    if (pFather.isA(xOiPlGroup) &&
		(pFather.hasMember(@isBusyAttPt) || 
		 pFather.hasMember(@isFreeAttPt))) {
		var tAttPtIsBusy = (pFather.hasMember(@isBusyAttPt) ?
				    pFather.isBusyAttPt(pRefObj, pAttPt) :
				    !pFather.isFreeAttPt(pRefObj, pAttPt));
		if (tAttPtIsBusy) {
		    tPR = [tPR[0], tPR[1], "::ofml::xoi::@busyattpt"];
		    tPR = pFather.handleInsertion(pRefObj, tEl, tPR);
		    if (tPR[2] != NULL) {
			xOiDebugPrint(@Warn, 1, 
				      ["cannot insert new element between ",
				       pRefObj, " and its current neighbor!"]);
			pFather.remove(tEl);
			tEl = NULL;
		    }
		}
	    }
	    if (tEl != NULL && tFatherHasPrepare4NewElement) {
		var tElBounds = tEl.getLocalGeoBounds();
		var tAttPts = oiGetAttPt4LastPosRot2(pRefObj);
		pFather.prepare4NewElement([pRefObj, tElBounds, 
					    tAttPts[0], tAttPts[1]]);
	    }
	}
	else {
	    pFather.remove(tEl);
	    tEl = NULL;
	}

	pRefObj.setActiveAttPt(tCurrActAttPt);

	if (tEl == NULL) {
	    xOiDebugPrint(@Warn, 1, ["cannot add to attach point ", pAttPt,
				     " of reference ", pRefObj]);
	    if (tPlanPID != pPID) tPlan.setProgram(tPlanPID);
	    xOiDebugFinishFunc(NULL);
	    return;
	}
	tEl.beAnElement();
    }

    if (tMode == @extended) {
	

	tPlan.assignDefaultPropValues(tEl);
	tPlan.articleInserted(tEl, pRefObj, @CR);
    }
    else 
    if (pVariant == @V2) {
	
	tPlan.articleInserted(tEl, (tMode == @lite ? NULL : pRefObj), @CR);
    }

    if (tPlanPID != pPID) tPlan.setProgram(tPlanPID);

    if (tMode != @att_pts && pProperties != NULL) {
	xOiDebugPrint(@Info, 1, "assigning properties... ");
	var tP;
	foreach(tP; pProperties)
	    tEl.setPropValue(tP[0], tP[1]);
    }

    xOiDebugFinishFunc(tEl);
    return(tEl);
}





























































































































public func xOiCreateArticle(pFather, pRefObj, pAttPt, pPID, pArticle, 
			     pVarCode, pCodeType, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiCreateArticle", 
		      [pFather, pRefObj, pAttPt, pPID, pArticle, 
		       pVarCode, pCodeType]); 

    var tCodeType = pCodeType[0];

    var tMode = @extended;
    if (pCodeType.size() > 1)
	tMode = pCodeType[1];

    var tChAttPt = NULL;
    if (pCodeType.size() > 2 && pCodeType[2] instanceof Symbol)
	tChAttPt = pCodeType[2];

    var tEl = _xOiCreateArticle(@V1, pFather, pRefObj, pAttPt, pPID, pArticle, 
			        pVarCode, tCodeType, tMode, tChAttPt, NULL, 0,
			        1, NULL);
    xOiDebugFinishFunc(tEl);
    return(tEl);
}


































































public func xOiCreateArticle2(pFather, pRefObj, pAttPt, pPID, pArticle, 
			      pVarCode, pCodeType, pMode, pChAttPt, pName, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiCreateArticle2", 
		      [pFather, pRefObj, pAttPt, pPID, pArticle, 
		       pVarCode, pCodeType, pMode, pChAttPt, pName]); 

    var tName = pName[0];

    var tApplyMetaType = 1;
    if (pName.size() > 1) {
	if (pName[1] instanceof Int)
	    tApplyMetaType = pName[1]; 
	else
	    xOiDebugPrint(@Warn, 1, 
			  "invalid 1st optional parameter! (using 1)");
    }

    var tProperties = NULL;
    if (pName.size() > 2) {
	if (pName[2] instanceof Vector)
	    tProperties = pName[2]; 
	else
	if (pName[2] != NULL)
	    xOiDebugPrint(@Warn, 1, "ignoring invalid 2nd optional parameter!");
    }

    var tReplace = 0;
    if (pName.size() > 3) {
	if (pName[3] instanceof Int)
	    tReplace = pName[3]; 
	else
	    xOiDebugPrint(@Warn, 1, 
			  "invalid 3rd optional parameter! (using 1)");
    }
 
    var tEl = _xOiCreateArticle(@V2, pFather, pRefObj, pAttPt, pPID, pArticle, 
			        pVarCode, pCodeType, pMode, pChAttPt, tName,
			        tReplace, tApplyMetaType, tProperties);
    xOiDebugFinishFunc(tEl);
    return(tEl);
}











































public func xOiCloneArticle(pFather, pSrc, pRefObj, pAttPt, ...)
{
    xOiDebugStartFunc("xOiFuncs::xOiCloneArticle", 
		      [pFather, pSrc, pRefObj, pAttPt]);

    if (pFather == NULL) {
	xOiDebugPrint(@Warn, 1, "father object required!");
	xOiDebugFinishFunc(NULL);
	return;
    }

    var tName = NULL;
    if (pAttPt.size() > 1 && pAttPt[1] instanceof String)
	tName = pAttPt[1];

    if (tName == NULL && pSrc.isAnElement()) {
	tName = "e" + String(xOiGetHighestEnameNum(pFather)+1);
	xOiDebugPrint(@Info, 1, ["using name ", tName]);
    }

    if (tName == NULL) {
	xOiDebugPrint(@Warn, 1, "no name for clone!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tCloneAttPt = NULL;
    if (pAttPt.size() > 2 && pAttPt[2] instanceof Symbol)
	tCloneAttPt = pAttPt[2];

    var tClone = NULL;
    try {
	tClone = oiClone(pSrc, pFather.getName()+"."+tName);
    }
    catch (&e: Error) {
	xOiDebugPrint(@Warn, 1, e.where+": "+e.what);
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tPR = NULL;

    if (pRefObj != NULL && pRefObj.getFather() == pFather &&
	pRefObj.isCat(@IF_AttachPts) && tClone.isCat(@IF_AttachPts)) {
	var tCurrActAttPt = NULL;
	if (pAttPt instanceof Symbol) {
	    tCurrActAttPt = pRefObj.getActiveAttPt();
	    pRefObj.setActiveAttPt(pAttPt);
	}
	tPR = oiGetPosRot4AttachPts(pRefObj, tClone, tCloneAttPt, 0);
	if (pAttPt instanceof Symbol)
	    pRefObj.setActiveAttPt(tCurrActAttPt);
    }
    else 
    if (pFather.isCat(@IF_AttachPts) && tClone.isCat(@IF_AttachPts))
	tPR = oiGetPosRot4AttachPts(pFather, tClone, tCloneAttPt, 0);

    xOiDebugPrint(@Info, 2, ["position/rotation: ", tPR]);

    if (tPR != NULL && tPR[0] != NULL) {
	tClone.setPosition(tPR[0]);
	xOiSetYRotation(tClone, tPR[1]);
    }

    xOiDebugFinishFunc(tClone);
    return(tClone);
}














public func xOiAutoDecoration4Obj(pObj, pContext, pTemplate)
{
    return(xOiAutoDecoration().placeAccOnObject(pObj, pContext, pTemplate));
}







public func xOiDeleteAutoDeco(pObj, ...)
{
    var tObj = pObj[0];
    var tContext = NULL;
    if (pObj.size() > 1)
	tContext = pObj[1];

    xOiAutoDecoration().deleteAccessories(tObj, tContext);
}




































































public func xOiGetMetaInfo(pObj, pInfoType, pArgs)
{
    xOiDebugStartFunc("xOiFuncs::xOiGetMetaInfo", [pObj, pInfoType, pArgs]);

    if (pInfoType != @AutoDecoration) {
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tPID = pObj.getProgram();
    var tTbl = xOiOpenDataTbl("metainfo", tPID, xOiGetAppProgInfoDBPath(tPID));

    if (tTbl == NULL) {
	xOiDebugPrint(@Warn, 1, "could not open metainfo table!");
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tInfoType = "@"+String(pInfoType);

    var tEntries = tTbl.readNEntriesFor(@(["type", tInfoType]),
					    @("args", "value"), 0);
    if (tEntries == NULL) {
	xOiDebugPrint(@Warn, 1, ["no entry in metainfo table for info type ",
			         pInfoType]);
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tValidAutoDecoConds = @("@Cat", "@Article", "@ArticleNOT", "@Type");
    var tContext = NULL;
    if (pArgs != NULL)
	tContext = String(pArgs).toLower();
    var tMEntries = @();
    var tEntry;
    foreach(tEntry; tEntries) {
	xOiDebugPrint(@Info, 2, ["check entry ", tEntry]);
	var tArgs = @();
	var tTokens = xOiParseString2List(tEntry[0], ['[',']'], 0);
	var tToken;
	foreach(tToken; tTokens) {
	    var tCV = xOiParseString2List(tToken, ',', 1);
	    tArgs.pushBack(tCV);
	}
	tTokens = xOiParseString2List(tEntry[1], ',', 1);
	var tValueV = _xOiGetAutoDecoValues(pObj, tTokens);
	if (tValueV == NULL) {
	    xOiDebugPrint(@ExplWarn, 1, ["invalid value '", tEntry[1],
					 "' for info type ", pInfoType]);
	    continue;
	}

	var tTemplCat = String(tValueV[3]).toLower();
	var tColons = tTemplCat.find("::");
	if (tColons < 0 || 
	    (tContext != NULL && tTemplCat.substr(0, tColons) != tContext)) {
	    xOiDebugPrint(@Info, 2, "template category does not match context");
	    continue;
	}
	var tMatch = 1;
	var tArg, i;
	foreach(tArg; tArgs) {
	    if (tValidAutoDecoConds.find(tArg[0]) < 0) {
		    tMatch = 0; break;
	    }
	    if (tArg[0] == "@Cat") {
		var tOk = 0;
		for(i=1; i < tArg.size(); i++) {
		    var tCat;
		    try { tCat = eval(tArg[i]); }
		    if (tCat != NULL && pObj.isCat(tCat)) { tOk = 1; break; }
		}
		if (!tOk) { tMatch = 0; break; }
	    }
	    if (tArg[0] == "@Article" || tArg[0] == "@ArticleNOT") {
		var tArtSpec = pObj.getArticleSpec();
		var tOk = (tArg[0] == "@Article") ? 0 : 1;
		for(i=1; i < tArg.size(); i++) {
		    if (tArg[i] == tArtSpec) {
			tOk = (tArg[0] == "@Article") ? 1 : 0; 
			break; 
		    }
		}
		if (!tOk) { tMatch = 0; break; }
	    }
	    if (tArg[0] == "@Type") {
		var tOk = 0;
		for(i=1; i < tArg.size(); i++) {
		    var tType;
		    try { tType = eval(tArg[i]); }
		    if (tType != NULL && pObj.isA(tType)) { tOk = 1; break; }
		}
		if (!tOk) { tMatch = 0; break; }
	    }
	}
	xOiDebugPrint(@Info, 2, ["conditions match? ", tMatch]);

	if (tMatch) tMEntries.pushBack(tValueV);
    }

    var tRet = NULL;

    var tCount = tMEntries.size();
    if (tCount == 0)
	xOiDebugPrint(@Warn, 1, ["no matching entry for ", pInfoType]);
    else
    if (tCount > 1)
	xOiDebugPrint(@Warn, 1, ["ambigues entries for ", pInfoType]);
    else
	tRet = tMEntries.front();

    xOiDebugFinishFunc(tRet);
    return(tRet);
}

func _xOiGetAutoDecoValues(pObj, pTokens)
{
    var tTCnt = pTokens.size();
    if (tTCnt != 1 && tTCnt != 4) return(NULL);

    var tTokens = pTokens;
    if (tTCnt == 1)
	tTokens = @("$auto"+"", "$auto"+"", "$auto"+"", pTokens[0]);

    if (tTokens[0] == "$auto")
	tTokens[0] = "$self.getWidth()";
    if (tTokens[1] == "$auto")
	tTokens[1] = "$self.getHeight()-$self.getOrigin()[1]";
    if (tTokens[2] == "$auto")
	tTokens[2] = "$self.getDepth()";

    var tValueV = Vector(4);
    var tObjName = pObj.getName();

    var i;
    for(i=0; i<3; i++) {
	try {
	    tValueV[i] = eval(xOiSubstString(tTokens[i], "$self", tObjName));
	}
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 2, e.where + ": " + e.what);
	    return(NULL);
	}
    }
    tValueV[3] = tTokens[3];

    return(tValueV);
}








public func xOiPrepareVarCode(pObj, pSpec)
{
    xOiDebugStartFunc("xOiFuncs::xOiPrepareVarCode", [pObj, pSpec]);

    var tSpec = String(pSpec);
    var tPDM = pObj.getPDManager();
    var tPDB = tPDM.getPDBFor(pObj);

    var tBaseSpec = pObj.getArticleSpec();
    xOiDebugPrint(@Info, 2, ["base spec: ", tBaseSpec]);

    if (tBaseSpec == NULL)  {
	xOiDebugPrint(@Warn, 1, "NULL base article number!");
        xOiDebugFinishFunc(tSpec);
        return(tSpec);
    }

    var tBaseLen = tBaseSpec.size();
    var tSpecLen = pSpec.size();

    if (tPDB != NULL && 
	((tPDB.isA(xOiEPDFProductDB) || tPDB.isA(xOiNativeOCDProductDB)) && 
	  tPDB.getVarCodeType(tBaseSpec) != @UserDefined) && 
	(tBaseLen > tSpecLen || pSpec.substr(0, tBaseLen) != tBaseSpec)) {
	var tSep = "";
	if (tPDB.isA(xOiEPDFProductDB) || tPDB.isA(xOiNativeOCDProductDB))
	    tSep = tPDB.getSpecSeparator(tBaseSpec);
	tSpec = tBaseSpec + tSep + pSpec;
    }
    else
    if (tPDB != NULL && 
	(tPDB.isA(xOiEPDFProductDB) || tPDB.isA(xOiNativeOCDProductDB)) &&
	tPDB.getVarCodeType(tBaseSpec) == @Complete &&
	(tBaseLen != tSpecLen || tBaseSpec != pSpec)) {
	tSpec = tBaseSpec + tPDB.getSpecSeparator(tBaseSpec) + pSpec;
    }

    xOiDebugFinishFunc(tSpec);
    return(tSpec);
}


















public func xOiCollectPrices(pEL, pPrice, pLanguage, pCurrency, pRecursive)
{
    xOiDebugStartFunc("xOiFuncs::xOiCollectPrices", 
		      [pEL, pPrice, pLanguage, pCurrency, pRecursive]);

    var tE;
    foreach(tE; pEL) {
	var tPriceL = tE.getArticlePrice(pLanguage, pCurrency);
	xOiDebugPrint(@Info, 2, ["price of element ", tE, " is ", tPriceL]);
	if (tPriceL != NULL) {
	    if (pPrice.empty()) {
		    pPrice.pushBack(tPriceL.front());
		    pPrice.pushBack([""+"", 0.0+0, 0.0+0]);
	    }
	    var tAccPrice = pPrice.back();
	    var tAddPrice = tPriceL.back();
	    tAccPrice[1] += tAddPrice[1];
	    tAccPrice[2] += tAddPrice[2];
	}
	if (pRecursive)
	    xOiCollectPrices(tE.getElements(), pPrice, pLanguage, pCurrency, 1);
    }

    xOiDebugPrint(@Info, 1, ["(modified) result: ", pPrice]);
    xOiDebugFinishFunc(NULL);
}









public func xOiCollision(pObj) 
{
    return(xOi_Collision(pObj, NULL));
}

public func xOi_Collision(pObj, pObj2) 
{
    var tF = pObj.getFather();

    
    if (tF == NULL) return(0);

    if (tF.isA(OiPlElement) || tF.isA(OiPart)) 
	return(tF.checkChildColl(pObj, pObj2) != NULL);

    var tCh;
    foreach(tCh; tF.getChildren()) 
        if (tCh == pObj) continue;
        else 
        if (oiCollision(tCh, pObj)) return(1);

    return(xOi_Collision(tF, pObj2));
}














public func xOiElRefs2ElIdcs(pObj, pRefList)
{
    xOiDebugStartFunc("xOiFuncs::xOiElRefs2ElIdcs", pObj);

    if (pRefList == NULL || 
	!(pRefList instanceof List || pRefList instanceof Vector)) {
        xOiDebugPrint(@Warn, 1, ["invalid parameter pRefList: ", pRefList]);
        xOiDebugFinishFunc(NULL);
        return(NULL);
    }

    xOiDebugPrint(@Info, 1, ["count of objects: ", pRefList.size()]);

    var tEL = pObj.getElements();
    var tRetL = @();
    var tE;
    foreach(tE; pRefList) {
	var tIdx = xOiObjPosInList(tE, tEL);
	if (tIdx == -1) {
	    tRetL = NULL;
	    break;
	}
	else
	    tRetL.pushBack(tIdx);
    }
    xOiDebugPrint(@Info, 1, ["indices: ", tRetL == NULL ? NULL : tRetL.size()]);
    xOiDebugFinishFunc(NULL);
    return(tRetL);
}








public func xOiElIdcs2ElRefs(pObj, pIdxList)
{
    xOiDebugStartFunc("xOiFuncs::xOiElIdcs2ElRefs", pObj);

    if (pIdxList == NULL || 
	!(pIdxList instanceof List || pIdxList instanceof Vector)) {
        xOiDebugPrint(@Warn, 1, ["invalid parameter pIdxList: ", pIdxList]);
        xOiDebugFinishFunc(NULL);
        return(NULL);
    }

    xOiDebugPrint(@Info, 1, ["count of indices: ", pIdxList.size()]);
    xOiDebugPrint(@Info, 2, ["indices: ", pIdxList]);

    var tEL = pObj.getElements();
    xOiDebugPrint(@Info, 3, ["elements: ", tEL]);

    var tMaxIdx = tEL.size() - 1;
    var tRetL = @();
    var i;
    foreach(i; pIdxList) {
	if (i < 0 || i > tMaxIdx) {
	    tRetL = NULL;
	    break;
	}
	else 
	    tRetL.pushBack(tEL[i]);
    }
    xOiDebugPrint(@Info, 1, ["objects: ", tRetL == NULL ? NULL : tRetL.size()]);
    xOiDebugFinishFunc(NULL);
    return(tRetL);
}







public func xOiObjRefs2Names(pObjList)
{
    var tRetL = @();
    var tO;
    foreach(tO; pObjList) 
	tRetL.pushBack(tO.getName());

    return(tRetL);
}











public func xOiObjNames2Refs(pObjList)
{
    var tRetL = @();
    var tO;
    foreach(tO; pObjList) 
	if (oiExists(tO)) tRetL.pushBack(eval(tO));

    return(tRetL);
}











public func xOiNoCreation(pObj, pType, pArg) 
{
    return(-1);
}





public func xOiExclCreation(pObj, pType, pExclType) 
{
    if (pType == pExclType) return(0);

    return(-1);
}











public func xOiNoDelete(pObj, pDObj, pArg) 
{
    return(-1);
}




public func xOiExclDelete(pObj, pDObj, pExclType) 
{
    var tDObj = $pDObj;
    if (tDObj.isA(pExclType)) return(0);

    return(-1);
}


















public func xOiRotateLY(pObj, pOld, pArc, pObj2, pFather)
{
    var tR  = pObj.getRotation(@PY);

    if (tR > 0) 
	pObj.rotate(@PY, -tR);
    else 
    if (tR < -pArc) 
	pObj.rotate(@PY, -pArc-tR);

    var tObj = pFather;
    if (tObj == NULL) tObj = pObj;
    if (xOi_Collision(tObj, pObj2))
        pObj.rotate(@PY, pOld-pObj.getRotation(@PY));

    return(0);
}










public func xOiRotateRY(pObj, pOld, pArc, pObj2, pFather)
{
    var tR  = pObj.getRotation(@PY);

    if (tR < 0)	
	pObj.rotate(@PY, -tR);
    else 
    if (tR > pArc)
	pObj.rotate(@PY, pArc-tR);

    var tObj = pFather;
    if (tObj == NULL) tObj = pObj;
    if (xOi_Collision(tObj, pObj2))
        pObj.rotate(@PY, pOld-pObj.getRotation(@PY));

    return(0);
}










public func xOiRotateXUp(pObj, pOld, pArc, pObj2, pFather)
{
    var tR  = pObj.getRotation(@PX);

    if (tR > 0)
	pObj.rotate(@PX, -tR);
    else 
    if (tR < -pArc)
	pObj.rotate(@PX, -pArc-tR);

    var tObj = pFather;
    if (tObj == NULL) tObj = pObj;
    if (xOi_Collision(tObj, pObj2))
        pObj.rotate(@PX, pOld-pObj.getRotation(@PX));

    return(0);
}










public func xOiRotateXDown(pObj, pOld, pArc, pObj2, pFather)
{
    var tR  = pObj.getRotation(@PX);

    if (tR < 0)	
	pObj.rotate(@PX, -tR);
    else 
    if (tR > pArc)
	pObj.rotate(@PX, pArc-tR);

    var tObj = pFather;
    if (tObj == NULL) tObj = pObj;
    if (xOi_Collision(tObj, pObj2))
        pObj.rotate(@PX, pOld-pObj.getRotation(@PX));

    return(0);
}




public func xOiCheckRotation(pObj, pOldRot, pUnused) 
{
    if (xOiIsAComplex(pObj.getFather())) {
        var tRA = pObj.getRtAxis();
        var tAx = @PX;
        if (tRA & 2) 
	    tAx = @PY;
        else
	if (tRA & 4) 
	    tAx = @PZ;

        if (pObj.getFather().checkElPos(pObj, NULL) == 0)
    	    pObj.rotate(tAx, pOldRot-pObj.getRotation(tAx));
    }

    return(0);
}




public func xOiNoRotation(pObj, pOldRot, pUnused)
{
    var tRA = pObj.getRtAxis();
    var tAx = @PX;
    if (tRA & 2) tAx = @PY;
    else
    if (tRA & 4) tAx = @PZ;

    pObj.rotate(tAx, pOldRot-pObj.getRotation(tAx));

    return(0);
}











public func xOiCheckTranslation(pObj, pOldPos, pUnused) 
{
    if (xOiIsAComplex(pObj.getFather())) {
        if (pObj.getFather().checkElPos(pObj, pOldPos) == 0)
        pObj.setPosition(pOldPos);
    }
    return(0);
}
 



public func xOiNoTranslation(pObj, pOldPos, pUnused) 
{
    pObj.setPosition(pOldPos);
    return(0);
}





public func xOiTrXRange(pObj, pOldPos, pMinMax) 
{
    var tPos = pObj.getPosition();
    var tMin = pMinMax[0];
    var tMax = pMinMax[1];
    
    if (tPos[0] < tMin+0.05) pObj.setPosition([tMin, tPos[1], tPos[2]]);
    else 
    if (tPos[0] > tMax-0.05) pObj.setPosition([tMax, tPos[1], tPos[2]]);
    
    if (xOiCollision(pObj)) pObj.setPosition(pOldPos);

    return(0);
}





public func xOiTrYRange(pObj, pOldPos, pMinMax) 
{
    var tPos = pObj.getPosition();
    var tMin = pMinMax[0];
    var tMax = pMinMax[1];
    
    if (tPos[1] < tMin+0.05) pObj.setPosition([tPos[0], tMin, tPos[2]]);
    else 
    if (tPos[1] > tMax-0.05) pObj.setPosition([tPos[0], tMax, tPos[2]]);
    
    if (xOiCollision(pObj)) pObj.setPosition(pOldPos);

    return(0);
}





public func xOiTrZRange(pObj, pOldPos, pMinMax) 
{
    var tPos = pObj.getPosition();
    var tMin = pMinMax[0];
    var tMax = pMinMax[1];
    
    if (tPos[2] < tMin+0.05) pObj.setPosition([tPos[0], tPos[1], tMin]);
    else 
    if (tPos[2] > tMax-0.05) pObj.setPosition([tPos[0], tPos[1], tMax]);
    	
    if (xOiCollision(pObj)) pObj.setPosition(pOldPos);

    return(0);
}




public func xOiTrXRaster(pObj, pOldPos, pR) 
{
    var tPos = pObj.getPosition();
    
    pObj.setPosition([pR*Int(tPos[0]/pR+0.5), tPos[1], tPos[2]]);

    try {
        if (pObj.getFather().outside(pObj)) {
            pObj.setPosition(pOldPos);
            return(0);
        }
    }
    
    if (xOiCollision(pObj)) pObj.setPosition(pOldPos);

    return(0);
}




public func xOiTrYRaster(pObj, pOldPos, pR) 
{
    var tPos = pObj.getPosition();
    
    pObj.setPosition([tPos[0], pR*Int(tPos[1]/pR+0.5), tPos[2]]);

    try {
        if (pObj.getFather().outside(pObj)) {
            pObj.setPosition(pOldPos);
            return(0);
        }
    }
    
    if (xOiCollision(pObj)) pObj.setPosition(pOldPos);

    return(0);
}




public func xOiTrZRaster(pObj, pOldPos, pR) 
{
    var tPos = pObj.getPosition();
    
    pObj.setPosition([tPos[0], tPos[1], pR*Int(tPos[2]/pR+0.5)]);

    try {
        if (pObj.getFather().outside(pObj)) {
            pObj.setPosition(pOldPos);
            return(0);
        }
    }
    
    if (xOiCollision(pObj)) pObj.setPosition(pOldPos);

    return(0);
}





































func xOiGetMatchingEntryValue(pTbl, pEntryType, pArticle, pDataType, 
			      pValidVals, ...)
{
    xOiDebugStartFunc2("xOiFuncs::xOiGetMatchingEntryValue", 
		       [pEntryType, pArticle, 
			pDataType == NULL ? NULL : pDataType.getName(), 
			pValidVals]);

    xOiDebugPrint(@Info, 2, ["table: ", pTbl.getID()]);

    var tEntries = pTbl.readNEntriesFor(@(["type", pEntryType]),
					@("args", "value"), 0);
    if (tEntries == NULL) {
	xOiDebugPrint(@Warn, 1, ["no entries for ", pEntryType]);
	xOiDebugFinishFunc(NULL);
	return(NULL);
    }

    xOiDebugPrint(@Info, 2, ["entries: ", tEntries]);

    var tValidVals = pValidVals[0];
    var tNeedPropCond = 0;
    if (pValidVals.size() > 1 && pArticle != NULL)
	tNeedPropCond = pValidVals[1];

    var tArtCondTypes  = @(@Article, @ArticleX, @ArticleNOT);
    var tPropCondTypes = @(@Property, @PropertyNOT);

    
    
    
    
    var tEntries2 = Vector(0);	
    var tValue    = NULL;	
    var tEmptyArg = 0;
    var tE;
    foreach(tE; tEntries) {
	var tArgs = tE[0];
	if (tArgs.empty()) {
	    if (tEmptyArg) {
		xOiDebugPrint(@Warn, 1, 
			      ["multiple entries without argument(s) for ", 
			       pEntryType, " -> using first entry!"]);
		continue;
	    }
	    tEmptyArg = 1;
	    if (tNeedPropCond) {
		var tVal = tE[1];
		tEntries2.pushBack([[], tVal]);
	    }
	    else
		tValue = tE[1];
	    continue;
	}

	xOiDebugPrint(@Info, 2, ["check entry with argument(s): ", tArgs]);

	if (pArticle == NULL) {
	    xOiDebugPrint(@Info, 2, "-> ignored ...");
	    continue;
	}

	try {
	    tArgs = eval("["+tArgs+"]");
	}
	catch(&e: Error) {
	    xOiDebugPrint(@Warn, 1, ["could not evaluate argument(s) ", 
				     tE[0], ": ", e.what]);
	    continue;
	}

	var tPropArgs = Vector(0); 
	var tHasArtArg = 0;
	var tArg;
	foreach(tArg; tArgs) {
	    if (!(tArg instanceof Vector) || tArg.size() != 2 ||
		!(tArg[0] instanceof Symbol) ||
		!(tArtCondTypes.find(tArg[0]) >= 0 ||
		  tPropCondTypes.find(tArg[0]) >= 0) ||
		!(tArg[1] instanceof Vector || tArg[1] instanceof List)) {
		xOiDebugPrint(@Warn, 1, ["invalid argument(s): ", tArg]);
		continue;
	    }

	    if (tPropCondTypes.find(tArg[0]) >= 0) {
		if (tNeedPropCond)
		    tPropArgs.pushBack(xOiCopyAggr(tArg, NULL, 1));
		else
		    xOiDebugPrint(@Warn, 1, ["ignoring prop condition ", tArg]);
		continue;
	    }

	    if (tHasArtArg) {
		xOiDebugPrint(@Warn, 1, 
			      ["ignoring odd article condition: ", tArg]);
		continue;
	    }

	    tHasArtArg = 1;

	    var tA, tArtMatch = 0;
	    foreach(tA; tArg[1]) {
		if (!(tA instanceof String)) {
		    xOiDebugPrint(@Warn, 1, 
			          ["ignoring non-String condition value: ", 
				   tA]);
		    continue;
		}
		if ((tArg[0] == @ArticleX && pArticle == tA) ||
		    (tArg[0] != @ArticleX && 
		    pArticle.substr(0, tA.size()) == tA)) {
		    tArtMatch = 1;
		    break;
		}
	    }
	    if (((tArg[0] == @Article || tArg[0] == @ArticleX) && tArtMatch) ||
	        (tArg[0] == @ArticleNOT && !tArtMatch)) {
	        tValue = tE[1];
	        continue;
	    }
	}

	if (tValue != NULL) {
	    if (!tNeedPropCond) break;
	    tEntries2.pushBack([tPropArgs, tValue]);
	}
	else
	if (!tHasArtArg && tNeedPropCond) {
	    var tVal = tE[1];
	    tEntries2.pushBack([tPropArgs, tVal]);
	}
    }

    if (!tNeedPropCond) {
	if (tValue == NULL || pDataType == NULL) {
	    if (tValue == NULL)
		xOiDebugPrint(@Info, 1, 
			      ["no (matching) entry for ", pEntryType]);
	    xOiDebugFinishFunc(tValue);
	    return(tValue);
	}
	var tCValue = _xOiCheckDataTableValue(tValue, pDataType, tValidVals);
	xOiDebugFinishFunc(tCValue);
	return(tCValue);
    }

    var tRet = Vector(0);

    foreach(tE; tEntries2) {
	xOiDebugPrint(@Info, 2, 
		      ["processing entry with prop condition(s): ", tE]);
	var tConds = tE[0];
	var tValue = tE[1];
	if (pDataType != NULL)
	    tValue = _xOiCheckDataTableValue(tValue, pDataType, tValidVals);
	if (tValue == NULL)
	    xOiDebugPrint(@Warn, 1, 
			  ["ignoring entry with invalid value: ", tE]);
	else
	    tRet.pushBack([tValue, tConds]);
    }

    xOiDebugFinishFunc(tRet);
    return(tRet);
}

func _xOiCheckDataTableValue(pValue, pDataType, pValidVals)
{
    var tCValue;
    try {
	tCValue = eval(pValue);
    }
    catch (&e: Error) {
	xOiDebugPrint(@ExplWarn, 1, 
		      ["could not convert value ", tCValue, " (", e.what, ")"]);
    }
 
    var tValIsSimple = (tCValue instanceof Int || 
			tCValue instanceof Symbol ||
			tCValue instanceof String);
    if (tCValue != NULL && 
	(!(tCValue instanceof pDataType) ||
	 (tValIsSimple && pValidVals instanceof List && 
	  pValidVals.find(tCValue) < 0)))
	tCValue = NULL;

    return(tCValue);
}

xOiMath();
xOiAutoDecoration();

