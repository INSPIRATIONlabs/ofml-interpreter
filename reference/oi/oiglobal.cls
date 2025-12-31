// === ofml/oi/oiglobal.cls ===
package ::ofml::oi;
import ::egr::found::*;











public class OiGlobal
{
    static var sPlanning = NULL; 

    static var sHasOfmlAppFuncRegisterPackage = -1;
    static var sHasOfmlAppFuncGetLanguage = -1;
    static var sHasOfmlAppFuncGetLengthInAppUnit = -1;
    static var sHasOfmlAppFuncGetMeasureParameters = -1;

    
    
    static var sAttPt4PosRot = NULL;

    
    
    
    
    
    static var sAttPts4PosRot = Hash(); 
}

OiOAM();



public func oiGetPlanning()
{
    

    var tScene;

    try { tScene = ::t; }

    if (tScene != NULL && tScene.isA(OiPlanning)) return(tScene);

    return(NULL);
}


public func oiGetChangeManager()
{
    var tPlan = oiGetPlanning();

    if (tPlan == NULL) return(NULL);

    return(tPlan.getChangeManager());
}


public func oiGetOdbParamManager()
{
    var tPlan = oiGetPlanning();

    if (tPlan == NULL) return(NULL);

    return(tPlan.getOdbParamManager());
}


public func oiGetPlGroupManager()
{
    var tPlan = oiGetPlanning();

    if (tPlan == NULL) return(NULL);

    return(tPlan.getPlGroupManager());
}


public func oiGetCurrentDate()
{
    oiDebugStartFunc("OiGlobal::oiGetCurrentDate", NULL);

    var tTime = ::time::localtime(::time::time());
    var tDay = String(tTime[@DAY_OF_MONTH]);
    if (tDay.size() == 1) tDay = "0" + tDay;
    var tMonth = String(tTime[@MONTH]);
    if (tMonth.size() == 1) tMonth = "0" + tMonth;
    var tCurrentDate = String(tTime[@YEAR]) + tMonth + tDay;

    oiDebugFinishFunc(tCurrentDate);
    return(tCurrentDate);
}


public func oiGenerateUUID()
{
    return(afGenerateUUID());
}


public func oiClearHistory()
{
    afClearHistory();
}


public func oiCollision(pObject1, pObject2)
{
    return(afCollision(pObject1, pObject2));
}


public func oiClone(pSrc, pDest)
{
    if (pSrc == NULL || !(pDest instanceof String)) return(NULL);
    return(afClone(pSrc, pDest));
}


public func oiCopy(pObject)
{
    return(afCopy(pObject));
}


public func oiCut(pObject)
{
    return(afCut(pObject));
}


public func oiDump2String(pObj)
{
    return(afDump2String(pObj));
}





public func oiGetTopCompositeArticle(pObj)
{
    oiDebugStartFunc("OiGlobal::oiGetTopCompositeArticle", pObj);

    var tComposite = NULL;

    var tF, tObj = pObj;
    while ((tF = tObj.getFather()) != NULL) {
	tObj = tF;
	if (tObj.isCat(@IF_CompositeArticle)) tComposite = tObj;
    }

    oiDebugFinishFunc(tComposite);
    return(tComposite);
}






public func oiGetCompositeArticleUp(pObj)
{
    oiDebugStartFunc("OiGlobal::oiGetCompositeArticleUp", pObj);

    var tComposite = NULL;

    var tF = pObj;
    while ((tF = tF.getFather()) != NULL)
	if (tF.hasMember(@isCat) && tF.isCat(@IF_CompositeArticle))  {
	    tComposite = tF;
	    break;
        }

    oiDebugPrint(@Info, 2, ["composite ", tComposite]);

    if (tComposite == NULL ||
	(tComposite.isCat(@IF_MetaType) && tComposite.getMainChild() == pObj)) {
       oiDebugFinishFunc(tComposite);
       return(tComposite);
    }

    var tSubArticles = @();
    var tSubID;
    foreach(tSubID; tComposite.getSubArticleIDs())
	tSubArticles.pushBack(tComposite.getSubArticle(tSubID));

    oiDebugPrint(@Info, 2, ["sub articles: ", tSubArticles]);

    if (tSubArticles.find(pObj) < 0) tComposite = NULL;

    oiDebugFinishFunc(tComposite);
    return(tComposite);
}





















