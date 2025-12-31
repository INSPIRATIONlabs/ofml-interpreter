// === ofml/xoi/xoiplanning.cls ===
package ::ofml::xoi;

import ::ofml::oi::*;

import ::stdio::*;


















public class xOiPlanning: xOiBasePlanning
{
    
    var mErrorLog;

    var mCheckAddPos;

    
    
    
    var mLibPaths;

    
    var mMarkedGroupObjs;

    var mPlanMode;
    var mXoiTmpPlanMode;

    var mXoiInsertMode;

    
    var mXoiAppViewMode;
    var mXoiCheckWallCrossing;
    var mXoiConversionAppMode;
    var mXoiTransformationAppMode;
    var mXoiEditOpAppMode;
    var mXoiNeedElemConfig;

    
    
    static var sXoiMultiFMLProcAppMode = 0;
    static var sXoiExportAppMode = @None;
    static var sXoiCatInsertionAppMode = @Select;
    static var sXoiGlobalCollCheckAppMode = 1;
    static var sXoiChangingPriceDate = 0;

    
    
    var mXoiAtGrouping;

    
    
    
    
    var mCCheck;

    
    var mCCheckRes;

    
    var mHiddenElems;
    var mHiddenCats;
    var mHiddenProgs;

    
    var mMeasure;

    
    
    
    var mXoiPrintedMessages;

    var mAtEval;

    
    var mXoiBatchOp;
    var mXoiBatchObjects;

    static var sFreePlElemModes = NULL;
    static var sTestMode = 0;

    static var sObjectPaste = 0;

