// === ofml/xoi/xoiplelement.cls ===
package ::ofml::xoi;

import ::ofml::oi::*;
import ::stdio::*;




















public class xOiPlElement: OiPlElement
{
    var mActAttPt;

    
    static var sxOiPlElementSetGetMembers =
    @(@ActiveAttPt, @StdAttPts, @AddAttPts, @AttPtsOrder, @AddAttPtDef);

    
    public func initialize(pFa, pNa)
    {
	OiPlElement::initialize(pFa, pNa);

	setRtAxis(2);
	setTrAxis(5);
    }

    public func forceDynamicProp(pPKey)
    {
        if (xOiPlElement.sxOiPlElementSetGetMembers.find(pPKey) >= 0) return(1);

        return(OiPlElement::forceDynamicProp(pPKey));
    }

    public func isCat(pCat)
    {
	if (pCat == @IF_ATTPT || pCat == @IF_AttachPts) return(1);
	else
	    return(OiPlElement::isCat(pCat));
    }

    
    
    
    
    
    
    
    

    
    
    
    
    
    
    
    
    public func getStdAttPts()
    {
	return(oiGetStdAttPts());
    }

    
    
    
    
    
    public func getAddAttPts()
    {
	return(NULL);
    }

    
    
    
    
    
    
    
    
    
    public func getAttPtsOrder()
    {
	xOiDebugStartFunc("xOiPlElement::getAttPtsOrder", NULL);

	var tRet = oiGetStdAttPtsOrder(self);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getAddAttPtDef(pKey)
    {
	return(NULL);
    }

    
    
    
    
    
    
    
    
    
    public func oppositeAttPts(pKey, pDir)
    {
	return(oiGetOppositeAttPts4Std(self, pKey, pDir));
    }

    
    
    public func setActiveAttPt(pKey)
    {
	mActAttPt = pKey;
    }

    
    
    
    public func getActiveAttPt()
    {
	if (mActAttPt != NULL) return(mActAttPt);

	var tCh;
	foreach(tCh; getChildren())
	    if ((tCh instanceof xOiAttPtInteractor) && tCh.isActive())
		return(tCh.getKey());

	return(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    protected func showStdAttPts()
    {
	xOiAttPtIF.showStdAttPts(self);
    }

    
    
    
    protected func showAddAttPts()
    {
	xOiAttPtIF.showAddAttPts(self);
    }

    private func mustShowAddAttPts()
    {
	xOiDebugStartFunc("xOiPlElement::mustShowAddAttPts", NULL);

	var tRet = 1;

	var tTbl = openDataTbl();
	if (tTbl == NULL) {
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tRes = tTbl.readNEntriesFor(@(["type", "@ShowAddAttPts"]),
				        @("args", "value"), 0);
	if (tRes == NULL) {
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	tRet = mustShowAttPts(tRes, getAddAttPts());

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    private func mustShowAttPts(pEntries, pAttPts)
    {
	xOiDebugStartFunc("xOiPlElement::mustShowAttPts", [pEntries, pAttPts]);

	var tCount = 0;
	if (pAttPts != NULL)
	    tCount = pAttPts.size();

	var tRet = 0; 
	var tE;
	foreach(tE; pEntries) {
	    var tArgs  = tE[0];
	    if (tArgs.empty()) 
		tArgs = "[@CountGT, 0]";
	    try {
		tArgs = eval("@("+tArgs+")");
	    }
	    catch(&e: Error) {
		xOiDebugPrint(@EvalErr, 1,
			      ["could not evaluate argument ", tE[0], 
			       ": ", e.what]);
		continue;
	    }
	    var tOk = 1;
	    var tCond;
	    foreach(tCond; tArgs) {
		xOiDebugPrint(@Info, 2, ["check condition ", tCond]);
		if (tCond[0] == @CountGT) {
		    if (tCount <= tCond[1]) {
			tOk = 0;
			break;
		    }
		}
		else
		if (tCond[0] == @Category) {
		    var tMatch = 0;
		    var tCat;
		    foreach(tCat; tCond[1])
			if (self.isCat(tCat)) {
			    tMatch = 1;
			    break;
			}
		    if (!tMatch) {
			tOk = 0;
			break;
		    }
		}
		else
		if (tCond[0] == @Article) {
		    var tMatch = 0;
		    var tMyArt = self.getArticleObj().getArticleSpec();
		    xOiDebugPrint(@Info, 2, ["my article: ", tMyArt]);
		    if (tMyArt == NULL && !tCond[1].empty()) {
			tOk = 0;
			break;
		    }
		    var tArt;
		    foreach(tArt; tCond[1])
			if (tMyArt.substr(0, tArt.size()) == tArt) {
			    tMatch = 1;
			    break;
			}
		    if (!tMatch) {
			tOk = 0;
			break;
		    }
		}
	    }
	    xOiDebugPrint(@Info, 2, ["all conditions ok? ", tOk]);

	    if (!tOk) continue;

	    try {
		tRet = Int(tE[1]);
	    }
	    catch (&e: Error) {
		xOiDebugPrint(@EvalErr, 1,
			      ["could not convert value ", tE[1],
			       " for @ShowAddAttPts to Int!"]);
	    }
	    break;
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkAdd(pType, pObj, pPos, pParams)
    {
	xOiDebugStartFunc("xOiPlElement::checkAdd", 
			  [pType, pObj, pPos, pParams]);

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

	xOiDebugPrint(@Info, 2, ["child ", self.ch]);

	if (self.ch.isCat(@TOP_ELEM) || 
	    (!self.ch.isCat(@IF_ATTPT) && !self.ch.isCat(@IF_AttachPts))) {
	    xOiDebugPrint(@Info, 1, "using inherited implementation");
	    var tRet = OiPlElement::checkAdd(pType, pObj, pPos, pParams);
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tCPR = NULL;

	var tCollCheckMode = (self.ch.isA(xOiFreeArticle) ? 2 : 1);

	

	if (pObj != NULL && pObj != self && 
	    (pObj.isCat(@IF_ATTPT) || pObj.isCat(@IF_AttachPts)))
	    tCPR = oiGetPosRot4AttachPts(pObj, self.ch, NULL, tCollCheckMode);

	xOiDebugPrint(@Info, 2, ["sibling pos: ", tCPR]);

	if (tCPR != NULL) {
	    var tPos = tCPR[0];
	    var tRot = tCPR[1];
	    var tErr = tCPR[2];
	    if (tErr != NULL || tPos == NULL) {
	        xOiDebugFinishFunc(NULL);
		return(removeCh(tErr));
	    }

	    if (self.ch.isA(OiPlElement))
	    	setMethod("onCreate("+String(tRot)+", NULL, NULL)");

	    xOiDebugFinishFunc(tPos);
	    return(acceptCh(tPos));
	}

	
	tCPR = oiGetPosRot4AttachPts(self, self.ch, NULL, tCollCheckMode);

	xOiDebugPrint(@Info, 2, ["child pos: ", tCPR]);

	if (tCPR == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return(removeCh(NULL));
	}

	var tPos = tCPR[0];
	var tRot = tCPR[1];
	var tErr = tCPR[2];
	if (tErr != NULL || tPos == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return(removeCh(tErr));
	}

	if (self.ch.isA(OiPlElement))
	    setMethod("onCreate("+String(tRot)+", NULL, NULL)");

	xOiDebugFinishFunc(tPos);
	return(acceptCh(tPos));
    }

    
    
    
    
    public func checkElPos(pEl, pOldPos)
    {
	xOiDebugStartFunc("xOiPlElement::checkElPos", [pEl, pOldPos]);

	var tRet = xOiCheckElPos(self, pEl, pOldPos);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func autoDecoration(pContext, pTemplate)
    {
	xOiDebugStartFunc("xOiPlElement::autoDecoration",
			  [pContext, pTemplate]);

	var tObjects = @(self);
	var tPlanning = getPlanning();
	if (tPlanning.isA(xOiPlanning))
	    tObjects = tPlanning.resolveObjects4AutoDeco(tObjects, pContext);
	var tCheckSelf = (tObjects.size() == 1 && tObjects.front() == self);
	var tObj;
	foreach(tObj; tObjects) {
	    var tOk = xOiAutoDecoration4Obj(tObj, pContext, pTemplate);
	    if (tCheckSelf && !tOk)
		oiOutput(@WARNING, "::ofml::xoi::@NoAutoDeco");
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    public func deleteAutoDecoration()
    {
	deleteAutoDecoration2(NULL);
    }

    
    
    
    
    public func deleteAutoDecoration2(pContext, ...)
    {
	var tContext = pContext[0];

	var tObjects = @(self);
	var tPlanning = getPlanning();
	if (tPlanning.isA(xOiPlanning))
	    tObjects = tPlanning.resolveObjects4AutoDeco(tObjects);
	var tObj;
	foreach(tObj; tObjects) xOiDeleteAutoDeco(tObj, tContext);
    }

    
    
    
    
    
    
    
    
    public func getMetaInfo(pInfoType, pArgs)
    {
	return(xOiGetMetaInfo(self, pInfoType, pArgs));
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    rule PICK (pVal)
    {
	xOiDebugStartFunc("xOiPlElement::PICK", pVal);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	mActAttPt = NULL;

	if (mustShowAddAttPts()) showAddAttPts();

	oiDebugFinishFunc(0);
	return(0);
    }

    
    
    
    rule UNPICK (pVal)
    {
	xOiDebugStartFunc("xOiPlElement::UNPICK", pVal);

	oiDebugPrint(@Info, 1, ["self: ", self]);

	xOiAttPtIF.hideAttPts(self);

	oiDebugFinishFunc(0);
	return(0);
    }

    
    
    
    
    rule INTERACTOR(pSym)
    {
	if (hasMember(pSym)) xOiAttPtIF.attPtSelected(self, $pSym);

	return(0);
    }
}

xOiAttPtIF();

