// === ofml/xoi/xoidebug.cls ===
package ::ofml::xoi;

import ::ofml::oi::*;

import ::stdio::*;












public class xOiDebug
{
    static var sEnabled = 0;

    static var sXOIVersion = "1.64.0";
    static var sVersionPrinted = 0;
    static var sOutFile = "debug.out";

    
    static var sMaxLineSize = 100000;

    
    var mPlanning;
    var mIsTestPlanning;
    var mModes;
    var mTraceLevel;
    var mDetailLevel;
    var mDevice;
    var mEvalErrDevice;
    var mDebugClasses;
    var mFilePath;

    var mFTraceIndent;
    var mFInfoIndent;

    
    var mFuncLevel;
    var mFuncNames;

    
    var mSessionOn;

    var mOutStream;
    var mModeMap;

    func initialize()
    {
	mPlanning = NULL;
	mIsTestPlanning = 0;
	mModes = NULL;
	mTraceLevel = 0;
	mDetailLevel = 1;
	mDevice = "stderr";
	mEvalErrDevice = NULL;
	mDebugClasses = NULL;
	mFilePath = NULL;

	mOutStream = stderr;

	mFuncLevel = 0;
	mFuncNames  = Vector(0).pushBack(["", @Func, 0]);

	mSessionOn = 0;

	mModeMap = Hash();
	mModeMap[@Warn]  = "W: ";
	mModeMap[@Info]  = "I: ";
	mModeMap[@Time2] = "T: ";
    }

    func getVersion()
    {
	return(sXOIVersion);
    }

    
    func setPlanning(pPlanning)
    {
	mPlanning = pPlanning;

	if (mPlanning != NULL && mPlanning.isCat(@TestPlanning) &&
	    mPlanning.hasMember(@receiveDebugMsg))
	    mIsTestPlanning = 1;
	else
	    mIsTestPlanning = 0;
    }

    private func setupDevice(pDebugInfo)
    {
	mDevice = pDebugInfo[3];

	mFilePath = NULL;
	if (pDebugInfo.size() > 4 && pDebugInfo[4] instanceof String)
	    mFilePath = pDebugInfo[4];

	
	

	if (mDevice == NULL || mDevice == "") mDevice = "stderr";

	if (mDevice == "file") {
	    var tFile = (mFilePath == NULL ? "" : mFilePath+"/") + sOutFile;
	    try {
		mOutStream = OFStream(tFile, "a");
	    } catch (&e: Error) {
		mDevice = "stderr";
	    }
	}
	if (mDevice == "stderr")
	    mOutStream = stderr;
    }

    private func printSessionHeader()
    {
	output("Debug session started (XOI version "+sXOIVersion+")");
	output("-- debug modi: " + xOiParam2Str(mModes));
	output("-- trace level: " + xOiParam2Str(mTraceLevel));
	output("-- detail level: " + xOiParam2Str(mDetailLevel));
    }

    
    private func startSession(pDebugInfo)
    {
	if (mSessionOn && mFuncLevel > 0) 
	    
	    return(0);

	mSessionOn = 1;
	sEnabled = 1;
	if (mPlanning != NULL && mPlanning.hasMember(@enableDebugging))
	    mPlanning.enableDebugging(1);

	mModes = pDebugInfo[0];
	mTraceLevel = pDebugInfo[1];
	mDetailLevel = pDebugInfo[2];

	mFTraceIndent = 1;
	mFInfoIndent = 1; 

	if (pDebugInfo.size() > 5 && pDebugInfo[5] instanceof Int)
	    mFTraceIndent = pDebugInfo[5];
	if (pDebugInfo.size() > 6 && pDebugInfo[6] instanceof Int)
	    mFInfoIndent = pDebugInfo[6];

	setupDevice(pDebugInfo);

	mDebugClasses = NULL;

	printSessionHeader();

	return(1);
    }

    private func endSession()
    {
	if (mDevice == "file") mOutStream.close();

	mSessionOn = 0;
	sEnabled = 0;
	if (mPlanning != NULL && mPlanning.hasMember(@enableDebugging))
	    mPlanning.enableDebugging(0);
    }

