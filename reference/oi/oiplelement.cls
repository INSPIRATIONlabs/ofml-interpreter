// === ofml/oi/oiplelement.cls ===
package ::ofml::oi;

import ::egr::found::*;















public class OiPlElement: xxScElement
{
    var mOrderID;  
    var mCatInfo;  
    var mWidth, mHeight, mDepth;

    var mOiMeasureMode;
    var mOiPID;
    var mOiDataTbl;

    var mOiUpdateState;
    var mOiConsistencyState;

    var mOiLang;
    var mOiPriceDate;

    static var sHasCategoryEntries; 
    static var sArticleCategories;  
				    

    
    static var sPlElementSetGetMembers =
    @(@CatalogInfo, @PlProgram, @XArticleSpec, @MatPackage, @OrderID, @Planning,
      @MeasureMode, @ArticleFeatures2, @AllArticleFeatures, 
      @ArticleFeaturesDescr, @SpecialProductData, @ArticleObj, @OrderUnit, 
      @PriceDate);

    
    public func initialize(pFa, pNa)
    {
	oiDebugStartFunc("OiPlElement::initialize", [pFa, pNa]);

	oiDebugPrint(@Info, 1, ["self ", self]);

        xxScElement::initialize(pFa, pNa);

	mOrderID = NULL;

	mOiUpdateState = @Up2Date;

	mOiLang = "de";
	mOiPriceDate = NULL;

	var tPlan = getPlanning();

	if (tPlan != NULL) {
	    mOiLang = tPlan.getPDLanguage(self);
	    if (tPlan.getPriceDateMode() == @AnySave)
		mOiPriceDate = tPlan.getPriceDate4Obj(self);
	    else
		oiDebugPrint(@Info, 1, "price date mode is not @AnySave");
	}
	else
	    oiDebugPrint(@Warn, 1, "no planning?!");

	oiDebugPrint(@Info, 1, ["price date: ", mOiPriceDate]);
	oiDebugPrint(@Info, 2, ["PD language: ", mOiLang]);

	oiDebugFinishFunc(NULL);
    }

    public func manualSetMembers(pMembers)
    {
	var tRes = xxScElement::manualSetMembers(pMembers);

	if (!pMembers.hasKey(@mCatInfo))
	    tRes[1].pushBack(@mCatInfo);
	if (!pMembers.hasKey(@mUserBasketPos))
	    tRes[1].pushBack(@mUserBasketPos);
	if (!pMembers.hasKey(@mUserArtTxt))
	    tRes[1].pushBack(@mUserArtTxt);
	if (!pMembers.hasKey(@mUserSalesPrice))
	    tRes[1].pushBack(@mUserSalesPrice);
	if (!pMembers.hasKey(@mUserDiscount))
	    tRes[1].pushBack(@mUserDiscount);
	if (!pMembers.hasKey(@mNeedCUpdate))
	    tRes[1].pushBack(@mNeedCUpdate);
	if (!pMembers.hasKey(@mOiMeasureMode))
	    tRes[1].pushBack(@mOiMeasureMode);
	if (!pMembers.hasKey(@mOiPID))
	    tRes[1].pushBack(@mOiPID);
	if (!pMembers.hasKey(@mOiDataTbl))
	    tRes[1].pushBack(@mOiDataTbl);
	if (!pMembers.hasKey(@mOiUpdateState))
	    tRes[1].pushBack(@mOiUpdateState);
	if (!pMembers.hasKey(@mOiConsistencyState))
	    tRes[1].pushBack(@mOiConsistencyState);
	if (!pMembers.hasKey(@mOiLang))
	    tRes[1].pushBack(@mOiLang);
	if (!pMembers.hasKey(@mOiPriceDate))
	    tRes[1].pushBack(@mOiPriceDate);
	if (!pMembers.hasKey(@mOiSavedPropClassDescr))
	    tRes[1].pushBack(@mOiSavedPropClassDescr);

	
	if (pMembers.hasKey(@mUserBasketPos))
	    tRes[0].pushBack(@mUserBasketPos);
	if (pMembers.hasKey(@mUserArtTxt))
	    tRes[0].pushBack(@mUserArtTxt);
	if (pMembers.hasKey(@mUserSalesPrice))
	    tRes[0].pushBack(@mUserSalesPrice);
	if (pMembers.hasKey(@mUserDiscount))
	    tRes[0].pushBack(@mUserDiscount);
	if (pMembers.hasKey(@mNeedCUpdate))
	    tRes[0].pushBack(@mNeedCUpdate);
	if (pMembers.hasKey(@mOiSavedPropClassDescr))
	    tRes[0].pushBack(@mOiSavedPropClassDescr);

	return (tRes);
    }

    
    
    


    
    public func getPlanning()
    {
	var tPlan = getScene();
	if (tPlan !=  NULL && !tPlan.isA(OiPlanning))
	    tPlan = NULL;

	return(tPlan);
    }

    
    public func setPlProgram()
    {   
	setScProgram();
    }

    public func getProgram()
    {
	

	var tPlan;
	if (mOiPID == NULL && (tPlan = getPlanning()) != NULL) {
	    var tPID = tPlan.getProgram();
	    if (tPID != NULL)
		
		
		self.setProgram(tPlan.getProgram());
	}

	
	return(mOiPID);
    }