public func oiGetPropVarCode(pObj, pState)
{
    oiDebugStartFunc("OiGlobal::oiGetPropVarCode", [pObj, pState]);

    if (pObj == NULL || !pObj.hasMember(@isCat) ||
        !(pObj.isCat(@IF_Property) && pObj.isCat(@IF_Article))) {
	oiDebugPrint(@Warn, 1, "invalid object");
	oiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tRet = "";
    var tProps = pObj.getPropKeys();
    var tP;
    foreach(tP; tProps) {
	var tState = pObj.getPropState2(tP);
	if (tState < pState) {
	    oiDebugPrint(@Info, 1, 
			 ["skipping property ", tP, " with state ", tState]);
	    continue;
	}
	var tValue = pObj.getPropValue(tP);
	var tValStr = xgF.param2Str(tValue);
	var tValLen = tValStr.size();
	oiDebugPrint(@Info, 2, ["value ", tValue, " (", tValLen, 
			        ") represented as \"", tValStr, "\""]);
	tRet += (tRet.empty() ? "" : ";") + String(tP) + "=" + tValStr;
    }

    oiDebugFinishFunc(tRet);
    return(tRet);
}






public func oiRegisterPackage(pPID)
{
    oiDebugStartFunc("OiGlobal::oiRegisterPackage", pPID);

    if (OiGlobal.sHasOfmlAppFuncRegisterPackage == 0) {
	oiDebugPrint(@Warn, 1, "not supported by application");
	oiDebugFinishFunc(0);
	return(0);
    }

    var tOk = 0;
    var tRet;

    if (OiGlobal.sHasOfmlAppFuncRegisterPackage == 1) {
	tRet = ::ofml::app::registerPackage(xgF.param2Str(pPID));
	if (tRet instanceof String && !tRet.empty()) {
	    if (tRet[0] == '@') tRet = tRet.substr(1);
	    if (Symbol(tRet) == pPID) tOk = 1;
	}
	oiDebugFinishFunc(tOk);
	return(tOk);
    }

    OiGlobal.sHasOfmlAppFuncRegisterPackage = 1;
    try {
	tRet = ::ofml::app::registerPackage(xgF.param2Str(pPID));
    }
    catch (&e: Error) {
	OiGlobal.sHasOfmlAppFuncRegisterPackage = 0;
	oiDebugPrint(@ExplWarn, 1, 
		      "Error during evaluation of function " +
		      "::ofml::app::registerPackage(): " + e.what);
    }
    if (tRet instanceof String && !tRet.empty()) {
	if (tRet[0] == '@') tRet = tRet.substr(1);
	if (Symbol(tRet) == pPID) tOk = 1;
    }
    oiDebugFinishFunc(tOk);
    return(tOk);
}





public func oiGetAppLanguage()
{
    oiDebugStartFunc("OiGlobal::oiGetAppLanguage", NULL);

    var tRes = NULL;

    if (OiGlobal.sHasOfmlAppFuncGetLanguage == 0) {
	oiDebugFinishFunc(tRes);
	return(tRes);
    }

    if (OiGlobal.sHasOfmlAppFuncGetLanguage == 1) {
        tRes = ::ofml::app::getLanguage("");
	if (tRes == "NULL") tRes = NULL;
	oiDebugFinishFunc(tRes);
	return(tRes);
    }

    OiGlobal.sHasOfmlAppFuncGetLanguage = 1;
    try {
        tRes = ::ofml::app::getLanguage("");
	if (tRes == "NULL") tRes = NULL;
    }
    catch (&e: Error) {
	OiGlobal.sHasOfmlAppFuncGetLanguage = 0;
	oiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function getLanguage()!\n");
    }

    oiDebugFinishFunc(tRes);
    return(tRes);
}


public func oiGetLengthInAppUnit(pLen)
{
    oiDebugStartFunc("OiGlobal::oiGetLengthInAppUnit", pLen);

    var tLen = pLen;

    if (!(pLen instanceof Float || pLen instanceof Int)) {
	oiDebugPrint(@Warn, 1, "wrong parameter type!");
	if (pLen instanceof String) {
	    
	    try {
		tLen = eval(pLen);
	    }
	    catch (&e: Error) {
		oiDebugPrint(@Warn, 1, 
			     "couldn't convert String parameter: "+e.what);
	    }
	    if (!(tLen instanceof Float || tLen instanceof Int)) {
		oiDebugFinishFunc(NULL);
		return(NULL);
	    }
	}
	else {
	    oiDebugFinishFunc(NULL);
	    return(NULL);
	}
    }

    var tFB = String(xgM.getRoundFloat(tLen, 3)) + " m"; 

    if (OiGlobal.sHasOfmlAppFuncGetLengthInAppUnit == 0) {
	oiDebugPrint(@Info, 1, "using fallback");
	oiDebugFinishFunc(tFB);
	return(tFB);
    }

    var tRes = NULL;

    if (OiGlobal.sHasOfmlAppFuncGetLengthInAppUnit == 1) {
        tRes = ::ofml::app::getLengthInAppUnit(String(tLen));
	oiDebugPrint(@Info, 1, "using result of app-callback");
	oiDebugFinishFunc(tRes);
	return(tRes);
    }

    OiGlobal.sHasOfmlAppFuncGetLengthInAppUnit = 1;
    try {
        tRes = ::ofml::app::getLengthInAppUnit(String(tLen));
	oiDebugPrint(@Info, 1, "using result of app-callback");
    }
    catch (&e: Error) {
	OiGlobal.sHasOfmlAppFuncGetLengthInAppUnit = 0;
	tRes = tFB;
	oiDebugPrint(@ExplWarn, 1,
	    "Application does not support package ::ofml::app or that\n" +
	    "package does not implement function getLengthInAppUnit()!\n");
    }

    oiDebugFinishFunc(tRes);
    return(tRes);
}


public func oiGetMeasureParameters()
{
    return(_oiGetMeasureParameters(""));
}


public func oiGetMeasureParameters2(pView)
{
    return(_oiGetMeasureParameters(pView));
}

private func _oiGetMeasureParameters(pArgs)
{
    oiDebugStartFunc("OiGlobal::_oiGetMeasureParameters", pArgs);

    var tParams = NULL;

    
    var tScale = 0.05;
    var tFont = "default";
    var tMat = "$ amb 0 0 0";
    var tShowUnit = NULL;
    var tTerminationType = @Arrow;

    var tProjLineColor = NULL;
    var tDimLineColor  = NULL;

    var tArgs = pArgs;
    if (!(pArgs instanceof String)) {
	oiDebugPrint(@Warn, 1, 
		     ["invalid parameter for oiGetMeasureParameters2: ", pArgs,
		      "! Using empty String instead."]);
	tArgs = "";
    }

    var tRes;

    if (OiGlobal.sHasOfmlAppFuncGetMeasureParameters == 1)
        tRes = ::ofml::app::getMeasureParameters(tArgs);
    else
    if (OiGlobal.sHasOfmlAppFuncGetMeasureParameters == -1) {
	OiGlobal.sHasOfmlAppFuncGetMeasureParameters = 1;
	try {
            tRes = ::ofml::app::getMeasureParameters(tArgs);
	}
	catch (&e: Error) {
	    oiDebugPrint(@ExplWarn, 1, 
			 "Application function " +
			 "::ofml::app::getMeasureParameters() not available!");
	    OiGlobal.sHasOfmlAppFuncGetMeasureParameters = 0;
	}
    }

    if (tRes != NULL && !tRes.empty()) {
	try {
	    tParams = eval(tRes);
	}
	catch (&e: Error) {
	    oiDebugPrint(@ExplWarn, 1, 
			 ["Wrong result from ",
			  "::ofml::app::getMeasureParameters(): ", tRes, "!"]);
	}
    }

    if (tParams == NULL && tArgs == "2D") {
        oiDebugFinishFunc(NULL);
	return(NULL);
    }

    if (tParams != NULL) {
	var tP;
	foreach(tP; tParams) {
	    if (tP[0] == @TextHeight) {
		if (tP[1] > 0.0001) tScale = tP[1];
		continue;
	    }
	    if (tP[0] == @TextFont) {
		tFont = tP[1];
		continue;
	    }
	    if (tP[0] == @Color || tP[0] == @TextColor) {
		tMat = "$ amb " + xgF.param2Str(tP[1][0]) + " " + 
				  xgF.param2Str(tP[1][1]) + " " + 
			   	  xgF.param2Str(tP[1][2]);
		continue;
	    }
	    if (tP[0] == @ProjLineColor) {
		tProjLineColor = "$ amb " + xgF.param2Str(tP[1][0]) + " " + 
				            xgF.param2Str(tP[1][1]) + " " + 
			   	            xgF.param2Str(tP[1][2]);
		continue;
	    }
	    if (tP[0] == @DimLineColor) {
		tDimLineColor = "$ amb " + xgF.param2Str(tP[1][0]) + " " + 
				           xgF.param2Str(tP[1][1]) + " " + 
			   	           xgF.param2Str(tP[1][2]);
		continue;
	    }
	    if (tP[0] == @ShowUnit) {
		tShowUnit = tP[1];
		continue;
	    }
	    if (tP[0] == @TerminationType) {
		tTerminationType = tP[1];
		continue;
	    }
	    oiDebugPrint(@ExplWarn, 1, 
			 ["::ofml::app::getMeasureParameters(): ",
			  "unrecognized parameter ", tP[0]]);
	}
    }

    if (tProjLineColor == NULL)
	tProjLineColor = tMat;
    if (tDimLineColor == NULL)
	tDimLineColor = tMat;

    var tRet = [tScale, tFont, tMat, tShowUnit, 
		tTerminationType, tProjLineColor, tDimLineColor];

    oiDebugFinishFunc(tRet);
    return(tRet);
}






public func oiGetOrderUnitDescr(pCode, pLanguage)
{
    var tResource = "::ofml::oi::@OrderUnit_"+pCode;
    var tTxt = oiGetStringResource(tResource, pLanguage);

    var tDescr = pCode;
    if (tTxt != tResource) 
	tDescr = tTxt;

    return(tDescr);
}


public func oiReplace(pObj, pBuf)
{
    afReplace(pObj, pBuf);
}


public func oiOutput(pLevel, pMessage)
{
    if (pLevel == @MESSAGE)
	return(afMessage(pMessage));
    if (pLevel == @WARNING)
	return(afWarning(pMessage));
    if (pLevel == @ERROR) {
	xxComplex.sError = pMessage;
	return(afError(pMessage));
    }
    if (pLevel == @FATAL)
	return(afFatal(pMessage));

    var tS;

    if (pLevel == NULL) tS = "void";
    else tS = String(pLevel);

    afError("Symbol ["+tS+"] for parameter [pLevel] in function [oiOutput]"+
            " is undefined.");
}


public func oiGetStringResource(pStr, pLanguage, ...)
{
    if (pStr == NULL || !(pStr instanceof String || pStr instanceof Vector))
	return(NULL);
    var tLanguage = pLanguage[0];
    if (tLanguage == NULL || !(tLanguage instanceof String))
	return(pStr);
    var tRes = pStr;
    if (pLanguage.size() > 1)
	
	tRes = afGetStringResource(pStr, tLanguage, pLanguage[1]);
    else
	tRes = afGetStringResource(pStr, tLanguage);

    return(tRes);
}


public func oiGetMaterialDefinition(pMatName)
{
    oiDebugStartFunc("OiGlobal::oiGetMaterialDefinition", pMatName);

    var tRes = afGetMaterialDefinition(pMatName);

    oiDebugFinishFunc(tRes);
    return(tRes);
}


public func oiExists(pName)
{
    return(afExists(pName));
}


public func oiGetNearestObject(pPosition, pDirection)
{
    return(afGetNearestObject(pPosition, pDirection));
}


public func oiGetDistance(pPosition, pDirection)
{
    return(afGetDistance(pPosition, pDirection));
}


public func oiGetRoots()
{
    return(afGetRoots());
}

private func _oiGetNameForNewElem(pFather)
{
    
    
    var tNum = 0;
    var tCh;
    foreach(tCh; pFather.getChildren()) {
	var tName = tCh.getName();
	tName = tName.substr(pFather.getName().size() + 1);
	if (tName[0] == 'e') {
	    var tDigits = 1;
	    var i;
	    for (i=1; i < tName.size(); i++)
		if (tName[1] < '0' || tName[1] > '9') {
		    tDigits = 0;
		    break;
		}
	    if (tDigits) {
		var tN = Int(tName.substr(1));
		if (tN > tNum) tNum = tN;
	    }
	}
    }
    return(Symbol("e" + String(tNum+1)));
}


public func oiPaste(pFather, pName)
{
    TFather = pFather;
    if (pName != NULL)
	TName = pName;
    else
	TName = _oiGetNameForNewElem(pFather);

    try {
	afPaste();
    }
    catch (&e: Error) {
	::stdio::stderr.printf("%s: %s\n", e.where, e.what);
    }

    
    var tObj = NULL;
    if (afExists(pFather.getName()+"."+String(TName))) tObj = pFather.$(TName);

    return(tObj);
}

public func oiReparent(pFather, pObj)
{
    var tName = afReparent(pFather, pObj);
    var tObj = NULL;
    try {
	tObj = eval(tName);
    }
    catch (&e: Error) {
	::stdio::stderr.printf("%s: %s\n", e.where, e.what);
    }

    return(tObj);
}

public func oiApplPaste(pFather, pName)
{
    var tName;

    if (pName != NULL)
	tName = pName;
    else {
    	if (pFather == NULL) return;

	tName = _oiGetNameForNewElem(pFather);
    }

    return(afXPaste(pFather.getName()+"."+String(tName)));
}


public func oiQuestion(pFormat, pQuestion)
{
    return(afQuestion(pFormat, pQuestion));
}


public func oiDialog(pDialog, pIcon, pMessage)
{
    var tDialogs = @(@OK, @OK_CAN, @ABT_IGN, @YES_NO_CAN, @YES_NO);
    if (tDialogs.find(pDialog) < 0)
	return(@INVALID_DIALOG);

    var tIcons = @(@NONE, @STOP, @QUESTION, @WARNING, @INFO);
    if (pIcon != NULL && tIcons.find(pIcon) < 0)
	return(@INVALID_ICON);

    var tIconStr = "";
    if (pIcon != NULL && pIcon != @NONE)
	tIconStr = "_" + String(pIcon);

    var tMessage = pMessage;
    if (pMessage == NULL || 
	!((pMessage instanceof String) || (pMessage instanceof Vector)))
	tMessage = "";

    return(afQuestion("@PD_"+String(pDialog)+tIconStr, tMessage));
}


public func oiSetCheckString(pString)
{
    return(afSetCheckString(pString));
}


public func oiTable(pRequest, pArgs, ...)
{
    var tArgs = pArgs[0];
    var tOptions = NULL;
    if (pArgs.size() > 1)
	tOptions = pArgs[1];

    var tRes;
    try {
	if (tOptions == NULL || pRequest != @readTE)
            tRes = afTable(pRequest, tArgs);
	else
	    tRes = afTable(pRequest, tArgs, tOptions);
    }
    catch (&e: Error) {
        oiDebugPrint(@TableErr, 1, ["Error during oiTable(", pRequest, ", ", 
				pArgs, "): ", e.what]);
        if (pRequest == @openTbl) tRes = 0;
    }
    
    return(tRes);
}



public func oiIsTmpCh(pObj)
{
    var tF = pObj.getFather();

    if (tF != NULL && tF.hasMember(@isCat) && 
	tF.isCat(@IF_Complex) && tF.objIsTmpCh(pObj)) return(1);

    var tRes = 0;
    while (tF != NULL) {
	var tF2 = tF.getFather();
	if (tF2 != NULL && tF2.hasMember(@isCat) &&
	    tF2.isCat(@IF_Complex) && tF2.objIsTmpCh(tF)) {
	    tRes = 1;
	    break;
	}
	else
	    tF = tF2;
    }
    return(tRes);
}



func oiSubstString(pStr, pWhat, pBy)
{
    var tStr = String(pStr);
    var tPos = 0;
    var tWhatLen = pWhat.size();
    var tByLen   = pBy.size();

    while ((tPos = tStr.find(pWhat, tPos)) >= 0) {
	tStr.replace(tPos, tWhatLen, pBy);
	tPos += tByLen;
    }

    return(tStr);
}












func oiParseString2List(pStr, pSeparator, ...)
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
	do {
	    tSepPos = pStr.find(tSeparator, tPos1);
	    tPos2 = tSepPos;
	    if (tSepPos < 0) tPos2 = pStr.size();
	    tRes.pushBack(pStr.substr(tPos1, tPos2-tPos1));
	    tPos1 = tPos2+1;
	} while (tSepPos > 0 && tPos1 < pStr.size());
    }
    else {
	while (1) {
	    tPos1 = pStr.find(tSeparator[0], tPos1);
	    if (tPos1 < 0) break;
	    tPos2 = pStr.find(tSeparator[1], tPos1);
	    if (tPos2 < 0) break;
	    tRes.pushBack(pStr.substr(tPos1+1, tPos2-tPos1-1));
	    tPos1 = tPos2+1;
	}
    }

    if (!tStripPads) return(tRes);

    var tRes2 = @();
    var tStr;
    foreach(tStr; tRes) tRes2.pushBack(oiStripPadChar(tStr, ' ', 1));

    return(tRes2);
}










