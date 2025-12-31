// === ofml/xoi/xoiplgroup.cls ===
package ::ofml::xoi;
import ::ofml::oi::*;

import ::stdio::*;
























































public class xOiPlGroup: xOiPlElement
{
    static var sXoiDuringArticleInit = 0;
    static var sXoiDuringCheckAdd = 0;
    static var sXoiDuringCommonPropSet = 0;

    static var sXoiAddStatePDLanguage = NULL;
    static var sXoiEnforcePropsUpdate = 0;

    static var sXoiObjects4ElCatSelStateUpdate;

    static var sErrInvalidDim =
    "Attempt to assign a new dimension to a planning group.";

    static var sDummyMatName = "::ofml::xoi::PlGroupDummy";

    
    var mWidth0, mHeight0, mDepth0;

    
    var mXoiCheckAddState;

    
    
    
    var mXoiDoFs;

    
    
    
    
    
    
    
    
    var mXoiInsertMode;

    
    
    

    
    var mAtEval;

    
    
    var mAtDump;

    
    
    
    
    
    
    
    
    
    
    
    
    
    var mXoi2DMode;

    var mXoi2DSymID;

    
    var mXoiElCatSelectable;

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func initialize(pFa, pNa)
    {
	xOiPlElement::initialize(pFa, pNa);

	mWidth = NULL;
	mHeight = NULL;
	mDepth = NULL;

	mWidth0 = 1.0;
	mHeight0 = 1.0;
	mDepth0 = 1.0;

	mAtEval = 0;
	mAtDump = 0;

	mXoiDoFs = Hash();

	mXoi2DMode = @Combined;

	mXoi2DSymID = NULL;

	mXoiElCatSelectable = Hash();

	showGroupDummy();

	var tPlan = getPlanning();
	if (tPlan != NULL) {
	    tPlan.getChangeManager().register(self, @(@PropertyChange),
					      [@(self), 0]);
	    tPlan.getChangeManager().register(self, @(@MetaMainChildPropChange),
					      [@(self), 0]);
	    tPlan.getChangeManager().register(self, @(@ArticleInserted),
					      [@(self), 0]);
	    tPlan.getChangeManager().register(self, @(@ElementRemoval),
					      [@(self), 0]);
	    tPlan.getChangeManager().register(self, @(@ChildTransformed), 
					      [@(self), 0]);
	}

	sXoiDuringArticleInit = 0;
    }

    public func manualSetMembers(pMembers)
    {
	var tRes = xOiPlElement::manualSetMembers(pMembers);

	if (!pMembers.hasKey(@mAtDump))
	    tRes[1].pushBack(@mAtDump);
	if (!pMembers.hasKey(@mXoiDoFs))
	    tRes[1].pushBack(@mXoiDoFs);
	if (!pMembers.hasKey(@mXoiInsertMode))
	    tRes[1].pushBack(@mXoiInsertMode);
	if (!pMembers.hasKey(@mXoi2DMode))
	    tRes[1].pushBack(@mXoi2DMode);
	if (!pMembers.hasKey(@mXoi2DSymID))
	    tRes[1].pushBack(@mXoi2DSymID);
	if (!pMembers.hasKey(@mXoiCheckAddState))
	    tRes[1].pushBack(@mXoiCheckAddState);
	if (!pMembers.hasKey(@mXoiElCatSelectable))
	    tRes[1].pushBack(@mXoiElCatSelectable);

	
	if (pMembers.hasKey(@mAtCollect))
	    tRes[0].pushBack(@mAtCollect);
	if (pMembers.hasKey(@mRebuildCommonProps))
	    tRes[0].pushBack(@mRebuildCommonProps);
	if (pMembers.hasKey(@mChangedEL))
	    tRes[0].pushBack(@mChangedEL);

	return (tRes);
    }

    
    
    
    
    
    public func localObjName(pObj)
    {
	var tMyName  = self.getName();
	var tMySize  = tMyName.size();
	var tObjName = pObj.getName();

	if (tObjName.size() <= tMySize ||
	    tObjName.substr(0, tMySize) != tMyName)
	    return(NULL);

	return(tObjName.substr(tMySize+1));

	

	if (self.getChildren().find(pObj) < 0)
	    return(NULL);

	return(pObj.getName().substr(self.getName().size()+1));
    }

    
    
    
    public func localName2Obj(pName)
    {
	var tObj = NULL;

	try {
	    tObj = eval(self.getName()+"."+pName);
	}
	catch (&e: Error) {
	    xOiDebugPrint(@ExplWarn, 1,
			  ["invalid local name (", e.where, ": ", e.what, ")"]);
	}

	return(tObj);
    }

