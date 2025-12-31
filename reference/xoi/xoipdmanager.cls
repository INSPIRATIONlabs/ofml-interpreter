// === ofml/xoi/xoipdmanager.cls ===
package ::ofml::xoi;
import ::ofml::oi::*;

import ::stdio::*;
























public class xOiPDManager: OiPDManager
{
    
    var mTransObj;
    var mTransArticle;
    var mTransPDB;
    var mTransPClasses;
    var mPropKey2PropName;
    var mPropName2PropKey;
    var mTransTables;
    var mTransactionFinished;

    static var sPropValList;
    static var sTransFinalType;
    static var sTransVarCode;
    static var sTransOFMLVarCode;

    static var sTransDepth 	= 0;

    static var sTmpChildFather;
    static var sTmpChildArticle;
    static var sTmpChildPVals;
    static var sTmpChildProps;
    static var sTmpChildVarCode;
    static var sTmpChildFinalSpec;
    static var sTmpChildOFMLVC;
    static var sTmpChildAddSpec;
    static var sTmpChildChfPValTexts;
    static var sTmpChildVisPropVals;

    
    static var sAggrObj;
    static var sTmpAggrObjName;
    static var sTmpAggrChildProps; 
				   
    static var sTmpAggrChildPropsEnabled;

    static var sUsePriceProfiles = 1;

    static var sFinalType4PDCachePropKey = @_XOI_PDM_FAN4PDCACHE;

    
    
    static var sConsistencyCheckCache;

    
    
    static var sUpdatabilityCheckPropDefs;
    static var sUpdatabilityCheckVisPropVals;

    private var mChangedProp;	

    
    
    
    
    
    
    
    
    
    
    static var sPDCache = Hash();

    
    
    
    
    
    static var sArticleTextCache = Hash();

    
    
    
    
    static var sPDInfoCache = Hash();

    var mUsePDCache;	 

    var mPDCache; 

    var mUpdatabilityCheckContext; 
    var mClassesCache; 

    
    
    static var sOutputVarcodeErr = 1;

    
    static var sReplacementsTbl = "replacements";
    static var sReplacementsTE = @(["article_nr", 0, 0, @s, ""+"", "0"],
				  ["manufacturer", 0, 0, @s, ""+"", "0"],
				  ["series", 0, 0, @s, ""+"", "0"],
				  ["replacement", 0, 0, @s, ""+"", "0"]);                                
    
    static var sSpecialFinalArtMapName = "final2base";
    static var sSpecialFinalArtMapTE = @(["final_nr", 0, 0, @s, ""+"", "0"],
				         ["base_nr", 0, 0, @s, ""+"", "0"]);
    static var sArticleSearchCache; 
    static var sSpecialFinalArtMapCache; 

    var mXoiInsignificantPClasses;	

    static var sSafePropNames = NULL;	

    static var sXoiIdxPricesPaths;

    
    public func initialize(pFa, pNa)
    {
	xOiDebugStartFunc("xOiPDManager::initialize", [pFa, pNa]);

	OiPDManager::initialize(pFa, pNa);

	mUsePDCache = 1;

	mClassesCache = Hash();

	
	
	if (oiExists(getName()+".proddbs")) {
	    clearProductDBs(); 
	    remove(self.proddbs);
	    xOiInfoObjSet(self, @proddbs);
	}

	cleanUpTransaction(0);

	xOiDebugFinishFunc(NULL);
    }

    func getUpdatabilityCheckContext()
    {
	return(mUpdatabilityCheckContext);
    }

    func outputVarcodeErr()
    {
	return(sOutputVarcodeErr);
    }

    
    func enablePDCache()
    {
	mUsePDCache = 1;
    }

    
    func disablePDCache()
    {
	mUsePDCache = 0;
    }