public func oiGetStdAttPts()
{
    return(@(@LBF, @CBF, @RBF, @RBC, @RBB, @CBB, @LBB, @LBC, @CBC,
	     @LTF, @CTF, @RTF, @RTC, @RTB, @CTB, @LTB, @LTC, @CTC));
}









public func oiGetStdAttPtsOrder(pObj)
{
    oiDebugStartFunc("OiGlobal::oiGetStdAttPtsOrder", pObj);

    var tResL = @();

    var tSc = oiGetPlanning();

    if (tSc != NULL) {
        if (tSc.getPlanDirS() == @H)
	    tResL = oiGetBottomAttPts(pObj);
        else 
	    tResL = oiGetTopAttPts(pObj);
    }

    oiDebugFinishFunc(tResL);
    return(tResL);
}








public func oiGetOppositeAttPts4Std(pObj, pKey, pDir)
{
    oiDebugStartFunc("OiGlobal::oiGetOppositeAttPts4Std", [pObj, pKey, pDir]);

    var tRes = @();

    var tSc = oiGetPlanning();
    var tStdAttPts = pObj.getStdAttPts();

    oiDebugPrint(@Info, 2, ["standard attach points: ", tStdAttPts]);

    if (tSc == NULL || tStdAttPts == NULL || tStdAttPts.find(pKey) < 0 || 
	pKey == @CBC) {
	if (tStdAttPts == NULL)
	    oiDebugPrint(@Warn, 1, "getStdAttPts() returned NULL!");
	oiDebugFinishFunc(tRes);
	return(tRes);
    }

    var tKStr = String(pKey);
    var tKW   = tKStr.substr(0, 1);
    var tKH   = tKStr.substr(1, 1);
    var tKD   = tKStr.substr(2, 1);

    if (tKH == "B") {
	if (tKW == "L") xgF.copyAggr(@(@RBB, @RBC, @RBF), tRes, 0);
	if (tKW == "R") xgF.copyAggr(@(@LBB, @LBC, @LBF), tRes, 0);
	if (tKD == "B") xgF.copyAggr(@(@LBF, @CBF, @RBF), tRes, 0);
	if (tKD == "F") xgF.copyAggr(@(@LBB, @CBB, @RBB), tRes, 0);

	var tDir;
	if (pDir != NULL) 
	    tDir = String(pDir)[0];
	else
	    tDir = String(tSc.getPlanDirXZ())[0];

	if (tDir == 'R')
	    tKW = "L";
	else
	if (tDir == 'L')
	    tKW = "R";
	else
	if (tDir == 'F')
	    tKD = "B";
	else
	    tKD = "F";
    }
    else 
	tKH = "B";

    var tBest = Symbol(tKW+tKH+tKD);

    tRes.remove(tBest);
    tRes.pushFront(tBest);

    oiDebugFinishFunc(tRes);
    return(tRes);
}







