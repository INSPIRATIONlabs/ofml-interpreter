// === ofml/oi/oifuncs.cls ===
package ::ofml::oi;

import ::stdio::*;
import ::egr::found::*;









class OiFuncs
{
    static var sSuperClasses = Hash();
    static var sDebug = 0;

    static var sNeighborEps = 0.001;

    static var sHasEgrAppFuncGetProductDBPath = -1;
    static var sHasOfmlAppFuncGetProductDBPath = -1;
    static var sHasOfmlAppFuncGetLibPath = -1;
    static var sHasOfmlAppFuncGetProgInfoDBPath = -1;
    static var sHasOfmlAppFuncGetOAMPath = -1;
    static var sHasOfmlAppFuncGetManufacturerID = -1;
    static var sHasOfmlAppFuncGetSeriesID = -1;
    static var sHasOfmlAppFuncGetProgState = -1;
    static var sHasEgrAppFuncGetRegistryKey = -1;
    static var sHasOfmlAppFuncGetRegistryKey = -1;
    static var sHasOfmlAppFuncGetPDLanguage = -1;

    
    static var sAppProductDBPath = NULL;
    static var sAppProgInfoDBPath = NULL;
    static var sAppLibPath = NULL;
    static var sAppOAMPath = NULL;

    static var sProgInfoObjCache;

    
    static var sDataTblStruct = @(["type",  0, 0+0, @s, ""+"", "0"],
				  ["args",  0, 0+0, @s, ""+"", "0"],
				  ["value", 0, 0+0, @s, ""+"", "0"]);

    
    static var sDecomposedChoiceLists;

    static var 
    sSavedMigratablePropStatesDynProp = @__OI_SAVED_MIGRATABLE_PROP_STATES;
}

func oiMaterial2RGB(pMat)
{
    oiDebugStartFunc("OiFuncs::oiMaterial2RGB", pMat);

    var tDefault = [0, 0, 0]; 
    var tPos1 = pMat.find("amb");
    if (tPos1 < 0) {
        oiDebugFinishFunc(tDefault);
	return(tDefault);
    }
    tPos1 = pMat.findFirstNotOf(' ', tPos1+3);
    if (tPos1 < 0) {
        oiDebugFinishFunc(tDefault);
	return(tDefault);
    }
    var tPos2 = pMat.find(' ', tPos1);
    if (tPos2 < 0) {
        oiDebugFinishFunc(tDefault);
	return(tDefault);
    }
    var tRed = pMat.substr(tPos1, tPos2-tPos1);
    oiDebugPrint(@Info, 2, ["red: '", tRed, "'"]);
    tPos1 = pMat.findFirstNotOf(' ', tPos2);
    if (tPos1 < 0) {
        oiDebugFinishFunc(tDefault);
	return(tDefault);
    }
    tPos2 = pMat.find(' ', tPos1);
    if (tPos2 < 0) {
        oiDebugFinishFunc(tDefault);
	return(tDefault);
    }
    var tGreen = pMat.substr(tPos1, tPos2-tPos1);
    oiDebugPrint(@Info, 2, ["green: '", tGreen, "'"]);
    tPos1 = pMat.findFirstNotOf(' ', tPos2);
    if (tPos1 < 0) {
        oiDebugFinishFunc(tDefault);
	return(tDefault);
    }
    tPos2 = pMat.find(';', tPos1);
    if (tPos2 < 0) tPos2 = pMat.size();
    var tBlue = pMat.substr(tPos1, tPos2-tPos1);
    oiDebugPrint(@Info, 2, ["blue: '", tBlue, "'"]);

    var tRet = [Float(tRed), Float(tGreen), Float(tBlue)];

    oiDebugFinishFunc(tRet);
    return(tRet);
}







func oiOpenDataTbl(pName, pPID, pAddPath)
{
    oiDebugStartFunc("OiFuncs::oiOpenDataTbl", [pName, pPID, pAddPath]);

    var tPlan = oiGetPlanning();
    var tLibPath = NULL;
    if (tPlan != NULL && tPlan.hasMember(@getLibPath))
	tLibPath = tPlan.getLibPath(pPID);

    if (tLibPath == NULL) {
	oiDebugPrint(@Warn, 1, ["no lib path in planning for program ", pPID]);
	tLibPath = oiGetAppLibPath(pPID);
	if (tLibPath == NULL) {
	    oiDebugPrint(@Warn, 1, 
			 ["no lib path in registry for program ", pPID]);
	    oiDebugFinishFunc(NULL);
	    return(NULL);
	}
    }
    var tTblID = "VAR EBASE,CSV " + tLibPath + " ofml " + pName;
    var tTable = OiTable(tTblID, OiFuncs.sDataTblStruct);

    oiDebugPrint(@Info, 2, ["table ID: ", tTblID]);

    if (tTable.open()) {
	oiDebugFinishFunc(tTable);
	return(tTable);
    }

    oiDebugPrint(@Warn, 1, ["cannot open table : ", tTblID]);

    
    if (pAddPath != NULL) {
	tTblID = "VAR EBASE,CSV " + pAddPath + " ofml " + pName;
	tTable = OiTable(tTblID, OiFuncs.sDataTblStruct);
	if (!tTable.open()) {
	    oiDebugPrint(@Warn, 1, ["cannot open table : ", tTblID]);
	    tTable = NULL;
	}
    }
    else
	tTable = NULL;

    oiDebugFinishFunc(tTable);
    return(tTable);
}



public func oiIsValidDate(pDate)
{
    oiDebugStartFunc2("OiFuncs::oiIsValidDate", pDate);

    if (!(pDate instanceof String && pDate.size() == 8)) {
	oiDebugFinishFunc(0);
	return(0);
    }
    var tYear, tMonth, tDay;

    try {
	tYear = Int(pDate.substr(0, 4));
	tMonth = pDate.substr(4, 2);
	if (tMonth[0] == '0')
	    tMonth = pDate.substr(5, 1);
	tMonth = Int(tMonth);
	if (tMonth < 1 || tMonth > 12) tMonth = NULL;
	tDay = pDate.substr(6, 2);
	if (tDay[0] == '0')
	    tDay = pDate.substr(7, 1);
	tDay = Int(tDay);
	if (tDay < 1 || tDay > 31) tDay = NULL;
	if (tMonth == 2 && (tDay > 29 || (tYear % 4 != 0 && tDay > 28)))
	    tDay = NULL;
	if ((tMonth == 4 || tMonth == 6 || tMonth == 9 || tMonth == 11) &&
	    tDay > 30)
	    tDay = NULL;
    }
    catch (&e: Error) {
	oiDebugPrint(@Warn, 1, e.where + ": " + e.what);
    }

    var tRet = (tYear != NULL && tMonth != NULL && tDay != NULL);

    oiDebugFinishFunc(tRet);
    return(tRet);
}







func oiGetAppProgState(pPID)
{
    if (OiFuncs.sHasOfmlAppFuncGetProgState == 0)
	return(NULL);

    if (OiFuncs.sHasOfmlAppFuncGetProgState == 1) {
        var tCode = ::ofml::app::getProgState(xgF.param2Str(pPID));
	if (tCode == "NULL") tCode = NULL;
	return(tCode);
    }

    var tCode = NULL;

    OiFuncs.sHasOfmlAppFuncGetProgState = 1;
    try {
        tCode = ::ofml::app::getProgState(xgF.param2Str(pPID));
        if (tCode == "NULL") tCode = NULL;
    }
    catch (&e: Error) {
	OiFuncs.sHasOfmlAppFuncGetProgState = 0;
	oiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function getProgState()!\n");
    }
    return(tCode);
}