    func isPDCacheEnabled()
    {
	return(mUsePDCache);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func updatePDCache(pObj)
    {
	xOiDebugStartFunc("xOiPDManager::updatePDCache", pObj);

	var tRes = NULL;

	if (!isPDCacheEnabled()) {
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tPlan = getPlanning();
	
	var tLanguage = tPlan.getPDLanguage(pObj);
	if (tLanguage == NULL) tLanguage = "de";

	var tObj = pObj;
	var tArticle;
	if (!tObj.isCat(@IF_Article) ||
	    (tArticle = tObj.getArticleSpec()) == NULL) {
	    xOiDebugPrint(@Warn, 1, "no article!");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	startTransaction(tObj, NULL); 

	if (mTransPDB != NULL) {
	    var tVarCode  = tObj.getXArticleSpec(@VarCode);
	    var tFinalArt = tObj.getXArticleSpec(@Final);
	    var tOFMLVC   = tObj.getXArticleSpec(@OFMLVarCode);
	    getArticleText(tObj, tLanguage, @s);
	    getArticleText(tObj, tLanguage, @l);
	    getArticlePrice(tObj, tLanguage, tPlan.getPrimaryCurrency());
	    getArticleFeatures(tObj, tLanguage);
	    getArticleFeatures(tObj, NULL);
	    getArticleFeatures2(tObj, tLanguage);
	    getAllArticleFeatures(tObj, tLanguage);
	    getPDInfo(tObj, tLanguage);
	    var tFinalType = 
		finalType4PDCache(mTransPDB, tObj,
				  buildPropValList(tObj, mTransPDB, tArticle));
	    if (sPDCache.hasKey(tFinalType)) {
		tRes = [tVarCode, tFinalArt, tOFMLVC];
		xOiDebugPrint(@Info, 2,
			      ["new/changed entry for '", tFinalType, "': ",
			       sPDCache[tFinalType]]);
	    }
	    else
		xOiDebugPrint(@Warn, 1, ["no entry created in PD cache for ", 
					 tFinalType]);
	}

	finishTransaction(tObj);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    func cleanUpPDCache()
    {
	xOiDebugStartFunc2("xOiPDManager::cleanUpPDCache", NULL);

	sPDCache = Hash();
	sArticleTextCache = Hash();
	sPDInfoCache = Hash();

	xOiDebugFinishFunc(NULL);
    }

    
    private func getTblID(pType, pDbPath, pName, pSuffix, pPID)
    {
	var tName = pName;
	if (pSuffix != NULL)
	    tName += "_" + pSuffix;

	if (pType == @EBASE)
	    return("EBASE " + pDbPath + "/pdata.ebase " + tName);
	else
	if (pType == @FTXT)
	    return("FTXT " + pDbPath + "/" + pName+"."+pSuffix + " " + pName);
	else
	if (pType == @VAR_FTXT)
	    return("VAR EBASE,FTXT " + pDbPath + " pdata " + tName);
	else 
	    return("VAR EBASE,CSV "+pDbPath+" pdata "+pName);
    }

    private func _String2PKey(pStr, pObj)
    {
	var tPDB;
	if (pObj == NULL) {
	    if (mUpdatabilityCheckContext != NULL)
		tPDB = mUpdatabilityCheckContext[3];
	}
	else
	if (hasTransaction(pObj))
	    tPDB = mTransPDB;
	else
	    tPDB = getPDBFor(pObj);

	var tPrefix = NULL;

	
	
	if (tPDB != NULL && tPDB.hasMember(@propKeyPrefix))
	    tPrefix = tPDB.propKeyPrefix();

	if (tPrefix == NULL && xOiIsOfmlKeyword(pStr))
	    tPrefix = "_";

	return(_xOiString2Symbol(pStr, tPrefix));
    }

    private func _PKey2String(pKey, pObj, pPDB)
    {
	var tPDB = pPDB;
	if (tPDB == NULL) {
	    if (hasTransaction(pObj))
		tPDB = mTransPDB;
	    else
		tPDB = getPDBFor(pObj);
	}

	var tKeyStr = String(pKey);
	if (tKeyStr[0] == '_' && xOiIsOfmlKeyword(tKeyStr.substr(1))) {
	    return(tKeyStr.substr(1));
	}

	
	var tPrefix = NULL;
	if (tPDB != NULL && tPDB.hasMember(@propKeyPrefix))
	    tPrefix = tPDB.propKeyPrefix();

	return(_xOiSymbol2String(pKey, tPrefix));
    }

    
    
    

    private func cleanUpTransaction(pOnlyArticleStuff)
    {
	xOiDebugStartFunc("xOiPDManager::cleanUpTransaction", 
			  pOnlyArticleStuff);

	if (!pOnlyArticleStuff) {
	    if (mTransArticle != NULL && mTransPDB != NULL)
	        mTransPDB.finishTransaction(mTransArticle);
	    if (mTransTables != NULL) {
	        var tT;
	        foreach(tT; mTransTables.values())
	            if (tT != NULL) {
		        xOiDebugPrint(@Info, 2, ["closing table ", tT.getID(),
					         ", open: ", tT.mIsOpen]);
		        tT.close();
	            }
	    }
	    mTransObj = NULL;
	    mTransPDB = NULL;
	    mTransTables = NULL;
	    sTransDepth = 0;
	}

	mTransArticle = NULL;
	mTransPClasses = NULL;
	mPropKey2PropName = NULL;
	mPropName2PropKey = NULL;
	sPropValList = NULL;
        sTransFinalType = NULL;
        sTransVarCode = NULL;
        sTransOFMLVarCode = NULL;

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    func startTransaction(pObj, pArticle)
    {
	xOiDebugStartFunc("xOiPDManager::startTransaction", [pObj, pArticle]);

	if (pObj == NULL) {
	    xOiDebugPrint(@Warn, 1, "no transaction object given!");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	var tName = NULL;
	try { tName = mTransObj.getName(); }
	if (tName == NULL) mTransObj = NULL;

	xOiDebugPrint(@Info, 2, ["current transaction object: ", mTransObj]);

	if (mTransObj != NULL && mTransObj != pObj && !mTransactionFinished) {
	    xOiDebugPrint(@Warn, 1, "no transaction for two objects!");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	
	if (mTransPDB != NULL) {
	    var tPDBName = NULL;
	    try {
		tPDBName = mTransPDB.getName();
	    }
	    if (tPDBName == NULL)
		mTransPDB = NULL;
	}

	if (hasTransaction(pObj, 1)) {
	    sTransDepth++;
	    if (mTransactionFinished) {
	        mTransactionFinished = 0;
	        xOiDebugPrint(@Info, 1, "new transaction for " +
					"previous transaction object");
		if (mTransPDB != NULL && mTransArticle != NULL) {
		    
		    
		    
		    mTransPDB.startTransaction(mTransArticle);
		}
	    }
	    else
	        xOiDebugPrint(@Info, 1, "nested transaction");
	    
	    var tNewArt = pArticle;
	    if (tNewArt == NULL)
		tNewArt = pObj.getArticleSpec();
	    if (!xOiCompareObjs(tNewArt, mTransArticle)) {
		
		
	        if (pObj.getDynamicProps().hasKey(sFinalType4PDCachePropKey))
		    pObj.getDynamicProps().remove(sFinalType4PDCachePropKey);
		if (mTransPDB != NULL) {
		    if (mTransArticle != NULL)
			mTransPDB.finishTransaction(mTransArticle);
		    if (tNewArt != NULL) {
			mTransPDB.startTransaction(tNewArt);
			cleanUpTransaction(1);
			mTransPClasses = 
			mTransPDB.getArticlePropClasses(tNewArt);
		    }
		}
		mTransArticle = tNewArt;
	    }
	}
	else {
	    xOiDebugPrint(@Info, 1, "new transaction object");

	    var tArticle = pArticle;
	    if (tArticle == NULL)
	    	tArticle = pObj.getArticleSpec();

	    
	    

	    if (sTmpChildFather != NULL) {
		
		var tName = NULL;
		try { tName = sTmpChildFather.getName(); }
		if (tName == NULL) sTmpChildFather = NULL;
	    }
	    xOiDebugPrint(@Info, 2, ["sTmpChildFather ", sTmpChildFather]);

	    var tFather = pObj.getFather();
	    xOiDebugPrint(@Info, 3, ["father ", tFather]);
	    if (tFather.hasMember(@isCat) &&
		(tFather.isCat(@MetaType) || tFather.isCat(@IF_MetaType)) &&
		tFather.hasMember(@getMainChild) &&
		tFather.getMainChild() == pObj)
		tFather = tFather.getFather();

	    if (mTransObj != NULL && mTransObj == pObj && 
		mTransArticle != tArticle &&
	        mTransObj.getDynamicProps().hasKey(sFinalType4PDCachePropKey))
		mTransObj.getDynamicProps().remove(sFinalType4PDCachePropKey);

	    if (pObj.getDynamicProps().hasKey(@__XOIPDM_STATE) ||
		sTmpChildFather == NULL || sTmpChildFather != tFather ||
		mTransArticle != tArticle)
	        cleanUpTransaction(0);

	    if (tArticle != NULL) {
		sTransDepth++;
		mTransObj = pObj;
		mTransPDB = getPDBFor(pObj);
		mTransTables = Hash();
		mTransArticle = tArticle;
	    }
	    else {
		xOiDebugPrint(@Warn, 1, "ignoring non-article object");
		cleanUpTransaction(0);
	    }

	    mTransactionFinished = 0;

	    if (mTransArticle != NULL && mTransPDB != NULL) {
		mTransPDB.startTransaction(mTransArticle);
		mTransPClasses = mTransPDB.getArticlePropClasses(mTransArticle);
	    }
	}
	xOiDebugPrint(@Info, 2, ["new transaction depth: ", sTransDepth]);
	xOiDebugFinishFunc(1);
	return(1);
    }

    
    
    
    public func finishTransaction(pObj)
    {
	xOiDebugStartFunc("xOiPDManager::finishTransaction", pObj);

	if (mTransObj == NULL || mTransObj != pObj) {
	    xOiDebugPrint(@Warn, 1, "no transaction started for object");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	sTransDepth--;
	xOiDebugPrint(@Info, 2, ["new transaction depth: ", sTransDepth]);

	if (!sTransDepth) {
	    
	    
	    mTransactionFinished = 1;
	}
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    public func hasTransaction(pObj, ...)
    {
	xOiDebugStartFunc2("xOiPDManager::hasTransaction", pObj);

	var tObj = pObj[0];
	var tCheckCacheOnly = 0;
	if (pObj.size() > 1)
	    tCheckCacheOnly = pObj[1];

	if (mTransObj != NULL) {
	    
	    
	    
	    var tName = NULL;
	    try { tName = mTransObj.getName(); }
	    if (tName == NULL) cleanUpTransaction(0);
	}

	xOiDebugPrint(@Info, 2, ["transaction ", mTransObj]);
	xOiDebugPrint(@Info, 2, ["article: ", mTransArticle]);
	xOiDebugPrint(@Info, 2, ["PDB: ", mTransPDB]);
	xOiDebugPrint(@Info, 2, ["mTransactionFinished: ", mTransactionFinished]);

	var tRet;

	if (tObj != NULL)
	    tRet = (mTransObj != NULL && mTransObj == tObj &&
		    mTransArticle != NULL && mTransPDB != NULL &&
		    (tCheckCacheOnly || !mTransactionFinished));
	else
	    tRet = (mTransObj != NULL && mTransArticle != NULL &&
		    mTransPDB != NULL && !mTransactionFinished);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    public func getTransObj()
    {
	if (mTransObj != NULL) {
	    
	    
	    
	    
	    var tName = NULL;
	    try { tName = mTransObj.getName(); }
	    if (tName == NULL) cleanUpTransaction(0);
	}
	return(mTransObj);
    }

    func getTransObjUnchecked()
    {
	return(mTransObj);
    }

    
    
    
    
    
    public func getTransArticle()
    {
	return(mTransArticle);
    }

    
    
    public func startAggrCreation(pAggrObj)
    {
	xOiDebugStartFunc("xOiPDManager::startAggrCreation", pAggrObj);

	xOiDebugPrint(@Info, 2, ["current sAggrObj ", sAggrObj]);

	if ((pAggrObj != NULL && sAggrObj != NULL) ||
	    !pAggrObj.isA(xOiAggregate))  {
	    xOiDebugFinishFunc("rejected");
	    return;
	}

	sAggrObj = pAggrObj;

	if (sTmpAggrObjName == NULL)
	    sTmpAggrChildProps = Hash();

	var tFather = pAggrObj.getFather();
	if (tFather.isCat(@IF_Complex) && tFather.objIsTmpCh(pAggrObj))
	    sTmpAggrObjName = sAggrObj.getName();

	xOiDebugFinishFunc("ok");
    }

    
    
    public func finishAggrCreation(pAggrObj)
    {
	xOiDebugStartFunc("xOiPDManager::finishAggrCreation", pAggrObj);

	xOiDebugPrint(@Info, 2, ["current sAggrObj ", sAggrObj]);

	if (sAggrObj != pAggrObj)  {
	    xOiDebugFinishFunc("rejected");
	    return;
	}

	sAggrObj = NULL;
	if (sTmpAggrObjName != pAggrObj.getName()) 
	    
	    sTmpAggrObjName = NULL;

	xOiDebugPrint(@Info, 2, ["sTmpAggrObjName: ", sTmpAggrObjName]);

	xOiDebugFinishFunc("ok");
    }

    public func getPDDataDir(pObj)
    {
	var tPDB;
	if (pObj.isA(OiProductDB))
	    tPDB = pObj;
	else {
	    if (hasTransaction(pObj))
		tPDB = mTransPDB;
	    else
		tPDB = getPDBFor(pObj);
	}
	if (tPDB == NULL) return(NULL);

	return(tPDB.getDataRootDir()+"/");
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func openMapTable(pTblType, pTblName, pTblStruct, pObj, ...)
    {
	xOiDebugStartFunc2("xOiPDManager::openMapTable",
			  [pTblType, pTblName, pObj]);

	var tObj = pObj[0];
	var tSuffix = "map";
	if (pObj.size() > 1)
	    tSuffix = pObj[1];

	var tPID = NULL;
	var tPDB;
	if (tObj.isA(OiProductDB)) {
	    if (mUpdatabilityCheckContext != NULL)
		tPID = mUpdatabilityCheckContext[0];
	    else
	    if (hasTransaction(NULL)) {
		tPID = mTransObj.getProgram();
		
		
		
		
		if ((tPDB = getProgPDB(tPID)) == NULL || tPDB != tObj) {
		    xOiDebugPrint(@Info , 1, 
				  "not using program ID of transaction object");
		    tPID = getPlanning().getProgram();
		}
	    }
	    else
		tPID = getPlanning().getProgram();
	    tPDB = tObj;
	}
	else {
	    tPID = tObj.getProgram();
	    if (hasTransaction(tObj))
		tPDB = mTransPDB;
	    else
		tPDB = getPDBFor(tObj);
	}
	if (tPDB == NULL) {
	    xOiDebugPrint(@Warn , 1, "no product database");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}
	var tDir = tPDB.getDataRootDir();
	if (tDir == NULL) {
	    xOiDebugPrint(@Warn , 1, "no data root dir");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	xOiDebugPrint(@Info, 2, ["PID: ", tPID]);
	xOiDebugPrint(@Info, 2, ["Dir: ", tDir]);

	var tTbl = NULL;

	if (pTblType == @OAM) {
	    tTbl = oiOpenOAMTbl(pTblName, tPID);
	    xOiDebugFinishFunc(tTbl == NULL ? NULL : tTbl.getID());
	    return(tTbl);
	}

	var tTblID = getTblID(pTblType, tDir, pTblName, tSuffix, tPID);

	xOiDebugPrint(@Info, 2, ["table ID: ", tTblID]);

	
	if (mTransTables != NULL && mTransTables.hasKey(tTblID)) {
	    tTbl = mTransTables[tTblID];
	    if (tTbl != NULL) {
		
		
		tTbl.close();
		tTbl.open();
	    }
	}
	else {
	    
	    tTbl = xOiTable2(tTblID, pTblStruct);
	    if (tTbl.open())
		tTblID = tTbl.getID();
	    else
	        tTbl = NULL;
	    if (mTransTables != NULL)
		mTransTables[tTblID] = tTbl;
	}

	xOiDebugFinishFunc(tTbl != NULL ? tTblID : NULL);
	return(tTbl);
    }

    
    
    protected func closeMapTable(pTbl, pObj)
    {
	var tTblID = pTbl.getID();

	xOiDebugStartFunc2("xOiPDManager::closeMapTable", [tTblID, pObj]);

	if (mTransTables != NULL && mTransTables.hasKey(tTblID))
	    xOiDebugPrint(@Info, 2, "ongoing transaction -> delay closure!");
	else
	    pTbl.close();

	xOiDebugFinishFunc(NULL);
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func searchArticles(pArtKey, pPrograms, pLanguage)
    {
	xOiDebugStartFunc("xOiPDManager::searchArticles",
			  [pArtKey, pPrograms, pLanguage]);

	var tRes = @();
	var tLanguage = pLanguage;
	if (!(pLanguage instanceof String))
	    tLanguage = "NULL";

	var tPlan = getPlanning();
	var tPlanPID = tPlan.getProgram();

	var tProgress = 0;
	if (pPrograms.size() >= 10) {
	    var tTitle = "searching product databases ...";
	    tTitle = NULL;
	    
	    
	    
	    tProgress = xOiInitProgressDialog(tTitle, pPrograms.size(), 0);
	}
	var tP;
	foreach(tP; pPrograms) {
	    var tPID = tP;
	    if (tP instanceof Vector || tP instanceof List) 
		
		
		tPID = tP[0];

	    
	    oiRegisterPackage(tPID);

	    var tPDB = getProgPDB(tPID);

	    if (tPDB == NULL)
		xOiDebugPrint(@Info, 1, 
			      ["no product database registered for ", tP]);
	    else {
		var tR = searchArticlesInPDB(tPID, tPDB, pArtKey, tLanguage);
		xOiCopyAggr(tR, tRes, 1);
	    }
	    if (tProgress) xOiProcessProgressStep();
	}

	if (tProgress) xOiFinishProgressDialog();

	tPlan.setProgram(tPlanPID);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    private func searchArticlesInPDB(pPID, pPDB, pArtKey, pLanguage)
    {
	xOiDebugStartFunc("xOiPDManager::searchArticlesInPDB",
			  [pPID, pPDB, pArtKey, pLanguage]);

	if (sArticleSearchCache == NULL)
	    sArticleSearchCache = Hash();
	if (sSpecialFinalArtMapCache == NULL)
	    sSpecialFinalArtMapCache = Hash();

	var tRes = @();

	var tCacheKey = String(pPID)+pArtKey+pLanguage;
	xOiDebugPrint(@Info, 2, ["cache key: ", tCacheKey]);

	if (sArticleSearchCache.hasKey(tCacheKey)) {
	    xOiDebugPrint(@Info, 1, "using cache");
	    tRes = sArticleSearchCache[tCacheKey];
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	

	if (pArtKey != NULL && pArtKey != "*" &&
	    !(sSpecialFinalArtMapCache.hasKey(pPID) && 
	      sSpecialFinalArtMapCache[pPID] == NULL)) {

	    if (pPDB.isA(xOiNativeOCDProductDB) &&
		pPDB.implemented(@searchInSpecialFinalArtMap)) {
		var tRes2 = pPDB.searchInSpecialFinalArtMap(pArtKey, pLanguage);
		if (!sSpecialFinalArtMapCache.hasKey(pPID))
		    sSpecialFinalArtMapCache[pPID] = (tRes2 != NULL ? 1 : NULL);
		if (tRes2 == NULL)
		    xOiDebugPrint(@Info, 1, 
				  "couldn't open special final article map!");
		else
		if (!tRes2.empty()) {
		    var tMatch;
		    foreach(tMatch; tRes2) {
			var tArt = tMatch[0];
			var tTxt = tMatch[1];
			tRes.pushBack([pPID, tArt, tTxt]);
		    }
		}
	    }
	    else {
		var tTbl;
		if (sSpecialFinalArtMapCache.hasKey(pPID))
		    tTbl = sSpecialFinalArtMapCache[pPID];
		else {
		    tTbl = openMapTable(@VAR_CSV, sSpecialFinalArtMapName,
				        sSpecialFinalArtMapTE, pPDB, NULL);
		    sSpecialFinalArtMapCache[pPID] = tTbl;
		    if (tTbl == NULL)
			xOiDebugPrint(@Info, 1, 
				      "couldn't open special final article map!");
		}
		if (tTbl != NULL)
		    tRes = searchInSpecialFinalArtMap(pPID, tTbl, pPDB, 
						      pArtKey, pLanguage);
	    }
	    if (!tRes.empty()) {
		xOiDebugPrint(@Info, 1, 
			      ["found ", tRes.size(), 
			       " matches in special final article mapping"]);
		sArticleSearchCache[tCacheKey] = tRes;
		xOiDebugFinishFunc(tRes);
		return(tRes);
	    }
	}

	

	var tAllArticles = getMapArticles(pPDB);
	if (tAllArticles == NULL)
	    tAllArticles = @();

	var tWildcard = readOFMLClass(pPDB, "*");
	if (tWildcard instanceof String && !tWildcard.empty()) {
	    var tMID  = xOiGetAppManufacturerID(pPID);
	    var tSIDs = xOiParseString2List(xOiGetAppSeriesID(pPID), ';');
	    tAllArticles = @();
	    var tSID;
	    foreach(tSID; tSIDs) {
		var tArticles = pPDB.getArticles(tMID, tSID);
		if (tArticles != NULL && !tArticles.empty())
		xOiCopyAggr(tArticles, tAllArticles, 0);
	    }
	}

	if (tAllArticles == NULL || tAllArticles.empty()) {
	    xOiDebugPrint(@Warn, 1, 
			  ["no map articles and no articles for '*' in PDB ", 
			   pPDB.getID(), "?!"]);
	    sArticleSearchCache[tCacheKey] = tRes;
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tKeyLen = 0;
	if (pArtKey != NULL)
	    tKeyLen = pArtKey.size();
	var tArt;
	foreach(tArt; tAllArticles) {
	    xOiDebugPrint(@Info, 2, ["check article: ", tArt]);
	    var tArticle = tArt; 
	    var tArtLen = tArticle.size();
	    if (tArtLen == 0) continue;
	    var tText = NULL;
	    if (pArtKey == NULL || pArtKey == "*" ||
		(tArtLen >= tKeyLen && pArtKey == tArticle.substr(0, tKeyLen)) ||
		(tArtLen < tKeyLen && tArticle == pArtKey.substr(0, tArtLen))) {
		if (pLanguage != "NULL")
		    tText = pPDB.getArticleText(tArticle, pLanguage, @s);
	    	tRes.pushBack([pPID, tArticle, tText]);
	    }
	}

	sArticleSearchCache[tCacheKey] = tRes;

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    private func searchInSpecialFinalArtMap(pPID, pTbl, pPDB, pArtKey, 
					    pLanguage)
    {
	xOiDebugStartFunc("xOiPDManager::searchInSpecialFinalArtMap",
			  [pPID, pPDB, pArtKey, pLanguage]);

	var tRes = @();

	var tArticles = @();

	

	var tTblEntry = pTbl.readFirstEntry();
	if (tTblEntry != NULL) {
	    var tCheck = checkSpecialFinalArtMapEntry(tTblEntry, pPDB, pArtKey,
						      pLanguage, tArticles);
	    if (tCheck != NULL)
		tRes.pushBack([pPID, tCheck[0], tCheck[1]]);

	    while ((tTblEntry = pTbl.readNextEntry()) != NULL) {
	        tCheck = checkSpecialFinalArtMapEntry(tTblEntry, pPDB, pArtKey,
						      pLanguage, tArticles);
		if (tCheck != NULL)
		    tRes.pushBack([pPID, tCheck[0], tCheck[1]]);
	    }
	}

	if (tRes.size() > 1)
	    xOiDebugPrint(@Warn, 1, 
			  ["multiple matching base articles for ", pArtKey]);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    private func checkSpecialFinalArtMapEntry(pTblEntry, pPDB, pArtKey, 
					      pLanguage, pArticles)
    {
	xOiDebugStartFunc2("xOiPDManager::checkSpecialFinalArtMapEntry",
			   [pTblEntry, pPDB, pArtKey, pLanguage, pArticles]);

	var tRes    = NULL;
	var tKeyLen = pArtKey.size();

	var tFinal = pTblEntry[0];
	var tBase  = pTblEntry[1];
	var tText  = NULL;
	if (tFinal.size() >= tKeyLen && pArtKey == tFinal.substr(0, tKeyLen) &&
	    pArticles.find(tBase) < 0) {
	    if (pLanguage != "NULL")
		tText = pPDB.getArticleText(tBase, pLanguage, @s);
	    tRes = [tBase, tText];
	    pArticles.pushBack(tBase);
	}

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getReplacements(pProg, pArticle, pLanguage)
    {
	xOiDebugStartFunc("xOiPDManager::getReplacements",
			  [pProg, pArticle, pLanguage]);

	var tPID     = pProg[0];
	var tPDBObj  = pProg[1];
	var tPDBPath = pProg[2];

	var tRes = @();

	if (tPDBObj == NULL) {
	    xOiDebugPrint(@Warn, 1, "ignoring request for program without " +
				    "product database!");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tLanguage = pLanguage;
	if (pLanguage == NULL)
	    
	    tLanguage = getPlanning().getPDLanguage(tPID);

	var tRegisteredPDBs = getPDB_IDs();
	var tRegisteredProgs = @();
	var tID;
	foreach(tID; tRegisteredPDBs) {
	    var tPDB = getProgPDB(tID);
	    if (tPDB == NULL)
		xOiDebugPrint(@Warn, 1, ["no product database for registered ",
					 "program ", tID]);
	    else
	        xOiCopyAggr(tPDB.getPrograms(), tRegisteredProgs, 0);
	}

	var tRegistered = (tRegisteredProgs.find(tPID) >= 0);

	if (!tRegistered) addProductDB(tPDBObj, tPID, tPDBPath, @(tPID));

	var tPDB  = getProgPDB(tPID);

	var tReplacements = NULL;

	if (tPDB.hasMember(@getReplacements))
	    tReplacements = tPDB.getReplacements(pArticle);
	else
	    xOiDebugPrint(@Info, 1,
			  [tPDBObj, " has no method getReplacements()!"]);

	if (tReplacements == NULL) {
	    var tTbl = openMapTable(@VAR_CSV, sReplacementsTbl, 
				    sReplacementsTE, tPDB, NULL);
	    if (tTbl == NULL) {
		xOiDebugPrint(@Info, 1, "trying old style naming scheme");
		tTbl = openMapTable(@VAR_CSV, "replacement_dat", 
				    sReplacementsTE, tPDB, NULL);
	    }
	    if (tTbl == NULL)
		xOiDebugPrint(@Warn, 1, "could not open replacements table!");
	    else
		tReplacements = 
		tTbl.readNEntriesFor(@(["article_nr", pArticle]),
				     @("manufacturer", "series", "replacement"),
				     0);
	}

	if (tReplacements == NULL) tReplacements = @();

	var tRepl;
	foreach(tRepl; tReplacements) {
	    var tManufacturer = tRepl[0];
	    var tSeries = tRepl[1];
	    var tArticle = tRepl[2];

	    
	    
	    

	    
	    var tATxt = tPDB.getArticleText(tArticle, tLanguage, @s);

	    tRes.pushBack([tArticle, xOiConcatStrings(tATxt)]);
	}

	if (!tRegistered) delProductDB(tPID);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    

    public func paramsString(pParams)
    {
	if (pParams == NULL) return("");

	if (pParams instanceof String) return(pParams);

	if (pParams instanceof Vector) return(xOiParam2Str(pParams));

	return("");
    }

    
    
    func getPDBFor(pObj)
    {
	xOiDebugStartFunc2("xOiPDManager::getPDBFor", pObj);

	if (hasTransaction(pObj) && mTransPDB != NULL) {
	    xOiDebugFinishFunc([mTransPDB.getID(), " (use transaction PDB)"]);
	    return(mTransPDB);
	}

	xOiDebugPrint(@Info, 2, ["PID: ", pObj.getProgram()]);

	var tPDB = OiPDManager::getPDBFor(pObj);

	if (hasTransaction(pObj)) mTransPDB = tPDB;

	var tID = NULL;
	if (tPDB != NULL) tID = [tPDB.getID(), tPDB.getClass()];
	xOiDebugFinishFunc(tID);

	return(tPDB);
    }

    
    
    
    
    
    public func getMapArticles(pPDB)
    {
	return(NULL);
    }

    
    
    
    
    
    
    
    
    
    protected func readOFMLClass(pPDB, pArticle)
    {
	return (NULL);
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getNativePropKeys(pObj, pPKey, pPVal, ...)
    {
	var tPVal = pPVal[0];
	var tReplaceVoid = 0;
	if (pPVal.size() > 1)
	    tReplaceVoid = pPVal[1];
	var tPadStr = 0;
	if (pPVal.size() > 2)
	    tPadStr = pPVal[2];

	xOiDebugStartFunc("xOiPDManager::getNativePropKeys", 
			  [pObj, pPKey, tPVal, tReplaceVoid, tPadStr]);

	var tRet = NULL;

	var tObj = pObj;
	if (pObj != NULL && pObj.hasMember(@isCat) && pObj.isCat(@IF_Article))
	    tObj = pObj.getArticleObj();

	xOiDebugPrint(@Info, 2, ["effective ", tObj]);

	if (tObj == NULL || !tObj.hasMember(@isCat) || 
	    !tObj.isCat(@IF_Article)) {
	    xOiDebugPrint(@Warn, 1, "invalid object!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tArticle = tObj.getArticleSpec();

	if (tArticle == NULL) {
	    xOiDebugPrint(@Warn, 1, "no article!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	if (!(pPKey instanceof Symbol) || !tObj.isCat(@IF_Property) || 
	    !tObj.hasProperty(pPKey)) {
	    xOiDebugPrint(@Warn, 1, "invalid property!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tPDB = self.getPDBFor(tObj);

	if (tPDB == NULL || !tPDB.hasMember(@getNativeValue)) {
	    xOiDebugPrint(@Warn, 1, "no or invalid PDB!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tPName = self.propKey2PropName(tObj, pPKey);

	if (tPName == NULL) {
	    xOiDebugPrint(@Warn, 1, "no PD property!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tPClass    = tObj.getPropClass(pPKey);

	var tPValues = tPVal;
	if (!(tPVal instanceof Vector))
	    tPValues = [tPVal];

	var tNValues = Vector(0);

	var tPV;
	foreach(tPV; tPValues) {
	    var tNVal = tPDB.getNativeValue(tArticle, tPClass, tPName, tPV, 
					    tReplaceVoid, tPadStr);
	    tNValues.pushBack(tNVal);
	}

	var tNativeVal = tNValues;
	if (!(tPVal instanceof Vector))
	    tNativeVal = tNValues.front();

	tRet = [tPClass, tPName, tNativeVal];

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    public func propKey2PropName(pObj, pPKey)
    {
	xOiDebugStartFunc2("xOiPDManager::propKey2PropName", [pObj, pPKey]);

	var tPlanning = getPlanning();
	var tPName = NULL;

	var tSafePNames = safePropNames(pObj.getProgram());
	xOiDebugPrint(@Info, 1, ["safe property names: ", tSafePNames]);

	if (pObj.isCat(@AnyArticle)) {
	    tPName = xOiSymbol2String(pPKey);
	    xOiDebugPrint(@Info, 2, 
			  "using prop key for prop name (stripping 'S')");
	    xOiDebugFinishFunc(tPName);
	    return(tPName);
	}

	var tArticle = pObj.getArticleSpec();
	xOiDebugPrint(@Info, 2, ["article: ", tArticle]);

	if (tArticle == NULL) {
	    xOiDebugPrint(@Info, 1, "ignore non PD relevant object");
	    xOiDebugFinishFunc(tPName);
	    return(tPName);
	}

	if (hasTransaction(pObj)) {
	    if (mPropKey2PropName == NULL)
		buildTransPropMaps(pObj, mTransPDB, mTransArticle);
	    if (mPropKey2PropName == NULL) {
		if (readNonPDProps(pObj, NULL).find(pPKey) < 0) {
	    	    xOiDebugPrint(@Warn, 1,
			          "no property mapping -> " +
			          "using prop key for prop name");
		    if (tSafePNames)
			tPName = String(pPKey);
		    else
	    	        tPName = _PKey2String(pPKey, pObj, NULL);
		}
	    }
	    else
	    if (mPropKey2PropName.hasKey(pPKey))
		tPName = mPropKey2PropName[pPKey][1];
	}
	else {
	    if (mPropKey2PropName != NULL && mPropKey2PropName.hasKey(pPKey))
		tPName = mPropKey2PropName[pPKey][1];
	    else
	    	tPName = readPropNameForKey(pObj, pPKey);
	}

	if (tPName != NULL && tPName instanceof Int && tPName == -1) {
	    if (readNonPDProps(pObj, NULL).find(pPKey) < 0) {
		xOiDebugPrint(@Warn, 1, "no property mapping -> " +
			                "using prop key for prop name");
		if (tSafePNames)
		    tPName = String(pPKey);
		else
		    tPName = _PKey2String(pPKey, pObj, NULL);
	    }
	    else {
	        xOiDebugPrint(@Info, 1, "non PD property");
		tPName = NULL;
	    }
	}

	xOiDebugFinishFunc(tPName);
	return(tPName);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func propName2PropKey(pObj, pPClass, pPName)
    {
	xOiDebugStartFunc2("xOiPDManager::propName2PropKey",
			  [pObj, pPClass, pPName]);

	var tRet = Vector(2);

	var tObj = pObj;
	if (pObj == NULL) tObj = mTransObj;

	if (tObj == NULL) {
	    xOiDebugPrint(
		@Warn, 1,
		"method for NULL object called not from within a transaction");
	    if (mUpdatabilityCheckContext == NULL) {
	        xOiDebugFinishFunc(tRet);
	        return(tRet);
	    }
	}

	xOiDebugPrint(@Info, 2, ["object is ", tObj]);

	var tPlanning = getPlanning();

	var tSafePNames = 0;
	if (tObj != NULL)
	    tSafePNames = safePropNames(tObj.getProgram());
	xOiDebugPrint(@Info, 1, ["safe property names: ", tSafePNames]);

	if (((tObj != NULL && tObj.isCat(@AnyArticle)) ||
	     (tObj == NULL && 
	      eval(mUpdatabilityCheckContext[5]).subClassOf(xOiAnyArticle)))) {
	    tRet = [xOiString2Symbol(pPName), ""];
	    xOiDebugPrint(@Info, 2, 
			  "using prop name for prop key (prefix 'S')");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	if (tObj != NULL && pPClass != NULL &&
	    (hasTransaction(tObj) || mPropName2PropKey != NULL)) {
	    if (mPropName2PropKey == NULL)
		buildTransPropMaps(tObj, mTransPDB, mTransArticle);
	    if (mPropName2PropKey == NULL) {
	    	xOiDebugPrint(@Warn, 1,
			      "no property mapping -> " +
			      "using prop name for prop key");
		if (tSafePNames)
		    tRet = [Symbol(pPName), ""];
		else
		    tRet = [_String2PKey(pPName, tObj), ""];
	    }
	    else {
	    	xOiDebugPrint(@Info, 1, "using cached property mapping");
	    	var tKey = pPClass+pPName;
	    	if (mPropName2PropKey.hasKey(tKey))
		    tRet = mPropName2PropKey[tKey];
	    }
	}
	else {
	    var tPClasses = [pPClass];
	    var tPDB, tObj2 = NULL;
	    if (pPClass == NULL && tObj != NULL) {
		tPDB  = getPDBFor(tObj);
		tObj2 = tObj.getArticleObj();
		if (tPDB != NULL && tObj2 != NULL)
		    tPClasses = 
		    tPDB.getArticlePropClasses(tObj2.getArticleSpec());
	    }
	    xOiDebugPrint(@Info, 2, ["checking classes: ", tPClasses]);
	    var tPClass;
	    foreach(tPClass; tPClasses) {
		if (tPClass == NULL) continue;
	    	tRet = readPropKeyForName(tObj2 != NULL ? tObj2 : tObj, 
					  tPClass, pPName);
		if (tRet != NULL && tRet[0] != NULL) break;
	    }
	    if (tRet == NULL) {
	    	xOiDebugPrint(@Warn, 1,
			      "no property mapping -> " +
			      "using prop name for prop key");
		if (tSafePNames)
		    tRet = [Symbol(pPName), ""];
		else
		    tRet = [_String2PKey(pPName, tObj), ""];
	    }
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    public func superTypeOf(pType)
    {
	return(pType.getSuperClass());
	
    }

    private func getTypeObj(pArg)
    {
	if (pArg instanceof String)
	    
	    return(eval(pArg));

	
	return(typeOf(pArg));
    }

    
    
    func getClasses(pObj)
    {
	xOiDebugStartFunc2("xOiPDManager::getClasses", pObj);

	var tCL = @();

	var tPlanning = getPlanning();

	var tType = getTypeObj(pObj);
	var tFullName = tType.getFullName();

	if (mClassesCache.hasKey(tFullName)) {
	    xOiDebugPrint(@Info, 1, "using cache");
	    tCL = mClassesCache[tFullName];
	    xOiDebugFinishFunc(tCL);
	    return(tCL);
	}

	while (tType != NULL) {
	    tCL.pushBack(tType.getFullName());
	    tType = superTypeOf(tType);
	}

	mClassesCache[tFullName] = tCL;

	xOiDebugFinishFunc(tCL);
	return(tCL);
    }

    
    
    
    
    
    
    
    protected func readClassProperties(pObj)
    {
	return(NULL);
    }

    
    
    
    
    func buildTransPropMaps(pObj, pPDB, pArticle, ...)
    {
	xOiDebugStartFunc2("xOiPDManager::buildTransPropMaps",
			   [pObj, pPDB, pArticle]);

	var tArticle = pArticle[0];
	var tUseTransCache = 1;
	if (pArticle.size() > 1)
	    tUseTransCache = pArticle[1];

	var tPlanning = getPlanning();
	var tClassProps = NULL;

	if (pObj.isCat(@AnyArticle) && !pObj.getPropClasses().empty()) {
	    tClassProps = Vector(0);
	    var tP;
	    foreach(tP; pObj.getPropertyKeys()) {
		var tPKey = tP;
		var tPClass = pObj.getPropClass(tPKey);
		if (tPClass != NULL)
		    tClassProps.pushBack([tPClass, xOiSymbol2String(tPKey),
					  tPKey, ""]);
	    }
	}
	else
	    tClassProps = readClassProperties(pObj);

	if (tClassProps == NULL) {
	    mPropKey2PropName = NULL;
	    mPropName2PropKey = NULL;
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tPClasses = (hasTransaction(pObj) && tUseTransCache) ?
			mTransPClasses : pPDB.getArticlePropClasses(tArticle);
	xOiDebugPrint(@Info, 2, ["prop classes: ", tPClasses]);

	mPropKey2PropName = Hash();
	mPropName2PropKey = Hash();
	var tP;
	foreach(tP; tClassProps) {
	    var tPClass = tP[0];
	    var tPName  = tP[1];
	    var tPKey   = Symbol(tP[2]);
	    var tSpType = tP[3];

	    
	    if (tPClasses.find(tPClass) < 0) continue;

	    xOiDebugPrint(@Info, 2, ["add: ", [tPClass, tPName, tPKey, tSpType]]);
	    mPropKey2PropName[tPKey] = [tPClass, tPName];
	    mPropName2PropKey[tPClass+tPName] = [tPKey, tSpType];
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    protected func readPropNameForKey(pObj, pPKey)
    {
	return(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func readNonPDProps(pObj, pTbl)
    {
	return(@());
    }

    
    
    private func propKey2PropClassName(pObj, pPKey, pPDB, pNonPDProps)
    {
	var tSafePNames = safePropNames(pObj.getProgram());
	xOiDebugPrint(@Info, 1, ["safe property names: ", tSafePNames]);

	var tPClass = NULL;
	var tPName = NULL;

	if (mPropKey2PropName == NULL) {
	    
	    
	    if (pNonPDProps.find(pPKey) < 0) {
		if (tSafePNames)
		    tPName = String(pPKey);
		else
		    tPName = _PKey2String(pPKey, pObj, pPDB);
		
		tPClass = pObj.getPropClass(pPKey);
		if (tPClass == NULL) {
		    
		    xOiDebugPrint(@Warn, 1, 
				  ["no class for property ", tPName, "?!"]);
		    tPClass = "";
		}
	    }
	}
	else
	if (mPropKey2PropName.hasKey(pPKey)) {
	    var tPCN = mPropKey2PropName[pPKey];
	    tPClass = tPCN[0];
	    tPName = tPCN[1];
	}

	return([tPClass, tPName]);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    protected func readPropKeyForName(pObj, pPClass, pPName)
    {
	return(NULL);
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    public func buildPropValList(pObj, pPDB, pArticle, ...)
    {
	xOiDebugStartFunc("xOiPDManager::buildPropValList",
			   [pObj, pPDB, pArticle]);

	var tArticle = pArticle[0];
	var tPropChanged = 0;
	if (pArticle.size() > 1)
	    tPropChanged = pArticle[1];
	var tArtChanged = 0;
	if (pArticle.size() > 2)
	    tArtChanged = pArticle[2];

	var tHasTransaction = hasTransaction(pObj);

	xOiDebugPrint(@Info, 2, ["is transaction object? ", tHasTransaction]);
	xOiDebugPrint(@Info, 2, 
		      ["sPropValList != NULL ? ", (sPropValList != NULL)]);

	if (!tPropChanged && tHasTransaction && sPropValList != NULL) {
	    xOiDebugPrint(@Info, 1, "using transaction cache");
	    xOiDebugPrint(@Info, 2, ["return list: ", sPropValList]);
	    xOiDebugFinishFunc(NULL);
	    return(sPropValList);
	}

	var tPropL = @();

	if (pPDB == NULL) {
	    xOiDebugPrint(@Warn, 1, "no product database object given!");
	    xOiDebugFinishFunc(tPropL);
	    return(tPropL);
	}

	var tUseTransCache = !(tPropChanged && tArtChanged);
	buildTransPropMaps(pObj, pPDB, tArticle, tUseTransCache);

	var tPCL = hasTransaction(pObj) ?
		   mTransPClasses : pPDB.getArticlePropClasses(tArticle);

	var tNonPDProps = readNonPDProps(pObj, NULL);

	xOiDebugPrint(@Info, 2, ["non PD props: ", tNonPDProps]);

	var tPositions = Hash();
	var tPKey;
	foreach(tPKey; pObj.getPropertyKeys()) {
	    var tPCN = propKey2PropClassName(pObj, tPKey, pPDB, tNonPDProps);
	    var tPClass = tPCN[0];
	    var tPName  = tPCN[1];

	    xOiDebugPrint(@Info, 2, ["key: ", tPKey, " -> prop: ", tPName]);

	    if (tPName == NULL) continue;

	    var tPType  = pObj.getPropertyDef(tPKey)[4];
	    var tPVal   = pObj.getPropValue(tPKey);
	    var tPVals  = xOiGetValuesFromChoiceList(tPType);
	    var tPState = pObj.getPropState(tPKey);

	    if (tPClass != NULL && tPClass.empty()) {
		
		
		if (tPCL != NULL && tPCL.size() == 1) tPClass = tPCL.front();
	    }
	    if (tPropChanged && tArtChanged) tPClass = "";

	    tPropL.pushBack([tPClass, tPName, tPVal, tPVals, tPState]);
	    tPositions[tPClass+tPName] = pObj.getPropertyPos(tPKey);
	}

	
	if (!tPropL.empty())
	    tPropL.sort(PropListSort(tPositions));

	if (tHasTransaction)
	    sPropValList = tPropL;
	else {
	    mPropKey2PropName = NULL;
	    mPropName2PropKey = NULL;
	}

	xOiDebugPrint(@Info, 2, ["return list: ", tPropL]);
	xOiDebugFinishFunc(NULL);
	return(tPropL);
    }

    
    
    
    
    private func buildPropValList2(pObj, pPropDefs)
    {
	xOiDebugStartFunc2("xOiPDManager::buildPropValList2", NULL);

	var tPropL = @();

	var tPositions = Hash();

	var tPD;
	foreach(tPD; pPropDefs) {
	    var tPClass = tPD[0];
	    var tPName  = tPD[1];
	    var tPDef   = tPD[2];
	    var tCVal   = tPD[3];
	    var tCValues= tPD[4];
	    var tPPos   = tPD[5];
	    var tState  = tPD[6];

	    if (pObj != NULL) {
	        var tPKeyType = propName2PropKey(pObj, tPClass, tPName);
	        var tPKey  = tPKeyType[0];
	        if (tPKey == NULL) {
		    xOiDebugPrint(@Warn, 1, ["no property associated with ", 
					     tPClass, ".", tPName]);
		    continue;
		}
	    }

	    if (tPDef == NULL) continue;

	    var tPValues = @();
	    if (tCValues == NULL || tCValues.empty())
		tPValues = NULL;
	    else {
		var tCV;
		foreach(tCV; tCValues) {
		    var tPVal = tCV[0];
		    tPValues.pushBack(tPVal);
		}
	    }
	    tPropL.pushBack([tPClass, tPName, tCVal, tPValues, tState]);
	    tPositions[tPClass+tPName] = tPPos;
	}

	
	if (!tPropL.empty()) tPropL.sort(PropListSort(tPositions));

	xOiDebugPrint(@Info, 2, ["return list: ", tPropL]);
	xOiDebugFinishFunc(NULL);
	return(tPropL);
    }

    
    
    
    
    
    protected func readParamProps(pObj)
    {
	return(NULL);
    }

    
    
    
    
    
    public func getParamProps4TransObj()
    {
	xOiDebugStartFunc2("xOiPDManager::getParamProps4TransObj", NULL);

	if (mTransObj == NULL || !hasTransaction(mTransObj)) {
	    xOiDebugPrint(@Info, 1, "no transaction");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tParamProps = readParamProps(mTransObj);
	if (tParamProps == NULL) {
	    xOiDebugPrint(@Info, 1, "no parameter properties");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tPropL = @();

	buildTransPropMaps(mTransObj, mTransPDB, mTransArticle);

	var tPCL = mTransPClasses;
	var tNonPDProps = readNonPDProps(mTransObj, NULL);

	var tPositions = Hash();
	var tPKey;
	foreach(tPKey; mTransObj.getPropertyKeys()) {
	    if (tParamProps.find(tPKey) < 0) continue;

	    var tPCN = propKey2PropClassName(mTransObj, tPKey, mTransPDB, 
					     tNonPDProps);
	    var tPClass = tPCN[0];
	    var tPName  = tPCN[1];

	    xOiDebugPrint(@Info, 2, ["key: ", tPKey, " -> prop: ", tPName]);

	    if (tPName == NULL) continue;

	    var tPVal   = mTransObj.getPropValue(tPKey);

	    if (tPClass != NULL && tPClass.empty()) {
		
		
		if (tPCL != NULL && tPCL.size() == 1) tPClass = tPCL.front();
	    }
	    tPropL.pushBack([tPClass, tPName, tPVal]);
	    tPositions[tPClass+tPName] = mTransObj.getPropertyPos(tPKey);
	}

	
	if (!tPropL.empty())
	    tPropL.sort(PropListSort(tPositions));

	xOiDebugFinishFunc(tPropL);
	return(tPropL);
    }

    
    
    
    func getPropType(pPType, pValues, pObj)
    {
	var tRetType = xOiBuildChoiceList2(pPType, pValues, pObj);
	if (tRetType == NULL) tRetType = pPType;
	return(tRetType);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setupProps(pObj, ...)
    {
	xOiDebugStartFunc("xOiPDManager::setupProps", pObj);

	var tObj = pObj[0];
	var tArt = NULL;
	if (pObj.size() > 1)
	    tArt = pObj[1];
	var tAddSpec = NULL;
	var tAddSpecType = NULL;
	if (pObj.size() > 3) {
	    tAddSpec =  pObj[2];
	    tAddSpecType =  pObj[3];
	}

	if (tObj.getDynamicProps().hasKey(@_XOI_SETUP_CONFIGURATION)) {
	    xOiDebugPrint(@Info, 1, "during setupConfiguration() ...");
	    
	    var tInfo = tObj.getDynamicProps()[@_XOI_SETUP_CONFIGURATION];
	    updatePropList(tObj, tInfo[0], tInfo[1], tInfo[2], NULL, 0, 0, 0);

	    
	    sTransVarCode = NULL;
	    sTransOFMLVarCode = NULL;
	    sTransFinalType = NULL;

	    xOiDebugFinishFunc(NULL);
	    return;
	}

	
	
	
	
	
	
	
	
	
	

	var tBaseArt = tObj.getArticleSpec();

	if (tArt != NULL && tBaseArt != NULL && tArt != tBaseArt &&
	    tAddSpec == NULL) {
	    tAddSpec = tArt;
	    tAddSpecType = @Final;
	    tArt = tBaseArt;
	}

	_setupProps(tObj, tArt, tAddSpec, tAddSpecType);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    public func resetProps(pObj)
    {
	var tDynProps = pObj.getDynamicProps();
	if (tDynProps.hasKey(@__XOIPDM_STATE) &&
	    tDynProps[@__XOIPDM_STATE] == @Default) return;

	if (pObj.isCat(@AnyArticle)) pObj.clearProperties();

	var tChangedProps = _setupProps(pObj, NULL, NULL, NULL);

	if (tChangedProps != NULL) pObj.propsChanged(tChangedProps, 0);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getAllPropSpecs(pObj, pRequiredState)
    {
	xOiDebugStartFunc("xOiPDManager::getAllPropSpecs", 
			  [pObj, pRequiredState]);

	var tSpecs = @();

	var tArticle;

	if (pObj == NULL || (tArticle = pObj.getArticleSpec()) == NULL || 
	    !(tArticle instanceof String) || tArticle.empty()) {
	    xOiDebugPrint(@Warn, 1, "no article!");
	    xOiDebugFinishFunc(tSpecs);
	    return(tSpecs);
	}

	var tPDB = getPDBFor(pObj);

	if (tPDB == NULL || !tPDB.isA(xOiNativeOCDProductDB) ||
	    !tPDB.implemented(@getAllPropSpecs)) {
	    xOiDebugPrint(@Warn, 1, "no/invalid product database!");
	    xOiDebugFinishFunc(tSpecs);
	    return(tSpecs);
	}

	var tDbRes = tPDB.getAllPropSpecs(tArticle);

	

	var tP;
	foreach(tP; tDbRes) {
	    xOiDebugPrint(@Info, 2, ["check prop ", tP]);
	    var tClass  = tP[0];
	    var tProp   = tP[1];
	    var tPDef   = tP[2];
	    var tTxtID  = tP[3];
	    var tPos    = tP[4];
	    var tState  = _old2NewPropState(tP[5]);
	    var tHintID = tP[6];

	    if (tState < pRequiredState) {
		xOiDebugPrint(@Info, 1, ["state of ", tProp, 
					 " doesn't match required state"]);
		continue;
	    }

	    var tPKeyType = propName2PropKey(pObj, tClass, tProp);
	    var tPKey  = tPKeyType[0];
	    var tPKSpT = tPKeyType[1];  

	    if (tPKey == NULL) {
		xOiDebugPrint(@Info, 1, ["no prop key for ", tProp, " ?!"]);
		continue;
	    }

	    if (tPKSpT != NULL && !tPKSpT.empty())
		tPDef = [tPDef[0], tPDef[1], tPDef[2], tPDef[3], "u "+tPKSpT];

	    var tTextMap = tPDB.getPropertyTextEntries(tTxtID);

	    var tSpec = [tClass, tProp, tPDef, tTextMap, tPos, tState, NULL];

	    xOiDebugPrint(@Info, 1, 
			  ["adding property ", tPKey, " with spec ", tSpec]);

	    tSpecs.pushBack([tPKey, tSpec]);
	}

	xOiDebugFinishFunc(NULL);
	return(tSpecs);
    }

    
    
    private func _old2NewPropState(pState)
    {
	if (pState == -1) return(0);
	if (pState ==  0) return(1);
	if (pState ==  1) return(3);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getAllPropValueInfos(pObj, pPKey, pPClass)
    {
	xOiDebugStartFunc("xOiPDManager::getAllPropValueInfos", 
			  [pObj, pPKey, pPClass]);

	var tArticle;

	if (pObj == NULL || (tArticle = pObj.getArticleSpec()) == NULL || 
	    !(tArticle instanceof String) || tArticle.empty()) {
	    xOiDebugPrint(@Warn, 1, "no article!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tPDB = getPDBFor(pObj);

	if (tPDB == NULL || !tPDB.isA(xOiNativeOCDProductDB) ||
	    !tPDB.implemented(@getAllPropValueInfos)) {
	    xOiDebugPrint(@Warn, 1, "no/invalid product database!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tPName  = propKey2PropName(pObj, pPKey);

	if (tPName == NULL) {
	    xOiDebugPrint(@Info, 1, "ignore non PD relevant property");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tDbRes = tPDB.getAllPropValueInfos(tArticle, pPClass, tPName);

	if (tDbRes == NULL) {
	    xOiDebugPrint(@Info, 1, "no info from PDB");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tValues = @();

	var tVI;
	foreach(tVI; tDbRes) {
	    var tValue      = tVI[0];
	    var tTxtID      = tVI[1];
	    var tXtraCharge = tVI[2];
	    var tXtraChCurr = tVI[3];

	    var tTextMap = tPDB.getPropValueTextEntries(tTxtID);

	    var tInfo = [tValue, tTextMap, tXtraCharge, tXtraChCurr];

	    xOiDebugPrint(@Info, 1, ["adding value info ", tInfo]);

	    tValues.pushBack(tInfo);
	}

	xOiDebugFinishFunc(NULL);
	return(tValues);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getDefaultPropGroupSpecs(pObj)
    {
	xOiDebugStartFunc("xOiPDManager::getDefaultPropGroupSpecs", pObj);

	var tSpecs = @();

	var tArticle;

	if (pObj == NULL || (tArticle = pObj.getArticleSpec()) == NULL || 
	    !(tArticle instanceof String) || tArticle.empty()) {
	    xOiDebugPrint(@Warn, 1, "no article!");
	    xOiDebugFinishFunc(tSpecs);
	    return(tSpecs);
	}

	var tPDB = getPDBFor(pObj);

	if (tPDB == NULL || !tPDB.isA(xOiNativeOCDProductDB) ||
	    !tPDB.implemented(@getDefaultPropGroups)) {
	    xOiDebugPrint(@Warn, 1, "no/invalid product database!");
	    xOiDebugFinishFunc(tSpecs);
	    return(tSpecs);
	}

	var tDbRes = tPDB.getDefaultPropGroups(tArticle);

	

	var tG;
	foreach(tG; tDbRes) {
	    xOiDebugPrint(@Info, 2, ["check group ", tG]);
	    var tGroup   = tG[0];
	    var tProps1  = tG[1];
	    var tIsClass = tG[2];
	    var tTxtID   = tG[3];

	    var tProps2 = @();
	    var tP;
	    foreach(tP; tProps1) {
		var tPKeyType = propName2PropKey(pObj, tP[0], tP[1]);

		if (tPKeyType[0] == NULL)
		    xOiDebugPrint(@Warn, 1, ["no prop key for ", tP[1], " ?!"]);
		else
		    tProps2.pushBack(tPKeyType[0]);
	    }

	    var tTextMap = Vector(0);
	    if (tTxtID.size() > 0)
		tTextMap = (tIsClass ? tPDB.getPropClassTextEntries(tTxtID) :
				       tPDB.getPropGroupTextEntries(tTxtID));

	    tSpecs.pushBack([tGroup, tProps2, tIsClass, tTextMap]);
	}

	xOiDebugFinishFunc(tSpecs);
	return(tSpecs);
    }

    
    
    
    
    
    
    
    public func updateConfiguration(pObj)
    {
	xOiDebugStartFunc("xOiPDManager::updateConfiguration", pObj);

	self._updateConfiguration(pObj, 0);

	xOiDebugFinishFunc(NULL);
    }

    
    public func updateConfiguration2(pObj, pUpdateLanguageOnly, ...)
    {
	xOiDebugStartFunc("xOiPDManager::updateConfiguration2", 
			  [pObj, pUpdateLanguageOnly]);

	var tUpdateLanguageOnly = pUpdateLanguageOnly[0];

	self._updateConfiguration(pObj, tUpdateLanguageOnly);

	xOiDebugFinishFunc(NULL);
    }

    private func _updateConfiguration(pObj, pUpdateLanguageOnly)
    {
	xOiDebugStartFunc("xOiPDManager::_updateConfiguration", 
			  [pObj, pUpdateLanguageOnly]);

	

	if (pObj == NULL || !pObj.hasMember(@isCat) || 
	    !pObj.isCat(@IF_Article)) {
	    xOiDebugPrint(@Warn, 1, "no article instance given!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tObj = pObj.getArticleObj();

	var tUpdateState = tObj.getObjState(@OI_UpdateState);

	if (!(tUpdateState == @Updatable || tUpdateState == @Migratable ||
	      (tUpdateState == @Up2Date && pUpdateLanguageOnly))) {
	    xOiDebugPrint(@Info, 1, "nothing to do");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tArticle = tObj.getArticleSpec();

	startTransaction(tObj, tArticle); 

	var tPDB = mTransPDB;

	if (tPDB == NULL || tArticle == NULL || tArticle.empty()) {
	    
	    finishTransaction(tObj);
	    if (tArticle == NULL || tArticle.empty())
		xOiDebugPrint(@Warn, 1, "no article!");
	    if (tPDB == NULL)
		xOiDebugPrint(@Warn, 1, "no product database!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	

	var tObjName  = tObj.getName();
	var tPropDefs = NULL;

	var tPVals = buildPropValList(tObj, tPDB, tArticle);
	if (tPVals.empty()) tPVals = NULL;

	if (sUpdatabilityCheckPropDefs != NULL &&
	    sUpdatabilityCheckPropDefs.hasKey(tObjName)) {
	    xOiDebugPrint(@Info, 1, 
			  "using prop defs from previous checkUpdatability()");
	    tPropDefs = sUpdatabilityCheckPropDefs[tObjName];
	    var tPropOffset = tObj.getExtPropOffset();
	    var tPropDefs2 = Vector(0), tPD;
	    foreach(tPD; tPropDefs) 
		tPropDefs2.pushBack([tPD[0], tPD[1], tPD[2], tPD[3], tPD[4],
				     tPD[5] + tPropOffset, tPD[6]]);
	    tPropDefs = tPropDefs2;
	    sUpdatabilityCheckPropDefs.remove(tObjName);
	}
	else {
	    
	    var tLanguage = getPlanning().getPDLanguage(tObj);

	    if (pUpdateLanguageOnly && tPDB.isA(xOiNativeOCDProductDB) &&
		tPDB.implemented(@getPropDefs3))
		tPropDefs = tPDB.getPropDefs3(tArticle, tObj.getExtPropOffset(),
					      tLanguage, tPVals);
	    else
		tPropDefs = tPDB.getPropDefs(tArticle, tObj.getExtPropOffset(),
					     tLanguage, NULL, tPVals);
	}

	

	if (tPropDefs != NULL)
	    updatePropList(tObj, tPropDefs, tPDB, tArticle, NULL, 0, 0, 0);

	xOiDebugPrint(@Info, 2, ["props: ", tObj.getPropertyKeys()]);

	
        sTransFinalType = NULL;
        sTransVarCode = NULL;
        sTransOFMLVarCode = NULL;

	if (tPVals == NULL || tPropDefs == NULL || pUpdateLanguageOnly) {
	    finishTransaction(tObj);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	
	var tPropDefHash = Hash();
	var tP;
	foreach(tP; tPropDefs) {
	    var tPClass = tP[0];
	    var tPName  = tP[1];
	    tPropDefHash[tPName] = 1;
	}
	foreach(tP; tPVals) {
	    var tPClass = tP[0];
	    var tPName  = tP[1];
	    if (!tPropDefHash.hasKey(tPName)) {
	        var tPKeyType = propName2PropKey(tObj, tPClass, tPName);
	        var tPKey  = tPKeyType[0];
		if (tPKey != NULL) tObj.removeProperty(tPKey);
	    }
	}

	finishTransaction(tObj);

	xOiDebugFinishFunc(NULL);
	return;
    }

    
    
    
    
    private func checkCompleteVarCodes(pNewCode, pOldCode, pIgnorePClass)
    {
	xOiDebugStartFunc2("xOiPDManager::checkCompleteVarCodes", 
			   [pNewCode, pOldCode, pIgnorePClass]);

	var tNewProps = xOiParseString2List(pNewCode, ';');
	var tOldProps = xOiParseString2List(pOldCode, ';');

	xOiDebugPrint(@Info, 2, ["old props: ", tOldProps]);
	xOiDebugPrint(@Info, 2, ["new props: ", tNewProps]);

	if (tNewProps == NULL || tOldProps == NULL || 
	    tNewProps.empty() || tOldProps.empty()) {
	    xOiDebugPrint(@Warn, 1, "varcode parsing error!");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	if (pIgnorePClass) {
	    tNewProps = stripPClasses(tNewProps);
	    tOldProps = stripPClasses(tOldProps);
	    xOiDebugPrint(@Info, 2, 
			  ["old props (stripped classes): ", tOldProps]);
	    xOiDebugPrint(@Info, 2, 
			  ["new props (stripped classes): ", tNewProps]);
	    if (tOldProps == NULL || tNewProps == NULL) {
		xOiDebugPrint(@Warn, 1, "varcode parsing error!");
		xOiDebugFinishFunc(0);
		return(0);
	    }
	}

	var tOldProp;
	foreach(tOldProp; tOldProps) {
	    if (tNewProps.find(tOldProp) >= 0) 
		continue;
	    
	    var tPCV  = xOiParseString2List(tOldProp, '=');
	    if (tPCV == NULL || tPCV.size() != 2) {
		xOiDebugPrint(@Warn, 1, "varcode parsing error!");
		xOiDebugFinishFunc(0);
		return(0);
	    }
	    if (tPCV[1] == "VOID") 
		xOiDebugPrint(@Info, 1, 
			      ["ignoring unspecified property: ", tOldProp]);
	    else {
		xOiDebugPrint(@Info, 1, ["'", tOldProp, 
					 "' not contained in new varcode!"]);
		xOiDebugFinishFunc(0);
		return(0);
	    }
	}

	xOiDebugFinishFunc(1);
	return(1);
    }

    
    
    
    private func stripPClasses(pProps)
    {
	xOiDebugStartFunc2("xOiPDManager::stripPClasses", pProps);

	var tNewList = @();
	var tProp;
	foreach(tProp; pProps) {
	    
	    var tPCV  = xOiParseString2List(tProp, '=');
	    if (tPCV == NULL || tPCV.size() == 0 || tPCV.size() > 2) {
		xOiDebugPrint(@Warn, 1, "varcode parsing error!");
		xOiDebugFinishFunc(NULL);
		return(NULL);
	    }
	    if (tPCV.size() == 1) {
		
		tPCV.pushBack("");
	    }
	    
	    var tPC = xOiParseString2List(tPCV[0], '.');
	    if (tPC == NULL || tPC.size() != 2) {
		xOiDebugPrint(@Warn, 1, "varcode parsing error!");
		xOiDebugFinishFunc(NULL);
		return(NULL);
	    }
	    tNewList.pushBack(tPC[1]+"="+tPCV[1]);
	}

	xOiDebugFinishFunc(tNewList);
	return(tNewList);
    }

    
    
    private func checkInsignificantPClasses(pPDB)
    {
	xOiDebugStartFunc2("xOiPDManager::checkInsignificantPClasses", NULL);

	var tPDBId = pPDB.getID();
	xOiDebugPrint(@Info, 1, ["PDB-Id: ", tPDBId]);

	var tRet = 0;

	var tPrograms = pPDB.getPrograms();

	if (tPrograms.size() > 1) {
	    xOiDebugPrint(@Warn, 1,
			  "cannot use table for database " +
			  "containing more than one program!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tPID = tPrograms.front();

	xOiDebugPrint(@Info, 2, ["program: ", tPID]);

	if (mXoiInsignificantPClasses == NULL)
	    mXoiInsignificantPClasses = Hash();
	if (mXoiInsignificantPClasses.hasKey(tPID)) {
	    tRet = mXoiInsignificantPClasses[tPID];
	    xOiDebugPrint(@Info, 1, "using cache");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tTbl = xOiOpenDataTbl("epdfproductdb", tPID, 
				  xOiGetAppProgInfoDBPath(tPID));
	if (tTbl != NULL) {
	    var tRes = 
		tTbl.read1EntryFor(@(["type", "@InsignificantPropClasses"]),
				   @("args", "value"), 0);
	    if (tRes != NULL) {
		var tValue;
		try {
		    tValue = eval(tRes[1]);
		    if (tValue instanceof Int)
			tRet = tValue;
		    else
			xOiDebugPrint(@ExplWarn, 1, 
				      "invalid value for " +
				      "InsignificantPropClasses");
		}
		catch(&e: Error) {
		    xOiDebugPrint(@EvalErr, 1,
				  "could not evaluate value for " +
				  "InsignificantPropClasses: " + e.what);
		}
	    }
	    xOiCloseDataTbl(tTbl);
	    mXoiInsignificantPClasses[tPID] = tRet;
	}
	else
	    xOiDebugPrint(@Warn, 1, 
			  ["could not open epdfproductdb.csv for ", tPID]);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    public func safePropNames(pProgram)
    {
	xOiDebugStartFunc("xOiPDManager::safePropNames", pProgram);

	var tRet = 0;

	if (sSafePropNames == NULL)
	    sSafePropNames = Hash();
	if (sSafePropNames.hasKey(pProgram)) {
	    tRet = sSafePropNames[pProgram];
	    xOiDebugPrint(@Info, 1, "using cache");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tTbl = xOiOpenDataTbl("epdfproductdb", pProgram, 
				  xOiGetAppProgInfoDBPath(pProgram));
	if (tTbl != NULL) {
	    var tRes = 
		tTbl.read1EntryFor(@(["type", "@SafePropertyNames"]),
				   @("args", "value"), 0);
	    if (tRes != NULL) {
		var tValue;
		try {
		    tValue = eval(tRes[1]);
		    if (tValue instanceof Int)
			tRet = tValue;
		    else
			xOiDebugPrint(@ExplWarn, 1, 
				      "invalid value for @SafePropertyNames");
		}
		catch(&e: Error) {
		    xOiDebugPrint(@EvalErr, 1,
				  "could not evaluate value for " +
				  "@SafePropertyNames: " + e.what);
		}
	    }
	    xOiCloseDataTbl(tTbl);
	}
	else
	    xOiDebugPrint(@Warn, 1, 
			  ["could not open epdfproductdb.csv for ", pProgram]);

	sSafePropNames[pProgram] = tRet;

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    private func checkArtFeatures(pOldFeatures, pNewFeatures)
    {
	xOiDebugStartFunc2("xOiPDManager::checkArtFeatures", 
			   [pOldFeatures, pNewFeatures]);

	var tNewHash = xOiList2Hash(pNewFeatures);

	var tRes = 1;
	var tF;
	foreach(tF; pOldFeatures)
	    if (!tNewHash.hasKey(tF[0]) || tF[1] != tNewHash[tF[0]]) {
		tRes = 0;
		break;
	    }
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setupConfiguration(pObj, pBaseArticle, pArticleCode, 
				   pCodeType, pMigration, ...)
    {
	xOiDebugStartFunc("xOiPDManager::setupConfiguration", 
			  [pObj, pBaseArticle, pArticleCode, pCodeType, 
			   pMigration]);

	var tMigration = pMigration[0];
	var tSavedFeatures = NULL; 
	if (pMigration.size() > 1)
	    tSavedFeatures = pMigration[1];

	var tOk = 1;

	if (!pObj.isCat(@IF_Article)) {
	    xOiDebugPrint(@Warn, 1, "no article interface!?");
	    xOiDebugFinishFunc(tOk);
	    return(tOk);
	}

	

	var tValidCodeTypes = @(@VarCode, @OFMLVarCode);

	if (tValidCodeTypes.find(pCodeType) < 0 || 
	    !(pBaseArticle instanceof String) || pBaseArticle.empty() ||
	    !(pArticleCode instanceof String)) {
	    
	    xOiDebugPrint(@Warn, 1, 
			  "invalid code type or no article spec given!");
	    xOiDebugFinishFunc(tOk);
	    return(tOk);
	}

	var tPlanning = getPlanning();
	var tOldArticle = pObj.getArticleSpec();

	

	startTransaction(pObj, pBaseArticle); 

	var tPDB = mTransPDB;

	if (tPDB == NULL || !tPDB.hasMember(@getPropDefs2) ||
	    (pCodeType == @OFMLVarCode && !tPDB.hasMember(@getOFMLVarCode))) {
	    xOiDebugPrint(@Warn, 1, "no or invalid product database!");
	    finishTransaction(pObj);
	    tOk = 0;
	    pObj.setObjState(@OI_UpdateState, @Invalid);
	    xOiDebugPrint(@Info, 1, ["update state: ", @Invalid]);
	    xOiDebugFinishFunc(tOk);
	    return(tOk);
	}

	

	tOk = self.checkPDB4Article(pObj.getProgram(), 
				    NULL, NULL, pBaseArticle);

	if (!tOk) {
	    xOiDebugPrint(@Warn, 1, "invalid article!");
	    pObj.setObjState(@OI_UpdateState, @Invalid);
	    finishTransaction(pObj);
	    xOiDebugFinishFunc(tOk);
	    return(tOk);
	}

	var tPropClasses = tPDB.getArticlePropClasses(pBaseArticle);
	if (tPropClasses == NULL  || tPropClasses.empty()) {
	    xOiDebugPrint(@Warn, 1, "no property classes for article!");
	    
	    
	    
	    
	    
	    finishTransaction(pObj);
	    
	    
	    
	    tOk = 1;
	    pObj.setArticleSpec(pBaseArticle);
	    pObj.setXArticleSpec(pCodeType, pArticleCode);
	    pObj.setObjState(@OI_UpdateState, @Up2Date);
	    xOiDebugPrint(@Info, 1, ["update state: ", @Up2Date]);
	    _report2PlGroupFather(pObj);
	    xOiDebugFinishFunc(tOk);
	    return(tOk);
	}

	var tIgnorePClass = checkInsignificantPClasses(tPDB);

	

	sOutputVarcodeErr = 0;

	var tPropPosOffset = pObj.getExtPropOffset();
	
	var tLanguage = getPlanning().getPDLanguage(pObj);
	var tPropDefs = tPDB.getPropDefs2(pBaseArticle, tPropPosOffset,
					  tLanguage, pArticleCode, pCodeType, 
					  NULL);
	if (tPropDefs == NULL) tPropDefs =  @();

	xOiDebugPrint(@Info, 3, ["new configuration: ", tPropDefs]);

	

	var tPVals = buildPropValList2(pObj, tPropDefs);

	var tNewVarCode     = tPDB.getVarCode(pBaseArticle, tPVals);
	var tNewOFMLVarCode = NULL;
	if (pCodeType == @OFMLVarCode)
	    tNewOFMLVarCode = tPDB.getOFMLVarCode(pBaseArticle, tPVals);

	xOiDebugPrint(@Info, 2, ["new variant code: ", tNewVarCode]);
	xOiDebugPrint(@Info, 2, ["new OFML varcode: ", tNewOFMLVarCode]);

	var tNewArticleCode = tNewVarCode;
	if (pCodeType == @OFMLVarCode)
	    tNewArticleCode = tNewOFMLVarCode;
	var tOldArticleCode = pArticleCode;

	if (pCodeType == @VarCode && tNewArticleCode != tOldArticleCode) {
	    var tAtPos1, tAtPos2;
	    if ((tPDB.isA(xOiEPDFProductDB) ||
		 tPDB.isA(xOiNativeOCDProductDB)) &&
		tPDB.getVarCodeType(pBaseArticle) == @UserDefined &&
		(tAtPos1 = tNewArticleCode.find('@')) > 0 &&
		(tAtPos2 = tOldArticleCode.find('@')) > 0) {
		
		var tNewCode = tNewArticleCode.substr(tAtPos1+1);
		var tOldCode = tOldArticleCode.substr(tAtPos2+1);
		var tOldLen = tOldCode.size();
		xOiDebugPrint(@Info, 2,
			      ["comparing varcodes for user defined scheme: '",
			       tOldCode, "' vs. '", tNewCode, "'"]);
		if (tNewCode == tOldCode ||
		    (tNewCode.size() > tOldLen &&
		     tNewCode.substr(0, tOldLen) == tOldCode))
		    tOldArticleCode = tNewArticleCode;
	    }
	}

	sOutputVarcodeErr = 1;

	xOiDebugPrint(@Info, 2, ["new article code: ", tNewArticleCode]);

	

	var tUpdate = 1;

	if (tNewArticleCode != tOldArticleCode) {
	    xOiDebugPrint(@Info, 1, 
			  ["article code mismatch: '", tNewArticleCode, "'"]);
	    tOk = 0;
	    tUpdate = tMigration;
	    if ((pCodeType == @OFMLVarCode ||
		 ((tPDB.isA(xOiEPDFProductDB) || 
		   tPDB.isA(xOiNativeOCDProductDB)) &&
		  tPDB.getVarCodeType(pBaseArticle) == @Complete)) &&
		self.checkCompleteVarCodes(tNewArticleCode, pArticleCode, 
					   tIgnorePClass)) 
	    {
		tOk = 1;
		tUpdate = 1;
	    }
	    else {
		var tOldLen = tOldArticleCode.size();
		
		if (tNewArticleCode.size() > tOldLen &&
		    tNewArticleCode.substr(0, tOldLen) == tOldArticleCode) {
		    xOiDebugPrint(@Info, 1, "tolerating added properties");
		    tOk = 1;
		    tUpdate = 1;
		}
	    }
	}

	
	
	
	
	finishTransaction(pObj);

	

	if (tUpdate) {
	    
	    
	    
	    if (pBaseArticle != tOldArticle)
	        xOiDebugPrint(@Info, 1, ["assigning new article spec, old: ",
					 tOldArticle]);
	    
	    
	    
	    
	    
	    
	    var tInfo = [tPropDefs, tPDB, pBaseArticle];
	    pObj.getDynamicProps()[@_XOI_SETUP_CONFIGURATION] = tInfo;
	    pObj.setArticleSpec(pBaseArticle);
	    if (pCodeType == @OFMLVarCode &&
		(tPDB.isA(xOiEPDFProductDB) || tPDB.isA(xOiNativeOCDProductDB)))
	        pObj.setXArticleSpec(@OFMLVarCode, tNewOFMLVarCode);
	    else
	        pObj.setXArticleSpec(@VarCode, 
				     pCodeType == @OFMLVarCode ?
				     tNewVarCode : tNewArticleCode);
	    pObj.getDynamicProps().remove(@_XOI_SETUP_CONFIGURATION);
	    pObj.setObjState(@OI_UpdateState, @Up2Date);
	    _report2PlGroupFather(pObj);
	}
	else {
	    pObj.setObjState(@OI_UpdateState, @Migratable);
	    
	    if (sUpdatabilityCheckPropDefs == NULL)
		sUpdatabilityCheckPropDefs = Hash();
	    sUpdatabilityCheckPropDefs[pObj.getName()] = tPropDefs;
	    if (tPDB.hasMember(@getVisiblePropValues)) {
		if (sUpdatabilityCheckVisPropVals == NULL)
		    sUpdatabilityCheckVisPropVals = Hash();
		sUpdatabilityCheckVisPropVals[pObj.getName()] = 
		tPDB.getVisiblePropValues();
	    }
	}

	

	xOiDebugPrint(@Info, 1, ["update state: ", 
				 pObj.getObjState(@OI_UpdateState)]);
	xOiDebugFinishFunc(tOk);
	return(tOk);
    }

    private func _report2PlGroupFather(pObj)
    {
	if (pObj.isA(xOiPlGroup)) return;

	var tF = pObj.getFather();
	while (tF != NULL && !tF.isA(xOiPlGroup))
	    tF = tF.getFather();
	
	if (tF != NULL) tF._xOiSetupConfigurationFinished(pObj);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkUpdatability(pProgram, pRegion, pVersion, pBaseArticle, 
				  pArticleCode, pCodeType, ...)
    {
	xOiDebugStartFunc("xOiPDManager::checkUpdatability", 
			  [pProgram, pRegion, pVersion, pBaseArticle, 
			   pArticleCode, pCodeType]);

	var tCodeType = pCodeType[0];
	var tSavedFeatures = NULL;
	if (pCodeType.size() > 1)
	    tSavedFeatures = pCodeType[1];

	var tRet = @Invalid;

	

	var tValidCodeTypes = @(@VarCode, @OFMLVarCode);

	if (tValidCodeTypes.find(tCodeType) < 0 || 
	    !(pBaseArticle instanceof String) || pBaseArticle.empty() ||
	    !(pArticleCode instanceof String)) {
	    if (pProgram != NULL) {
	        xOiDebugPrint(@Warn, 1, 
			      ["invalid code type or no article spec passed ",
			       "to xOiPDManager::checkUpdatability(): ", 
			       tCodeType, ", '", pBaseArticle, "', '", 
			       pArticleCode, "'"]);
		
	    } else {
		xOiDebugPrint(@Info, 1, "probably a pseudo article");
		tRet = @Updatable;
	    }
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tPlanning = getPlanning();

	

	var tPDB = self.getProgPDB(pProgram);

	if (tPDB == NULL || !tPDB.hasMember(@getPropDefs2) ||
	    (tCodeType == @OFMLVarCode && !tPDB.hasMember(@getOFMLVarCode))) {
	    xOiDebugPrint(@Warn, 1, "no or invalid product database!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	

	var tIsValid = 
	self.checkPDB4Article(pProgram, pRegion, pVersion, pBaseArticle);

	if (!tIsValid) {
	    xOiDebugPrint(@Warn, 1, "invalid article!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tPropClasses = tPDB.getArticlePropClasses(pBaseArticle);
	if (tPropClasses == NULL  || tPropClasses.empty()) {
	    xOiDebugPrint(@Warn, 1, "no property classes for article!");
	    
	    
	    
	    tRet = @Updatable;
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tIgnorePClass = checkInsignificantPClasses(tPDB);

	var tOldPlanPID = tPlanning.getProgram();
	
	
	if (tOldPlanPID != pProgram) tPlanning.setProgram(pProgram);
	var tAClass = article2Class(pBaseArticle);
	if (tOldPlanPID != pProgram) tPlanning.setProgram(tOldPlanPID);

	if (tAClass == NULL) {
	    xOiDebugPrint(@Warn, 1, "no class for article!?");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	mUpdatabilityCheckContext = [pProgram, pRegion, pVersion, tPDB,
				     pBaseArticle, tAClass];

	

	sOutputVarcodeErr = 0;

	var tTransObj;

	if (!self.hasTransaction(NULL)) {
	    
	    tTransObj = mTransObj;
	    mTransObj = NULL;
	    if (mTransArticle != NULL && mTransPDB != NULL)
	        mTransPDB.finishTransaction(mTransArticle);
	    tPDB.startTransaction(pBaseArticle);
	}
	

	var tPropPosOffset = (tPDB.isA(xOiEPDFProductDB) ? NULL : 0);
	
	var tLanguage = tPlanning.getPDLanguage(pProgram);
	var tPropDefs = tPDB.getPropDefs2(pBaseArticle, tPropPosOffset,
					  tLanguage, pArticleCode, tCodeType, 
					  NULL);
	if (tPropDefs == NULL) tPropDefs =  @();

	if (self.hasTransaction(NULL)) {
	    
	    var tTransObj = self.getTransObj();
	    if (sUpdatabilityCheckPropDefs == NULL)
		sUpdatabilityCheckPropDefs = Hash();
	    sUpdatabilityCheckPropDefs[tTransObj.getName()] = tPropDefs;
	    if (tPDB.hasMember(@getVisiblePropValues)) {
		if (sUpdatabilityCheckVisPropVals == NULL)
		    sUpdatabilityCheckVisPropVals = Hash();
		sUpdatabilityCheckVisPropVals[tTransObj.getName()] = 
		tPDB.getVisiblePropValues();
	    }
	}

	

	var tPVals = buildPropValList2(NULL, tPropDefs);

	var tNewVarCode     = tPDB.getVarCode(pBaseArticle, tPVals);
	var tNewOFMLVarCode = NULL;
	if (tCodeType == @OFMLVarCode)
	    tNewOFMLVarCode = tPDB.getOFMLVarCode(pBaseArticle, tPVals);

	xOiDebugPrint(@Info, 2, ["new variant code: ", tNewVarCode]);
	xOiDebugPrint(@Info, 2, ["new OFML varcode: ", tNewOFMLVarCode]);

	var tNewArticleCode = tNewVarCode;
	if (tCodeType == @OFMLVarCode)
	    tNewArticleCode = tNewOFMLVarCode;
	var tOldArticleCode = pArticleCode;

	if (tCodeType == @VarCode && tNewArticleCode != tOldArticleCode) {
	    var tAtPos1, tAtPos2;
	    if ((tPDB.isA(xOiEPDFProductDB) || 
		 tPDB.isA(xOiNativeOCDProductDB)) &&
		tPDB.getVarCodeType(pBaseArticle) == @UserDefined &&
		(tAtPos1 = tNewArticleCode.find('@')) > 0 &&
		(tAtPos2 = tOldArticleCode.find('@')) > 0) {
		
		var tNewCode = tNewArticleCode.substr(tAtPos1+1);
		var tOldCode = tOldArticleCode.substr(tAtPos2+1);
		var tOldLen = tOldCode.size();
		xOiDebugPrint(@Info, 2,
			      ["comparing varcodes for user defined scheme: '",
			       tOldCode, "' vs. '", tNewCode, "'"]);
		if (tNewCode == tOldCode ||
		    (tNewCode.size() > tOldLen &&
		     tNewCode.substr(0, tOldLen) == tOldCode))
		    tOldArticleCode = tNewArticleCode;
	    }
	}

	xOiDebugPrint(@Info, 2, ["new article code: ", tNewArticleCode]);

	sOutputVarcodeErr = 1;

	

	if (tNewArticleCode != tOldArticleCode) {
	    tRet = @Migratable;
	    xOiDebugPrint(@Info, 1, 
			  ["article code mismatch:\n* new: '", tNewArticleCode,
			   "'\n* old: '", tOldArticleCode, "'"]);
	    if ((tCodeType == @OFMLVarCode ||
		 ((tPDB.isA(xOiEPDFProductDB) || 
		   tPDB.isA(xOiNativeOCDProductDB)) &&
		  tPDB.getVarCodeType(pBaseArticle) == @Complete)) &&
		self.checkCompleteVarCodes(tNewArticleCode, pArticleCode, tIgnorePClass)) {
	        tRet = @Updatable;
	    }
	    else {
		var tOldLen = tOldArticleCode.size();
	        
		if (tNewArticleCode.size() > tOldLen &&
		    tNewArticleCode.substr(0, tOldLen) == tOldArticleCode) {
		    xOiDebugPrint(@Info, 1, "tolerating added properties");
		    tRet = @Updatable;
		}
	    }
	}
	else
	    tRet = @Updatable;

	if (!self.hasTransaction(NULL)) {
	    tPDB.finishTransaction(pBaseArticle);
	    mTransObj = tTransObj;
	}

	mUpdatabilityCheckContext = NULL;

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkObjUpdatability(pObj, pCodeType)
    {
	xOiDebugStartFunc("xOiPDManager::checkObjUpdatability", 
			  [pObj, pCodeType]);

	
	if (pObj == NULL || !pObj.hasMember(@isCat) || 
	    !pObj.isCat(@IF_Article)) {
	    xOiDebugPrint(@Warn, 1, "no article instance given!");
	    xOiDebugFinishFunc(@Invalid);
	    return(@Invalid);
	}

	var tObj = pObj.getArticleObj();

	var tPlan = getPlanning();

	startTransaction(tObj, NULL); 

	var tCodeType = pCodeType;
	var tVarCode  = tObj.getXArticleSpec(tCodeType);
	if (tVarCode == NULL && tCodeType == @OFMLVarCode) {
	    
	    
	    xOiDebugPrint(@Warn, 1, "no OFMLVarCode available -> " +
				    "using manufacturer varcode");
	    tCodeType = @VarCode;
	    tVarCode  = tObj.getXArticleSpec(tCodeType);
	}

	var tRet = self.checkUpdatability(tObj.getProgram(), NULL, NULL, 
				          tObj.getArticleSpec(), 
				          tVarCode, tCodeType);
	finishTransaction(tObj);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    private func getInvisibleProps(pProps)
    {
	var tRes = @();
	var tP;
	foreach(tP; pProps) {
	    var tPClass = tP[0];
	    var tPName  = tP[1];
	    var tState  = tP[4];
	    if (tState == -1) tRes.pushBack(tPClass+"::"+tPName);
	}
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func isArticleValid(pObj)
    {
	xOiDebugStartFunc("xOiPDManager::isArticleValid", pObj);

	var tArticle, tPDB, tPID;

	if (pObj instanceof Vector) {
	    tArticle = pObj[0];
	    tPID     = pObj[1];
	    tPDB     = pObj[2];
	}
	else {
	    tArticle = pObj.getArticleSpec();
	    tPID     = pObj.getProgram();
	    tPDB     = getPDBFor(pObj);
	}

	if (tPDB == NULL || tArticle == NULL || tArticle.empty()) {
	    if (tArticle == NULL || tArticle.empty())
		xOiDebugPrint(@Warn, 1, "no article!");
	    if (tPDB == NULL)
		xOiDebugPrint(@Warn, 1, "no product database!");
	    xOiDebugFinishFunc(1);
	    return(1);
	}

	var tOk = _isArticleValid(tArticle, tPID, tPDB);

	xOiDebugFinishFunc(tOk);
	return(tOk);
    }

    
    
    private func _isArticleValid(pArticle, pPID, pPDB)
    {
	xOiDebugStartFunc2("xOiPDManager::_isArticleValid",
			   [pArticle, pPID, pPDB]);

	var tOk = 0;

	var tMID  = xOiGetAppManufacturerID(pPID);
	var tSIDs = xOiParseString2List(xOiGetAppSeriesID(pPID), ';');

	xOiDebugPrint(@Info, 2, ["manufacturer: ", tMID]);
	xOiDebugPrint(@Info, 2, ["series: ", tSIDs]);

	if (tMID == NULL || tSIDs == NULL) {
	    if (tMID == NULL)
		xOiDebugPrint(@Warn, 1,
			      ["no manufacturer ID for program ", pPID]);
	    if (tSIDs == NULL)
		xOiDebugPrint(@Warn, 1,
			      ["no series ID for program ", pPID]);
	}
	else {
	    var tSID;
	    foreach(tSID; tSIDs)
		if (pPDB.containsArticle(tMID, tSID, pArticle)) {
		    tOk = 1;
		    break;
		}
	}
	xOiDebugFinishFunc(tOk);
	return(tOk);
    }

    private func _setupProps(pObj, pArticle, pAddSpec, pAddSpecType)
    {
	xOiDebugStartFunc("xOiPDManager::_setupProps",
			  [pObj, pArticle, pAddSpec, pAddSpecType]);

	var tPlanning = getPlanning();
	var tNeedConfig = NULL;
	if (tPlanning.isA(xOiPlanning))
	    tNeedConfig = tPlanning.getAppMode(@NeedConfiguration);

	xOiDebugPrint(@Info, 2, ["@NeedConfiguration: ", tNeedConfig]);

	if (tNeedConfig != NULL && !(tNeedConfig & 1)) {
	    xOiDebugPrint(@Info, 1, "no configuration required by application");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tPDB, tArticle;

	var tTransaction = startTransaction(pObj, pArticle);

	if (tTransaction) {
	    tArticle = mTransArticle;
	    tPDB     = mTransPDB;
	}
	else {
	    tArticle = pObj.getArticleSpec();
	    tPDB = getProgPDB(pObj.getProgram());
	}

	if (tPDB == NULL || tArticle == NULL) {
	    var tMsg = " (no product database)";
	    if (tArticle == NULL)
		tMsg = " (no article)";
	    if (tTransaction) finishTransaction(pObj);
	    xOiDebugFinishFunc(tMsg);
	    return(NULL);
	}

	xOiDebugPrint(@Info, 2, ["article to configure: ", tArticle]);
	xOiDebugPrint(@Info, 2, ["temporary article was: ", sTmpChildArticle]);

	var tIsTmpChild = 0;
	var tFather = pObj.getFather();
	var tObj = pObj;

	if ((tFather.isCat(@MetaType) || tFather.isCat(@IF_MetaType)) && 
	    tFather.hasMember(@getMainChild) &&
	    tFather.getMainChild() == pObj) {
	    tObj = tFather;
	    tFather = tObj.getFather();
	}

	xOiDebugPrint(@Info, 2, ["current father ", tFather]);
	xOiDebugPrint(@Info, 2, ["temporary father ", sTmpChildFather]);
	xOiDebugPrint(@Info, 2, ["temporary addspec: ", sTmpChildAddSpec]);

	var tAddSpecV = NULL;
	if (pAddSpec != NULL && pAddSpecType != NULL)
	    tAddSpecV = [pAddSpec, pAddSpecType];

	var tUseTmpChildProps = 0;

	if (tFather.isCat(@IF_Complex) && tFather.objIsTmpCh(tObj))
	    tIsTmpChild = 1;

	xOiDebugPrint(@Info, 2, ["is temporary object? ", tIsTmpChild]);

	var tPropDefs = NULL;
	var tPVals = NULL;
	var tVarCode = NULL;
	var tFinalSpec = NULL;
	var tOFMLVC = NULL;

	if (sTmpChildProps != NULL && sTmpChildFather == tFather &&
	    sTmpChildArticle == tArticle && 
	    sTmpChildAddSpec == tAddSpecV) {
	    sPropValList = NULL;
	    tPVals = buildPropValList(pObj, tPDB, tArticle);
	    if (!(sTmpChildPVals.empty() && !tPVals.empty())) {
	        xOiDebugPrint(@Info, 1, "using prop defs of temporary object");
	        tUseTmpChildProps = 1;
	        tPropDefs  = sTmpChildProps;
		tVarCode   = sTmpChildVarCode;
		tFinalSpec = sTmpChildFinalSpec;
		tOFMLVC    = sTmpChildOFMLVC;
	    }
	}
	else
	
	
	
	if (sTmpAggrObjName != NULL && sAggrObj != NULL &&
	    sAggrObj.getName() != sTmpAggrObjName &&
	    xOiGetNextAggregateUp(pObj.getFather()) == sAggrObj) {
	    var tOldObjName = xOiSubstString(pObj.getName(),
					     sAggrObj.getName(),
					     sTmpAggrObjName);
	    xOiDebugPrint(@Info, 2, ["old aggr child name: ", tOldObjName]);
	    var tHKey = tOldObjName + tArticle;
	    if (sTmpAggrChildProps.hasKey(tHKey)) {
	        xOiDebugPrint(@Info, 1,
			      "using prop defs of temporary aggregate child");
	        tPropDefs  = sTmpAggrChildProps[tHKey][0];
	        tVarCode   = sTmpAggrChildProps[tHKey][1];
	        tFinalSpec = sTmpAggrChildProps[tHKey][2];
	        tOFMLVC    = sTmpAggrChildProps[tHKey][3];
	    }
	}
	else
	if (sTmpAggrObjName != NULL && sAggrObj != NULL &&
	    sAggrObj.getName() == sTmpAggrObjName &&
	    xOiGetNextAggregateUp(pObj.getFather()) == sAggrObj)
	    
	    
	    
	    sPropValList = NULL;

	if (tPropDefs == NULL) {
	    if (tPVals == NULL)
		tPVals = buildPropValList(pObj, tPDB, tArticle);
	    
	    var tLanguage = tPlanning.getPDLanguage(pObj);
	    if (pAddSpec == NULL) {
	        tPropDefs = tPDB.getPropDefs(tArticle, pObj.getExtPropOffset(),
				             tLanguage, NULL,
					     tPVals.empty() ? NULL : tPVals);
	    }
	    else
	    if (tPDB.hasMember(@getPropDefs2))
	        tPropDefs = tPDB.getPropDefs2(tArticle, pObj.getExtPropOffset(),
				              tLanguage, pAddSpec, pAddSpecType,
					      tPVals.empty() ? NULL : tPVals);
	    else
	        xOiDebugPrint(@ExplWarn, 1,
			      ["product database ", tPDB,
			       " does not implement required method ",
			       "getPropDefs2()!"]);
	}
	else
	if (tPropDefs instanceof Int && tPropDefs == -1)
	    tPropDefs = NULL;

	var tResetTmpChildProps = 0;

	if (tIsTmpChild) {
	    sTmpChildArticle = tArticle;
	    sTmpChildPVals = tPVals;
	    sTmpChildProps = tPropDefs;
	    if (tPropDefs == NULL)
		sTmpChildProps = @();
	    else
	        xOiDebugPrint(@Info, 1,
			      ["caching prop defs for temporary child: ", pObj,
			       " with base article spec ", tArticle, 
			       " and additional spec: ", pAddSpec, 
			       " (", pAddSpecType, ")"]);
	    sTmpChildFather = tFather;
	    if (pAddSpec != NULL) sTmpChildAddSpec = [pAddSpec, pAddSpecType];
	    if (sTmpChildChfPValTexts == NULL) sTmpChildChfPValTexts = Hash();
	}
	else
	if ((sAggrObj == NULL && sTmpAggrObjName == NULL) &&
	    (sTmpChildAddSpec == NULL || pAddSpec != NULL))
	    tResetTmpChildProps = 1;

	var tChangedProps = NULL;

	if (tPropDefs != NULL && !tPropDefs.empty()) {
	    tChangedProps = updatePropList(pObj, tPropDefs, tPDB, tArticle,
					   NULL, 0, tIsTmpChild,
					   tUseTmpChildProps);
	    tPVals = buildPropValList(pObj, tPDB, tArticle, 1);
	    
	    sTransVarCode = NULL;
	    sTransOFMLVarCode = NULL;
	    sTransFinalType = NULL;
	    if (tVarCode == NULL)
		tVarCode = getXArticleSpec(pObj, @VarCode);
	    else
		sTransVarCode = tVarCode;
	    if (tFinalSpec == NULL)
		tFinalSpec = getXArticleSpec(pObj, @Final);
	    else
		sTransFinalType = tFinalSpec;
	    if (tOFMLVC == NULL)
		tOFMLVC = getXArticleSpec(pObj, @OFMLVarCode);
	    else
		sTransOFMLVarCode = tOFMLVC;
	}

	if (tIsTmpChild) {
	    sTmpChildVarCode = tVarCode;
	    sTmpChildFinalSpec = tFinalSpec;
	    sTmpChildOFMLVC = tOFMLVC;
	}

	var tAggregate;

	if (sTmpAggrObjName != NULL && 
	    (tAggregate = xOiGetNextAggregateUp(pObj.getFather())) != NULL &&
	    tAggregate.getName() == sTmpAggrObjName) {
	    var tObjName = pObj.getName();
	    xOiDebugPrint(@Info, 1,
			  ["caching prop defs for aggr child: ", tObjName,
			   " with base article spec ", tArticle]);
	    sTmpAggrChildProps[tObjName+tArticle] = [tPropDefs, tVarCode, 
						     tFinalSpec, tOFMLVC];
	}

	if (tResetTmpChildProps) {
	    sTmpChildArticle = NULL;
	    sTmpChildPVals = NULL;
	    sTmpChildProps = NULL;
	    sTmpChildFather = NULL;
	    sTmpChildAddSpec = NULL;
	    sTmpChildChfPValTexts = NULL;
	    sTmpChildVisPropVals = NULL;
	    sTmpChildVarCode = NULL;
	    sTmpChildFinalSpec = NULL;
	    sTmpChildOFMLVC = NULL;
	}

	if (pAddSpec == NULL && tPVals == NULL) {
	    
	    
	    if (!tIsTmpChild)
		pObj.getDynamicProps()[@__XOIPDM_STATE] = @Default;
	}
	else
	    pObj.getDynamicProps()[@__XOIPDM_STATE] = @Changed;

	if (isPDCacheEnabled()) {
	    var tCheckSum = checkSum4PDCache(tPVals);
	    var tFAN      = finalType4PDCache(tPDB, pObj, NULL);
	    xOiDebugPrint(@Info, 2, ["storing final type info for PD cache: ",
				     [tCheckSum, tFAN]]);
	    pObj.getDynamicProps()[sFinalType4PDCachePropKey] = 
							[tCheckSum, tFAN];
	}

	if (tTransaction) finishTransaction(pObj);

	xOiDebugFinishFunc(tChangedProps);
	return(tChangedProps);
    }

    
    
    
    public func evalPropValue(pObj, pPKey, pValue, pOldValue, pOldArticle)
    {
	xOiDebugStartFunc("xOiPDManager::evalPropValue",
			   [pObj, pPKey, pValue, pOldValue, pOldArticle]);

	var tPlanning = getPlanning();
	var tNeedConfig = NULL;
	if (tPlanning.isA(xOiPlanning))
	    tNeedConfig = tPlanning.getAppMode(@NeedConfiguration);

	xOiDebugPrint(@Info, 2, ["@NeedConfiguration: ", tNeedConfig]);

	if (tNeedConfig != NULL && !(tNeedConfig & 1)) {
	    xOiDebugPrint(@Info, 1, "no configuration required by application");
	    xOiDebugFinishFunc(1);
	    return(1);
	}

	if (pObj.isA(xOiGroup) || pObj.isA(xOiFreePlElement)) {
	    xOiDebugFinishFunc(1);
	    return(1);
	}

	var tPDB = getPDBFor(pObj);
	if (tPDB == NULL) {
	    xOiDebugFinishFunc("1 (no product database)");
	    return(1);
	}

	if (!tPDB.hasProductKnowledge()) {
	    xOiDebugFinishFunc("1 (no product knowledge)");
	    return(1);
	}

	var tArticle = pObj.getArticleSpec();
	xOiDebugPrint(@Info, 2, ["new article: ", tArticle]);

	if (tArticle == NULL && pOldArticle == NULL) {
	    xOiDebugPrint(@Info, 1, "ignore non PD relevant object");
	    xOiDebugFinishFunc(1);
	    return(1);
	}

	if (tArticle == NULL)
	    tArticle = checkArticleParam(pObj, pPKey);

	xOiDebugPrint(@Info, 2, ["param modified article ", tArticle]);

	if (tArticle == NULL) {
	    xOiDebugFinishFunc("0 (no article)");
	    return(0);
	}

	startTransaction(pObj, tArticle); 
	tPDB = mTransPDB;
	tArticle = mTransArticle;

	var tPName = propKey2PropName(pObj, pPKey);
	var tChangedProp = [tPName, pValue, pOldValue];

	xOiDebugPrint(@Info, 2, ["property ", tPName]);

	if (tPName == NULL) {
	    
	    
	    
	    if (tArticle == pOldArticle) {
	        finishTransaction(pObj);
	    	xOiDebugPrint(@Info, 1, "ignore non PD relevant property");
	    	xOiDebugFinishFunc(1);
	    	return(1);
	    }
	    tChangedProp = NULL;
	}

	var tPropDefs = NULL;

	try {
	    var tPropValList = buildPropValList(pObj, tPDB, pOldArticle, 1,
						(tArticle != pOldArticle));
	    
	    
	    
	    buildTransPropMaps(pObj, tPDB, tArticle);

	    
	    var tLanguage = tPlanning.getPDLanguage(pObj);
	    tPropDefs = tPDB.getPropDefs(tArticle, pObj.getExtPropOffset(),
				    	 tLanguage, tChangedProp, tPropValList);
	    
	    

	    if (tPropDefs != NULL) {
	        
	        sTransVarCode = NULL;
	        sTransOFMLVarCode = NULL;
	        sTransFinalType = NULL;
		updatePropList(pObj, tPropDefs, tPDB, tArticle,
			       tArticle != pOldArticle ? pOldArticle : NULL,
			       0, 0, 0);
	        tPropValList = buildPropValList(pObj, tPDB, tArticle, 1);
		var tChSum = checkSum4PDCache(tPropValList);
		var tFAN   = finalType4PDCache(tPDB, pObj, NULL);
		xOiDebugPrint(@Info, 2,
			      ["storing final type info for PD cache: ",
			       [tChSum, tFAN]]);
		pObj.getDynamicProps()[sFinalType4PDCachePropKey] =
								[tChSum, tFAN];
		pObj.getDynamicProps()[@__XOIPDM_STATE] = @Changed;
	    }
	    else
		
		sPropValList = NULL;
		
	}
	catch (&e: Error) {
	    xOiDebugPrint(@EvalErr, 1,
		          "Evaluation error in " + e.where+": "+e.what);
	}

	finishTransaction(pObj);

	var tOk = (tPropDefs != NULL);

	xOiDebugFinishFunc(tOk);
	return(tOk);
    }

    
    
    
    public func startGlobalConsistencyCheck()
    {
	sConsistencyCheckCache = Hash();
    }

    
    
    
    public func finishGlobalConsistencyCheck()
    {
	sConsistencyCheckCache = NULL;
    }

    
    
    
    
    
    
    
    
    public func checkConsistency(pObj, pLanguage, pReport)
    {
	xOiDebugStartFunc("xOiPDManager::checkConsistency",
			  [pObj, pLanguage]);

	xOiDebugPrint(@Info, 3, ["current report log: ", pReport]);

	var tRes = 1;
	var tLogKey = pObj.getOrderID();
	if (tLogKey == NULL)
	    tLogKey = pObj.getName();

	if (!pReport.hasKey(tLogKey)) pReport[tLogKey] = @();

	var tErrors = pReport[tLogKey];  
	var tErrCnt = tErrors.size();    

	var tErrors2;		         
					 

	startTransaction(pObj, NULL); 

	var tPriceDate = pObj.getPriceDate();

	var tCacheKey = NULL;

	
	if (mTransArticle != NULL) {
	    var tVarCode = self.getXArticleSpec(pObj, @VarCode);
	    tCacheKey = mTransArticle + 
			(tVarCode == NULL ? "NULL" : tVarCode) + 
			(tPriceDate == NULL ? "NULL" : tPriceDate);
	    if (sConsistencyCheckCache != NULL && tCacheKey != NULL &&
		sConsistencyCheckCache.hasKey(tCacheKey)) {
		xOiDebugPrint(@Info, 1, "using cache");
		tErrors2 = sConsistencyCheckCache[tCacheKey];
		tRes = (tErrors2.size() == 0);
		if (!tRes) xOiCopyAggr(tErrors2, tErrors, 1);
		finishTransaction(pObj);
		xOiDebugFinishFunc(tRes);
		return(tRes);
	    }
	}

	var tPlan = getPlanning();
	var tLanguage = NULL;
	if (tPlan != NULL && tPlan.isA(xOiPlanning))
	    tLanguage = tPlan.getPDLanguage(pObj);
	if (tLanguage == NULL)
	    tLanguage = pLanguage;
	else
	if (tLanguage != pLanguage)
	    xOiDebugPrint(@Info, 1, ["using language '", tLanguage, 
				     "' instead of passed '", pLanguage, "'"]);

	
	tErrors2 = @();
	if (mTransPDB != NULL && mTransArticle != NULL) {
	    var tPropValList = buildPropValList(pObj, mTransPDB, mTransArticle);

	    if (mTransPDB.isA(xOiNativeOCDProductDB) &&
		mTransPDB.implemented(@checkConsistency2))
		tRes = mTransPDB.checkConsistency2(mTransArticle, tPropValList,
					           tLanguage, tPriceDate, 
						   tErrors);
	    else
		tRes = mTransPDB.checkConsistency(mTransArticle, tPropValList,
					          tLanguage, tErrors);
	    if (sConsistencyCheckCache != NULL && tCacheKey != NULL) {
		if (!tRes) {
		    
		    var i;
		    for(i=tErrCnt; i < tErrors.size(); i++) {
			var tErr = tErrors[i];
			tErrors2.pushBack(tErr);
		    }
		}
	    }
	}

	
	
	
	getArticlePrice(pObj, pLanguage, tPlan.getPrimaryCurrency());

	if (pObj.getDynamicProps().hasKey(@_XOI_INVALID_PRICE_DATE)) {
	    xOiDebugPrint(@Info, 3, ["checking XOI_INVALID_PRICE_DATE",
				     " for ", pObj, ": 1"]);
	    tRes = 0;
	    var tErr = ["::ofml::xoi::@InvalidPrice", getName(),
			"xOiPDManager::checkConsistency"];
	    tErrors.pushBack(tErr);
	    tErrors2.pushBack(tErr);
		    
	} 
	else 
	    xOiDebugPrint(@Info, 3, ["checking XOI_INVALID_PRICE_DATE",
				     " for ", pObj, ": 0"]);

	if (sConsistencyCheckCache != NULL && tCacheKey != NULL)
	    sConsistencyCheckCache[tCacheKey] = tErrors2;

	xOiDebugPrint(@Info, 2, ["log messages: ", tErrors]);

	finishTransaction(pObj);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    public func checkArticleParam(pObj, pPKey)
    {
	pObj.propsChanged(@(pPKey), 0);
	return(pObj.getArticleSpec());
    }

    
    
    
    private func getValues4Prop(pPValues, pCValues)
    {
	var tVL = Vector(0);
	var tV;
	foreach(tV; pPValues)
	    if (pCValues.find(tV[0]) >= 0) tVL.pushBack(tV);
	return(tVL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    private func updatePropList(pObj, pPropDefs, pPDB, pArticle, pOldArticle,
				pNoValueChange, pIsTmpChild, pUseTmpChildProps)
    {
	xOiDebugStartFunc("xOiPDManager::updatePropList",
			  [pObj, pPropDefs, pPDB, pArticle, pOldArticle,
			   pNoValueChange, pIsTmpChild, pUseTmpChildProps]);

	var tNewPDPropKeys = Vector(0);
	var tChangedProps  = @();
	var tChangedPropVals = @(); 

	var tPlan = getPlanning();

	
	
	var tPropDefs = @();
	xOiCopyAggr(pPropDefs, tPropDefs, 1);
	var tPositions = Hash();
	var tPD;
	foreach(tPD; pPropDefs) {
	    var tPClass = tPD[0];
	    var tPName  = tPD[1];
	    var tPPos   = tPD[5];
	    tPositions[tPClass+tPName] = tPPos;
	}
	if (!tPropDefs.empty())
	    tPropDefs.sort(PropListSort(tPositions));

	xOiDebugPrint(@Info, 2, ["sorted prop list: ", tPropDefs]);

	foreach(tPD; tPropDefs) {
	    var tPClass = tPD[0];
	    var tPName  = tPD[1];
	    var tPDef   = tPD[2];
	    var tCVal   = tPD[3];
	    var tCValues= tPD[4];
	    var tPPos   = tPD[5];
	    var tState  = tPD[6];

	    var tPKeyType = propName2PropKey(pObj, tPClass, tPName);
	    var tPKey  = tPKeyType[0];
	    var tPKSpT = tPKeyType[1];	

	    if (tPKey == NULL) continue;

	    if (tPDef == NULL && tState == -1) {
		
		xOiDebugPrint(@Info, 2, ["handling invalid property ", tPKey]);
		var tTypeMismatch = 0;
		var tOldVal = NULL;
		if (pObj.hasProperty(tPKey))
		    tOldVal = pObj.getPropValue(tPKey);
		xOiDebugPrint(@Info, 2, ["old value: ", tOldVal]);
		var tPType = "s";
		if (tCVal instanceof String) {
		    if (tOldVal != NULL &&
			!(tOldVal instanceof String)) tTypeMismatch = 1;
		}
		else
		if (tCVal instanceof Int) {
		    tPType = "i";
		    if (tOldVal != NULL &&
			!(tOldVal instanceof Int)) tTypeMismatch = 1;
		}
		else
		if (tCVal instanceof Float) {
		    tPType = "f";
		    if (tOldVal != NULL &&
			!(tOldVal instanceof Float)) tTypeMismatch = 1;
		}
		if (tTypeMismatch)
		    xOiDebugPrint(@Warn, 2,
				  ["type mismatch ", tOldVal, " vs. ", tCVal]);
    	    	initProp(pObj, pArticle, pPDB, tPName, tPKey,
			 [tPName, NULL, NULL, NULL, tPType],
			 tPPos, tCVal, tState, 1, pUseTmpChildProps);
		pObj.setPropClass(tPKey, tPClass);
		if (tTypeMismatch || tOldVal != tCVal)
		    tChangedProps.pushBack(tPKey);
		continue;
	    }

	    if (tPDef == NULL) {
		
		
		
		if (pObj.hasProperty(tPKey)) {
		    if (pNoValueChange) {
		        xOiDebugPrint(@Warn, 1, ["invalid property ", tPKey]);
			xOiDebugFinishFunc(NULL);
			return(NULL);
		    }
		    xOiDebugPrint(@Info, 2, ["removing property ", tPKey,
				             ", setting value to ", tCVal]);
		    setPropValue(pObj, tPKey, tCVal);
		    pObj.delPropClass(tPKey);
		    pObj.removeProperty(tPKey);
		    tChangedProps.pushBack(tPKey);
	    	}
		continue;
	    }

	    tNewPDPropKeys.pushBack(tPKey);

	    

	    if (!pObj.hasProperty(tPKey)) {
		
    	    	initProp(pObj, pArticle, pPDB, tPName, tPKey,
			 buildPropDef(tPDef, tCValues, tPKSpT, pObj),
			 tPPos, tCVal, tState, 1, pUseTmpChildProps);
		pObj.setPropClass(tPKey, tPClass);
		tChangedProps.pushBack(tPKey);
		continue;
	    }

	    

	    

	    var tOPClass = pObj.getPropClass(tPKey);
	    if (tOPClass != tPClass) {
		xOiDebugPrint(@Info, 1,
			      ["changing prop class for ", tPKey, " from ",
			       tOPClass, " to ", tPClass]);
		pObj.setPropClass(tPKey, tPClass);
	    }

	    

	    tPDef = buildPropDef(tPDef, tCValues, tPKSpT, pObj);
	    var tCPDef  = pObj.getPropertyDef(tPKey); 
	    var tCPos   = pObj.getPropertyPos(tPKey); 
	    var tCState = pObj.getPropState(tPKey);   

	    xOiDebugPrint(@Info, 2, ["current state: ", tCState]);
	    xOiDebugPrint(@Info, 2, ["current pos: ", tCPos]);

	    if (tCPDef[0] != tPDef[0] ||	
		tCPDef[1] != tPDef[1] ||	
		tCPDef[2] != tPDef[2] ||	
		tCPDef[4] != tPDef[4] ||	
		tCState != tState || tCPos != tPPos) {
		xOiDebugPrint(@Info, 2,
			      ["changed definition for property ", tPKey]);
		pObj.removeProperty(tPKey);
    	    	initProp(pObj, pArticle, pPDB, tPName, tPKey, tPDef,
			 tPPos, tCVal, tState, !pNoValueChange,
			 pUseTmpChildProps);
		pObj.setPropClass(tPKey, tPClass);
		tChangedProps.pushBack(tPKey);
		if (!pNoValueChange) {
		    continue;
		}
	    }

	    var tCPVal = pObj.getPropValue(tPKey); 

	    xOiDebugPrint(@Info, 2, ["new value after evaluation is ", tCVal,
			             " vs. current property value ", tCPVal]);

	    

	    if (tCValues != NULL) {
	    	var tPType = tCPDef[4];
	    	var tCPValues = xOiGetValuesFromChoiceList(tPType);
	        var tCVL = @();
	    	var tCV;
	    	foreach(tCV; tCValues) tCVL.pushBack(tCV[0]);

		xOiDebugPrint(@Info, 2, ["new values: ", tCVL]);

	    	if (xOiCompareAggr(tCPValues, tCVL, 1) != 1) {
		    
		    xOiDebugPrint(@Info, 2,
			          ["changed choice list for property ", tPKey]);
		    pObj.removeProperty(tPKey);
		    var tSetValue = !pNoValueChange;
		    
		    
		    
		    
		    
    	    	    initProp(pObj, pArticle, pPDB, tPName, tPKey, tPDef,
			     tPPos, tCVal, tState, tSetValue,
			     pUseTmpChildProps);
		    pObj.setPropClass(tPKey, tPClass);
		    tChangedProps.pushBack(tPKey);
		    if (tSetValue) {
			continue;
		    }
		}
	    }

	    

	    var tNoValueChange = pNoValueChange;

	    if (!xOiCompareObjs(tCVal, tCPVal)) {
		xOiDebugPrint(@Info, 2, 
			      ["value mismatch: ", tCVal, " vs. ", tCPVal]);

		if (!tNoValueChange || (tCValues == NULL && tState == 0)) {
		    
		    
		    setPropValue(pObj, tPKey, tCVal);
		    tChangedPropVals.pushBack(tPKey);
		}
		tChangedProps.pushBack(tPKey);
	    }
	}

	if (pNoValueChange && !tChangedPropVals.empty()) {
	    
	    tChangedPropVals.sort();
	    tChangedPropVals.unique();
	    pObj.propsChanged(tChangedPropVals, 0);
	}

	xOiDebugPrint(@Info, 2, ["new PD keys: ", tNewPDPropKeys]);

	if (pOldArticle != NULL) {
	    
	    
	    
	    
	    
	    
	    var tAllPDProperties = readClassProperties(pObj);
	    var tOldPropClasses = pPDB.getArticlePropClasses(pOldArticle);

	    xOiDebugPrint(@Info, 2, ["old prop classes: ", tOldPropClasses]);

	    if (tAllPDProperties == NULL) tAllPDProperties = @();

	    var tP;
	    foreach(tP; tAllPDProperties) {
		var tPClass = tP[0];
		var tPKey   = Symbol(tP[2]);

		xOiDebugPrint(@Info, 3,
			      ["check class ", tPClass, " and key ", tPKey]);
		if (tOldPropClasses.find(tPClass) >= 0 && 
		    pObj.hasProperty(tPKey) && tNewPDPropKeys.find(tPKey) < 0) {
		    xOiDebugPrint(@Info, 2, ["remove property ", tPKey]);
		    pObj.delPropClass(tPKey);
		    pObj.removeProperty(tPKey);
		    tChangedProps.pushBack(tPKey);
		}
	    }
	}

	tChangedProps.sort();
	tChangedProps.unique();

	if (!tChangedProps.empty()) sPropValList = NULL;

	if (pPDB.hasMember(@getVisiblePropValues))
	    assignVisiblePropValues(pObj, pPDB, pIsTmpChild, pUseTmpChildProps);

	xOiDebugFinishFunc(tChangedProps);
	return(tChangedProps);
    }

    private func assignVisiblePropValues(pObj, pPDB, pIsTmpChild, 
					 pUseTmpChildProps)
    {
	xOiDebugStartFunc2("xOiPDManager::assignVisiblePropValues",
			   [pObj, pIsTmpChild, pUseTmpChildProps]);

	var tObjName = pObj.getName();
	var tVisPropVals;

	if (sUpdatabilityCheckVisPropVals != NULL &&
	    sUpdatabilityCheckVisPropVals.hasKey(tObjName)) {
	    xOiDebugPrint(@Info, 1, 
			  "using info from previous checkUpdatability()");
	    tVisPropVals = sUpdatabilityCheckVisPropVals[tObjName];
	    sUpdatabilityCheckVisPropVals.remove(tObjName);
	}
	else
	if (!pUseTmpChildProps) {
	    xOiDebugPrint(@Info, 2, "calling PDB");
	    tVisPropVals = pPDB.getVisiblePropValues();
	    if (pIsTmpChild)
		sTmpChildVisPropVals = tVisPropVals;
	}
	else {
	    xOiDebugPrint(@Info, 2, "using sTmpChildVisPropVals");
	    tVisPropVals = sTmpChildVisPropVals;
	}
	if (tVisPropVals == NULL || tVisPropVals.empty()) {
	    xOiDebugPrint(@Info, 1, "no visible values info");
	    pObj.setVisiblePropValues(NULL);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tPDInfo = pObj.getPDInfo("en");
	if (tPDInfo == NULL) {
	    xOiDebugPrint(@Warn, 1,
			  ["getPDInfo() for ", pObj, " returned NULL!"]);
	    pObj.setVisiblePropValues(NULL);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tMID = tPDInfo[0];
	var tSID = tPDInfo[1];
	var tBaseArtNr = pObj.getArticleSpec();

	var tFinalArtNr, tVarCode;

	if (!pUseTmpChildProps) {
	    tFinalArtNr = pObj.getXArticleSpec(@Final);
	    tVarCode    = pObj.getXArticleSpec(@VarCode);
	}
	else {
	    tFinalArtNr = sTmpChildFinalSpec;
	    tVarCode    = sTmpChildVarCode;
	}

	
	var tLanguage = getPlanning().getPDLanguage(pObj);

	var tVisPropVals2 = @();
	var tPD;
	foreach(tPD; tVisPropVals) {
	    var tPClass = tPD[0];
	    var tPName  = tPD[1];
	    var tValues = tPD[2];

	    var tPKey = propName2PropKey(pObj, tPClass, tPName)[0];
	    if (tPKey == NULL) {
		
		xOiDebugPrint(@Warn, 1, ["no property associated with ",
					 tPClass, ".", tPName, "?!"]);
		continue;
	    }

	    
	    var tValues2 = @();
	    var tV;
	    foreach(tV; tValues) {
		var tVal       = tV[0];
		var tTxt       = tV[1];
		var tVisibilty = tV[2];
		var tPrice     = tV[3];
		var tCurrency  = tV[4];

		if (tTxt == "@VOID")
		    tTxt = oiGetStringResource("@VOID", tLanguage, pObj);

		if (tPrice != NULL && tCurrency != NULL) {
		    var tRes = 
		    xOiConvert2AppCurrency(tMID, tSID, tBaseArtNr, tFinalArtNr, 
				           tVarCode, tCurrency, tPrice, tPrice, 
				           @Extra);
		    if (tRes != NULL) {
		        tCurrency = tRes[0];
		        tPrice    = tRes[2]; 
		    }
		}
		tValues2.pushBack([tVal, tTxt, tVisibilty, tPrice, tCurrency]);
	    }
	    tVisPropVals2.pushBack([tPKey, tValues2]);
	    if (tValues2.size() > 1)
		
		pObj.setPropState(tPKey, 1);
	}

	xOiDebugPrint(@Info, 2, 
		      ["assigning visible values info: ", tVisPropVals2]);

	pObj.setVisiblePropValues(tVisPropVals2);

	xOiDebugFinishFunc(NULL);
    }

    func buildPropDef(pPDef, pPValues, pPKSpT, pObj)
    {
	var tType;
	if (pPKSpT != NULL && !pPKSpT.empty())
	    tType = "u " + pPKSpT;
	else
	    tType = pPDef[4];

	var tPDef = [pPDef[0], pPDef[1], pPDef[2], pPDef[3],
		     getPropType(tType, pPValues, pObj)];

	return(tPDef);
    }

    func initProp(pObj, pArticle, pPDB, pPName, pPKey, pPDef,
		  pPPos, pPVal, pState, pSetVal, pUseTmpChildChfPValTexts)
    {
	xOiDebugStartFunc2("xOiPDManager::initProp",
			   [pObj, pArticle, pPDB, pPName, pPKey, pPDef, pPPos,
			    pPVal, pState, pSetVal, pUseTmpChildChfPValTexts]);

	pObj.setupProperty(pPKey, pPDef, pPPos);
	if (pSetVal) setPropValue(pObj, pPKey, pPVal);
	pObj.setPropState(pPKey, pState);

	xOiDebugFinishFunc(NULL);
    }

    
    func setPropValue(pObj, pPKey, pValue)
    {
	if (xOiCallPropSetMethod(pObj, pPKey, pValue)) {
	    pObj.changedPropList().pushBack(pPKey);
	}
    }

    
    
    

    
    
    
    
    
    private func finalType4PDCache(pPDB, pObj, pValueList)
    {
	xOiDebugStartFunc2("xOiPDManager::finalType4PDCache", pObj);

	xOiDebugPrint(@Info, 2, ["PDB ", pPDB]);

	if (pValueList != NULL && !(pObj instanceof Vector) &&
	    pObj.getDynamicProps().hasKey(sFinalType4PDCachePropKey)) {
	    var tData = pObj.getDynamicProps()[sFinalType4PDCachePropKey];
	    var tCheckSum1 = tData[0];
	    var tCheckSum2 = checkSum4PDCache(pValueList);
	    xOiDebugPrint(@Info, 2, ["stored final type: ", tData]);
	    xOiDebugPrint(@Info, 3, ["current value list: ", pValueList]);
	    xOiDebugPrint(@Info, 2, ["current check sum: ", tCheckSum2]);
	    if (tCheckSum1 == tCheckSum2) {
		xOiDebugPrint(@Info, 1, "using stored final type");
		xOiDebugFinishFunc(tData[1]);
		return(tData[1]);
	    }
	}

	var tPID, tFinalType = NULL;

	if (pObj instanceof Vector) {
	    tPID         = pObj[0];
	    var tArticle = pObj[1];
	    var tVarCode = pObj[2];
	    if (pPDB.hasMember(@getOFMLVarCode)) {
		var tPValues = pPDB.varCode2PValues(tArticle, tVarCode);
		if (tPValues == NULL)
		    tPValues = @();
		var tOFMLVarCode = pPDB.getOFMLVarCode(tArticle, tPValues);
		if (tOFMLVarCode != NULL)
		    tFinalType = tArticle + " " + tOFMLVarCode;
	    }
	    else
		tFinalType = pPDB.getFinalArticleSpec(tArticle, tVarCode);
	}
	else {
	    tPID = pObj.getProgram();
	    if (pPDB.hasMember(@getOFMLVarCode)) {
		var tArticle = pObj.getArticleSpec();
		var tOFMLVarCode = pObj.getXArticleSpec(@OFMLVarCode);
		if (tArticle != NULL && tOFMLVarCode != NULL)
		    tFinalType = tArticle + " " + tOFMLVarCode;
	    }
	    else
		tFinalType = pObj.getXArticleSpec(@Final);
	}

	if (tFinalType != NULL) {
	    tFinalType = String(tPID) + ":" + tFinalType;
	    xOiDebugPrint(@Info, 1, ["original final type: ", tFinalType]);
	    
	}
	
	xOiDebugFinishFunc(tFinalType);
	return(tFinalType);
    }

    
    
    
    private func checkSum4PDCache(pValueList)
    {
	if (pValueList == NULL) return(0);

	var tStr = "";
	var tP;
	foreach(tP; pValueList) {
	    var tClass = "NULL";
	    if (tP[0] instanceof String)
		tClass = tP[0];
	    
	    
	    var tVals = tP[3];
	    if (tVals == NULL)
		tVals = "NULL";
	    else
		tVals = String(tVals.size());
	    
	    tStr += "["+tClass+","+tP[1]+","+xOiParam2Str(tP[2])+","+tVals+"]";
	}
	

	var tSum = 0;
	var i;
	for (i=0; i<tStr.size(); i++) tSum += tStr[i];

	return(tSum);
    }

    
    
    
    func usePriceProfiles(pState)
    {
	sUsePriceProfiles = pState;
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticlePrice(pObj, pLanguage, ...)
    {
	
	
	
	
	
	

	if (pObj == NULL) return(NULL);
	if (!(pObj instanceof Vector) && !pObj.isCat(@IF_Article)) return(NULL);

	var tPlan = getPlanning();
	var tCheckUpdateState = (tPlan != NULL && tPlan.getCreationMode() == 0);

	var tLanguage = pLanguage[0];
	var tCurrency = NULL;
	if (pLanguage.size() > 1)
	    tCurrency = pLanguage[1];
	var tDate = NULL;
	if (pLanguage.size() > 2 && xOiIsValidDate(pLanguage[2]))
	    tDate = pLanguage[2];

	xOiDebugStartFunc("xOiPDManager::getArticlePrice",
			  [pObj, tLanguage, tCurrency, tDate]);

	var tRes = NULL;
	if (!(pObj instanceof Vector) && 
	    tCheckUpdateState && !pObj.isUp2Date()) {
	    xOiDebugPrint(@Info, 1, "given article instance is not up-to-date");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	
	
	
	

	var tPID, tDynProps, tPDB, tArticle, tVarCode, tTransaction = 0;

	var tForObject = 1;

	if (pObj instanceof Vector) {
	    tForObject = 0;
	    tPID     = pObj[0];
	    tArticle = pObj[1];
	    tVarCode = pObj[2];
	    
	    oiRegisterPackage(tPID);
	    tPDB     = getProgPDB(tPID);
	    if (tDate == NULL && tPlan.isA(xOiPlanning))
		tDate = tPlan.getPriceDate(tPID);
	}
	else {
	    tPID      = pObj.getProgram();
	    tDynProps = pObj.getDynamicProps();
	    if (tDate == NULL)
		tDate = pObj.getPriceDate();
	    tTransaction = startTransaction(pObj, NULL);
	    if (tTransaction) {
		tArticle = mTransArticle;
		tPDB     = mTransPDB;
	    }
	    else {
		tArticle = pObj.getArticleSpec();
		tPDB = getProgPDB(tPID);
	    }
	}

	xOiDebugPrint(@Info, 2, ["Transaction? ", tTransaction]);
	xOiDebugPrint(@Info, 2, ["using price date ", tDate]);

	var tHashKey = "Price:" + xOiParam2Str(tLanguage) +
		       xOiParam2Str(tCurrency) + xOiParam2Str(tDate);
	var tHashEntry;

	var tInvalidDatePolicy = @Inconsistency; 

	
	
	

	if (sXoiIdxPricesPaths == NULL)
	    sXoiIdxPricesPaths = Hash();

	if (!sXoiIdxPricesPaths.hasKey(tPID))
	    sXoiIdxPricesPaths[tPID] = xOiGetAppIndexpricesPath(tPID);

	var tIdxPricesPath = sXoiIdxPricesPaths[tPID];

	if (tPDB != NULL && tArticle != NULL) {
	    var tFinalType;
	    var tUseCache = ((!tForObject || 
			      !tCheckUpdateState || 
			      pObj.isUp2Date()) &&
			     isPDCacheEnabled());

	    if (tUseCache) {
		tFinalType =
		finalType4PDCache(tPDB,
				  tForObject ?
				  pObj : [tPID, tArticle, tVarCode],
				  tForObject ?
				  buildPropValList(pObj, tPDB, tArticle) :
				  tPDB.varCode2PValues(tArticle, tVarCode));
		xOiDebugPrint(@Info, 2,
			      ["final type for PD cache: ", tFinalType]);

	    	if (sPDCache.hasKey(tFinalType)) {
		    tHashEntry = sPDCache[tFinalType];
		    if (tHashEntry.hasKey(tHashKey)) {
		    	xOiDebugPrint(@Info, 1, "using PD cache");
			var tCacheValue = tHashEntry[tHashKey];
			
			
			
			if (tCacheValue == NULL) 
		    	    tRes = NULL;
			else
			if (tCacheValue[0][0] == "::ofml::xoi::@invalid_date") {
			    tRes = @();
			    xOiCopyAggr(tCacheValue, tRes, 1);
			    if (tDynProps != NULL) {
				tDynProps[@_XOI_INVALID_PRICE_DATE] = 1;
				xOiDebugPrint(@Info, 3,
					      ["setting XOI_INVALID_PRICE_DATE",
					       " for ", pObj]);
			    }
			}
			else {
			    tRes = @();
			    xOiCopyAggr(tCacheValue, tRes, 1);
			    if (sUsePriceProfiles) {
				var tRes2 = NULL;
				if (pObj instanceof Vector) {
				    var tPDInfo =
				    getPDInfo([tPID, tArticle, tVarCode],
					      tLanguage);
				    tRes2 =
				    getProfiledPrice(
					[tPDInfo[0], tPDInfo[1], tArticle],
					 tRes, tLanguage);
				}
				else
				    tRes2 =
				    getProfiledPrice(pObj, tRes, tLanguage);

				if (tRes2 != NULL) {
				    xOiDebugPrint(@Info, 1,
						  "applied price profiles");
				    tRes = tRes2;
				}
			    }
			    if (priceHasVarConds(tRes))
				tRes = stripPriceVarConds(tRes);
			    if (tDynProps != NULL &&
				tDynProps.hasKey(@_XOI_INVALID_PRICE_DATE)) {
				
				tDynProps.remove(@_XOI_INVALID_PRICE_DATE);
				xOiDebugPrint(@Info, 3,
					      ["removing XOI_INVALID_PRICE_DATE",
					       " for ", pObj]);
			    }
			}
		    	if (tTransaction) finishTransaction(pObj);
			sUsePriceProfiles = 1;
		    	xOiDebugFinishFunc(tRes);
		    	return(tRes);
		    }
		    else
		    	xOiDebugPrint(@Info, 2,
				      ["no PD cache entry for key ", tHashKey]);
	    	}
	    	else
		if (tFinalType != NULL && !tFinalType.empty()) {
		    
		    xOiDebugPrint(@Info, 2,
			          ["making PD cache entry for final type: ",
				   tFinalType]);
		    sPDCache[tFinalType] = Hash(); 
	    	}
	    }

	    var tPValues;
	    if (tForObject)
		tPValues = buildPropValList(pObj, tPDB, tArticle);
	    else {
		tPValues = tPDB.varCode2PValues(tArticle, tVarCode);
		if (tPValues == NULL)
		    tPValues = @();
		if (tPDB.isA(xOiEPDFProductDB))
		    tPDB.setProgram(tPID); 
		    
	    }

	    var tInvalidDate = 0;

	    if (tPDB.hasMember(@getArticlePrice3)) {
		if (tInvalidDatePolicy == @Inconsistency && 
		    tDynProps != NULL &&
		    tDynProps.hasKey(@_XOI_INVALID_PRICE_DATE)) {
		    tDynProps.remove(@_XOI_INVALID_PRICE_DATE);
		    xOiDebugPrint(@Info, 3, ["removing XOI_INVALID_PRICE_DATE",
					     " for ", pObj]);
		}
	        tRes =
		tPDB.getArticlePrice3(tArticle, tPValues, tLanguage, tCurrency,
				      tDate, tIdxPricesPath);
		if ((tRes instanceof Symbol && tRes == @InvalidDate) ||
		    ((tRes instanceof List || tRes instanceof Vector) &&
		     tRes.size() > 1 && tRes[0][0] instanceof String &&
		     tRes[0][0] == "::ofml::xoi::@invalid_date"))  {
		    xOiDebugPrint(@Info, 1, ["invalid price date ", tDate]);
		    tInvalidDate = 1;
		    if (pObj instanceof Vector) 
			
			tRes = NULL;
		    else
		    if (tInvalidDatePolicy == @Inconsistency) {
			if (tRes instanceof Symbol && tRes == @InvalidDate)
			    tRes = NULL;
			tDynProps[@_XOI_INVALID_PRICE_DATE] = 1;
			xOiDebugPrint(@Info, 3, 
				      ["setting XOI_INVALID_PRICE_DATE",
				       " for ", pObj]);
		    }
		    else { 
			xOiDebugPrint(@Info, 1, "using current date!");
			var tCurrDate = oiGetCurrentDate();
			pObj.setPriceDate(tCurrDate);
			tHashKey = "Price:" + xOiParam2Str(tLanguage) + 
				   xOiParam2Str(tCurrency) + 
				   xOiParam2Str(tCurrDate);
			tRes =
			tPDB.getArticlePrice3(tArticle, tPValues, tLanguage, 
					      tCurrency, NULL, tIdxPricesPath);
		    }
		}
	    }
	    else
	    if (tPDB.hasMember(@getArticlePrice2))
	        tRes =
		tPDB.getArticlePrice2(tArticle, tPValues, tLanguage, tCurrency,
				      tIdxPricesPath);
	    else
	        tRes =
		tPDB.getArticlePrice(tArticle, tPValues, tLanguage, tCurrency);

	    if (tUseCache && tHashEntry != NULL) {
		
		
		var tCacheValue = @();
		if (tRes == NULL)
		    tCacheValue = NULL;
		else
		    
		    xOiCopyAggr(tRes, tCacheValue, 1);
		xOiDebugPrint(@Info, 2,
			      ["making PD cache entry for key ", tHashKey]);
	        tHashEntry[tHashKey] = tCacheValue;
	    }

	    if (sUsePriceProfiles && !tInvalidDate) {
		var tRes2 = NULL;
		if (pObj instanceof Vector) {
		    var tPDInfo = getPDInfo([tPID, tArticle, tVarCode],
					    tLanguage);
		    if (tPDInfo == NULL)
			xOiDebugPrint(@Warn, 1, 
				      ["getPDInfo() returned NULL for ",
				       "article ", tArticle, " (", tPID, ")!"]);
		    else
			tRes2 = 
			getProfiledPrice([tPDInfo[0], tPDInfo[1], tArticle],
					 tRes, tLanguage);
		}
		else
		    tRes2 = getProfiledPrice(pObj, tRes, tLanguage);
		if (tRes2 != NULL) {
		    xOiDebugPrint(@Info, 1, "applied price profiles");
		    tRes = tRes2;
		}
	    }
	    if (priceHasVarConds(tRes))
		tRes = stripPriceVarConds(tRes);
	}

	if (tTransaction) 
	    finishTransaction(pObj);
	else
	if (tPDB.isA(xOiEPDFProductDB))
	    tPDB.setProgram(NULL); 

	sUsePriceProfiles = 1;

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    private func priceHasVarConds(pPrice)
    {
	return(pPrice != NULL && !pPrice.empty() && pPrice.front().size() == 4);
    }

    
    
    
    private func stripPriceVarConds(pPrice)
    {
	var tNewPrice = @();

	var tP;
	foreach(tP; pPrice) tNewPrice.pushBack([tP[0], tP[1], tP[2]]);

	return(tNewPrice);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticleTaxInfo(pObj, pCountry, pRegion)
    {
	xOiDebugStartFunc("xOiPDManager::getArticleTaxInfo",
			  [pObj, pCountry, pRegion]);

	
	
	
	
	

	if (pObj == NULL ||
	    (!(pObj instanceof Vector) && !pObj.isCat(@IF_Article))) {
	    xOiDebugPrint(@Warn, 1, "invalid object!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRes, tPID, tPDB, tArticle, tVarCode, tPValues, tTransaction = 0;

	if (pObj instanceof Vector) {
	    tPID     = pObj[0];
	    tArticle = pObj[1];
	    tVarCode = pObj[2];
	    
	    oiRegisterPackage(tPID);
	    tPDB     = getProgPDB(tPID);
	}
	else {
	    tTransaction = 1;
	    startTransaction(pObj, NULL); 
	    tArticle = mTransArticle;
	    tPDB     = mTransPDB;
	    tPID     = pObj.getProgram();
	}

	if (tPDB != NULL && tArticle != NULL &&
	    tPDB.hasMember(@getArticleTaxInfo)) {
	    if (tTransaction)
		tPValues = buildPropValList(pObj, tPDB, tArticle);
	    else
		tPValues = tPDB.varCode2PValues(tArticle, tVarCode);
	    tRes = tPDB.getArticleTaxInfo(tArticle, tPValues, 
					  pCountry, pRegion);
	}

	if (tTransaction) finishTransaction(pObj);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getProfiledPrice(pObj, pPrice, pLanguage)
    {
	xOiDebugStartFunc("xOiPDManager::getProfiledPrice",
			  [pObj, pPrice, pLanguage]);

	if (pPrice == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tMID, tSID, tArticle;
	if (pObj instanceof Vector) {
	    tMID = pObj[0];
	    tSID = pObj[1];
	    tArticle = pObj[2];
	}
	else {
	    var tPDInfo = pObj.getPDInfo(pLanguage);
	    if (tPDInfo == NULL) {
		xOiDebugPrint(@Warn, 1,
			      ["getPDInfo() for ", pObj, " returned NULL!"]);
		xOiDebugFinishFunc(NULL);
		return(NULL);
	    }
	    tMID = tPDInfo[0];
	    tSID = tPDInfo[1];
	    tArticle = pObj.getArticleSpec();
	}

	var tPValues = "";
	if (!(pObj instanceof Vector)) {
	    var tPKeys = pObj.getPropertyKeys();
	    var tPKey;
	    foreach (tPKey; tPKeys) {
		if (!tPValues.empty()) tPValues += ",";
		var tPVal = pObj.getPropValue(tPKey);
		tPValues += xOiEscapeQuotes(xOiParam2Str(tPVal));
	    }
	}

	var tPrice = @();
	var tPriceHasVarConds = (!pPrice.empty() && pPrice.front().size() == 5);
	var tP;
	foreach(tP; pPrice) {
	    var tText     = xOiSubstString(tP[0], "\"", "\\\"");
	    var tSales    = tP[1];
	    var tPurchase = tP[2];
	    if (tPriceHasVarConds) {
		var tVC = tP[3];
		var tPF = tP[4];
	        tPrice.pushBack([tText, tSales, tPurchase, tVC, tPF]);
	    }
	    else
	        tPrice.pushBack([tText, tSales, tPurchase]);
	}

	xOiDebugPrint(@Info, 2,
		      ["profile params: ", 
		       [tMID, tSID, tArticle, tPValues, tPrice]]);

	var tRes = 
	xOiApplyAppPriceProfiles(tMID, tSID, tArticle, tPValues, tPrice);

	if (priceHasVarConds(tRes)) {
	    xOiDebugPrint(@Warn, 1, 
			  "application function returend invalid price " +
			  "with extended price items!");
	    tRes = stripPriceVarConds(tRes);
	}

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    public func getScaleArticlePrice(pObj, pQuantity, pLanguage, pCurrency)
    {
	xOiDebugStartFunc("xOiPDManager::getScaleArticlePrice",
			  [pObj, pQuantity, pLanguage, pCurrency]);

	
	
	
	
	

	if (pObj == NULL ||
	    (!(pObj instanceof Vector) && !pObj.isCat(@IF_Article))) {
	    xOiDebugPrint(@Warn, 1, "invalid object!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRes, tPID, tPDB, tArticle, tVarCode, tPValues, tTransaction = 0;

	if (pObj instanceof Vector) {
	    tPID     = pObj[0];
	    tArticle = pObj[1];
	    tVarCode = pObj[2];
	    
	    oiRegisterPackage(tPID);
	    tPDB     = getProgPDB(tPID);
	}
	else {
	    tTransaction = 1;
	    startTransaction(pObj, NULL); 
	    tArticle = mTransArticle;
	    tPDB     = mTransPDB;
	    tPID     = pObj.getProgram();
	}

	if (tPDB != NULL && tArticle != NULL &&
	    tPDB.hasMember(@getScaleArticlePrice)) {
	    if (tTransaction)
		tPValues = buildPropValList(pObj, tPDB, tArticle);
	    else
		tPValues = tPDB.varCode2PValues(tArticle, tVarCode);
	    tRes = tPDB.getScaleArticlePrice(tArticle, tPValues, pQuantity,
					     pLanguage, pCurrency);
	}

	if (tTransaction) finishTransaction(pObj);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticlePackagingInfo(pObj)
    {
	xOiDebugStartFunc("xOiPDManager::getArticlePackagingInfo", pObj);

	
	
	
	
	

	if (pObj == NULL ||
	    (!(pObj instanceof Vector) && !pObj.isCat(@IF_Article))) {
	    xOiDebugPrint(@Warn, 1, "invalid object!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRes, tPID, tPDB, tArticle, tVarCode, tPValues, tTransaction = 0;

	if (pObj instanceof Vector) {
	    tPID     = pObj[0];
	    tArticle = pObj[1];
	    tVarCode = pObj[2];
	    
	    oiRegisterPackage(tPID);
	    tPDB     = getProgPDB(tPID);
	}
	else {
	    tTransaction = 1;
	    startTransaction(pObj, NULL); 
	    tArticle = mTransArticle;
	    tPDB     = mTransPDB;
	    tPID     = pObj.getProgram();
	}

	if (tPDB != NULL && tArticle != NULL &&
	    tPDB.hasMember(@getArticlePackagingInfo)) {
	    if (tTransaction)
		tPValues = buildPropValList(pObj, tPDB, tArticle);
	    else
		tPValues = tPDB.varCode2PValues(tArticle, tVarCode);
	    tRes = tPDB.getArticlePackagingInfo(tArticle, tPValues);
	}

	if (tTransaction) finishTransaction(pObj);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    public func getArticleText(pObj, pLanguage, pForm)
    {
	
	
	
        
	

	if (pObj == NULL) return(NULL);
	if (!(pObj instanceof Vector) && !pObj.isCat(@IF_Article)) return(NULL);

	xOiDebugStartFunc("xOiPDManager::getArticleText",
			  [pObj, pLanguage, pForm]);

	var tRes = NULL;

	var tHashKey = xOiParam2Str(pForm) + xOiParam2Str(pLanguage);
	var tHashEntry;

	var tPID, tPDB, tArticle, tVarCode, tTransaction = 0;

	var tForObject = 1;

	if (pObj instanceof Vector) {
	    tForObject = 0;
	    tPID     = pObj[0];
	    tArticle = pObj[1];
	    tVarCode = pObj[2];
	    
	    oiRegisterPackage(tPID);
	    tPDB     = getProgPDB(tPID);
	}
	else {
	    tPID = pObj.getProgram();
	    tTransaction = startTransaction(pObj, NULL);
	    if (tTransaction) {
		tArticle = mTransArticle;
		tPDB     = mTransPDB;
	    }
	    else {
		tArticle = pObj.getArticleSpec();
		tPDB = getProgPDB(tPID);
	    }
	}

	xOiDebugPrint(@Info, 1, ["article: ", tArticle]);
	xOiDebugPrint(@Info, 2, ["Transaction? ", tTransaction]);

	if (tPDB != NULL && tArticle != NULL) {
	    var tUseCache = isPDCacheEnabled();

	    if (tUseCache) {
		var tHashKey = String(tPID) + ":" + tArticle;
	    	if (sArticleTextCache.hasKey(tHashKey)) {
		    tHashEntry = sArticleTextCache[tHashKey];
		    if (tHashEntry.hasKey(tHashKey)) {
		    	xOiDebugPrint(@Info, 1, "using PD cache");
			
			
			
			if (tHashEntry[tHashKey] == NULL)
		    	    tRes = NULL;
			else {
		    	    
			    tRes = @();
			    xOiCopyAggr(tHashEntry[tHashKey], tRes, 1);
			}
		    	if (tTransaction) finishTransaction(pObj);
		    	xOiDebugFinishFunc(tRes);
		    	return(tRes);
		    }
		    else
		    	xOiDebugPrint(@Info, 2,
				      ["no PD cache entry for key ", tHashKey]);
	    	}
	    	else {
		    xOiDebugPrint(@Info, 2,
			          ["making PD cache entry for: ", tHashKey]);
		    sArticleTextCache[tHashKey] = Hash(); 
		}
	    }

	    tRes = tPDB.getArticleText(tArticle, pLanguage, pForm);

	    if (tUseCache && tHashEntry != NULL) {
		
		
		var tCacheValue = @();
		if (tRes == NULL)
		    tCacheValue = NULL;
		else
		    
		    xOiCopyAggr(tRes, tCacheValue, 1);
		xOiDebugPrint(@Info, 2,
			      ["making PD cache entry for key ", tHashKey]);
	        tHashEntry[tHashKey] = tCacheValue;
	    }
	}

	if (tTransaction) finishTransaction(pObj);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticleFeatures(pObj, pLanguage)
    {
	return(_getArticleFeatures(pObj, pLanguage, 0, 0));
    }

    
    
    
    
    
    
    
    
    
    
    public func getArticleFeatures2(pObj, pLanguage)
    {
	return(_getArticleFeatures(pObj, pLanguage, 1, 0));
    }

    
    
    
    
    
    
    
    
    
    
    
    
    public func getAllArticleFeatures(pObj, pLanguage)
    {
	return(_getArticleFeatures(pObj, pLanguage, 1, 1));
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticleFeaturesDescr(pObj, pType, pLanguage)
    {
	var tValidTypes = @(@Text, @AllIDs, @ID_Text);

	if (tValidTypes.find(pType) < 0) return(NULL);

	if (pType == @Text) 
	    return(self.getArticleFeatures(pObj, pLanguage));

	if (pType == @AllIDs) 
	    return(self.getAllArticleFeatures(pObj, pLanguage));

	
	return(_getArticleFeatures(pObj, pLanguage, 2, 0));
    }

    
    
    
    
    
    
    private func _getArticleFeatures(pObj, pLanguage, pNeedIDs, pAllProps)
    {
	if (pObj == NULL) return(NULL);
	if (!(pObj instanceof Vector) && !pObj.isCat(@IF_Article)) return(NULL);

	xOiDebugStartFunc("xOiPDManager::getArticleFeatures",
			  [pObj, pLanguage, pNeedIDs, pAllProps]);

	var tPlan = getPlanning();
	var tCheckUpdateState = (tPlan != NULL && tPlan.getCreationMode() == 0);

	var tRes = NULL;
	if (!(pObj instanceof Vector) && 
	    tCheckUpdateState && !pObj.isUp2Date()) {
	    xOiDebugPrint(@Info, 1, "given article instance is not up-to-date");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tXCR_Export = 0;
	if (tPlan != NULL)
	    tXCR_Export = (tPlan.getAppMode(@Export) == @XCR);

	var tHashKey = "Features:" + String(pNeedIDs) + 
		       xOiParam2Str(pLanguage) + String(pAllProps);
	var tHashEntry;

	var tPID, tPDB, tArticle, tVarCode, tTransaction = 0;

	var tForObject = 1;

	if (pObj instanceof Vector) {
	    tForObject = 0;
	    tPID     = pObj[0];
	    tArticle = pObj[1];
	    tVarCode = pObj[2];
	    
	    oiRegisterPackage(tPID);
	    tPDB     = getProgPDB(tPID);
	}
	else {
	    tPID = pObj.getProgram();
	    tTransaction = startTransaction(pObj, NULL);
	    if (tTransaction) {
		tArticle = mTransArticle;
		tPDB = mTransPDB;
	    }
	    else {
		tArticle = pObj.getArticleSpec();
		tPDB = getProgPDB(tPID);
	    }
	}

	xOiDebugPrint(@Info, 2, ["Transaction? ", tTransaction]);

	if (tPDB != NULL && tArticle != NULL) {
	    var tFinalType;
	    var tUseCache = ((!tForObject || 
			      !tCheckUpdateState || 
			       pObj.isUp2Date()) &&
			     pNeedIDs != 2 && isPDCacheEnabled());

	    if (tUseCache) {
		tFinalType =
		finalType4PDCache(tPDB,
				  tForObject ?
				  pObj : [tPID, tArticle, tVarCode],
				  tForObject ?
				  buildPropValList(pObj, tPDB, tArticle) :
				  tPDB.varCode2PValues(tArticle, tVarCode));
		xOiDebugPrint(@Info, 2,
			      ["final type for PD cache: ", tFinalType]);

	    	if (sPDCache.hasKey(tFinalType)) {
		    tHashEntry = sPDCache[tFinalType];
		    if (tHashEntry.hasKey(tHashKey)) {
		    	xOiDebugPrint(@Info, 1, "using PD cache");
			
			
			
			if (tHashEntry[tHashKey] == NULL)
		    	    tRes = NULL;
			else {
		    	    
			    tRes = @();
			    xOiCopyAggr(tHashEntry[tHashKey], tRes, 1);
			}
		    	if (tTransaction) finishTransaction(pObj);
			if (pNeedIDs == 1 && pAllProps == 0 && tXCR_Export)
			    tRes = prepareArtFeatures4XCR(tRes, pObj, 
							  tArticle, tPDB); 
		    	xOiDebugFinishFunc(tRes);
		    	return(tRes);
		    }
		    else
		    	xOiDebugPrint(@Info, 2,
				      ["no PD cache entry for key ", tHashKey]);
	    	}
	    	else
		if (tFinalType != NULL && !tFinalType.empty()) {
		    
		    xOiDebugPrint(@Info, 2,
			          ["making PD cache entry for final type: ",
				   tFinalType]);
		    sPDCache[tFinalType] = Hash(); 
		}
	    }

	    var tPValues;
	    if (tForObject)
		tPValues = buildPropValList(pObj, tPDB, tArticle);
	    else
		tPValues = tPDB.varCode2PValues(tArticle, tVarCode);

	    if (pAllProps)
	        tRes = tPDB.getAllPropDescriptions(tArticle, tPValues, 
					           pLanguage);
	    else {
		if (pNeedIDs == 2) {
		    if (!tPDB.hasMember(@getPropDescription2))
			xOiDebugPrint(@Warn, 1, 
				     [tPDB, " has no getPropDescription2()!"]); 
		    else
	                tRes = tPDB.getPropDescription2(tArticle, tPValues, 
					                pLanguage);
		}
		else {
	            tRes = tPDB.getPropDescription(tArticle, tPValues, 
					           pNeedIDs, pLanguage);
		}
	    }

	    if (tUseCache && tHashEntry != NULL) {
		
		
		var tCacheValue = @();
		if (tRes == NULL)
		    tCacheValue = NULL;
		else
		    
		    xOiCopyAggr(tRes, tCacheValue, 1);
		xOiDebugPrint(@Info, 2,
			      ["making PD cache entry for key ", tHashKey]);
	        tHashEntry[tHashKey] = tCacheValue;
	    }
	}

	if (pNeedIDs == 1 && pAllProps == 0 && tXCR_Export)
	    tRes = prepareArtFeatures4XCR(tRes, pObj, tArticle, tPDB); 

	if (tTransaction) finishTransaction(pObj);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    private func prepareArtFeatures4XCR(pRes, pObj, pArticle, pPDB)
    {
	xOiDebugStartFunc2("xOiPDManager::prepareArtFeatures4XCR", 
			   [pObj, pArticle, pPDB]);

	var tNonPDProps = readNonPDProps(pObj, NULL);
	var tPDInfo = pPDB.getPDInfo(pArticle, "en");
	var tSeries = tPDInfo[1];

	var tPClasses = Hash(); 
	var tPKey;
	foreach(tPKey; pObj.getPropertyKeys()) {
	    var tPCN = propKey2PropClassName(pObj, tPKey, pPDB, tNonPDProps);
	    var tPClass = tPCN[0];
	    var tPName  = tPCN[1];

	    xOiDebugPrint(@Info, 2, ["key: ", tPKey, " -> prop: ", tPName]);

	    if (tPName == NULL) continue;

	    var tPClass2 = pObj.getPropClass(tPKey);

	    if (tPClass == NULL || tPClass.empty() || tPClass != tPClass2) {
		tPClass = tPClass2;
		xOiDebugPrint(@Info, 1, 
			      ["using prop class ", tPClass2, " instead of ", 
			       tPClass, " for property ", tPKey, 
			       " (", tPName, ")" ]);
	    }
	    tPClasses[tPName] = tPClass;
	}

	var tRes = @();
	var tP;
	foreach(tP; pRes) {
	    xOiDebugPrint(@Info, 1, ["check feature ", tP]);
	    if (tP.size() < 4) continue; 
	    var tPName = tP[0];
	    var tPText = tP[1];
	    var tPVal  = tP[2];
	    var tVText = tP[3];
	    if (!tPClasses.hasKey(tPName)) {
		
		xOiDebugPrint(@Warn, 1, 
			      ["no property/class found for ", tPName, "!?"]);
		continue;
	    }
	    var tPClass = tPClasses[tPName];
	    xOiDebugPrint(@Info, 2, ["class: ", tPClass]);
	    tPName = tSeries + ":" + tPClass + "." + tPName;
	    tRes.pushBack([tPName, tPText, tPVal, tVText]);
	}

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    public func getPDInfo(pObj, pLanguage)
    {
	if (pObj == NULL) return(NULL);
	if (!(pObj instanceof Vector) && !pObj.isCat(@IF_Article)) return(NULL);

	xOiDebugStartFunc("xOiPDManager::getPDInfo", [pObj, pLanguage]);

	var tRes = NULL;

	var tHashKey = xOiParam2Str(pLanguage);
	var tHashEntry;

	var tPID, tPDB, tArticle, tVarCode, tTransaction = 0;

	var tForObject = 1;

	if (pObj instanceof Vector) {
	    tForObject = 0;
	    tPID     = pObj[0];
	    tArticle = pObj[1];
	    tVarCode = pObj[2];
	    
	    oiRegisterPackage(tPID);
	    tPDB     = getProgPDB(tPID);
	}
	else {
	    tPID = pObj.getProgram();
	    tTransaction = startTransaction(pObj, NULL);
	    if (tTransaction) {
		tArticle = mTransArticle;
		tPDB = mTransPDB;
	    }
	    else {
		tArticle = pObj.getArticleSpec();
		tPDB = getProgPDB(tPID);
	    }
	}

	xOiDebugPrint(@Info, 1, ["article? ", tArticle]);
	xOiDebugPrint(@Info, 2, ["Transaction? ", tTransaction]);

	if (tPDB != NULL && tArticle != NULL) {
	    var tUseCache = isPDCacheEnabled();

	    if (tUseCache) {
		var tHashKey = String(tPID) + ":" + tArticle;
	    	if (sPDInfoCache.hasKey(tHashKey)) {
		    tHashEntry = sPDInfoCache[tHashKey];
		    if (tHashEntry.hasKey(tHashKey)) {
		    	xOiDebugPrint(@Info, 1, "using PD cache");
			
			
			
			if (tHashEntry[tHashKey] == NULL)
		    	    tRes = NULL;
			else {
		    	    
			    tRes = Vector(0);
			    xOiCopyAggr(tHashEntry[tHashKey], tRes, 1);
			}
		    	if (tTransaction) finishTransaction(pObj);
		    	xOiDebugFinishFunc(tRes);
		    	return(tRes);
		    }
		    else
		    	xOiDebugPrint(@Info, 2,
				      ["no PD cache entry for key ", tHashKey]);
	    	}
	    	else {
		    xOiDebugPrint(@Info, 2,
			          ["making PD cache entry for: ", tHashKey]);
		    sPDInfoCache[tHashKey] = Hash(); 
	    	}
	    }

	    tRes = tPDB.getPDInfo(tArticle, pLanguage);

	    if (tUseCache && tHashEntry != NULL) {
		
		
		var tCacheValue = Vector(0);
		if (tRes == NULL)
		    tCacheValue = NULL;
		else
		    
		    xOiCopyAggr(tRes, tCacheValue, 1);
		xOiDebugPrint(@Info, 2,
			      ["making PD cache entry for key ", tHashKey]);
	        tHashEntry[tHashKey] = tCacheValue;
	    }
	}

	if (tTransaction) finishTransaction(pObj);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    

    
    
    
    
    
    
    
    public func getXArticleSpec(pObj, pMode)
    {
	xOiDebugStartFunc("xOiPDManager::getXArticleSpec", [pObj, pMode]);

	var tRes = NULL;

	var tValidModes = @(@VarCode, @OFMLVarCode, @Final, @EAN_UCC);

	if (tValidModes.find(pMode) < 0 ||
	    (pObj == NULL && mTransObj == NULL)) {
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	if (pMode == @EAN_UCC) {
	    xOiDebugPrint(@Info, 1, "calling inherited implementation");
	    tRes = OiPDManager::getXArticleSpec(pObj, pMode);
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tObj = pObj;
	if (tObj == NULL) tObj = mTransObj;

	if (tObj == NULL) {
	    xOiDebugPrint(@Warn, 1, "no object");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tHasTransaction = hasTransaction(tObj);

	var tPDB     = mTransPDB;
	var tArticle = mTransArticle;

	if (tHasTransaction) {
	    if (pMode == @VarCode)
		tRes = sTransVarCode;
	    else
	    if (pMode == @OFMLVarCode)
		tRes = sTransOFMLVarCode;
	    else 
		tRes = sTransFinalType;

	    if (tRes != NULL) {
	    	xOiDebugPrint(@Info, 1, "using transaction cache");
		xOiDebugFinishFunc(tRes);
		return(tRes);
	    }
	}
	else {
	    tPDB = getPDBFor(tObj);
	    tArticle = tObj.getArticleSpec();
	}

	if (tPDB == NULL || tArticle == NULL) {
	    if (tArticle == NULL)
		xOiDebugPrint(@Warn, 2, "no article specified");
	    else
		xOiDebugPrint(@Warn, 2, "no product database");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	xOiDebugPrint(@Info, 2, ["product database: ", tPDB.getID(), " ",
			         tPDB.getClass()]);

	if (pMode == @OFMLVarCode && !tPDB.hasMember(@getOFMLVarCode)) {
	    xOiDebugPrint(@Warn, 1, 
			  "invalid product database for @OFMLVarCode!");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tPlan      = getPlanning();

	var tTransObj = mTransObj; 
	var tRestart  = 0;
	var tTmpTransaction = 0;   

	if (tPlan == NULL || tPlan.isA(xOiPlanning) ||
	    !tPlan.isCat(@TestPlanning) || 
	    !tPlan.hasMember(@getXArticleSpec4Obj) ||
	    (tRes = tPlan.getXArticleSpec4Obj(pMode)) == NULL) {

	    xOiDebugPrint(@Info, 1, "getting spec from product database");

	    if (!tHasTransaction) {
	        
	        if (mTransObj != NULL && hasTransaction(mTransObj)) {
		    finishTransaction(mTransObj);
		    tRestart = 1;
	        }
	        startTransaction(tObj, tArticle); 
		tTmpTransaction = 1;
	    }
	    var tPValues = buildPropValList(tObj, tPDB, tArticle);
	    if (pMode == @VarCode)
		tRes = tPDB.getVarCode(tArticle, tPValues);
	    else
	    if (pMode == @OFMLVarCode)
		tRes = tPDB.getOFMLVarCode(tArticle, tPValues);
	    else 
		tRes = tPDB.getFinalArticleSpec(tArticle, tPValues);
	}

	if (tHasTransaction) {
	    if (pMode == @VarCode)
		sTransVarCode = tRes;
	    else
	    if (pMode == @OFMLVarCode)
		sTransOFMLVarCode = tRes;
	    else 
		sTransFinalType = tRes;
	}
	else 
	if (tTmpTransaction) {
	    finishTransaction(tObj);
	    if (tRestart)
		startTransaction(tTransObj, NULL);
	    
	}

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func getVarCodeFromImport(pObj, pPValues)
    {
	xOiDebugStartFunc("xOiPDManager::getVarCodeFromImport",
			  [pObj, pPValues]);

	var tRes = NULL;
	var tPDB = NULL;
	var tArticle = NULL;
	var tObj = NULL;
	var tHasTransaction = 0;

	if (pObj instanceof Vector) {
	    
	    oiRegisterPackage(pObj[0]);
	    tPDB = getProgPDB(pObj[0]);
	    tArticle = pObj[1];
	}
	else {
	    tPDB = getPDBFor(pObj);
	    tArticle = pObj.getArticleSpec();
	    tObj = pObj;
	    tHasTransaction = hasTransaction(tObj);
	}

	if (tPDB == NULL || tArticle == NULL) {
	    if (tArticle == NULL)
		xOiDebugPrint(@Warn, 1, "no article specified");
	    else
		xOiDebugPrint(@Warn, 1, "no product database");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tTmpObj = NULL;
	if (tObj == NULL) {
	    var tClass = article2Class(tArticle);
	    if (tClass == NULL)
		xOiDebugPrint(@Warn, 1,
			      ["no class found in mapping for ", tArticle]);
	    else {
		tClass = eval(tClass);
		if (tClass.subClassOf(xOiBTGPlElement)) {
		    tObj = tTmpObj = tClass(self, @trans_obj);
		    tObj.setArticleSpec(tArticle);
		    xOiDebugPrint(@Info, 1, ["created temporary ", tObj]);
		}
	    }
	}
	var tTransObj = mTransObj;
	if (!tHasTransaction)
	    
	    
	    mTransObj = tObj;

	tRes = tPDB.getVarCode(tArticle, pPValues, 0);

	if (!tHasTransaction) mTransObj = tTransObj;
	if (tTmpObj != NULL) remove(tTmpObj);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    public func setXArticleSpec(pObj, pMode, pSpec)
    {
	xOiDebugStartFunc("xOiPDManager::setXArticleSpec",
			  [pObj, pMode, pSpec]);

	var tValidModes = @(@VarCode, @OFMLVarCode, @Final);
	if (tValidModes.find(pMode) < 0) {
	    xOiDebugPrint(@Warn, 1, "invaild mode");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	if (pObj == NULL) {
	    xOiDebugPrint(@Warn, 1, "no object");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	var tArticle = pObj.getArticleSpec();

	if (tArticle == NULL) {
	    xOiDebugPrint(@Warn, 2, "no article specified");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	if (pSpec == NULL || pSpec.empty()) {
	    xOiDebugPrint(@Warn, 2, "no spec given");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	if (!startTransaction(pObj, tArticle)) {
	    xOiDebugPrint(@Warn, 1, "could not start transaction!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tPDB = mTransPDB;
	if (tPDB == NULL) {
	    xOiDebugPrint(@Warn, 2, "no product database");
	    xOiDebugFinishFunc(NULL);
	    return;
	}
	xOiDebugPrint(@Info, 2,
		      ["product database: ", tPDB.getID(), " ",
		       tPDB.getClass()]);

	if (pMode == @OFMLVarCode &&
	    !(tPDB.isA(xOiEPDFProductDB) || tPDB.isA(xOiNativeOCDProductDB))) {
	    xOiDebugPrint(@Warn, 1, 
			  "@OFMLVarCode cannot be handled by product database");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	if (tPDB.hasMember(@getPropDefs2)) {
	    
	    var tChangedPropVals = pObj.changedPropList();
	    tChangedPropVals.erase(0, tChangedPropVals.size());

	    _setupProps(pObj, tArticle, pSpec, pMode);

	    
	    if (!tChangedPropVals.empty()) {
		tChangedPropVals.sort();
		tChangedPropVals.unique();
		pObj.propsChanged(tChangedPropVals, 0);
	    }
	}
	else
	    assignPropValues(pObj, pMode, pSpec, tPDB, tArticle);

	finishTransaction(pObj);

	xOiDebugFinishFunc(NULL);
    }

    
    
    private func assignPropValues(pObj, pMode, pSpec, pPDB, pArticle)
    {
	var tPValues;

	if (pMode == @VarCode)
	    tPValues = pPDB.varCode2PValues(pArticle, pSpec);
	else
	    tPValues = pPDB.finalArt2PValues(pArticle, pSpec);

	xOiDebugPrint(@Info, 2, ["prop values: ", tPValues]);

	if (tPValues != NULL && !tPValues.empty()) {
	    var tPV;
	    foreach(tPV; tPValues) {
		var tPClass = tPV[0];
		var tPName = tPV[1];
		var tPNewVal = tPV[2];
		var tNewIsVoid = (tPNewVal instanceof Symbol &&
				  tPNewVal == @VOID);
		var tPKey = propName2PropKey(pObj, tPClass, tPName)[0];
		xOiDebugPrint(@Info, 2, ["prop ", tPName, " -> key ", tPKey]);
		if (tPKey != NULL && pObj.hasProperty(tPKey)) {
		    var tPOldVal = pObj.getPropValue(tPKey);
		    var tOldIsVoid = (tPOldVal instanceof Symbol &&
				      tPOldVal == @VOID);
		    if ((tNewIsVoid && !tOldIsVoid) ||
			(!tNewIsVoid && tOldIsVoid) || tPNewVal != tPOldVal) {
			xOiDebugPrint(@Info, 1,
				      ["new val for ", tPClass, ".", tPName,
				       " = ", tPNewVal]);
			
			
			
			pObj.setPropValue(tPKey, tPNewVal);
		    }
		}
	    }
	}
    }

    public func manualSetMembers(pMembers)
    {
	var tRes = OiPDManager::manualSetMembers(pMembers);

	if (!pMembers.hasKey(@mPDCache))
	    tRes[1].pushBack(@mPDCache);
	if (!pMembers.hasKey(@mUsePDCache))
	    tRes[1].pushBack(@mUsePDCache);
	if (!pMembers.hasKey(@mTransactionFinished))
	    tRes[1].pushBack(@mTransactionFinished);
	if (!pMembers.hasKey(@mUpdatabilityCheckContext))
	    tRes[1].pushBack(@mUpdatabilityCheckContext);
	if (!pMembers.hasKey(@mClassesCache))
	    tRes[1].pushBack(@mClassesCache);
	if (!pMembers.hasKey(@mXoiInsignificantPClasses))
	    tRes[1].pushBack(@mXoiInsignificantPClasses);

	return (tRes);
    }

    rule START_DUMP(pArg)
    {
	
	sTmpChildFather = NULL;
	sTmpChildArticle = NULL;
	sTmpChildPVals = NULL;
	sTmpChildProps = NULL;
	sTmpChildAddSpec = NULL;
	sTmpChildChfPValTexts = NULL;
	sTmpChildVisPropVals = NULL;
	cleanUpTransaction(0);

	return(0);
    }

    rule START_EVAL(pArg)
    {
	xOiDebugStartFunc("xOiPDManager::START_EVAL", NULL);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	
	if (mUsePDCache == NULL) mUsePDCache = 1;

	

	mClassesCache = Hash();

	xOiDebugFinishFunc(0);
	return(0);
    }

    rule FINISH_EVAL(pArg)
    {
	xOiDebugStartFunc("xOiPDManager::FINISH_EVAL", NULL);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	xOiDebugFinishFunc(0);
	return(0);
    }
}

private class PropListSort
{
    var mPositions;

    public func initialize(pPosList)
    {
	mPositions = pPosList;
    }

    public func operator() (pP1, pP2)
    {
	var tKey1 = pP1[0]+pP1[1];
	var tKey2 = pP2[0]+pP2[1];
	return(mPositions[tKey1] < mPositions[tKey2]);
    }
}

xOiFuncs();
xOiMath();
xOiDebug();
xOiOAM();