    static var sMultiInsert = 0;

    
    static var sXoiOrderMap = Hash(); 

    
    public func initialize(pFa, pNa)
    {
	xOiDebugStartFunc("xOiPlanning::initialize", [pFa, pNa]);

        xOiBasePlanning::initialize(pFa, pNa);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	mHiddenElems = @();
	mHiddenCats = @();
	mHiddenProgs = @();

	mXoiInsertMode = 0;
	mXoiCheckWallCrossing = 1;
	mXoiConversionAppMode = 0;
	mXoiTransformationAppMode = @Constrained;
	mXoiEditOpAppMode = @None;

	mAtEval = 0;

	mXoiAtGrouping = 0;

	if (pNa != @t) {
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	

	mCCheck = 0;
	mCCheckRes = 1;

	mLibPaths = Hash();

        mMarkedGroupObjs = @();
	xOiDebugPrint(@Info, 2, ["marked objects: ", mMarkedGroupObjs]);

	
	addInfoObj(xOiGroupProgInfo, @xoi_tmpgroup);

	
	xOiOrderItem(self, @orderitems, @Folder, @orderitems, NULL);

	mXoiNeedElemConfig = 3;

	mXoiPrintedMessages = @();

	setBorder([NULL, [0.0, 0.0, 0.0]]);

	registerWithChangeManager();
	initializeStaticStuff();

	xOiDebugFinishFunc(NULL);
    }

    
    func evalFunc(pFunc)
    {
	var tRes;

	try {
	    tRes = eval(pFunc);
	}
	catch (&e: Error) {
	    oiOutput(@WARNING, e.what);
	}

	return(tRes);
    }

    private func hasBorder()
    {
	var tBorder = self.getBorder();

	if (tBorder == NULL) return(0);

	var tDim = tBorder[1];

	if (tDim == NULL) return(0);

	if (tDim[0] < eps || tDim[1] < eps || tDim[2] < eps) return(0);

	return(1);
    }

    public func setTestMode(pMode)
    {
	sTestMode = pMode;
    }

    
    
    
    
    public func isCat(pCat)
    {
	if (pCat == @Configurator)
	    return(getPlanningMode() == 2 || getPlanningMode() == 4);

	return(xOiBasePlanning::isCat(pCat));
    }

    
    func atGrouping()
    {
	return(mXoiAtGrouping);
    }

    public func receivePropertyChange(pPublisher, pArgs)
    {
	xOiDebugStartFunc("xOiPlanning::receivePropertyChange",
			  [pPublisher, pArgs]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	if (pPublisher.isCat(@IF_Article) &&
	    pPublisher.getObjState(@OI_UpdateState) != @Up2Date) {
	    
	    xOiDebugPrint(@Warn, 1, "publisher is not up-2-date!?");
	}

	var tFather = pPublisher.getFather();
	xOiDebugPrint(@Info, 2, ["father ", tFather]);

	if (pPublisher.isCat(@IF_Article))
	    updatePDCache(pPublisher, 0);

	xOiDebugFinishFunc(NULL);
    }

    
    
    

    
    
    protected func setProgLibPath(pProgID, pLibPath)
    {
	mLibPaths[pProgID] = pLibPath;
    }

    
    public func getLibPath(pPID)
    {
	xOiDebugStartFunc("xOiPlanning::getLibPath", pPID);

	var tRes = NULL;
	if (mLibPaths == NULL)
	    
	    
	    
	    
	    tRes = xOiProgID2Path(pPID);
	else
	if (mLibPaths.hasKey(pPID))
	    tRes = mLibPaths[pPID];

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    public func getProgPDB(pPID)
    {
	xOiDebugStartFunc("xOiPlanning::getProgPDB", pPID);

	var tPDM = getPDManager();
	var tPDB = NULL;
	if (tPDM != NULL)
	    tPDB = tPDM.getProgPDB(pPID);

	xOiDebugFinishFunc(tPDB);
	return(tPDB);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func setRegion4(pRegion, pPrograms, pUpdate)
    {
	xOiDebugStartFunc("xOiPlanning::setRegion4",
			  [pRegion, pPrograms, pUpdate]);

	var tPDM = getPDManager();
	if (tPDM == NULL) {
	    xOiDebugPrint(@Info, 1, "no PD Manager");
	    xOiDebugFinishFunc(NULL);
	    return;
	}
	var tPrograms = pPrograms;
	if (pPrograms == NULL)
	    tPrograms = getInfoIDs();

	var tSwitched = @();
	var tPID;
	foreach(tPID; tPrograms) {
	    var tPID2 = tPID; 
	    var tPDB = tPDM.getProgPDB(tPID2);
	    if (tPDB != NULL && tPDB.hasMember(@switchRegion)) {
		var tOk = tPDB.switchRegion(pRegion);
		if (tOk) tSwitched.pushBack(tPID2);
	    }
	}

	if (!tSwitched.empty()) tPDM.cleanUpPDCache();

	if (pUpdate) _updateArticles(tSwitched, NULL);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    private func updateProductDBs()
    {
	xOiDebugStartFunc2("xOiPlanning::updateProductDBs", NULL);

	var tPDM = getPDManager();

	if (tPDM == NULL) {
	    xOiDebugPrint(@Info, 1, "no PD manager!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tPIDs = getInfoIDs();
	var tPID;
	foreach(tPID; tPIDs) {
	    xOiDebugPrint(@Info, 2, ["check program ", tPID]);

	    var tPDB = tPDM.getProgPDB(tPID);
	    if (tPDB == NULL) {
		xOiDebugPrint(@Info, 2, "no product DB registered");
		continue;
	    }

	    var tOldType = tPDB.getType();
	    var tNewType = xOiGetAppPDBType(tPID);
	    xOiDebugPrint(@Info, 2,
			  ["old type ", tOldType, " vs. new ", tNewType]);

	    var tOldPath = tPDB.getDataBasePath();
	    var tNewPath = xOiGetAppProductDBPath(tPID);
	    xOiDebugPrint(@Info, 2,
			  ["old path ", tOldPath, " vs. new ", tNewPath]);

	    if (tNewType != NULL && tOldType != tNewType) {
	        xOiDebugPrint(@Info, 1,
			      ["setting new product DB type for ", tPID, ": ",
			       tNewType]);
		tPDM.delProductDB(tPDB.getID());
	        var tProgID = tPID; 
		addProductDB(tNewType, tProgID, tNewPath, @(tProgID));
	    }
	    else
	    if (tNewPath != NULL && tNewPath != tOldPath) {
	        xOiDebugPrint(@Info, 1,
			      ["setting new product data path for ", tPID, ": ",
			       tNewPath]);
	    	tPDB.setDataBasePath(tNewPath);
		if (tPDB.hasMember(@updateTables)) tPDB.updateTables();
	    }
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    private func updateLibPaths()
    {
	xOiDebugStartFunc2("xOiPlanning::updateLibPaths", NULL);

	var tPIDs = getInfoIDs();
	var tPID;
	foreach(tPID; tPIDs) {
	    var tNewPath = xOiGetAppLibPath(tPID);;
	    var tOldPath = NULL;
	    if (mLibPaths.hasKey(tPID))
	    	tOldPath = mLibPaths[tPID];
	    xOiDebugPrint(@Info, 2,
			  ["check library paths for ", tPID, ": old=",
			   tOldPath, ", new =", tNewPath]);
	    if (tNewPath != NULL && tNewPath != tOldPath) {
	        xOiDebugPrint(@Info, 1,
			      ["new library path for ", tPID, ": ",
			       tNewPath]);
		mLibPaths[tPID] = tNewPath;
	    }
	}

	xOiDebugFinishFunc(NULL);
    }

    
    private func reRegisterPackages()
    {
	var tPIDs = getInfoIDs();
	var tPID;
	foreach(tPID; tPIDs) xOiRegisterPackage(tPID);
    }

    
    
    
    
    private func getDefaultLibPaths()
    {
	mLibPaths = Hash();

	var tPID;
	foreach(tPID; getInfoIDs()) {
	    var tID = tPID; 
	    mLibPaths[tID] = xOiProgID2Path(tID);
	}
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getProgElements(pPID, ...)
    {
	xOiDebugStartFunc("xOiPlanning::getProgElements", pPID);
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugPrint(@Warn, 1, "Obsolete method!");

	var tPID = pPID[0];
	var tCount = 0;
	if (pPID.size() > 2)
	    tCount = pPID[2];

	var tEL = @();

	var tE;
	foreach(tE; getElements())
	    if ((tE.isA(OiPlElement) ||
		 (getPlanningMode() == 4 && tE.isA(OiPart))) &&
		tE.getProgram() == tPID) {
		var tEl = tE;
		tEL.pushBack(tEl);
	    }
	var tRes = tCount ? tEL.size() : tEL;
	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getObjPosition(pObj, pCS)
    {
	xOiDebugStartFunc("xOiPlanning::getObjPosition", [pObj, pCS]);

	var tValidCS = @(@WC, @LC);
	var tCS = pCS;
	if (tValidCS.find(tCS) < 0)
	    tCS = @WC;

	var tPos;
	if (tCS == @WC)
	    tPos = pObj.getWorldPosition();
	else
	    tPos = pObj.getPosition();

	var tTrAxis = pObj.getTrAxis();
	if (pObj.isCat(@FREE_MOVING) || pObj.isCat(@UNRESTRICTED_MOVING))
	    tTrAxis = 7;

	var tDOG = [(tTrAxis & 1) > 0, (tTrAxis & 2) > 0, (tTrAxis & 4) > 0];

	var tRes = [tPos, tDOG];

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setFreeObjPosition(pObj, pCS, pMode, pPos, ...)
    {
	var tPos = pPos[0];
	var tConfirm = 1;
	if (pPos.size() > 1)
	    tConfirm = pPos[1];

	xOiDebugStartFunc("xOiPlanning::setFreeObjPosition",
			  [pObj, pCS, pMode, tPos, tConfirm]);

        tPos = _setFreeObjPosition(pObj, pCS, pMode, tPos, tConfirm, 0);

	if (pCS == @WC)
	    tPos = pObj.getWorldPosition();

	xOiDebugFinishFunc(tPos);
	return(tPos);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkFreeObjPosition(pObj, pCS, pMode, pPos)
    {
        return(_setFreeObjPosition(pObj, pCS, pMode, pPos, 1, 1));
    }

    
    
    
    
    
    
    
    
    
    public func assignFreeObjPosition(pObj, pPos)
    {
        return(_setFreeObjPosition(pObj, @LC, @absolute, pPos, 0, 0));
    }

    private func _setFreeObjPosition(pObj, pCS, pMode, pPos, pConfirm, pNoSet)
    {
	xOiDebugStartFunc("xOiPlanning::_setFreeObjPosition",
			  [pObj, pCS, pMode, pPos, pConfirm, pNoSet]);

	var tValidCS = @(@WC, @FC, @LC);
	var tCS = pCS;
	if (tValidCS.find(tCS) < 0)
	    tCS = @WC;

	var tValidModes = @(@absolute, @relative);
	var tMode = pMode;
	if (tValidModes.find(tMode) < 0)
	    tMode = @absolute;

	var tFather = pObj.getFather();
	var tOldPos = pObj.getPosition();

	if (!tFather.hasMember(@checkChildColl)) {
	    if (pConfirm && pNoSet) tOldPos = NULL;
	    xOiDebugPrint(@ExplWarn, 1,
			  ["cannot check collision at ", tFather]);
	    xOiDebugFinishFunc(tOldPos);
	    return(tOldPos);
	}

	var tNewPos = pPos; 

	if (tMode == @relative && tCS == @LC)
	    tNewPos = _getMovePos(pObj, tOldPos, pPos);
	else
	if (tMode == @relative && (tCS == @WC || tCS == @FC))
	    tNewPos = [ tOldPos[0] + pPos[0],
			tOldPos[1] + pPos[1],
			tOldPos[2] + pPos[2]];
	else
	if (tMode == @absolute && tCS == @WC) {
	    var tOldPosWC = pObj.getWorldPosition();
	    tNewPos = [ tOldPos[0] + pPos[0] - tOldPosWC[0],
			tOldPos[1] + pPos[1] - tOldPosWC[1],
			tOldPos[2] + pPos[2] - tOldPosWC[2]];
	}

	xOiDebugPrint(@Info, 2, ["new pos: ", tNewPos]);

	var tCat = NULL;
	if (pObj.isCat(@FREE_MOVING))
	    tCat = @FREE_MOVING;
	if (pObj.isCat(@UNCONSTRAINED_MOVING) && tCat == NULL)
	    
	    tCat = @UNCONSTRAINED_MOVING;
	if (pObj.isCat(@UNRESTRICTED_MOVING) && tCat != @FREE_MOVING) {
	    
	    if (tCat == @UNCONSTRAINED_MOVING)
		xOiDebugPrint(@ExplWarn, 1,
			      ["ambigous categories ", tCat,
			       " <> @UNRESTRICTED_MOVING for ", pObj]);
	    tCat = @UNRESTRICTED_MOVING;
	}
	xOiDebugPrint(@Info, 2, ["effective category: ", tCat]);

	var tDoF = pObj.getTrAxis();

	if (tCat == @UNRESTRICTED_MOVING) pObj.setTrAxis(7);

	if (tCat == @UNCONSTRAINED_MOVING && tDoF != 7)
	    
	    tNewPos = [tDoF & 1 ? tNewPos[0] : tOldPos[0],
		       tDoF & 2 ? tNewPos[1] : tOldPos[1],
		       tDoF & 4 ? tNewPos[2] : tOldPos[2]];

	if (tCat == @FREE_MOVING || tCat == @UNCONSTRAINED_MOVING)
	    pObj.setPosition(tNewPos);
	else 
	    
	    pObj.translate([tNewPos[0]-tOldPos[0],
			    tNewPos[1]-tOldPos[1],
			    tNewPos[2]-tOldPos[2]]);

	var tMsg = "::ofml::xoi::@confirm_collpos";

	if (pConfirm && tFather.checkChildColl(pObj, NULL) != NULL &&
	    !xOiAskYesNo(tMsg, @QUESTION, getLanguage())) {
	    if (pObj.isCat(@FREE_MOVING))
	        pObj.setPosition(tOldPos);
	    else
	        pObj.translate([tOldPos[0]-tNewPos[0],
			        tOldPos[1]-tNewPos[1],
			        tOldPos[2]-tNewPos[2]]);
	    if (pNoSet)
		tNewPos = NULL;
	    else
		tNewPos = tOldPos;
	}
	else
	    if (pNoSet) {
	        if (pObj.isCat(@FREE_MOVING))
		    pObj.setPosition(tOldPos);
	        else
	    	    pObj.translate([tOldPos[0]-tNewPos[0],
			    	    tOldPos[1]-tNewPos[1],
			    	    tOldPos[2]-tNewPos[2]]);
	    }

	if (tCat == @UNRESTRICTED_MOVING) pObj.setTrAxis(tDoF);

	xOiDebugFinishFunc(tNewPos);
	return(tNewPos);
    }

    private func _getMovePos(pObj, pOldPos, pMV)
    {
	xOiDebugPrint(@Info, 3, ["old pos: ", pOldPos]);

	
	
	
	
	var tRotX = pObj.getRotation(@PX);
	var tRotY = pObj.getRotation(@PY);
	var tRotZ = pObj.getRotation(@PZ);

	xOiDebugPrint(@Info, 3,
		      ["rotations: ",
		       [tRotX*180/xOiPi, tRotY*180/xOiPi, tRotZ*180/xOiPi]]);

	var tP = pMV;	
	
	tP = [tP[0], tP[1]*cos(tRotX) - tP[2]*sin(tRotX),
		     tP[1]*sin(tRotX) + tP[2]*cos(tRotX)];
	xOiDebugPrint(@Info, 3, ["point after X rotation: ", tP]);

	
	tP = [ tP[0]*cos(tRotY) + tP[2]*sin(tRotY), tP[1],
	      -tP[0]*sin(tRotY) + tP[2]*cos(tRotY)];
	xOiDebugPrint(@Info, 3, ["point after Y rotation: ", tP]);

	
	tP = [tP[0]*cos(tRotZ) - tP[1]*sin(tRotZ),
	      tP[0]*sin(tRotZ) + tP[1]*cos(tRotZ), tP[2]];
	xOiDebugPrint(@Info, 3, ["point after Z rotation: ", tP]);

	
	tP = [pOldPos[0] + tP[0], pOldPos[1] + tP[1], pOldPos[2] + tP[2]];

	return(tP);
    }

    
    
    
    
    
    
    
    
    
    public func getObjRotation(pObj)
    {
	xOiDebugStartFunc("xOiPlanning::getObjRotation", pObj);

	var tRot = [pObj.getRotation(@PX),
		    pObj.getRotation(@PY),
		    pObj.getRotation(@PZ)];

	var tRtAxis = pObj.getRtAxis();
	if (pObj.isCat(@FREE_MOVING) || pObj.isCat(@UNRESTRICTED_MOVING))
	    tRtAxis = 7;

	var tDOG = [(tRtAxis & 1) > 0, (tRtAxis & 2) > 0, (tRtAxis & 4) > 0];

	var tRes = [tRot, tDOG];

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setFreeObjRotation(pObj, pAxis, pMode, pRot, ...)
    {
	var tRot = pRot[0];
	var tConfirm = 1;
	if (pRot.size() > 1)
	    tConfirm = 0;

        return(_setFreeObjRotation(pObj, pAxis, pMode, tRot, tConfirm, 0));
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkFreeObjRotation(pObj, pAxis, pMode, pRot)
    {
        return(_setFreeObjRotation(pObj, pAxis, pMode, pRot, 1, 1));
    }

    private func _setFreeObjRotation(pObj, pAxis, pMode, pRot, pConfirm, pNoSet)
    {
	xOiDebugStartFunc("xOiPlanning::_setFreeObjRotation",
			  [pObj, pAxis, pMode, pRot, pConfirm, pNoSet]);

	var tValidModes = @(@absolute, @relative);
	var tMode = pMode;
	if (tValidModes.find(tMode) < 0)
	    tMode = @absolute;

	var tFather = pObj.getFather();
	var tOldRot = pObj.getRotation(pAxis);

	if (!tFather.hasMember(@checkChildColl)) {
	    xOiDebugPrint(@ExplWarn, 1,
			  ["cannot check collision at ", tFather]);
	    xOiDebugFinishFunc(tOldRot);
	    return(tOldRot);
	}

	var tAngle = pRot; 

	if (tMode == @absolute)
	    tAngle = pRot - tOldRot;

	xOiDebugPrint(@Info, 2, ["angle: ", tAngle, "/", tAngle*180/xOiPi]);

	var tOldRtAxis = pObj.getRtAxis();
	var tTmpRtAxis = 2;	
	if (pAxis == @PX || pAxis == @NX)
	    tTmpRtAxis = 1;
	else
	if (pAxis == @PZ || pAxis == @NZ)
	    tTmpRtAxis = 4;

	pObj.setRtAxis(tTmpRtAxis);
	pObj.setRecursionState(1);
	pObj.rotate(pAxis, tAngle);

	var tNewRot;

	var tMsg = "::ofml::xoi::@confirm_collpos";

	if (tFather.checkChildColl(pObj, NULL) != NULL && pConfirm &&
	    !xOiAskYesNo(tMsg, @QUESTION, getLanguage())) {
	    pObj.rotate(pAxis, -tAngle);
	    if (pNoSet)
		tNewRot = NULL;
	    else
		tNewRot = tOldRot;
	}
	else {
	    tNewRot = pObj.getRotation(pAxis);
	    xOiDebugPrint(@Info, 2, ["new rotation: ", tNewRot*180/xOiPi]);
	    if (pNoSet) {
		pObj.rotate(pAxis, -tAngle);
		if (tMode == @relative) tNewRot = tAngle;
	    }
	 }

	pObj.setRecursionState(0);
	pObj.setRtAxis(tOldRtAxis);

	xOiDebugFinishFunc(tNewRot);
	return(tNewRot);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getObjDistance(pObj, pDir, pCS, pMode, ...)
    {
	xOiDebugStartFunc("xOiPlanning::getObjDistance",
			  [pObj, pDir, pCS, pMode]);

	var tValidDir = @(@L, @R, @B, @F, @U, @D);
	var tDir = pDir;
	if (tValidDir.find(tDir) < 0)
	    tDir = @R;

	var tValidCS = @(@WC, @FC, @LC);
	var tCS = pCS;
	if (tValidCS.find(tCS) < 0)
	    tCS = @WC;

	var tValidModes = @(@absolute, @relative);
	var tMode = pMode[0];
	if (tValidModes.find(tMode) < 0)
	    tMode = @absolute;

	var tNeedObj = 0;
	if (pMode.size() > 1)
	    tNeedObj = pMode[1];

	var tStrength = 1;
	if (pMode.size() > 2)
	    tStrength = pMode[2];

	var tF     = pObj.getFather();
	var tLPos  = pObj.getPosition();
	var tWPos  = pObj.getWorldPosition();
	var tWYRot = xOiGetWorldRotation(pObj, @PY);
	var tLYRot = pObj.getRotation(@PY);

	if (tCS == @FC && tF.getFather() == NULL)
	    
	    tCS = @WC;

	

	var tA = 0.0; 
	if (tCS == @LC)
	    tA = tWYRot;
	else
	if (tCS == @FC)
	    tA = xOiGetWorldRotation(tF, @PY);

	xOiDebugPrint(@Info, 2, ["global rotation of object: ", tWYRot]);
	xOiDebugPrint(@Info, 2, ["used coordinate system: ", tCS]);
	xOiDebugPrint(@Info, 3, ["base rotation of search vector: ", tA]);

	var tSVSpec = _getSearchVector(tDir, tA);
	var tSV     = tSVSpec[0];
	var tSVYRot = tSVSpec[1]; 
	var tPoints = _getDistSearchPoints(pObj, tDir, tCS,
					   tWPos, tLPos, tWYRot, tLYRot);

	xOiDebugPrint(@Info, 3,
		      ["search vector: ", tSV, ", rotation: ", tSVYRot]);
	xOiDebugPrint(@Info, 3,
		      ["search points: ", tPoints]);

	
	

	var tDL = _getDistances(pObj, tPoints, tSV, tStrength);
	var tDistances = tDL[0];
	var tDObjects = tDL[1];

	xOiDebugPrint(@Info, 3, ["distances: ", tDistances]);

	var tMaxD = NULL;
	if (tDistances.size() > 1)
	    tMaxD = xOiMaxInList(tDistances);
	else
	if (tDistances.size() == 1)
	    tMaxD = tDistances.front();
	xOiDebugPrint(@Info, 2, ["max distance: ", tMaxD]);

	if (tMaxD == NULL || !pObj.getFather().hasMember(@checkChildColl)) {
	    if (tMaxD == NULL)
	        xOiDebugPrint(@Info, 1, "no object in search direction");
	    else
		xOiDebugPrint(@Warn, 1,
			      ["cannot check collision at ", pObj.getFather()]);
	    var tRet = [0.0, 0.0, 0.0];
	    if (tMode == @absolute) {
		tRet = tLPos;
	        if (tCS == @WC) tRet = tWPos;
	    }
	    if (tNeedObj) tRet = [[tRet[0], tRet[1], tRet[2]], NULL]; 

	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tDObj = tDObjects[tDistances.find(tMaxD)];
	xOiDebugPrint(@Info, 2, ["object at distance: ", tDObj]);

	
	

	
	
	var tMVRot = tSVYRot - (tWYRot-tLYRot);
	var tObjMV = [tSV[1] != 0 ? 0.0 :  cos(tMVRot), tSV[1],
		      tSV[1] != 0 ? 0.0 : -sin(tMVRot)];

	var tDist = tMaxD;
	if (tStrength)
	    tDist = _getDistWithoutColl(pObj, tLPos, tObjMV, tMaxD);

	xOiDebugPrint(@Info, 2, ["valid distance: ", tDist]);
	xOiDebugPrint(@Info, 3, ["move vector rotation: ", tMVRot*180/xOiPi]);

	

	tDist = xOiGetRoundFloat(tDist, 3);

	var tRet = [tDist*tObjMV[0], tDist*tObjMV[1], tDist*tObjMV[2]];

	xOiDebugPrint(@Info, 3, ["absolute distance vector (try): ", tRet]);

	
	var tPos = [tLPos[0] + tRet[0], tLPos[1] + tRet[1], tLPos[2] + tRet[2]];
	pObj.setPosition(tPos);
	if (tStrength && tF.checkChildColl(pObj, NULL) != NULL)
	    
	    tRet = [tRet[0]-tObjMV[0]*0.001,
		    tRet[1]-tObjMV[1]*0.001,
		    tRet[2]-tObjMV[2]*0.001];
	tPos = [tLPos[0] + tRet[0], tLPos[1] + tRet[1], tLPos[2] + tRet[2]];
	pObj.setPosition(tPos);
	if (tStrength && tF.checkChildColl(pObj, NULL) != NULL)
	    tRet = NULL;
	pObj.setPosition(tLPos); 

	if (tRet == NULL) {
	    xOiDebugPrint(@ExplWarn, 1,
			  "calculation error in xOiPlanning::getObjDistance()!");
	    tRet = [0.0, 0.0, 0.0];
	    if (tMode == @absolute) {
		tRet = tLPos;
	        if (tCS == @WC) tRet = tWPos;
	    }
	    if (tNeedObj) tRet = [[tRet[0], tRet[1], tRet[2]], NULL]; 

	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	xOiDebugPrint(@Info, 2, ["absolute distance vector: ", tRet]);

	if (tCS == @LC && tMode == @relative) {
	    var tA = tLYRot;
	    tRet = [xOiGetRoundFloat(tRet[0]*cos(tA) - tRet[2]*sin(tA), 3),
		    tRet[1],
		    xOiGetRoundFloat(tRet[0]*sin(tA) + tRet[2]*cos(tA), 3)];
	    xOiDebugPrint(@Info, 2, ["relative distance vector: ", tRet]);
	}

	

	if (tMode == @absolute) {
	    tPos = tLPos; 
	    if (tCS == @WC)
		tPos = tWPos;
	    xOiDebugPrint(@Info, 3, ["reference point: ", tPos]);
	    tRet = [tPos[0] + tRet[0], tPos[1] + tRet[1], tPos[2] + tRet[2]];
	}

	if (tNeedObj) tRet = [[tRet[0], tRet[1], tRet[2]], tDObj]; 

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    private func _getSearchVector(pDir, pYRot)
    {
	if (pDir == @U)
	    return([[0, 1, 0], 0.0]);
	if (pDir == @D)
	    return([[0, -1, 0], 0.0]);
	if (pDir == @R)
	    return([[cos(pYRot), 0, -sin(pYRot)], pYRot]);
	if (pDir == @B)
	    return([[-sin(pYRot), 0, -cos(pYRot)], pYRot+xOiPi2]);
	if (pDir == @L)
	    return([[-cos(pYRot), 0, sin(pYRot)], pYRot+xOiPi]);
	if (pDir == @F)
	    return([[sin(pYRot), 0, cos(pYRot)],  pYRot-xOiPi2]);
    }

    
    
    private func _getDistSearchPoints(pObj, pDir, pCS, pWPos, pLPos,
				      pWYRot, pLYRot)
    {
	
	
	
	
	
	
	
	
	
	
	

	var tO = 0.01;		
	var tR = 0.05;		

	

	var tBPoints = @();	
	var tRVs     = @();	
				
				
	var tGeoBBs = @();	
	var tAngles = @();	

	if (pCS == @FC) {
	    tGeoBBs.pushBack(pObj.getLocalGeoBounds());
	    tAngles.pushBack(pWYRot-pLYRot);
	}
	else
	if (pCS == @LC) {
	    
	    xOiSetYRotation(pObj, 0.0);
	    tGeoBBs.pushBack(pObj.getLocalGeoBounds());
	    tAngles.pushBack(pWYRot);
	    xOiSetYRotation(pObj, pLYRot); 
	}
	else
	if (pCS == @WC) {
	    var tWYRot = fabs(pWYRot);
	    if (tWYRot < 0.0001 || fabs(tWYRot-xOiPi2) < 0.0001 ||
		fabs(tWYRot-xOiPi) < 0.0001 ||
		fabs(tWYRot-xOiPi-xOiPi2) < 0.0001) {
	        tGeoBBs.pushBack(pObj.getWorldGeoBounds());
	        tAngles.pushBack(NULL);
	    }
	    else {
		
		var tA = pWYRot;
		if (tA < 0)
		    tA += xOi2Pi;
		var tA1 = Int(tA/xOiPi2) * xOiPi2;
		var tA2 = tA1 + xOiPi2;
		tA1 = tA1 - tA; 
		tA2 = tA2 - tA; 
		pObj.setRecursionState(1);
		pObj.rotate(@PY, tA1);
	        tGeoBBs.pushBack(pObj.getWorldGeoBounds());
	        tAngles.pushBack(-tA1);
		pObj.rotate(@PY, xOiPi2);
	        tGeoBBs.pushBack(pObj.getWorldGeoBounds());
	        tAngles.pushBack(-tA2);
		pObj.rotate(@NY, tA2); 
		pObj.setRecursionState(0);
	    }
	}

	xOiDebugPrint(@Info, 2, ["bounds: ", tGeoBBs]);

	var tGeoBB;
	foreach(tGeoBB; tGeoBBs)
	    _getBaseSearchPoints(tBPoints, tRVs, pDir, tGeoBB, tO);

	xOiDebugPrint(@Info, 2, ["base points: ", tBPoints]);
	xOiDebugPrint(@Info, 2, ["transformation angles: ", tAngles]);

	

	var tPoints = @();	

	var tSize = tGeoBBs.size() * 4;
	var i;
	for(i = 0; i < tSize; i++) {
	    var tA    = tAngles[Int(i/4)]; 
	    var tBP1  = tBPoints[i];
	    var tBP2  = tBPoints[(i+1)%4 + Int(i/4)*4];
	    var tRV   = tRVs[i];
	    var tP    = [tBP1[0], tBP1[1], tBP1[2]];
	    var tAxis = 0;
	    if (tRV[0] == 0 && tRV[2] == 0) tAxis = 1;
	    else
	    if (tRV[0] == 0 && tRV[1] == 0) tAxis = 2;

	    xOiDebugPrint(@Info, 3,
			  ["adding points between ", tBP1, " and ", tBP2,
			   " along axis ", tAxis]);
	    do {
		tPoints.pushBack([[tP[0], tP[1], tP[2]], tA]);
		var tPX = tP[0] + tRV[0]*tR;
		var tPY = tP[1] + tRV[1]*tR;
		var tPZ = tP[2] + tRV[2]*tR;
		if (fabs(tPX) < 0.000001) tPX = 0.0;
		if (fabs(tPY) < 0.000001) tPY = 0.0;
		if (fabs(tPZ) < 0.000001) tPZ = 0.0;
		tP = [tPX, tPY, tPZ];
	        xOiDebugPrint(@Info, 4, ["check new point ", tP]);
	    }
	    while ((tRV[tAxis] > 0 && tP[tAxis] < tBP2[tAxis]) ||
		   (tRV[tAxis] < 0 && tP[tAxis] > tBP2[tAxis]));
	}

	

	var tPos = pWPos;
	if (pCS != @WC)
	    tPos = [0.0, 0.0, 0.0]; 

	xOiDebugPrint(@Info, 3, ["origin for transformation: ", tPos]);

	var tTPoints = @(); 
	var tPA;
	foreach(tPA; tPoints) {
	    var tP = tPA[0];
	    var tA = tPA[1];
	    if (tA == NULL) {
		tTPoints.pushBack([tP[0], tP[1], tP[2]]);
		continue;
	    }
	    xOiDebugPrint(@Info, 4, ["transform point ", tP]);
	    var tX = tP[0] - tPos[0];
	    var tY = tP[1] - tPos[1];
	    var tZ = tP[2] - tPos[2];
	    var tWPos = xOiTransformXZCoords(pWPos, tA, tX, tZ);
	    tWPos[1] = tY + pWPos[1];
	    xOiDebugPrint(@Info, 4, ["origin ", tPos]);
	    if (fabs(tWPos[0]) < 0.000001) tWPos[0] = 0.0;
	    if (fabs(tWPos[1]) < 0.000001) tWPos[1] = 0.0;
	    if (fabs(tWPos[2]) < 0.000001) tWPos[2] = 0.0;
	    xOiDebugPrint(@Info, 4,
			  ["transform ", [[tX, tY, tZ], tA], " -> ", tWPos]);
	    tTPoints.pushBack([tWPos[0], tWPos[1], tWPos[2]]);
	}
	return(tTPoints);
    }

    
    
    
    
    
    
    private func _getBaseSearchPoints(pBPoints, pRVs, pDir, pGeoBB, pO)
    {
	var tBPoints, tRVs;

	if (pDir == @D) {
	    
	    tBPoints =
		[[pGeoBB[0][0]+pO, pGeoBB[0][1], pGeoBB[0][2]+pO], 
		 [pGeoBB[0][0]+pO, pGeoBB[0][1], pGeoBB[1][2]-pO], 
		 [pGeoBB[1][0]-pO, pGeoBB[0][1], pGeoBB[1][2]-pO], 
		 [pGeoBB[1][0]-pO, pGeoBB[0][1], pGeoBB[0][2]+pO]  
		];
	    tRVs = [[0, 0, 1], [1, 0, 0], [0, 0, -1], [-1, 0, 0]];
	}
	else
	if (pDir == @U) {
	    
	    tBPoints =
		[[pGeoBB[0][0]+pO, pGeoBB[1][1], pGeoBB[0][2]+pO], 
		 [pGeoBB[0][0]+pO, pGeoBB[1][1], pGeoBB[1][2]-pO], 
		 [pGeoBB[1][0]-pO, pGeoBB[1][1], pGeoBB[1][2]-pO], 
		 [pGeoBB[1][0]-pO, pGeoBB[1][1], pGeoBB[0][2]+pO]  
		];
	    tRVs = [[0, 0, 1], [1, 0, 0], [0, 0, -1], [-1, 0, 0]];
	}
	else
	if (pDir == @L) {
	    
	    tBPoints =
		[[pGeoBB[0][0], pGeoBB[0][1]+pO, pGeoBB[0][2]+pO], 
		 [pGeoBB[0][0], pGeoBB[1][1]-pO, pGeoBB[0][2]+pO], 
		 [pGeoBB[0][0], pGeoBB[1][1]-pO, pGeoBB[1][2]-pO], 
		 [pGeoBB[0][0], pGeoBB[0][1]+pO, pGeoBB[1][2]-pO]  
		];
	    tRVs = [[0, 1, 0], [0, 0, 1], [0, -1, 0], [0, 0, -1]];
	}
	else
	if (pDir == @R) {
	    tBPoints =
		[[pGeoBB[1][0], pGeoBB[0][1]+pO, pGeoBB[0][2]+pO], 
		 [pGeoBB[1][0], pGeoBB[1][1]-pO, pGeoBB[0][2]+pO], 
		 [pGeoBB[1][0], pGeoBB[1][1]-pO, pGeoBB[1][2]-pO], 
		 [pGeoBB[1][0], pGeoBB[0][1]+pO, pGeoBB[1][2]-pO]  
		];
	    tRVs = [[0, 1, 0], [0, 0, 1], [0, -1, 0], [0, 0, -1]];
	}
	else
	if (pDir == @B) {
	    tBPoints =
		[[pGeoBB[0][0]+pO, pGeoBB[0][1]+pO, pGeoBB[0][2]], 
		 [pGeoBB[0][0]+pO, pGeoBB[1][1]-pO, pGeoBB[0][2]], 
		 [pGeoBB[1][0]-pO, pGeoBB[1][1]-pO, pGeoBB[0][2]], 
		 [pGeoBB[1][0]-pO, pGeoBB[0][1]+pO, pGeoBB[0][2]]  
		];
	    tRVs = [[0, 1, 0], [1, 0, 0], [0, -1, 0], [-1, 0, 0]];
	}
	else
	if (pDir == @F)  {
	    tBPoints =
		[[pGeoBB[0][0]+pO, pGeoBB[0][1]+pO, pGeoBB[1][2]], 
		 [pGeoBB[0][0]+pO, pGeoBB[1][1]-pO, pGeoBB[1][2]], 
		 [pGeoBB[1][0]-pO, pGeoBB[1][1]-pO, pGeoBB[1][2]], 
		 [pGeoBB[1][0]-pO, pGeoBB[0][1]+pO, pGeoBB[1][2]]  
		];
	    tRVs = [[0, 1, 0], [1, 0, 0], [0, -1, 0], [-1, 0, 0]];
	}
	xOiCopyAggr(tBPoints, pBPoints, 1);
	xOiCopyAggr(tRVs, pRVs, 1);
    }

    
    
    
    private func _getDistances(pObj, pPoints, pSV, pStrength)
    {
	xOiDebugStartFunc2("xOiPlanning::_getDistances",
			   [pObj, pPoints, pSV, pStrength]);

	var tRSV = [-pSV[0], -pSV[1], -pSV[2]]; 
	var tOff = 0.0001;
	var tDistances = @();
	var tDObjects = @();
	var tP;
	foreach(tP; pPoints) {
            var tD = oiGetDistance(tP, pSV);

	    xOiDebugPrint(@Info, 3, ["distance from point ", tP, ": ", tD]);

	    if (tD < 0.0) continue;

	    if (tD < tOff) {
		
		
		
	        var tP1 = [tP[0]+pSV[0]*tOff, tP[1]+pSV[1]*tOff,
			   tP[2]+pSV[2]*tOff];
                var tD1 = oiGetDistance(tP1, pSV);
	        xOiDebugPrint(@Info, 3,
			      ["distance from check point ", tP1, ": ", tD1]);
		if (tD1 > 0.0 && tD1 > tD+tOff) {
		    tP = tP1;
		    tD = tD1;
		}
	    }

	    var tDObj = oiGetNearestObject(tP, pSV);
	    xOiDebugPrint(@Info, 2, ["object at distance is ", tDObj]);

	    if (pStrength) {
	        
	        
	        var tObj = oiGetNearestObject(tP, tRSV);
	        xOiDebugPrint(@Info, 3, ["reverse nearest ", tObj]);
	        if (tObj != NULL && !xOiIsObjChildOf(tObj, pObj)) continue;
	    }

	    
	    if (tD >= 0.001) {
		tDistances.pushBack(tD);
		tDObjects.pushBack(tDObj);
	    }
	}
	xOiDebugFinishFunc([tDistances, tDObjects]);
	return([tDistances, tDObjects]);
    }

    
    
    
    private func _getDistWithoutColl(pObj, pPos, pMV, pMaxD)
    {
	var tFather = pObj.getFather();
	var tCR     = 0.05;	
	var tMinD   = 0.0;	
	var tMaxD   = pMaxD;	

	xOiDebugPrint(@Info, 3, ["check with raster ", tCR]);

	var tP, tD;
	for (tD=tCR; tD < pMaxD; tD += tCR) {
	    tP = [pPos[0] + tD*pMV[0],
		  pPos[1] + tD*pMV[1],
		  pPos[2] + tD*pMV[2]];
	    xOiDebugPrint(@Info, 3, ["check pos: ", tP]);
	    pObj.setPosition(tP);
	    if (tFather.checkChildColl(pObj, NULL) == NULL)
		tMinD = tD;
	    else {
		tMaxD = tD;
		break;
	    }
	}

	xOiDebugPrint(@Info, 3, ["check in interval ", [tMinD, tMaxD]]);

	var tS = -1;
	tD = tMaxD;

	while (tMaxD-tMinD >= 0.001) {
	    tD = tD + tS*(tMaxD-tMinD)/2;
	    tP = [pPos[0] + tD*pMV[0],
		  pPos[1] + tD*pMV[1],
		  pPos[2] + tD*pMV[2]];
	    xOiDebugPrint(@Info, 3, ["check pos: ", tP]);
	    pObj.setPosition(tP);
	    if (tFather.checkChildColl(pObj, NULL) == NULL) {
		tMinD = tD;
		tS = 1;
	    }
	    else {
		tMaxD = tD;
		tS = -1;
	    }
	}

	pObj.setPosition(pPos);

	return(tMinD+(tMaxD-tMinD)/2);
    }

    
    
    

    
    
    
    
    
    
    public func isAtEval()
    {
	return(mAtEval);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func issueMessageOnce(pMsg, ...)
    {
	var tMsg = pMsg[0];
	var tType = @MESSAGE;
	if (pMsg.size() > 1)
	    tType = pMsg[1];
	var tPackage = "::ofml::xoi";
	if (pMsg.size() > 2)
	    tPackage = pMsg[2];

	if (self.getAppMode(@MultiFMLProcessing)) {
	    if (xOiGetAppMultiFMLProcessState(tMsg) == NULL) {
		oiOutput(tType, tPackage+"::@"+String(tMsg));
		xOiSetAppMultiFMLProcessState(tMsg, 1);
	    }
	    return;
	}

	if (mXoiPrintedMessages.find(tMsg) < 0) {
	    oiOutput(tType, tPackage+"::@"+String(tMsg));
	    mXoiPrintedMessages.pushBack(tMsg);
	}
    }

    public func measure(pMode)
    {
	xOiDebugStartFunc("xOiPlanning::measure", pMode);

	mMeasure = Vector(0);
	mMeasure.pushBack(pMode);

	try {
	xOiBasePlanning::measure(pMode);
	}
	catch(&e: Error) {
	    xOiDebugPrint(@EvalErr, 1, 
			  "evaluation error during xOiPlanning::measure(): " +
			  e.where + ": " + e.what);
	}

	xOiDebugFinishFunc(NULL);
    }

    public func unMeasure()
    {
	mMeasure = NULL;

	xOiBasePlanning::unMeasure();
    }

    
    
    
    
    
    
    
    
    public func useSavedPD()
    {
	var tPossibleResL = @(@Yes, @No, @Dialog);
	var tRes = NULL;

	try {
	    tRes = ::ofml::app::useSavedPD(xOiParam2Str(getInfoIDs()));
	}
	catch (&e: Error) {
	    xOiDebugPrint(@ExplWarn, 1,
			  "Application does not support package ::ofml::app\n" +
			  "or that package does not implement function " +
			  "useSavedPD()!");
	}
	if (tRes == NULL)
	    tRes = @No;
	else {
	    tRes = eval(tRes);
	    if (!(tRes instanceof Symbol) || tPossibleResL.find(tRes) < 0)
		tRes = @No;
	}

	return(tRes);
    }

    
    
    
    
    public func exclClassFromBorderCheck()
    {
	var tCL = OiPlanning::exclClassFromBorderCheck();
	tCL.pushBack(xOiOrderItem);
	tCL.pushBack(@TEXT3D);
    }

    public func setCheckAddPos(pPos)
    {
        mCheckAddPos = pPos;
    }

    
    
    public func isValidForCollCheck(pCh)
    {
	if (pCh.isA(xOiOrderItem) || pCh.isA(OiChangeManager) ||
	    pCh.isA(xOiOAPManager) || pCh.isA(xOiOdbParamManager)) return(0);

	return(xOiBasePlanning::isValidForCollCheck(pCh));
    }

    private func _getDistToNextObj(pP, pR, pW, pD)
    {
	xOiDebugPrint(@Info, 2, ["world pos: ", pP, ", rotation: ", pR]);
	xOiDebugPrint(@Info, 2, ["width: ", pW, ", depth: ", pD]);

	var tH    = 0.01;
	var tPDir = getPlanDirXZ1();
        var tS    = planDirSign(tPDir);
	var tR    = pR;
	if (tPDir == @F || tPDir == @B) tR -= xOiPi2;

        var tV  = [tS*cos(tR), 0, -tS*sin(tR)]; 
        var tPW = [pP[0] + (tS+1)/2*pW*cos(tR), tH,
                   pP[2] - (tS+1)/2*pW*sin(tR)];

	xOiDebugPrint(@Info, 2, ["check pos 1: ", tPW, ", dir: ", tV]);

        var tD1 = oiGetDistance(tPW, tV);

	
	
	
	var tPW2 = xOiTransformXZCoords(tPW, tR, 0.0, -0.0001);
        var tD2  = oiGetDistance(tPW2, tV);
	var tPW3 = xOiTransformXZCoords(tPW, tR, 0.0, 0.0001);
        var tD3  = oiGetDistance(tPW3, tV);

	xOiDebugPrint(@Info, 3, ["check pos 2: ", tPW2, ", dir: ", tV]);
	xOiDebugPrint(@Info, 3, ["check pos 3: ", tPW3, ", dir: ", tV]);
	xOiDebugPrint(@Info, 2, ["distances: ", @(tD1, tD2, tD3)]);

	var tD = tD1;
	if (tD2 >= 0.0 && (tD < 0.0 || tD > tD2)) tD = tD2;
	if (tD3 >= 0.0 && (tD < 0.0 || tD > tD3)) tD = tD3;

        if (tD > -0.1) return(tD);

        

        var tBdPos = getBorder()[0];
        var tBdDim = getBorder()[1];

        if (tBdDim[0] < eps || tBdDim[1] < eps || tBdDim[2] < eps) return(-1);

        if (tBdPos == NULL)
            if (hasBorderBox())
                tBdPos = xOiGetBoundsFor(getElements(), NULL)[0];
            else
                tBdPos = getWorldBounds()[0];

        
        var tBP1 = [tBdPos[0], tBdPos[2]];                      
        var tBP2 = [tBdPos[0]+tBdDim[0], tBdPos[2]];            
        var tBP3 = [tBdPos[0]+tBdDim[0], tBdPos[2]+tBdDim[2]];  
        var tBP4 = [tBdPos[0], tBdPos[2]+tBdDim[2]];            
        var tBL1 = [tBP1, tBP2];                                
        var tBL2 = [tBP2, tBP3];                                
        var tBL3 = [tBP3, tBP4];                                
        var tBL4 = [tBP4, tBP1];                                

        
        var tL1, tL2;
        if (tV[0] >= 0.0 && tV[2] >= 0.0)
            if (tV[0] > tV[2])  {
                tL1 = tBL2; tL2 = tBL3;
            }
            else {
                tL1 = tBL3; tL2 = tBL2;
            }
        else
        if (tV[0] >= 0.0 && tV[2] < 0.0)
            if (tV[0] > fabs(tV[2])) {
                tL1 = tBL2; tL2 = tBL1;
            }
            else {
                tL1 = tBL1; tL2 = tBL2;
            }
        else
        if (tV[0] < 0.0 && tV[2] < 0.0)
            if (fabs(tV[0]) > fabs(tV[2])) {
                tL1 = tBL4; tL2 = tBL1;
            }
            else {
                tL1 = tBL1; tL2 = tBL4;
            }
        else
        if (tV[0] < 0.0 && tV[2] >= 0.0)
            if (fabs(tV[0]) > tV[2]) {
                tL1 = tBL4; tL2 = tBL3;
            }
            else {
                tL1 = tBL3; tL2 = tBL4;
            }

        tPW = [pP[0] + pW*cos(pR), 0.1, pP[2] - pW*sin(pR)];

        var tVL = [[pP[0], pP[2]], [tPW[0], tPW[2]]];   
        var tIP = xOiLineIntersection(tVL, tL1);

        if (tIP == NULL || !xOiPointInArea(tIP, tL1[0], tL1[1])) {
            tIP = xOiLineIntersection(tVL, tL2);
            if (tIP == NULL || !xOiPointInArea(tIP, tL2[0], tL2[1])) {
                
		xOiDebugPrint(@EvalErr, 1,
			      "xOiPlanning::getDistToNextObj - internal error");
                return(-1);
            }
        }

        if (getPlanDirXZ1() == @R)
            return(xOiGetDistance([tPW[0], tPW[2]], tIP));
        else
            return(xOiGetDistance([pP[0], pP[2]], tIP));
    }

    
    
    
    
    
    
    
    
    
    public func getDistToNextObj(pObj)
    {
	xOiDebugStartFunc("xOiPlanning::getDistToNextObj", pObj);

        var tP = pObj.getWorldPosition();
        var tR = pObj.getRotation(@PY);

        var tW = pObj.getWidth();
        var tD = pObj.getDepth();

        var tDist = _getDistToNextObj(tP, tR, tW, tD);

	xOiDebugFinishFunc(tDist);
        return(tDist);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func getDistForNew(pObj, pNew)
    {
	xOiDebugStartFunc("xOiPlanning::getDistForNew", [pObj, pNew]);

        var tPR = oiGetPosRot4AttachPts(pObj, pNew);
	if (tPR == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tP = tPR[0];
	var tR = tPR[1];

	var tW = pNew.getWidth();
	var tD = pNew.getDepth();

	var tDist = _getDistToNextObj(tP, tR, tW, tD);

	xOiDebugFinishFunc(tDist);
	return(tDist);
    }

    
    
    

    
    
    
    
    
    public func disintegrateComposite(pObj, pPolicy, pInvalidate,
				      pTrAxis, pRtAxis, pCutable)
    {
	xOiDebugStartFunc("xOiPlanning::disintegrateComposite",
			  [pObj, pPolicy, pInvalidate, 
			   pTrAxis, pRtAxis, pCutable]);
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func createFreePlElement(pSourceObj, pMode, ...)
    {
	xOiDebugStartFunc("xOiPlanning::createFreePlElement",
			  [pSourceObj, pMode]);

	var tCreationMode = pMode[0];

	if (pMode.size() > 1)
	    sFreePlElemModes = pMode[1];

	if (pSourceObj == NULL) {
	    xOiDebugPrint(@EvalErr, 1,
			  "no object passed to createFreePlElement()!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}
	if (getElements().find(pSourceObj) < 0 ||
	    !(pSourceObj.isA(OiPlElement) || pSourceObj.isA(OiPart)) ||
	    pSourceObj.isA(xOiFreePlElement) ||
	    !pSourceObj.hasMember(@getArticleObj) ||
	    pSourceObj.getArticleObj().getProgram() == NULL) {
	    xOiDebugPrint(@EvalErr, 1, "Object passed to createFreePlElement()"+
				       " is not a top planning element!");
	    
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}
	var tValidCModes = @(0, 1);
	if (tValidCModes.find(tCreationMode) < 0) {
	    xOiDebugPrint(@ExplWarn, 1,
			  ["ignoring invalid creation mode ", tCreationMode,
			   " for createFreePlElement(), using 1 instead"]);
	    tCreationMode = 1;
	}
	if (tCreationMode == 0) {
	    
	    
	    oiClearHistory();
	}

	var tPos;

	if (tCreationMode == 0) {
	    tPos = pSourceObj.getPosition();
            setMethod("onCreate(" + String(pSourceObj.getRotation(@PY)) +
		      ", NULL, NULL)");
	}
	else
	if (tCreationMode == 1) {
	    
	    tPos = doCheckAdd(xOiFreePlElement, pSourceObj, NULL,
			      [NULL, pSourceObj]);
	    if (tPos == NULL) {
	        xOiDebugFinishFunc(NULL);
	        return(NULL);
	    }
	}

	var tNewObj = add(xOiFreePlElement, pSourceObj);
	tNewObj.setPosition(tPos);
	eval("tNewObj."+getMethod()); 

	if (tCreationMode == 0) remove(pSourceObj);

	xOiDebugFinishFunc(tNewObj);
	return(tNewObj);
    }

    
    func getFreePlElemModes()
    {
	if (sFreePlElemModes != NULL) return(sFreePlElemModes);

	return(@(@COM, @MAT, @GEO));
    }

    
    
    
    
    
    
    public func onEnvironmentRemoval()
    {
	xOiDebugStartFunc("xOiPlanning::onEnvironmentRemoval", NULL);
	xOiDebugPrint(@Warn, 1, "Obsolete method!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func hideElements(pObj, pCategories, ...)
    {
	xOiDebugStartFunc("xOiPlanning::hideElements", [pObj, pCategories]);

	var tCategories = pCategories[0];
	var tAreProgIDs = 0;
	if (pCategories.size() > 1)
	    tAreProgIDs = pCategories[1];

	if (pObj != NULL) {
	    var tRoot = pObj.getRoot();
	    if (tRoot != self && tRoot.isA(xOiPlanning)) {
		tRoot.hideElements(pObj, tCategories, tAreProgIDs);
		xOiDebugPrint(@Info, 1, "delegating to other root object");
		xOiDebugFinishFunc(NULL);
		return;
	    }
	}

	if (pObj != NULL && pObj != self && pObj.hasMember(@hideElements)) {
	    xOiDebugPrint(@Info, 1, "delegating to object");
	    tCategories = pObj.hideElements(tCategories, tAreProgIDs);
	    if (tCategories == NULL || !(tCategories instanceof List)) {
	        xOiDebugPrint(@Warn, 1, ["invalid return value from object's ",
					 "hideElements(): ", tCategories]);
		tCategories = @();
	    }
	    xOiDebugPrint(@Info, 1, ["categories/programs to process further: ",
				     tCategories]);
	}

	var tObj = pObj;
	if (tObj == NULL) {
	    tObj = self;
	    
	    var tList = tAreProgIDs ? mHiddenProgs : mHiddenCats;
	    var tCatProgs = @(); 
	    xOiCopyAggr(tCategories, tCatProgs, 0);
	    tCatProgs.sort();
	    tList.merge(tCatProgs);
	    tList.unique();
	}

	_hideShowElements(tObj, 1, tCategories, tAreProgIDs, 0);

	xOiDebugPrint(@Info, 2, ["elements hidden by me: ", mHiddenElems]);
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func showElements(pObj, pCategories, ...)
    {
	xOiDebugStartFunc("xOiPlanning::showElements", [pObj, pCategories]);

	var tCategories = pCategories[0];
	var tAreProgIDs = 0;
	if (pCategories.size() > 1)
	    tAreProgIDs = pCategories[1];

	if (pObj != NULL) {
	    var tRoot = pObj.getRoot();
	    if (tRoot != self && tRoot.isA(xOiPlanning)) {
		tRoot.showElements(pObj, tCategories, tAreProgIDs);
		xOiDebugPrint(@Info, 1, "delegating to other root object");
		xOiDebugFinishFunc(NULL);
		return;
	    }
	}

	if (pObj != NULL && pObj != self && pObj.hasMember(@showElements)) {
	    xOiDebugPrint(@Info, 1, "delegating to object");
	    tCategories = pObj.showElements(tCategories, tAreProgIDs);
	    if (tCategories == NULL || !(tCategories instanceof List)) {
	        xOiDebugPrint(@Warn, 1, ["invalid return value from object's ",
					 "showElements(): ", tCategories]);
		tCategories = @();
	    }
	    xOiDebugPrint(@Info, 1, ["categories/programs to process further: ",
				     tCategories]);
	}

	var tObj = pObj;
	if (tObj == NULL) {
	    tObj = self;
	    
	    var tList = tAreProgIDs ? mHiddenProgs : mHiddenCats;
	    var tCat;
	    foreach(tCat; tCategories) tList.remove(tCat);
	}

	_hideShowElements(tObj, 0, tCategories, tAreProgIDs, 0);

	xOiDebugPrint(@Info, 2, ["elements hidden by me: ", mHiddenElems]);
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    public func getHiddenCategories()
    {
	return(mHiddenCats);
    }

    
    
    
    public func getHiddenPrograms()
    {
	return(mHiddenProgs);
    }

    
    
    
    
    private func checkCategories(pObj, pCategories, pAreProgIDs)
    {
	xOiDebugStartFunc("xOiPlanning::checkCategories",
			   [pObj, pCategories, pAreProgIDs]);

	var tRet = 0;

	if (pAreProgIDs) {
	    if (pObj.isCat(@IF_Base) &&
		pCategories.find(pObj.getProgram()) >= 0) tRet = 1;
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tCat;
	foreach(tCat; pCategories)
	    if (pObj.isCat(tCat) &&
		(tCat != @IF_Article || pObj.getArticleSpec() != NULL)) {
		tRet = 1;
		break;
	    }

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    private func _hideShowElements(pObj, pHide, pCategories, pAreProgIDs, 
				   pIsODBChild)
    {
	xOiDebugStartFunc("xOiPlanning::hideShowElements",
			   [pObj, pHide, pCategories, pAreProgIDs, 
			    pIsODBChild]);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if (pCategories == NULL || pCategories.empty()) {
	    xOiDebugPrint(@Warn, 1, "no categories specified!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tIsODBChild = pIsODBChild;
	if (!tIsODBChild && pObj.isA(OiOdbPlElement))
	    tIsODBChild = 1;

	var tE;
	foreach(tE; pObj.getChildren()) {
	    
	    xOiDebugPrint(@Info, 3, ["check ", tE]);
	    var tHasCat = (tE.hasMember(@isCat));
	    if (tE.isA(OiProgInfo) || tE.isA(xOiOrderItem) ||
		tE.isA(OiOdbParamManager) || tE.isA(OiChangeManager) ||
		tE.isA(OiPDManager) || (!tHasCat && !tIsODBChild) ||
		(pAreProgIDs && !tE.hasMember(@getProgram))) continue;
	    if (tHasCat && checkCategories(tE, pCategories, pAreProgIDs)) {
		var tObjName = tE.getName();
		var tIsHidden = tE.isHidden();
		var tIsHiddenByMe = (mHiddenElems.find(tObjName) >= 0);
		xOiDebugPrint(@Info, 2, [tObjName, " is hidden? ", tIsHidden,
					 ", by me? ", tIsHiddenByMe]);
		if (pHide) {
		    if (!tIsHidden) {
		    	tE.hide();
			
			
			
			tE.setObjState(@OI_PickTarget, 0);
			tE.setObjState(@OI_CamCollide, 0);
			mHiddenElems.pushBack(tObjName);
		    }
		}
		else {
		    
		    
		    
		    
		    var tAddCheckCats = pAreProgIDs ? mHiddenCats: mHiddenProgs;
		    
		    if (1 && tIsHidden && tIsHiddenByMe &&
			!checkCategories(tE, tAddCheckCats, !pAreProgIDs)) {
		    	tE.show();
			
			tE.setObjState(@OI_PickTarget, 1);
			tE.setObjState(@OI_CamCollide, 1);
			mHiddenElems.remove(tObjName);
		    }
		    
		    
		    
		    
		    if (0 && tIsHiddenByMe &&
			!checkCategories(tE, tAddCheckCats, !pAreProgIDs)) {
		    	if (tIsHidden) tE.show();
			
			tE.setObjState(@OI_PickTarget, 1);
			tE.setObjState(@OI_CamCollide, 1);
			mHiddenElems.remove(tObjName);
		    }
		    
		    
		    _hideShowElements(tE, 0, pCategories, pAreProgIDs, tIsODBChild);
		}
	    }
	    else
		_hideShowElements(tE, pHide, pCategories, pAreProgIDs, tIsODBChild);
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getEditOpObj(pObj, pOp)
    {
	xOiDebugStartFunc("xOiPlanning::getEditOpObj", [pObj, pOp]);

	var tValidOps = @(@Delete, @Cut, @Copy, @UnGroup);

	var tRet = NULL;

	if (tValidOps.find(pOp) < 0)
	    xOiDebugPrint(@Warn, 1, "invalid operation!");

	else
	if (pOp == @Copy)
	    tRet = getCopyableObj(pObj);
	else
	if (pOp == @Cut) {
	    tRet = getCutableObj(pObj);
	    xOiDebugPrint(@Info, 2, ["cutable ", tRet]);
	}
	else
	if (pOp == @Delete) {
	    tRet = getDeletableObj(pObj);
	}
	else
	if (pOp == @UnGroup) {
	    tRet = getUngroupableObj(pObj);
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    public func getPasteOrigPropClassDescr(pObj)
    {
	xOiDebugStartFunc("xOiPlanning::getPasteOrigPropClassDescr", pObj);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    public func getCopyableObj(pObj)
    {
	if (pObj == NULL ||
	    (pObj.hasMember(@isCat) && pObj.isCat(@IF_RecoveredObject)))
	    return(NULL);

	if (pObj.isA(xOiGroup) && xOiTraverseHierarchy(pObj, @HasRecovered))
	    return(NULL);

	if (!pObj.hasMember(@isCopyable)) return(getCutableObj(pObj));

	var tState = pObj.isCopyable();

	if (tState == -1) return(NULL);

	if (tState == 1) return(pObj);

	var tF = pObj.getFather();

	while(tF != NULL &&
	      (!tF.hasMember(@isCat) || tF.isCat(@IF_RecoveredObject) ||
	       !tF.hasMember(@isCopyable) || tF.isCopyable() != 1))
	    tF = tF.getFather();

	return(tF);
    }

    
    
    
    
    
    
    
    
    public func getCutableObj(pObj)
    {
	if (pObj == NULL || 
	    (pObj.hasMember(@isCat) && pObj.isCat(@IF_RecoveredObject)))
	    return(NULL);

	if (pObj.isA(xOiGroup) && xOiTraverseHierarchy(pObj, @HasRecovered))
	    return(NULL);

	if (!pObj.hasMember(@isCutable)) return(pObj);

	var tState = pObj.isCutable();

	if (tState == -1) return(NULL);

	if (tState == 1) return(pObj);

	var tF = pObj.getFather();

	while(tF != NULL && 
	      (!tF.hasMember(@isCat) || tF.isCat(@IF_RecoveredObject) ||
	       !tF.hasMember(@isCutable) || tF.isCutable() != 1))
	    tF = tF.getFather();

	return(tF);
    }

    
    
    
    
    
    
    
    public func getDeletableObj(pObj)
    {
	if (pObj == NULL || !pObj.hasMember(@isCutable)) return(pObj);

	var tTopEl = self.getTopPlElement(pObj);

	if (pObj.hasMember(@isCat) && pObj.isCat(@IF_RecoveredObject) &&
	    tTopEl != NULL && tTopEl.isA(xOiGroup)) {
	    
	    
	    
	    return(NULL);
	}
	    
	var tState = pObj.isCutable();

	if (tState == -1) return(NULL);

	if (tState >= 1) return(pObj);

	var tF = pObj.getFather();

	while(tF != NULL && (!tF.hasMember(@isCutable) || tF.isCutable() < 1))
	    tF = tF.getFather();

	return(tF);
    }

    
    
    
    
    
    
    
    public func getUngroupableObj(pObj)
    {
	if (pObj == NULL) return(NULL);

	if (pObj.isA(xOiAggregate))
	    return(isUngroupable(pObj) ? pObj : NULL);

	var tTopEl = self.getTopPlElement(pObj);

	return(tTopEl != NULL && isUngroupable(tTopEl) ? tTopEl : NULL);
    }

    
    
    
    
    
    
    
    
    public func moveObjects(pObjects, pVector)
    {
	xOiDebugStartFunc("xOiPlanning::moveObjects", [pObjects, pVector]);

	var tObjects = @();
	var i;
	for(i=0; i < pObjects.size(); i++) {
	    var tObj = pObjects[i];
	    if (tObj.getFather() == self)
		tObjects.pushBack(tObj);
	    else
		xOiDebugPrint(@Warn, 1, ["passed ", tObj,
					 " is not a direct child of planning"]);
	}

	if (tObjects.empty()) {
	   xOiDebugPrint(@Warn, 1, "no objects specified!");
	   xOiDebugFinishFunc(0);
           return(0);
	}

	var tDOF = 0;
	if (fabs(pVector[0]) > eps) tDOF = 1;
	if (fabs(pVector[1]) > eps) tDOF = tDOF | 2;
	if (fabs(pVector[2]) > eps) tDOF = tDOF | 4;

	var tInvalid = 0;
	var tNo = tObjects.size();
	for(i=0; i < tNo; i++) {
	    var tObj = tObjects[i];
	    if (!(tObj.getTrAxis() & tDOF)) {
		tInvalid = 1;
		break;
	    }
	}
	
	if (tInvalid) {
           
	   xOiDebugPrint(@ExplWarn, 1,
			"one or more objects cannot be translated accordingly");
	   xOiDebugFinishFunc(0);
           return(0);
	}

	for(i=0; i < tNo; i++) {
	    var tObj = tObjects[i];
	    _setFreeObjPosition(tObj, @WC, @relative, pVector, 0, 0);
	    
	}

	var tRet = 1;

	if (!_checkCollisions(tObjects, 0, 0, 1).empty() &&
	    !xOiAskYesNo("::ofml::xoi::@moveObjectsWithCollision", @QUESTION,
			 getLanguage())) {
	    tRet = 0;
	    var tVector = [-pVector[0], -pVector[1], -pVector[2]];
	    var tObj;
	    foreach(tObj; tObjects)
	        _setFreeObjPosition(tObj, @WC, @relative, tVector, 0, 0);
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func checkCollisions(pMode, ...)
    {
	var tMode = 0;
	var tCheckOtherHierarchy = 0;
	if (pMode.size() > 1)
	    tCheckOtherHierarchy = pMode[1];

	return(_checkCollisions(getElements(), tMode, 1, tCheckOtherHierarchy));
    }

    
    private func resolvePlGroups(pObjects)
    {
	xOiDebugStartFunc2("xOiPlanning::resolvePlGroups",  pObjects);

	var tObjects = @();
	var tObj;
	foreach(tObj; pObjects) {
	    var tObj2 = tObj;
	    if (tObj.isCat(@PlGroup))
		xOiCopyAggr(resolvePlGroups(tObj.getElements()), tObjects, 0);
	    else
		tObjects.pushBack(tObj2);
	}

	xOiDebugFinishFunc(tObjects);
	return(tObjects);
    }

    func resolveObjects4AutoDeco(pObjects, ...)
    {
	xOiDebugStartFunc2("xOiPlanning::resolveObjects4AutoDeco",  pObjects);

	var tObjects = pObjects[0];
	var tContext = NULL;
	if (pObjects.size() > 1)
	    tContext = pObjects[1];

	var tRetL = @();
	var tObj;
	foreach(tObj; tObjects) {
	    var tObj2 = tObj;
	    var tOL = @(); 
	    if ((tObj.isA(OiPlElement) || tObj.isA(OiPart)) &&
		(!tObj.isCat(@PlGroup) || tObj.isCat(@Composite) ||
		 tObj.hasMember(@getChildren4AutoDeco)))
		tRetL.pushBack(tObj2);
	    var tChildren;
	    if (tObj.hasMember(@getChildren4AutoDeco) &&
		(tChildren = tObj.getChildren4AutoDeco()) != NULL &&
		(tChildren instanceof Vector || tChildren instanceof List))
		
		xOiCopyAggr(tChildren, tOL, 0);
	    else
	    if (tObj.isCat(@Composite) || tObj.isCat(@PlGroup) || 
		tObj.isA(xOiGroup))
		xOiCopyAggr(resolveObjects4AutoDeco(tObj.getElements()),
			    tOL, 0);
	    if (pObjects.size() == 1) {
		xOiCopyAggr(tOL, tRetL, 0);
		continue;
	    }
	    if (tOL.empty()) continue;

	    
	    
	    
	    foreach(tObj; tOL) {
		var tObj2 = tObj;
		if (tObj.hasMember(@getMetaInfo) &&
		    tObj.getMetaInfo(@AutoDecoration, tContext) != NULL)
		    tRetL.pushBack(tObj2);
	    }
	}

	xOiDebugFinishFunc(tRetL);
	return(tRetL);
    }

    
    
    private func _checkCollisions(pObjects, pMode, pDownTraversal,
				  pCheckOtherHierarchy)
    {
	xOiDebugStartFunc("xOiPlanning::_checkCollisions",
			   [pObjects, pMode, pDownTraversal,
			    pCheckOtherHierarchy]);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	var tOtherHierarchy = NULL;
	if (pCheckOtherHierarchy) {
	    tOtherHierarchy = getEnvironment();
	    if (tOtherHierarchy == self)
		tOtherHierarchy = xOiBasePlanning.getPlanning();
	}
	xOiDebugPrint(@Info, 1, ["other hierarchy ", tOtherHierarchy]);

	var tObjects = @();
	xOiCopyAggr(pObjects, tObjects, 0);

	if (pDownTraversal) {
	    tObjects = resolvePlGroups(tObjects);
	    xOiDebugPrint(@Info, 2, ["resolved object list: ", tObjects]);
	}

	var tCObjects = @();
	var tObjects2 = @();
	xOiCopyAggr(tObjects, tObjects2, 0);

	var tObj;
        foreach(tObj; tObjects) {
	    var tObj1 = tObj;
	    var tObj2;
            foreach(tObj2; tObjects2) {
        	if (tObj1 == tObj2) continue;

        	if (oiCollision(tObj1, tObj2)) {
                    tCObjects.pushBack(tObj1);
                    tCObjects.pushBack(tObj2);
		}
            }
            tObjects2.remove(tObj1);

	    if (tOtherHierarchy != NULL && oiCollision(tObj1, tOtherHierarchy))
		tCObjects.pushBack(tObj1);
        }

        var tCFunc = func(pO1, pO2) { return(pO1.getName() == pO2.getName()); };
        tCObjects.unique(tCFunc);

	xOiDebugFinishFunc(tCObjects);
	return(tCObjects);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func autoDecoration(pObjects, pContext, pTemplate)
    {
	xOiDebugStartFunc("xOiPlanning::autoDecoration",
			  [pObjects, pContext, pTemplate]);

	var tSingleObj = NULL;
	if (pObjects != NULL && pObjects.size() ==1)
	    tSingleObj = pObjects.front();
	var tObjects = pObjects;
	if (tObjects == NULL)
	    tObjects = getElements();
	tObjects = resolveObjects4AutoDeco(tObjects, pContext);
	var tCount = tObjects.size();
	var tCheckSingle = (tCount == 1 && tObjects.front() == tSingleObj);

	xOiDebugPrint(@Info, 2, ["check 4 single object? ", tCheckSingle]);

	var tThreshold = 20;
	var tTxt = oiGetStringResource("::ofml::xoi::@auto_decorating",
				       self.getLanguage());
	if (tCount >= tThreshold) xOiInitProgressDialog(tTxt, tCount, 1);

	var tObj;
	foreach(tObj; tObjects) {
	    var tOk = xOiAutoDecoration4Obj(tObj, pContext, pTemplate);
	    if (tCheckSingle && !tOk) {
		xOiDebugPrint(@Warn, 1, "could not create autodeco!");
		oiOutput(@WARNING, "::ofml::xoi::@NoAutoDeco");
	    }
	    if (tCount >= tThreshold) xOiProcessProgressStep();
	}
	if (tCount >= tThreshold) xOiFinishProgressDialog();

	oiClearHistory();

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    public func deleteAutoDecoration(pObjects, ...)
    {
	var tObjects = pObjects[0];
	var tContext = NULL;
	if (pObjects.size() > 1)
	    tContext = pObjects[1];

	if (tObjects == NULL)
	    tObjects = getElements();
	tObjects = resolveObjects4AutoDeco(tObjects);

	var tObj;
	foreach(tObj; tObjects) xOiDeleteAutoDeco(tObj, tContext);

	oiClearHistory();
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setPlanningMode(pMode)
    {
	xOiDebugStartFunc("xOiPlanning::setPlanningMode", pMode);

	mPlanMode = pMode;

	xOiDebugFinishFunc(NULL);
    }

    
    
    public func getPlanningMode()
    {
	if (mXoiTmpPlanMode != NULL) return(mXoiTmpPlanMode);
	if (mPlanMode == NULL) return(0);

	return(mPlanMode);
    }

    
    
    
    
    public func setInsertMode(pMode)
    {
	mXoiInsertMode = pMode;
    }

    
    
    public func getInsertMode()
    {
	return(mXoiInsertMode);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setAppMode(pModeType, pModeValue)
    {
	xOiDebugStartFunc("xOiPlanning::setAppMode", [pModeType, pModeValue]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	if (getName() != "t") {
	    var tPlan = xOiPlanning.getPlanning();
	    if (tPlan != NULL)
	        tPlan.setAppMode(pModeType, pModeValue);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tOk = 0;

	if (pModeType == @View) {
	    var tValidVals = @(@_2D, @_3D, @Basket);
	    if (tValidVals.find(pModeValue) >= 0 || pModeValue == NULL) {
		mXoiAppViewMode = pModeValue;
		tOk = 1;
	    }
	}
	else
	if (pModeType == @Conversion) {
	    if (pModeValue >= 0 && pModeValue <= 1) {
		mXoiConversionAppMode = pModeValue;
		tOk = 1;
	    }
	}
	else
	if (pModeType == @Export) {
	    var tValidVals = @(@None, @XCR, @KPSA_XML);
	    if (tValidVals.find(pModeValue) >= 0) {
		sXoiExportAppMode = pModeValue;
		tOk = 1;
	    }
	}
	else
	if (pModeType == @CheckWallCrossing) {
	    if (pModeValue >= 0 && pModeValue <= 3) {
		mXoiCheckWallCrossing = pModeValue;
		tOk = 1;
	    }
	}
	else
	if (pModeType == @NeedConfiguration) {
	    if (pModeValue >= 0 && pModeValue <= 3) {
		mXoiNeedElemConfig = pModeValue;
		tOk = 1;
	    }
	}
	else
	if (pModeType == @Transformation) {
	    var tValidVals = @(@None, @Constrained, @Unconstrained);
	    if (tValidVals.find(pModeValue) >= 0) {
		mXoiTransformationAppMode = pModeValue;
		tOk = 1;
	    }
	}
	else
	if (pModeType == @EditOp) {
	    var tValidVals = @(@None, @Undo, @Redo);
	    if (tValidVals.find(pModeValue) >= 0) {
		mXoiEditOpAppMode = pModeValue;
		tOk = 1;
	    }
	}
	else
	if (pModeType == @MultiFMLProcessing) {
	    if (pModeValue >= 0 && pModeValue <= 2) {
		sXoiMultiFMLProcAppMode = pModeValue;
		tOk = 1;
	    }
	}
	else
	if (pModeType == @CatInsertion) {
	    var tValidVals = @(@Select, @Input);
	    if (tValidVals.find(pModeValue) >= 0) {
		sXoiCatInsertionAppMode = pModeValue;
		tOk = 1;
	    }
	}
	else
	if (pModeType == @GlobalCollCheck) {
	    if (pModeValue instanceof Int &&
		(pModeValue == 0 || pModeValue == 1)) {
		sXoiGlobalCollCheckAppMode = pModeValue;
		tOk = 1;
	    } 
	}
	else
	if (pModeType == @ChangingPriceDate) {
	    if (pModeValue instanceof Int &&
		(pModeValue == 0 || pModeValue == 1)) {
		sXoiChangingPriceDate = pModeValue;
		tOk = 1;
	    } 
	}

	if (tOk)
	    getChangeManager().changeIssued(self, @AppModeChanged, 
					    [pModeType, pModeValue]);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    public func getAppMode(pModeType)
    {
	

	if (getName() != "t") {
	    var tPlan = xOiPlanning.getPlanning();
	    if (tPlan != NULL)
	        return(tPlan.getAppMode(pModeType));
	}

	if (pModeType == @View) return(mXoiAppViewMode);
	if (pModeType == @Conversion) return(mXoiConversionAppMode);
	if (pModeType == @Export) return(sXoiExportAppMode);
	if (pModeType == @CheckWallCrossing) return(mXoiCheckWallCrossing);
	if (pModeType == @NeedConfiguration) return(mXoiNeedElemConfig);
	if (pModeType == @Transformation) return(mXoiTransformationAppMode);
	if (pModeType == @EditOp) return(mXoiEditOpAppMode);
	if (pModeType == @MultiFMLProcessing) return(sXoiMultiFMLProcAppMode);
	if (pModeType == @CatInsertion) return(sXoiCatInsertionAppMode);
	if (pModeType == @GlobalCollCheck) return(sXoiGlobalCollCheckAppMode);
	if (pModeType == @ChangingPriceDate) return(sXoiChangingPriceDate);

	return(NULL);
    }

    
    
    

    
    
    
    
    
    
    
    public func newMetaElement(pObj)
    {
	
    }

    
    public func newElement(pFather, pObj)
    {
	xOiDebugStartFunc("xOiPlanning::newElement", [pFather, pObj]);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if (pObj == NULL || !pObj.hasMember(@getRoot)) {
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	

	var tRoot = pObj.getRoot();

	
	
	if (tRoot == self) {
	    var tCat, tHideCats = @();
	    var tHiddenCats  = getHiddenCategories();
	    var tHiddenProgs = getHiddenPrograms();
	    xOiDebugPrint(@Info, 2, ["hidden categories: ", tHiddenCats]);
	    xOiDebugPrint(@Info, 2, ["hidden programs  : ", tHiddenProgs]);
	    foreach(tCat; tHiddenCats) {
		var tC = tCat; 
		if (pObj.hasMember(@isCat) && pObj.isCat(tCat))
		    tHideCats.pushBack(tCat);
	    }
	    xOiDebugPrint(@Info, 2, ["hide categories: ", tHideCats]);
	    var tPID = NULL;
	    if (pObj.hasMember(@getProgram))
		tPID = pObj.getProgram();
	    xOiDebugPrint(@Info, 2, ["hide program   : ", tPID]);
	    if (tPID != NULL && !pObj.isHidden() &&
	    	(!tHideCats.empty() || tHiddenProgs.find(tPID) >= 0)) {
		pObj.hide();
		mHiddenElems.pushBack(pObj.getName());
	    	if (tRoot.getName() != "e" || !tRoot.isAtEval())
	            oiOutput(@WARNING, "::ofml::xoi::@hidingElement");
	    }
	}

	
	

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    public func getOriginalPasteObj(pObj)
    {
	xOiDebugStartFunc("xOiPlanning::getOriginalPasteObj", pObj);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func recreateBasket()
    {
	xOiDebugStartFunc("xOiPlanning::recreateBasket", NULL);

	var tTopItem = self.orderitems;

	tTopItem.removeAllItems();
	sXoiOrderMap = Hash();

	var tE;
	foreach(tE; self.getElements()) {
	    var tOID;
	    if (tE.isCat(@IF_Article) && (tOID = tE.getOrderID()) != NULL) {
		var tEl = tE;
		var tItem = tTopItem.addItem(@Article, tOID, NULL);
		sXoiOrderMap[tOID] = [tEl, tItem];
		xOiDebugPrint(@Info, 1, 
			      ["announcing item of type @Article with ID ",
                                tOID, " at father with ID NULL"]);
		xOiAppBasketAddItem(@Article, tOID, NULL);
	    }
	}
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func recreateBasket2(pObjects, pDeep)
    {
	xOiDebugStartFunc("xOiPlanning::recreateBasket2", [pObjects, pDeep]);
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    public func removeElement(pFather, pObj)
    {
	xOiDebugStartFunc2("xOiPlanning::removeElement", [pFather, pObj]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	if (getName() != "t") {
	    var tPlan = xOiPlanning.getPlanning();
	    if (tPlan != NULL) tPlan.removeElement(pFather, pObj);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	
	var tRoot = pObj.getRoot();
	xOiDebugPrint(@Info, 2, ["root ", tRoot]);

	if (tRoot != self &&
	    (tRoot.getName() != "e" || !tRoot.isA(OiPlanning))) {
	    xOiDebugPrint(@Info, 1, "request from foreign hierarchy");
	    var tTopEl = NULL;
	    if (tRoot.isA(OiPlanning))
		tTopEl = tRoot.getTopPlElement(pObj);
	    xOiDebugPrint(@Info, 2, ["top father planning element ", tTopEl]);
	    if (tTopEl != NULL && tTopEl.isA(xOiPlGroup)) {
		
		
		xOiDebugPrint(@Info, 2, "issuing @ElementRemoval");
		getChangeManager().changeIssued(pObj, @ElementRemoval, NULL);
	    }
	    xOiBasePlanning::removeElement(pFather, pObj);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	
	
	
	
	if (mAtEval) {
	    xOiDebugPrint(@Info, 1, "at evaluation");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	xOiDebugPrint(@Info, 2, ["at group action: ", mXoiAtGrouping]);
	if (!mXoiAtGrouping) {
	    xOiDebugPrint(@Info, 1, "issuing @ElementRemoval");
	    getChangeManager().changeIssued(pObj, @ElementRemoval, NULL);
	}

	xOiBasePlanning::removeElement(pFather, pObj);

	getMarkedGroupObjects().remove(pObj);

	mCCheck = 1;

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    func invalidateArticles(pObjects)
    {
	xOiDebugStartFunc("xOiPlanning::invalidateArticles", pObjects);
	xOiDebugFinishFunc(NULL);
    }

    
    
    public func getAllOrderIDs()
    {
	xOiDebugStartFunc2("xOiPlanning::getAllOrderIDs", NULL);
	xOiDebugFinishFunc("Method no longer supported!");
	return(@());
    }

    
    
    
    
    
    
    public func orderID2Obj(pID)
    {
	xOiDebugStartFunc("xOiPlanning::orderID2Obj", pID);

	xOiDebugPrint(@Info, 3, ["order map: ", sXoiOrderMap]);

	var tObj = (sXoiOrderMap.hasKey(pID) ? sXoiOrderMap[pID][0] : NULL);

	xOiDebugFinishFunc(tObj);
	return(tObj);
    }

    
    
    
    public func orderID2Item(pID)
    {
	xOiDebugStartFunc("xOiPlanning::orderID2Item", pID);

	var tItem = (sXoiOrderMap.hasKey(pID) ? sXoiOrderMap[pID][1] : NULL);

	xOiDebugFinishFunc(tItem);
	return(tItem);
    }

    
    
    public func getPlElement(pOrderID)
    {
	return(NULL);
    }

    
    
    
    public func orderID2ArticleSpec(pID, pMode)
    {
	xOiDebugStartFunc("xOiPlanning::orderID2ArticleSpec", [pID, pMode]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    func getNArticleSpecs(pObjects)
    {
	xOiDebugStartFunc("xOiPlanning::getNArticleSpecs", pObjects);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	var tRet = Vector(0);
	xOiDebugFinishFunc(tRet);
	return(tRet);
    }
 
    
    
    func setNArticleSpecs(pSpecs)
    {
	xOiDebugStartFunc("xOiPlanning::setNArticleSpecs", pSpecs);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    public func obj2OrderID(pObj)
    {
	xOiDebugStartFunc("xOiPlanning::obj2OrderID", pObj);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    public func item2OrderID(pItem)
    {
	xOiDebugStartFunc("xOiPlanning::item2OrderID", pItem);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    func atObjectPaste()
    {
	return(sObjectPaste);
    }

    private func startObjectPaste(pIsGroup)
    {
	xOiDebugStartFunc2("xOiPlanning::startObjectPaste", pIsGroup);

	sObjectPaste = 1;

	if (pIsGroup)
	    startMultiInsert(NULL);

	xOiDebugFinishFunc(NULL);
    }

    private func finishObjectPaste(pNewObj)
    {
	xOiDebugStartFunc2("xOiPlanning::finishObjectPaste", pNewObj);

	sObjectPaste = 0;

	if (sMultiInsert)
	    finishMultiInsert(NULL);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    public func getPriceDate4Obj(pObj)
    {
	xOiDebugStartFunc2("xOiPlanning::getPriceDate4Obj", pObj); 

	var tDate = self.getPriceDate(NULL);

	if (tDate == NULL) {
	    tDate = xOiBasePlanning::getPriceDate4Obj(pObj); 
	    xOiDebugPrint(@Info, 1, "using current date for prices");
	}

	xOiDebugFinishFunc(tDate);
	return(tDate);
    }

    
    
    
    private func updatePDCache(pObj, pInit)
    {
	xOiDebugStartFunc2("xOiPlanning::updatePDCache", [pObj, pInit]); 

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if (getName() != "t" ||
	    !(getAppMode(@NeedConfiguration) & 2)) {
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tPDM = getPDManager();
	if (tPDM != NULL && tPDM.isA(xOiPDManager)) {
	    var tObj = pObj;
	    if (pInit)
		
		tObj = pObj.getArticleObj();
	    tPDM.updatePDCache(tObj);
	}
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    public func hasArticlesNotUp2Date(pArticles)
    {
	xOiDebugStartFunc("xOiPlanning::hasArticlesNotUp2Date", pArticles);
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(0);
        return(0);
    }

    
    func getArticleMapEntries(pObj, pInclChildren, ...)
    {
	xOiDebugStartFunc("xOiPlanning::getArticleMapEntries", 
			  [pObj, pInclChildren]);
	var tInclChildren = pInclChildren[0];
	var tNeedHash = 0;
	if (pInclChildren.size() > 1)
	    tNeedHash = pInclChildren[1];
	var tRet = Vector(0);
	if (tNeedHash)
	    tRet = Hash();
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(tRet);
        return(tRet);
    }

    
    func restoreArticleMapPD(pEntries)
    {
	xOiDebugStartFunc("xOiPlanning::restoreArticleMapPD", NULL);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    public func invalidArticleUserMessage(pObj, pMsg)
    {
	xOiDebugStartFunc("xOiPlanning::invalidArticleUserMessage", NULL);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func issueUpdateMessage(pObj, pState, pMsg)
    {
	var tMsg = pMsg;

	if (tMsg == NULL) {
	    if (pState == @InvalidArticle)
		tMsg = "::ofml::xoi::@cannotUpdateArticle";
	    else
	    if (pState == @NoPriceDate) {
		if (self.getAppMode(@ChangingPriceDate))
		    xOiDebugPrint(@Info, 1, 
				  "app mode @ChangingPriceDate is on");
		else
		    tMsg = "::ofml::xoi::@assigningPriceDate4Old";
	    }
	}

	if (tMsg != NULL) oiOutput(@MESSAGE, tMsg);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func updateArticles()
    {
	_updateArticles(NULL, NULL);
    }

    
    
    
    
    
    
    
    
    public func updateArticles2(pArticles)
    {
	
	
	

	return(_updateArticles(NULL, pArticles));
    }

    
    
    
    private func _updateArticles(pPrograms, pArticles)
    {
	xOiDebugStartFunc("xOiPlanning::_updateArticles", 
			  [pPrograms, pArticles]);
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    public func updatePrices(pArticles)
    {
	xOiDebugStartFunc("xOiPlanning::updatePrices", pArticles);
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func updateSingleArticle(pObj)
    {
	xOiDebugStartFunc("xOiPlanning::updateSingleArticle", pObj);
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(1);
	return(1);
    }

    
    
    

    
    
    
    
    public func checkConsistency()
    {
	xOiDebugStartFunc("xOiPlanning::checkConsistency", NULL);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(1);
	return(1);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkObjConsistency(pObj, ...)
    {
	xOiDebugStartFunc("xOiPlanning::checkObjConsistency", pObj);

	var tObj = pObj[0];
	var tReportErr = 1;
	if (pObj.size() > 1)
	    tReportErr = pObj[1];
	var tMsg2User = 1;
	if (pObj.size() > 2)
	    tMsg2User = pObj[2];
	var tResolveResource = 1;
	if (pObj.size() > 3)
	    tResolveResource = pObj[3];

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if(tObj == NULL || !(tObj.isA(OiPlElement) || tObj.isA(OiPart))) {
	    xOiDebugPrint(@Info, 1, "object is not an article");
	    xOiDebugFinishFunc(1);
	    return(1);
	}

	var tPlan = oiGetPlanning();
	var tErrorLog = tPlan.getErrorLog(); 

	xOiDebugPrint(@Info, 2, ["current error log: ", tErrorLog]);

	var tTmpLog = Hash();
	if (tReportErr && tMsg2User)
	    tTmpLog = NULL; 
	tPlan.setErrorLog(tTmpLog);

	var tOk = tObj.checkConsistency();

	xOiDebugPrint(@Info, 2, ["temporary error log: ", tTmpLog]);

	tPlan.setErrorLog(tErrorLog);

	var tRes = tOk;

	var tLogKey = tObj.getOrderID();
	if (tLogKey == NULL)
	    tLogKey = tObj.getName();

	if (tOk) {
	    if (tErrorLog != NULL && tErrorLog.hasKey(tLogKey))
		tErrorLog.remove(tLogKey);
	    if (tReportErr && !tMsg2User) tRes = NULL;
	}
	else
	if (tReportErr && !tMsg2User) {
	    tRes = "";
	    if (tTmpLog.hasKey(tLogKey)) {
		var tErrors = tTmpLog[tLogKey];
	        var tLang = getLanguage();
		var tE;
		foreach(tE; tErrors) {
		    xOiDebugPrint(@Info, 2, ["error: ", tE]);
		    var tErrStr = NULL;
		    if (tE instanceof Vector && tE[0] instanceof String) 
			tErrStr = tE[0];
		    else
		    if (tE instanceof String) 
			tErrStr = tE;
		    if (tErrStr != NULL)
			tRes += (tResolveResource ? 
				 oiGetStringResource(tErrStr, tLang) :
				 tErrStr) + "\n";
		}
	    }
	}

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    public func setErrorLog(pLog)
    {
	mErrorLog = pLog;
    }

    
    public func getErrorLog()
    {
	return(mErrorLog);
    }

    

    
    public func prepareBasket(pMode, pDepth, ...)
    {
	xOiDebugStartFunc("xOiPlanning::prepareBasket", [pMode, pDepth]);
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(1);
	return(1);
    }

    
    public func finishBasket()
    {
	xOiDebugStartFunc("xOiPlanning::finishBasket", NULL);
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    

    private func orderItemType(pObj)
    {
	var tType = @Article;
	if (pObj.isA(xOiGroup) || pObj.isCat(@ORDER_GROUP) || 
	    (pObj.isCat(@IF_MetaType) && !pObj.isCat(@IF_CompositeArticle)))
	    tType = @Group;

	return(tType);
    }

    
    private func orderGroupLabel(pObj, pLang)
    {
	xOiDebugStartFunc2("xOiPlanning::orderGroupLabel", [pObj, pLang]);

	var tLabel = NULL;

	if (pObj.isA(xOiGroup))
	    tLabel = pObj.getLabel();
	else
	if (pLang != NULL && pObj.hasMember(@getOrderGroupLabel2))
	    tLabel = pObj.getOrderGroupLabel2(pLang);
	else
	if (pObj.hasMember(@getOrderGroupLabel))
	    tLabel = pObj.getOrderGroupLabel();

	xOiDebugFinishFunc(tLabel);
	return(tLabel);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getSingleOrderData(pObj, pLanguage, ...)
    {
	var tLanguage = pLanguage[0];
	var tFType = @Standard;
	if (pLanguage.size() > 1)
	    tFType = pLanguage[1];

	xOiDebugStartFunc("xOiPlanning::getSingleOrderData",
			  [pObj, tLanguage, tFType]);
	var tObj = pObj;
	if (pObj instanceof Symbol) {
	    tObj = self.orderID2Obj(pObj);;
	    xOiDebugPrint(@Info, 1, ["affected object is ", tObj]);
	}

	if (tObj == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRes = NULL;
	if (tLanguage == NULL || !(tLanguage instanceof String))
	    tLanguage = "de";

	var tOrderItem = xOiOrderItem4El(self, @oi, tObj);
	tOrderItem.initArticleCache();
	tRes = tOrderItem.getCompleteData2(tLanguage, tFType, 1);
	remove(tOrderItem);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticleConfiguration(pArticle, pRecursive)
    {
	xOiDebugStartFunc("xOiPlanning::getArticleConfiguration",
			  [pArticle, pRecursive]);

	if (pArticle == NULL || 
	    !(pArticle.isA(OiPlElement) || pArticle.isA(OiPart))) {
	    xOiDebugPrint(@Warn, 1, "no article instance given!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tResult = Vector(6);

	var tSubArtData = Vector(4);
	    
	tSubArtData[1] = pArticle;
	tSubArtData[2] = tResult;

	var tPending = List();
	tPending.pushBack(tSubArtData);

	var tArtCount = 0;

	do {
	    xOiDebugPrint(@Info, 1, [" article #", ++tArtCount, " ... "]);

	    tSubArtData = tPending.popFront();
	    var tArticle = tSubArtData[1];

	    var tArtCfg = tSubArtData[2];
	    tArtCfg[0] = tArticle.getProgram();
	    tArtCfg[1] = tArticle.getArticleSpec();
	    tArtCfg[2] = tArticle.getXArticleSpec(@OFMLVarCode);
	    tArtCfg[3] = tArticle.getAddStateCode(NULL);
	    tArtCfg[5] = tArticle.getPriceDate();

	    var tCompositeType, tMainChild;
	    if (tArticle.isCat(@IF_MetaType)) {
		tCompositeType = @IF_MetaType;
		tMainChild = tArticle.getMainChild();
	    } 
	    else 
	    if (tArticle.isCat(@IF_CompositeArticle)) {
		tCompositeType = @IF_CompositeArticle;
	    } 
	    else {
		xOiDebugPrint(@Info, 1, 
			      [tArticle, ": configuration = ", 
			       [tArtCfg[0], tArtCfg[1], tArtCfg[2], 
			        "AddStateCodes", tArtCfg[4]]]);
		xOiDebugPrint(@Info, 1, 
			      ["... pending articles: ", tPending.size()]);
		continue;
	    }

	    var tCompositeData = Vector(tCompositeType == @IF_MetaType ? 3 : 2);
	    tArtCfg[4] = tCompositeData;

	    tCompositeData[0] = tCompositeType;

	    xOiDebugPrint(@Info, 1, 
			  [tArticle, ": configuration = ", 
			   [tArtCfg[0], tArtCfg[1], tArtCfg[2], 
			    "AddStateCodes", tArtCfg[4]]]);

	    if (pRecursive) {
		var tSubArtList = [];
		tCompositeData[1] = tSubArtList;

		if (tMainChild != NULL) {
		    tSubArtData = Vector(4);
		    tSubArtData[0] = "";
		    tSubArtData[1] = tMainChild;
		    tSubArtData[2] = Vector(6);
		    tSubArtData[3] = tArticle.isOrderPos();

		    xOiDebugPrint(@Info, 1, 
			  	  [tArticle, ": main child = ", 
				   ["", tMainChild, tSubArtData[3]]]); 

		    tSubArtList.pushBack(tSubArtData);
		    tPending.pushBack(tSubArtData);
		}

		var tSubArtId;
		foreach (tSubArtId; tArticle.getSubArticleIDs()) {
		    var tSubArt = tArticle.getSubArticle(tSubArtId);

		    tSubArtData = Vector(4);
		    tSubArtData[0] = tSubArtId;
		    tSubArtData[1] = tSubArt;
		    tSubArtData[2] = Vector(6);
		    tSubArtData[3] = tCompositeType == @IF_MetaType ?
				     tArticle.isMetaSubPos(tSubArt) :
				     1;

		    xOiDebugPrint(@Info, 1, 
			  	  [tArticle, ": sub article = ", 
				   [tSubArtId, tSubArt, tSubArtData[3]]]);

		    tSubArtList.pushBack(tSubArtData);
		    tPending.pushBack(tSubArtData);
		}
	    }

	    if (tCompositeType == @IF_MetaType) {
		if (tArticle.hasMember(@getMTID))
		    tCompositeData[2] = tArticle.getMTID();
		else
		    xOiDebugPrint(@Warn, 1, 
				 [tArticle, " doesn't implement getMTID()!?"]);
	    }

	    xOiDebugPrint(@Info, 1, 
			  ["... pending articles: ", tPending.size()]);
	} while (!tPending.empty());

	xOiDebugFinishFunc(NULL);
	return(tResult);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getExtraArticleData(pArticle)
    {
	xOiDebugStartFunc("xOiPlanning::getExtraArticleData", pArticle);

	if (pArticle == NULL || 
	    !(pArticle.isA(OiPlElement) || pArticle.isA(OiPart))) {
	    xOiDebugPrint(@Warn, 1, "no article instance given!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tResult = Vector(6);

	tResult[0] = orderItemType(pArticle);
	tResult[1] = _getExtraArticleCodes(pArticle);
	tResult[2] = _getExtraArticleInfo(pArticle);
	tResult[3] = _getArticlePackagingInfo(pArticle);
	tResult[4] = pArticle.getArticleClassifications([]);
	tResult[5] = pArticle.getGLDFInfo();

	xOiDebugFinishFunc(tResult);
	return(tResult);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticleDisplayData(pArticle, pLang)
    {
	xOiDebugStartFunc("xOiPlanning::getArticleDisplayData", 
			  [pArticle, pLang]);

	if (pArticle == NULL || 
	    !(pArticle.isA(OiPlElement) || pArticle.isA(OiPart))) {
	    xOiDebugPrint(@Warn, 1, "no article instance given!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}
	var tLabel = NULL;
	if (orderItemType(pArticle) == @Group)
	    tLabel = orderGroupLabel(pArticle, pLang);

	var tArtDescr, tFeatures, tAllFeatures, tFeaturesDescr;

	var tArtSpec = pArticle.getArticleSpec();
	if (tArtSpec == NULL || !tArtSpec.empty()) {
	    tArtDescr = _getArticleDescription(pArticle, pLang);
	    tFeatures = pArticle.getArticleFeatures(pLang);
	    tAllFeatures = pArticle.getAllArticleFeatures(pLang);
	    tFeaturesDescr = pArticle.getArticleFeaturesDescr(@ID_Text, pLang);
	}

	var tRet = [tLabel, tArtDescr, tFeatures, tAllFeatures, tFeaturesDescr];

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticlePriceInfo(pArticle, pLanguages)
    {
	xOiDebugStartFunc("xOiPlanning::getArticlePriceInfo",
			  [pArticle, pLanguages]);

	var tArtNo;

	if (pArticle == NULL || 
	    !(pArticle.isA(OiPlElement) || pArticle.isA(OiPart)) ||
	    (tArtNo = pArticle.getArticleSpec()) == NULL) {
	    xOiDebugPrint(@Warn, 1, "no article instance given!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}
	if (!(pLanguages instanceof String || 
	      pLanguages instanceof List ||
	      pLanguages instanceof Vector)) {
	    xOiDebugPrint(@Warn, 1, "invalid language(s)!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tAppCurr = self.getPrimaryCurrency();

	var tNeedPriceItems = _getProgOrderInfo(pArticle, @NeedsAddPrices);
	if (tNeedPriceItems == NULL)
	    tNeedPriceItems = 1; 

	if (pLanguages instanceof String || !tNeedPriceItems) {
	    var tLanguage = pLanguages;
	    if (!(pLanguages instanceof String))
		tLanguage =  pLanguages.front();
	    var tPrice = pArticle.getArticlePrice(tLanguage, tAppCurr);
	    if (tPrice == NULL || tPrice.empty()) {
		xOiDebugPrint(@Info, 1, "article has no price");
		xOiDebugFinishFunc(tPrice);
		return(tPrice);
	    }
	    var tCurrency = Vector(0);
	    xOiCopyAggr(tPrice.front(), tCurrency, 1);
	    var tEndPrice = Vector(0);
	    xOiCopyAggr(tPrice.back(), tEndPrice, 1);
	    tEndPrice[0] = oiGetStringResource("::ofml::xoi::@endprice", 
					       tLanguage);
	    if (!(pLanguages instanceof String)) {
		var tCurrencyTexts = @([NULL, "::ofml::xoi::@currency"]);
		var tEndPriceTexts = @();
		var tLang;
		foreach(tLang; pLanguages) {
		    var tLanguage = tLang; 
		    var tTxt = oiGetStringResource("::ofml::xoi::@endprice",
						   tLanguage);
		    tEndPriceTexts.pushBack([tLanguage, tTxt]);
		}
		tCurrency[0] = tCurrencyTexts;
		tEndPrice[0] = tEndPriceTexts;
	    }
	    tPrice = @(tCurrency, tEndPrice);
	    xOiDebugFinishFunc(tPrice);
	    return(tPrice);
	}

	
	

	var tPDM = getPDManager();
	var tTransaction = 0;
	if (tPDM != NULL && tPDM.isA(xOiPDManager)) {
	    tPDM.startTransaction(pArticle, tArtNo);
	    tTransaction = 1;
	}

	var t1stPrice = NULL;
	var tItemCnt;
	var tTexts = Vector(0);

	var tLang;
	foreach(tLang; pLanguages) {
	    if (!(tLang instanceof String)) {
		xOiDebugPrint(@Warn, 1, ["invalid language: ", tLang]);
		if (tTransaction) tPDM.finishTransaction(pArticle);
		xOiDebugFinishFunc(NULL);
		return(NULL);
	    }
	    var tLanguage = tLang; 
	    var tPrice = pArticle.getArticlePrice(tLanguage, tAppCurr);
	    xOiDebugPrint(@Info, 2, [tLanguage, ": ", tPrice]);
	    
	    if (tPrice == NULL || tPrice.empty()) {
		xOiDebugPrint(@Info, 1, "article has no price");
		if (tTransaction) tPDM.finishTransaction(pArticle);
		xOiDebugFinishFunc(tPrice);
		return(tPrice);
	    }
	    if (t1stPrice == NULL) {
		t1stPrice = tPrice;
		tItemCnt = tPrice.size();
		var tItem, i = 0;
		foreach(tItem; tPrice) {
		    if (i++) {
			var tTxt = tItem[0];
			if (i == tItemCnt)
			    tTxt = oiGetStringResource("::ofml::xoi::@endprice",
						       tLanguage);
			tTexts.pushBack(@([tLanguage, tTxt]));
		    }
		    else
			tTexts.pushBack(@([NULL, tItem[0]]));
		}
	    }
	    else {
		if (tPrice.size() != t1stPrice.size()) {
		    xOiDebugPrint(@Warn, 1, 
				  ["price item mismatch: ", t1stPrice, " vs. ",
				   tPrice]);
		    if (tTransaction) tPDM.finishTransaction(pArticle);
		    xOiDebugFinishFunc(NULL);
		    return(NULL);
		}
		var tItem, i = 0;
		foreach(tItem; tPrice) {
		    if (i) {
			var tTxt = tItem[0];
			if (i == tItemCnt-1)
			    tTxt = oiGetStringResource("::ofml::xoi::@endprice",
						       tLanguage);
			tTexts[i].pushBack([tLanguage, tTxt]);
		    }
		    i++;
		}
	    }
	}

	var tResult = @();
	var tItem, i = 0;
	foreach(tItem; t1stPrice) {
	    var tItem2 = Vector(0);
	    xOiCopyAggr(tItem, tItem2, 1);
	    tItem2[0] = tTexts[i++];
	    tResult.pushBack(tItem2);
	}

	if (tTransaction) tPDM.finishTransaction(pArticle);

	xOiDebugFinishFunc(tResult);
	return(tResult);
    }

    private func _getExtraArticleCodes(pArticle)
    {
	var tUseFinal = 0;
	var tFinalSpec = pArticle.getXArticleSpec(@Final);
	if (tFinalSpec == NULL) {
	    tFinalSpec = "";
	} 
	else 
	if (!tFinalSpec.empty()) {
	    tUseFinal = _getUseFinal(pArticle);
	}

	var tResult = Vector(4);
	tResult[0] = tFinalSpec;
	tResult[1] = tUseFinal;
	tResult[2] = pArticle.getXArticleSpec(@VarCode);
	tResult[3] = pArticle.getPropVarCode(0);

	return(tResult);
    }

    private func _getUseFinal(pArticle)
    {
	var tSpecMode = _getProgOrderInfo(pArticle, @ArtSpecMode);

	return(tSpecMode != NULL ? tSpecMode[0] == 1 : 0);
    }

    private func _getExtraArticleInfo(pArticle)
    {
	var tProgID = pArticle.getProgram();
	var tLang = tProgID != NULL ? getPDLanguage(tProgID) : NULL;

	var tManuID = "", tSeriesID = "";
	var tSource, tAccountGrp, tCtrlCode, tProductClass;
	if (tProgID != NULL) {
	    var tPDInfo = pArticle.getPDInfo(tLang);
	    if (tPDInfo != NULL) {
		tManuID = tPDInfo[0];
		tSeriesID = tPDInfo[1];
		var tSize = tPDInfo.size();
		if (tSize > 2)
		    tSource = tPDInfo[2];
		if (tSize > 3)
		    tAccountGrp = tPDInfo[3];
		if (tSize > 4)
		    tCtrlCode = tPDInfo[4];
		if (tSize > 5)
		    tProductClass = tPDInfo[5];
	    }
	}

	var tResult = Vector(13);
	tResult[0]  = xOiPackage2ProgID(xOiGetPackage(pArticle));
	tResult[1]  = tManuID;
	tResult[2]  = tSeriesID;
	tResult[3]  = tSource;
	tResult[4]  = tAccountGrp;
	tResult[5]  = tCtrlCode;
	tResult[6]  = tProductClass;
	tResult[7]  = pArticle.getOrderUnit(NULL);
	tResult[8]  = pArticle.isCat(@SALESONLY_ARTICLE);
	tResult[9]  = pArticle.getArticleAttribute(@Discountable);
	tResult[10] = pArticle.isCat(@PseudoArticle);
	tResult[11] = pArticle.isCat(@NonOrderArticle);
	tResult[12] = pArticle.isCat(@NonOfferArticle);

	return(tResult);
    }

    private func _getArticlePackagingInfo(pArticle)
    {
	if (pArticle.hasMember(@getArticlePackagingInfo))
	    return(pArticle.getArticlePackagingInfo());

	var tResult = NULL;
	var tPDM = getPDManager();
	if (tPDM != NULL && tPDM.isA(xOiPDManager))
	    tResult = tPDM.getArticlePackagingInfo(pArticle);

	return(tResult);
    }

    private func _getProgOrderInfo(pArticle, pInfoType, ...)
    {
	var tInfoType = pInfoType[0];
	var tProgInfo = NULL;
	if (pInfoType.size() > 1)
	    tProgInfo = pInfoType[1];

	if (tProgInfo == NULL) {
	    var tProgID = pArticle.getProgram();
	    if (tProgID != NULL) 
		tProgInfo = self.getInfo(tProgID);
	}

	if (tProgInfo == NULL || !tProgInfo.hasMember(@getOrderInfo))
	    return NULL;

	return(tProgInfo.getOrderInfo(@Standard, 0, tInfoType, pArticle));
    }

    private func _getArticleDescription(pArticle, pLang)
    {
	var tProgID = pArticle.getProgram();
	var tProgInfo = NULL;
	if (tProgID != NULL)
	    tProgInfo = self.getInfo(tProgID);

	var tTextMode = 1;
	if (tProgInfo != NULL) {
	    tTextMode = _getProgOrderInfo(pArticle, @ArtTextMode, tProgInfo);
	    if (tTextMode == NULL) tTextMode = 1;
	}

	var tShortText = pArticle.getArticleText(pLang, @s);
	var tLongText = pArticle.getArticleText(pLang, @l);

	if (tShortText == NULL && tTextMode == 0 && tLongText != NULL)
	    tShortText = @( tLongText.front() );
	if (tLongText == NULL && tTextMode == 1 && tShortText != NULL)
	    tLongText = tShortText;

	var tAddPriceNote = 0;
	var tPriceOnRequest = 0;
	if (tProgInfo != NULL && tProgInfo.hasMember(@getPricesOnRequest))
	    tPriceOnRequest = tProgInfo.getPricesOnRequest();

	if (tPriceOnRequest) {
	    var tPrice = pArticle.getArticlePrice(pLang, 
						  self.getPrimaryCurrency());
	    var tSalesPrice = NULL;
	    if (tPrice != NULL) {
		var tEndPrice = tPrice.back();
		tSalesPrice = tEndPrice[1];
	    }
	    if (tPrice == NULL || tSalesPrice == NULL || tSalesPrice == 0.0)
		tAddPriceNote = 1;
	}

	if (tAddPriceNote) {
	    var tNote = oiGetStringResource("::ofml::xoi::@RequestPrice", 
					    pLang);
	    if (tShortText != NULL &&
		(tTextMode == 0 || (tTextMode == 2 && tLongText == NULL)))
		tShortText.pushBack(tNote);
	    else
	    if (tLongText != NULL && tTextMode > 0)
		tLongText.pushBack(tNote);
	}
	
	return([tShortText, tLongText, tTextMode]);
    }

    
    
    

    private func getBoundsSideOrder()
    {
        var tDirXZ = getPlanDirXZ();
        if (tDirXZ == @RF) return(@(@R, @F, @L, @B));
        if (tDirXZ == @RB) return(@(@R, @B, @L, @F));
        if (tDirXZ == @LF) return(@(@L, @F, @R, @B));
        if (tDirXZ == @LB) return(@(@L, @B, @R, @F));
        if (tDirXZ == @FR) return(@(@F, @R, @B, @L));
        if (tDirXZ == @FL) return(@(@F, @L, @B, @R));
        if (tDirXZ == @BR) return(@(@B, @R, @F, @L));
        if (tDirXZ == @BL) return(@(@B, @L, @F, @R));
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public static var checkAddBounds = func(pBndObj, pBnds, pCh, ...)
    {
	xOiDebugStartFunc("xOiPlanning::checkAddBounds",
			  [pBndObj, pBnds, pCh]);

	var tCh = pCh[0];
	var tImpact = 0;
	if (pCh.size() > 1)
	    tImpact = pCh[1];

        var tChBnds = tCh.getLocalBounds();
        var tXC = 0.0;
	if (tCh.hasMember(@getPDistance))
	    tXC = tCh.getPDistance();
        var tPos;
	var tY = 0.0;
	var tChYOffset = tCh.getLocalGeoBounds()[0][1];
	if (tChYOffset < 0.0) tY -= tChYOffset;

        var tScW = pBnds[1][0]-pBnds[0][0];
        var tScD = pBnds[1][2]-pBnds[0][2];

        
        
	var tPlan = xOiPlanning.getPlanning();
	xOiDebugPrint(@Info, 2, ["global planning ", tPlan]);
        var tSideL = @(@R, @F, @L, @B);
	if (tPlan.isA(xOiPlanning))
            tSideL = tPlan.getBoundsSideOrder();

	var tErr = "::ofml::xoi::@coll";
	if (tPlan.hasBorder())
	    tErr = "::ofml::xoi::@badpos";

        var tRet = [NULL, 0, tErr];

        

	var tSideHash = Hash(); 
	tSideHash[@L] = [pBnds[0][0]-tChBnds[1][0]-tXC, 0.0+0.0, 1];
	tSideHash[@R] = [pBnds[1][0]-tChBnds[0][0]+tXC, 0.0+0.0, 1];
	tSideHash[@F] = [0.0+0.0, pBnds[1][2]-tChBnds[0][2]+tXC, 0];
	tSideHash[@B] = [0.0+0.0, pBnds[0][2]-tChBnds[1][2]-tXC, 0];
	var tCollObj = NULL;
        var tS;
        foreach(tS; tSideL) {
	    xOiDebugPrint(@Info, 2, ["check side: ", tS]);
	    var tIterator = tSideHash[tS][2];
            while (tSideHash[tS][tIterator] < tScD+xOiEps && tRet[2] != NULL) {
                tPos = [tSideHash[tS][0], tY, tSideHash[tS][1]];
		xOiDebugPrint(@Info, 3, ["check pos: ", tPos]);
                tCh.setPosition(tPos);
                if ((pBndObj != tPlan && pBndObj.checkElPos(tCh, tPos)) ||
		    (pBndObj == tPlan &&
		     (tCollObj = tPlan.checkChildColl(tCh, NULL)) == NULL &&
		     tPlan.checkBorderObj(tCh) == NULL))
		    tRet = [tPos, 0, NULL];
		else
                    tSideHash[tS][tIterator] += 0.5;
            }
	    if (tRet[2] == NULL) break;
        }

	if (tRet[2] == NULL || pBndObj != tPlan || !tPlan.isA(xOiPlanning) ||
	    !tImpact) {
	    xOiDebugFinishFunc(tRet);
            return(tRet);
	}

	xOiDebugPrint(@Info, 2, ["collided ", tCollObj]);

	
	
	
	
	
	
	
	
	
	

	if (tScW < xOiEps && tScD < xOiEps) { 
	    
	    
	    
	    
	    var tPlWall = NULL;
	    if (tCollObj != NULL && tCollObj.isCat(@IF_Wall) &&
		xOiBBox.boxInsideBox(pBnds, tCollObj.getWorldBounds()))
		tPlWall = tCollObj;
	    else {
		var tMinD = NULL;
		var tDirs = @(@L, @R, @F, @B);
		var tDir;
		foreach(tDir; tDirs) {
		    var tDO = tPlan.getObjDistance(tCh, tDir, @LC,
						   @relative, 1, 0);
		    var tD, tNP = [0.0, 0.0, 0.0];
		    var tDObj = NULL;
		    if (tDO[1] != NULL)
			tDObj = tPlan.getTopPlElement(tDO[1]);
		    if (tDObj != NULL && tDObj.isCat(@IF_Wall) &&
			(tMinD == NULL ||
			 (tD = xOiGetDistance3(tDO[0], tNP)) < tMinD)) {
			tPlWall = tDObj;
			tMinD = tD;
		    }
		}
	    }
	    
	    
	    var tWalls = @();
	    var tEnv = tPlan.getEnvironment();
	    if (tPlWall != NULL) tWalls.pushBack(tPlWall);
	    if (tEnv != NULL) {
		var tE;
		foreach(tE; tEnv.getChildren())
		    if (tE.isCat(@IF_Wall) && tE != tPlWall)
			tWalls.pushBack(tE);
	    }
	    var tPR = tPlan.checkAddToWalls(tWalls, tCh);
	    if (tPR != NULL) {
		tRet = [tPR[0], tPR[1], NULL];
		xOiDebugFinishFunc(tRet);
        	return(tRet);
	    }
	}

	

	var tRaster = 0.5;
	var tMaxDim = 10.0;
	var tBnds = [[pBnds[0][0], pBnds[0][1], pBnds[0][2]],
		     [pBnds[1][0], pBnds[1][1], pBnds[1][2]]];
	do {
	    tBnds = [[tBnds[0][0]-tRaster, tBnds[0][1], tBnds[0][2]-tRaster],
		     [tBnds[1][0]+tRaster, tBnds[1][1], tBnds[1][2]+tRaster]];
	    tRet = xOiPlanning.checkAddBounds(pBndObj, tBnds, tCh, 0);
	} while (tRet[0] == NULL && tBnds[1][0]-tBnds[0][0] < tMaxDim);

	xOiDebugFinishFunc(tRet);
        return(tRet);
    };

    
    
    
    
    
    
    
    public func checkAddToWalls(pWalls, pCh)
    {
	xOiDebugStartFunc("xOiPlanning::checkAddToWalls", [pWalls, pCh]);

	var tRet = NULL;

	var tZ = 0.0;
	if (pCh.hasMember(@getWallOffset))
	    tZ = pCh.getWallOffset();

	var tXOff = 0.0;
	if (pCh.hasMember(@getPDistance))
	    tXOff = pCh.getPDistance();

	var tChPos = pCh.getPosition();

	

	var tW;
	foreach(tW; pWalls) {
	    xOiDebugPrint(@Info, 2, ["1.pass - check wall ", tW]);
	    var tWP = tW.getWallParams();
	    xOiDebugPrint(@Info, 2, ["wall params: ", tWP]);
	    var tWW   = tWP[0];
	    var tA    = tWP[1];
	    var tWPos = xOiTransformXZCoords(tW.getWorldPosition(), tA,
					     tWP[2][0], tWP[2][2]);
	    xOiSetYRotation(pCh, tA);
	    var tPos, tOff;
	    var tX = tXOff;
	    while (tX < tWW) {
		tOff = [tX*cos(tA)+tZ*sin(tA), 0, -tX*sin(tA)+tZ*cos(tA)];
		
		tPos = [tWPos[0]+tOff[0], tChPos[1], tWPos[2]+tOff[2]];

		pCh.setPosition(tPos);

		if (checkElPos(pCh, NULL)) {
		    tRet = [tPos, tA];
		    xOiDebugFinishFunc(tRet);
		    return(tRet);
		}

		tX += 0.5;
	    }
	}

	
	
	
	
	

	var tChW = pCh.getWidth();
	xOiDebugPrint(@Info, 2, ["child width: ", tChW]);
	foreach(tW; pWalls) {
	    xOiDebugPrint(@Info, 2, ["2.pass - check wall ", tW]);
	    var tWP = tW.getWallParams();
	    var tWW = tWP[0];
	    var tA  = tWP[1];
	    var tWPos = xOiTransformXZCoords(tW.getWorldPosition(), tA,
					     tWP[2][0], tWP[2][2]);
	    xOiSetYRotation(pCh, tA);
	    var tPos = tWPos;
	    pCh.setPosition(tPos);
	    if (checkElPos(pCh, NULL)) {
		tRet = [tPos, tA];
		xOiDebugFinishFunc(tRet);
		return(tRet);
	    }
	    
	    var tDO = getObjDistance(pCh, @L, @LC, @relative, 1, 0);
	    var tD = xOiGetDistance3(tDO[0], [0.0, 0.0, 0.0]);
	    if (tD < 0.0001 && tDO[1] != NULL) {
		xOiDebugPrint(@Info, 1,
			      ["no negative translation possible along ", tW]);
		continue;
	    }
	    var tDMax = 10.0;
	    if (tD > 0.0001 && tD < tDMax) tDMax = tD;
	    var tR = 0.1;
	    if (tD > 0.0001 && tD < 0.1)
		tR = tD;
	    var tOff, tMoved = 0.0;
	    xOiDebugPrint(@Info, 2,
			  ["maximum move distance: ", tDMax, ", raster: ", tR]);
	    var tX = 0.0;
	    if (tChW > tWW) {
		var tDiff = tWW - tChW;
		if (tDiff < tDMax) tX = tWW - tChW;
	    }
	    while ((tMoved += tR) < tDMax) {
	        xOiDebugPrint(@Info, 2, ["moved distance: ", tMoved]);
		tX -= tR;
		tOff = [tX*cos(tA)+tZ*sin(tA), 0, -tX*sin(tA)+tZ*cos(tA)];
		tPos = [tWPos[0]+tOff[0], tWPos[1], tWPos[2]+tOff[2]];

	        xOiDebugPrint(@Info, 3, ["check pos: ", tPos]);
		pCh.setPosition(tPos);

		if (checkElPos(pCh, NULL)) {
		    tRet = [tPos, tA];
		    xOiDebugFinishFunc(tRet);
		    return(tRet);
		}
	    }
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    public func checkAdd(pType, pObj, pPos, pParams)
    {
	xOiDebugStartFunc("xOiPlanning::checkAdd",
			  [pType, pObj, pPos, pParams]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	if (getName() != "t") {
	    var tRet = xOiBasePlanning::checkAdd(pType, pObj, pPos, pParams);
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	xOiDebugPrint(@Info, 2, ["planning mode: ", getPlanningMode()]);

	var tObj = pObj;

	if (tObj != NULL && !tObj.hasMember(@isCat)) {
	    xOiDebugPrint(@Warn, 1, ["ignoring illegal reference ", tObj]);
	    tObj = NULL;
	}

	if (tObj != NULL) {
	    if (tObj.isCat(@IF_Wall))
		xOiDebugPrint(@Info, 2, "ref object is a wall.");
	    if (tObj.isCat(@IF_Floor))
		xOiDebugPrint(@Info, 2, "ref object is a floor.");
	}

	if (getPlanningMode() == 5 &&
	    (tObj == NULL || tObj == self ||
	     (pPos != NULL && (tObj.isCat(@IF_Wall) || tObj.isCat(@IF_Floor)))))
	    mXoiTmpPlanMode = 4;
	else
	    mXoiTmpPlanMode = NULL;

	xOiDebugPrint(@Info, 2, ["temp planning mode: ", getPlanningMode()]);

	var tPasteMode = getPasteMode();
	xOiDebugPrint(@Info, 2, ["paste mode: ", tPasteMode]);

	if (tPasteMode == @PA) startObjectPaste(pType.subClassOf(xOiGroup));

	var tRet;

	if (pPos != NULL && pPos[0] instanceof Vector && pPos.size() > 2 &&
	    pPos[2] == 0)
	    
	    
	    tRet = self.doCheckAdd(pType, tObj, pPos, pParams);
	else
	    tRet = xOiBasePlanning::checkAdd(pType, tObj, pPos, pParams);

	if (tRet == NULL) mXoiTmpPlanMode = NULL;

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func doCheckAdd(pType, pObj, pPos, pParams)
    {
	xOiDebugStartFunc("xOiPlanning::doCheckAdd",
			  [pType, pObj, pPos, pParams]);

	xOiDebugPrint(@Info, 2, ["working planning mode: ", getPlanningMode()]);

	if (getPlanningMode() == 3) {
	    
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tMsg = "::ofml::xoi::@confirm_delete";

	if (getPlanningMode() == 2) {
	    
	    if (!getElements().empty()) {
		if (!sTestMode &&
		    !xOiAskYesNo(tMsg, @QUESTION, getLanguage())) {
		    xOiDebugFinishFunc(NULL);
		    return(NULL);
		}
		else {
	    	    var tEl;
	    	    foreach(tEl; getElements()) remove(tEl);
		}
	    }
	    if (pType.subClassOf(OiPlElement))
		setMethod("onCreate(0, NULL, NULL)");

	    xOiDebugFinishFunc([0, 0, 0]);
	    return([0, 0, 0]);
	}

	

        if (mCheckAddPos != NULL) {
	    xOiDebugPrint(@Info, 1, "using mCheckAddPos");
	    xOiDebugFinishFunc(mCheckAddPos);
	    return(mCheckAddPos);
	}

        var tScBnds = getWorldBounds();

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

	var tNonConfigModes = @(0, 1, 5, 6);
        if (tNonConfigModes.find(getPlanningMode()) >= 0 &&
	    !pType.subClassOf(OiPlElement)) {
	    oiOutput(@WARNING, "::ofml::xoi::@needscel");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tElCount = getElements().size();
	xOiDebugPrint(@Info, 2,
		      ["current number of planning elements: ", tElCount]);

	var tOnCreateParam = "NULL";
	if (pType.subClassOf(xOiGroup))
	    tOnCreateParam = xOiParam2Str(getPasteMode());

	if (pType.subClassOf(OiPlElement))
            setMethod("onCreate(0, NULL, " + tOnCreateParam + ")");

        var tEnv = getEnvironment();
	var tObjIsWall  = (pObj != NULL && pObj.isCat(@IF_Wall));
	var tObjIsFloor = (pObj != NULL && pObj.isCat(@IF_Floor));

	

	var tExplPos  = pPos;
	var tExplA    = 0.0;
	var tExplMode = 2; 
	if (pPos != NULL && pPos[0] instanceof Vector) {
	    if (pPos.size() > 0) tExplPos  = pPos[0];
	    if (pPos.size() > 1) tExplA    = pPos[1];
	    if (pPos.size() > 2) tExplMode = pPos[2];
	}

	xOiDebugPrint(@Info, 2, ["mode: ", tExplMode]);

	if (tExplPos != NULL &&
	    (pObj == NULL || (tExplMode & 1) || tObjIsWall || tObjIsFloor)) {

	    var tObj4OnCreate = ", NULL, ";
	    if (pObj != NULL &&
		((tExplMode & 1) || tObjIsWall || tObjIsFloor))
		tObj4OnCreate = ", " + pObj.getName() + ", ";

	    var tHasCh = 0;
	    if ((!sMultiInsert && tExplMode & 2) || tObjIsWall || tObjIsFloor) {
        	createChParams(pType, tChParams);
		tHasCh = 1;
	    }

	    if (tObjIsWall) {
		var tY = tExplPos[1];
		if (!tHasCh || !self.ch.isCat(@WALL_ELEM))
		    tY = 0.0;

		tExplPos = [tExplPos[0], tY, tExplPos[2]];
		tExplA   = pObj.getWallParams()[1];

		if (tHasCh)
		    tExplPos = 
		    xOiTransformXZCoords(tExplPos, tExplA, 0.0,
					 -self.ch.getLocalBounds()[0][2]);
	    }
	    else
	    if (tObjIsFloor) {
		var tY = pObj.getWorldPosition()[1] + pObj.getTopOffset();
		var tChYOffset = self.ch.getLocalGeoBounds()[0][1];
		if (tChYOffset < 0.0) tY -= tChYOffset;
		tExplPos = [tExplPos[0], tY, tExplPos[2]];

		if (tHasCh)
		    tExplPos = [tExplPos[0], -self.ch.getLocalBounds()[0][1],
				tExplPos[2]];
	    }

	    xOiDebugPrint(@Info, 2, ["multi insert ? ", sMultiInsert]);

	    if (!sMultiInsert && tExplMode & 2) { 
		xOiDebugPrint(@Info, 2, "checking collision ...");
	        self.ch.setPosition(tExplPos);
	        xOiSetYRotation(self.ch, tExplA);

		var tCollObj;

	        if ((tCollObj = checkChildColl(self.ch, NULL)) == NULL &&
		    (!hasBorder() || checkBorderObj(self.ch) == NULL)) {
		    if (self.ch.isA(OiPlElement))
        	        setMethod("onCreate(" + String(tExplA) + tObj4OnCreate +
				 	        tOnCreateParam + ")");
	            xOiDebugFinishFunc(tExplPos);
        	    return(acceptCh(tExplPos));
	        }
		xOiDebugPrint(@Info, 2, ["collision ", tCollObj]);

		if (hasBorder()) {
		    var tErr = "::ofml::xoi::@badpos";
	            xOiDebugFinishFunc(tErr);
		    return(removeCh(tErr));
		}

		var tMsg = "::ofml::xoi::@confirm_collpos";

	    	if (xOiAskYesNo(tMsg, @QUESTION, getLanguage()))
		    tExplPos = self.ch.getPosition();
		else {
	            xOiDebugFinishFunc(NULL);
		    return(removeCh(NULL));
		}
	    }

	    if (pType.subClassOf(OiPlElement))
        	setMethod("onCreate(" + String(tExplA) + tObj4OnCreate +
				    tOnCreateParam + ")");
	    xOiDebugFinishFunc(tExplPos);
            return(acceptCh(tExplPos));
	}

	xOiDebugPrint(@Info, 2, "creating temporary object ...");
	try {
        createChParams(pType, tChParams);
	}
	catch(&e: Error) {
	    xOiDebugPrint(@EvalErr, 1, e.where + ": " + e.what);
	}
	xOiDebugPrint(@Info, 2, "... temporary object created.");

	var tObj = pObj;
	if (xOiIsObjChildOf(tObj, tEnv) && !tObjIsWall)
	    
	    tObj = NULL;

	if (getPlanningMode() == 4 && tObj != NULL) {
	    tObj = getTopLevelObj(tObj);
	    if (tObj != NULL && 
		!tObj.isCat(@IF_ATTPT) && !tObj.isCat(@IF_AttachPts)) 
		tObj = NULL;
	}

	xOiDebugPrint(@Info, 2, ["reference ", tObj]);

        

        if (tObj == NULL || tObjIsWall ||
	    !(tObj.isA(OiPlElement) || tObj.isA(OiPart))) {
            var tWall = NULL;
            if (tObjIsWall)
                tWall = tObj;
            else {
                if (tEnv != NULL && (tEnv.isA(OiLevel)))
                    tWall = tEnv.getPlanningWall();
            }

            var tWP, tA;
            if (tWall != NULL) {
                tWP = checkAddToWall(tWall, self.ch);
	        xOiDebugPrint(@Info, 2, ["wall pos: ", tWP]);

                if (tWP[0] != NULL && self.ch.isA(OiPlElement))
                    
                    setMethod("onCreate("+String(tWP[1])+", "+
                              tWall.getName()+", " + tOnCreateParam + ")");
            }
            else {
		var tBorder = getBorder();
	        xOiDebugPrint(@Info, 2, ["border: ", tBorder]);
		if (tElCount == 0 && tBorder[0] != NULL) {
		    
		    var tPos = [tBorder[0][0]+tBorder[1][0]/2, 0.0,
				tBorder[0][2]+tBorder[1][2]/2];
                    self.ch.setPosition(tPos);
                    if (checkElPos(self.ch, tPos))
			tWP = [tPos, 0, NULL];
		    else {
			var tErr = "::ofml::xoi::@coll";
			if (hasBorder())
			    tErr = "::ofml::xoi::@badpos";
        		tWP = [NULL, 0, tErr];
		    }
		}
		else
                    tWP = xOiPlanning.checkAddBounds(self, tScBnds, self.ch, 1);
	        xOiDebugPrint(@Info, 2, ["scene pos: ", tWP]);
                if (tWP[0] != NULL && self.ch.isA(OiPlElement))
                    setMethod("onCreate("+String(tWP[1])+", NULL, "+
                              tOnCreateParam + ")");
	    }

            if (tWP[0] != NULL) {
	        xOiDebugFinishFunc(tWP[0]);
                return(acceptCh(tWP[0]));
	    }
            else {
	        xOiDebugFinishFunc(tWP[2]);
                return(removeCh(tWP[2]));
	    }
        }

        

        
        
	if (getPlanningMode() != 4 && getElements().find(tObj) < 0)
            tObj = getTopPlElement(tObj);

	xOiDebugPrint(@Info, 2, ["add to selected ", tObj]);

        if (tObj == NULL ||
	    (!tObj.isCat(@IF_ATTPT) && !tObj.isCat(@IF_AttachPts)) || 
	    (!self.ch.isCat(@IF_ATTPT) && !self.ch.isCat(@IF_AttachPts))) {
            
            
	    xOiDebugFinishFunc("::ofml::xoi::@invinsobj");
            return(removeCh("::ofml::xoi::@invinsobj"));
	}

        

	var tCollCheck = (self.ch.isA(xOiFreeArticle) ? 2 : 1);

        var tPR = oiGetPosRot4AttachPts(tObj, self.ch, tChAttPt, tCollCheck);
        if (tPR == NULL) {
	    xOiDebugFinishFunc("::ofml::xoi::@invins");
	    return(removeCh("::ofml::xoi::@invins"));
	}

	xOiDebugPrint(@Info, 2, ["Pos/Rot: ", tPR]);

        var tFPos = tPR[0];
        var tRR   = tPR[1];
        var tErr  = tPR[2];

	var tRetErr = NULL;

        if (tErr == NULL && tFPos == NULL)
	    tRetErr = "::ofml::xoi::@invins";
	else
        if (tErr != NULL) {
            if (tErr == "::ofml::xoi::@coll") {
                if (tFPos == NULL)
		    tRetErr = "::ofml::xoi::@nofreeattpt";
                else
                    
		    
		    
		    tRetErr = "::ofml::xoi::@invinsposcoll";
            }
            else
		tRetErr = tErr;
        }
	if (tRetErr != NULL && (1 || getPlanningMode() == 6)) {
	    
	    xOiDebugPrint(@Info, 2, "trying to place at scene border...");
            var tWP = xOiPlanning.checkAddBounds(self, tScBnds, self.ch, 1);
	    xOiDebugPrint(@Info, 2, ["scene pos: ", tWP]);
            if (tWP[0] != NULL) {
		tFPos = tWP[0];
		tRR = tWP[1];
		tObj = NULL;
		tRetErr = NULL;
		xOiShowAppHint("::ofml::xoi::@placedAtBorder", 1);
	    }
            else
		tRetErr = tWP[2];
	}
	if (tRetErr != NULL) {
	    xOiDebugFinishFunc(tRetErr);
            return(removeCh(tRetErr));
	}

	if (self.ch.isA(OiPlElement))
            setMethod("onCreate(" + String(tRR) + ", " +
		      (tObj != NULL ? tObj.getName() : "NULL") +", " +
		      tOnCreateParam + ")");

	xOiDebugFinishFunc(tFPos);
        return(acceptCh(tFPos));
    }

    
    
    
    
    
    
    
    
    
    
    
    public func elemTranslation(pObj, pOld)
    {
	xOiDebugStartFunc("xOiPlanning::elemTranslation", [pObj, pOld]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tHistoryOps = @(@Undo, @Redo);

	var tEditOpAppMode = getAppMode(@EditOp);

	xOiDebugPrint(@Info, 2, ["edit-op app mode: ", tEditOpAppMode]);

	if (tHistoryOps.find(tEditOpAppMode) >= 0) {
	    xOiDebugFinishFunc(NULL);
	    return;
	}

        if (!pObj.translateValid(pOld)) {
	    xOiDebugPrint(@Info, 1, "translation not valid for object");
            pObj.setPosition(pOld);
	    xOiDebugFinishFunc(NULL);
            return;
        }

        
        
        if (pObj.isA(OiPlElement) && pObj.translated(pOld)) {
            
	    xOiDebugPrint(@Info, 1, "translation handled by object");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	var tPos = pObj.getPosition();

	var tCollision = (checkChildColl(pObj, NULL) != NULL);

	xOiDebugPrint(@Info, 2, ["collision? ", tCollision]);

	var tMsg = "::ofml::xoi::@confirm_collpos";

	var tTransfAppMode = getAppMode(@Transformation);

	xOiDebugPrint(@Info, 2, ["transformation app mode: ", tTransfAppMode]);

	var tUnconstrMoving = (pObj.hasMember(@isCat) && 
			       pObj.isCat(@UNCONSTRAINED_MOVING));

	xOiDebugPrint(@Info, 2, ["unconstrained moving? ", tUnconstrMoving]);

	if (tCollision && tTransfAppMode == @Unconstrained && tUnconstrMoving) {
	    if (!xOiAskYesNo(tMsg, @QUESTION, getLanguage()))
                pObj.setPosition(pOld);
	    else
	        xOiDebugPrint(@Info, 1, 
			      "unconstrained translation with collision");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

        if (tCollision ||
	    (xOiCheckWallCrossing(pObj, tPos, pOld, 1) &&
	     hasBorder() && checkBorderObj(pObj) != NULL)) {
	    xOiDebugPrint(@Info, 2, "translating as far as possible ...");
	    var tNewPos = xOiTranslateAlongAxes(pObj, pOld);
	    xOiTranslateAsFarAsPossible(pObj, tNewPos, tPos);
        }

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    public func elemRotation(pObj, pOld)
    {
	xOiDebugStartFunc("xOiPlanning::elemRotation", [pObj, pOld]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tHistoryOps = @(@Undo, @Redo);

	var tEditOpAppMode = getAppMode(@EditOp);

	xOiDebugPrint(@Info, 2, ["edit-op app mode: ", tEditOpAppMode]);

	var tRtAxis = xOiRotationMode2Axis(pObj.getRtAxis());

	if (tRtAxis == NULL) {
	    
	    xOiDebugPrint(@Warn, 1, 
			  ["invalid rotation mode: ", pObj.getRtAxis()]);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

        if (!pObj.rotateValid(pOld)) {
	    xOiDebugPrint(@Info, 1, "rotation not valid for object");
            pObj.setPosition(pOld);
	    xOiSetRotation(pObj, tRtAxis, pOld);
	    xOiDebugFinishFunc(NULL);
            return;
        }

        
        if (pObj.isA(OiPlElement) && pObj.rotated(pOld)) {
	    xOiDebugPrint(@Info, 1, "rotation handled by object");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	var tCollision = (checkChildColl(pObj, NULL) != NULL);

	if (!tCollision) {
	    xOiDebugPrint(@Info, 1, "no collision");
	    xOiDebugFinishFunc(NULL);
            return;
	}

	var tMsg = "::ofml::xoi::@confirm_collpos";

	var tTransfAppMode = getAppMode(@Transformation);

	xOiDebugPrint(@Info, 2, ["transformation app mode: ", tTransfAppMode]);

	var tUnconstrMoving = (pObj.hasMember(@isCat) && 
			       pObj.isCat(@UNCONSTRAINED_MOVING));

	xOiDebugPrint(@Info, 2, ["unconstrained moving? ", tUnconstrMoving]);

	if (tTransfAppMode == @Unconstrained && tUnconstrMoving &&
	    xOiAskYesNo(tMsg, @QUESTION, getLanguage())) {
	    xOiDebugPrint(@Info, 1, "unconstrained rotation with collision");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	xOiSetRotation(pObj, tRtAxis, pOld);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func checkChildColl(pObj, pExclObj)
    {
	xOiDebugStartFunc("xOiPlanning::checkChildColl", [pObj, pExclObj]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tRet = NULL;

	if (getAppMode(@GlobalCollCheck))
	    tRet = xOiBasePlanning::checkChildColl(pObj, pExclObj);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    

    
    
    func commonPropsChanged()
    {
	xOiDebugStartFunc("xOiPlanning::commonPropsChanged", NULL);

	if (self.getAppMode(@View) != @Basket)
	    xOiDebugPrint(@Info, 1, "no basket mode");
	else
	    getDynamicProps()[@_BASKET_COMMON_PROPS_CHANGED] = 1;

	xOiDebugFinishFunc(NULL);
    }

    func needAllMarkedObjects(pState)
    {
	xOiDebugStartFunc("xOiPlanning::needAllMarkedObjects", NULL);

	if (self.getAppMode(@View) != @Basket)
	    xOiDebugPrint(@Info, 1, "no basket mode");
	else
	    if (pState)
	        getDynamicProps()[@_NEED_ALL_MARKED_OBJECTS] = 1;
	    else
	        getDynamicProps().remove(@_NEED_ALL_MARKED_OBJECTS);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    public func markForGroup(pObj, ...)
    {
	xOiDebugStartFunc2("xOiPlanning::markForGroup", pObj);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	_markForGroup(pObj);

	xOiDebugFinishFunc(NULL);
    }

    private func _markForGroup(pObj)
    {
	var tObj = pObj[0];

	if (tObj == NULL) {
	    xOiDebugPrint(@ExplWarn, 1, "NULL object passed to markForGroup!");
	    return;
	}

	if (getGroupOf(tObj) != NULL) {
	    xOiDebugPrint(@ExplWarn, 1,
			  ["markForGroup: ", tObj, " already in group"]);
	    return;
	}

        if (xOiObjInList(tObj, mMarkedGroupObjs)) {
            unmarkForGroup(tObj);
            return;
        }

        mMarkedGroupObjs.pushBack(tObj);
    }

    
    
    public func unmarkForGroup(pObj)
    {
	xOiDebugStartFunc2("xOiPlanning::unmarkForGroup", pObj);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if (getName() != "t") {
	    var tPlan = xOiPlanning.getPlanning();
	    if (tPlan != NULL) tPlan.unmarkForGroup(pObj);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	_cleanUpMarkedGroupObjs(); 

        if (!xOiObjInList(pObj, mMarkedGroupObjs)) {
	    xOiDebugPrint(@Warn, 1, "object not marked!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	xOiDebugPrint(@Info, 2, ["marked elements: ", mMarkedGroupObjs]);

        mMarkedGroupObjs.remove(pObj);

	xOiDebugFinishFunc(NULL);
    }

    
    
    private func _cleanUpMarkedGroupObjs()
    {
	var tRes = @();

	var tE;
	foreach(tE;mMarkedGroupObjs) {
	    var tName = NULL;
	    try { tName = tE.getName(); }
	    if (tName != NULL) {
		var tE2 = tE; 
		tRes.pushBack(tE2);
	    }
	}
	mMarkedGroupObjs = tRes;
    }

    
    
    public func isMarkedForGroup(pObj)
    {
	var tIsMarked = xOiObjInList(pObj, mMarkedGroupObjs);
	return(tIsMarked);
    }

    
    
    
    
    
    
    public func markGroupObjects(pObjects, ...)
    {
	xOiDebugStartFunc2("xOiPlanning::markGroupObjects", pObjects);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tObjects = pObjects[0];

	var tObj;
	foreach(tObj; tObjects)	_markForGroup(tObj);

	xOiDebugFinishFunc(NULL);
    }

    
    
    public func unmarkGroupObjects()
    {
	xOiDebugStartFunc("xOiPlanning::unmarkGroupObjects", NULL);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	if (getName() != "t") {
	    var tPlan = xOiPlanning.getPlanning();
	    if (tPlan != NULL) tPlan.unmarkGroupObjects();
	    xOiDebugFinishFunc(NULL);
	    return;
	}

        if (mMarkedGroupObjs.empty()) {
	    xOiDebugPrint(@Info, 1, "no marked objects");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	clearMarkedGroupObjects(); 

	xOiDebugPrint(@Info, 2, ["marked objects: ", mMarkedGroupObjs]);

	xOiDebugFinishFunc(NULL);
    }

    private func clearMarkedGroupObjects()
    {
	xOiDebugStartFunc2("xOiPlanning::clearMarkedGroupObjects", NULL);

	xOiDebugPrint(@Info, 1, ["self ", self]);

        while(mMarkedGroupObjs) {
            var tObj = mMarkedGroupObjs.popFront();
	    
	    var tName = NULL;
	    try { tName = tObj.getName(); };
            if (tName != NULL) {
		getChangeManager().changeIssued(tObj, @UnmarkedForGroup, NULL);
	    }
        }
	xOiDebugFinishFunc(NULL);
    }

    
    public func getMarkedGroupObjects()
    {
	xOiDebugStartFunc("xOiPlanning::getMarkedGroupObjects", NULL);
	xOiDebugPrint(@Info, 1, ["self ", self]);

	
	
	

	var tNewList = @();
	var tObj;
	foreach(tObj; mMarkedGroupObjs) {
	   var tObjName = NULL;
	   try {
		tObjName = tObj.getName();
	   }
	   if (tObjName != NULL) tNewList.pushBack(tObj);
	}
	mMarkedGroupObjs = tNewList;

	
	var tRes = @();
	xOiCopyAggr(mMarkedGroupObjs, tRes, 1);

	if (self.getAppMode(@View) != @Basket) {
	    xOiDebugPrint(@Info, 1, "no basket mode");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}

	

	if (getDynamicProps().hasKey(@_NEED_ALL_MARKED_OBJECTS)) {
	    xOiDebugPrint(@Info, 1, "need all marked objects");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}
	if (!getDynamicProps().hasKey(@_BASKET_COMMON_PROPS_CHANGED)) {
	    xOiDebugPrint(@Info, 1, "no basket common props change");
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	}
	getDynamicProps().remove(@_BASKET_COMMON_PROPS_CHANGED);

	var tInvalidArticles = @();
	if (getDynamicProps().hasKey(@_INVALID_BASKET_ARTICLES))
	    tInvalidArticles = getDynamicProps()[@_INVALID_BASKET_ARTICLES];
	var tArt;
	foreach(tArt; tInvalidArticles) tRes.remove(tArt);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    public func deleteObjects(pObjects)
    {
	xOiDebugStartFunc("xOiPlanning::deleteObjects", pObjects);

	var tObjects = pObjects;
	if (pObjects == NULL) {
	    tObjects = @();
	    xOiCopyAggr(getMarkedGroupObjects(), tObjects, 0);
	    clearMarkedGroupObjects();
	}
	else
	if (pObjects instanceof Int) {
	    
	    tObjects = @();
	    var tObj, tCnt = 0;
	    foreach(tObj; getElements()) {
		if (tCnt++ < pObjects) {
		    var tObj2 = tObj;
		    tObjects.pushBack(tObj2);
		}
		else break;
	    }
	}

	self.startBatchOp(@Delete, tObjects);

	var tObj;
	foreach(tObj; tObjects) 
	    tObj.getFather().remove(tObj);

	self.finishBatchOp(@Delete);

	xOiDebugFinishFunc(NULL);
    }

    
    public func group(pSelType, pSelArgs, ...)
    {
	xOiDebugStartFunc("xOiPlanning::group", [pSelType, pSelArgs]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(NULL);
    }

    
    func getGroupOf(pObj)
    {
	var tF = pObj.getFather();
	while(tF != NULL && !tF.isA(xOiGroup) && !tF.isA(OiPlanning))
	    tF = tF.getFather();

	if (tF == NULL) return(NULL);
	if (tF.isA(OiPlanning)) return(NULL);

	return(tF);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func isUngroupable(pObj, ...)
    {
	var tObj = pObj[0];
	var tMsg = 0;
	if (pObj.size() > 1)
	    tMsg = pObj[1];

	var tMainChild = NULL;

	var tRet = (_isUngroupable(tObj, tMsg) ||
		    (tObj.hasMember(@getMainChild) &&
		     (tMainChild = tObj.getMainChild()) != NULL && 
		     !tObj.isOrderPos() && _isUngroupable(tMainChild, tMsg)));

	return(tRet);
    }

    private func _isUngroupable(pObj, pMsg)
    {
	return((pObj.isA(xOiAggregate) && pObj.isUngroupable(pMsg)) ||
	       (pObj.isCat(@Ungroupable) && pObj.hasMember(@cleanUp)));
    }

    
    
    
    
    
    
    
    
    
    
    
    
    public func unGroup(pGroup)
    {
	xOiDebugStartFunc("xOiPlanning::unGroup", pGroup);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tGroup = pGroup;

	if (tGroup == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tGrpFather = tGroup.getFather();

	if (tGrpFather != self && tGrpFather.isA(xOiPlanning)) {
	    xOiDebugPrint(@Info, 1, "delegating to other root instance");
	    tGrpFather.unGroup(tGroup);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	if (tGrpFather != self && !pGroup.isA(xOiAggregate)) {
	    xOiDebugPrint(@Warn, 1, "group is not a direct child of planning!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tIsUngroupable = self.isUngroupable(tGroup, 1);

	if (!tIsUngroupable) {
	    xOiDebugPrint(@Warn, 1, "given object is not ungroupable!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	mXoiAtGrouping = 1;
        tGroup.unMeasure(); 
	unmarkForGroup(tGroup);

	startMultiInsert(NULL);

        tGroup.cleanUp();

	finishMultiInsert(NULL);

        tGrpFather.remove(tGroup);
	mXoiAtGrouping = 0;

	xOiDebugFinishFunc(NULL);
    }

    
    public func setupProperties4Group(pSelType, pSelArgs, pUnion, pCheckMan)
    {
	xOiDebugStartFunc("xOiPlanning::setupProperties4Group", 
			  [pSelType, pSelArgs, pUnion, pCheckMan]);
	xOiDebugPrint(@Warn, 1, "Method no longer supported/implemented!");
	xOiDebugFinishFunc(0);
	return(0);
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func makeConfigProfileEntry(pTableID, pObj, pProps, pScope)
    {
	xOiDebugStartFunc("xOiPlanning::makeConfigProfileEntry",
			  [pTableID, pObj, pProps, pScope]);

	
	var tTblID = pTableID;
	var tTbl = xOiTable2(tTblID, xOiGlobal.sConfProfileTE);
	tTbl.setIndices(xOiGlobal.sConfProfileIndices);
	if (!tTbl.open()) {
	    
	    xOiDebugPrint(@Warn, 1,
			  ["trying to create not existing table: ", tTblID]);
	    if (!tTbl.create()) {
		xOiDebugPrint(@EvalErr, 1, ["cannot open configuration " +
					    "profile value table: ", tTblID]);
		xOiDebugFinishFunc(NULL);
		return;
	    }
	}

	if (pObj == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	
	var tPID = pObj.getProgram();
	var tManufacturerID = xOiGetAppManufacturerID(tPID);
	
	if (tManufacturerID == NULL || tManufacturerID.empty()) {
	    xOiDebugPrint(@EvalErr, 1, "cannot get manufacturer ID for " +
				       "configuration profile entry!");
	    tTbl.close();
	    xOiDebugFinishFunc(NULL);
	    return;
	}
	var tSeriesID = "";
	if (pScope & 2) {
	    tSeriesID = xOiGetAppSeriesID(tPID);
	    if (tSeriesID == NULL || tSeriesID.empty()) tSeriesID = "NULL";
	}
	var tPClass = "";
	var tArticle = "";
	if (pScope & 8) {
	    var tObj = pObj.getArticleObj();
	    if (tObj != NULL)
	        tArticle = tObj.getArticleSpec();
	    if (tArticle == NULL || tArticle.empty()) tArticle = "NULL";
	}

	var tSParams = [tManufacturerID, tSeriesID, tPClass, tArticle];
	xOiDebugPrint(@Info, 2, ["scope parameters: ", tSParams]);

	
	var tProp;
	foreach(tProp; pProps) {
	    var tVal = pObj.getPropValue(tProp);
	    var tValStr = xOiParam2Str(tVal);

	    if (pScope & 4) {
		tPClass = pObj.getPropClass(tProp);
	        if (tPClass == NULL || tPClass.empty()) tPClass = "NULL";
	    }

	    var tAttributes = @(["manufacturer", tManufacturerID],
				["series", tSeriesID],
				["pclass", tPClass],
				["article", tArticle],
				["property", xOiParam2Str(tProp)]);

	    var tRes = tTbl.read1EntryFor(tAttributes, @("value"), 0);

	    if (tRes != NULL) {
		
		
		if (tRes[0] == tValStr) {
	    	    xOiDebugPrint(@Info, 1,
				  ["entry for ", tProp, "=", tVal,
				   " with scope ", tSParams,
				   " already exists!"]);
		    continue;
		}
		else {
	    	    xOiDebugPrint(@Warn, 1,
				  ["deleting old entry with scope ", tSParams]);
		    tTbl.deleteEntries(tAttributes);
		}
	    }
	    xOiDebugPrint(@Info, 1,
			  ["new entry for ", tProp, "=", tVal, " with scope ",
			   tSParams]);
	    tAttributes.pushBack(["value", tValStr]);
	    tTbl.insertEntry(tAttributes);
	}

	tTbl.close();
	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    public func assignDefaultPropValues(pObj)
    {
	xOiDebugStartFunc("xOiPlanning::assignDefaultPropValues", pObj);

	
	if (!pObj.isA(OiPlElement) && !pObj.isA(OiPart)) {
	    xOiDebugPrint(@Warn, 1, "no article given!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	try {
	    var tObjName = pObj.getName();
	    if (eval(::ofml::app::assignDefaultPropValues(tObjName)) >= 0) {
		xOiDebugPrint(@Info, 1, 
			      "request handled by application function");
		updatePDCache(pObj, 1);
		xOiDebugFinishFunc(NULL);
		return;
	    }
	}

	
	var tProfID = xOiGetAppConfigProfileTableID();
	if (tProfID == NULL) {
	    xOiDebugPrint(@Info, 1, "no configuration profile!");
	    updatePDCache(pObj, 1);
	    xOiDebugFinishFunc(NULL);
	    return;
	    
	}

	xOiDebugPrint(@Info, 2, ["profile ID: ", tProfID]);

	
	var tTbl = xOiTable2(tProfID, xOiGlobal.sConfProfileTE);
	tTbl.setIndices(xOiGlobal.sConfProfileIndices);
	if (!tTbl.open()) {
	    xOiDebugPrint(@EvalErr, 1,
			  ["cannot open configuration profile: ", tProfID]);
	    updatePDCache(pObj, 1);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	
	var tPID = pObj.getProgram();
	var tManufacturerID = xOiGetAppManufacturerID(tPID);
	if (tManufacturerID == NULL || tManufacturerID.empty()) {
	    xOiDebugPrint(@EvalErr, 1, "cannot get manufacturer ID for " +
				       "configuration profile entry!");
	    updatePDCache(pObj, 1);
	    xOiDebugFinishFunc(NULL);
	    return;
	}
	var tSeriesID = xOiGetAppSeriesID(tPID);
	if (tSeriesID == NULL || tSeriesID.empty())
	    tSeriesID = "NULL";
	var tPClass = NULL;
	var tArticle = pObj.getArticleSpec();
	var tObj = pObj.getArticleObj();
	if (tObj != NULL)
	    tArticle = tObj.getArticleSpec();
	if (tArticle == NULL || tArticle.empty())
	    tArticle = "NULL";

	

	var tRepeatAssigned = @();
	var tRepeat = 1;
	var tInconsistency = 0;
	var tProp;
	while (tRepeat && !tInconsistency) {
	    var tPKeys = pObj.getPropertyKeys(); 
	    if (tPKeys.empty()) break;

	    foreach (tProp; tPKeys) {
		tPClass = pObj.getPropClass(tProp);
		if (tPClass == NULL || tPClass.empty()) tPClass = "NULL";

		tRepeat =
		_assignDefaultPropValue(tTbl, pObj, tProp, tManufacturerID,
				        tSeriesID, tPClass , tArticle);
		if (!tRepeat) continue;

		if (tRepeatAssigned.find(tProp) >= 0) {
		    
		    tInconsistency == 1;
		    break;
		}

		
		var tP = tProp; 
		tRepeatAssigned.pushBack(tP);
		tArticle = pObj.getArticleSpec(); 
		if (tArticle == NULL) tArticle = "";
		break;
	    }
	}
	if (tInconsistency)
	    xOiDebugPrint(@EvalErr, 1,
			  "inconsistency due to property profile!");

	updatePDCache(pObj, 1);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func articleInserted(pNewObj, pRefObj, pInsertMode)
    {
	xOiDebugStartFunc("xOiPlanning::articleInserted",
			  [pNewObj, pRefObj, pInsertMode]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	var tPlMode = getPlanningMode();

	if (!sTestMode && (tPlMode == 2 || tPlMode == 3))
	    
	    pNewObj.setTrAxis(0);
	else
	if (!sTestMode && tPlMode == 4 && pInsertMode == @CR) {
	    
	    
	    if (pNewObj.getRtAxis() == 0) pNewObj.setRtAxis(2);
	    if (pNewObj.getTrAxis() == 0) pNewObj.setTrAxis(5);
	}

	finishObjectPaste(pNewObj);

	getChangeManager().changeIssued(pNewObj, @ArticleInserted,
					[pRefObj, pInsertMode]);

	mXoiTmpPlanMode = NULL;

	if (!pNewObj.isA(OiPlElement) && !pNewObj.isA(OiPart)) {
	    xOiDebugPrint(@Warn, 1, "no article given!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	
	
	var tLang = getLanguage();
	var tObj  = pNewObj.getArticleObj();
	var tPID  = tObj.getProgram();
	var tArt  = tObj.getArticleSpec();
	var tMID  = xOiGetAppManufacturerID(tPID);
	var tSID  = xOiGetAppSeriesID(tPID);
	var tPDInfo = tObj.getPDInfo("de");
	xOiDebugPrint(@Info, 2, ["article ", tObj, " (", tArt, ") ", 
				 [tPID, tMID, tSID, tPDInfo]]);
	if (tArt != NULL && tPDInfo != NULL) {
	    if (tPDInfo[0] != tMID) {
	        var tMsg = oiGetStringResource("::ofml::xoi::@E040100", tLang);
		tMsg += " (" + 
			oiGetStringResource("::ofml::xoi::@Article", tLang) +
			": '" + tArt + "', " +
	        	oiGetStringResource(["::ofml::xoi::@WrongRegistryID",
					     String(tPID), 
					     xOiParam2Str(tPDInfo[0]),
					     xOiParam2Str(tMID)], tLang) + ")";
		xOiDebugPrint(@ExplWarn, 1, tMsg);
		afLog(10, "XOI", tMsg);
	    }
	    if (tSID == NULL || 
		xOiParseString2List(tSID, ';').find(tPDInfo[1]) < 0) {
		xOiDebugPrint(@Info, 2, ["wrong series registr ID: ", tSID,
					 " vs. ", tPDInfo[1]]);
	        var tMsg = oiGetStringResource("::ofml::xoi::@E040200", tLang);
		var tSeries = xOiParam2Str(tPDInfo[1]);
		var tRegistry = xOiParam2Str(tSID);
		tMsg += " (" + 
			oiGetStringResource("::ofml::xoi::@Article", tLang) +
			": '" + tArt + "', " +
	        	oiGetStringResource(["::ofml::xoi::@WrongRegistryID",
					     String(tPID), 
					     xOiParam2Str(tPDInfo[1]), 
					     xOiParam2Str(tSID)], tLang) + ")";
		xOiDebugPrint(@ExplWarn, 1, tMsg);
		afLog(10, "XOI", tMsg);
	    }
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    private func _assignDefaultPropValue(pTbl, pObj, pProp, pManufacturerID,
					 pSeriesID, pPClass , pArticle)
    {
	xOiDebugStartFunc("xOiPlanning::_assignDefaultPropValue",
			  [pTbl, pObj, pProp, pManufacturerID,
			   pSeriesID, pPClass , pArticle]);

	var tCriterion = [ pSeriesID+"",	
			   pPClass+"",		
			   pArticle+"" ];	

	var tRet = 0;

	var tEntries =
		pTbl.readNEntriesFor(@(["manufacturer", pManufacturerID],
				       ["property", xOiParam2Str(pProp)]),
				     @("series", "pclass", "article", "value"),
				     0);

	if (tEntries == NULL) {
	    xOiDebugPrint(@Info, 1,
			  ["no entries for manufacturer ", pManufacturerID,
			   " and property ", pProp]);
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	xOiDebugPrint(@Info, 2, ["entries: ", tEntries]);

	
	if (pSeriesID.size() > 0 && pPClass.size() > 0 && pArticle.size() > 0) {
	    tRet = _assignDefaultPropValue2(pObj, pProp, tEntries, tCriterion);
	    if (tRet != NULL) {
		xOiDebugFinishFunc(tRet);
		return(tRet);
	    }
	    xOiDebugPrint(@Info, 2, "full criterion not satisfied");
	}

	
	tCriterion[2] = "";
	if (pSeriesID.size() > 0 && pPClass.size() > 0) {
	    tRet = _assignDefaultPropValue2(pObj, pProp, tEntries, tCriterion);
	    if (tRet != NULL) {
		xOiDebugFinishFunc(tRet);
		return(tRet);
	    }
	    xOiDebugPrint(@Info, 2,
			  "criterion without article spec not satisfied");
	}

	
	tCriterion[1] = "";
	if (pSeriesID.size() > 0) {
	    tRet = _assignDefaultPropValue2(pObj, pProp, tEntries, tCriterion);
	    if (tRet != NULL) {
		xOiDebugFinishFunc(tRet);
		return(tRet);
	    }
	    xOiDebugPrint(@Info, 2,
			  "criterion without article spec and prop class " +
			  "not satisfied");
	}

	
	tCriterion[0] = "";
	tCriterion[1] = pPClass;
	if (pPClass.size() > 0) {
	    tRet = _assignDefaultPropValue2(pObj, pProp, tEntries, tCriterion);
	    if (tRet != NULL) {
		xOiDebugFinishFunc(tRet);
		return(tRet);
	    }
	    xOiDebugPrint(@Info, 2,
			  "criterion without article spec and series " +
			  "not satisfied");
	}

	
	tCriterion[1] = "";

	tRet = _assignDefaultPropValue2(pObj, pProp, tEntries, tCriterion);

	if (tRet == NULL) {
	    tRet = 0;
	    xOiDebugPrint(@Info, 2,
			  "criterion for manufacturer only not satisfied");
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    private func _assignDefaultPropValue2(pObj, pProp, pEntries, pCriterion)
    {
	xOiDebugStartFunc2("xOiPlanning::_assignDefaultPropValue2",
			   [pObj, pProp, pEntries, pCriterion]);

	var tRet = NULL;

	var tEntry;
	foreach(tEntry; pEntries) {
	    var tCheck = [tEntry[0], tEntry[1], tEntry[2]];
	    if (tCheck == pCriterion) {
	    	var tVal = tEntry[3];
		if (tVal[0] == '@')
		    
		    tVal = Symbol(tVal.substr(1));
		else
		    tVal = eval(tEntry[3]);
		var tOldProps = pObj.getProperties();

		var tValCheck = xOiCheckPropertyValue(pObj, pProp, tVal);

		if (tValCheck == 0) continue; 

		if (tValCheck == 2) {
		    
		    tRet = 0; 
		    break;
		}

	    	var tRes = pObj.setPropValue(pProp, tVal);
		tRet = tRes;
		if (tRet && tOldProps == pObj.getProperties())
		    
		    
		    
		    tRet = 0;
		break;
	    }
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    

    
    
    private func startMultiInsert(pArg)
    {
	sMultiInsert = 1;
    }

    
    
    private func finishMultiInsert(pArg)
    {
	sMultiInsert = 0;
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func startBatchOp(pOp, pObjects)
    {
	xOiDebugStartFunc("xOiPlanning::startBatchOp", [pOp, pObjects]);

	if (mXoiBatchOp != NULL) {
	    xOiDebugPrint(@Warn, 1, "there is an ongoing operation!");
	    oiOutput(@MESSAGE, "::ofml::xoi::@ongoingBatchOperation");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	mXoiBatchOp = pOp;
	mXoiBatchObjects = pObjects;

	if ((pOp == @Insert || pOp == @CutPaste || pOp == @CopyPaste) && 
	    pObjects != NULL) 
	    startMultiInsert(NULL);

	getChangeManager().changeIssued(self, @BatchOpStarted, [pOp, pObjects]);

	xOiDebugFinishFunc(1);
    }

    
    
    
    
    
    
    public func finishBatchOp(pOp)
    {
	xOiDebugStartFunc("xOiPlanning::finishBatchOp", pOp);

	if (mXoiBatchOp != pOp) {
	    xOiDebugPrint(@Warn, 1, "no such operation started!");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	if ((pOp == @Insert || pOp == @CutPaste || pOp == @CopyPaste) && 
	    mXoiBatchObjects != NULL) 
	    finishMultiInsert(NULL);

	mXoiBatchOp = NULL;
	mXoiBatchObjects = NULL;

	getChangeManager().changeIssued(self, @BatchOpFinished, pOp);

	xOiDebugFinishFunc(1);
	return(1);
    }

    
    
    public func getBatchOp()
    {
	return(mXoiBatchOp);
    }

    
    
    
    
    public func getBatchObjects()
    {
	return(mXoiBatchObjects);
    }

    

    private func registerWithChangeManager()
    {
	getChangeManager().register(self, @(@PropertyChange), NULL);
    }

    private func initializeStaticStuff()
    {
	xOiOrderItem::sPlanning = NULL;
	sObjectPaste = 0;
    }

    
    rule START_DUMP(pArg)
    {
	xOiDebugStartFunc("xOiPlanning::START_DUMP", NULL);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	unmarkGroupObjects();

	if (getName() != "t") {
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	var tTopItem = self.orderitems;

	tTopItem.removeAllItems();

	if (mXoiBatchOp != NULL) {
	    oiOutput(@MESSAGE, "::ofml::xoi::@ongoingBatchOperation");
	    mXoiBatchObjects = NULL;
	}

	xOiDebugFinishFunc(0);
	return(0);
    }

    
    rule FINISH_DUMP(pArg)
    {
	xOiDebugStartFunc("xOiPlanning::FINISH_DUMP", NULL);
	xOiDebugPrint(@Info, 1, ["self: ", self]);
	xOiDebugFinishFunc(0);
	return(0);
    }

    
    rule START_EVAL(pArg)
    {
	xOiDebugStartFunc("xOiPlanning::START_EVAL", pArg);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if (mMarkedGroupObjs == NULL)
            mMarkedGroupObjs = @();

	if (getName() != "t") {
	    mAtEval = 1;
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	mXoiPrintedMessages = @();

	if (mLibPaths != NULL && mLibPaths instanceof List)
	    mLibPaths = xOiList2Hash(mLibPaths);

	xOiDebugFinishFunc(0);
	return(0);
    }

    
    rule FINISH_EVAL(pArg)
    {
	xOiDebugStartFunc("xOiPlanning::FINISH_EVAL", pArg);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if (self.getName() == "t") {
	    registerWithChangeManager();
	    initializeStaticStuff();
	}

	if (self.getName() == "e") {
	    
	    
	    unmarkGroupObjects();
	}

	if (mAtEval && self.getName() == "t") {
	    var tPDM = getPDManager();
	    xOiDebugPrint(@Info, 2, ["PD Manager ", tPDM]);
	    if (tPDM != NULL && tPDM.isA(xOiPDManager))
		tPDM.cleanUpPDCache();
	    
	    if (mErrorLog instanceof List)
	    	mErrorLog = xOiList2Hash(mErrorLog);
	}

	
	if (mHiddenElems == NULL) {
	    mHiddenElems = @();
	    mHiddenCats = @();
	    mHiddenProgs = @();
	}
	if (mXoiInsertMode == NULL)
	    mXoiInsertMode = 0;
	if (mXoiCheckWallCrossing == NULL)
	    mXoiCheckWallCrossing = 1;
	if (mXoiConversionAppMode == NULL)
	    mXoiConversionAppMode = 0;
	if (mXoiTransformationAppMode == NULL)
	    mXoiTransformationAppMode = @Constrained;
	if (mXoiEditOpAppMode == NULL)
	    mXoiEditOpAppMode = @None;

	if (self.getName() != "t") {
	    mAtEval = 0;
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	if (oiExists("t.order"))
	    self.remove(t.order);
	if (!oiExists("t.orderitems"))
	    xOiOrderItem(self, @orderitems, @Folder, @orderitems, NULL);

	if (mCCheck == NULL || mCCheckRes == NULL)
	    mCCheck = 1; 
	if (mLibPaths == NULL)
	    getDefaultLibPaths();
	if (getInfo(@xoi_tmpgroup) == NULL)
	    addInfoObj(xOiGroupProgInfo, @xoi_tmpgroup);
	if (mXoiNeedElemConfig == NULL)
	    mXoiNeedElemConfig = 3;
	if (mXoiAtGrouping == NULL)
	    mXoiAtGrouping = 0;

	updateProductDBs();
	updateLibPaths();
	reRegisterPackages();

	var tUpdateMode = xOiGetAppUpdateMode();
	if (tUpdateMode != NULL && tUpdateMode == @ImplicitUpdate)
	    updateArticles();

	mAtEval = 0;

	
	

	xOiDebugFinishFunc(0);
	return(0);
    }

    public func manualSetMembers(pMembers)
    {
	var tRes = xOiBasePlanning::manualSetMembers(pMembers);

	if (!pMembers.hasKey(@mHiddenElems))
	    tRes[1].pushBack(@mHiddenElems);
	if (!pMembers.hasKey(@mHiddenCats))
	    tRes[1].pushBack(@mHiddenCats);
	if (!pMembers.hasKey(@mHiddenProgs))
	    tRes[1].pushBack(@mHiddenProgs);
	if (!pMembers.hasKey(@mShowUserBasketPos))
	    tRes[1].pushBack(@mShowUserBasketPos);
	if (!pMembers.hasKey(@mMeasure))
	    tRes[1].pushBack(@mMeasure);
	if (!pMembers.hasKey(@mXoiIgnoreNewElems))
	    tRes[1].pushBack(@mXoiIgnoreNewElems);
	if (!pMembers.hasKey(@mXoiInsertMode))
	    tRes[1].pushBack(@mXoiInsertMode);
	if (!pMembers.hasKey(@mXoiTmpPlanMode))
	    tRes[1].pushBack(@mXoiTmpPlanMode);
	if (!pMembers.hasKey(@mXoiAppViewMode))
	    tRes[1].pushBack(@mXoiAppViewMode);
	if (!pMembers.hasKey(@mXoiCheckWallCrossing))
	    tRes[1].pushBack(@mXoiCheckWallCrossing);
	if (!pMembers.hasKey(@mXoiConversionAppMode))
	    tRes[1].pushBack(@mXoiConversionAppMode);
	if (!pMembers.hasKey(@mXoiTransformationAppMode))
	    tRes[1].pushBack(@mXoiTransformationAppMode);
	if (!pMembers.hasKey(@mXoiEditOpAppMode))
	    tRes[1].pushBack(@mXoiEditOpAppMode);
	if (!pMembers.hasKey(@mXoiNeedElemConfig))
	    tRes[1].pushBack(@mXoiNeedElemConfig);
	if (!pMembers.hasKey(@mXoiAtGrouping))
	    tRes[1].pushBack(@mXoiAtGrouping);
	if (!pMembers.hasKey(@mXoiPrintedMessages))
	    tRes[1].pushBack(@mXoiPrintedMessages);
	if (!pMembers.hasKey(@mXoiBatchOp))
	    tRes[1].pushBack(@mXoiBatchOp);
	if (!pMembers.hasKey(@mXoiBatchObjects))
	    tRes[1].pushBack(@mXoiBatchObjects);
	if (!pMembers.hasKey(@mXoiObj2OrderID))
	    tRes[1].pushBack(@mXoiObj2OrderID);

	
	if (pMembers.hasKey(@mXoiExportAppMode))
	    tRes[0].pushBack(@mXoiExportAppMode);

	
	if (pMembers.hasKey(@mXoiIgnoreNewElems))
	    tRes[0].pushBack(@mXoiIgnoreNewElems);
	if (pMembers.hasKey(@mInterimPriceLevel))
	    tRes[0].pushBack(@mInterimPriceLevel);
	if (pMembers.hasKey(@mOrderMode))
	    tRes[0].pushBack(@mOrderMode);
	if (pMembers.hasKey(@mOrderDepth))
	    tRes[0].pushBack(@mOrderDepth);
	if (pMembers.hasKey(@mWorkFolder))
	    tRes[0].pushBack(@mWorkFolder);
	if (pMembers.hasKey(@mArticleMap))
	    tRes[0].pushBack(@mArticleMap);
	if (pMembers.hasKey(@mLastArtIDNr))
	    tRes[0].pushBack(@mLastArtIDNr);
	if (pMembers.hasKey(@mXoiObj2OrderID))
	    tRes[0].pushBack(@mXoiObj2OrderID);
	if (pMembers.hasKey(@mOrderPosMap))
	    tRes[0].pushBack(@mOrderPosMap);
	if (pMembers.hasKey(@mOrderPosMapDone))
	    tRes[0].pushBack(@mOrderPosMapDone);
	if (pMembers.hasKey(@mOrderPrice))
	    tRes[0].pushBack(@mOrderPrice);
	if (pMembers.hasKey(@mAccumulatesPrice))
	    tRes[0].pushBack(@mAccumulatesPrice);
	if (pMembers.hasKey(@mShowUserBasketPos))
	    tRes[0].pushBack(@mShowUserBasketPos);

	return (tRes);
    }
}

xOiGlobal();
xOiMath();
xOiFuncs();
xOiDebug();