    func setDebugClasses(pArg)
    {
	mDebugClasses = pArg;
	output("-- debug classes:  " + xOiParam2Str(mDebugClasses));
    }

    func setEvalErrDevice(pDevice)
    {
	mEvalErrDevice = pDevice;
    }

    private func argString(pArgs, pMode)
    {
	var tArgs = pArgs;

	if (pMode == @Finish) return(xOiParam2Str(pArgs));

	if (!(tArgs instanceof Vector)) tArgs = [pArgs];

	var tRes = "";
	var i = 0, size = tArgs.size();
	for(i; i<size; i++) {
	    if (i && pMode == @Start) 
		tRes = tRes + ", ";
	    var tAStr = tArgs[i];
	    if (pMode == @Start || !(tArgs[i] instanceof String)) 
	        tAStr = xOiParam2Str(tArgs[i]);
	    tRes = tRes + tAStr;
	}
	return(tRes);
    }

    private func output(pMsg, ...)
    {
	var tMsg = pMsg[0];
	if (sMaxLineSize != NULL && pMsg[0].size() > sMaxLineSize) {
	    tMsg = "";
	    tMsg.insert(0, pMsg[0], 0, sMaxLineSize-4);
	    tMsg += "...";
	}

	var tDevice = mDevice;
	if (pMsg.size() > 1 && pMsg[1] != NULL)
	    tDevice = pMsg[1];

	if (tDevice == "dialog") 
	    oiOutput(@MESSAGE, tMsg);
	else {
	    mOutStream.printf("%s\n", tMsg);
	    mOutStream.flush();
	}
    }

    
    
    private func checkClass(pFuncName)
    {
	if (mDebugClasses == NULL) {
	    mDebugClasses = mPlanning.getDebugClasses();
	    output("-- debug classes:  " + xOiParam2Str(mDebugClasses));
	}

	var tPos = pFuncName.find("::", 0);
	if (tPos < 0)
	    
	    return(1);

	var tClass = pFuncName.substr(0, tPos);
	var tCL = mDebugClasses[0];
	var tExcl = mDebugClasses[1];
	var tRet;

	if (tExcl)
	    tRet = (tCL.find(tClass) < 0);
	else
	    tRet = (tCL.find(tClass) >= 0);

	return(tRet);
    }

    private func startFunc(pName, pArgs, pMode)
    {
	if (mTraceLevel == 0) return;

	
	

	mFuncNames.pushBack([pName, pMode, 
			     mModes.find(@Time) < 0 ? NULL : afGetTime()]);

	var tIsDebugClass = checkClass(pName);

	if (!tIsDebugClass) return;

	mFuncLevel++;

	if (mFuncLevel > mTraceLevel || mModes.find(pMode) < 0) return;

	var tMsg = String(mFuncLevel).insert(0, (mFuncLevel-1)*mFTraceIndent) +
		   "> " + pName + "(" + argString(pArgs, @Start) + ")";

	output(tMsg);

	if (mModes.find(@Time2) >= 0) {
	    tMsg = String(::time::ctime(::time::time()));
	    var tLastPos = tMsg.size()-1;
	    if (tMsg[tLastPos] == '\n') 
		tMsg.remove(tLastPos);
	    tMsg.insert(0, (mFuncLevel-1)*mFTraceIndent);
	    output(tMsg);
	}
    }

