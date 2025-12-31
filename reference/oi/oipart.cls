// === ofml/oi/oipart.cls ===
package ::ofml::oi;

import ::egr::found::*;
















public class OiPart: xxPart
{
    var mOrderID;
    var mCatInfo;

    var mOiUpdateState;
    var mOiConsistencyState;

    var mOiLang;
    var mOiPriceDate;

    
    
    static var sPartSetGetMembers =
    @(@OrderID, @CatalogInfo, @XArticleSpec, @OrderID, @Planning, 
      @ArticleFeatures2, @AllArticleFeatures, @ArticleFeaturesDescr, 
      @SpecialProductData, @ArticleObj, @OrderUnit, @Origin, @PriceDate);

    
    public func initialize(pFa, pNa)
    {
        xxPart::initialize(pFa, pNa);

	mOrderID = NULL;

	mOiUpdateState = @Up2Date;

	mOiLang = "de";
	mOiPriceDate = NULL;

	var tPlan = getPlanning();

	if (tPlan != NULL) {
	    mOiLang = tPlan.getPDLanguage(self);
	    if (tPlan.getPriceDateMode() == @AnySave)
		mOiPriceDate = tPlan.getPriceDate4Obj(self);
	}
    }

    public func manualSetMembers(pMembers)
    {
	var tRes = xxPart::manualSetMembers(pMembers);

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
	if (tPlan != NULL && !tPlan.isA(OiPlanning))
	    tPlan = NULL;

	return(tPlan);
    }