    protected func readOptionInsertMode(pPID, pTblName, pArticleSpec)
    {
	xOiDebugStartFunc("xOiPlGroup::readOptionInsertMode", 
			  [pPID, pTblName, pArticleSpec]);

	var tTbl = xOiOpenDataTbl(pTblName, pPID,
				  xOiGetAppProgInfoDBPath(pPID));

	if (tTbl == NULL) {
	    xOiDebugPrint(@Warn, 1, "couldn't open the table!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tValue = xOiGetMatchingEntryValue(tTbl, "@InsertMode",
					      pArticleSpec, Int, @(0, 1, 2));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or no valid value");

	xOiCloseDataTbl(tTbl);

	xOiDebugFinishFunc(tValue);
	return(tValue);
    }

    
    
    
    
    protected func mustShowEmptyGroupDummy()
    {
	return(1);
    }

    func showGroupDummy()
    {
	if (!mustShowEmptyGroupDummy()) return;

	xOiDebugStartFunc("xOiPlGroup::showGroupDummy", NULL);

	try {remove(self.dummy);}

	OiBlock(self, @dummy, [mWidth0, mHeight0, mDepth0])
	.setCMaterial(@ANY, Symbol(sDummyMatName));

	xOiDebugPrint(@Info, 2, ["symbol ID: ", mXoi2DSymID]);

	if (mXoi2DSymID != NULL) 
	    
	    
	    try{delete2DObj(mXoi2DSymID);}

	invalidatePicture();

	var tPoints = [[0.0, 0.0], [mWidth0, 0.0],
		       [mWidth0, -mDepth0], [0.0, -mDepth0]];
	mXoi2DSymID = new2DObj(0, @LINE_LOOP, tPoints);

	xOiDebugPrint(@Info, 2, ["symbol ID: ", mXoi2DSymID]);

	xOiDebugFinishFunc(NULL);
    }

    func hideGroupDummy()
    {
	try {remove(self.dummy);}

	if (mXoi2DSymID != NULL) {
	    try{delete2DObj(mXoi2DSymID);}
	    mXoi2DSymID = NULL;
	}

	invalidatePicture();
    }

    func setWidth(pWidth)
    {
        afLog(10, "XOI", sErrInvalidDim);
    }

    func getWidth()
    {
	if (mWidth != NULL) return(mWidth);

	var tBB = getLocalBounds();
	return(tBB[1][0]-tBB[0][0]);
    }

    func setHeight(pHeight)
    {
        afLog(10, "XOI", sErrInvalidDim);
    }

    func getHeight()
    {
	if (mHeight != NULL) return(mHeight);

	var tBB = getLocalBounds();
	return(tBB[1][1]-tBB[0][1]);
    }

    func setDepth(pDepth)
    {
        afLog(10, "XOI", sErrInvalidDim);
    }

    func getDepth()
    {
	if (mDepth != NULL) return(mDepth);

	var tBB = getLocalBounds();
	return(tBB[1][2]-tBB[0][2]);
    }

    private func planningHasBorder()
    {
	var tPlan = getPlanning();
	if (tPlan == NULL) return(0);

	var tBorder = tPlan.getBorder();

	if (tBorder == NULL) return(0);

	var tDim = tBorder[1];

	if (tDim == NULL) return(0);

	if (tDim[0] < eps || tDim[1] < eps || tDim[2] < eps) return(0);

	return(1);
    }

    public func isCat(pCat)
    {
	if (pCat == @ORDER_GROUP) return(1);
	if (pCat == @PlGroup) return(1);

	return(xOiPlElement::isCat(pCat));
    }

    public func getPictureInfo()
    {
	
	if (mXoi2DMode == @Combined)
	    return([typeOf(self).getFullName(), @TRAVERSAL_CONT, @SHARE_OFF]);
	if (mXoi2DMode == @Elements)
	    return([NULL, @TRAVERSAL_CONT, @SHARE_OFF]);
	if (mXoi2DMode == @Group)
	    return([typeOf(self).getFullName(), @TRAVERSAL_STOP, @SHARE_OFF]);
	if (mXoi2DMode == @Rectangle)
	    return([NULL, @TRAVERSAL_STOP, @SHARE_OFF]);

	return(xOiPlElement::getPictureInfo());
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func update2DRepr()
    {
	xOiDebugStartFunc("xOiPlGroup::update2DRepr", NULL);

	xOiDebugPrint(@Info, 2, ["2D representation type: ", mXoi2DMode]);

	xOiDebugPrint(@Info, 2, ["symbol ID: ", mXoi2DSymID]);

	if (mXoi2DSymID != NULL) {
	    try {delete2DObj(mXoi2DSymID);} 
	    mXoi2DSymID = NULL;
	}

	invalidatePicture();

	if (mXoi2DMode != @Rectangle) {
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tBB =  self.getLocalBounds();
	
	xOiDebugPrint(@Info, 2, ["bounds: ", tBB]);
	var tPoints = [[tBB[0][0], -tBB[0][2]], [tBB[1][0], -tBB[0][2]],
		       [tBB[1][0], -tBB[1][2]], [tBB[0][0], -tBB[1][2]]];
	mXoi2DSymID = new2DObj(0, @LINE_LOOP, tPoints);

	xOiDebugPrint(@Info, 2, ["symbol ID: ", mXoi2DSymID]);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getElDistance(pObj, pAttPt, ...)
    {
	xOiDebugStartFunc("xOiPlGroup::getElDistance", [pObj, pAttPt]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tEL   = self.getElements();
	if (tEL.find(pObj) < 0) {
	    xOiDebugPrint(@Warn, 1, "given instance is not a group element!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tAttPt = pAttPt[0];

	if (!(tAttPt instanceof Symbol || 
	      (tAttPt instanceof Vector && tAttPt.size() >= 3))) {
	    xOiDebugPrint(@Warn, 1, "invalid attach point parameter!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tAttPtDef = NULL;
	if (tAttPt instanceof Symbol) {
	    var tStdAttPts = pObj.getStdAttPts();
	    var tAddAttPts = pObj.getAddAttPts();
	    if (tAddAttPts.find(tAttPt) < 0 && tStdAttPts.find(tAttPt) < 0)
		xOiDebugPrint(@Warn, 1, "given instance does not possess " +
					"specified attach point!");
	    else {
		tAttPtDef = oiGetAttPtDef(pObj, tAttPt);
		xOiDebugPrint(@Info, 2, ["attach point def: ", tAttPtDef]);
	    }
	}
	else {
	    var tPos  = tAttPt[0];
	    var tRot  = tAttPt[2];
	    var tMode = @Sibling;
	    if (tAttPt.size() >= 4 && tAttPt[3] instanceof Symbol) 
		tMode = tAttPt[3];
	    if (tPos.size() == 3 &&
		tPos[0] instanceof Float &&
		tPos[1] instanceof Float &&
		tPos[2] instanceof Float &&
		tRot instanceof Float)
		tAttPtDef = [tPos, tAttPt[1], tRot, tMode];
	    else
		xOiDebugPrint(@Warn, 1, "invalid attach point definition!");
	}

	if (tAttPtDef == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tAPPos = tAttPtDef[0];
	var tAPRot = tAttPtDef[2];
	var tMode  = tAttPtDef[3];

	if (tMode != @Sibling) {
	    xOiDebugPrint(@Warn, 1, ["invalid attach point mode ", tMode]);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tOffset = [0.0, 0.0, 0.0];

	if (pAttPt.size() > 1) {
	    tOffset = pAttPt[1];
	    if (!(tOffset instanceof Vector &&
		  tOffset.size() == 3 &&
		  tOffset[0] instanceof Float &&
		  tOffset[1] instanceof Float &&
		  tOffset[2] instanceof Float)) {
		xOiDebugPrint(@Warn, 1, "invalid offset (using default)!");
		tOffset = [0.0, 0.0, 0.0];
	    }
	}

	xOiDebugPrint(@Info, 2, ["offset for AP pos: ", tOffset]);

	tAPPos = [tAPPos[0]+tOffset[0], 
		  tAPPos[1]+tOffset[1], 
		  tAPPos[2]+tOffset[2]];

	xOiDebugPrint(@Info, 2, ["attach point position: ", tAPPos]);
	xOiDebugPrint(@Info, 2, ["attach point rotation: ", tAPRot*180/xOiPi]);

	var tObjPos = pObj.getPosition();
	var tObjRot = pObj.getRotation(@PY);

	xOiDebugPrint(@Info, 2, ["object's position: ", tObjPos]);
	xOiDebugPrint(@Info, 2, ["object's rotation: ", tObjRot*180/xOiPi]);

	var tXYZ = xOiTransformXZCoords(tObjPos, tObjRot, tAPPos[0], tAPPos[2]);

	tAPPos = [tXYZ[0], tObjPos[1]+tAPPos[1], tXYZ[2]];
	tAPRot = tObjRot + tAPRot;

	xOiDebugPrint(@Info, 2, ["applied AP position: ", tAPPos]);
	xOiDebugPrint(@Info, 2, ["applied AP rotation: ", tAPRot*180/xOiPi]);

	var tMyPos = self.getPosition();
	var tMyRot = self.getRotation(@PY);

	xOiDebugPrint(@Info, 2, ["my position: ", tMyPos]);
	xOiDebugPrint(@Info, 2, ["my rotation: ", tMyRot*180/xOiPi]);

	tXYZ = xOiTransformXZCoords(tMyPos, tMyRot, tAPPos[0], tAPPos[2]);

	var tPos = [tXYZ[0], tMyPos[1]+tAPPos[1], tXYZ[2]];

	var tRot = tMyRot + tAPRot;
	var tSearchV = [cos(tRot), 0, -sin(tRot)];

	xOiDebugPrint(@Info, 2, ["search position: ", tPos]);
	xOiDebugPrint(@Info, 2, ["search rotation: ", tRot*180/xOiPi]);
	xOiDebugPrint(@Info, 2, ["search vector:   ", tSearchV]);

	var tRet = oiGetDistance(tPos, tSearchV);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkElDistance(pOp, pCheckVal, pObj, pAttPt, ...)
    {
	xOiDebugStartFunc("xOiPlGroup::checkElDistance", 
		          [pOp, pCheckVal, pObj, pAttPt]);

	var tValidOps = @(@GE, @GT, @LE, @LT);

	if (!(pOp instanceof Symbol && tValidOps.find(pOp) >= 0)) {
	    xOiDebugPrint(@Warn, 1, "invalid operator!");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	var tAttPt = pAttPt[0];
	var tOffset = [0.0, 0.0, 0.0];
	if (pAttPt.size() > 1)
	    tOffset = pAttPt[1];

	var tDist = getElDistance(pObj, tAttPt, tOffset);

	if (tDist == NULL) {
	    xOiDebugPrint(@Warn, 1, "invalid parameter(s)!");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	var tRet = 0;

	if (pOp == @GE) {
	    if (tDist < 0.0 || tDist > pCheckVal-xOiEps)
		tRet = 1;
	}
	else
	if (pOp == @GT) {
	    if (tDist < 0.0 || 
	        (tDist > pCheckVal && fabs(tDist-pCheckVal) > xOiEps))
		tRet = 1;
	}
	else
	if (pOp == @LE) {
	    if (tDist >= 0.0 && tDist < pCheckVal+xOiEps)
		tRet = 1;
	}
	else { 
	    if (tDist >= 0.0 && 
		(tDist < pCheckVal && fabs(tDist-pCheckVal) > xOiEps))
		tRet = 1;
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    public func getMatName(pMat)
    {
	if ((pMat instanceof Symbol) && String(pMat) == sDummyMatName)
	    return(sDummyMatName);

	return(xOiPlElement::getMatName(pMat));
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    public func receivePropertyChange(pPublisher, pArgs)
    {
	xOiDebugStartFunc("xOiPlGroup::receivePropertyChange", 
			  [pPublisher, pArgs]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	if (mXoi2DMode == @Rectangle) update2DRepr();
	else
	if (mXoi2DMode != @Elements) invalidatePicture();

	_receivePropertyChange(pPublisher, pArgs);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    public func receiveMetaMainChildPropChange(pPublisher, pArgs)
    {
	xOiDebugStartFunc("xOiPlGroup::receiveMetaMainChildPropChange", 
			  [pPublisher, pArgs]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	_receivePropertyChange(pPublisher, @(pArgs));

	xOiDebugFinishFunc(NULL);
    }

    private func _receivePropertyChange(pPublisher, pProps)
    {
	xOiDebugStartFunc2("xOiPlGroup::_receivePropertyChange", 
			   [pPublisher, pProps]);

	if (pPublisher.isCat(@IF_MetaType) &&
            (!pPublisher.isMetaInitialized() || 
	     (pPublisher.hasMember(@sInSetAddStateCode) &&
	      pPublisher.sInSetAddStateCode))) {
	    xOiDebugPrint(@Info, 1, 
			  "nothing to do during Meta type initialization");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	if (sXoiDuringArticleInit) {
	    xOiDebugPrint(@Info, 1, 
			  "nothing to do during article initialization");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	
	if (sXoiDuringCommonPropSet) {
	    
	    xOiDebugPrint(@Info, 1, 
			  "nothing to do during setCommonPropsDeep");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	if (duringSetAddStateCode()) {
	    xOiDebugPrint(@Info, 1, "nothing to do during setAddStateCode()");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	if (pPublisher == self) {
	    xOiDebugPrint(@Info, 1, "nothing to do for self");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	if (!_xOiGetOptionAllObjs4CommonProps()) {
	    xOiDebugPrint(@Info, 1, 
			  "nothing to do as @AllObjs4CommonProps is false (0)");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	var tCommonPropObjs = getObjs4OptionCommonProps();

	if (tCommonPropObjs.find(pPublisher) < 0) {
	    xOiDebugPrint(@Info, 1, 
			  "publisher is not relevant for option @CommonProps");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	var tCommonProps = _xOiGetOptionCommonProps();
	var tP, tUpdate = 0;
	foreach(tP; pProps) {
	    if (tCommonProps.find(tP) >= 0) {
		xOiDebugPrint(@Info, 2, ["changed property ", tP, " is ",
					 "contained in option @CommonProps"]);
		tUpdate = 1;
		break;
	    }
	}

	if (tUpdate) {
	    var tCPH = xOiCommonPropsHandler(self);
	    tCPH.initCommonProps(NULL, 1); 
	}
	else
	    xOiDebugPrint(@Info, 2, "none of changed properties is " +
				    "contained in option @CommonProps");

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func receiveArticleInserted(pPublisher, pArgs)
    {
	xOiDebugStartFunc("xOiPlGroup::receiveArticleInserted",
			  [pPublisher, pArgs]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	if (mXoi2DMode == @Rectangle) update2DRepr();
	else
	if (mXoi2DMode != @Elements) invalidatePicture();

	if (pPublisher == self) {
	    sXoiDuringArticleInit = 0;
	    
	    
	    
	    
	    
	    
	    
	    
	    
	    
	}
	else
	if (_xOiGetOptionAllObjs4CommonProps() && !sXoiDuringCommonPropSet) {
	    
	    
	    var tCPH = xOiCommonPropsHandler(self);
	    tCPH.initCommonProps(NULL, 0);
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func receiveElementRemoval(pPublisher, pArgs)
    {
	xOiDebugStartFunc("xOiPlGroup::receiveElementRemoval",
			  [pPublisher, pArgs]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	if (pPublisher == self) {
	    xOiDebugPrint(@Info, 1, "nothing to do for self");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	

	if (_xOiGetOptionAllObjs4CommonProps() && !duringSetAddStateCode() &&
	    !sXoiDuringCommonPropSet) {
	    
	    
	    
	    var tCPH = xOiCommonPropsHandler(self);
	    tCPH.initCommonProps(NULL, 0);
	}

	

	var tCatHash = _xOiGetElementCatHash();

	var tName = NULL;
	var tEL   = self.getElements();
	var tIsEl = (tEL.find(pPublisher) >= 0);

	if (tIsEl || isGrandSubArticle(pPublisher))
	    tName = localObjName(pPublisher);

	if (tName != NULL)
	    _xOiRemoveElementCat(tName, NULL, tCatHash);

	if (tIsEl && pPublisher.isCat(@IF_CompositeArticle)) {
	    var tSubID;
	    foreach(tSubID; pPublisher.getSubArticleIDs()) {
		tName = localObjName(pPublisher.getSubArticle(tSubID));
		_xOiRemoveElementCat(tName, NULL, tCatHash);
	    }
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    public func receiveChildTransformed(pPublisher, pArgs)
    {
	if (mXoi2DMode == @Rectangle) update2DRepr();
	else
	if (mXoi2DMode != @Elements) invalidatePicture();
    }

    
    
    

    public func setArticleSpec(pArticle)
    {
	sXoiDuringArticleInit = 1;
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func getArticleFeatures4El(pEl, pLanguage)
    {
	xOiDebugStartFunc("xOiPlGroup::getArticleFeatures4El", 
			  [pEl, pLanguage]);

	if (!pEl.isCat(@IF_Article)) {
	    xOiDebugPrint(@Warn, 1, "given element is not an article?!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tEl = pEl;
	if (tEl.getArticleSpec() == NULL)
	    tEl = tEl.getArticleObj();

	xOiDebugPrint(@Info, 2, ["effective article instance: ", tEl]);

	var tPlan = oiGetPlanning();
	var tPDM = NULL;
	if (tPlan != NULL)
	    tPDM = tPlan.getPDManager();

	if (tPDM == NULL) {
	    xOiDebugPrint(@Warn, 1, "no PD manager?!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tArticleFeatures4Common = _xOiGetOptionArticleFeatures4Common();
	var tCheckPropConds = (tArticleFeatures4Common instanceof Vector);
	xOiDebugPrint(@Info, 2, 
		      ["@ArticleFeatures4Common: ", tArticleFeatures4Common]);

	var tCPH = xOiCommonPropsHandler(self);

	var tCommonNativeProps = @();
	var tP;
	foreach(tP; getFixProperties()) {
	    if (!tEl.hasProperty(tP)) {
		xOiDebugPrint(@Info, 1, [tEl, " has no common prop ", tP]);
		continue;
	    }
	    if (tCheckPropConds) {
		var tAddFeature = 
		    tCPH.checkPropConditions(tArticleFeatures4Common, tP);
		if (tAddFeature == NULL) {
		    xOiDebugPrint(@Info, 1,
				  [tP, " doesn't match any prop cond for ",
				   "option @ArticleFeatures4Common: ", 
				   tArticleFeatures4Common]);
		    tAddFeature = 0; 
		}
		xOiDebugPrint(@Info, 2, 
			      ["effective option @ArticleFeatures4Common: ",
			       tAddFeature]);
		if (!tAddFeature)
		    continue;
	    }
	    var tPName = tPDM.propKey2PropName(tEl, tP);
	    xOiDebugPrint(@Info, 2, ["prop name for ", tP, " is ", tPName]);
	    if (tPName == NULL)
		xOiDebugPrint(@Warn, 1, ["no native prop for common ", tP]);
	    else
		tCommonNativeProps.pushBack(tPName);
	}

	if (tCommonNativeProps.empty()) {
	    xOiDebugPrint(@Info, 1, "no common native properties");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tHasTransaction = tPDM.hasTransaction(self);
	if (tHasTransaction)
	    tPDM.finishTransaction(self);

	var tFeatures = @();
	var tElF;
	foreach(tElF; tEl.getArticleFeatures2(pLanguage)) {
	    xOiDebugPrint(@Info, 2, ["check feature: ", tElF]);
	    if (tCommonNativeProps.find(tElF[0]) < 0)
		continue;
	    var tPropTxt = tElF[1];
	    var tValTxt  = tElF[3];
	    tFeatures.pushBack([tPropTxt, tValTxt]);
	}

	if (tHasTransaction)
	    tPDM.startTransaction(self, NULL);

	xOiDebugFinishFunc(tFeatures);
	return(tFeatures);
    }

    
    
    
    
    
    
    public func XXXsetPriceDate(pDate)
    {
	

	if (!xOiIsValidDate(pDate)) {
	    
	    
	    return(NULL);
	}

	xOiPlElement::setPriceDate(pDate);

	var tE;
	foreach(tE; getElements()) {
	    if (tE.hasMember(@isCat) && tE.isCat(@IF_Article)) {
		
		tE.setPriceDate(pDate);
	    }
	}

	
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkConsistency()
    {
	xOiDebugStartFunc("xOiPlGroup::checkConsistency", NULL);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tMyPriceDate = self.getPriceDate();

	xOiDebugPrint(@Info, 2, ["my price date: ", tMyPriceDate]);

	var tPlan = oiGetPlanning();
	var tLang = tPlan.getPDLanguage(self);
	var tLog  = tPlan.getErrorLog();

	var tInconsCnt = 0;
	var tInconsCntNotSel = 0;
	var tNotSelMessages = @();
	var tE;
	foreach(tE; getElements()) {
	    if (!(tE.isA(OiPlElement) || tE.isA(OiPart))) {
		xOiDebugPrint(@Warn, 1, [tE, " has no interface Article!"]);
		continue;
	    }
	    var tElPriceDate = tE.getPriceDate();
	    xOiDebugPrint(@Info, 2, 
			  ["current price date of ", tE, " is ", tElPriceDate]);
	    if (tElPriceDate != tMyPriceDate) {
		xOiDebugPrint(@Info, 1, ["assigning new price date ", 
					 tMyPriceDate, " to ", tE]);
		tE.setPriceDate(tMyPriceDate);
	    }

	    if (!tE.checkConsistency()) {
		tInconsCnt++;
		if (!tE.isSelectable()) {
		    
		    
		    var tElLogKey = tE.getOrderID();
		    if (tElLogKey == NULL)
			tElLogKey = tE.getName();
		    if (!tLog.hasKey(tElLogKey))
			
			xOiDebugPrint(@Warn, 1, 
				     ["no log messages for inconsistent ", 
				      tE, " ?!"]);
		    else {
			tInconsCntNotSel++;
			var tElMessages = tLog[tElLogKey];
			var tLogMsg;
			foreach(tLogMsg; tElMessages) {
			    var tMsg = tLogMsg[0];
			    if (tNotSelMessages.find(tMsg) < 0)
				tNotSelMessages.pushBack(tMsg);
			}
		    }
		}
	    }
	}

	if (tInconsCnt == 0) {
	    xOiDebugFinishFunc(1);
	    return(1);
	}

	if (tLog == NULL) {
	    xOiDebugPrint(@Warn, 1, "no global log?!");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	

	var tPublisher = self.getName();
	var tLogKey = self.getOrderID();
	if (tLogKey == NULL)
	    tLogKey = tPublisher;
	if (!tLog.hasKey(tLogKey)) tLog[tLogKey] = @();

	var tLogErrors = tLog[tLogKey];

	if (tInconsCntNotSel) {
	    xOiDebugPrint(@Info, 2, 
			  ["log messages for not selectable elements: ",
			   tNotSelMessages]);
	    if (tNotSelMessages.empty())
		
		xOiDebugPrint(@Warn, 1, "no log messages for not selectable " +
					"inconsistent elements !?");
	    else {
		var tM;
		foreach(tM; tNotSelMessages) {
		    var tMsg = oiGetStringResource(tM, tLang);
		    tLogErrors.pushBack([tMsg, tPublisher, 
					 "xOiPlGroup::checkConsistency()"]);
		}
	    }
	}

	var tInconsCntSelectable = tInconsCnt - tInconsCntNotSel;

	if (tInconsCntSelectable > 0) {
	    var tMsg = 
		oiGetStringResource("::ofml::xoi::@InconsistentElement", 
				    tLang);
	    if (tInconsCntSelectable > 1)
		tMsg = 
		oiGetStringResource(["::ofml::xoi::@InconsistentElements", 
				     tInconsCntSelectable], 
				    tLang);
	    tLogErrors.pushBack([tMsg, tPublisher, 
				 "xOiPlGroup::checkConsistency()"]);
	}

	xOiDebugFinishFunc(0);
	return(0);
    }

    public func markInconsistency()
    {
	if (oiExists(getName()+".dummy"))
	    self.dummy.setCMaterial(@ANY, Symbol("::ofml::xoi::InconsBBox"));
	else
	if (!oiExists(self.getName()+"._incons_bbox"))
	    xOiBBox(self, @_incons_bbox, getLocalBounds())
	    .setCMaterial(@ANY, @InconsBBox);
    }

    public func unmarkInconsistency()
    {
	if (oiExists(getName()+".dummy"))
	    self.dummy.setCMaterial(@ANY, Symbol(sDummyMatName));
	else
	    if (oiExists(getName()+"._incons_bbox"))
		remove(self._incons_bbox);
    }

    
    
    
    
    
    
    
    
    
    
    public func updateConfiguration()
    {
	
	

	if (self.getArticleSpec() == NULL)
	    self.setObjState(@OI_UpdateState, @Up2Date);
	else
	    xOiPlElement::updateConfiguration();
    }

    public func onCreate(pRot, pObj, pParams)
    {
	
	
	if (getElements().empty()) showGroupDummy();

	xOiPlElement::onCreate(pRot, pObj, pParams);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getAddStateCode(pDomain)
    {
	xOiDebugStartFunc("xOiPlGroup::getAddStateCode", NULL);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tRes = "";

	if (pDomain == NULL || pDomain != @xOiPlGroup)
	    tRes = xOiPlElement::getAddStateCode(pDomain);

	if (pDomain != NULL && pDomain != @xOiPlGroup) {
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	 }

	var tProps = @();
	var tP;
	foreach(tP; getFixProperties()) {
	    if (!hasProperty(tP)) {
		xOiDebugPrint(@Info, 1, 
			      ["currently not defined property ", tP]);
		continue;
	    }

	    var tDef = getPropertyDef(tP);

	    if ((tDef = getPropertyDef(tP)) == NULL) {
		xOiDebugPrint(@Warn, 1, "no prop def?!");
		continue;
	    }

	    
	    
	    
	    
	    
	    
	    
	    
	    
	    
	    
	    
	    

	    var tNewDef = getPropDef2(tP);

	    xOiDebugPrint(@Info, 2, 
			  ["check property ", tP, ": ", tDef, " / ", tNewDef]);

	    var tPos   = getPropertyPos(tP);
	    var tState = getPropState(tP);
	    var tValue = getPropValue(tP);
	    var tClass = getPropClass(tP);

	    var tChL4Code = NULL;

	    if (tNewDef[3] != @None) {
		var tChLNew = NULL;
		try {
		    tChLNew = getPropChoiceList(tP, 0);
		} catch (&e: Error) {
		    
		    
		    xOiDebugPrint(@Warn, 1, 
				  ["error during call of getPropChoiceList(): ",
				   e.what]);
		}
		xOiDebugPrint(@Info, 3, 
			      ["choicelist (new interface): ", tChLNew]);
		if (tChLNew instanceof String) {
		    xOiDebugPrint(@Info, 2, 
				  ["saving choicelist function: ", tChLNew]);
		    tChL4Code = tChLNew;
		}
		else 
		if (tChLNew != NULL) {
		    var tChLOld = xOiGetValuesFromChoiceList(tDef[4]);
		    xOiDebugPrint(@Info, 3, 
			          ["choicelist (old interface): ", tChLOld]);
		    if (tChLOld.size() < tChLNew.size()) {
			
			xOiDebugPrint(@Info, 2, 
				      ["saving choicelist : ", tChLNew]);
			tChL4Code = tChLNew;
		    }
		}
	    }

	    var tKey   = tP; 

	    tProps.pushBack([tKey, tDef, tPos, tState, tValue, tClass, 
			     tChL4Code]);
	}

	var tAddState = [@V3, xOiHash2List(mXoiDoFs), mXoiInsertMode, 
			 mXoi2DMode, tProps, self.getPDLanguage(), 
		         xOiHash2List(mXoiElCatSelectable)]; 

	
	
	

	

	var tMyCode = xOiParam2Str(tAddState, 0, 1);

	if (pDomain == @xOiPlGroup)
	    tRes = tMyCode;
	else
	if (pDomain == NULL && !tMyCode.empty())
	    tRes.pushBack([@xOiPlGroup, tMyCode]);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setAddStateCode(pDomain, pCode)
    {
	xOiDebugStartFunc("xOiPlGroup::setAddStateCode", [pDomain, pCode]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	sXoiObjects4ElCatSelStateUpdate = Hash();

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
		if (tDomain == @xOiPlGroup)
		    tMyCode = tCode;
		else
		    tBaseCode.pushBack([tDomain, tCode]);
	    }
	}
	else {
	    if (pDomain != @xOiPlGroup)
		tBaseCode = pCode;
	    else
		tMyCode = pCode;
	}

	if (pDomain == NULL || pDomain != @xOiPlGroup)
	    xOiPlElement::setAddStateCode(pDomain, tBaseCode);

	xOiDebugPrint(@Info, 1, ["my code: ", tMyCode]);
 
	if (tMyCode == NULL || tMyCode.empty()) {
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tAddState = NULL;

	try {
	    tAddState = eval(tMyCode);
	}
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 1, 
			  ["invalid code (", e.where, ": ", e.what, ")"]);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	xOiDebugPrint(@Info, 2, ["add state: ", tAddState]);
 
	var tVersion = @V0;
	if (tAddState[0] instanceof Symbol) 
	    tVersion = tAddState[0];

	var tValidVersions = @(@V0, @V1, @V2, @V3);
	if (tValidVersions.find(tVersion) < 0)
	    
	    
	    xOiDebugPrint(@Warn, 1, ["unexpected code version ", tVersion]);

	if (tVersion == @V0) {
	    mXoiDoFs = xOiList2Hash(tAddState[0]);
	    mXoiInsertMode = tAddState[1];
	    mXoi2DMode = tAddState[2];

	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tVersionNum = Int(String(tVersion).substr(1));

	

	mXoiDoFs = xOiList2Hash(tAddState[1]);
	mXoiInsertMode = tAddState[2];
	mXoi2DMode = tAddState[3];

	var tProps = tAddState[4];

	var tDynProps = getDynamicProps();
	var tP;
	foreach(tP; tProps) {
	    
	    
	    xOiDebugPrint(@Info, 2, ["processing property ", tP]);
	    var tKey  = tP[0];
	    var tDef  = tP[1];
	    var tType = tDef[4];

	    if (tP.size() >= 7 && tP[6] instanceof String && 
		!xOiPropIsCHF(tType)) {
		if ((tType.size() > 1 && tType.substr(0, 2) == "ch") ||
		    (tType.size() > 3 && tType.substr(0, 4) == "u ch")) {
		    tType = "chf "+ tP[6];
		    xOiDebugPrint(@Info, 2, 
				  ["replacing explicite choice list (", tDef[4],
				   ") with function -> ", tType]);
		    tDef = [tDef[0], tDef[1], tDef[2], tDef[3], tType];
		    
		    
		    
		    
		    
		    
		    
		}
		else
		    xOiDebugPrint(@Warn, 1, ["property ", tKey, 
					     " has no explicite choice list (",
					     tType, ") ?!"]);
	    }
	    self.setupProperty(tKey, tDef, tP[2]);
	    self.setPropState(tKey, tP[3]);
	    tDynProps[tKey] = tP[4];
	    if (tP.size() >= 6)
		self.setPropClass(tKey, tP[5]);
	    if (tP.size() >= 7 && tP[6] instanceof List) {
		xOiDebugPrint(@Info, 2, ["replacing choicelist ", tP[6]]);
		self.setPropChoiceList(tKey, tP[6]);
	    }
	}

	
	if (tAddState.size() >= 6)
	    sXoiAddStatePDLanguage = tAddState[5];
	
	    sXoiEnforcePropsUpdate = 1;

	if (tVersionNum >= 3) 
	    mXoiElCatSelectable = xOiList2Hash(tAddState[6]);

	xOiDebugFinishFunc(NULL);
    }

    
    
    func duringSetAddStateCode()
    {
	return(0);
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getSubArticleIDs()
    {
	xOiDebugStartFunc("xOiPlGroup::getSubArticleIDs", NULL);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tRet = xOiGetSubArticleIDs2(self, 
					self.getSubArticleElements(),
				        self.getInvalidSubArticleSpecs());
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    protected func getSubArticleElements()
    {
	return(@());
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func getInvalidSubArticleSpecs()
    {
	return(@());
    }

    
    
    
    
    
    
    
    
    public func getSubArticle(pID)
    {
	xOiDebugStartFunc("xOiPlGroup::getSubArticle", pID);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	
	var tObj = xOiGetSubArticle(self, pID);
	

	xOiDebugFinishFunc(tObj);
	return(tObj);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func isGrandSubArticle(pObj, ...)
    {
	xOiDebugStartFunc("xOiPlGroup::isGrandSubArticle", pObj);

	var tObj = pObj[0];
	var tNeedInfo = 0;
	if (pObj.size() > 1 && pObj[1] instanceof Int)
	    tNeedInfo = pObj[1];

	xOiDebugPrint(@Info, 1, ["self ", self]);
	xOiDebugPrint(@Info, 2, ["need info? ", tNeedInfo]);

	var tMyChildren = self.getChildren();
	if (tMyChildren.find(tObj) >= 0) {
	    xOiDebugPrint(@Info, 1, 
			  "given instance is an immediate child of the group");
	    xOiDebugFinishFunc(tNeedInfo ? NULL : 0);
	    return(tNeedInfo ? NULL : 0);
	}

	var tRet = (tNeedInfo ? NULL : 0);
	var tEl;
	foreach(tEl; self.getSubArticleElements()) {
	    if (!tEl.isCat(@IF_CompositeArticle)) {
		xOiDebugPrint(@Info, 2, [tEl, " is not a CompositeArticle"]); 
		continue;
	    }
	    xOiDebugPrint(@Info, 2, ["checking sub articles of ", tEl]);
	    var tSubID;
	    foreach(tSubID; tEl.getSubArticleIDs()) {
		var tSubArticle = tEl.getSubArticle(tSubID);
		xOiDebugPrint(@Info, 2, [tSubID, " -> ", tSubArticle]);
		if (tSubArticle == tObj) {
		    tRet = (tNeedInfo ? [tEl, tSubID] : 1);
		    break;
		}
	    }
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    

    
    public func setWidth0(pWidth)
    {
	mWidth0 = pWidth;
    }

    
    public func getWidth0()
    {
	return(mWidth0);
    }

    
    public func setHeight0(pHeight)
    {
	mHeight0 = pHeight;
    }

    
    public func getHeight0()
    {
	return(mHeight0);
    }

    
    public func setDepth0(pDepth)
    {
	mDepth0 = pDepth;
    }

    
    public func getDepth0()
    {
	return(mDepth0);
    }

    
    
    
    
    
    
    public func redrawEmptyGroup()
    {
	showGroupDummy();
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    protected func setInsertMode(pMode)
    {
	xOiDebugStartFunc("xOiPlGroup::setInsertMode", pMode);

	var tModes = @(0, 1, 2);

	if (tModes.find(pMode) >= 0) mXoiInsertMode = pMode;

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    protected func getInsertMode()
    {
	var tMode = mXoiInsertMode;
	if (mXoiInsertMode == NULL)
	    tMode = 0;

	return(tMode);
    }

    
    
    
    
    
    public func isValidForCollCheck(pEl)
    {           
	xOiDebugStartFunc("xOiPlGroup::isValidForCollCheck", pEl);

	
	
	
	
	

	
	
	
	

	var tRet = sXoiDuringCheckAdd || !getInsertMode();

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    protected func canTransformElements()
    {
	return(0);
    }

    
    
    
    
    
    
    protected func isPlGroupEmpty()
    {
	return(getElements());
    }

    
    
    
    
    
    
    
    protected func startCheckAdd()
    {
	var tBB = getLocalBounds();
	mWidth  = tBB[1][0]-tBB[0][0];
	mHeight = tBB[1][1]-tBB[0][1];
	mDepth  = tBB[1][2]-tBB[0][2];
	hideGroupDummy();

	
	
	
	sXoiDuringCheckAdd = 1;
    }

    
    
    
    
    
    
    
    protected func finishCheckAdd()
    {
	mWidth  = NULL;
	mHeight = NULL;
	mDepth  = NULL;
	if (getElements().empty()) showGroupDummy();

	sXoiDuringCheckAdd = 0;
    }

    
    
    
    
    
    
    
    
    
    
    
    
    protected func validateCheckAddType(pObj, pType)
    {
	return(pType.subClassOf(OiPlElement));
    }

    
    
    
    
    
    
    
    
    
    protected func validateCheckAdd(pObj, pCh)
    {
	return(pCh.isA(OiPlElement));
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func validateCheckAddPos(pCh)
    {
	return(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func showCheckAddErrUserMessage()
    {
	return(NULL);
    }

    
    
    
    
    
    protected func useBoundary()
    {
	return(1);
    }

    
    
    
    
    
    
    
    
    protected func getSelectedMember(pObj)
    {
	if (pObj == NULL || pObj == self) return(NULL);

	var tObj = pObj;
	while(!xOiObjInList(tObj, self.getChildren()))
	    tObj = tObj.getFather();

	return(tObj);
    }

    
    
    
    
    
    
    
    
    protected func getFirstPosRot(pChild)
    {
	if (getPlanning().getPlanDirX() == @L) {
	    var tBounds = pChild.getLocalBounds();
	    return([[mWidth0-tBounds[1][0]+tBounds[0][0], 0.0, 0.0], 0.0]);
	}
	return([[0.0, 0.0, 0.0], 0.0]);
    }

    
    
    
    
    
    
    
    
    
    protected func getExplicitPosRot()
    {
	return([[0.0, 0.0, 0.0], 0.0]);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func handleInsertion(pObj, pCh, pPosRotErr)
    {
	xOiDebugStartFunc("xOiPlGroup::handleInsertion", 
			  [pObj, pCh, pPosRotErr]);

	if (getInsertMode() != 2) {
	    xOiDebugPrint(@Info, 1, "no insertion allowed!");
	    xOiDebugFinishFunc(pPosRotErr);
	    return(pPosRotErr);
	}

	if (checkChildColl(pObj, NULL) == pCh) {
	    xOiDebugPrint(@Info, 1, 
			  "new element collides with reference object");
	    
	    
	    
	}

	
	var tInfo = getRepositionInfo(@InsertAfter, pObj, pCh);

	if (tInfo == NULL) {
	    xOiDebugPrint(@Warn, 1, "no information for repositioning!");
	    xOiDebugFinishFunc(pPosRotErr);
	    return(pPosRotErr);
	}

	var tRefObj = tInfo[0]; 
	var t2BeMoved = tInfo[1];
	var tSwitchPlanDirX = 0;
	if (tInfo.size() > 2)
	    tSwitchPlanDirX = tInfo[2];
	var tRefAttPt = NULL;
	if (tInfo.size() > 3)
	    tRefAttPt = tInfo[3];

	xOiDebugPrint(@Info, 2, ["reference ", tRefObj]);
	xOiDebugPrint(@Info, 2, ["attach point: ", tRefAttPt]);
	xOiDebugPrint(@Info, 2, ["elements to be moved: ", t2BeMoved]);
	xOiDebugPrint(@Info, 2, ["switch X plan dir? ", tSwitchPlanDirX]);

	if (t2BeMoved.empty()) {
	    
	    xOiDebugPrint(@ExplWarn, 1, "no neighbors to be moved!");
	    xOiDebugFinishFunc(pPosRotErr);
	    return(pPosRotErr);
	}

	var tPlan = getPlanning();

	if (tSwitchPlanDirX) tPlan.switchPlanDirX();

	
	
	
	
	var tOldPosRots = repositionElements2(tRefObj, tRefAttPt, t2BeMoved, 
					      @Targets, NULL);

	if (tSwitchPlanDirX) tPlan.switchPlanDirX();

	if (tOldPosRots == NULL) {
	    xOiDebugPrint(@Warn, 1, "neighbors couldn't been repositioned!");
	    xOiDebugFinishFunc(pPosRotErr);
	    return(pPosRotErr);
	}

	
	
	
	if (self.getFather().checkChildColl(self, NULL) != NULL) {
	    xOiDebugPrint(@Warn, 1, "collision of extended planning group!");
	    resetPositionRotations(t2BeMoved, tOldPosRots[0], tOldPosRots[1]);
	    xOiDebugFinishFunc(pPosRotErr);
	    return(pPosRotErr);
	}

	xOiDebugFinishFunc([pPosRotErr[0], pPosRotErr[1], NULL]);
	return([pPosRotErr[0], pPosRotErr[1], NULL]);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func getCheckAddState()
    {
	return(mXoiCheckAddState);
    }

    public func createChParams(pType, pChParams)
    {
	xOiDebugStartFunc2("xOiPlGroup::createChParams", [pType, pChParams]);

	try {
	    xOiPlElement::createChParams(pType, pChParams);
	}
	catch (&e: Error) {
	    xOiDebugPrint(@ExplWarn, 1, e.where+": "+ e.what);
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    protected func getCheckAddTmpChild()
    {
	if (oiExists(getName()+".ch"))
	    return(self.ch);
	return(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public final func checkAdd(pType, pObj, pPos, pParams)
    {
	xOiDebugStartFunc("xOiPlGroup::checkAdd",
			  [pType, pObj, pPos, pParams]);

	mXoiCheckAddState = @Started;

	startCheckAdd();

	mXoiCheckAddState = @CheckedStart;

	var tMyBnds = getLocalBounds();

	var tObj = getSelectedMember(pObj);

	mXoiCheckAddState = @MemberSelected;

	var tErr = validateCheckAddType(tObj, pType);

	mXoiCheckAddState = @CheckedType;

	var tErrIsInt = (tErr instanceof Int);
	var tErrIsStr = (tErr instanceof String);
	var tValidErr = (tErrIsInt || tErrIsStr);

	if ((tErrIsInt && !tErr) || tErrIsStr || !tValidErr) {
	    finishCheckAdd();
	    mXoiCheckAddState = @Finished;
	    if (tErrIsStr) oiOutput(@ERROR, tErr);
	    xOiDebugPrint(@Warn, 1, "invalid type!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tChAttPt = NULL;
	var tChParams = NULL;
	if (pParams != NULL) {
	    if (pParams instanceof Vector) {
		var tSize = pParams.size();
		if (tSize > 0) tChAttPt = pParams[0];
		if (tSize > 1) tChParams = pParams[1];
	    }
	    else
		tChAttPt = pParams;
	}

	createChParams(pType, tChParams); 
					  

	mXoiCheckAddState = @CreatedChild;

	tErr = validateCheckAdd(tObj, self.ch);

	mXoiCheckAddState = @CheckedChild;

	tErrIsInt = (tErr instanceof Int);
	tErrIsStr = (tErr instanceof String);
	tValidErr = (tErrIsInt || tErrIsStr);

	if ((tErrIsInt && !tErr) || tErrIsStr || !tValidErr) {
	    finishCheckAdd();
	    mXoiCheckAddState = @Finished;
	    var tMsg = NULL;;
	    if (tErrIsStr) tMsg = tErr;
	    xOiDebugPrint(@Warn, 1, "invalid child!");
	    xOiDebugFinishFunc(tMsg);
	    return(removeCh(tMsg));
	}

	var tIsFirstEl = isPlGroupEmpty();
	var tExplicit = (tObj != NULL && tObj == self);

	
	
	

	var tFather = getFather();
	var tReEnableCD = 0;
	if (tFather.getFather() == NULL && tFather != oiGetPlanning() && 
	    self.isEnabledCD()) {
	    self.disableCD();
	    tReEnableCD = 1;
	}

	if (tIsFirstEl || tExplicit) {
	    if (tIsFirstEl)
		xOiDebugPrint(@Info, 2, "first element");
	    else
		xOiDebugPrint(@Info, 2, "explicit position");
		
	    var tPosRot = getFirstPosRot(self.ch);
	    if (!tIsFirstEl)
		tPosRot = getExplicitPosRot();

	    var tPos = tPosRot[0];
	    var tRot = tPosRot[1];

    	    self.ch.setPosition(tPos);
    	    xOiSetYRotation(self.ch, tRot);

	    var tPosCheck = checkElPos(self.ch, [0, 0, 0]);
	    if (tReEnableCD) self.enableCD();

	    if (!tPosCheck) {
		mXoiCheckAddState = @NoValidPos4First;
		if (!tIsFirstEl)
		    mXoiCheckAddState = @NoValidExplicitPos;
		finishCheckAdd();
	        mXoiCheckAddState = @Finished;
		var tErr = "::ofml::xoi::@coll";
		if (planningHasBorder())
		    tErr = "::ofml::xoi::@badpos";
	    	xOiDebugFinishFunc(tErr);
		return(removeCh(tErr));
	    }
	    else {
		mXoiCheckAddState = @ValidPos4First;
		if (!tIsFirstEl)
		    mXoiCheckAddState = @ValidExplicitPos;
		var tErr = validateCheckAddPos(self.ch);
		mXoiCheckAddState = @CheckedValidPos4First;
		if (!tIsFirstEl)
		    mXoiCheckAddState = @CheckedValidExplicitPos;
	    	finishCheckAdd();
	        mXoiCheckAddState = @Finished;
		if (tErr == NULL || tErr instanceof Vector) {
		    if (tErr instanceof Vector) {
			tPos = tErr[0];
			tRot = tErr[1];
		    }
		    setMethod("onCreate("+String(tRot)+", NULL, NULL)");
	    	    xOiDebugFinishFunc(tPos);
		    return(acceptCh(tPos));
		}
	        xOiDebugFinishFunc(tErr);
	        return(removeCh(tErr));
	    }
	}

	
	if (tObj != NULL) {
	    try {
	        tObj.getName();
	    } catch (&e: Error) {
		tObj = NULL;
		xOiDebugPrint(@ExplWarn, 1, "selected object was removed!");
	    }
	}

	
	try {
	    self.ch.getName();
	} catch (&e: Error) {
	    if (tReEnableCD) self.enableCD();
	    finishCheckAdd();
	    mXoiCheckAddState = @Finished;
	    xOiDebugPrint(@ExplWarn, 1, "temporary child was removed!");
	    xOiDebugFinishFunc(NULL);
	    return(removeCh(NULL));
	}

	
	if (tObj != NULL && tObj != self &&
	    (tObj.isCat(@IF_ATTPT) || tObj.isCat(@IF_AttachPts)) && 
	    (self.ch.isCat(@IF_ATTPT) || self.ch.isCat(@IF_AttachPts))) {
	    var tAP = (tObj.hasMember(@getActiveAttPt) ? 
		       tObj.getActiveAttPt() : NULL);
	    var tPR = oiGetPosRot4AttachPts(tObj, self.ch, tChAttPt, 
					    1, 
					    1, 
					    getInsertMode() == 2);
	    xOiDebugPrint(@Info, 2, ["new obj to group element at: ", tPR]);
	    
	    
	    if (tReEnableCD) self.enableCD();
	    if (tPR != NULL) {
		var tPos = tPR[0];
		var tRR  = tPR[1];
		var tErr = tPR[2];
		if (tPos != NULL && tErr != NULL && 
		    (tErr == "::ofml::xoi::@coll" || 
		     tErr == "::ofml::oi::@coll")) {
		    mXoiCheckAddState = @CollPos4Sibling;
		    tPR = handleInsertion(tObj, self.ch, tPR);
		    mXoiCheckAddState = @InsertionHandled;
		    tPos = tPR[0];
		    tRR  = tPR[1];
		    tErr = tPR[2];
		}
		if (tPos != NULL && tErr != NULL && 
		    (tErr == "::ofml::xoi::@coll" || 
		     tErr == "::ofml::oi::@coll") &&
		     tAP != NULL) {
		    xOiDebugPrint(@Info, 1, 
				  ["cannot add to attach point ", tAP, 
				   " of ref ", tObj, " due to collision"]);
	    	    finishCheckAdd();
	            mXoiCheckAddState = @Finished;
	            xOiDebugFinishFunc(tErr);
	            return(removeCh(tErr));
		}
		if (tPos != NULL && tErr == NULL) {
		    mXoiCheckAddState = @ValidPos4Sibling;
		    var tErr = validateCheckAddPos(self.ch);
		    mXoiCheckAddState = @CheckedValidPos4Sibling;
	    	    finishCheckAdd();
	            mXoiCheckAddState = @Finished;
		    if (tErr == NULL || tErr instanceof Vector) {
		        if (tErr instanceof Vector) {
			    tPos = tErr[0];
			    tRR  = tErr[1];
		        }
		        setMethod("onCreate(" + String(tRR) + ", " + 
					    tObj.getName() + ", NULL)");
	    	        xOiDebugFinishFunc(tPos);
		        return(acceptCh(tPos));
		    }
	            xOiDebugFinishFunc(tErr);
	            return(removeCh(tErr));
		}
	    }
	    mXoiCheckAddState = @NoValidPos4Sibling;
	}

	
	if (self.ch.isCat(@IF_ATTPT) || self.ch.isCat(@IF_AttachPts)) {
	    var tPR = oiGetPosRot4AttachPts(self, self.ch, tChAttPt);
	    xOiDebugPrint(@Info, 2, ["new obj to group at: ", tPR]);
	    if (tReEnableCD) self.enableCD();
	    if (tPR != NULL) {
		var tPos = tPR[0];
		var tRot = tPR[1];
		var tErr = tPR[2];
		if (tErr != NULL) {
	    	    mXoiCheckAddState = @NoValidPos4Child;
		    finishCheckAdd();
	            mXoiCheckAddState = @Finished;
	    	    xOiDebugFinishFunc(tErr);
		    return(removeCh(tErr));
		}
		if (tPos != NULL) {
	    	    mXoiCheckAddState = @ValidPos4Child;
		    var tErr = validateCheckAddPos(self.ch);
	    	    mXoiCheckAddState = @CheckedValidPos4Child;
	    	    finishCheckAdd();
	            mXoiCheckAddState = @Finished;
		    if (tErr == NULL || tErr instanceof Vector) {
		        if (tErr instanceof Vector) {
			    tPos = tErr[0];
			    tRot = tErr[1];
		        }
	    	        setMethod("onCreate("+String(tRot)+", NULL, NULL)");
	    	        xOiDebugFinishFunc(tPos);
	    	        return(acceptCh(tPos));
		    }
	            xOiDebugFinishFunc(tErr);
	            return(removeCh(tErr));
		}
	    }
	    mXoiCheckAddState = @NoValidPos4Child;
	}

	
	if (useBoundary()) {
	    var tWP = xOiPlanning.checkAddBounds(self, tMyBnds, self.ch);
	    xOiDebugPrint(@Info, 2, ["at bounds: ", tWP]);
	    if (tReEnableCD) self.enableCD();

	    var tPos = tWP[0];
	    var tRot = 0.0;
	    var tErr = tWP[2];

	    if (tPos != NULL) {
	    	mXoiCheckAddState = @ValidBoundaryPos;
		tErr = validateCheckAddPos(self.ch);
	    	mXoiCheckAddState = @CheckedValidBoundaryPos;
	    }
	    else
	    	mXoiCheckAddState = @NoValidBoundaryPos;

	    if (tErr != NULL && tErr instanceof Vector) {
		tPos = tErr[0];
		tRot = tErr[1];
		tErr = NULL;
	    }

	    finishCheckAdd();
	    mXoiCheckAddState = @Finished;

	    if (tPos != NULL && tErr == NULL) {
	        setMethod("onCreate("+String(tRot)+", NULL, NULL)");
		xOiDebugFinishFunc(tPos);
		return(acceptCh(tPos));
	    }
	    else {
		xOiDebugFinishFunc(tErr);
		return(removeCh(tErr));
	    }
	}

	if (tReEnableCD) self.enableCD();

	finishCheckAdd();
	mXoiCheckAddState = @Finished;

	var tErrMsg = showCheckAddErrUserMessage();

	if (tErrMsg != NULL &&
	    (!(tErrMsg instanceof String) || tErrMsg.empty())) {
	    tErrMsg = "::ofml::xoi::@coll";
	    if (planningHasBorder())
		tErrMsg = "::ofml::xoi::@badpos";
	}

	
	xOiDebugFinishFunc(tErrMsg);
	return(removeCh(tErrMsg));
    }

    
    
    
    
    
    public func checkElPos(pChild, pOldPos)
    {
	return(checkChildColl(pChild, NULL) == NULL &&
	       getPlanning().checkBorderObj(pChild) == NULL);
    }

    
    
    
    
    
    public func cleanUp()
    {
	xOiDebugStartFunc("xOiPlGroup::cleanUp", NULL);

	xOiDisintegrateGroup(self, mXoiDoFs, NULL, NULL, NULL);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    protected func isInnerElement(pEl)
    {
	return(0);
    }

    
    
    
    
    
    
    
    
    
    protected func handleInnerElemRemoval(pEl)
    {
	xOiDebugStartFunc2("xOiPlGroup::handleInnerElemRemoval", pEl);

	
	
	var tInfo = getRepositionInfo(@Removal, pEl, NULL);

	if (tInfo == NULL) {
	    xOiDebugPrint(@Warn, 1, "no information for repositioning!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRefObj = tInfo[0]; 
	var t2BeMoved = tInfo[1];
	var tRefAttPt = NULL;
	if (tInfo.size() > 3)
	    tRefAttPt = tInfo[3];

	xOiDebugPrint(@Info, 2, ["reference ", tRefObj]);
	xOiDebugPrint(@Info, 2, ["attach point; ", tRefAttPt]);
	xOiDebugPrint(@Info, 2, ["elements to be moved: ", t2BeMoved]);

	if (t2BeMoved.empty()) {
	    
	    xOiDebugPrint(@ExplWarn, 1, "no neighbors to be moved!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	
	
	
	
	var tRet = repositionElements2(tRefObj, tRefAttPt, t2BeMoved, 
				       @SingleObj, pEl);

	if (tRet == NULL)
	    xOiDebugPrint(@Warn, 1, "neighbors couldn't been repositioned!");

	xOiDebugFinishFunc(NULL);
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func handleDimensionChange(pEl, pOldBB)
    {
	xOiDebugStartFunc("xOiPlGroup::handleDimensionChange", [pEl, pOldBB]);

	
	var tNewBB = pEl.getLocalGeoBounds();
	var tPos   = pEl.getPosition();

	xOiDebugPrint(@Info, 2, ["new bounds: ", tNewBB]);

	var tDiffW = 0.0;
	var tDiffD = 0.0;

	
	

	if (pOldBB != NULL) {
	    var tDiffX0 = pOldBB[0][0] - tNewBB[0][0];
	    var tDiffX1 = pOldBB[1][0] - tNewBB[1][0];

	    var tDiffZ0 = pOldBB[0][2] - tNewBB[0][2];
	    var tDiffZ1 = pOldBB[1][2] - tNewBB[1][2];

	    xOiDebugPrint(@Info, 2, 
			  ["X0 / Z0 diff: ", tDiffX0, " / ", tDiffZ0]);
	    xOiDebugPrint(@Info, 2,
			  ["X1 / Z1 diff: ", tDiffX1, " / ", tDiffZ1]);

	    tDiffW = tDiffX1-tDiffX0;
	    tDiffD = tDiffZ1-tDiffZ0;
	    xOiDebugPrint(@Info, 2, ["width diff: ", tDiffW]);
	    xOiDebugPrint(@Info, 2, ["depth diff: ", tDiffD]);

	    if (fabs(tDiffX1) < 0.0001 && fabs(tDiffZ1) < 0.0001 &&
		fabs(tDiffX0) < 0.0001 && fabs(tDiffZ0) < 0.0001) {
		xOiDebugPrint(@Info, 1, "bounds have not changed");
		xOiDebugFinishFunc(tNewBB);
		return(tNewBB);
	    }
	}

	
	var tInfo = getRepositionInfo(@DimensionChange, pEl, NULL);

	if (tInfo == NULL) {
	    xOiDebugPrint(@Warn, 1, "no information for repositioning!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRefObj = tInfo[0];
	var t2BeMoved = tInfo[1];
	var tRefAttPt = NULL;
	if (tInfo.size() > 3)
	    tRefAttPt = tInfo[3];

	xOiDebugPrint(@Info, 1, ["effective reference ", tRefObj]);
	xOiDebugPrint(@Info, 1, ["attach point ", tRefAttPt]);
	xOiDebugPrint(@Info, 1, ["elements to be moved: ", t2BeMoved]);

	if (t2BeMoved.empty()) {
	    
	    var tRet = tNewBB;
	    if ((tDiffW < 0 || tDiffD < 0) &&
		self.getFather().checkChildColl(self, NULL) != NULL) {
		xOiDebugPrint(@Warn, 1, "collision detected!");
		tRet = NULL;
	    }
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	
	
	
	
	var tOldPosRots = repositionElements2(tRefObj, tRefAttPt, t2BeMoved, 
					      @Targets, NULL);
	if (tOldPosRots == NULL) {
	    xOiDebugPrint(@Warn, 1, "neighbors couldn't been repositioned!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRet = tNewBB;

	
	if (self.getFather().checkChildColl(self, NULL) != NULL) {
	    xOiDebugPrint(@Warn, 1, "collision of extended planning group!");
	    resetPositionRotations(t2BeMoved, tOldPosRots[0], tOldPosRots[1]);
	    tRet = NULL;
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func getRepositionInfo(pEvent, pEl, pNewEl)
    {
	return(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    private func repositionElements2(pRefObj, pRefAttPt, pTargets, 
				     pMoveOut, pMoveObj)
    {
	xOiDebugStartFunc("xOiPlGroup::repositionElements2", 
			  [pRefObj, pRefAttPt, pTargets, pMoveOut, pMoveObj]);

	var tMoveOutModes = @(@Targets, @SingleObj);
	if (tMoveOutModes.find(pMoveOut) < 0 ||
	    (pMoveOut == @SingleObj && pMoveObj == NULL)) {
	    xOiDebugPrint(@Warn, 1, "invalid parameters!");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	
	
	var tPosRotData = getPosRotData(pTargets);
	var tPositions = tPosRotData[0];
	var tRotations = tPosRotData[1];
	var tPosDiffs  = tPosRotData[2];
	var t1stRot    = tPosRotData[3];;

	xOiDebugPrint(@Info, 2, ["old positions: ", tPositions]);
	xOiDebugPrint(@Info, 2, ["differences:   ", tPosDiffs]);

	
	if (pMoveOut == @Targets)
	    moveOutOfPlanBounds(pTargets, tPosDiffs);
	else
	if (pMoveOut == @SingleObj)
	    moveOutOfPlanBounds([pMoveObj], [[0.0, 0.0, 0.0]]);

	var tRet = NULL;

	if (moveElements2(pRefObj, pRefAttPt, pTargets, 
			  tPositions, tPosDiffs, t1stRot))
	    tRet = [tPositions, tRotations];
	else
	if (pMoveOut == @Targets)
	    resetPositionRotations(pTargets, tPositions, tRotations);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    private func repositionElements(pRefObj, pTargets, pMoveOut, pMoveObj)
    {
	xOiDebugStartFunc("xOiPlGroup::repositionElements", 
			  [pRefObj, pTargets, pMoveOut, pMoveObj]);

	var tRet = 
	    repositionElements2(pRefObj, NULL, pTargets, pMoveOut, pMoveObj);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    protected func getUsedAttPt(pRefObj, pNeighbor)
    {
	return(NULL);
    }

    
    private func getPosRotData(pNeighbors)
    {
	var tPositions = Vector(0);
	var tRotations = Vector(0);
	var tPosDiffs  = Vector(0);
	var t1stPos    = NULL;
	var t1stRot    = NULL;
	var tE;
	foreach(tE; pNeighbors) {
	    var tPos = tE.getPosition();
	    var tRot = tE.getRotation(@PY);
	    tPositions.pushBack(tPos);
	    tRotations.pushBack(tRot);
	    if (t1stPos == NULL) {
		t1stPos = tPos;
		t1stRot = tRot;
	    }
	    tPosDiffs.pushBack([tPos[0]-t1stPos[0], 
			        tPos[1]-t1stPos[1],
			        tPos[2]-t1stPos[2]]);
	}
	return([tPositions, tRotations, tPosDiffs, t1stRot]);
    }

    private func moveOutOfPlanBounds(pNeighbors, pPosDiffs)
    {
	var tScBnds = getPlanning().getWorldBounds();
	var tOutPos = tScBnds[1];
	var i;
	for(i = 0; i < pNeighbors.size(); i++) {
	    var tOffset = pPosDiffs[i];
	    pNeighbors[i].setPosition([tOutPos[0]+tOffset[0],
				       tOutPos[1]+tOffset[1],
				       tOutPos[2]+tOffset[2]]);
	}
    }

    
    
    
    
    
    
    
    
    
    private func moveElements2(pRefObj, pRefAttPt, pNeighbors, 
			       pOldPositions, pPosDiffs, p1stRot)
    {
	xOiDebugStartFunc2("xOiPlGroup::moveElements2", 
			   [pRefObj, pRefAttPt, pNeighbors, p1stRot]);

        xOiDebugPrint(@Info, 3, 
		      ["position of ref obj: ", pRefObj.getPosition()]);
        xOiDebugPrint(@Info, 3, 
		      ["bounds of ref obj: ", pRefObj.getLocalBounds()]);

	var t1stNeighbor = pNeighbors[0];

	
	

	var tAttPt = pRefAttPt;
	if (tAttPt == NULL)
	    tAttPt = getUsedAttPt(pRefObj, t1stNeighbor);
	    
	    
	    

	var tCurrAttPt = NULL;
	if (tAttPt != NULL && pRefObj.hasMember(@getActiveAttPt)) {
	    tCurrAttPt = pRefObj.getActiveAttPt();
	    pRefObj.setActiveAttPt(tAttPt);
	}

	var t1stPR = oiGetPosRot4AttachPts(pRefObj, t1stNeighbor, NULL, 0);
        xOiDebugPrint(@Info, 2, 
		      ["first neighbor to reference object at: ", t1stPR]);

	if (tCurrAttPt != NULL)
	    pRefObj.setActiveAttPt(tCurrAttPt); 

	if (t1stPR == NULL || t1stPR[0] == NULL || t1stPR[2] != NULL) {
	    xOiDebugPrint(@Warn, 1, 
			  "cannot place first neighbor beside ref object!");
	    resetPositionRotations(pNeighbors, pOldPositions, NULL);
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	
	
	
	var tNew1stPos = t1stPR[0];
	var tNew1stRot = t1stPR[1];
	var tRotDiff   = tNew1stRot - p1stRot;

	var i;
	for(i = 0; i < pNeighbors.size(); i++) {
	    var tPosDiff = pPosDiffs[i];
	    var tNewPos  = xOiTransformXZCoords(tNew1stPos, tRotDiff, 
					        tPosDiff[0], tPosDiff[2]);
            xOiDebugPrint(@Info, 2, 
			  ["moving ", pNeighbors[i], " to ", tNewPos]);
	    pNeighbors[i].setPosition(tNewPos); 
	    var tNewRot = pNeighbors[i].getRotation(@PY) + tRotDiff;
	    xOiSetYRotation(pNeighbors[i], tNewRot);
	}

	xOiDebugFinishFunc(1);
	return(1);
    }

    private func moveElements(pRefObj, pNeighbors, pOldPositions, pPosDiffs, 
			      p1stRot)
    {
	xOiDebugStartFunc2("xOiPlGroup::moveElements", 
			   [pRefObj, pNeighbors, p1stRot]);

	var tRet = moveElements2(pRefObj, NULL, pNeighbors, 
				 pOldPositions, pPosDiffs, p1stRot);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    private func resetPositionRotations(pElements, pOldPositions, pOldRotations)
    {
	var i;
	for(i = 0; i < pElements.size(); i++) {
	    pElements[i].setPosition(pOldPositions[i]);
	    if (pOldRotations != NULL)
		xOiSetYRotation(pElements[i], pOldRotations[i]);
	}
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    
    
    
    
    
    
    
    protected func needsCommonProperties()
    {
	return(0);
    }

    
    
    
    
    
    
    
    protected func getCommonPropElements()
    {
	return(getElements());
    }

    
    
    
    protected func getFixProperties()
    {
	return(@());
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func updateProperties()
    {
	xOiDebugStartFunc("xOiPlGroup::updateProperties", NULL);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tOldLang = self.getPDLanguage();

	if (sXoiAddStatePDLanguage != NULL)
	    tOldLang = sXoiAddStatePDLanguage;

	xOiPlElement::updateProperties();

	var tNewLang = self.getPDLanguage();

	
	
	
	
 	
	
	
	
	
	
	
	
	
	
	
	
	
	

	if (!sXoiEnforcePropsUpdate && tNewLang == tOldLang) {
	    xOiDebugPrint(@Info, 1, "PD language hasn't changed");
	    xOiDebugFinishFunc(1);
	    return(1);
	}

	if (tNewLang != tOldLang)
	    xOiDebugPrint(@Info, 1, ["PD language has changed from ", tOldLang, 
				     " to ", tNewLang]);

	var tCommonProps = _xOiGetOptionCommonProps();
	var tHasCP = (!tCommonProps.empty());
	var tCPDepth = _xOiGetOptionCommonPropsDepth();

	var tE;
	foreach(tE; self.getElements()) {
	    if (tHasCP)
		xOiUpdatePropsDeep(tE, tCPDepth);
	    else
		tE.updateProperties();
	}

	var tCPH = xOiCommonPropsHandler(self);
	tCPH.initCommonProps(NULL, 1);

	if (tNewLang != tOldLang)
	    updateOtherFixProperties();
	updateOtherFixProperties2(sXoiEnforcePropsUpdate, tNewLang != tOldLang);

	sXoiEnforcePropsUpdate = 0;
	sXoiAddStatePDLanguage = NULL;

	xOiDebugFinishFunc(1);
	return(1);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func updateOtherFixProperties()
    {
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func updateOtherFixProperties2(pAfterAddStateCode, 
					     pChangedLanguage, ...)
    {
    }

    
    
    
    
    
    
    
    
    
    
    
    public func getPropInfo(pPropKey, pPropValue, pInfoType)
    {
	xOiDebugStartFunc("xOiPlGroup::getPropInfo", 
			  [pPropKey, pPropValue, pInfoType]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tCommonProps = _xOiGetOptionCommonProps();

	var tInfo = NULL;

	if (!tCommonProps.empty() && tCommonProps.find(pPropKey) >= 0) {
	    var tCPH = xOiCommonPropsHandler(self);
	    tInfo = tCPH.getCommonPropInfoDeep(pPropKey, pPropValue, pInfoType);
	}
	else
	    
	    tInfo = xOiPlElement::getPropInfo(pPropKey, pPropValue, pInfoType);

	xOiDebugFinishFunc(tInfo);
	return(tInfo);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setPropValue(pProp, pValue)
    {
	xOiDebugStartFunc("xOiPlGroup::setPropValue", [pProp, pValue]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tRet = 0;

	var tCurrValue = self.getPropValue(pProp);
	xOiDebugPrint(@Info, 2, ["current value: ", tCurrValue]);

	if (tCurrValue != pValue) {
	    tRet = xOiPlElement::setPropValue(pProp, pValue);
	    
	    
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	

	var tCPs = _xOiGetOptionCommonProps();

	if (!tCPs.empty() && tCPs.find(pProp) >= 0) {
	    var tCPH = xOiCommonPropsHandler(self);
	    if (tCPH.setCommonPropsDeep(@(pProp), NULL)) {
		
		tRet = 1;
		tCPH.initCommonProps(NULL, 0);
	    }
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func propsChanged(pProps, pDoChecks)
    {
	xOiDebugStartFunc("xOiPlGroup::propsChanged", [pProps, pDoChecks]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tChangedFixProps = @();

	var tFixProps = self.getFixProperties();
	var tProp;
	foreach(tProp; pProps) {
	    if (tFixProps.find(tProp) >= 0)
		tChangedFixProps.pushBack(tProp);
	}

	var tRet = 1;

	if (!tChangedFixProps.empty())
	    tRet = fixPropsChanged(tChangedFixProps, pDoChecks);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func fixPropsChanged(pProps, pDoChecks)
    {
	xOiDebugStartFunc("xOiPlGroup::fixPropsChanged", [pProps, pDoChecks]);

	var tCPs = _xOiGetOptionCommonProps();

	if (tCPs.empty()) {
	    xOiDebugPrint(@Info, 1, "no @CommonProps");
	    xOiDebugFinishFunc(1);
	    return(1);
	}

	var tCPH = xOiCommonPropsHandler(self);
	if (tCPH.setCommonPropsDeep(pProps, NULL)) 
	    
	    tCPH.initCommonProps(NULL, 0);

	xOiDebugFinishFunc(1);
	return(1);
    }

    
    
    

    
    
    
    
    
    
    func getCommonPropsOptions()
    {
	var tOptions = Hash();
	tOptions[@CommonProps] = _xOiGetOptionCommonProps();
	tOptions[@FirstPos4CommonProps] = _xOiGetOption1stPos4CommonProps();
	tOptions[@CommonPropsPos] = _xOiGetOptionCommonPropsPos();
	tOptions[@AllObjs4CommonProps] = _xOiGetOptionAllObjs4CommonProps();
	tOptions[@CommonPropsChLMode] = _xOiGetOptionCommonPropsChLMode();
	tOptions[@SortUnionCommonProps] = _xOiGetOptionSortUnionCommonProps();
	tOptions[@Classes4CommonProps] = _xOiGetOptionClasses4CommonProps();
	tOptions[@Meta4CommonProps] = _xOiGetOptionMeta4CommonProps();
	tOptions[@CommonPropsDepth] = _xOiGetOptionCommonPropsDepth();
	tOptions[@NonVisibleProps4Common] = _xOiGetOptionNonVisibleProps4Common();
	tOptions[@ROPropsEditable4Common] = _xOiGetOptionROPropsEditable4Common();
	

	return(tOptions);
    }

    
    
    
    
    func getObjs4OptionCommonProps()
    {
	return(@());
    }

    
    func duringSetCommonProps(pState)
    {
	sXoiDuringCommonPropSet = pState;
    }

    
    
    
    
    

    
    
    protected func _xOiGetOptionCommonProps()
    {
	return(@());
    }

    
    
    
    
    protected func _xOiGetOption1stPos4CommonProps()
    {
	return(1);
    }

    
    
    
    protected func _xOiGetOptionCommonPropsPos()
    {
	return(Vector(0));
    }

    
    
    
    protected func _xOiGetOptionAllObjs4CommonProps()
    {
	return(0); 
    }

    
    
    
    
    
    
    
    
    
    
    
    
    protected func _xOiGetOptionCommonPropsChLMode()
    {
	return(@FirstEl); 
    }

    
    
    
    
    
    
    
    
    
    
    
    protected func _xOiGetOptionSortUnionCommonProps()
    {
	return(0); 
    }

    
    
    
    
    
    
    
    
    
    protected func _xOiGetOptionClasses4CommonProps()
    {
	return(0); 
    }

    
    
    
    
    
    protected func _xOiGetOptionMeta4CommonProps()
    {
	return(0); 
    }

    
    
    
    
    protected func _xOiGetOptionCommonPropsDepth()
    {
	return(0); 
    }

    
    
    
    
    
    
    
    
    
    
    protected func _xOiGetOptionNonVisibleProps4Common()
    {
	return(0); 
    }

    
    
    
    
    
    
    
    
    
    protected func _xOiGetOptionROPropsEditable4Common()
    {
	return(0); 
    }

    
    
    
    
    
    
    
    
    protected func _xOiGetOptionArticleFeatures4Common()
    {
	return(0); 
    }

    
    
    

    

    
    
    
    
    
    
    
    protected func _xOiGetElementCatHash()
    {
	return(Hash());
    }

    private func _xOiGetName4CatHash(pObj)
    {
	xOiDebugStartFunc2("xOiPlGroup::_xOiGetName4CatHash", pObj);

	var tName = NULL;
	var tEL   = self.getElements();

	if (tEL.find(pObj) >= 0 || isGrandSubArticle(pObj))
	    tName = localObjName(pObj);

	xOiDebugFinishFunc(tName);
	return(tName);
    }

    
    protected func _xOiAssignElementCat(pObj, pCat, pHash)
    {
	xOiDebugStartFunc("xOiPlGroup::_xOiAssignElementCat", [pObj, pCat]);

	var tName = _xOiGetName4CatHash(pObj);

	if (tName == NULL) {
	    xOiDebugPrint(@Info, 1, "given instance is neither a group " +
				    "element nor a grand sub-article");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	if (!(pCat instanceof Symbol)) {
	    xOiDebugPrint(@Warn, 1, "invalid category type");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	if (pHash.hasKey(pCat)) {
	    var tList = pHash[pCat];
	    if (tList.find(tName) < 0)
		tList.pushBack(tName);
	    else
		xOiDebugPrint(@Info, 1, 
			      "element already is assigned to given cat");
	}
	else
	    pHash[pCat] = @(tName);

	xOiDebugPrint(@Info, 2, ["(new) list of category elements: ", 
				 pHash[pCat]]);

	if (mXoiElCatSelectable.hasKey(pCat)) {
	    var tInfo = mXoiElCatSelectable[pCat];
	    if (tInfo[0])
		if (tInfo[1])
		    pObj.hierSelectable();
		else
		    pObj.selectable();
	    else
		if (tInfo[1])
		    pObj.notHierSelectable();
		else
		    pObj.notSelectable();
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    protected func _xOiRemoveElementCat(pObj, pCat, pHash)
    {
	xOiDebugStartFunc("xOiPlGroup::_xOiRemoveElementCat", [pObj, pCat]);

	var tName = NULL;

	if (pObj instanceof String)
	    tName = pObj;
	else
	    tName = _xOiGetName4CatHash(pObj);

	if (tName == NULL) {
	    xOiDebugPrint(@Info, 1, "given instance is neither a group " +
				    "element nor a grand sub-article");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	if (!(pCat instanceof Symbol || pCat instanceof Void)) {
	    xOiDebugPrint(@Warn, 1, "invalid category type");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	if (pCat instanceof Symbol && !pHash.hasKey(pCat)) {
	    xOiDebugPrint(@Warn, 1, "unknown category");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tCats = @();

	if (pCat != NULL)
	    tCats.pushBack(pCat);
	else
	    xOiCopyAggr(_xOiGetElementCategories(pHash, pObj), tCats, 0);

	var tCat;
	foreach(tCat; tCats) {
	    xOiDebugPrint(@Info, 2, ["processing category ", tCat, " ..."]);

	    var tList = pHash[tCat];
	    tList.remove(tName);
	    xOiDebugPrint(@Info, 2, ["(new) list of category elements: ", 
				     pHash[tCat]]);
	    if (tList.empty()) {
		xOiDebugPrint(@Info, 1, 
			      "removed last cat element -> removing cat");
		pHash.remove(tCat);
	    }
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    protected func _xOiIsElementCat(pObj, pCat, pHash)
    {
	xOiDebugStartFunc2("xOiPlGroup::_xOiIsElementCat", [pObj, pCat]);

	var tName = _xOiGetName4CatHash(pObj);

	if (tName == NULL) {
	    xOiDebugPrint(@Info, 1, "given instance is neither a group " +
				    "element nor a grand sub-article");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	if (!(pCat instanceof Symbol)) {
	    xOiDebugPrint(@Warn, 1, "invalid category type");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	if (!pHash.hasKey(pCat)) {
	    xOiDebugPrint(@Warn, 1, "unknown category");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	var tList = pHash[pCat];

	var tRet = (tList.find(tName) >= 0);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    protected func _xOiGetElementCategories(pHash, pObj)
    {
	xOiDebugStartFunc("xOiPlGroup::_xOiGetElementCategories", pObj);

	var tCats = pHash.keys();
	var tRet  = Vector(0);

	if (pObj == NULL)
	    xOiCopyAggr(tCats, tRet, 0);
	else {
	    var tName = NULL;
	    if (pObj instanceof String)
		tName = pObj;
	    else
		tName = _xOiGetName4CatHash(pObj);
	    if (tName == NULL)
		xOiDebugPrint(@Info, 1, "given instance is neither a group " +
				        "element nor a grand sub-article");
	    else {
		var tCat;
		foreach(tCat; tCats) {
		    var tList = pHash[tCat];
		    if (tList.find(tName) >= 0)
			tRet.pushBack(tCat);
		}
	    }
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    protected func _xOiGetCatElements(pCat, pHash)
    {
	xOiDebugStartFunc("xOiPlGroup::_xOiGetCatElements", pCat);

	var tRet = @();

	if (!(pCat instanceof Symbol)) {
	    xOiDebugPrint(@Warn, 1, "invalid category type");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	if (!pHash.hasKey(pCat)) {
	    xOiDebugPrint(@Warn, 1, "unknown category");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tList = pHash[pCat];
	var tToBeRemoved = @();
	var tName;
	foreach(tName; tList) {
	    var tObj  = localName2Obj(tName);
	    if (tObj == NULL)
		
		tToBeRemoved.pushBack(tName);
	    else
		tRet.pushBack(tObj);
	}

	foreach(tName; tToBeRemoved) {
	    xOiDebugPrint(@Info, 1, ["removing non-existing ", tName]);
	    tList.remove(tName);
	}
	
	if (tList.empty()) {
	    xOiDebugPrint(@Info, 1, "removed last cat element -> removing cat");
	    pHash.remove(pCat);
	    
	    
	    
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    protected func _xOiSetSelectability4ElemCat(pCat, pState, pApplyHier, pHash)
    {
	xOiDebugStartFunc("xOiPlGroup::_xOiSetSelectability4ElemCat", 
			  [pCat, pState, pApplyHier]);

	if (!(pCat instanceof Symbol)) {
	    xOiDebugPrint(@Warn, 1, "invalid category type");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	if (!(pState instanceof Int && (pState == 0 || pState == 1))) {
	    xOiDebugPrint(@Warn, 1, "invalid state");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	mXoiElCatSelectable[pCat] = [pState, pApplyHier];

	var tObj;
	foreach(tObj; _xOiGetCatElements(pCat, pHash)) {
	    if (pState)
		if (pApplyHier)
		    tObj.hierSelectable();
		else
		    tObj.selectable();
	    else
		if (pApplyHier)
		    tObj.notHierSelectable();
		else
		    tObj.notSelectable();
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    protected func _xOiGetSelectability4ElemCat(pCat)
    {
	xOiDebugStartFunc("xOiPlGroup::_xOiGetSelectability4ElemCat", pCat);

	if (!(pCat instanceof Symbol)) {
	    xOiDebugPrint(@Warn, 1, "invalid category type");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRet = NULL;
	if (mXoiElCatSelectable.hasKey(pCat))
	    tRet = mXoiElCatSelectable[pCat][0];

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    protected func _xOiElCategoriesRestored()
    {
	xOiDebugStartFunc("xOiPlGroup::_xOiElCategoriesRestored", NULL);

	xOiDebugPrint(@Info, 2, ["mXoiElCatSelectable: ", mXoiElCatSelectable]);

	var tCatHash = _xOiGetElementCatHash();

	var tCat;
	foreach(tCat; mXoiElCatSelectable.keys()) {
	    var tObjList = tCatHash[tCat];
	    var tName;
	    foreach(tName; tObjList) {
		if (!sXoiObjects4ElCatSelStateUpdate.hasKey(tName))
		    sXoiObjects4ElCatSelStateUpdate[tName] = @();
		var tCats = sXoiObjects4ElCatSelStateUpdate[tName];
		tCats.pushBack(tCat);
	    }
	}

	xOiDebugPrint(@Info, 2, ["objects->categories: ", 
				 sXoiObjects4ElCatSelStateUpdate]);

	xOiDebugFinishFunc(NULL);
    }

    
    func _xOiSetupConfigurationFinished(pObj)
    {
	xOiDebugStartFunc("xOiPlGroup::_xOiSetupConfigurationFinished", pObj);

	var tLName = localObjName(pObj);

	if (tLName == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	xOiDebugPrint(@Info, 2, ["objects->categories: ", 
				 sXoiObjects4ElCatSelStateUpdate]);
	var tName;
	foreach(tName; sXoiObjects4ElCatSelStateUpdate.keys()) {
	    xOiDebugPrint(@Info, 2, ["checking for ", tName]);
	    var tNameLen = tName.size();
	    
	    if (tLName.size() >= tNameLen && 
		tLName.substr(0, tNameLen) == tName) {
		var tObj = localName2Obj(tName);
		var tCat;
		foreach(tCat; sXoiObjects4ElCatSelStateUpdate[tName]) {
		    var tInfo = mXoiElCatSelectable[tCat];
		    xOiDebugPrint(@Info, 2, 
				  ["setting state ", tInfo[0], " for category ",				   tCat, tInfo[1] ? " hierarchically" : ""]);
		    if (tInfo[0])
			if (tInfo[1])
			    tObj.hierSelectable();
			else
			    tObj.selectable();
		    else
			if (tInfo[1])
			    tObj.notHierSelectable();
			else
			    tObj.notSelectable();
		}
		sXoiObjects4ElCatSelStateUpdate.remove(tName);
	    }
	}

	xOiDebugFinishFunc(NULL);
	return;
    }

    
    protected func _xOiClearElementCat(pCat, pHash)
    {
	xOiDebugStartFunc("xOiPlGroup::_xOiClearElementCat", pCat);

	if (!(pCat instanceof Symbol)) {
	    xOiDebugPrint(@Warn, 1, "invalid category type");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	if (mXoiElCatSelectable.hasKey(pCat))
	    mXoiElCatSelectable.remove(pCat);

	if (!pHash.hasKey(pCat)) {
	    xOiDebugPrint(@Warn, 1, "unknown category");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	pHash.remove(pCat);

	xOiDebugFinishFunc(NULL);
    }

    
    
    

    rule NEW_ELEMENT (pValue)
    {
	var tEl = $pValue;

	xOiDebugStartFunc2("xOiPlGroup::NEW_ELEMENT", tEl);

	if (objIsTmpCh(tEl) || !tEl.isCat(@IF_Article)) {
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	hideGroupDummy();

	
	mXoiDoFs[tEl.getName()] = [tEl.getTrAxis(), tEl.getRtAxis()];

	xOiDebugPrint(@Info, 3, ["DoF ", mXoiDoFs]);

	if (!canTransformElements()) {
	    tEl.setTrAxis(0);
	    tEl.setRtAxis(0);
	}

	xOiDebugFinishFunc(0);
	return(0);
    }

    rule REMOVE_ELEMENT (pValue)
    {
        var tEl = $pValue;

	xOiDebugStartFunc("xOiPlGroup::REMOVE_ELEMENT", tEl);

	if (!tEl.isCat(@IF_Article)) {
	    xOiDebugPrint(@Info, 1, "element ist not an article.");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	if (getElements().size() == 1)
	    
	    showGroupDummy();
	else
	    update2DRepr(); 

	if (mXoiDoFs.hasKey(tEl.getName()))
	    mXoiDoFs.remove(tEl.getName());

	if (isInnerElement(tEl) && getInsertMode() == 2) 
	    handleInnerElemRemoval(tEl);

	xOiDebugFinishFunc(0);
	return(0);
    }

    rule START_DUMP(pArg)
    {
	xOiDebugStartFunc("xOiPlGroup::START_DUMP", pArg);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	mAtDump = 1;

	xOiDebugFinishFunc(0);
	return(0);
    }
    
    rule FINISH_DUMP(pArg)
    {
	mAtDump = 0;

	return(0);
    }
    
    rule START_EVAL(pArg)
    {
	mAtEval = 1;

	
	if (mAtDump == NULL) mAtDump = 0;
	if (mXoiDoFs == NULL) mXoiDoFs = Hash();
	if (mXoi2DMode == NULL) mXoi2DMode = @Combined;

	return(0);
    }

    rule FINISH_EVAL(pArg)
    {
	mAtEval = 0;

	getPlanning().getChangeManager().register(self, @(@ChildTransformed), 
						  [@(self), 0]);

	if (mXoiElCatSelectable == NULL)
	    mXoiElCatSelectable = Hash();

	return(0);
    }
}

