// === ofml/xoi/xoitable2.cls ===
package ::ofml::xoi;

import ::ofml::oi::*;





















public class xOiTable2
{
    static var sOpenErr  	= "error while opening table ";
    static var sNotOpenErr  	= "table not open!";
    static var sReadErr  	= "table read failed!";
    static var sManyEntryErr 	= "more than one entry found!";
    static var sNoEntryErr 	= "no entry found!";
    static var sUnknownAttrErr 	= "unknown table attribute!";

    var mIsOpen;
    var mID;
    var mStruct;
    var mIndices;
    var mErrPrintMode;

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func initialize(pID, pStructure)
    {
	mIsOpen = 0;
	mID = pID;

	
	mStruct = @();
	xOiCopyAggr(pStructure, mStruct, 1);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setIndices(pIndices)
    {
	mIndices = pIndices;
    }

    
    public func getID()
    {
	return(mID);
    }

    
    
    public func setErrPrintMode(pMode)  
    {
	mErrPrintMode = pMode;
    }   
        
    
    
    public func getErrPrintMode()
    {
	if (mErrPrintMode == NULL) return(@TableErr);
            
	return(mErrPrintMode);
    }

    public func printStructure(pStruct)
    {
	var tTA;
	foreach(tTA; pStruct) {
	    var tName = tTA[0];
	    var tPrimKey = tTA[1];
	    var tKey = tTA[2];
	    var tType = "@"+String(tTA[3]);
	    var tValue = tTA[4];
	    if (tValue == NULL) tValue = "NULL";
	    else		tValue = String(tValue);
	    var tFormat = tTA[5];

	    ::stdio::stderr.printf("  %s: %d, %d, %s, '%s', '%s'\n", tName, 
				   tPrimKey, tKey, tType, tValue, tFormat);
	}
    }

    
    
    public func getAttrDef(pAttrName)
    {
	var tTA;
	foreach(tTA; mStruct)
	    if (tTA[0] == pAttrName) return(tTA);

	xOiDebugPrint(getErrPrintMode(), 1, pAttrName + ": " + sUnknownAttrErr);
	return(NULL);
    }

    
    
    
    public func getAttrIdx(pAttrName)
    {
	var tTA;
	var i = 0;
	foreach(tTA; mStruct) {
	    if (tTA[0] == pAttrName) return(i);
	    i++;
	}

	return(-1);
    }

    
    public func isOpen()
    {
	return(mIsOpen);
    }

    
    
    
    public func open()
    {
	if (mIsOpen) return(1);

	xOiDebugStartFunc("xOiTable2::open", mID);

	mIsOpen = 0;

	xOiDebugPrint(@Info, 2, ["indices: ", mIndices]);

	try {
	    if (mID.rfind("VAR ", 0) != 0) {
		if (mIndices == NULL)
		    mIsOpen = oiTable(@openTbl, @([mID, mStruct]));
		else
		    mIsOpen = oiTable(@openTbl, @([mID, mStruct, mIndices]));
	    } else {
		var tIds = splitVarId(mID);
		xOiDebugPrint(@Info, 2, ["splitted table IDs: ", tIds]);
		if (tIds instanceof List) {
		    var tId, tErr;
		    foreach (tId; tIds) {
		        xOiDebugPrint(@Info, 2, ["check table ID: ", tId]);
			try {
			    if (mIndices == NULL)
				mIsOpen = oiTable(@openTbl,
						  @([tId, mStruct]));
			    else
				mIsOpen = oiTable(@openTbl, 
						  @([tId, mStruct, mIndices]));
		            xOiDebugPrint(@Info, 2, ["open: ", mIsOpen]);
			    if (mIsOpen) {
				mID = tId;
				break;
			    }
			} catch (&e : Error) {
			    tErr = e;
		            xOiDebugPrint(@Warn, 1, e.where + ": " + e.what);
			}
		    }
		    if (!mIsOpen && tErr != NULL)
			throw tErr;
		}
	    }
	}
	catch (&e: Error) {
	    var tMsg = mID + ": " + e.what;
	    xOiDebugPrint(getErrPrintMode(), 1, tMsg);
	}
	xOiDebugFinishFunc(mIsOpen);
	return(mIsOpen);
    }

    
    
    
    public func create()
    {
	if (mIsOpen) return(1);

	xOiDebugStartFunc("xOiTable2::create", mID);

	mIsOpen = 0;

	try {
	    if (mIndices == NULL)
		mIsOpen = oiTable(@createTbl, @([mID, mStruct]));
	    else {
		xOiDebugPrint(@Info, 2, 
			      ["calling oiTable() with op @createTbl " +
			       "and attributes ", @([mID, mStruct, mIndices])]);
		mIsOpen = oiTable(@createTbl, @([mID, mStruct, mIndices]));
		if (mIsOpen == NULL) mIsOpen = 0;
	    }
	}
	catch (&e: Error) {
	    var tMsg = mID + ": " + e.what;
	    xOiDebugPrint(getErrPrintMode(), 1, tMsg);
	    mIsOpen = 0;
	}
	xOiDebugFinishFunc(mIsOpen);
	return(mIsOpen);
    }

    private func splitVarId(pId)
    {
	if (pId.rfind("VAR ", 0) != 0)
	    return (NULL);
	var tLen = pId.size(), tPos0 = 4, tPos1;

	for (; tPos0 < tLen && pId[tPos0] == ' '; ++tPos0) ;
	if ((tPos1 = pId.find(' ', tPos0)) == -1)
	    return (NULL);
	var tFmts = pId.substr(tPos0, tPos1 - tPos0);

	tPos0 = tPos1 + 1;
	for (; tPos0 < tLen && pId[tPos0] == ' '; ++tPos0) ;
	if ((tPos1 = pId.find(' ', tPos0)) == -1)
	    return (NULL);
	var tDir = pId.substr(tPos0, tPos1 - tPos0);

	tPos0 = tPos1 + 1;
	for (; tPos0 < tLen && pId[tPos0] == ' '; ++tPos0) ;
	if ((tPos1 = pId.find(' ', tPos0)) == -1)
	    return (NULL);
	var tDbName = pId.substr(tPos0, tPos1 - tPos0);

	tPos0 = tPos1 + 1;
	for (; tPos0 < tLen && pId[tPos0] == ' '; ++tPos0) ;
	if ((tPos1 = pId.find(' ', tPos0)) == -1)
	    tPos1 = tLen;
	var tTableName = pId.substr(tPos0, tPos1 - tPos0);

	var tRes = @();
	tLen = tFmts.size();
	for (tPos0 = 0; tPos0 < tLen; tPos0 = tPos1 + 1) {
	    if ((tPos1 = tFmts.find(',', tPos0)) == -1)
		tPos1 = tFmts.size();
	    var tFmt = tFmts.substr(tPos0, tPos1 - tPos0);
	    var tId = tFmt + " " + tDir + "/";
	    if (tFmt == "EBASE") {
		tId += tDbName + ".ebase " + tTableName;
	    } else if (tFmt == "FTXT") {
		var tSfxPos = tTableName.rfind('_');
		if (tSfxPos == -1) {
		    tId += tTableName + ".txt " + tTableName;
		} else {
		    var tBaseName = tTableName.substr(0, tSfxPos);
		    var tSuffix = tTableName.substr(tSfxPos + 1);
		    tId += tBaseName + "." + tSuffix + " " + tBaseName;
		}
	    } else if (tFmt == "CSV") {
		tId += tTableName + ".csv " + tTableName;
	    } else {
		xOiDebugPrint(getErrPrintMode(), 1,
		  "invalid table fmt: " + tFmt);
		tId = NULL;
	    }
	    if (tId != NULL)
		tRes.pushBack(tId);
	}

	return (tRes);
    }

    
    public func close()
    {
	if (!mIsOpen) return;

	xOiDebugStartFunc("xOiTable2::close", mID);

	
	
	
	
	try {
	    oiTable(@closeTbl, @(mID));
	}
	catch (&e: Error) {
	    oiDebugPrint(@TableErr, 1, 
			 ["cannot close table ", mID, ": " + e.what]);
	}

	mIsOpen = 0;

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    public func readFirstEntry()
    {
	if (!mIsOpen) {
	    xOiDebugPrint(getErrPrintMode(), 1, sNotOpenErr);
	    return(NULL);
	}

	var tRes  = oiTable(@getFirstTE, mID);

	if (tRes == NULL)
	    xOiDebugPrint(getErrPrintMode(), 1, mID+": "+sReadErr);
	else {
	    var tOutTAL = tRes[1];
	    tRes = @();
	    var tTA;
	    foreach(tTA; tOutTAL) tRes.pushBack(tTA[4]);
	}

	return(tRes);
    }

    
    
    
    
    
    public func readNextEntry()
    {
	if (!mIsOpen) {
	    xOiDebugPrint(getErrPrintMode(), 1, sNotOpenErr);
	    return(NULL);
	}

	var tRes  = oiTable(@getNextTE, mID);

	if (tRes == NULL)
	    xOiDebugPrint(getErrPrintMode(), 1, mID+": "+sReadErr);
	else {
	    var tOutTAL = tRes[1];
	    tRes = @();
	    var tTA;
	    foreach(tTA; tOutTAL) tRes.pushBack(tTA[4]);
	}

	return(tRes);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func read1EntryFor(pInAttr, pOutAttr, pReport)
    {
	xOiDebugStartFunc("xOiTable2::read1EntryFor", 
			  [pInAttr, pOutAttr, pReport]);

	if (pInAttr == NULL  || pInAttr.empty() ||
	    pOutAttr == NULL || pOutAttr.empty()) {
	    xOiDebugFinishFunc("NULL (invalid arg!)");
	    return(NULL);
	}

	var tErrPrintMode = getErrPrintMode();

	if (!mIsOpen) {
	    xOiDebugPrint(tErrPrintMode, 1, sNotOpenErr);
	    xOiDebugFinishFunc("NULL (table not open!)");
	    return(NULL);
	}

	var tInTAL = @();
	var tA, tTA;
	foreach(tA; pInAttr) {
	    var tAName  = tA[0];
	    var tAValue = tA[1];
	    if (tAValue == NULL) {
	        xOiDebugPrint(tErrPrintMode, 1, 
			      ["void value for attribute ", tAName]);
	        xOiDebugFinishFunc("NULL (void attribute value!)");
		return(NULL);
	    }
	    xOiDebugPrint(@Info, 2, ["attribute ", tAName, ": ", tAValue]);
	    tTA = getAttrDef(tAName);
	    if (tTA == NULL) {
	        xOiDebugFinishFunc("NULL (unknown attribute!)");
		return(NULL);
	    }
	    tTA[4] = tAValue;
	    tInTAL.pushBack(tTA);
	}

	var tResL = NULL;
	var tRes  = oiTable(@readTE, @([mID, tInTAL]));

	if (tRes != NULL) {
	    if (tRes.size() > 1) {
		if (pReport)
		    xOiDebugPrint(tErrPrintMode, 1, mID+": "+sManyEntryErr);
	    }
	    else
	    if (tRes.size() == 0) {
		if (pReport)
		    xOiDebugPrint(tErrPrintMode, 1, mID+": "+sNoEntryErr);
	    }
	    else {
		var tOutTAL = tRes[0][1];
		tResL = @();
		foreach(tA; pOutAttr)
		    tResL.pushBack(tOutTAL[getAttrIdx(tA)][4]);
	    }
	}
	else
	    xOiDebugPrint(tErrPrintMode, 1, mID+": "+sReadErr);

	xOiDebugFinishFunc(tResL);
	return(tResL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func readNEntriesFor(pInAttr, pOutAttr, pReport, ...)
    {
	xOiDebugStartFunc("xOiTable2::readNEntriesFor", 
			  [pInAttr, pOutAttr, pReport]);

	if (pInAttr == NULL  || pInAttr.empty() ||
	    pOutAttr == NULL || pOutAttr.empty()) {
	    xOiDebugFinishFunc("NULL (invalid arg!)");
	    return(NULL);
	}

	var tErrPrintMode = getErrPrintMode();

	if (!mIsOpen) {
	    xOiDebugPrint(tErrPrintMode, 1, sNotOpenErr);
	    xOiDebugFinishFunc("NULL (table not open!)");
	    return(NULL);
	}

	var tReport = pReport[0];
	var tReturnVector = 0;
	if (pReport.size() > 1)
	    tReturnVector = pReport[1];
	var tFilter = 0;
	if (pReport.size() > 2)
	    tFilter = pReport[2];

	var tInTAL = @();
	var tA, tTA;
	foreach(tA; pInAttr) {
	    var tAName  = tA[0];
	    var tAValue = tA[1];
	    if (tAValue == NULL) {
	        xOiDebugPrint(tErrPrintMode, 1, 
			      ["void value for attribute ", tAName]);
	        xOiDebugFinishFunc("NULL (void attribute value!)");
		return(NULL);
	    }
	    xOiDebugPrint(@Info, 2, ["attribute ", tAName, ": ", tAValue]);
	    tTA = getAttrDef(tAName);
	    if (tTA == NULL) {
	        xOiDebugFinishFunc("NULL (unknown attribute!)");
		return(NULL);
	    }
	    tTA[4] = tAValue;
	    tInTAL.pushBack(tTA);
	}

	var tIdcs, tResIdcs, tIdx, tRes, tResL = NULL;

	if (tFilter) {
	    tIdcs = Vector(0);
	    foreach(tA; pOutAttr) tIdcs.pushBack(getAttrIdx(tA));
	    xOiDebugPrint(@Info, 2, ["indices: ", tIdcs]);
	    tRes  = oiTable(@readTE, @([mID, tInTAL]), tIdcs);
	    tResIdcs = filterIdcs2ReturnIdcs(tIdcs);
	    xOiDebugPrint(@Info, 2, ["result indices: ", tResIdcs]);
	}
	else
	    tRes  = oiTable(@readTE, @([mID, tInTAL]));

	xOiDebugPrint(@Info, 2, ["oiTable() result: ", tRes]);

	if (tRes != NULL) {
	    if (tRes.size() == 0) {
		if (tReport)
		    xOiDebugPrint(tErrPrintMode, 1, mID+": "+sNoEntryErr);
	    }
	    else {
		tResL = @();
		if (tReturnVector) tResL = Vector(0);
		var tR;
		foreach(tR; tRes) {
		    var tOutTAL = tR[1];
		    xOiDebugPrint(@Info, 2, ["out attributes: ", tOutTAL]);
		    var tAVL = @();
		    if (tFilter) {
			foreach(tIdx; tResIdcs) tAVL.pushBack(tOutTAL[tIdx][4]);
		    }
		    else {
		        foreach(tA; pOutAttr)
	    	    	    tAVL.pushBack(tOutTAL[getAttrIdx(tA)][4]);
		    }
		    tResL.pushBack(tAVL);
		}
	    }
	}
	else
	    xOiDebugPrint(tErrPrintMode, 1, mID+": "+sReadErr);

	xOiDebugFinishFunc(tResL);
	return(tResL);
    }

    
    
    
    
    private func filterIdcs2ReturnIdcs(pIdcs)
    {
	var tList = @();
	xOiCopyAggr(pIdcs, tList, 0);
	tList.sort();
	var tRes = Vector(0);
	var tFilterIdx;
	foreach(tFilterIdx; pIdcs) {
	    var i;
	    for(i=0; i<tList.size(); i++)
		if (tList[i] == tFilterIdx) {
		    tRes.pushBack(i);
		    break;
		}
	}
	return(tRes);
    }

    
    
    
    
    
    
    
    public func insertEntry(pAttr)
    {
	xOiDebugStartFunc("xOiTable2::insertEntry", pAttr);

	if (pAttr == NULL  || pAttr.empty()) {
	    xOiDebugFinishFunc(" (invalid arg!)");
	    return;
	}

	if (!mIsOpen) {
	    xOiDebugPrint(getErrPrintMode(), 1, sNotOpenErr);
	    xOiDebugFinishFunc(" (table not open!)");
	    return;
	}

	var tTAL = @();
	var tA, tTA;
	foreach(tA; pAttr) {
	    if (tA[1] == NULL) {
	        xOiDebugPrint(getErrPrintMode(), 1, 
			      ["void value for attribute ", tA[0]]);
	        xOiDebugFinishFunc(" (void attribute value!)");
		return;
	    }
	    xOiDebugPrint(@Info, 2, ["attribute ", tA[0], ": ", tA[1]]);
	    tTA = getAttrDef(tA[0]);
	    if (tTA == NULL) {
	        xOiDebugFinishFunc(" (unknown attribute!)");
		return;
	    }
	    tTA[4] = tA[1];
	    tTAL.pushBack(tTA);
	}

	var tRes  = oiTable(@insertTE, @([mID, tTAL]));

	

	xOiDebugFinishFunc(" (ok)");
	return;
    }

    
    
    
    
    public func deleteEntries(pAttr)
    {
	xOiDebugStartFunc("xOiTable2::deleteEntries", pAttr);

	if (pAttr == NULL  || pAttr.empty()) {
	    xOiDebugFinishFunc(" (invalid arg!)");
	    return;
	}

	if (!mIsOpen) {
	    xOiDebugPrint(getErrPrintMode(), 1, sNotOpenErr);
	    xOiDebugFinishFunc(" (table not open!)");
	    return;
	}

	var tTAL = @();
	var tA, tTA;
	foreach(tA; pAttr) {
	    if (tA[1] == NULL) {
	        xOiDebugPrint(getErrPrintMode(), 1, 
			      ["void value for attribute ", tA[0]]);
	        xOiDebugFinishFunc(" (void attribute value!)");
		return;
	    }
	    xOiDebugPrint(@Info, 2, ["attribute ", tA[0], ": ", tA[1]]);
	    tTA = getAttrDef(tA[0]);
	    if (tTA == NULL) {
	        xOiDebugFinishFunc(" (unknown attribute!)");
		return;
	    }
	    tTA[4] = tA[1];
	    tTAL.pushBack(tTA);
	}

	var tRes  = oiTable(@deleteTE, @([mID, tTAL]));

	

	xOiDebugFinishFunc(" (ok)");
	return;
    }
}	