public func oiGetAttPtDef(pObj, pKey)
{
    oiDebugStartFunc("OiGlobal::oiGetAttPtDef", [pObj, pKey]);

    if ((!pObj.isCat(@IF_ATTPT) && !pObj.isCat(@IF_AttachPts)) ||
	!pObj.hasMember(@getStdAttPts)) {
	oiDebugPrint(@Warn, 1, "invalid object!");
	oiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tStdAttPts = pObj.getStdAttPts();

    var tIsStd = (tStdAttPts != NULL && tStdAttPts.find(pKey) >= 0);

    var tDef = NULL;

    if (tIsStd)
	tDef = oiGetStdAttPtDef(pObj, pKey);
    else
    if (pObj.hasMember(@getAddAttPtDef))
	tDef = pObj.getAddAttPtDef(pKey);
    else
	oiDebugPrint(@Warn, 1, "invalid object!");

    oiDebugFinishFunc(tDef);
    return(tDef);
}












































public func oiGetPosRot4AttachPts(pRefObj, pTarget, ...)
{
    oiDebugStartFunc("OiGlobal::oiGetPosRot4AttachPts", [pRefObj, pTarget]);

    OiGlobal.sAttPt4PosRot = NULL;

    var tRefName = pRefObj.getName();

    if (OiGlobal.sAttPts4PosRot.hasKey(tRefName))
        OiGlobal.sAttPts4PosRot.remove(tRefName);

    var tTarget = pTarget[0];

    if ((!pRefObj.isCat(@IF_ATTPT) && !pRefObj.isCat(@IF_AttachPts)) ||
	(!tTarget.isCat(@IF_ATTPT) && !tTarget.isCat(@IF_AttachPts))) {
	oiDebugPrint(@Warn, 1, "invalid objects!");
	oiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tChAttPt = NULL;
    if (pTarget.size() > 1)
	tChAttPt = pTarget[1];
    var tDoCollCheck = 1;
    if (pTarget.size() > 2)
	tDoCollCheck = pTarget[2];
    var tReportErr = 0;
    if (pTarget.size() > 3)
	tReportErr = pTarget[3];
    var tInsertAllowed = 0;
    if (pTarget.size() > 4)
	tInsertAllowed = pTarget[4];

    if (pRefObj.getDynamicProps().hasKey(@IF_ATTPT_LAST_PAIR))
	pRefObj.getDynamicProps().remove(@IF_ATTPT_LAST_PAIR);

    var tIsChild = (tTarget.getFather() == pRefObj);

    oiDebugPrint(@Info, 2, [ "is child?: ", tIsChild]);

    var tOAPL = pRefObj.getAttPtsOrder();
    var tBusyAttPts = @();

    oiDebugPrint(@Info, 2, [ "list of att pts: ", tOAPL]);

    if (tDoCollCheck == 2)  
	
	
	tBusyAttPts = oiGetBusyAttPts(pRefObj, tOAPL, tTarget);

    

    var tActAPK = oiGetActiveAttPt(pRefObj); 
    oiDebugPrint(@Info, 2, [ "active attach point: ", tActAPK]);

    if (tActAPK != NULL) {
	if (tBusyAttPts.find(tActAPK) >= 0) {
	    oiDebugPrint(@Warn, 1, 
			 ["active attach point ", tActAPK, " is busy!"]);
	    oiDebugFinishFunc(NULL);
	    return(NULL);
	}

        var tActAPDef = oiGetAttPtDef(pRefObj, tActAPK);
        if (tActAPDef == NULL || tActAPDef[0] == NULL || tActAPDef[2] == NULL) {
	    oiDebugPrint(@Warn, 1, ["missing or wrong attach point definition ",
				    "for active attach point ", tActAPK]);
	    oiDebugFinishFunc(NULL);
	    return(NULL);
	}
	if ((tActAPDef[3] == @Child) != tIsChild) {
	    oiDebugPrint(@Warn, 1, 
			 ["active attach point ", tActAPK,
			  " cannot be used due to mode ", tActAPDef[3]]);
	    oiDebugFinishFunc(NULL);
	    return(NULL);
	}

        var tCPE = oiCalcCheckAttPt(pRefObj, tActAPK, tActAPDef, NULL, tTarget, 
				    tChAttPt, tDoCollCheck);
        var tRet = Vector(3);
        tRet[0] = tCPE[0];
        tRet[1] = tCPE[1];
        tRet[2] = tCPE[2];

        var tErr = tRet[2];

        if (tErr != NULL && tErr != "::ofml::oi::@coll") tRet[0] = NULL;

	if (tRet[0] != NULL && 
	    (tErr == NULL ||
	     (tErr == "::ofml::oi::@coll" && tInsertAllowed))) {
	    OiGlobal.sAttPt4PosRot = tActAPK;
	    OiGlobal.sAttPts4PosRot[tRefName] = [tActAPK, tCPE[3]];
	}

	oiDebugFinishFunc(tRet);
        return(tRet);
    }

    if (tOAPL == NULL || tOAPL.empty()) {
	oiDebugPrint(@Warn, 1, "no attach point order available!");
	oiDebugFinishFunc(NULL);
	return(NULL);
    }

    var tOAPKD;
    var tCP;
    var tA;
    var tErr;
    foreach(tOAPKD; tOAPL) {
        var tOAPK = tOAPKD[0]; 
        var tOAPD = tOAPKD[1]; 
	if (tBusyAttPts.find(tOAPK) >= 0) {
	    oiDebugPrint(@Warn, 1, ["ignoring busy attach point ", tOAPK]);
	    continue;
	}
        var tOAPDef = oiGetAttPtDef(pRefObj, tOAPK);
        oiDebugPrint(@Info, 2, ["APDef: ", tOAPDef]);
        if (tOAPDef == NULL || tOAPDef[0] == NULL || tOAPDef[2] == NULL ||
	    (tOAPDef[3] == @Child) != tIsChild) continue;

        var tCPE = oiCalcCheckAttPt(pRefObj, tOAPK, tOAPDef, tOAPD, tTarget, 
				    tChAttPt, tDoCollCheck);
        oiDebugPrint(@Info, 2, ["tCPE: ", tCPE]);
        tCP  = tCPE[0];
        tA   = tCPE[1];
        tErr = tCPE[2];

        if (tCP == NULL) continue;

	
        if (tErr == NULL ||
	    (tErr == "::ofml::oi::@coll" && tInsertAllowed)) {
	    OiGlobal.sAttPt4PosRot = tOAPK;
	    OiGlobal.sAttPts4PosRot[tRefName] = [tOAPK, tCPE[3]];
	    break;
	}

	if (!tReportErr || (tErr != NULL && tErr != "::ofml::oi::@coll"))
            tCP = NULL;
    }

    if (!tReportErr)
	
	tErr = NULL;

    oiDebugFinishFunc([tCP, tA, tErr]);
    return([tCP, tA, tErr]);
}







public func oiGetAttPt4LastPosRot()
{
    oiDebugStartFunc("OiGlobal::oiGetAttPt4LastPosRot", NULL);

    oiDebugFinishFunc(OiGlobal.sAttPt4PosRot);
    return(OiGlobal.sAttPt4PosRot);
}

















public func oiGetAttPt4LastPosRot2(pRefObj)
{
    oiDebugStartFunc("OiGlobal::oiGetAttPt4LastPosRot2", pRefObj);

    var tRefName = pRefObj.getName();

    var tRet = NULL;
    if (OiGlobal.sAttPts4PosRot.hasKey(tRefName))
	tRet = OiGlobal.sAttPts4PosRot[tRefName];

    oiDebugFinishFunc(tRet);
    return(tRet);
}


























func oiGetBusyAttPts(pObj, pAttPts, pExclObj, ...)
{
    oiDebugStartFunc("OiGlobal::oiGetBusyAttPts", [pObj, pAttPts, pExclObj]);

    var tExclObj = pExclObj[0];
    var tEps = NULL;
    if (pExclObj.size() > 1 && pExclObj[1] instanceof Float)
	tEps = pExclObj[1];
    var tNeedObjects = 0;
    if (pExclObj.size() > 2 && pExclObj[2] instanceof Int)
	tNeedObjects = pExclObj[2];

    var tRet = @();
    var tRet2 = @();

    if (pAttPts == NULL || pAttPts.empty()) {
	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    oiDebugPrint(@Info, 2, 
		 ["count of attach points to be checked: ", pAttPts.size()]);

    var tPi = 2*acos(0);
    var tValidModes = @(@Child, @Sibling);

    var tNeighbors = NULL;
    var tElements  = pObj.getElements();

    var tA = pObj.getRotation(@PY);
    oiDebugPrint(@Info, 2, ["object's rotation: ", tA*180/tPi]);

    var tAP;
    foreach(tAP; pAttPts) {
	var tAPKey = tAP[0];
	var tAPDef = oiGetAttPtDef(pObj, tAPKey); 
	oiDebugPrint(@Info, 1, ["checking attach point ", tAPKey, 
				" with definition ", tAPDef]);
	if (tAPDef == NULL) {
	    oiDebugPrint(@Warn, 1, 
			 ["missing definition for attach point ", tAPKey]);
	    continue;
	}
	var tAPPos  = tAPDef[0];
	var tAPDir  = tAPDef[1];
	var tAPRot  = tAPDef[2];
	var tAPMode = tAPDef[3];
	if (tAPPos == NULL || !(tAPMode instanceof Symbol) ||
	    tValidModes.find(tAPMode) < 0) {
	    oiDebugPrint(@Warn, 1, 
			 ["invalid definition for attach point ", tAPKey]);
	    continue;
	}
	var tObjects = tElements;
	var tAPCoords = tAPPos;
	if (tAPMode == @Sibling) {
	    if (tNeighbors == NULL) {
		if (tEps != NULL) oiSetNeighborTolerance(tEps);
		tNeighbors = 
		    oiGetNeighbors(pObj, tExclObj == NULL ? NULL : @(tExclObj));
	    }
	    if (tNeighbors.empty()) {
		oiDebugPrint(@Info, 1, "no siblings for @Sibling attach point");
		continue;
	    }
	    tObjects = tNeighbors;
	    tAPCoords = oiTransformObjCoords(pObj, tAPPos, pObj.getFather());
	}
	oiDebugPrint(@Info, 2, ["coordinates of attach point: ", tAPCoords]);
	var tO;
	foreach(tO; tObjects) {
	    var tORot = tO.getRotation(@PY);
	    oiDebugPrint(@Info, 1, ["checking ", tAPMode, " ", tO, 
				    " (rotation: ", tORot*180/tPi, ")"]);
	    if (tExclObj != NULL && tO == tExclObj) continue;
	    if (!(tO.isCat(@IF_AttachPts) || tO.isCat(@IF_ATTPT))) {
		oiDebugPrint(@Info, 1, 
			     [tO, " does not implement interface AttachPts"]);
		continue;
	    }
	    var tOppAttPts = tO.oppositeAttPts(tAPKey, tAPDir);
	    if (tOppAttPts == NULL || tOppAttPts.empty()) {
		oiDebugPrint(@Info, 1, 
			     [tO, " has no opposite attach point(s) for ", 
			      tAPKey]);
		continue;
	    }
	    oiDebugPrint(@Info, 2, 
			 ["fitting opposite attach points: ", tOppAttPts]);
	    var tOppAP;
	    foreach(tOppAP; tOppAttPts) {
		var tOppAPKey = tOppAP;
		var tOppAPDef = oiGetAttPtDef(tO, tOppAP);
		oiDebugPrint(@Info, 2, ["opp att pt def: ", tOppAPDef]);
		if (tOppAPDef == NULL || tOppAPDef[0] == NULL ||
		    tOppAPDef[2] == NULL) {
		    oiDebugPrint(@Warn, 1, 
				 ["missing or invalid def for opp att pt ",
				  tOppAP]);
		    continue;
		}
		var tOppAPCoords = 
		    oiTransformObjCoords(tO, tOppAPDef[0], tO.getFather());
		oiDebugPrint(@Info, 2, 
			     ["coordinates of opp. att. pt.: ", tAPCoords]);
		if (oiAreEqualCoordinates(tAPCoords, tOppAPCoords)) {
		    
		    var tOppRot = _oiRotForAttPt(tA, tAPDef, tOppAPDef, NULL);
		    oiDebugPrint(@Info, 2, 
				 ["expected rotation: ", tOppRot*180/tPi]);
		    if (fabs(tOppRot-tORot) > 0.0001 && 
			!xgM.sameAngles(tOppRot, tORot))
		        oiDebugPrint(@Warn, 1, 
				     ["rotation mismatch (exp. vs. actual): ",
				      tOppRot*180/tPi, " vs. ", tORot*180/tPi]);
		    else {
			oiDebugPrint(@Info, 1, [tAPKey, " is busy"]);
			tRet.pushBack(tAPKey);
			if (tNeedObjects) {
			    var tObj = tO; 
			    tRet2.pushBack([tAPKey, tObj]);
			}
			break; 
		    }
		}
	    }
	    if (tRet.find(tAPKey) >= 0) break; 
	}
    }

    if (tNeedObjects) tRet = tRet2;

    oiDebugFinishFunc(tRet);
    return(tRet);
}


