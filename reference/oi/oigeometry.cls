// === ofml/oi/oigeometry.cls ===
package ::ofml::oi;
import ::egr::found::*;

















public class OiGeometry: Top
{
    public static var sRec = 0;

    public static var sPi2 = acos(0);
    public static var sPi4 = 0.5*sPi2;
    public static var sPi = 2*sPi2;
    public static var s2Pi = 2*sPi;
    public static var eps = 0.005;

    var mIsCutable;
    var mIsCopyable;

    var mCat;
    var mMat;	

    
    static var sStateTypes2GFStateFlags;

    
    public func initialize(pFa, pNa)
    {
	mCat = @ANY;
	mIsCutable = 0;
	notSelectable();

	
	
    }

    private func setupStateTypes2GFStateFlagsHash()
    {
	sStateTypes2GFStateFlags = Hash();

	

	sStateTypes2GFStateFlags[@OI_PickTarget] 	= 0x0004;
	sStateTypes2GFStateFlags[@OI_Visible2D] 	= 0x0008;
	sStateTypes2GFStateFlags[@OI_PickTarget2D] 	= 0x0010;
	sStateTypes2GFStateFlags[@OI_CamCollide] 	= 0x0020;
	sStateTypes2GFStateFlags[@OI_VisibleDist] 	= 0x0080;
	sStateTypes2GFStateFlags[@OI_VisibleBBox] 	= 0x0100;
	sStateTypes2GFStateFlags[@OI_PersSave] 		= 0x0400;
    }

    
    
    
    
    public func hierSelectable()
    {
	setState(getState() | 0x0040);
    }

    
    
    
    