    public func isCat(pCat)
    {
        if (pCat == @IF_MObject ||
            pCat == @IF_Base ||
            pCat == @IF_Complex ||
            pCat == @IF_Material ||
            pCat == @IF_Property ||
            pCat == @IF_Property2 ||
            pCat == @IF_Article ||
	    pCat == @FREE_MOVING)
            return(1);

        return(0);
    }

    
    public func setObjState(pStateType, pValue)
    {
	oiDebugStartFunc("OiPart::setObjState", [pStateType, pValue]);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (pStateType == @OI_UpdateState)
	    self._setUpdateState(pValue);
	else
	if (pStateType == @OI_ConsistencyState)
	    self._setConsistencyState(pValue);
	else
	    xxPart::setObjState(pStateType, pValue);

	oiDebugFinishFunc(NULL);
    }

    
    public func getObjState(pStateType)
    {
	oiDebugStartFunc("OiPart::getObjState", pStateType);

	var tRet = NULL;

	if (pStateType == @OI_UpdateState)
	    tRet = self._getUpdateState();
	else
	if (pStateType == @OI_ConsistencyState)
	    tRet = self._getConsistencyState();
	else
	    tRet = xxPart::getObjState(pStateType);

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
	oiDebugStartFunc("OiPart::getPropVarCode", pState);

	var tRet = oiGetPropVarCode(self, pState);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    public func isElemCatValid(pCat)
    {
	return (pCat == @TOP_ELEM);
    }

    public func setCMaterial(pCat, pMat)
    {
	if (pCat != @ALL) return(xxPart::setCMaterial(pCat, pMat));

	var tMat = String(pMat);
	if (tMat.size() < 2 || tMat[0] != ':' || tMat[1] != ':')
            tMat = getMatName(pMat);

	oiSetMatRecursiv(self, tMat);

	return(1);
    }

    private func _setUpdateState(pState)
    {
	var tValidStates = @(@Up2Date, @Undefined, @Updatable, @Migratable, 
			     @Invalid);

	if (tValidStates.find(pState) < 0 || mOiUpdateState == pState) return;

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

    public func getPDLanguage()
    {
	return(mOiLang);
    }

    
    public func setPriceDate(pDate)
    {
	oiDebugStartFunc("OiPart::setPriceDate", pDate);

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
	oiDebugStartFunc("OiPart::getPriceDate", NULL);

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
	oiDebugStartFunc("OiPart::setupConfiguration", 
			 [pBaseArticle, pArticleCode, pCodeType, pMigration]);

	var tPDM = self.getPDManager();
	if (tPDM == NULL) {
	    
	    oiDebugPrint(@ExplWarn, 1, 
			 "OiPart::setupConfiguration(): no PD manager!");
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
	oiDebugStartFunc("OiPart::checkUpdatability", pCodeType);

	var tPDM = self.getPDManager();
	if (tPDM == NULL) {
	    
	    oiDebugPrint(@ExplWarn, 1, 
			 "OiPart::checkUpdatability(): no PD manager!");
	    oiDebugFinishFunc(@Invalid);
	    return(@Invalid);
	}
	var tRet = tPDM.checkObjUpdatability(self, pCodeType);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    public func updateConfiguration()
    {
	if (self.isUp2Date()) return;

	oiDebugStartFunc("OiPart::updateConfiguration", NULL);

	oiDebugPrint(@Info, 1, ["self ", self]);

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
	var tUpdateLanguageOnly = pUpdateLanguageOnly[0];

	var tRet = oiUpdateConfiguration2(self, tUpdateLanguageOnly);

	return(tRet);
    }

    
    
    

    
    

    func useForceDynamicProp()
    {
	var tProgInfo = oiGetProgInfo4Obj(self);

	var tRet = 0;

	if (tProgInfo != NULL)
	    tRet = tProgInfo.useForceDynamicProp();

	return(tRet);
    }

    public func forceDynamicProp(pPKey)
    {
	if (OiPart.sPartSetGetMembers.find(pPKey) >= 0 ||
	    OiProperties.sProp2SetGetMembers.find(pPKey) >= 0) 
	    return(1);

	return(xxPart::forceDynamicProp(pPKey));
    }

    public func updateProperties()
    {
	var tPlan = getPlanning();

	var tPlanLang;

	if (tPlan != NULL && self.getObjState(@OI_UpdateState) == @Up2Date &&
	    mOiLang != (tPlanLang = tPlan.getPDLanguage(self))) {
	    if (mOiLang != NULL)
		updateConfiguration2(1);
	    
	    mOiLang = tPlanLang;
	}

	return(1);
    }

    
    public func resetProperties()
    {
	oiResetProperties(self);
    }

    
    public func checkPropValue(pPKey, pPValue, ...)
    {
	oiDebugStartFunc("OiPart::checkPropValue", [pPKey, pPValue]);

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
	oiDebugStartFunc("OiPart::setPropValue", [pPKey, pPValue]);

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

	var tRet = (xxPart::setPropValue(pPKey, pPValue) || tPropDefChanged);

	if (tIsArticle && self.hasProperty(pPKey) &&
	    oiPropChoiceListHasPrices(self, pPKey)) {
	    
	    tRet = 1;
	    oiDebugPrint(@Info, 1, "enforcing editor update for value prices");
	}

	var tChangedProps = self.changedPropList();

	oiDebugPrint(@Info, 2, 
		     ["changed prop vals after assignment: ", tChangedProps]);

	if (!tChangedProps.empty())
	    oiIssueChange(self, @PropertyChange, tChangedProps);

	oiDebugFinishFunc(tRet);
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
	oiDebugStartFunc("OiPart::getVisiblePropValues", NULL);

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

	oiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    public func setVisiblePropValues(pInfo)
    {
	oiDebugStartFunc("OiPart::setVisiblePropValues", pInfo);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (!(pInfo instanceof List)) {
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
	return(OiPart::getPropGroupDescriptions(pLanguage));
    }

    public func getPropGroupDescriptions(pLanguage)
    {
	oiDebugStartFunc("OiPart::getPropGroupDescriptions", pLanguage);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	var tRes;

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

    
    
    

    
    public func checkConsistency()
    {
	return(oiCheckConsistency(self));
    }

    
    public func getArticleObj()
    {
	return(self);
    }

    
    public func getArticleSpec()
    {
	oiDebugStartFunc("OiPart::getArticleSpec", NULL);
        var tRes = getFinalType();
	oiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    public func getXArticleSpec(pMode)
    {
	if (pMode == @Base) return(getArticleSpec());

	var tSpec = NULL;

	var tPDM = getPDManager(); 
	if (tPDM  != NULL) tSpec = tPDM.getXArticleSpec(self, pMode);

	return(tSpec);
    }

    
    public func setArticleSpec(pFType)
    {
	var tPlan = getPlanning();
	if (tPlan != NULL && tPlan.isAtEval()) return;

        setFinalType(pFType);
    }

    
    public func setXArticleSpec(pMode, pSpec)
    {               
	var tPlan = getPlanning();
	if (tPlan != NULL && tPlan.isAtEval()) return;

	if (pMode == @Base)
	    setArticleSpec(pSpec); 
	else {
	
	    var tPDM = getPDManager();
	    if (tPDM != NULL) tPDM.setXArticleSpec(self, pMode, pSpec);
	}
    }

    
    
    
    
    
    
    
    public func getArticleClassifications(pLanguages)
    {
	return(oiGetArticleClassifications(self, pLanguages));
    }

    
    
    public func getArticleAttribute(pAttr)
    {
	return(oiGetArticleAttr(self, pAttr));
    }

    
    public func getOrderUnit(pLanguage)
    {
	return(oiGetOrderUnit(self, pLanguage));
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
	oiDebugStartFunc("OiPart::setCatalogInfo", pInfo);

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

    
    
    

    public func getWidth()
    {
        if (mWidth == NULL) {
            var tBB = getLocalBounds();
            mWidth = tBB[1][0]-tBB[0][0];
        }
        return(mWidth);
    }

    
    public func setWidth(pWidth)
    {
        mWidth = pWidth;
    }

    
    public func getHeight()
    {
        if (mHeight == NULL) {
            var tBB = getLocalBounds();
            mHeight = tBB[1][1]-tBB[0][1];
        }
        return(mHeight);
    }

    
    public func setHeight(pHeight)
    {
        mHeight = pHeight;
    }

    
    public func getDepth()
    {
        if (mDepth == NULL) {
            var tBB = getLocalBounds();
            mDepth = tBB[1][2]-tBB[0][2];
        }
        return(mDepth);
    }

    
    public func setDepth(pDepth)
    {
        mDepth = pDepth;
    }

    
    public func getRecoveryInfo()
    {
	oiDebugStartFunc("OiPart::getRecoveryInfo", NULL);
	oiDebugPrint(@Info, 1, ["self ", self]);
	oiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	var tRes = Hash();
	oiDebugFinishFunc(tRes);
	return(tRes);
    }               

    
    public func isElOrderSubPos(pEl)
    {
	return(0);
    }

    
    public func onTranslate(pOldPos)
    {
    }

    
    public func onRotate(pOldRot)
    {
    }

    
    public func removeValid() 
    {
	return(1);
    }

    
    public func elRemoveValid(pEl)
    {
	return(1);
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

	if (oiIsTmpCh(tEl) || tEl.isA(OiDimensioning2D)) return(0);

	if (!tEl.removeValid() || !elRemoveValid(tEl)) return(-1);

	getPlanning().removeElement(self, tEl);

	return(0);
    }

    rule TRANSLATE(pOldPos)
    {
        public var tF = getFather();

        if (tF.isA(OiPlElement))
            tF.elemTranslation(self, pOldPos);
        else
            onTranslate(pOldPos);

	var tPlan = getPlanning();
	var tChMgr = (tPlan != NULL) ? tPlan.getChangeManager() : NULL;
	if (tChMgr != NULL)
	    tChMgr.changeIssued(self, 
				@ChildTransformed, [@Translated, pOldPos]);
        return(0);
    }

    rule ROTATE(pOldRot)
    {
	if (sRec) return(0);
	sRec = 1;

        public var tF = getFather();

        if (tF.isA(OiPlElement))
            tF.elemRotation(self, pOldRot);
        else
            onRotate(pOldRot);

	sRec = 0;

	var tPlan = getPlanning();
	var tChMgr = (tPlan != NULL) ? tPlan.getChangeManager() : NULL;
	if (tChMgr != NULL)
	    tChMgr.changeIssued(self, @ChildTransformed, [@Rotate, pOldRot]);

        return(0);
    }

    rule PICK (pVal)
    {
	var tPlan = getPlanning();
	var tChMgr = (tPlan != NULL) ? tPlan.getChangeManager() : NULL;
	if (tChMgr != NULL)
	    tChMgr.changeIssued(self, @Picked, NULL);

        return(0);
    }

    rule START_DUMP(pVal)
    {
	var tPlanning = getPlanning();
	var tProgInfo = NULL;
	if (tPlanning != NULL) 
	    tProgInfo = tPlanning.getInfo(getProgram());
	if (tProgInfo != NULL && tProgInfo.canReduceCatalogInfo())
	    oiReduceCatalogInfo(self);

	return (0);
    }

    rule FINISH_DUMP(pDummy)
    {
	oiCompleteCatalogInfo(self);

	return (0);
    }

    
    
    rule START_EVAL(pDummy) 
    {
	oiDebugStartFunc("OiPart::START_EVAL", pDummy);

	oiDebugPrint(@Info, 1, ["self ", self]);

	if (oiHasOldPropRepr(self)) oiOld2NewPropRepr(self);

	oiExtendNewPropRepr(self);

	oiDebugFinishFunc(0);
	return (0);
    }

    rule FINISH_EVAL(pDummy)
    {
	oiDebugStartFunc("OiPart::FINISH_EVAL", pDummy);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tPlan = getPlanning();
	var tPID = self.getProgram();

	oiCompleteCatalogInfo(self);

	if (tPlan != NULL && tPlan.isAtEval()) {
	    mOiUpdateState = @Undefined;
	    xgPH.restorePropertyStates(self, OiFuncs.sSavedMigratablePropStatesDynProp); 
	    if (oiGetAppManufacturerID(tPID) == NULL) {
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