    public func setProgram(pPID)
    {
	oiDebugStartFunc("OiPlElement::setProgram", pPID);

	oiDebugPrint(@Info, 1, ["self ", self]);

	mOiPID = pPID;
	var tPlan = getPlanning();
	if (tPlan != NULL) 
	    mOiLang = tPlan.getPDLanguage(self);

	oiDebugPrint(@Info, 2, ["PD language: ", mOiLang]);

	oiDebugFinishFunc(NULL);
    }

    
    public func getPDLanguage()
    {
	return(mOiLang);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func isCat(pCat)
    {
	if (pCat == @IF_MObject || 
	    pCat == @IF_Base || 
	    pCat == @IF_Complex || 
	    pCat == @IF_Property || 
	    pCat == @IF_Property2 || 
	    pCat == @IF_Material ||
	    pCat == @IF_Article ||
	    pCat == @FREE_MOVING ||
	    (pCat == @UNCONSTRAINED_MOVING && 
	     self.getFather() == self.getRoot())) return(1);

	if (pCat == @IF_RecoveredObject) return(0);

	oiDebugStartFunc("OiPlElement::isCat", pCat);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (sHasCategoryEntries == NULL)
	    sHasCategoryEntries = Hash();

	var tPID = getProgram();

	if (tPID == NULL) {
	    oiDebugPrint(@Info, 1, "no OFML program specified!");
	    oiDebugFinishFunc(0);
	    return(0);
	}

	if (sHasCategoryEntries.hasKey(tPID) && !sHasCategoryEntries[tPID]) {
	    oiDebugPrint(@Info, 1, "no entries with type @Category");
	    oiDebugFinishFunc(0);
	    return(0);
	}

	var tArticle = NULL;
	if (self.isCat(@IF_Article))
	    tArticle = self.getArticleObj().getArticleSpec();
	oiDebugPrint(@Info, 2, ["article: ", tArticle]);

	if (tArticle == NULL) {
	    oiDebugPrint(@Info, 1, "no article");
	    oiDebugFinishFunc(0);
	    return(0);
	}

	var tHashKey = String(tPID)+tArticle;

	if (sArticleCategories == NULL)
	    sArticleCategories = Hash();

	if (sArticleCategories.hasKey(tHashKey)) {
	    oiDebugPrint(@Info, 1, "using cache");
	    var tCategories = sArticleCategories[tHashKey];
	    var tRet = (tCategories.find(pCat) >= 0);
	    oiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tTbl = openDataTbl();
	if (tTbl == NULL) {
	    sHasCategoryEntries[tPID] = 0;
	    oiDebugPrint(@Info, 1, "no control data table");
	    oiDebugFinishFunc(0);
	    return(0);
	}

	oiDebugPrint(@Info, 2, ["table ID: ", tTbl.getID()]);

	var tRes = tTbl.readNEntriesFor(@(["type", "@Category"]),
					@("args", "value"), 0);
	if (tRes == NULL) {
	    sHasCategoryEntries[tPID] = 0;
	    oiDebugPrint(@Info, 1, "no entries with type @Category");
	    oiDebugFinishFunc(0);
	    return(0);
	}
	else
	    sHasCategoryEntries[tPID] = 1;

	var tCategories = @();
	var tE;
	foreach(tE; tRes) {
	    var tArgs  = tE[0];
	    var tValue = tE[1];
	    oiDebugPrint(@Info, 2, ["entry value: ", tValue]);

	    if (tValue.empty()) {
		oiDebugPrint(@Warn, 1, "entry with empty value field!?");
		continue;
	    }

	    if (tArgs.empty())
		tArgs = "[]";
	    try {
		tArgs = eval("@("+tArgs+")");
	    }
	    catch(&e: Error) {
		oiDebugPrint(@Warn, 1, ["could not evaluate argument ", 
					 tE[0], ": ", e.what]);
		continue;
	    }

	    if (tValue.empty()) {
		oiDebugPrint(@Warn, 1, "entry with empty argument field!");
		continue;
	    }

	    var tOk = 1;
	    var tCond;
	    foreach(tCond; tArgs) {
		oiDebugPrint(@Info, 2, ["check condition ", tCond]);
		if (tCond.empty()) continue;

		if (tCond[0] == @BaseArticle) {
		    var tMatch = 0;
		    var tArt;
		    foreach(tArt; tCond[1]) {
			if (tArticle.substr(0, tArt.size()) == tArt) {
			    tMatch = 1;
			    break;
			}
			
		    }
		    if (!tMatch) {
			tOk = 0;
			break;
		    }
		}
	    }
	    oiDebugPrint(@Info, 2, ["all conditions ok? ", tOk]);

	    if (!tOk) continue; 

	    if (tValue[0] == '@')
		tValue = tValue.substr(1);
	    var tCategory = NULL;
	    try {
		tCategory = Symbol(tValue);
	    }
	    catch (&e: Error) {
		oiDebugPrint(@EvalErr, 1,
			      ["could not convert value ", tE[1],
			       " for @Category to Symbol!"]);
	    }
	    oiDebugPrint(@Info, 2, ["category: ", tCategory]);

	    if (tCategory != NULL) tCategories.pushBack(tCategory);
	}

	oiDebugPrint(@Info, 2, ["categories: ", tCategories]);

	sArticleCategories[tHashKey] = tCategories;

	var tRet = (tCategories.find(pCat) >= 0);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    public func setObjState(pStateType, pValue)
    {
	oiDebugStartFunc("OiPlElement::setObjState", [pStateType, pValue]);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (pStateType == @OI_UpdateState)
	    self._setUpdateState(pValue);
	else
	if (pStateType == @OI_ConsistencyState)
	    self._setConsistencyState(pValue);
	else
	    xxScElement::setObjState(pStateType, pValue);

	oiDebugFinishFunc(NULL);
    }

    
    public func getObjState(pStateType)
    {
	oiDebugStartFunc("OiPlElement::getObjState", pStateType);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tRet = NULL;

	if (pStateType == @OI_UpdateState)
	    tRet = self._getUpdateState();
	else
	if (pStateType == @OI_ConsistencyState)
	    tRet = self._getConsistencyState();
	else
	    tRet = xxScElement::getObjState(pStateType);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getAddStateCode(pDomain)
    {
	if (pDomain != NULL) return("");

	var tRet = Vector(0);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setAddStateCode(pDomain, pCode)
    {
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getPropVarCode(pState)
    {
	oiDebugStartFunc("OiPlElement::getPropVarCode", pState);

	var tRet = oiGetPropVarCode(self, pState);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    public func getMatPackage()
    {
	oiDebugStartFunc("OiPlElement::getMatPackage", NULL);

	var tPckg = NULL;

	var tTbl = openDataTbl();

	if (tTbl != NULL) {
	    var tRes = tTbl.read1EntryFor(@(["type", "@MatPackage"]),
				          @("args", "value"), 0);
	    if (tRes != NULL) {
	        tPckg = tRes[1];
	        var tLen = tPckg.size();
	        if (tLen > 2 && tPckg.substr(tLen-2, 2) != "::") tPckg += "::";
	    }
	}
	if (tPckg == NULL) {
	    
	    var tPI = oiGetProgInfo4Obj(self);
	    if (tPI != NULL)
		tPckg = tPI.getMatPackage();
	}

	oiDebugFinishFunc(tPckg);
	return(tPckg);
    }

    public func isElemCatValid(pCat)
    {
	return (pCat == @TOP_ELEM);
    }

    public func isValidForCollCheck(pOb)
    {
	if (pOb != NULL && pOb.isA(OiMeasureIAct)) return(0);

	return(xxScElement::isValidForCollCheck(pOb));
    }

    
    
    
    protected func openDataTbl()
    {
	oiDebugStartFunc2("OiPlElement::openDataTbl", NULL);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	if (mOiDataTbl != NULL) {
	    oiDebugPrint(@Info, 1, "using existing handle");
	    
	    
	    
	    
	    
	    mOiDataTbl.close(); 
	    mOiDataTbl.open(); 
	    oiDebugFinishFunc(mOiDataTbl);
	    return(mOiDataTbl);
	}

	mOiDataTbl = oiOpenDataTbl("plelement", getProgram(), NULL);

	oiDebugFinishFunc(mOiDataTbl);
	return(mOiDataTbl);
    }

    public func setCMaterial(pCat, pMat)
    {
	if (pCat != @ALL) return(xxScElement::setCMaterial(pCat, pMat));

	var tMat = String(pMat);
	if (tMat.size() < 2 || tMat[0] != ':' || tMat[1] != ':')
            tMat = getMatName(pMat);

	oiSetMatRecursiv(self, tMat);

	return(1);
    }

    
    public func onCreate(pAngle, pObj, pParams)
    {
	oiDebugStartFunc2("OiPlElement::onCreate", [pAngle, pObj, pParams]);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	xgF.setYRotation(self, pAngle);

	oiDebugFinishFunc(NULL);
    }

    
    
    

    
    public func setWidth(pWidth)
    {
        mWidth = pWidth;
    }

    
    public func getWidth()
    {
	if (mWidth == NULL) {
	    var tBB = getLocalBounds();
	    mWidth = tBB[1][0]-tBB[0][0];
	}

	return(mWidth);
    }

    
    public func setHeight(pHeight)
    {
        mHeight = pHeight;
    }

    
    public func getHeight()
    {
	if (mHeight == NULL) {
	    var tBB = getLocalBounds();
	    mHeight = tBB[1][1]-tBB[0][1];
	}

	return(mHeight);
    }

    
    public func setDepth(pDepth)
    {
        mDepth = pDepth;
    }

    
    public func getDepth()
    {   
	if (mDepth == NULL) {
	    var tBB = getLocalBounds();
	    mDepth = tBB[1][2]-tBB[0][2];
	}

	return(mDepth);
    }

    
    
    

    
    

    public func forceDynamicProp(pPKey)
    {
	if (OiPlElement.sPlElementSetGetMembers.find(pPKey) >= 0 ||
	    OiProperties.sProp2SetGetMembers.find(pPKey) >= 0) 
	    return(1);

	return(xxScElement::forceDynamicProp(pPKey));
    }

    
    func useForceDynamicProp()
    {
	var tProgInfo = oiGetProgInfo4Obj(self);

	var tRet = 0;

	if (tProgInfo != NULL)
	    tRet = tProgInfo.useForceDynamicProp();

	return(tRet);
    }

    public func updateProperties()
    {
	oiDebugStartFunc("OiPlElement::updateProperties", NULL);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	var tPlan = getPlanning();

	oiDebugPrint(@Info, 2, ["current language: ", mOiLang]);

	var tPlanLang;

	if (tPlan != NULL && self.getObjState(@OI_UpdateState) == @Up2Date &&
	    mOiLang != (tPlanLang = tPlan.getPDLanguage(self))) {
	    if (mOiLang != NULL)
		updateConfiguration2(1);
	    
	    
	    
	    
	    
	    
	    mOiLang = tPlanLang;
	}

	oiDebugFinishFunc(1);
	return(1);
    }

    
    public func resetProperties()
    {
	oiResetProperties(self);
    }

    
    public func getPropTitle()
    {
	oiDebugStartFunc("OiPlElement::getPropTitle", NULL);

	var tPlan = getPlanning();
	var tLanguage = "de";
	if (tPlan != NULL)
	    tLanguage = tPlan.getLanguage();

	var tUseArticle, tUseText;

	var tTbl = openDataTbl();
	if (tTbl != NULL) {
	    var tRes = tTbl.read1EntryFor(@(["type", "@Article4PropTitle"]),
					  @("args", "value"), 0);
	    if (tRes != NULL) {
		try {
		    tUseArticle = eval(tRes[1]);
		}
		catch(&e: Error) {
		    oiDebugPrint(@EvalErr, 1,
				 "could not evaluate value for " +
				 "Article4PropTitle: " + e.what);
		}
		if (tUseArticle != NULL &&
		    (!(tUseArticle instanceof Vector) || 
		     tUseArticle.size() != 2)) {
		    oiDebugPrint(@ExplWarn, 1,
				 ["invalid value format for Article4PropTitle:",
				  tUseArticle]);
		    tUseArticle = NULL;
		}
	    }
	    tRes = tTbl.read1EntryFor(@(["type", "@ArtText4PropTitle"]),
				      @("args", "value"), 0);
	    if (tRes != NULL) {
		try {
		    tUseText = eval(tRes[1]);
		}
		catch(&e: Error) {
		    oiDebugPrint(@EvalErr, 1,
				 "could not evaluate value for " +
				 "ArtText4PropTitle: " + e.what);
		}
		var tValidVals = @(@s, @l);
		if (tUseText != NULL &&
		    (!(tUseText instanceof Symbol) || 
		     tValidVals.find(tUseText) < 0)) {
		    oiDebugPrint(@ExplWarn, 1,
				 ["invalid value format for ArtText4PropTitle:",
				  tUseText]);
		    tUseText = NULL;
		}
	    }
	}

	if (tUseArticle == NULL)
	    tUseArticle = [@Base, 0];
	if (tUseText == NULL)
	    tUseText = @s;
	var tSpec = getXArticleSpec(tUseArticle[0]);
	if (tSpec == NULL)
	    tSpec = "";
	if (tSpec.size() > 0 && tUseArticle[1])
	    tSpec = tSpec.substr(0, tUseArticle[1]);

	var tText = getArticleText(tLanguage, tUseText);
	if (tText != NULL) {
	    if (!tText.empty()) tText = tText.front();
	}
	else
	    tText = "";

	var tSep = (tSpec.empty() || tText.empty()) ? "" : "  ";

	var tTitel = tSpec + tSep + tText;

	oiDebugFinishFunc(tTitel);
	return(tTitel);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkPropValue(pPKey, pPValue, ...)
    {
	oiDebugStartFunc("OiPlElement::checkPropValue", [pPKey, pPValue]);

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

    
    public func setPropValue(pPKey, pPValue)
    {
	oiDebugStartFunc("OiPlElement::setPropValue", [pPKey, pPValue]);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tPlan = getPlanning();
	if (tPlan != NULL && tPlan.isAtEval()) {
	    oiDebugPrint(@Warn, 1, "called during scene evaluation!");
	    oiDebugFinishFunc(0);
	    return(0);
	}

	var tCM = tPlan.getCreationMode();
	oiDebugPrint(@Info, 2, ["creation mode: ", tCM]);

	var tIsArticle = self.isCat(@IF_Article);

	var tPropDefChanged = 0;

	if (tIsArticle && tCM == 0 &&
	    (tPropDefChanged = oiUpdate4PropChange(self)) < 0) {
	    oiDebugPrint(@Info, 1, "article has failed to be updated!");
	    oiDebugFinishFunc(0);
	    return(0);
	}

	if (!oiCheckPropValue(self, pPKey, pPValue)) {
	    oiDebugPrint(@Info, 1, "invalid property or value!");
	    oiDebugFinishFunc(0);
	    return(0);
	}

	var tChangedProps = @();
	xgF.copyAggr(self.changedPropList(), tChangedProps, 1);

	var tRet = 
	(xxScElement::setPropValue(pPKey, pPValue) || tPropDefChanged);

	

	if (tIsArticle && self.hasProperty(pPKey) &&
	    oiPropChoiceListHasPrices(self, pPKey)) {
	    
	    
	    tRet = 1;
	    oiDebugPrint(@Info, 1, "enforcing editor update for value prices");
	}

	var tChangedProps2 = self.changedPropList();

	oiDebugPrint(@Info, 2, 
		     ["changed prop vals after assignment: ", tChangedProps2]);
	xgF.copyAggr(tChangedProps2, tChangedProps, 1);

	
	
	

	if (!tChangedProps2.empty()) {
	    
	    
	    
	    
	    
	    
	    
	    
	    
	    
	    oiIssueChange(self, @PropertyChange, tChangedProps2);

	    
	    if (mOiMeasureMode != NULL) measure(mOiMeasureMode);
	}

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    

    

    
    

    
    public func setupProperty(pPKey, pDef, pPos)
    {
	oiSetupProperty(self, pPKey, pDef, pPos);
    }

    
    public func setPropState(pPKey, pState)
    {
	oiDebugStartFunc("OiPlElement::setPropState", [pPKey, pState]);

	oiSetPropState(self, pPKey, oiOld2NewPropState(pState));

	oiDebugFinishFunc(NULL);
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
	oiDebugStartFunc("OiPlElement::getVisiblePropValues", NULL);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tRes = oiGetVisiblePropValues(self);

	if (tRes == NULL && self.isCat(@IF_Article)){
	    var tArtObj = self.getArticleObj();
	    if (tArtObj != NULL) tRes = oiGetVisiblePropValues(tArtObj);
	}

	var tPlan = oiGetPlanning();
        if (tRes != NULL && tPlan != NULL &&
	    tPlan.hasMember(@mustPrintDebugInfo) &&
            tPlan.mustPrintDebugInfo(@Info, 1)) {
	    oiDebugPrint(@Info, 1, "Result:");
	    var tP;
	    foreach(tP; tRes) oiDebugPrint(@Info, 1, ["  ", tP]);
	}

	oiDebugFinishFunc(NULL);
	return(tRes);
    }

    
    public func setVisiblePropValues(pInfo)
    {
	oiDebugStartFunc("OiPlElement::setVisiblePropValues", NULL);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (!(pInfo instanceof List)) {
	    if (pInfo != NULL)
		oiDebugPrint(@Warn, 1, "unexpected (invalid) parameter");
	    oiDebugFinishFunc(NULL);
	    return;
	}

	var tP;
	foreach(tP; pInfo) {
	    var tPKey = tP[0];
	    var tChL  = tP[1];
	    oiSetPropChoiceList(self, tPKey, tChL);
	}

	oiDebugFinishFunc(NULL);
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    

    
    public func getPropClassDescriptions(pLanguage)
    {
	oiDebugStartFunc("OiPlElement::getPropClassDescriptions", pLanguage);

	oiDebugPrint(@Info, 1, ["self ", self]);

	
	
	var tRes = OiPlElement::getPropGroupDescriptions(pLanguage);

	oiDebugFinishFunc(tRes);
	return(tRes);

    }

    
    public func getPropGroupDescriptions(pLanguage)
    {
	oiDebugStartFunc("OiPlElement::getPropGroupDescriptions", pLanguage);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	var tRes = @();

	if (self.getObjState(@OI_UpdateState) != @Up2Date)
	    oiDebugPrint(@Info, 1, "article instance is not up-2-date");
	else 
	    tRes = oiGetPropGroupDescriptions(self, pLanguage);

	oiDebugFinishFunc(tRes);
	return(tRes);
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
	oiDebugStartFunc("OiPlElement::setPropState2", [pPKey, pState]);

	oiSetPropState(self, pPKey, pState);

	oiDebugFinishFunc(NULL);
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
	return(oiGetAllPropSpecs(self, pRequiredState));
    }

    
    public func getAllPropValueInfos(pPKey, pPClass)
    {
	return(oiGetAllPropValueInfos(self, pPKey, pPClass));
    }

    
    public func getDefaultPropGroupSpecs()
    {
	return(oiGetDefaultPropGroupSpecs(self));
    }

    
    
    

    
    public func measure(pMode)
    {
	oiDebugStartFunc2("OiPlElement::measure", pMode);

	var tValidModes = @(@UPDATE, @TOP, @FRONT, @LEFT, @RIGHT);

	if (tValidModes.find(pMode) < 0) {
	    oiDebugPrint(@Warn, 1, "invalid mode");
	    oiDebugFinishFunc(NULL);
	    return;
	}

	var tWOff, tHOff, tDOff;
	tWOff = tHOff = tDOff = 0.1;

	var tWTextOff, tHTextOff, tDTextOff;
	tWTextOff = tHTextOff = tDTextOff = 0.0;

	var tNewMode = pMode;

	var tWTxt, tHTxt, tDTxt;

	if (pMode == @UPDATE) {
	    tNewMode = mOiMeasureMode;
	    if (oiExists(self.getName()+".measure_height")) {
		tHOff = self.measure_height.getLineOffset();
		tHTextOff = self.measure_height.getTextOffset();
		tHTxt = self.measure_height.getText();
	    }
	    if (oiExists(self.getName()+".measure_depth")) {
		tDOff = self.measure_depth.getLineOffset();
		tDTextOff = self.measure_depth.getTextOffset();
		tDTxt = self.measure_depth.getText();
	    }
	    if (oiExists(self.getName()+".measure_width")) {
		tWOff = self.measure_width.getLineOffset();
		tWTextOff = self.measure_width.getTextOffset();
		tWTxt = self.measure_width.getText();
	    }
	}

	unMeasure();

	mOiMeasureMode = tNewMode;

	oiDebugPrint(@Info, 2, ["persistent mode: ", mOiMeasureMode]);

	var tLB = getLocalGeoBounds();
	var tW  = tLB[1][0] - tLB[0][0];
	var tH  = tLB[1][1] - tLB[0][1];
	var tD  = tLB[1][2] - tLB[0][2];

	if (tWTxt == NULL)
	    tWTxt = oiGetLengthInAppUnit(tW);
	if (tHTxt == NULL)
	    tHTxt = oiGetLengthInAppUnit(tH);
	if (tDTxt == NULL)
	    tDTxt = oiGetLengthInAppUnit(tD);

	if (mOiMeasureMode == @FRONT) {
	    OiMeasureIAct(self, @measure_height, tH, tHTxt, 
			  @YFL, [tHOff, tHTextOff], NULL, NULL)
	    .setPosition([tLB[0][0], tLB[0][1], tLB[1][2]]);
	    OiMeasureIAct(self, @measure_width,  tW, tWTxt, 
			  @XFB, [tWOff, tWTextOff], NULL, NULL)
	    .setPosition([tLB[0][0], tLB[0][1], tLB[1][2]]);
	}
	else
	if (mOiMeasureMode == @LEFT) {
	    OiMeasureIAct(self, @measure_height, tH, tHTxt, 
			  @YLB, [tHOff, tHTextOff], NULL, NULL)
	    .setPosition([tLB[0][0], tLB[0][1], tLB[0][2]]);
	    OiMeasureIAct(self, @measure_depth,  tD, tDTxt, 
			  @ZLB, [tDOff, tDTextOff], NULL, NULL)
	    .setPosition([tLB[0][0], tLB[0][1], tLB[0][2]]);
	}
	else
	if (mOiMeasureMode == @RIGHT) {
	    OiMeasureIAct(self, @measure_height, tH, tHTxt, 
			  @YRB, [tHOff, tHTextOff], NULL, NULL)
	    .setPosition([tLB[1][0], tLB[0][1], tLB[0][2]]);
	    OiMeasureIAct(self, @measure_depth,  tD, tDTxt, 
			  @ZRB, [tDOff, tDTextOff], NULL, NULL)
	    .setPosition([tLB[1][0], tLB[0][1], tLB[0][2]]);
	}
	else
	if (mOiMeasureMode == @TOP) {
	    OiMeasureIAct(self, @measure_width, tW, tWTxt, 
			  @XTB, [tWOff, tWTextOff], NULL, NULL)
	    .setPosition([tLB[0][0], tLB[1][1], tLB[0][2]]);
	    OiMeasureIAct(self, @measure_depth,  tD, tDTxt, 
			  @ZTL, [tDOff, tDTextOff], NULL, NULL)
	    .setPosition([tLB[0][0], tLB[1][1], tLB[0][2]]);
	}

	oiDebugFinishFunc(NULL);
    }

    
    public func unMeasure()
    {
	if (oiExists(self.getName()+".measure_height"))
	    remove(self.measure_height);
	if (oiExists(self.getName()+".measure_depth"))
	    remove(self.measure_depth);
	if (oiExists(self.getName()+".measure_width"))
	    remove(self.measure_width);

	mOiMeasureMode = NULL;
    }

    
    public func getMeasureMode()
    {
	return(mOiMeasureMode);
    }

    
    
    

    private func _setUpdateState(pState)
    {
	oiDebugStartFunc2("OiPlElement::_setUpdateState", pState);

	var tValidStates = @(@Up2Date, @Undefined, @Updatable, @Migratable, 
			     @Invalid);

	oiDebugPrint(@Info, 2, ["current state: ", mOiUpdateState]);

	if (tValidStates.find(pState) < 0 || mOiUpdateState == pState) {
	    oiDebugPrint(@Info, 1, "nothing to do");
	    oiDebugFinishFunc(NULL);
	    return;
	}

	var tOldState  = mOiUpdateState;
	mOiUpdateState = pState;

	if (mOiUpdateState == @Migratable || mOiUpdateState == @Invalid) {
	    
	    if (mOiUpdateState == @Migratable) {
		
		
		xgPH.savePropertyStates(self, OiFuncs.sSavedMigratablePropStatesDynProp);
	    }
	    invalidateProperties();
	}

	if (mOiUpdateState == @Up2Date) {
	    var tPlan = getPlanning();
	    var tChMgr = (tPlan != NULL) ? tPlan.getChangeManager() : NULL;
	    if (tChMgr != NULL)
		tChMgr.changeIssued(self, @ArticleUpdated, tOldState);
	}

	oiDebugFinishFunc(NULL);
    }

    private func _getUpdateState()
    {
	return(mOiUpdateState);
    }

    
    public func isUp2Date()
    {
	return(mOiUpdateState == @Up2Date);
    }

    private func _setConsistencyState(pState)
    {
	mOiConsistencyState = pState;
    }

    private func _getConsistencyState()
    {
	return(mOiConsistencyState);
    }

    
    
    
    
    public func setPriceDate(pDate)
    {
	oiDebugStartFunc("OiPlElement::setPriceDate", pDate);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tAObj;

	if (self.isCat(@IF_Article) && 
	    (tAObj = self.getArticleObj()) != NULL && tAObj != self) {
	    oiDebugPrint(@Info, 1, ["delegating to article ", tAObj]);
	    tAObj.setPriceDate(pDate);
	    oiDebugFinishFunc(NULL);
	    return;
	}

	if (oiIsValidDate(pDate))
	    mOiPriceDate = pDate;
	else
	    oiDebugPrint(@Warn, 1, "ignoring invalid date!");

	oiDebugFinishFunc(NULL);
    }

    
    
    
    
    public func getPriceDate()
    {
	oiDebugStartFunc("OiPlElement::getPriceDate", NULL);

	var tDate = mOiPriceDate;
	var tAObj;

	if (self.isCat(@IF_Article) && 
	    (tAObj = self.getArticleObj()) != NULL && tAObj != self) {
	    oiDebugPrint(@Info, 1, ["delegating to article ", tAObj]);
	    tDate = tAObj.getPriceDate();
	}

	oiDebugFinishFunc(tDate);
	return(tDate);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setupConfiguration(pBaseArticle, pArticleCode, pCodeType,
				   pMigration)
    {
	oiDebugStartFunc("OiPlElement::setupConfiguration", 
			 [pBaseArticle, pArticleCode, pCodeType, pMigration]);

	var tPDM = self.getPDManager();
	if (tPDM == NULL) {
	    
	    oiDebugPrint(@ExplWarn, 1, 
			 "OiPlElement::setupConfiguration(): no PD manager!");
	    mOiUpdateState = @Invalid;
	    oiDebugFinishFunc(0);
	    return(0);
	}

	mOiUpdateState = @Undefined;
	    
	    
	    
	    
	    

	var tRet = tPDM.setupConfiguration(self, pBaseArticle, pArticleCode, 
				           pCodeType, pMigration);
	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkUpdatability(pCodeType)
    {
	oiDebugStartFunc("OiPlElement::checkUpdatability", pCodeType);

	var tPDM = self.getPDManager();
	if (tPDM == NULL) {
	    
	    oiDebugPrint(@ExplWarn, 1, 
			 "OiPlElement::checkUpdatability(): no PD manager!");
	    oiDebugFinishFunc(@Invalid);
	    return(@Invalid);
	}
	var tRet = tPDM.checkObjUpdatability(self, pCodeType);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func updateConfiguration()
    {
	oiDebugStartFunc("OiPlElement::updateConfiguration", NULL);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (self.isUp2Date()) {
	    oiDebugPrint(@Info, 1, "article is up-to-date");
	    oiDebugFinishFunc(NULL);
	    return;
	}

	var tComposite = oiGetCompositeArticleUp(self);

	oiDebugPrint(@Info, 2, ["composite father ", tComposite]);

	if (tComposite != NULL)
	    tComposite.updateConfiguration();
	else
	    oiUpdateConfiguration(self);

	oiDebugFinishFunc(NULL);
    }

    
    
    
    
    public func updateGeometry()
    {
    }

    
    public func updateConfiguration2(pUpdateLanguageOnly, ...)
    {
	oiDebugStartFunc("OiPlElement::updateConfiguration2", 
			 pUpdateLanguageOnly);

	var tUpdateLanguageOnly = pUpdateLanguageOnly[0];

	var tRet = oiUpdateConfiguration2(self, tUpdateLanguageOnly);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    public func checkConsistency()
    {
	return(oiCheckConsistency(self));
    }

    
    public func getArticleFeatures2(pLanguage)
    {
	var tPDM;
	if ((tPDM = getPDManager()) != NULL)
	    return(tPDM.getArticleFeatures2(self, pLanguage));

	return(NULL);
    }

    
    public func getAllArticleFeatures(pLanguage)
    {
	var tPDM;
	if ((tPDM = getPDManager()) != NULL)
	    return(tPDM.getAllArticleFeatures(self, pLanguage));

	return(NULL);
    }

    
    public func getArticleFeaturesDescr(pType, pLanguage)
    {
	return(oiGetArticleFeaturesDescr(self, pType, pLanguage));
    }

    
    
    
    
    
    public func getSpecialProductData(pType, pLanguage, pArgs)
    {       
	return(NULL);
    }

    
    public func getArticleObj()
    {
	return(self);
    }

    
    public func getArticleSpec()
    {
	oiDebugStartFunc2("OiPlElement::getArticleSpec", NULL);
	var tRes = getFinalType();
	oiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    public func getArticleParams()
    {
        var tPDM = getPDManager();
        if (tPDM == NULL || !tPDM.hasMember(@getArticleParams))
            return (NULL);

        return (tPDM.getArticleParams(self));
    }

    
    public func getXArticleSpec(pMode)
    {
	if (pMode == @Base) return(getArticleSpec());

	var tSpec = NULL;

	var tPDM = getPDManager();
	if (tPDM != NULL) tSpec = tPDM.getXArticleSpec(self, pMode);

	return(tSpec);
    }

    
    public func setArticleSpec(pFType)
    {
	var tPlan = getPlanning();
	if (tPlan != NULL && tPlan.isAtEval()) return;

	if (tPlan != NULL && mOiLang == NULL)
	    
	    
	    
	    
	    mOiLang = tPlan.getPDLanguage(self);

	setFinalType(pFType);
	
	
	
	
    }

    
    public func setXArticleSpec(pMode, pSpec)
    {
	oiDebugStartFunc("OiPlElement::setXArticleSpec", [pMode, pSpec]);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tPlan = getPlanning();
	if (tPlan != NULL && tPlan.isAtEval()) {
	    oiDebugFinishFunc("at scene evaluation");
	    return;
	}

	if (pMode == @Base) 
	    setArticleSpec(pSpec);
	else {
	
	    var tPDM = getPDManager();
	    if (tPDM != NULL) tPDM.setXArticleSpec(self, pMode, pSpec);
	}

	oiDebugPrint(@Info, 2, ["PD language: ", mOiLang]);

	oiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    public func getArticleClassifications(pLanguages)
    {
	oiDebugStartFunc("OiPlElement::getArticleClassifications", pLanguages);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tRet = oiGetArticleClassifications(self, pLanguages);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticleAttribute(pAttr)
    {
	return(oiGetArticleAttr(self, pAttr));
    }

    
    public func getOrderUnit(pLanguage)
    {
	return(oiGetOrderUnit(self, pLanguage));
    }

    
    public func setOrderID(pID)
    {
	mOrderID = pID;
	setChanged();
    }
    
    
    public func getOrderID()
    {
	return(mOrderID);
    }

    
    public func setCatalogInfo(pInfo)
    {
	oiDebugStartFunc("OiPlElement::setCatalogInfo", pInfo);

	oiDebugPrint(@Info, 1, ["self ", self]);

	mCatInfo = pInfo;
	setChanged();

	oiDebugFinishFunc(NULL);
    }
    
    
    public func getCatalogInfo()
    {
	return(mCatInfo);
    }

    
    public func getGLDFInfo()
    {
	return(NULL);
    }

    
    
    

    
    public func getRecoveryInfo()
    {
	oiDebugStartFunc("OiPlElement::getRecoveryInfo", NULL);
	oiDebugPrint(@Info, 1, ["self: ", self]);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	var tRes = Hash();
	oiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    public func isElOrderSubPos(pEl)
    {
	return(0);
    }

    
    public func checkElPos(pChild, pOldPos)
    {
	oiDebugStartFunc("OiPlElement::checkElPos", [pChild, pOldPos]);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	var tRet = (checkChildColl(pChild, NULL) == NULL);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    public func elemTranslation(pObj, pOld)
    {
	if (pObj.isA(OiSymbol2D) || pObj.isA(OiText2D) ||
	    pObj.isA(OiDimensioning2D)) {
	    return (0);
	}

	oiDebugStartFunc("OiPlElement::elemTranslation", [pObj, pOld]);

	oiDebugPrint(@Info, 1, ["self: ", self]);
	oiDebugPrint(@Info, 1, ["new pos: ", pObj.getPosition()]);

	if (pObj.isA(OiPlElement)) {
	    var tIsValid = pObj.translateValid(pOld);
	    oiDebugPrint(@Info, 1, ["translateValid: ", tIsValid]);
	    if (tIsValid) {
		
		var tReady = pObj.translated(pOld);

		oiDebugPrint(@Info, 1, ["translated: ", tReady]);
		if (!tReady) {
		    
		    var tPos = pObj.getPosition();
		    var tOK = checkElPos(pObj, pOld);

		    if (!tOK) {
		    	if (pObj.isCat(@WALL_ELEM))
			    pObj.setPosition(pOld);
			else {
			    var tNewPos = xgF.translateAlongAxes(pObj, pOld);
			    xgF.translateAsFarAsPossible(pObj, tNewPos, tPos);
			}
		    }
		}
	    }
	    else
		pObj.setPosition(pOld);
	}
	else
	if (pObj.isA(OiPart)) {
	    pObj.onTranslate(pOld);
	}

	oiDebugPrint(@Info, 1, ["current pos: ", pObj.getPosition()]);

	oiDebugFinishFunc(0);
	return(0);
    }
 
    
    public func elemRotation(pObj, pOld)
    {
	if (pObj.isA(OiPlElement)) {
	    if(pObj.rotateValid(pOld)) {
		
		var tReady = pObj.rotated(pOld);

		if (!tReady) {
		    
		    if (!checkElPos(pObj, pObj.getPosition())) {
			pObj.setRotationA(pOld);
		    }
		}
	    }
	    else
		pObj.setRotationA(pOld);
	}
	else
	if (pObj.isA(OiPart)) {
	    pObj.onRotate(pOld);
	}
	return(0);
    }

    
    public func elRemoveValid(pEl)
    {
	return(1);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getAppInteractorDefs(pArgs)
    {
	oiDebugStartFunc("OiPlElement::getAppInteractorDefs", NULL);
	oiDebugPrint(@Info, 1, ["self ", self]);

	var tRet = @();

	var tMode = getAppInteractorDefMode(pArgs);

	oiDebugPrint(@Info, 2, ["mode: ", tMode]);

	if (tMode == @Auto)
	    traverseAutoAppInteractors(self, tRet);
	else
	if (tMode == @Interface)
	    traverseIFAppInteractors(self, tRet);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    private func getAppInteractorDefMode(pArgs)
    {
	oiDebugStartFunc2("OiPlElement::getAppInteractorDefMode", pArgs);

	var tTbl = openDataTbl();
	if (tTbl == NULL) {
	    oiDebugPrint(@Info, 1, "no control data table");
	    oiDebugFinishFunc(@None);
	    return(@None);
	}

	oiDebugPrint(@Info, 2, ["table ID: ", tTbl.getID()]);

	var tRes = tTbl.readNEntriesFor(@(["type", "@AppInteractorDefs"]),
					@("args", "value"), 0);
	if (tRes == NULL) {
	    oiDebugPrint(@Info, 1, "no entries with type @AppInteractorDefs");
	    oiDebugFinishFunc(@None);
	    return(@None);
	}

	var tValidValues = @(@None, @Auto, @Interface);
	var tRet = NULL;
	var tE;
	foreach(tE; tRes) {
	    var tArgs  = tE[0];
	    if (tArgs.empty())
		tArgs = "[]";
	    try {
		tArgs = eval("@("+tArgs+")");
	    }
	    catch(&e: Error) {
		oiDebugPrint(@Warn, 1, ["could not evaluate argument ", 
					 tE[0], ": ", e.what]);
		continue;
	    }
	    var tOk = 1;
	    var tCond;
	    foreach(tCond; tArgs) {
		oiDebugPrint(@Info, 2, ["check condition ", tCond]);
		if (tCond.empty()) continue;

		if (tCond[0] == @SelfType) {
		    var tMatch = 0;
		    var tT;
		    foreach(tT; tCond[1]) {
			var tType = NULL;
			try {
			    tType = eval(tT);
			}
			catch(&e: Error) {
			    oiDebugPrint(@Warn, 1, 
					  ["could not evaluate type '", tT, 
					   "': ", e.what]);
			}
			if (tType != NULL && self.isA(tType)) {
			    tMatch = 1;
			    break;
			}
		    }
		    if (!tMatch) {
			tOk = 0;
			break;
		    }
		}
		else
		if (tCond[0] == @SelfCategory) {
		    var tMatch = 0;
		    var tCat;
		    foreach(tCat; tCond[1]) {
			if (self.isCat(tCat)) {
			    tMatch = 1;
			    break;
			}
		    }
		    if (!tMatch) {
			tOk = 0;
			break;
		    }
		}
		else
		if (tCond[0] == @SelfArticle) {
		    var tMatch = 0;
		    var tCheckArt = NULL;
		    if (self.isCat(@IF_Article))
			tCheckArt = self.getArticleObj().getArticleSpec();
		    oiDebugPrint(@Info, 2, ["check article: ", tCheckArt]);
		    if (tCheckArt == NULL && !tCond[1].empty()) {
			tOk = 0;
			break;
		    }
		    var tArt;
		    foreach(tArt; tCond[1]) {
			if (tCheckArt.substr(0, tArt.size()) == tArt) {
			    tMatch = 1;
			    break;
			}
		    }
		    if (!tMatch) {
			tOk = 0;
			break;
		    }
		}
	    }
	    oiDebugPrint(@Info, 2, ["all conditions ok? ", tOk]);

	    if (!tOk) continue; 

	    var tValue = tE[1];
	    if (tValue[0] == '@')
		tValue = tValue.substr(1);
	    var tMode = NULL;
	    try {
		tMode = Symbol(tValue);
	    }
	    catch (&e: Error) {
		oiDebugPrint(@EvalErr, 1,
			      ["could not convert value ", tE[1],
			       " for @AppInteractorDefs to Symbol!"]);
	    }
	    if (tValidValues.find(tMode) < 0) {
		oiDebugPrint(@EvalErr, 1,
			      ["invalid value ", tMode, 
			       " for option @AppInteractorDefs!"]);
		tMode = NULL;
	    }
	    oiDebugPrint(@Info, 2, ["effective mode of entry: ", tMode]);

	    if (tMode == NULL) continue;

	    if (tRet == NULL)
		tRet = tMode;
	    else {
		oiDebugPrint(@Warn, 1, "multiple matching entries for " +
				       "option @AppInteractorDefs!");
		tRet = NULL;
		break;
	    }
	}

	if (tRet == NULL) tRet = @None;

	oiDebugFinishFunc(tRet);
        return(tRet);
    }

    private func traverseAutoAppInteractors(pObj, pRetList)
    {
	oiDebugStartFunc2("OiPlElement::traverseAutoAppInteractors", pObj);

	var tCh;
	foreach(tCh;  pObj.getChildren()) {
	    if (tCh.hasMember(@isSelectable) && tCh.isSelectable()) {
		var tOffset = NULL;
		var tTrAxis = tCh.getTrAxis();
		var tRtAxis = tCh.getRtAxis();
		oiDebugPrint(@Info, 2, ["check ", tCh, 
					" -> TrAxis=", tTrAxis, 
					", RtAxis=", tRtAxis]);
		var tType = NULL;
		if (tRtAxis == 0 && tTrAxis != 0) {
		    tType = @Translate;
		    if (tTrAxis == 4 && tCh.isCat(@DRAWER)) {
			
			
			var tBB = tCh.getLocalBounds();
			tOffset = [(tBB[0][0]+tBB[1][0])/2,
				   (tBB[0][1]+tBB[1][1])/2,
				   tBB[1][2]];
		    }
		}
		else
		if (tTrAxis == 0 &&
		    (tRtAxis == 1 || tRtAxis == 2 || tRtAxis == 4)) {
		    tType = @Rotate;
		    if (tRtAxis == 1)
			tOffset = [0.0, 0.0, 0.05];
		    else
			tOffset = [-0.05, 0.0, 0.0];
		}
		else
		    tType = @Select;

		if (tType != NULL) {
		    var tDef = [tType, tCh.getName(), NULL, tOffset, NULL];
		    oiDebugPrint(@Info, 1, ["adding ", tDef]);
		    pRetList.pushBack(tDef);
		}
	    }
	    traverseAutoAppInteractors(tCh, pRetList);
	}
	oiDebugFinishFunc(NULL);
    }

    private func traverseIFAppInteractors(pObj, pRetList)
    {
	oiDebugStartFunc2("OiPlElement::traverseIFAppInteractors", pObj);

	var tValidTypes = @(@Translate, @Orbit, @Rotate, @Select, @Method, 
			    @TextInput, @Resize);
	var tCh;
	foreach(tCh;  pObj.getChildren()) {
	    if (tCh.hasMember(@isCat) && tCh.isCat(@IF_AppInteractorRef)) {
		var tDef = tCh.getInteractorDef();
		var tType   = tDef[0];
		var tPic    = tDef[1];
		var tOffset = tDef[2];
		var tArg    = tDef[3];
		if (tValidTypes.find(tType) < 0) {
		    oiDebugPrint(@Warn, 1, 
				 ["invalid interaction type ", tType]);
		}
		else {
		    var tDef = [tType, tCh.getName(), tPic, tOffset, tArg];
		    oiDebugPrint(@Info, 1, ["adding ", tDef]);
		    pRetList.pushBack(tDef);
		}
	    }
	    traverseIFAppInteractors(tCh, pRetList);
	}
	oiDebugFinishFunc(NULL);
    }

    

    rule NEW_ELEMENT (pValue)
    {
	var tEl = $pValue;

	if (!oiIsTmpCh(tEl) && !tEl.isA(OiDimensioning2D)) 
	    getPlanning().newElement(self, tEl);

	return(0);
    }

    rule REMOVE_ELEMENT (pValue)
    {
	var tEl = $pValue;

	oiDebugStartFunc("OiPlElement::REMOVE_ELEMENT", tEl);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	if (oiIsTmpCh(tEl) || tEl.isA(OiDimensioning2D)) {
	    oiDebugFinishFunc(0);
	    return(0); 
	}

	if (!tEl.removeValid()) {
	    oiDebugPrint(@Info, 1, ["removeValid() of ", tEl, " says: no"]);
	    oiDebugFinishFunc(-1);
	    return(-1);
	}

	if (!elRemoveValid(tEl)) {
	    oiDebugPrint(@Info, 1, "elRemoveValid() says: no");
	    oiDebugFinishFunc(-1);
	    return(-1);
	}

	getPlanning().removeElement(self, tEl);

	oiDebugFinishFunc(0);
	return(0);
    }

    rule PICK (pVal)
    {
	oiDebugStartFunc("OiPlElement::PICK", pVal);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	var tPlan = getPlanning();
	var tChMgr = (tPlan != NULL) ? tPlan.getChangeManager() : NULL;
	if (tChMgr != NULL)
	    tChMgr.changeIssued(self, @Picked, NULL);

	oiDebugFinishFunc(0);
	return (0);
    }

    rule TRANSLATE (pOld)
    {
	
	

	var tPlan = getPlanning();
	var tChMgr = (tPlan != NULL) ? tPlan.getChangeManager() : NULL;
	if (tChMgr != NULL)
	    tChMgr.changeIssued(self, @ChildTransformed, [@Translate, pOld]);

	return (0);
    }

    rule ROTATE (pOld)
    {
	
	

	var tPlan = getPlanning();
	var tChMgr = (tPlan != NULL) ? tPlan.getChangeManager() : NULL;
	if (tChMgr != NULL)
	    tChMgr.changeIssued(self, @ChildTransformed, [@Rotate, pOld]);

	return (0);
    }

    rule START_DUMP(pVal)
    {
	var tProgInfo = oiGetProgInfo4Obj(self);
	if (tProgInfo != NULL && tProgInfo.canReduceCatalogInfo())
	    oiReduceCatalogInfo(self);

	if (mOiDataTbl != NULL) {
	    mOiDataTbl.close();
	    mOiDataTbl = NULL;
	}

	return (0);
    }

    rule FINISH_DUMP(pDummy)
    {
	oiCompleteCatalogInfo(self);

	return (0);
    }

    
    
    rule START_EVAL(pDummy) 
    {
	oiDebugStartFunc("OiPlElement::START_EVAL", pDummy);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (oiHasOldPropRepr(self)) oiOld2NewPropRepr(self);

	oiExtendNewPropRepr(self);

	oiDebugFinishFunc(0);
	return (0);
    }

    rule FINISH_EVAL(pDummy)
    {
	oiDebugStartFunc("OiPlElement::FINISH_EVAL", pDummy);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tPlan = getPlanning();
	var tPID = self.getProgram();

	oiCompleteCatalogInfo(self);

	oiDebugPrint(@Info, 2, ["planning at eval?: ", 
		     tPlan != NULL && tPlan.isAtEval()]);

	

	if (tPlan != NULL && tPlan.isAtEval()) {
	    mOiUpdateState = @Undefined;
	    
	    
	    xgPH.restorePropertyStates(self, OiFuncs.sSavedMigratablePropStatesDynProp);
	    if (tPID != NULL && oiGetAppManufacturerID(tPID) == NULL) {
		
		
		oiDebugPrint(@Info, 1, 
			     ["library ", tPID, " is not registered"]);
		mOiUpdateState = @Invalid;
	        invalidateProperties();
	    }
	    
	    
	    

	    if (!tPlan.isAtEval())
		oiIssueChange(self, @LoadedElement, NULL);
	}

	if (mOiLang == NULL) {
	    if (tPlan != NULL)
	        mOiLang = tPlan.getPDLanguage(self);
	    else
	        mOiLang = "de";
	}

	oiDebugFinishFunc(0);
	return (0);
    }
}

OiGlobal();
OiFuncs();
OiProperties();