func oiGetAppLibPath(pPID)
{
    
    
    if (OiFuncs.sAppLibPath == NULL) OiFuncs.sAppLibPath = Hash();

    if (OiFuncs.sAppLibPath.hasKey(pPID)) 
	return(OiFuncs.sAppLibPath[pPID]);

    if (OiFuncs.sHasOfmlAppFuncGetLibPath == 0) {
	OiFuncs.sAppLibPath[pPID] = NULL;
	return(NULL);
    }

    if (OiFuncs.sHasOfmlAppFuncGetLibPath == 1) {
        var tPath = ::ofml::app::getLibPath(xgF.param2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
	OiFuncs.sAppLibPath[pPID] = tPath;
	return(tPath);
    }

    var tPath = NULL;

    OiFuncs.sHasOfmlAppFuncGetLibPath = 1;
    try {
        tPath = ::ofml::app::getLibPath(xgF.param2Str(pPID));
        if (tPath == "NULL") tPath = NULL;
    }
    catch (&e: Error) {
	OiFuncs.sHasOfmlAppFuncGetLibPath = 0;
	oiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function getLibPath()!\n");
    }
    OiFuncs.sAppLibPath[pPID] = tPath;
    return(tPath);
}


func oiGetAppProgInfoDBPath(pPID)
{
    
    
    if (OiFuncs.sAppProgInfoDBPath == NULL) OiFuncs.sAppProgInfoDBPath = Hash();

    if (OiFuncs.sAppProgInfoDBPath.hasKey(pPID)) 
	return(OiFuncs.sAppProgInfoDBPath[pPID]);

    if (OiFuncs.sHasOfmlAppFuncGetProgInfoDBPath == 0) {
	OiFuncs.sAppProgInfoDBPath[pPID] = NULL;
	return(NULL);
    }

    if (OiFuncs.sHasOfmlAppFuncGetProgInfoDBPath == 1) {
        var tPath = ::ofml::app::getProginfoDBPath(xgF.param2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
	OiFuncs.sAppProgInfoDBPath[pPID] = tPath;
	return(tPath);
    }

    var tPath = NULL;

    OiFuncs.sHasOfmlAppFuncGetProgInfoDBPath = 1;
    try {
        tPath = ::ofml::app::getProginfoDBPath(xgF.param2Str(pPID));
        if (tPath == "NULL") tPath = NULL;
    }
    catch (&e: Error) {
	OiFuncs.sHasOfmlAppFuncGetProgInfoDBPath = 0;
	oiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function getProginfoDBPath()!\n");
    }
    OiFuncs.sAppProgInfoDBPath[pPID] = tPath;
    return(tPath);
}









func oiGetAppManufacturerID(pPID, ...)
{
    var tPID = pPID[0];
    var tType = @PD;
    if (pPID.size() > 1)
	tType = pPID[1];
    var tArg = xgF.param2Str(tPID) + ", " + xgF.param2Str(tType);

    var tMID = NULL;

    if (OiFuncs.sHasOfmlAppFuncGetManufacturerID == 0) return(tMID);
    if (OiFuncs.sHasOfmlAppFuncGetManufacturerID == 1) {
	tMID = ::ofml::app::getManufacturerID(tArg);
	if (tMID == "NULL") tMID = NULL;
	return(tMID);
    }

    OiFuncs.sHasOfmlAppFuncGetManufacturerID = 1;
    try {
	tMID = ::ofml::app::getManufacturerID(tArg);
    }
    catch (&e: Error) {
	OiFuncs.sHasOfmlAppFuncGetManufacturerID = 0;
	oiDebugPrint(@ExplWarn, 1, 
		     "Error during evaluation of function " +
		     "::ofml::app::getManufacturerID(): " + e.what);
    }
    if (tMID == "NULL") tMID = NULL;
    return(tMID);
}










func oiGetAppSeriesID(pPID, ...)
{
    var tPID = pPID[0];
    var tType = @PD;
    if (pPID.size() > 1)
	tType = pPID[1];
    var tArg = xgF.param2Str(tPID) + ", " + xgF.param2Str(tType);

    var tSID = NULL;

    if (OiFuncs.sHasOfmlAppFuncGetSeriesID == 0) return(tSID);
    if (OiFuncs.sHasOfmlAppFuncGetSeriesID == 1) {
	tSID = ::ofml::app::getSeriesID(tArg);
	if (tSID == "NULL") tSID = NULL;
	return(tSID);
    }

    OiFuncs.sHasOfmlAppFuncGetSeriesID = 1;
    try {
	tSID = ::ofml::app::getSeriesID(tArg);
    }
    catch (&e: Error) {
	OiFuncs.sHasOfmlAppFuncGetSeriesID = 0;
	oiDebugPrint(@ExplWarn, 1, 
		     "Error during evaluation of function " +
		     "::ofml::app::getSeriesID(): " + e.what);
    }
    if (tSID == "NULL") tSID = NULL;
    return(tSID);
}





func oiGetAppProductDBPath(pPID)
{
    
    
    if (OiFuncs.sAppProductDBPath == NULL) OiFuncs.sAppProductDBPath = Hash();

    if (OiFuncs.sAppProductDBPath.hasKey(pPID)) 
	return(OiFuncs.sAppProductDBPath[pPID]);

    

    var tPath = NULL;

    if (OiFuncs.sHasOfmlAppFuncGetProductDBPath == 0 &&
	OiFuncs.sHasEgrAppFuncGetProductDBPath == 0) {
	OiFuncs.sAppProductDBPath[pPID] = tPath;
	return(tPath);
    }

    if (OiFuncs.sHasOfmlAppFuncGetProductDBPath == 1) {
	tPath = ::ofml::app::getProductDBPath(xgF.param2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
	OiFuncs.sAppProductDBPath[pPID] = tPath;
	return(tPath);
    }
    if (OiFuncs.sHasEgrAppFuncGetProductDBPath == 1) {
	tPath = ::egr::app::getProductDBPath(xgF.param2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
	OiFuncs.sAppProductDBPath[pPID] = tPath;
	return(tPath);
    }

    OiFuncs.sHasOfmlAppFuncGetProductDBPath = 1;
    try {
	tPath = ::ofml::app::getProductDBPath(xgF.param2Str(pPID));
    }
    catch (&e: Error) {
	OiFuncs.sHasOfmlAppFuncGetProductDBPath = 0;
	oiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::getProductDBPath(): " + e.what);
    }
    if (OiFuncs.sHasOfmlAppFuncGetProductDBPath == 1) {
	if (tPath == "NULL") tPath = NULL;
	OiFuncs.sAppProductDBPath[pPID] = tPath;
	return(tPath);
    }

    OiFuncs.sHasEgrAppFuncGetProductDBPath = 1;
    try {
	tPath = ::egr::app::getProductDBPath(xgF.param2Str(pPID));
    }
    catch (&e: Error) {
	OiFuncs.sHasEgrAppFuncGetProductDBPath = 0;
	oiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::egr::app::getProductDBPath():" + e.what);
    }
    if (tPath == "NULL") tPath = NULL;

    OiFuncs.sAppProductDBPath[pPID] = tPath;
    return(tPath);
}




func oiGetAppSeriesLanguages(pPID)
{
    oiDebugStartFunc("OiFuncs::oiGetAppSeriesLanguages", pPID);

    
    
    
    

    var tRet = @();

    if (OiFuncs.sHasEgrAppFuncGetRegistryKey == 0 &&
	OiFuncs.sHasOfmlAppFuncGetRegistryKey == 0) {
	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    var tArgs = xgF.param2Str(pPID) + ",\"languages\", NULL, NULL";

    if (OiFuncs.sHasEgrAppFuncGetRegistryKey == 1) {
        tRet = _oiConvertRegistryLanguages(::egr::app::getRegistryKey(tArgs));
	oiDebugFinishFunc(tRet);
	return(tRet);
    }
    if (OiFuncs.sHasOfmlAppFuncGetRegistryKey == 1) {
        tRet = _oiConvertRegistryLanguages(::ofml::app::getRegistryKey(tArgs));
	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    OiFuncs.sHasEgrAppFuncGetRegistryKey = 1;
    try {
        tRet = _oiConvertRegistryLanguages(::egr::app::getRegistryKey(tArgs));
	oiDebugFinishFunc(tRet);
	return(tRet);
    }
    catch (&e: Error) {
	OiFuncs.sHasEgrAppFuncGetRegistryKey = 0;
	oiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::egr::app or that\n" +
	    "package does not implement function getRegistryKey()!\n");
    }

    OiFuncs.sHasOfmlAppFuncGetRegistryKey = 1;
    try {
        tRet = _oiConvertRegistryLanguages(::ofml::app::getRegistryKey(tArgs));
    }
    catch (&e: Error) {
	OiFuncs.sHasOfmlAppFuncGetRegistryKey = 0;
	oiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function getRegistryKey()!\n");
    }

    oiDebugFinishFunc(tRet);
    return(tRet);
}



func _oiConvertRegistryLanguages(pLanguages)
{
    if (!(pLanguages instanceof String) || 
	pLanguages.empty() ||
	pLanguages == "NULL")
	return(@());

    var tLanguages = String(pLanguages);

    
    if (pLanguages != NULL && 
	pLanguages[0] == '\"' && pLanguages[pLanguages.size()-1] == '\"')
	tLanguages = pLanguages.substr(1, pLanguages.size()-2);

    return(oiParseString2List(tLanguages, ';', 1));
}

func oiGetAppPDLanguage(pPID)
{
    oiDebugStartFunc("OiFuncs::oiGetAppPDLanguage", pPID);

    var tRet = NULL;

    if (OiFuncs.sHasOfmlAppFuncGetPDLanguage == 0) {
	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    if (OiFuncs.sHasOfmlAppFuncGetPDLanguage == 1) {
        tRet = ::ofml::app::getPDLanguage(pPID);
	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    OiFuncs.sHasOfmlAppFuncGetPDLanguage = 1;
    try {
        tRet = ::ofml::app::getPDLanguage(pPID);
    }
    catch (&e: Error) {
	OiFuncs.sHasOfmlAppFuncGetPDLanguage = 0;
	oiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function getPDLanguage()!\n");
    }

    oiDebugFinishFunc(tRet);
    return(tRet);
}










func oiGetAppOAMPath(pPID)
{
    
    
    if (OiFuncs.sAppOAMPath == NULL) OiFuncs.sAppOAMPath = Hash();

    if (OiFuncs.sAppOAMPath.hasKey(pPID)) return(OiFuncs.sAppOAMPath[pPID]);

    var tPath = NULL;

    if (OiFuncs.sHasOfmlAppFuncGetOAMPath == 1) {
	tPath = ::ofml::app::getOAMPath(xgF.param2Str(pPID));
	if (tPath == "NULL") tPath = NULL;
    }
    else
    if (OiFuncs.sHasOfmlAppFuncGetOAMPath == -1) {
        OiFuncs.sHasOfmlAppFuncGetOAMPath = 1;
        try {
	    tPath = ::ofml::app::getOAMPath(xgF.param2Str(pPID));
            if (tPath == "NULL") tPath = NULL;
        }
        catch (&e: Error) {
	    OiFuncs.sHasOfmlAppFuncGetOAMPath = 0;
	    oiDebugPrint(@ExplWarn, 1, 
		          "Error during evaluation of function " +
		          "::ofml::app::getOAMPath(): " + e.what);
        }
    }

    if (tPath == NULL && (tPath = oiGetAppProductDBPath(pPID)) != NULL) {
	
	var tPos = tPath.rfind("/");
	if (tPos < 0)
	    tPath = NULL;
	else
	    tPath = tPath.substr(0,tPos+1) + "oam";
    }
    OiFuncs.sAppOAMPath[pPID] = tPath;
    return(tPath);
}

func oiDebugStartFunc(pName, pArgs)
{
    if (!xxFuncs.sDebuggingEnabled) return;
    var tPlan = oiGetPlanning();
    if (tPlan != NULL && tPlan.hasMember(@debugStartFunc))
	tPlan.debugStartFunc(pName, pArgs);
}

func oiDebugStartFunc2(pName, pArgs)
{
    if (!xxFuncs.sDebuggingEnabled) return;
    var tPlan = oiGetPlanning();
    if (tPlan != NULL && tPlan.hasMember(@debugStartFunc2))
	tPlan.debugStartFunc2(pName, pArgs);
}

func oiDebugStartFunc3(pName, pArgs)
{
    if (!xxFuncs.sDebuggingEnabled) return;
    var tPlan = oiGetPlanning();
    if (tPlan != NULL && tPlan.hasMember(@debugStartFunc3))
	tPlan.debugStartFunc3(pName, pArgs);
    else
    if (tPlan != NULL && tPlan.hasMember(@debugStartFunc2))
	tPlan.debugStartFunc2(pName, pArgs);
}

func oiDebugFinishFunc(pArg)
{
    if (!xxFuncs.sDebuggingEnabled) return;
    var tPlan = oiGetPlanning();
    if (tPlan != NULL && tPlan.hasMember(@debugFinishFunc))
	tPlan.debugFinishFunc(pArg);
}

func oiDebugPrint(pMode, pLevel, pArg)
{
    if (!xxFuncs.sDebuggingEnabled) return;
    var tPlan = oiGetPlanning();
    if (tPlan != NULL && tPlan.hasMember(@debugPrint))
	tPlan.debugPrint(pMode, pLevel, pArg);
}




func oiList2Hash(pList)
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
	    tHash[tEntry[0]] = oiList2Hash(tEntry[1]);
	else
	    tHash[tEntry[0]] = tEntry[1];
    }

    return(tHash);
}


private func oiSetMatRecursiv(pObj, pMat)
{
    pObj.setMaterial(pMat);
    var tCh;
    foreach(tCh; pObj.getChildren()) oiSetMatRecursiv(tCh, pMat);
}







func oiGetWorldRotation(pObj, pAxis)
{
    var tRot = pObj.getRotation(pAxis);
    var tObj = pObj.getFather();
    if (tObj == NULL) return(0.0);

    var t2Pi = 4*acos(0);
    var tF;
    while ((tF = tObj.getFather()) != NULL) {
	tRot += tObj.getRotation(pAxis);
	if (tRot > t2Pi)
	    tRot -= t2Pi;
	else
	if (tRot < -t2Pi)
	    tRot += t2Pi;
	tObj = tF;
    }
    return(tRot);
}


func oiGetClassesOf(pObj)
{
    var tCL = @();
    var tType = typeOf(pObj);  
    var tTName = tType.getName();
    if (OiFuncs.sSuperClasses.hasKey(tTName))
	return(OiFuncs.sSuperClasses[tTName]);

    while (tType != NULL) {
	tCL.pushBack(tType.getFullName());
	tType = tType.getSuperClass();
    }
    OiFuncs.sSuperClasses[tTName] = tCL;
    return(tCL);
}


func oiGetPackage(pObj)
{
    var tClass = pObj.getClass();
    var tType  = typeOf(pObj);
    var tSpec  = tType.getFullName();

    var tPos   = tSpec.find(tClass, 0);

    return(tSpec.substr(0, tPos-2));
}

func oiGetProgInfo4Obj(pObj)
{
    var tPID;

    if (pObj == NULL || !pObj.hasMember(@getProgram) ||
	(tPID = pObj.getProgram()) == NULL) 
	return(NULL);

    var tPI  = NULL;

    if (OiFuncs.sProgInfoObjCache == NULL) OiFuncs.sProgInfoObjCache = Hash();

    if (OiFuncs.sProgInfoObjCache.hasKey(tPID)) {
	tPI  = OiFuncs.sProgInfoObjCache[tPID];
	
	try {
	    tPI.getName();
	} catch (&e: Error) {
	    tPI = NULL;
	}
	if (tPI != NULL) return(tPI);
    }

    var tPlanning = oiGetPlanning();
    if (tPlanning != NULL)
	tPI = tPlanning.getInfo(tPID);

    OiFuncs.sProgInfoObjCache[tPID] = tPI;

    return(tPI);
}


func oiClasses2Packages(pClasses)
{
    var tRetL = @();
    var tClass;
    foreach(tClass; pClasses)
	tRetL.pushBack(tClass.substr(0, tClass.rfind("::")));

    tRetL.unique();

    return(tRetL);
}   



















func oiUpdateConfiguration(pObj)
{
    oiDebugStartFunc("OiFuncs::oiUpdateConfiguration", pObj);

    if (!pObj.isCat( @IF_Article)) {
	oiDebugPrint(@Info, 1, "object is no article");
	oiDebugFinishFunc(NULL);
	return;
    }

    var tUpdateState = pObj.getObjState(@OI_UpdateState);

    if (tUpdateState == @Up2Date || tUpdateState == @Invalid) {
	oiDebugPrint(@Info, 1, "nothing to do");
	oiDebugFinishFunc(NULL);
	return;
    }

    var tPlan = pObj.getPlanning();
    var tPDM = pObj.getPDManager();

    if (tPlan == NULL || tPDM == NULL) {
	
	oiDebugPrint(@ExplWarn, 1, "oiUpdateConfiguration(): no PD manager!");
	pObj.setObjState(@OI_UpdateState, @Invalid);
	oiDebugFinishFunc(NULL);
	return;
    }

    if (tUpdateState == @Undefined) {
	tUpdateState = tPDM.checkObjUpdatability(pObj, @OFMLVarCode);
        pObj.setObjState(@OI_UpdateState, tUpdateState);
	if (tUpdateState == @Invalid) {
	    tPlan.issueUpdateMessage(pObj, @InvalidArticle, NULL);
	    
	    oiDebugFinishFunc(NULL);
	    return;
	}
    }

    if (tUpdateState == @Migratable && 
	(tPlan == NULL || !tPlan.confirmArticleMigration(pObj))) {
	oiDebugPrint(@Info, 1, "migration refused");
	oiDebugFinishFunc(NULL);
	return;
    }

    xgPH.restorePropertyStates(pObj, OiFuncs.sSavedMigratablePropStatesDynProp);

    tPDM.updateConfiguration(pObj);

    pObj.updateGeometry();

    
    var tAssignPriceDate2Old = (tPlan.getPriceDateMode() == @AnySave);
    var tPriceDate = oiGetCurrentDate();
    if (tAssignPriceDate2Old) {
	var tObjPriceDate = pObj.getPriceDate();
	if (tObjPriceDate == NULL) {
	    oiDebugPrint(@Info, 2, ["assigning price date ", tPriceDate]);
	    tPlan.issueUpdateMessage(pObj, @NoPriceDate, NULL);
	    
	    pObj.setPriceDate(tPriceDate);
	}
    }

    pObj.setObjState(@OI_UpdateState, @Up2Date);

    oiDebugFinishFunc(NULL);
}

func oiUpdateConfiguration2(pObj, pUpdateLanguageOnly, ...)
{
    var tUpdateLanguageOnly = pUpdateLanguageOnly[0];

    var tOk = 1;
    var tPDM = pObj.getPDManager();
    if (tPDM != NULL && tPDM.hasMember(@updateConfiguration2))
        tOk = tPDM.updateConfiguration2(pObj, tUpdateLanguageOnly);

    return(tOk);
}

func oiGetArticleClassifications(pObj, pLanguages)
{
    var tObj = pObj.getArticleObj();
    var tPDM = tObj.getPDManager(); 
    var tRes = NULL;
    if (tPDM  != NULL) 
	tRes = tPDM.getArticleClassifications(tObj, pLanguages);

    return(tRes);
}

func oiGetArticleAttr(pObj, pAttr)
{
    var tValidAttributes = @(@Discountable);
    var tRes = NULL;

    if (!(pAttr instanceof Symbol) || tValidAttributes.find(pAttr) < 0)
	return(tRes);

    var tObj = pObj.getArticleObj();
    var tPDM = tObj.getPDManager(); 
    if (tPDM  != NULL) tRes = tPDM.getArticleAttribute(tObj, pAttr);

    return(tRes);
}

func oiGetOrderUnit(pObj, pLanguage)
{
    var tUnit = "C62";

    var tPDM = pObj.getPDManager(); 
    if (tPDM  != NULL) tUnit = tPDM.getOrderUnit(pObj);

    if (pLanguage != NULL)
	tUnit = oiGetOrderUnitDescr(tUnit, pLanguage);

    return(tUnit);
}






func oiUpdate4PropChange(pObj)
{
    oiDebugStartFunc("OiFuncs::oiUpdate4PropChange", pObj);

    var tDynProps = pObj.getDynamicProps();
    var tRes;
    if (tDynProps.hasKey(@_OI_UpdatingArticles))
	oiDebugPrint(@Info, 1,
		     "nested call during updateConfiguration()");

    var tPropDefChanged = 0;

    if (pObj.getObjState(@OI_UpdateState) == @Undefined && 
	!tDynProps.hasKey(@_OI_UpdatingArticles)) {
	tDynProps[@_OI_UpdatingArticles] = 1;
	tRes = pObj.updateConfiguration();
	tDynProps.remove(@_OI_UpdatingArticles);
	tPropDefChanged = pObj.propDefChanged(NULL);
        oiDebugPrint(@Info, 2,
		    ["changed prop defs during updateConfiguration()? ",
		     tPropDefChanged]);
        oiDebugPrint(@Info, 2,
		    ["changed prop vals during updateConfiguration(): ",
		     pObj.changedPropList()]);
    }

    
    if (pObj.getObjState(@OI_UpdateState) == @Undefined && 
	tRes instanceof Int) {
	oiDebugPrint(@Info, 1,
		     ["old style updateConfiguration(): ", tRes]);
	if (tRes)
	    pObj.setObjState(@OI_UpdateState, @Up2Date);
	else
	    pObj.setObjState(@OI_UpdateState, @Invalid);
    }

    var tUpdateState = pObj.getObjState(@OI_UpdateState);

    
    

    oiDebugPrint(@Info, 2, ["new update state: ", tUpdateState]);

    var tRet = tPropDefChanged;
    if (tUpdateState != @Up2Date)
	tRet = -1;

    oiDebugFinishFunc(tRet);
    return(tRet);
}





func oiPropIsCH(pType)
{  
    return((pType.size() > 1 && pType.substr(0, 2) == "ch") ||
	   (pType.size() > 3 && pType.substr(0, 4) == "u ch"));
}  






func oiGetPropValueFromString(pPType, pValStr)
{
    if (pPType == NULL || pValStr == NULL) return(NULL);

    
    var tIsFloat = 0;
    var tIsInt = 0;
    var tIsStr = 1;
    if (pPType.size() > 4 && pPType.substr(0, 4) == "u ch") {
        var tT5 = pPType[4];
        if (tT5 == 'f') tIsFloat = 1;
        else
        if (tT5 == 'i') tIsInt = 1;
    }
    else {
        var tT1 = pPType[0];
        if (tT1 == 'f') tIsFloat = 1;
        else
        if (tT1 == 'i' || tT1 == 'b') tIsInt = 1;
    }
    if (tIsInt || tIsFloat) tIsStr = 0;

    

    var tVal = pValStr;

    try {
    if (tIsInt)
        tVal = Int(tVal);
    else
    if (tIsFloat) 
        tVal = Float(tVal);
    else
    if (tVal[0] == '@') 
        tVal = Symbol(tVal.substr(1, tVal.size()-1));
    }
    catch (&e: Error) {
	oiDebugPrint(@EvalErr, 1, 
		     "couldn't convert value string ("+e.where+": "+e.what+")");
	tVal = NULL;
    }

    return(tVal);
}


func oiPropSpec2Type(pPSpec)
{
    
    var tSpace = pPSpec.find(" ", 0);
    if (tSpace < 0) return(pPSpec);

    if (pPSpec[0] == 'u') {
	tSpace = pPSpec.find(" ", tSpace+1);
	if (tSpace < 0) return(pPSpec);
    }
    return(pPSpec.substr(0, tSpace));
}



func oiEvalFunc(pFunc)
{
    oiDebugStartFunc2("OiFuncs::oiEvalFunc", pFunc);

    var tRet = NULL;
    try {
	tRet = eval(pFunc);
    }
    catch (&e: Error) {
	oiDebugPrint(@EvalErr, 1, 
		     ["Evaluation error during property choice list function '",
		      pFunc, "': ", e.where, ": ", e.what]);
    }

    
    

    oiDebugFinishFunc(tRet);
    return(tRet);
}





func oiCHFProp2CHProp(pObj, pPType)
{
    oiDebugStartFunc2("OiFuncs::oiCHFProp2CHProp", [pObj, pPType]);

    var tFunc = pObj.getName() + "." + pPType.substr(4);
    var tChL = oiEvalFunc(tFunc);

    if (tChL != NULL) 
	tChL = "ch " + tChL;
    else
	oiDebugPrint(@Warn, 1, 
		     ["error during chf-function '", pPType.substr(4),
		      "' called on ", pObj, "!"]);

    oiDebugFinishFunc(tChL);
    return(tChL);
}













func oiDecomposeChoiceList(pChList, pConvert, ...)
{
    oiDebugStartFunc2("OiFuncs::oiDecomposeChoiceList", [pChList, pConvert]);

    var tConvert = pConvert[0];
    var tNewStyle = 0;
    if (pConvert.size() > 1)
	tNewStyle = pConvert[1];
    var tPType = NULL; 
    if (pConvert.size() > 2)
	tPType = pConvert[2];

    var tVL = @();
    var tChList = String(pChList);
    var tLen = tChList.size();

    if (tPType == NULL) {
	
	tPType = oiPropSpec2Type(pChList);
	if (tPType.size() == pChList.size()) {
	    oiDebugPrint(@Warn, 1, "wrong property type (no choice list)!");
	    oiDebugFinishFunc(tVL);
	    return(tVL);
	}
	tChList = pChList.substr(tPType.size()+1);
    }

    if (OiFuncs.sDecomposedChoiceLists == NULL)
	OiFuncs.sDecomposedChoiceLists = Hash();

    var tCacheKey = tChList + "+" + String(tConvert) + "+" + String(tNewStyle);

    if (OiFuncs.sDecomposedChoiceLists.hasKey(tCacheKey)) {
	oiDebugPrint(@Info, 1, "using cache");
	tVL = OiFuncs.sDecomposedChoiceLists[tCacheKey];
	oiDebugFinishFunc(tVL);
	return(tVL);
    }

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
		oiDebugPrint(@Warn, 1, ["missing matching '\"' in ", tChList]);
		OiFuncs.sDecomposedChoiceLists[tCacheKey] = @();
		oiDebugFinishFunc(@());
		return(@());
	    }
	    tP2++;
	}
	else {
	    tP2 = tChList.find(" ", tP1);
    	    if (tP2 < 0) {
	        tP2 = tLen;
	        tEOS = 1;
	    }
	}
	var tSize = tP2-tP1;
        var tVal  = tChList.substr(tP1, tSize);

	oiDebugPrint(@Info, 2, ["value[", tP1, ",", tSize, "]: '", tVal, "'"]);

	var tDescr = NULL;

	if (tVal[0] == '"')
	    // no value-descr pairs are given but only choice strings
	    // -> put string as value and NULL as descr
	    tVal = oiSubstString(tVal.substr(1, tVal.size()-2), "\\\"", "\"");
	else {
    	    // get description
            if (!tEOS && (tP1 = tChList.find('\"', tP2)) > 0) {
	    	tP2 = tChList.find('\"', tP1+1);
		while (tP2 > 0 && tChList[tP2-1] == '\\')
		    tP2 = tChList.find('\"', tP2+1);
		if (tP2 == -1) {
		    oiDebugPrint(@Warn, 1, 
				 ["missing matching '\"' in ", tChList]);
		    OiFuncs.sDecomposedChoiceLists[tCacheKey] = @();
		    oiDebugFinishFunc(@());
		    return(@());
		}
	    	tDescr = tChList.substr(tP1+1, tP2-tP1-1);
		oiDebugPrint(@Info, 3, ["description (raw): ", tDescr]);
	    	tDescr = oiSubstString(tDescr, "\\", "");
	    	tP2++;
	    }
	}
	oiDebugPrint(@Info, 2, ["description: ", tDescr]);

	if (tConvert) {
	    tVal = oiGetPropValueFromString(tPType, tVal);
	    oiDebugPrint(@Info, 2, ["converted value: ", tVal]);
	}

	if (tNewStyle)
	    tVL.pushBack([tVal, tDescr, 1, NULL, NULL]);
	else
	    tVL.pushBack([tVal, tDescr]);

	tEOL = tP2;
    }
    OiFuncs.sDecomposedChoiceLists[tCacheKey] = tVL;
    oiDebugFinishFunc(tVL);
    return(tVL);
}

// Returns choice list string for given type and list of values.<br>
// The value list is a list of [value, description] pairs, where
// description may be NULL.<br>
// To be used for choice list type specifications in
// <i>Base::setupProperty()</i> (old property interface).<br>
// Note: pValues[0] may be a list of choicelist value specifications according
// to new property interface, ie. a vector of 5 elements. In this case we have
// to consider 3rd element: validity!
// If an additional optional parameter is given, it specifies whether quotes
// in description Strings have to be quoted (1) or not (0=default).
// Returns NULL if type is not a choice list or a user defined type, or
// if no value list is specified, or if list is empty.
// Precondition: pType must specify a choice list and pValues[0] != NULL.
func oiBuildChoiceList(pPType, pValues, ...)
{
    oiDebugStartFunc2("OiFuncs::oiBuildChoiceList", [pPType, pValues]);

    var tChoices = pPType;
    var tValues = pValues[0];
    var tEscapeQuotes = 0;
    if (pValues.size() > 1)
	tEscapeQuotes = pValues[1];

    // determine type
    var tIsFloat = 0;
    var tIsInt = 0;
    var tIsStr = 1;
    if (pPType.substr(0, 4) == "u ch") {
        var tT5 = pPType[4];
        if (tT5 == 'f') tIsFloat = 1;
        else
        if (tT5 == 'i') tIsInt = 1;
    }
    else {
        var tT1 = pPType[0];
        if (tT1 == 'f') tIsFloat = 1;
        else
        if (tT1 == 'i' || tT1 == 'b') tIsInt = 1;
    }
    if (tIsInt || tIsFloat) tIsStr = 0;

    oiDebugPrint(@Info, 2, 
		 ["float/int/str ? ", tIsFloat, "/", tIsInt, "/", tIsStr]);
    var tV;
    foreach(tV; tValues) {
	if (tV.size() > 2 && !tV[2]) {
	    oiDebugPrint(@Info, 2, ["skipping invalid value ", tV[0]]);
    	    continue;
	}
        if (tV[1] != NULL && tV[1] == "@VOID") {
    	    tChoices = tChoices + " " + tV[1] + " \"\"";
    	    continue;
        }

	var tValStr = NULL;
        var tDescr = NULL;

        if (tIsInt || tIsFloat) {
	    if (tV[0] == NULL)
		tValStr = "@UNSPECIFIED";
	    else
    		tValStr = String(tV[0]);
	}
        else
    	if (tV[0] instanceof Symbol)
    	    tValStr = "@" + String(tV[0]);
    	else
    	if (tV[0] instanceof String) {
    	    tValStr = "\"" + tV[0] + "\"";
	    // a possibly given description in tV[1] will be ignored!
	    tDescr = "";
	}

        if (tValStr == NULL) {
	    oiDebugPrint(@Info, 2, 
			 ["skipping value of not-supported type ", tV[0]]);
    	    continue;
	}

	tChoices += " " + tValStr;

        if (tDescr == NULL && tV[1] != NULL) {
	    tDescr = String(tV[1]);
	    if (tEscapeQuotes)
		tDescr = oiEscapeQuotes(tDescr);
	}

	if (tDescr != NULL && !tDescr.empty())
	    tChoices += " \"" + tDescr + "\"";
    }

    oiDebugFinishFunc(tChoices);
    return(tChoices);
}

// Calls changeIssued() on global change manager for given publisher.
func oiIssueChange(pPublisher, pChangeType, pArgs)
{
    var tPlan = pPublisher.getPlanning();
    var tChMgr = (tPlan != NULL) ? tPlan.getChangeManager() : NULL;
    if (tChMgr != NULL) 
	tChMgr.changeIssued(pPublisher, pChangeType, pArgs);
}

// Adds an entry [pLanguage, pText] to the given member list.
// Before, removes an existing entry with same language.
func oiSetUserArticleText(pMember, pText, pLanguage)
{
    var tEntry;
    foreach(tEntry; pMember)
	if (tEntry[0] == pLanguage) {
	    pMember.remove(tEntry);
	    break;
	}
    pMember.pushBack([pLanguage, pText]);
}

// Returns the user defined article text for specified language stored in
// given member list.
func oiGetUserArticleText(pMember, pLanguage)
{
    var tEntry;
    foreach(tEntry; pMember)
	if (tEntry[0] == pLanguage) return(tEntry[1]);
}

//
public func oiGetArticleFeaturesDescr(pObj, pType, pLanguage)
{
    var tValidTypes = @(@Text, @AllIDs, @ID_Text);

    if (tValidTypes.find(pType) < 0) return(NULL);

    var tLanguage = pLanguage;
    var tPlan = oiGetPlanning();

    if (tLanguage == NULL && tPlan != NULL)
	tLanguage = tPlan.getLanguage();

    if (pType == @Text) return(pObj.getArticleFeatures(tLanguage));

    if (pType == @AllIDs) return(pObj.getAllArticleFeatures(tLanguage));

    // pType == @ID_Text

    var tRet = NULL;
    var tPDM = pObj.getPDManager();
    if (tPDM != NULL)
	tRet = tPDM.getArticleFeaturesDescr(pObj, pType, tLanguage);

    return(tRet);
}

// Checks consistency for given article instance.
// If current update state of the article instance is not up-to-date, uses the 
// saved result of last call of checkConsistency() (if there is a saved state).
// Otherwise:
// First delegates to equal named method of global product data manager passing
// the given object. Then delegates to method checkObjConsistency() of 
// corresponding proginfo object passing the given object. (This way, the 
// proginfo object may perform consistency checks for given instance 
// additionally to the standard product data consistency checks.)
// Reports the possible errors to global error log if it is opened,
// otherwise to oiOutput().
// Returns True, if no errors occured.
func oiCheckConsistency(pObj)
{
    oiDebugStartFunc("OiFuncs::oiCheckConsistency", pObj);

    var tPDM = pObj.getPDManager();
    if (tPDM == NULL) {
	// should not happen!
	oiDebugPrint(@Warn, 1, "no PD manager!?");
	oiDebugFinishFunc(1);
	return(1);
    }

    var tLanguage = "de";
    var tReport = NULL;
    var tOwnDialog = 0;
    var tPlanning = pObj.getPlanning();

    if (tPlanning != NULL) {
	tLanguage = tPlanning.getLanguage();
	// XXX ??
	// tLanguage = tPlanning.getPDLanguage(pObj);
	tReport = tPlanning.getErrorLog();
    }
    if (tReport == NULL) {
	tReport = Hash();
	if (tPlanning != NULL) tPlanning.setErrorLog(tReport);
	tOwnDialog = 1;
    }

    var tLogKey = pObj.getOrderID();
    if (tLogKey == NULL)
	tLogKey = pObj.getName();

    var tRes = 1;

    if (!pObj.isUp2Date()) {
	// we have to use the saved state
	tRes = pObj.getObjState(@OI_ConsistencyState);
	if (tRes == NULL)
	    oiDebugPrint(@Warn, 1, "no saved consistency state!");
	else {
	    oiDebugPrint(@Info, 1, "using saved consistency state");
	    var tMsg = oiGetStringResource("::ofml::oi::@saved_inconsistency", 
				           tLanguage, pObj);
	    if (pObj.getObjState(@OI_UpdateState) != @Invalid)
		tMsg += "\n" + 
		        oiGetStringResource("::ofml::oi::@update_first",
					    tLanguage, pObj);
	    if (!tRes) {
	        if (tOwnDialog)
		    oiOutput(@WARNING, oiSubstString(tMsg, "\n", " "));
	        else {
		    if (!tReport.hasKey(tLogKey)) tReport[tLogKey] = @();
		    var tErrors = tReport[tLogKey];
		    var tClass  = pObj.isA(OiPlElement) ? 
				  "OiPlElement" : "OiPart";
		    tErrors.pushBack([tMsg, pObj.getName(), 
				      tClass+"::oiCheckConsistency"]);
		}
	    }
	    oiDebugFinishFunc(tRes);
	    return(tRes);
	}
    }

    tRes = tPDM.checkConsistency(pObj, tLanguage, tReport);

    var tPI = NULL;
    if (tPlanning != NULL)
	tPI = tPlanning.getInfo(pObj.getProgram());
    if (tPI != NULL && tPI.hasMember(@checkObjConsistency))
	tRes = (tRes && tPI.checkObjConsistency(pObj, tLanguage, tReport));

    // print errors
    if (tOwnDialog && tReport.hasKey(tLogKey)) {
	var tMsg = oiGetStringResource("::ofml::oi::@inconsistency", 
				       tLanguage, pObj) + ":\n";
	var tErrors = tReport[tLogKey];
        var tErrCnt = 0;
	var tE;
	foreach(tE; tErrors) {
	    var tErr = NULL;
	    if (tE instanceof Vector && tE[0] instanceof String) 
		tErr = tE[0];
	    else
	    if (tE instanceof String) 
		tErr = tE;
	    if (tErr != NULL) 
		tMsg += oiGetStringResource(tErr, tLanguage) + "\n";
	    tErrCnt++;
	}
	if (tErrCnt) oiOutput(@WARNING, tMsg);
    }

    if (tOwnDialog && tPlanning != NULL) tPlanning.setErrorLog(NULL);

    // if (pObj.isUp2Date()) // XXX ?
    pObj.setObjState(@OI_ConsistencyState, tRes);

    oiDebugFinishFunc(tRes);
    return(tRes);
}

// -------------
// Recovery Info (obsolete)
// -------------

// Returns True if given recovery info contains required general info.
func oiCheckGeneralRecoveryInfo(pObj, pInfo)
{
    oiDebugStartFunc("OiFuncs::oiCheckGeneralRecoveryInfo", [pObj, pInfo]);
    oiDebugPrint(@Warn, 1, "function no longer supported/implemented!");
    oiDebugFinishFunc(0);
    return(0);
}

// Returns True if given recovery info contains required complex info.
func oiCheckComplexRecoveryInfo(pObj, pInfo)
{
    oiDebugStartFunc("OiFuncs::oiCheckComplexRecoveryInfo", [pObj, pInfo]);
    oiDebugPrint(@Warn, 1, "function no longer supported/implemented!");
    oiDebugFinishFunc(0);
    return(0);
}

// Returns True if given recovery info contains required plelement info.
func oiCheckPlElementRecoveryInfo(pObj, pInfo)
{
    oiDebugStartFunc("OiFuncs::oiCheckPlElementRecoveryInfo", [pObj, pInfo]);
    oiDebugPrint(@Warn, 1, "function no longer supported/implemented!");
    oiDebugFinishFunc(0);
    return(0);
}

// Returns True if given recovery info contains required part info.
func oiCheckPartRecoveryInfo(pObj, pInfo)
{
    oiDebugStartFunc("OiFuncs::oiCheckPartRecoveryInfo", [pObj, pInfo]);
    oiDebugPrint(@Warn, 1, "function no longer supported/implemented!");
    oiDebugFinishFunc(0);
    return(0);
}

// Returns True if given recovery info contains required info for
// Article interface.
func oiCheckArticleRecoveryInfo(pObj, pInfo)
{
    oiDebugStartFunc("OiFuncs::oiCheckArticleRecoveryInfo", [pObj, pInfo]);
    oiDebugPrint(@Warn, 1, "function no longer supported/implemented!");
    oiDebugFinishFunc(0);
    return(0);
}

// pCommonProp contains a vector describing a common property:
// Sets up properties for given recovered object according to given
// Property recovery info.
// All properties are read-only.
func oiSetupRecoveredProperties(pObj, pGeneralInfo, pPropertyInfo)
{
    oiDebugStartFunc2("OiFuncs::oiSetupRecoveredProperties",
		      [pObj, pGeneralInfo, pPropertyInfo]);
    oiDebugPrint(@Warn, 1, "function no longer supported/implemented!");
    oiDebugFinishFunc(NULL);
}

// Returns True if given recovery info contains required info for
// Property interface.
func oiCheckPropertyRecoveryInfo(pObj, pInfo)
{
    oiDebugStartFunc("OiFuncs::oiCheckPropertyRecoveryInfo", [pObj, pInfo]);
    oiDebugPrint(@Warn, 1, "function no longer supported/implemented!");
    oiDebugFinishFunc(0);
    return(0);
}

// ------------
// Catalog Info
// ------------

// Removes items from catalog info of given object which can be restored
// by equivalent current article information of given object.
func oiReduceCatalogInfo(pObj)
{
    oiDebugStartFunc("OiFuncs::oiReduceCatalogInfo", pObj);

    var tCatInfo = pObj.getCatalogInfo();

    oiDebugPrint(@Info, 2, ["object's catalog info: ", tCatInfo]);

    if (tCatInfo == NULL) {
	oiDebugPrint(@Info, 1 , "no catalog info");
	oiDebugFinishFunc(NULL);
	return;
    }

    // 1.step: determine items to be removed
    var tRemoveItems = @();
    var tItem;
    foreach(tItem; tCatInfo) {
	var tKey = tItem[0];
	var tVal = tItem[1];
	if ((tKey == @CatID && tVal == pObj.getProgram()) ||
	    (tKey == @ArtNr && tVal == pObj.getArticleSpec()) ||
	    (tKey == @VarCode && tVal == pObj.getXArticleSpec(@VarCode)))
	    tRemoveItems.pushBack(tItem);
    }

    // 2.step: remove items from catalog info
    foreach(tItem; tRemoveItems) tCatInfo.remove(tItem);

    oiDebugPrint(@Info, 2, ["reduced catalog info: ", tCatInfo]);

    pObj.setCatalogInfo(tCatInfo);

    oiDebugFinishFunc(NULL);
}

// Completes missing catalog info of given object by equivalent current 
// article information of given object.
func oiCompleteCatalogInfo(pObj)
{
    oiDebugStartFunc("OiFuncs::oiCompleteCatalogInfo", pObj);

    var tCatInfo = pObj.getCatalogInfo();

    oiDebugPrint(@Info, 2, ["object's cat info: ", tCatInfo]);

    if (tCatInfo == NULL) {
	oiDebugPrint(@Info, 1, "no cat info for object");
	oiDebugFinishFunc(NULL);
	return;
    }

    // 1.step: determine missing items
    var tMissedItems = @(@CatID, @ArtNr, @VarCode);
    var tItem;
    foreach(tItem; tCatInfo) {
	var tKey = tItem[0];
	if (tMissedItems.find(tKey) >= 0) tMissedItems.remove(tKey);
    }

    // 2.step: add missing items
    var tK;
    foreach(tK; tMissedItems) {
	var tKey = tK; // need a copy here!
	if (tKey == @CatID)
	    tCatInfo.pushBack([tKey, pObj.getProgram()]);
	else
	if (tKey == @ArtNr)
	    tCatInfo.pushBack([tKey, pObj.getArticleSpec()]);
	else
	if (tKey == @VarCode)
	    tCatInfo.pushBack([tKey, pObj.getXArticleSpec(@VarCode)]);
    }

    oiDebugPrint(@Info, 2, ["completed cat info: ", tCatInfo]);

    pObj.setCatalogInfo(tCatInfo);

    oiDebugFinishFunc(NULL);
}

// ------------------------
// String related functions
// ------------------------

// Returns a duplicate of given string where quote characters are replaced
// by escape sequence '\"'.
public func oiEscapeQuotes(pString)
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

// Returns string where specified padding characters are stripped
// from the end of given string.<br>
// Parameter <i>pPadChar</i> must be a character (Int) or a List of pad 
// characters.<br>
// Additional optional parameter specifies whether padding characters
// have to be removed from the beginning, too. Default is False.
func oiStripPadChar(pStr, pPadChar, ...)
{
    var tPads = pPadChar[0];
    if (tPads instanceof Int)
	tPads = @(pPadChar[0]);
    var tBothSides = 0;
    if (pPadChar.size() > 1)
	tBothSides = pPadChar[1];

    var tL = pStr.size();
    var tPos = tL; // first pos with pad char
    var i;
    for(i=tPos-1; i >= 0; i--)
	if (tPads.find(pStr[i]) >= 0)
	    tPos = i;
	else
	   break;

    var tStr = pStr.substr(0, tPos);

    if (!tBothSides) return(tStr);

    for(i=0; i < tStr.size(); i++)
	if (tPads.find(tStr[i]) < 0) {
	    tPos = i;
	    break;
	}
    return(tStr.substr(tPos));
}

// -------------
// Feedback Box
// -------------

func oiAddFeedbackBox(pObj, pBoxName, pBoxMat)
{
    xxBBox(pObj, Symbol(pBoxName), pObj.getLocalBounds()).setMaterial(pBoxMat);
}

func oiRemoveFeedbackBox(pObj, pBoxName)
{
    var tBox = oiGetFeedbackBox(pObj, pBoxName);

    if (tBox != NULL) pObj.remove(tBox);
}

func oiGetFeedbackBox(pObj, pBoxName)
{
    var tBox;
    try {
	tBox = eval(pObj.getName()+"."+pBoxName);
    }
    return(tBox);
}

// -----------------------------------
// Coordinate system related functions
// -----------------------------------

// Is kept only until next call of oiGetNeighbors() 
// (may be via oiGetBusyAttPts())
func oiSetNeighborTolerance(pEps)
{
    OiFuncs.sNeighborEps = pEps;
}

// Returns a list of neighbor objects of given object, i.e. the sibling
// objects whose bounding box intersects with bounding box of given object
// or is located right beside the bounding box of given object.
// Do not consider objects given in list passed via pExclude.
//
// For calculation, the actual bounding box of pObj is enlarged in all
// dimensions (both in negative and positive direction) by a certain
// epsilon. Default is 0.001. May be changed temporarily by a call of
// oiSetNeighborTolerance(), see above.
// XXX Make this public !?
func oiGetNeighbors(pObj, pExclude)
{
    oiDebugStartFunc("OiFuncs::oiGetNeighbors", [pObj, pExclude]);

    var tExclude = pExclude;
    if (pExclude == NULL)
	tExclude = @();
    var tRet = @();
    var tFather = pObj.getFather();
    var tBounds = pObj.getWorldBounds();
    oiDebugPrint(@Info, 2, ["bounds: ", tBounds]);
    var tEps = OiFuncs.sNeighborEps;
    oiDebugPrint(@Info, 2, ["neighbor tolerance: ", tEps]);
    var tBox1 = [[tBounds[0][0]-tEps, tBounds[0][1]-tEps, tBounds[0][2]-tEps],
		 [tBounds[1][0]+tEps, tBounds[1][1]+tEps, tBounds[1][2]+tEps]];
    oiDebugPrint(@Info, 2, ["box 1:  ", tBox1]);
    var tO;
    foreach(tO; tFather.getElements()) {
	var tObj = tO;
	if (tObj == pObj || tExclude.find(tO) >= 0) continue;
	oiDebugPrint(@Info, 1, ["checking ", tO]);
	var tBox2 = tObj.getWorldBounds();
	oiDebugPrint(@Info, 2, ["box 2:  ", tBox2]);
	if (xgM.boxIntersectsBox(tBox1, tBox2))
	    tRet.pushBack(tObj);
    }
    OiFuncs.sNeighborEps = 0.001; // default
    oiDebugFinishFunc(tRet);
    return(tRet);
}

// Transforms given local coordinates of specified object into
// local coordinates of given (grand) father object.<br>
// Returns NULL if pFather is not a (grand) father of pObj. 
func oiTransformObjCoords(pObj, pCoords, pFather)
{
    oiDebugStartFunc2("OiFuncs::oiTransformObjCoords", [pObj, pCoords, pFather]);

    var tObj = pObj;
    var tCoords = pCoords;

    while (tObj != NULL && tObj != pFather) { 
	var tObjPos = tObj.getPosition();
	var tPos = xgM.transformXZCoords(tObjPos, tObj.getRotation(@PY),
					 tCoords[0], tCoords[2]);
	tCoords = [tPos[0], tObjPos[1]+tCoords[1], tPos[2]];
	tObj = tObj.getFather();
	if (tObj == NULL) tCoords = NULL;
    }
    oiDebugFinishFunc(tCoords);
    return(tCoords);
}

// Returns True, if given 2 coordinates are identical
// (considering a certain epsilon).
func oiAreEqualCoordinates(pCoords1, pCoords2)
{
    oiDebugStartFunc2("OiFuncs::oiAreEqualCoordinates", [pCoords1, pCoords2]);

    var tEps = 0.0001;
    var tRet = (fabs(pCoords1[0]-pCoords2[0]) < tEps &&
		fabs(pCoords1[1]-pCoords2[1]) < tEps &&
		fabs(pCoords1[2]-pCoords2[2]) < tEps);
    oiDebugFinishFunc(tRet);
    return(tRet);
}

// -----------------------------------------
// Utility functions regarding attach points
// -----------------------------------------

func oiGetActiveAttPt(pObj)
{
    oiDebugStartFunc("OiFuncs::oiGetActiveAttPt", pObj);

    if ((!pObj.isCat(@IF_ATTPT) && !pObj.isCat(@IF_AttachPts)) ||
	!pObj.hasMember(@getActiveAttPt)) {
	oiDebugPrint(@Warn, 1, "invalid object!");
	oiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tAttPt = pObj.getActiveAttPt();

    if (tAttPt == NULL) {
	var tCh;
	foreach(tCh; pObj.getChildren())
	    if ((tCh.isA(OiAttPtInteractor) ||
		 (tCh.hasMember(@isCat) && tCh.isCat(@AttPtInteractor))) && 
		tCh.isActive()) {
		tAttPt = tCh.getKey();
		break;
	    }
    }

    oiDebugFinishFunc(tAttPt);
    return(tAttPt);
}

// Precondition: pObj.getStdAttPts().find(pKey) >= 0
func oiGetStdAttPtDef(pObj, pKey)
{
    oiDebugStartFunc("OiFuncs::oiGetStdAttPtDef", [pObj, pKey]);

    var tPlan = oiGetPlanning();
    if (tPlan == NULL) {
	oiDebugPrint(@Warn, 1, "no global planning object!");
	oiDebugFinishFunc(NULL);
	return(NULL);
    }

    // 1. get local coordinates of the standard attach point 
    //    use current local bounds in order to consider current dimensions
    //    (including dynamic children) instead of getWidth() etc.
    var tBB = pObj.getLocalBounds();
    var tW  = tBB[1][0]-tBB[0][0];
    var tH  = tBB[1][1]-tBB[0][1];
    var tD  = tBB[1][2]-tBB[0][2];
    var tO  = [-tBB[0][0], -tBB[0][1], -tBB[0][2]];

    var tKStr = String(pKey);
    var tKW   = tKStr.substr(0, 1);
    var tKH   = tKStr.substr(1, 1);
    var tKD   = tKStr.substr(2, 1);

    var tX, tY, tZ;

    if (tKW == "L")
	tX = -tO[0];
    else
    if (tKW == "C")
	tX = -tO[0] + tW/2;
    else
    if (tKW == "R")
	tX = -tO[0] + tW;

    if (tKH == "B")
	tY = -tO[1];
    else
    if (tKH == "T")
	tY = -tO[1] + tH;

    if (tKD == "B")
	tZ = -tO[2];
    else
    if (tKD == "C")
	tZ = -tO[2] + tD/2;
    else
    if (tKD == "F")
	tZ = -tO[2] + tD;

    var tPos = [tX, tY, tZ];

    // 2. get attach direction for standard attach point 

    var tDir = NULL;

    if (tKH == "T") tDir = @T;
    if (tKH == "B") {
	if (pKey == @CBC)
	    tDir = @B;
	else {
	    if (tKW == "C")
		tDir = Symbol(tKD);
	    else
	    if (tKD == "C")
		tDir = Symbol(tKW);
	    else {
		// corner point -> consider current primary planning direction
		var tDirXZ1 = String(tPlan.getPlanDirXZ())[0];
		if (tDirXZ1 == 'R' || tDirXZ1 == 'L') // x axis rules
		    tDir = Symbol(tKW);
		else // z axis rules
		    tDir = Symbol(tKD);
	    }
	}
    }

    // 3. get attach mode for standard attach point 

    var tMode;
    if (tKH == "B") 
	tMode = @Sibling;
    else // tKH == "T"
	tMode = @Child;

    var tDef = [tPos, tDir, 0.0, tMode];

    oiDebugFinishFunc(tDef);
    return(tDef);
}

// Calculates and checks position for new object (pCh) at given attach
// point (pOAPK) of existing element (pObj).
// 
// If parameter pCAPK is not NULL it is used for determining the
// attach position of pCh, otherwise the first opposite attach point of 
// pOAPK is used.
//
// Parameter pDoCollCheck specifies whether a collision check
// has to be performed for each possible position. 
// (If value is 2, collisions with reference object have to be ignored.)
// 
// Return value is a vector [position, rotation, error_msg, attpt of pCh].
// Position is NULL, if pOAPDef specifies an invalid attach mode or if there 
// is no valid opposite attach point on behalf of pCh.
//
// If position is _not_ NULL, 4th element in return Vector is not NULL and
// specifies the attach point of pCh, used to calculate position/rotation.
//
// Error_msg is NULL if calculated position and rotation for new child pCh
// are ok. In case of collision or scene border violation returns an
// appropriate error message ("::ofml::oi::@coll" in case of 
// collision).
func oiCalcCheckAttPt(pObj, pOAPK, pOAPDef, pDir, pCh, pCAPK, pDoCollCheck)
{
    oiDebugStartFunc("OiFuncs::oiCalcCheckAttPt", 
		     [pObj, pOAPK, pOAPDef, pDir, pCh, pCAPK, pDoCollCheck]);

    var tCAPK = pCAPK;

    var tDir = pDir;
    if (tDir == NULL) tDir = pOAPDef[1];

    if (tCAPK == NULL) {
	// must call oppositeAttPts() of pCh (not of pObj) because new
	// has to decide whether it provides a fitting attach point
	var tCAPKL = pCh.oppositeAttPts(pOAPK, tDir);
	if (tCAPKL == NULL || tCAPKL.empty()) {
	    oiDebugPrint(@Warn, 1, ["no opposite attach point(s) for ", 
				    pOAPK, " in direction ", tDir]);
	    oiDebugFinishFunc([NULL, NULL, NULL, NULL]);
	    return([NULL, NULL, NULL, NULL]);
	}
	tCAPK = tCAPKL.front();
    }

    var tCAPDef = oiGetAttPtDef(pCh, tCAPK);

    oiDebugPrint(@Info, 2, ["opp att pt def: ", tCAPDef]);

    if (tCAPDef == NULL || tCAPDef[0] == NULL || tCAPDef[2] == NULL) {
	oiDebugPrint(@Warn, 1, ["missing or wrong definition for ",
				"opposite attact point ", tCAPK]);
	oiDebugFinishFunc([NULL, NULL, NULL, NULL]);
	return([NULL, NULL, NULL, NULL]);
    }

    var tA = pObj.getRotation(@PY);
    var tPos = pObj.getPosition();
    var tDist = 0.0;
    if (pCh.isA(OiPlElement))
	tDist = pCh.getPDistance();

    var tCPA = _oiPosForAttPt(tPos, tA, pOAPDef, tDir, tCAPDef, tDist);

    oiDebugPrint(@Info, 2, ["calculated pos/rot: ", tCPA]);

    if (tCPA == NULL) {
	// possible only, if pOAPDef specifies an invalid attach mode
	oiDebugFinishFunc([NULL, NULL, NULL, NULL]);
	return([NULL, NULL, NULL, NULL]);
    }

    tPos = tCPA[0];
    tA   = tCPA[1];

    var tErr = NULL;
    if (pDoCollCheck)
	tErr = _oiCheckChildPosRot(pObj, pCh, tPos, tA, pOAPDef[3], 
				   (pDoCollCheck == 2));

    if (tErr == NULL)
	pObj.getDynamicProps()[@IF_ATTPT_LAST_PAIR] = [pOAPK, tCAPK];

    oiDebugFinishFunc([tPos, tA, tErr, tCAPK]);
    return([tPos, tA, tErr, tCAPK]);
}

// Returns position (and rotation) for new child at given attach point (pOAPDef)
// of existing element at given position and @PY rotation (pPos, pA) and given 
// attach point of new child (pCAPDef).
// (Return value is NULL, if pOAPDef specifies an invalid attach mode.)
// Parameter pDir specifies the direction to be used on behalf of the
// attach point of existing element.
// Parameter pDist specifies distance of new element to the existing one.
func _oiPosForAttPt(pPos, pA, pOAPDef, pDir, pCAPDef, pDist)
{
    var tOAPD = pDir;
    var tCAPD = pCAPDef[1];
    var tDX = 0.0;
    var tDZ = 0.0;

    var tDirSign = (pDir == @R || pDir == @F) ? 1 : -1;

    if (tOAPD != NULL && (tCAPD == NULL || (tCAPD != NULL && tOAPD != tCAPD))) {
	if (tOAPD == @R || tOAPD == @L)
	    tDX = tDirSign * pDist;
	else
	if (tOAPD == @F || tOAPD == @B)
	    tDZ = tDirSign * pDist;
    }

    // attachment point for new element
    var tOAPC = pOAPDef[0];
    var tOAPR = pOAPDef[2];
    var tMode = pOAPDef[3];
    var tX, tZ, tAP;
    if (tMode == @Sibling) {
	tX = tOAPC[0]+tDX;
	tZ = tOAPC[2]+tDZ;
	// transform point [tX, tZ] in XZ-plane into higher coordinate system
	// with pPos and pA specifying pos of origin resp. rotation of local
	// coordinate system 
	tAP = [pPos[0]+cos(pA)*tX+sin(pA)*tZ,
	       pPos[1]+tOAPC[1],
	       pPos[2]-sin(pA)*tX+cos(pA)*tZ];
    }
    else
    if (tMode == @Child)
	tAP = tOAPC;
    else
	return(NULL);

    var tCAPC = pCAPDef[0];
    var tCAPR = pCAPDef[2];

    var tPi = 2*acos(0);
    var tA = (tMode == @Sibling ? pA : 0.0) + tOAPR - tCAPR;
    if (tOAPD != NULL && tCAPD != NULL && tOAPD == tCAPD) 
	tA += tPi;

    tX = -tCAPC[0];
    tZ = -tCAPC[2];

    // transform point [tX, tZ] in XZ-plane into higher coordinate system with 
    // tAP and tA specifying pos of origin resp. rotation of local coord. system
    tAP = [tAP[0]+cos(tA)*tX+sin(tA)*tZ, tAP[1], tAP[2]-sin(tA)*tX+cos(tA)*tZ];

    return([[tAP[0], tAP[1]-tCAPC[1], tAP[2]], tA]);
}

// Returns rotation for new child at given attach point (pOAPDef)
// of existing element with given @PY rotation (pA) and given attach point 
// of new child (pCAPDef).
// If pDir is not NULL it overrides the direction of attach point of existing 
// element.
// Used in global function oiGetBusyAttPts().
func _oiRotForAttPt(pA, pOAPDef, pCAPDef, pDir)
{
    var tOAPD = pOAPDef[1];
    if (pDir != NULL)
	tOAPD = pDir;

    var tOAPR = pOAPDef[2];
    var tMode = pOAPDef[3];

    var tCAPD = pCAPDef[1];
    var tCAPR = pCAPDef[2];

    var tPi = 2*acos(0);
    var tA = (tMode == @Sibling ? pA : 0.0) + tOAPR - tCAPR;

    if (tOAPD != NULL && tCAPD != NULL && tOAPD == tCAPD) 
	tA += tPi;

    return(tA);
}

// Returns NULL if given position and rotation for new child pCh are ok.
// In case of collision or scene border violation returns an appropriate
// error message ("::ofml::oi::@coll" in case of collision).
// Parameter pMode specifies add mode.
func _oiCheckChildPosRot(pObj, pCh, pPos, pA, pMode, pIgnRefObjColl)
{
    pCh.setPosition(pPos);
    xgF.setYRotation(pCh, pA);

    var tPos = pCh.getPosition();
    var tR   = pCh.getRotation(@PY);

    var tCollObj;
    if (pMode == @Sibling)
	tCollObj = pObj.getFather().checkChildColl(pCh, NULL);
    else
    if (pMode == @Child)
	tCollObj = pObj.checkChildColl(pCh, NULL);

    if (tCollObj != NULL && 
	!(pIgnRefObjColl && xgF.isObjChildOf(tCollObj, pObj))) 
	return("::ofml::oi::@coll");

    var tPlan = oiGetPlanning();
    if (tPlan == NULL) return(NULL);

    return(tPlan.checkBorderObj(pCh));
}

// Precondition: oiGetPlanning() != NULL
func oiGetBottomAttPts(pObj)
{
    var tPlan = oiGetPlanning();

    var tDirX = String(tPlan.getPlanDirX());
    var tDirZ = String(tPlan.getPlanDirZ());
    var tODirX = (tDirX == "R") ? "L" : "R";
    var tODirZ = (tDirZ == "F") ? "B" : "F";

    var tAlignX = tPlan.getAlignmentX();
    var tAlignZ = tPlan.getAlignmentZ();

    // order of attach points in x- resp. z-direction
    // (alignment rules over planning direction)
    var tSPX = _oiOrderAttPtsX(tAlignX, tDirX);
    var tSPZ = _oiOrderAttPtsZ(tAlignZ, tDirZ);

    var tX, tZ;

    // primary side in x-direction
    var tXSide1 = @();
    foreach(tZ; tSPZ) tXSide1.pushBack([Symbol(tDirX+"B"+tZ), Symbol(tDirX)]);

    // primary side in z-direction
    var tZSide1 = @();
    foreach(tX; tSPX) tZSide1.pushBack([Symbol(tX+"B"+tDirZ), Symbol(tDirZ)]);

    // secondary side in x-direction
    var tXSide2 = @();
    foreach(tZ; tSPZ) tXSide2.pushBack([Symbol(tODirX+"B"+tZ), Symbol(tODirX)]);

    // secondary side in z-direction
    var tZSide2 = @();
    foreach(tX; tSPX) tZSide2.pushBack([Symbol(tX+"B"+tODirZ), Symbol(tODirZ)]);

    // order of sides
    var tPrimDir = String(tPlan.getPlanDirXZ())[0];
    var tSides;
    if (tPrimDir == 'R' || tPrimDir == 'L') // x-axis
	tSides = @(tXSide1, tZSide1, tXSide2, tZSide2);
    else 
	tSides = @(tZSide1, tXSide1, tZSide2, tXSide2);

    var tKeyL = @();

    xgF.copyAggr(pObj.getStdAttPts(), tKeyL, 0);
    var tBPts = @();
    var tSide;
    foreach(tSide; tSides) {
	var tP;
	foreach(tP; tSide) 
	    if (tKeyL.find(tP[0]) >= 0) tBPts.pushBack(tP);
    }

    return(tBPts);
}

// Precondition: oiGetPlanning() != NULL
func oiGetTopAttPts(pObj)
{
    var tPlan = oiGetPlanning();

    var tDirX = String(tPlan.getPlanDirX());
    var tDirZ = String(tPlan.getPlanDirZ());
    var tODirX = (tDirX == "R") ? "L" : "R";
    var tODirZ = (tDirZ == "F") ? "B" : "F";

    var tPrimDir = String(tPlan.getPlanDirXZ())[0];

    var tAlignX = tPlan.getAlignmentX();
    var tAlignZ = tPlan.getAlignmentZ();

    var tTPts = @();
    var t1stP = Symbol(String(tAlignX)+"T"+String(tAlignZ));
    var tLines;
    var tLine1, tLine2, tLine3;
    if (tPrimDir == 'R' || tPrimDir == 'L') { // x-axis
    	var tSPX = _oiOrderAttPtsX(tAlignX, tDirX);
	var tX;
	// front line
	var tFL = @();
	foreach(tX; tSPX) tFL.pushBack([Symbol(tX+"T"+"F"), @T]);
	// center line
	var tCL = @();
	foreach(tX; tSPX) tCL.pushBack([Symbol(tX+"T"+"C"), @T]);
	// back line
	var tBL = @();
	foreach(tX; tSPX) tBL.pushBack([Symbol(tX+"T"+"B"), @T]);
	// order of lines
	if (tDirZ == "F")
	    tLines = @(tBL, tCL, tFL);
	else // tDirZ == "B"
	    tLines = @(tFL, tCL, tBL);
    }
    else {
    	var tSPZ = _oiOrderAttPtsZ(tAlignZ, tDirZ);
	var tZ;
	// left line
	var tLL = @();
	foreach(tZ; tSPZ) tLL.pushBack([Symbol("L"+"T"+tZ), @T]);
	// center line
	var tCL = @();
	foreach(tZ; tSPZ) tCL.pushBack([Symbol("C"+"T"+tZ), @T]);
	// right line
	var tRL = @();
	foreach(tZ; tSPZ) tRL.pushBack([Symbol("R"+"T"+tZ), @T]);
	// order of points
	if (tDirX == "R")
	    tLines = @(tLL, tCL, tRL);
	else // tDirX == "L"
	    tLines = @(tRL, tCL, tLL);
    }

    var tKeyL = @();
    xgF.copyAggr(pObj.getStdAttPts(), tKeyL, 0);
    var tP1;
    var tLine;
    foreach(tLine; tLines) {
	var tP;
	foreach(tP; tLine)
	    if (tP[0] != t1stP) {
		if (tKeyL.find(tP[0]) >= 0) tTPts.pushBack(tP);
	    }
	    else
		tP1 = tP;
    }
    if (tKeyL.find(tP1[0]) >= 0) tTPts.pushFront(tP1);

    return(tTPts);
}

// order of attach points in z-direction
// (alignment rules over planning direction)
func _oiOrderAttPtsZ(pAlignZ, pDirZ)
{
    var tRes;

    if (pAlignZ == @B)
	if (pDirZ == "F")
	    tRes = ["B", "C", "F"];
	else
	    tRes = ["B", "F", "C"];
    else
    if (pAlignZ == @C)
	if (pDirZ == "F")
	    tRes = ["C", "B", "F"];
	else
	    tRes = ["C", "F", "B"];
    else // tAlignZ == @F
	if (pDirZ == "F")
	    tRes = ["F", "B", "C"];
	else
	    tRes = ["F", "C", "B"];

    return(tRes);
}

// order of attach points in x-direction
// (alignment rules over planning direction)
func _oiOrderAttPtsX(pAlignX, pDirX)
{
    var tRes;

    if (pAlignX == @L)
	if (pDirX == "R")
	    tRes = ["L", "C", "R"];
	else
	    tRes = ["L", "R", "C"];
    else
    if (pAlignX == @C)
	if (pDirX == "R")
	    tRes = ["C", "L", "R"];
	else
	    tRes = ["C", "R", "L"];
    else // pAlignX == @R
	if (pDirX == "R")
	    tRes = ["R", "L", "C"];
	else
	    tRes = ["R", "C", "L"];

    return(tRes);
}

func oiShowAttachPts(pObj, pAPL)
{
    oiDebugStartFunc("OiFuncs::oiShowAttachPts", [pObj, pAPL]);

    if (pAPL == NULL) {
	oiDebugFinishFunc(NULL);
	return;
    }

    try {
    var tType = NULL;
    if (pObj.hasMember(@getAttPtInteractorType))
	tType = pObj.getAttPtInteractorType();
    if (tType == NULL || !tType.subClassOf(OiAttPtInteractor))
	tType = OiAttPtInteractor;
    oiDebugPrint(@Info, 2, ["interactor type: ", tType]);

    var tAP;
    foreach(tAP; pAPL) {
	var tAPN = "attpt_"+String(tAP);
	if (afExists(pObj.getName()+"."+tAPN)) continue;
	var tDef = oiGetAttPtDef(pObj, tAP);
	if (tDef == NULL) continue;
	var tXYZ = tDef[0];
	var tRot = tDef[2];
	if (tXYZ == NULL) continue;
	tType(pObj, Symbol(tAPN), tAP);
    }
    }
    catch (&e: Error) {
	 oiDebugPrint(@Warn, 1, e.where + ": " + e.what);
    }

    oiDebugFinishFunc(NULL);
}

// Handles selection of given attach point (interactor) of given object.
func oiAttPtSelected(pObj, pAttPt)
{
    var tAAPK = oiGetActiveAttPt(pObj);
    var tActiveAttPt = NULL; // interactor itself
    if (tAAPK != NULL) {
	var tCh;
	foreach(tCh; pObj.getChildren())
	    if ((tCh.isA(OiAttPtInteractor) ||
		 (tCh.hasMember(@isCat) && tCh.isCat(@AttPtInteractor))) && 
		tCh.getKey() == tAAPK) {
		tActiveAttPt = tCh;
		break;
	    }
    }

    if (tActiveAttPt != pAttPt) {
	if (tActiveAttPt != NULL) {
	    if (tActiveAttPt.isA(OiAttPtInteractor) ||
		(tActiveAttPt.hasMember(@isCat) && 
		 tActiveAttPt.isCat(@AttPtInteractor)))
		tActiveAttPt.setState(@inactive);
	    else
		tActiveAttPt.enable();
	}
	if (pAttPt.isA(OiAttPtInteractor) ||
	    (pAttPt.hasMember(@isCat) && pAttPt.isCat(@AttPtInteractor)))
	    pAttPt.setState(@active);
	else
	    pAttPt.activate();
    }
}