    private func finishFunc(pArg)
    {
	if (mTraceLevel == 0) return;

	if (mFuncNames.empty()) {
	    
	    
	    mFuncLevel = 0;
	    return;
	}

	var tFuncV = mFuncNames.popBack();
	var tFunc  = tFuncV[0];
	var tMode  = tFuncV[1];
	var tTime  = tFuncV[2];

	var tIsDebugFunc = checkClass(tFunc);

	if (tIsDebugFunc && mFuncLevel <= mTraceLevel && 
	    mModes.find(tMode) >= 0) {
	    if (mModes.find(@Time2) >= 0) {
		var tMsg = String(::time::ctime(::time::time()));
		var tLastPos = tMsg.size()-1;
		if (tMsg[tLastPos] == '\n')
		    tMsg.remove(tLastPos);
		tMsg.insert(0, (mFuncLevel-1)*mFTraceIndent);
		output(tMsg);
	    }
	    var tTimeStr = "";
	    if (mModes.find(@Time) >= 0)
		tTimeStr = timeUsed(tTime);
	    output(String(mFuncLevel).insert(0, (mFuncLevel-1)*mFTraceIndent) +
		   "< " + tFunc + ": " + argString(pArg, @Finish) + tTimeStr);
	}

	if (tIsDebugFunc) mFuncLevel--;
    }

    private func timeUsed(pTime1)
    {
	
	
	

	var tSecs = String(xOiGetRoundFloat(afGetTime() - pTime1, 6));
	if (tSecs == "0")
	    tSecs = "0.000000";

	return(" {seconds used: " + tSecs + "}");
    }

    private func printEvalErr(pArg)
    {
	output(argString(pArg, @Msg), mEvalErrDevice);
    }

    private func printSpecial(pArg)
    {
	output(argString(pArg, @Msg));
    }

    private func printInfoFloat(pArg)
    {
	if (!(mDevice != "dialog" && pArg instanceof Vector &&
	      pArg.size() == 2 && 
	      pArg[0] instanceof String && pArg[1] instanceof Float))
	return;

	mOutStream.printf("Float: %s = %s (%.100g)\n", 
			  pArg[0], String(pArg[1]), pArg[1]);
	mOutStream.flush();
    }

    private func print(pMode, pLevel, pArg)
    {
	if (mIsTestPlanning && (mModes == NULL || mModes.empty())) {
	    mPlanning.receiveDebugMsg(pMode, pLevel, argString(pArg, @Msg));
	    return;
	}

	var tSpecialModes = @(@EvalErr, @InfoF, @TableErr, @ExplWarn, 
			      @Collision, @Semantics, @Progress);
	if (tSpecialModes.find(pMode) >= 0) {
	    if (pMode == @EvalErr )
		printEvalErr(pArg);
	    else 
	    if (pMode == @InfoF)
		printInfoFloat(pArg);
	    else 
	    if (mModes.find(pMode) >= 0)
		printSpecial(pArg);
	    return;
	}

	if (mTraceLevel == 0 || mFuncNames.empty()) return;

	var tFunc = mFuncNames.back();

	

	if (mFuncLevel > mTraceLevel ||
	    !(pMode == @Warn || pMode == @Info) || 
	    mModes.find(pMode) < 0 ||
	    pLevel > mDetailLevel ||
	    !checkClass(tFunc[0]) || mModes.find(tFunc[1]) < 0) return;

	var tMsg = mModeMap[pMode];
	if (mModes.find(tFunc[1]) < 0) 
	    tMsg = tMsg + "{" + mFuncNames[mFuncLevel][0] + "} ";
	tMsg = tMsg + argString(pArg, @Msg);

	output(tMsg.insert(0, (mFuncLevel-1)*mFTraceIndent + mFInfoIndent));
    }
}

var gxOiDebug = xOiDebug();

public func xOiDebugStartFunc(pName, pArgs)
{
    if (xOiDebug.sEnabled)
    gxOiDebug.startFunc(pName, pArgs, @Func);
}

public func xOiDebugStartFunc2(pName, pArgs)
{
    if (xOiDebug.sEnabled)
    gxOiDebug.startFunc(pName, pArgs, @Func2);
}

public func xOiDebugStartFunc3(pName, pArgs)
{
    if (xOiDebug.sEnabled)
    gxOiDebug.startFunc(pName, pArgs, @Func3);
}

public func xOiDebugFinishFunc(pArg)
{
    if (xOiDebug.sEnabled)
    gxOiDebug.finishFunc(pArg);
}

public func xOiDebugPrint(pMode, pLevel, pArg)
{
    if (xOiDebug.sEnabled)
    gxOiDebug.print(pMode, pLevel, pArg);
}