    public func notHierSelectable()
    {
	setState(getState() & ~0x0040);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setObjState(pStateType, pValue)
    {
	oiDebugStartFunc("OiGeometry::setObjState", [pStateType, pValue]);

	if (sStateTypes2GFStateFlags == NULL) 
	    setupStateTypes2GFStateFlagsHash();

	if (sStateTypes2GFStateFlags.keys().find(pStateType) < 0) {
	    oiDebugPrint(@Warn, 1, "invalid state type!");
	    oiDebugFinishFunc(NULL);
	}

	if (pValue) 
	    setState(getState() | sStateTypes2GFStateFlags[pStateType]);
	else
	    setState(getState() & ~sStateTypes2GFStateFlags[pStateType]);

	oiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    public func getObjState(pStateType)
    {
	oiDebugStartFunc("OiGeometry::getObjState", pStateType);

	if (sStateTypes2GFStateFlags == NULL) 
	    setupStateTypes2GFStateFlagsHash();

	if (sStateTypes2GFStateFlags.keys().find(pStateType) < 0) {
	    oiDebugPrint(@Warn, 1, "invalid state type!");
	    oiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRet = ((getState() & sStateTypes2GFStateFlags[pStateType]) > 0);

	oiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    public func beAnElement()
    {
	_beAnElement();

	var tF = getFather();
	if (tF != NULL && tF.isCat(@IF_Base))
	    tF.childAddedToElements(self);
    }

    public func unBeAnElement()
    {
	_unBeAnElement();

	var tF = getFather();
	if (tF != NULL && tF.isCat(@IF_Base))
	    tF.childRemovedFromElements(self);
    }

    public func childAddedToElements(pEl)
    {
    }

    public func childRemovedFromElements(pCh)
    {
    }

    
    
    

    
    public func isCat(pCat)
    {
	if (pCat == @IF_MObject || 
	    pCat == @IF_Base || 
	    pCat == @IF_Material) 
	    return(1);

	return(0);
    }

    
    public func getProgram()
    {
	var tPlanning = getPlanning();
	if (tPlanning == NULL) return(NULL); 

	var tPlEl = tPlanning.getPlElementUp(self);

	if (tPlEl == NULL) return(NULL);

	return(tPlEl.getProgram());
    }

    
    public func isElemCatValid(pCat)
    {
	return(0);
    }

    
    
    

    
    public func setRecursionState(pState)
    {
	sRec = pState;
    }

    
    public func getRecursionState()
    {
	return(sRec);
    }

    
    
    

    public func hasMaterialIF(pObj)
    {
	var tHasMaterialIF = 0;
	if (pObj.hasMember(@isCat))
	    tHasMaterialIF = pObj.isCat(@IF_Material);
	return(tHasMaterialIF);
    }

    
    
    
    public func isMatCat(pCat) 
    {
	var tC;
	foreach(tC; getMatCategories())
	    if (tC == pCat) return(1);

	return(0);
    }
	
    
    public func setMatCat(pCat)
    {
	mCat = pCat;
    }

    
    public func getMatCategories()
    {
	if (mCat == NULL) return(NULL);

	return([mCat]);
    }

    
    public func getAllMatCats()
    {
	return(getMatCategories());
    }

    
    public func setCMaterial(pCat, pMat)
    {
	oiDebugStartFunc("OiGeometry::setCMaterial", [pCat, pMat]);

	oiDebugPrint(@Info, 1, ["self ", self]);

	var tOK = 0;
	if (isMatCat(pCat)) {
	    oiDebugPrint(@Info, 1, "is material category");
	    mMat = pMat;
	    tOK = 1;
	    var tMatName = self.getMatName(pMat);
	    oiDebugPrint(@Info, 2, ["material name: ", tMatName]);
	    if (tMatName == NULL)
		oiDebugPrint(@ExplWarn, 1, ["getMatName(", pMat, 
					    ") returned NULL for ", self, "!"]);
	    else {
	        setMaterial(tMatName);
		
		
		
		
		
		
		
		if (0 && oiExists(getName()+".geo"))
		    geo.setMaterial(tMatName);
	    }
	}

	
	var tCh;
	foreach(tCh; getChildren()) {
	    if (hasMaterialIF(tCh))
		if (tCh.setCMaterial(pCat, pMat) == 1) tOK = 1;
	}

	oiDebugFinishFunc(tOK);
	return(tOK);
    }
    
    
    public func getCMaterial(pCat)
    {
	if (isMatCat(pCat)) return(mMat);

	return(NULL);
    }

    public func getCMaterials(pCat)
    {
	var tF = getFather();
	if (hasMaterialIF(tF)) return(tF.getCMaterials(pCat));

	return(NULL);
    }

    public func getMatName(pMat)
    {
	var tF = getFather();
	if (hasMaterialIF(tF)) return(tF.getMatName(pMat));

	return(NULL);
    }

    
    
    

    
    public func setCutable(pVal)
    {
	mIsCutable = pVal;
    }

    
    public func isCutable()
    {
	return(mIsCutable);
    }

    
    public func setCopyable(pVal)
    {
	mIsCopyable = pVal;
    }

    
    public func isCopyable()
    {
	if (mIsCopyable == NULL) mIsCopyable = mIsCutable;

	return(mIsCopyable);
    }

    
    public func measure(pMd)
    {
    }

    
    public func unMeasure()
    {
    }

    
    
    

    
    public func getPlanning()
    {
	return(oiGetPlanning());
    }

    
    public func getRoot()
    {
	var tF, tRoot = self;
	while ((tF = tRoot.getFather()) != NULL) tRoot = tF;

	return(tRoot);
    }

    public func getType()
    {
	return(typeOf(self));
    }

    
    public func setAlignment(pAlign)
    {
	if (afExists(getName()+".geo")) {
	    self.geo.setPosition([0, 0, 0]);

	    var tBnds = self.geo.getLocalBounds();
	    var tX = tBnds[0][0];
	    var tY = tBnds[0][1];
	    var tZ = tBnds[0][2];

	    if (pAlign[0] == @C) tX = 0.5*(tBnds[0][0]+tBnds[1][0]);
	    else if (pAlign[0] == @A) tX = tBnds[1][0];

	    if (pAlign[1] == @C) tY = 0.5*(tBnds[0][1]+tBnds[1][1]);
	    else if (pAlign[1] == @A) tY = tBnds[1][1];

	    if (pAlign[2] == @C) tZ = 0.5*(tBnds[0][2]+tBnds[1][2]);
	    else if (pAlign[2] == @A) tZ = tBnds[1][2];

	    self.geo.setPosition([-tX, -tY, -tZ]);
	}
	else 
	    afError("AFML/OI internal error: No geo object in type " +
		    getClass() + ".");
    }

    public func manualSetMembers(pMembers)
    {
	var tRes = [[], []];
	if (!pMembers.hasKey(@mIsCopyable))
	    tRes[1].pushBack(@mIsCopyable);
	return (tRes);
    }
}

OiGlobal();

