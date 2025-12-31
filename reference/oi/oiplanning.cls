// === ofml/oi/oiplanning.cls ===
package ::ofml::oi;
import ::egr::found::*;

















public class OiPlanning: xxScene
{
    private var mOiAtEval;
    private var mOiAtDump;
    private var mOi2DBorder;
    private var mOiOdbParamMgrName;
    private var mOiPlGroupMgrName;
    private var mOiAlignment;
    private var mOiPlanDirs;
    private var mOiCreationMode;

    private var mOiNewProps;	
				

    static var sPDLanguageCache; 
				 
				 

    static var sOiRecreatedElements = NULL;

    
    public func initialize(pFa, pNa)
    {
	xxScene::initialize(pFa, pNa);

	OiChangeManager(self, @change_manager);

	mOiAlignment = [@L, @B];
	mOiPlanDirs  = [@H, @RF];

	mOiCreationMode = 0;

	getDynamicProps()[@AppPriceDateMode2] = getAppPriceDateMode();

	sPDLanguageCache = Hash();
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setCreationMode(pMode)
    {
	oiDebugStartFunc("OiPlanning::setCreationMode", pMode);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tPlan = oiGetPlanning();
	if (tPlan == NULL) {
	    
	    oiDebugPrint(@Warn, 1,
			 "internal error: no global planning instance!?");
	    oiDebugFinishFunc(NULL);
	    return(NULL);
	}

	if (tPlan != self) {
	    
	    tPlan.setCreationMode(pMode);
	    oiDebugFinishFunc(NULL);
	    return;
	}

	var tOldMode = mOiCreationMode;

	mOiCreationMode = pMode;

	if (pMode == 1)
	    sOiRecreatedElements = @(); 

	var tChM = self.getChangeManager();
	if (tChM == NULL)
	    
	    oiDebugPrint(@Warn, 1, "internal error: no ChangeManager!?");
	else
	if (tOldMode == 1 && pMode == 0) {
	    var tRecreatedEL = @();
	    
	    var tEl;
	    foreach(tEl; sOiRecreatedElements) {
		var tName;
		try { tName = tEl.getName(); }
		if (tName != NULL)
		    tRecreatedEL.pushBack(tEl);
	    }
	    oiDebugPrint(@Info, 1, 
			 ["re-created top article instances: ", tRecreatedEL]);
	    tChM.changeIssued(self, @RecreationModeFinished, tRecreatedEL);
	}

	if (pMode == 0)
	    sOiRecreatedElements = NULL;

	oiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    public func getCreationMode()
    {
	oiDebugStartFunc("OiPlanning::getCreationMode", NULL);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tRet = mOiCreationMode;

	var tPlan = oiGetPlanning();
	if (tPlan == NULL) {
	    
	    oiDebugPrint(@Warn, 1,
			 "internal error: no global planning instance!?");
	    oiDebugFinishFunc(tRet);
	    return(tRet);
	}

	if (tPlan != self) {
	    
	    tRet = tPlan.getCreationMode();
	}
	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    public func setLanguage(pLang)
    {
	oiDebugStartFunc("OiPlanning::setLanguage", pLang);

	xxScene::setLanguage(pLang);

	sPDLanguageCache = Hash();

	oiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    public func setPDLanguages(pLanguages)
    {
	oiDebugStartFunc("OiPlanning::setPDLanguages", pLanguages);

	var tDynProps = self.getDynamicProps();

	if (pLanguages == NULL || 
	    !(pLanguages instanceof List) || 
	    pLanguages.empty()) {
	    oiDebugPrint(@Warn, 1, "invalid parameter!");
	    if (tDynProps.hasKey(@AppPDLanguages))
		tDynProps.remove(@AppPDLanguages);
		
	}
	else
	    tDynProps[@AppPDLanguages] = pLanguages;

	sPDLanguageCache = Hash();

	oiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getPDLanguage(pArg)
    {
	oiDebugStartFunc("OiPlanning::getPDLanguage", pArg);

	var tDynProps = self.getDynamicProps();

	var tRet;

	if (pArg == NULL) {
	    
	    
	    var tPDLanguages = NULL;
	    if (tDynProps.hasKey(@AppPDLanguages)) 
		tPDLanguages = tDynProps[@AppPDLanguages];
	    else
		tPDLanguages = @(self.getLanguage());

	    tRet = tPDLanguages.front();

	    oiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tPID = NULL;

	if (pArg instanceof Symbol)
	    tPID = pArg;
	else
	try {
	    if (pArg.isA(OiProgInfo))
		tPID = pArg.getID();
	    else
		tPID = pArg.getProgram();
	}
	catch(&e: Error) {
	    oiDebugPrint(@Warn, 2, e.where + ": " + e.what);
	}

	if (tPID == NULL) {
	    oiDebugPrint(@Warn, 1, 
			  "invalid parameter (cannot determine program ID)!");
	    oiDebugFinishFunc(NULL);
	    return(NULL);
	}

	oiDebugPrint(@Info, 2, ["program: ", tPID]);

	if (sPDLanguageCache.hasKey(tPID)) {
	    tRet = sPDLanguageCache[tPID];
	    oiDebugPrint(@Info, 1, "using cache");
	    oiDebugFinishFunc(tRet);
	    return(tRet);
	}

	tRet = oiGetAppPDLanguage(tPID);

	if (tRet != NULL) {
	    sPDLanguageCache[tPID] = tRet;
	    oiDebugFinishFunc(tRet);
	    return(tRet);
	}

	

	oiDebugPrint(@Info, 1, "using backward compatibility implementation!");

	
	
	
	
	
	
	
	
	

	var tPDLanguages = NULL;
	if (tDynProps.hasKey(@AppPDLanguages)) 
	    tPDLanguages = tDynProps[@AppPDLanguages];
	else
	    tPDLanguages = @(self.getLanguage());

	oiDebugPrint(@Info, 2, ["application languages: ", tPDLanguages]);

	tRet = tPDLanguages.front();

	var tRegLanguages = oiGetAppSeriesLanguages(tPID);

	oiDebugPrint(@Info, 2, ["registry languages: ", tRegLanguages]);

	if (tRegLanguages.empty()) {
	    
	    oiDebugPrint(@Warn, 1, "no registry languages specified");
	    sPDLanguageCache[tPID] = tRet;
	    oiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tBestMatch = NULL;
	var tL;
	foreach(tL; tPDLanguages) {
	    if (tRegLanguages.find(tL) >= 0) {
		tBestMatch = String(tL); 
		break;
	    }
	}
	if (tBestMatch != NULL)
	    tRet = tBestMatch;
	else
	    tRet = tRegLanguages.front();

	sPDLanguageCache[tPID] = tRet;

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    public func changedPDLanguages()
    {
	oiDebugStartFunc("OiPlanning::changedPDLanguages", NULL);

	sPDLanguageCache = Hash();

	oiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    func getAppPriceDateMode()
    {
	oiDebugStartFunc2("OiPlanning::getAppPriceDateMode", NULL);

	var tValidModes = @(@CurrentOnly, @AnySave);

	var tPriceDateMode = @CurrentOnly;
	
	try {
	    var tMode = ::ofml::app::getPriceDateMode();
	    oiDebugPrint(@Info, 2, ["return value from app function: ", tMode]);
	    if (tValidModes.find(tMode) >= 0)
		tPriceDateMode = tMode;
	}
	catch (&e: Error) {
	    oiDebugPrint(@ExplWarn, 1, 
		          "Error during evaluation of function " +
		          "::ofml::app::getPriceDateMode(): " + e.what); 
	}

	oiDebugFinishFunc(tPriceDateMode);
	return(tPriceDateMode);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getPriceDateMode()
    {
	return(getDynamicProps()[@AppPriceDateMode2]);
    }

    
    
    
    
    
    public func getPriceDate4Obj(pObj)
    {
	return(oiGetCurrentDate());
    }

    public func manualSetMembers(pMembers)
    {
	var tRes = xxScene::manualSetMembers(pMembers);

	if (!pMembers.hasKey(@mOiAtEval))
	    tRes[1].pushBack(@mOiAtEval);
	if (!pMembers.hasKey(@mOiAtDump))
	    tRes[1].pushBack(@mOiAtDump);
	if (!pMembers.hasKey(@mOi2DBorder))
	    tRes[1].pushBack(@mOi2DBorder);
	if (!pMembers.hasKey(@mOiOdbParamMgrName))
	    tRes[1].pushBack(@mOiOdbParamMgrName);
	if (!pMembers.hasKey(@mOiPlGroupMgrName))
	    tRes[1].pushBack(@mOiPlGroupMgrName);
	if (!pMembers.hasKey(@mOiPlanDirs))
	    tRes[1].pushBack(@mOiPlanDirs);
	if (!pMembers.hasKey(@mOiAlignment))
	    tRes[1].pushBack(@mOiAlignment);
	if (!pMembers.hasKey(@mOiNewProps))
	    tRes[1].pushBack(@mOiNewProps);
	if (!pMembers.hasKey(@mOiCreationMode))
	    tRes[1].pushBack(@mOiCreationMode);

	return (tRes);
    }

    
    func enableDebugging(pVal)
    {
	xxFuncs.sDebuggingEnabled = pVal;
    }

    
    public func isAtDump()
    {
	return(mOiAtDump != NULL && mOiAtDump);
    }

    
    public func isAtEval()
    {
	return(mOiAtEval != NULL && mOiAtEval);
    }

    public func isCat(pCat)
    {
	if (pCat == @IF_MObject || 
	    pCat == @IF_Base || 
	    pCat == @IF_Complex || 
	    pCat == @IF_Property || 
	    pCat == @IF_Property2 || 
	    pCat == @IF_Material)
	    return(1);

	return(0);
    }

    
    public func finishApplication()
    {
	getChangeManager().changeIssued(self, @FinishApplication, NULL);
    }

    
    public func getErrorLog()
    {
	return(NULL);
    }

    
    public func setErrorLog(pLog)
    {
    }

    
    public func showBorder(pOn)
    {
	var tBorder = getBorder();

	var tBdPos = tBorder[0];
	var tBdDim = tBorder[1];

	if (tBdPos == NULL) return;

	
	hideBorderBox(); 
	if (mOi2DBorder != NULL) {
	    self.delete2DObj(mOi2DBorder); 
	    mOi2DBorder = NULL;
	}

	var tXPos =  tBdPos[0];
	var tYPos = -tBdPos[2];
	var tW    =  tBdDim[0];
	var tD    =  tBdDim[2];

	if (pOn) {
	    
	    drawBorderBox(NULL); 
	    
	    mOi2DBorder = 
		self.new2DObj(0, @LINE_LOOP, 
			      [[tXPos, tYPos], [tXPos+tW, tYPos],
			       [tXPos+tW, tYPos-tD], [tXPos, tYPos-tD]]);
	    self.set2DObjAttr(mOi2DBorder, @COLOR, [0.0, 0.0, 1.0]);
	}
    }

    
    public func getChangeManager()
    {
	var tPlan = oiGetPlanning();
	if (tPlan == NULL) return(NULL);

	if (tPlan != self) return(tPlan.getChangeManager());

	if (!oiExists(self.getName() + ".change_manager"))
	    
	    
	    
	    
	    OiChangeManager(self, @change_manager);

	return(self.change_manager);
    }

    
    public func article2ODBName(pArticle)
    {
	var tOdbName = NULL;
	var tPDM;
	if ((tPDM = getPDManager()) != NULL)
	    tOdbName = tPDM.article2ODBName(pArticle);
	return (tOdbName);
    }

    
    
    
    
    
    
    protected func createOdbParamManager()
    {
	return(NULL);
    }

    
    public func getOdbParamManager()
    {
	oiDebugStartFunc2("OiPlanning::getOdbParamManager", NULL);

	var tPlan = oiGetPlanning();
	if (tPlan == NULL) {
	    oiDebugFinishFunc("no planning instance!?");
	    return(NULL);
	}

	if (tPlan != self) {
	    oiDebugFinishFunc("delegating to global planning instance");
	    return(tPlan.getOdbParamManager());
	}

	if (mOiOdbParamMgrName == NULL) 
	    mOiOdbParamMgrName = createOdbParamManager();

	if (mOiOdbParamMgrName == NULL) {
	    oiDebugFinishFunc("no manager created!?");
	    return(NULL);
	}

	var tMgrName = self.getName() + "." + mOiOdbParamMgrName;
	oiDebugPrint(@Info, 2, ["name of manager: ", mOiOdbParamMgrName]);

	var tMgrObj  = NULL;
	try {
	if (oiExists(tMgrName))
	    tMgrObj = eval(tMgrName);
	}
	catch(&e: Error) {
	    oiDebugPrint(@Warn, 1, e.where + ": " + e.what);
	}

	oiDebugFinishFunc(tMgrObj);
	return(tMgrObj);
    }

    
    
    
    
    
    
    protected func createPlGroupManager()
    {
	var tName = "oi_plgrp_mgr";

	OiPlGroupManager(self, Symbol(tName));

	return(tName);
    }

    
    public func getPlGroupManager()
    {
	var tPlan = oiGetPlanning();
	if (tPlan == NULL) return(NULL);

	if (tPlan != self) return(tPlan.getPlGroupManager());

	if (mOiPlGroupMgrName == NULL) 
	    mOiPlGroupMgrName = createPlGroupManager();

	if (mOiPlGroupMgrName == NULL) return(NULL);

	var tMgrName = self.getName() + "." + mOiPlGroupMgrName;

	var tMgrObj  = NULL;
	if (oiExists(tMgrName))
	    tMgrObj = eval(tMgrName);

	return(tMgrObj);
    }

    
    public func getPlElementUp(pObj)
    {       
	return(getScElementUp(pObj));
    }

    
    public func getTopPlElement(pObj)
    {
	var tPlEl = getPlElementUp(pObj);

	if (tPlEl == NULL) return(NULL);

	var tObj = tPlEl;
	var tF;
	while ((tF = tObj.getFather()) != NULL) {
	    tObj = tF;
	    if (tObj.isA(OiPlElement)) tPlEl = tObj;
	}
	return(tPlEl);
    }

    
    public func checkConsistency()
    {
	oiDebugStartFunc("OiPlanning::checkConsistency", NULL);

	var tRes = 1;

	
	var tPID;
	foreach(tPID; getInfoIDs()) {
	    var tPI = getInfo(tPID);
	    if (tPI == NULL || !tPI.isA(OiProgInfo)) continue;
	    oiDebugPrint(@Info, 2, 
			 ["delegating to proginfo for ", tPID, " ..."]);
	    if (!tPI.checkConsistency()) tRes = 0;
	}

	
	var tCh;
	foreach(tCh; getChildren())
	    if (tCh.isA(OiPlElement) || tCh.isA(OiPart)) {
		var tOK = tCh.checkConsistency();
		oiDebugPrint(@Info, 2, ["check ", tCh, ": ", tOK]);
		if (!tOK) tRes = 0;
	    }

	oiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    public func checkObjConsistency(pObj)
    {
	return(pObj.checkConsistency());
    }

    
    public func assignDefaultPropValues(pObj)
    {
    }

    
    public func articleInserted(pNewObj, pRefObj, pInsertMode)
    {
    }

    
    public func confirmArticleMigration(pObj)
    {
	oiDebugStartFunc("xOiPlanning::confirmArticleMigration", pObj);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	oiDebugFinishFunc(0);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func issueUpdateMessage(pObj, pState, pMsg)
    {
    }

    
    public func hasSavedPD(pObj, ...)
    {
	oiDebugStartFunc("xOiPlanning::hasSavedPD", pObj);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	oiDebugFinishFunc(0);
	return(0);
    }

    
    public func getSavedArticlePrice(pObj, ...)
    {
	oiDebugStartFunc("xOiPlanning::getSavedArticlePrice", pObj);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	oiDebugFinishFunc(NULL);
    }

    
    public func getSavedArticleText(pObj, pForm, ...)
    {
	oiDebugStartFunc("xOiPlanning::getSavedArticleText", [pObj, pForm]);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	oiDebugFinishFunc(NULL);
    }

    
    public func getSavedArticleFeatures(pObj, pLanguage)
    {
	oiDebugStartFunc("xOiPlanning::getSavedArticleFeatures", 
			  [pObj, pLanguage]);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	oiDebugFinishFunc(NULL);
    }

    
    public func getSavedArticleFeatures2(pObj, ...)
    {
	oiDebugStartFunc("xOiPlanning::getSavedArticleFeatures2", pObj);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	oiDebugFinishFunc(NULL);
    }

    
    public func getSavedAllArticleFeatures(pObj, ...)
    {
	oiDebugStartFunc("xOiPlanning::getSavedAllArticleFeatures", pObj);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	oiDebugFinishFunc(NULL);
    }

    
    public func getSavedPDInfo(pObj, ...)
    {
	oiDebugStartFunc("xOiPlanning::getSavedPDInfo", pObj);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	oiDebugFinishFunc(NULL);
    }

    
    public func getRecoveryInfo()
    {
	var tRes = Hash();
	return(tRes);
    }

    
    public func isValidForCollCheck(pCh)
    {    
	if (pCh.isA(OiPDManager)) return(0);

	return(1);
    }

    
    public func elemTranslation(pObj, pOld)
    {
	childTranslation(pObj, pOld);
    }

    
    public func elemRotation(pObj, pOld)
    {
	childRotation(pObj, pOld);
    }

    
    public func newElement(pFather, pObj)
    {
    }

    
    public func removeElement(pFather, pObj)
    {
	getChangeManager().unRegister(pObj, NULL);
    }

    
    
    
    
    

    public func checkPropValue(pPKey, pPValue, ...)
    {
	oiDebugStartFunc("OiPlanning::checkPropValue", [pPKey, pPValue]);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tValue = pPValue[0];
	var tCheckCurr = 0;
	if (pPValue.size() > 1 && pPValue[1] instanceof Int)
	    tCheckCurr = pPValue[1];

	var tRet = oiCheckPropValue(self, pPKey, tValue, 1);

	if (tRet && tCheckCurr) {
	    tRet = !(xgF.compareObjs(self.getPropValue(pPKey), tValue));
	    if (!tRet)
		oiDebugPrint(@Info, 1, 
			     "current value and passed value are equal");
	}

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    public func setPropValue(pPKey, pValue)
    {
	if (self.isAtEval()) return(0);

	if (!oiCheckPropValue(self, pPKey, pValue)) return(0);

	var tRet = xxScene::setPropValue(pPKey, pValue);

	return(tRet);
    }

    
    
    

    

    
    

    
    public func setupProperty(pPKey, pDef, pPos)
    {
	oiSetupProperty(self, pPKey, pDef, pPos);
    }

    
    public func setPropState(pPKey, pState)
    {
	oiSetPropState(self, pPKey, oiOld2NewPropState(pState));
    }

    
    public func getPropState(pPKey)
    {
	return(oiGetPropState(self, pPKey, 0));
    }

    
    public func getPropertyKeys()
    {
        return(oiGetPropKeys(self));
    }

    
    public func setPropPosOnly(pPKey, pPos)
    {
	return(oiSetPropPos(self, pPKey, pPos));
    }

    
    public func getPropertyPos(pPKey)
    {   
	return(oiGetPropPos(self, pPKey));
    }

    
    public func getPropertyDef(pPKey)
    {
	return(oiGetPropertyDef(self, pPKey));
    }   

    public func getPropDefinition(pPKey)
    {
	return(oiGetPropDefinition(self, pPKey));
    }   

    
    public func getProperties()
    {
	return(oiGetProperties(self));
    }

    
    public func getVisiblePropValues()
    {
	return(oiGetVisiblePropValues(self));
    }

    
    
    

    
    public func getPropClassDescriptions(pLanguage)
    {
        return(OiPlanning::getPropGroupDescriptions(pLanguage));
    }

    
    public func getPropGroupDescriptions(pLanguage)
    {
        return(oiGetPropGroupDescriptions(self, pLanguage));
    }

    
    
    

    
    public func setupProperty2(pPKey, pDef, pName, pPos, pState)
    {
	oiSetupProperty2(self, pPKey, pDef, pName, pPos, pState);
    }

    
    public func getPropDef2(pPKey)
    {
        return(oiGetPropDef2(self, pPKey));
    }

    
    
    
    public func getPropKeys()
    {
        return(oiGetPropKeys(self));
    }

    
    public func setPropName(pPKey, pName)
    {
	oiSetPropName(self, pPKey, pName);
    }

    
    public func getPropName(pPKey)
    {
	return(oiGetPropName(self, pPKey));
    }

    
    public func setPropPos(pPKey, pPos)
    {
	return(oiSetPropPos(self, pPKey, pPos));
    }

    
    public func getPropPos(pPKey)
    {   
	return(oiGetPropPos(self, pPKey));
    }

    
    public func setPropState2(pPKey, pState)
    {
	oiSetPropState(self, pPKey, pState);
    }

    
    public func getPropState2(pPKey)
    {
	return(oiGetPropState(self, pPKey, 1));
    }

    
    public func setPropHintText(pPKey, pHint)
    {
	oiSetPropHintText(self, pPKey, pHint);
    }

    
    public func getPropHintText(pPKey)
    {
	return(oiGetPropHintText(self, pPKey));
    }

    
    public func setPropRanges(pPKey, pRanges)
    {
	oiSetPropRanges(self, pPKey, pRanges);
    }

    
    public func getPropRanges(pPKey)
    {
	return(oiGetPropRanges(self, pPKey));
    }

    
    public func setPropChoiceList(pPKey, pChoiceList)
    {
	oiSetPropChoiceList(self, pPKey, pChoiceList);
    }

    
    public func getPropChoiceList(pPKey, ...)
    {
	var tPKey = pPKey[0];
	var tCallMethod = 1;
	if (pPKey.size() > 1 && pPKey[1] instanceof Int && pPKey[1] >= 0)
	    tCallMethod = pPKey[1];
	return(oiGetPropChoiceList(self, tPKey, tCallMethod));
    }

    
    public func getPropChoiceList2(pPKey)
    {
	return(oiGetPropChoiceList2(self, pPKey));
    }

    
    public func getPropValue2(pPKey)
    {
	return(oiGetPropValue2(self, pPKey));
    }

    
    public func getPropSpec(pPKey)
    {
	return(oiGetPropSpec(self, pPKey));
    }

    
    public func getPropSpecs()
    {
	return(oiGetPropSpecs(self));
    }

    public func getAllPropSpecs(pRequiredState)
    {
	return(@());
    }

    public func getAllPropValueInfos(pObj, pPKey, pPClass)
    {
	return(NULL);
    }

    public func getDefaultPropGroupSpecs()
    {
	return(@());
    }

    
    
    

    
    
    
    
    
    public func setPlanDirS(pDir)
    {
        if (pDir == @H || pDir == @V)
            mOiPlanDirs = [pDir, mOiPlanDirs[1]];
    }

    
    public func getPlanDirS()
    {
        return(mOiPlanDirs[0]);
    }

    
    
    
    
    
    
    
    
    public func setPlanDirXZ(pDir)
    {
	oiDebugStartFunc("OiPlanning::setPlanDirXZ", pDir);

        var tVL = @(@RF, @RB, @LF, @LB, @FR, @FL, @BR, @BL);

	if (tVL.find(pDir) >= 0)
            mOiPlanDirs = [mOiPlanDirs[0], pDir];

	oiDebugFinishFunc(NULL);
    }

    
    public func getPlanDirXZ()
    {
        return(mOiPlanDirs[1]);
    }

    
    
    public func getPlanDirX()
    {
	if (String(mOiPlanDirs[1]).find('R') >= 0)
	    return(@R);
	else
	    return(@L);
    }

    
    
    public func getPlanDirZ()
    {
	if (String(mOiPlanDirs[1]).find('F') >= 0)
	    return(@F);
	else
	    return(@B);
    }

    
    
    

    
    
    
    public func setAlignmentX(pMode)
    {
        if (pMode == @L || pMode == @C || pMode == @R)
            mOiAlignment = [pMode, mOiAlignment[1]];
    }

    
    public func getAlignmentX()
    {
        return(mOiAlignment[0]);
    }

    
    
    
    public func setAlignmentZ(pMode)
    {
        if (pMode == @F || pMode == @C || pMode == @B)
            mOiAlignment = [mOiAlignment[0], pMode];
    }

    
    public func getAlignmentZ()
    {
        return(mOiAlignment[1]);
    }

    
    
    

    rule NEW_ELEMENT (pValue)
    {
	var tEl = $pValue;

	oiDebugStartFunc("OiPlanning::NEW_ELEMENT", tEl);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (!objIsTmpCh(tEl)) {
	    if (sOiRecreatedElements != NULL) 
		sOiRecreatedElements.pushBack(tEl);
	    newElement(self, tEl);
	}

	oiDebugFinishFunc(0);
	return(0);
    }

    rule REMOVE_ELEMENT (pValue)
    {
	var tEl = $pValue;

	oiDebugStartFunc("OiPlanning::REMOVE_ELEMENT", tEl);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (sOiRecreatedElements != NULL && sOiRecreatedElements.find(tEl) >= 0)
	    sOiRecreatedElements.remove(tEl);

	removeElement(self, tEl);

	oiDebugFinishFunc(0);
	return(0);
    }

    rule START_DUMP(pDummy) 
    {
	mOiAtDump = 1;

	return (0);
    }

    rule FINISH_DUMP(pDummy) 
    {
	mOiAtDump = 0;

	return (0);
    }

    
    
    rule START_EVAL(pDummy) 
    {
	mOiAtEval = 1;

	if (oiHasOldPropRepr(self)) oiOld2NewPropRepr(self);

	oiExtendNewPropRepr(self);

	

	if (mOiAlignment == NULL)
	    mOiAlignment = [@L, @B];
	if (mOiPlanDirs == NULL)
	mOiPlanDirs  = [@H, @RF];

	var tDynProps = getDynamicProps();
	if (tDynProps.hasKey(@AppPriceDateMode2)) {
	    var tSavedPriceDateMode = tDynProps[@AppPriceDateMode2];
	    var tCurrAppPriceDateMode = getAppPriceDateMode();
	    if (tSavedPriceDateMode != tCurrAppPriceDateMode) {
		
		
		oiOutput(@MESSAGE, 
			 "planning was saved by an application with an " +
			 "incompatible mode with respect to price date " +
			 "handling\n(saved " + String(tSavedPriceDateMode) +
			 " vs. current " + String(tCurrAppPriceDateMode) + ")");
		return(-1); 
	    }
	}
	else
	    tDynProps[@AppPriceDateMode2] = getAppPriceDateMode();

	
	var tLanguage = oiGetAppLanguage();
	if (tLanguage != NULL) setLanguage(tLanguage);

	if (mOiCreationMode == NULL)
	    mOiCreationMode = 0;

	return (0);
    }

    rule FINISH_EVAL(pDummy) 
    {
	mOiAtEval = 0;

	return (0);
    }
}

OiGlobal();
OiFuncs();
OiProperties();

