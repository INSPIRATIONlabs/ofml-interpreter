// === ofml/xoi/xoijointplgroup.cls ===
package ::ofml::xoi;
import ::ofml::oi::*;



























































































































































































































































































































































































































































public class xOiJointPlGroup: xOiLRPlGroup
{
    var mXoiArticleSpec;

    
    var mXoiSelectState4Layout;
    var mXoiSelectState4Other;
    var mXoiCutState4Layout;
    var mXoiCutState4Other;
    var mXoiIgnPlanDir4Rm;
    var mXoiShowAsArticle;
    var mXoiIsPseudoArticle;
    var mXoiIsNonOrderArticle;
    var mXoiIsNonOfferArticle;
    var mXoiStartLayoutAtRecreation;
    var mXoiCommonProps;
    var mXoiFirstPos4CommonProps;
    var mXoiCommonPropsPos;
    var mXoiAllObjs4CommonProps;
    var mXoiNonLayout4CommonProps;
    var mXoiCommonPropsChLMode;
    var mXoiSortUnionCommonProps;
    var mXoiClasses4CommonProps;
    var mXoiMeta4CommonProps; 
    var mXoiCommonPropsDepth;
    var mXoiNonVisibleProps4Common;
    var mXoiROPropsEditable4Common;
    var mXoiArticleFeatures4Common;
    var mXoiAllElmnts4SubArt;
    var mXoiApplyOCDProps;

    
    var mXoiCategories;

    static var sXoiDuringSetAddStateCode = 0;
    static var sXoiAllowRemoval = NULL;

    public func initialize(pFa, pNa)
    {
	xOiDebugStartFunc("xOiJointPlGroup::initialize", [pFa, pNa]);

	xOiDebugPrint(@Info, 1, ["self ", self]);

	xOiLRPlGroup::initialize(pFa, pNa); 

	mXoiSelectState4Layout = 1;
	mXoiSelectState4Other = 1;
	mXoiCutState4Layout = 1;
	mXoiCutState4Other = 1;
	mXoiIgnPlanDir4Rm = 0;
	mXoiShowAsArticle = 0; 
	mXoiIsPseudoArticle = 0;
	mXoiIsNonOrderArticle = 0;
	mXoiIsNonOfferArticle = 0;
	mXoiStartLayoutAtRecreation = 1;
	mXoiCommonProps = @();
	mXoiFirstPos4CommonProps = 1;
	mXoiCommonPropsPos = Vector(0);
	mXoiAllObjs4CommonProps = 0;
	mXoiNonLayout4CommonProps = 0;
	mXoiCommonPropsChLMode = @FirstEl;
	mXoiSortUnionCommonProps = 0;
	mXoiClasses4CommonProps = 0;
	mXoiMeta4CommonProps = 0;
	mXoiCommonPropsDepth = 0;
	mXoiNonVisibleProps4Common = 0;
	mXoiROPropsEditable4Common = 0;
	mXoiArticleFeatures4Common = 0;
	mXoiAllElmnts4SubArt = 0;
	mXoiApplyOCDProps = [@None];

	
	
	
	
	
	
	
	
	readDataTable(getProgram());

	mXoiCategories = Hash();

	xOiDebugFinishFunc(NULL);
    }

    public func manualSetMembers(pMembers)
    {
	var tRes = xOiLRPlGroup::manualSetMembers(pMembers);

	if (!pMembers.hasKey(@mXoiSelectState4Layout))
	    tRes[1].pushBack(@mXoiSelectState4Layout);
	if (!pMembers.hasKey(@mXoiSelectState4Other))
	    tRes[1].pushBack(@mXoiSelectState4Other);
	if (!pMembers.hasKey(@mXoiCutState4Layout))
	    tRes[1].pushBack(@mXoiCutState4Layout);
	if (!pMembers.hasKey(@mXoiCutState4Other))
	    tRes[1].pushBack(@mXoiCutState4Other);
	if (!pMembers.hasKey(@mXoiIgnPlanDir4Rm))
	    tRes[1].pushBack(@mXoiIgnPlanDir4Rm);
	if (!pMembers.hasKey(@mXoiCommonProps))
	    tRes[1].pushBack(@mXoiCommonProps);
	if (!pMembers.hasKey(@mXoiAllElmnts4SubArt))
	    tRes[1].pushBack(@mXoiAllElmnts4SubArt);
	if (!pMembers.hasKey(@mXoiAllObjs4CommonProps))
	    tRes[1].pushBack(@mXoiAllObjs4CommonProps);
	if (!pMembers.hasKey(@mXoiNonLayout4CommonProps))
	    tRes[1].pushBack(@mXoiNonLayout4CommonProps);
	if (!pMembers.hasKey(@mXoiClasses4CommonProps))
	    tRes[1].pushBack(@mXoiClasses4CommonProps);
	if (!pMembers.hasKey(@mXoiMeta4CommonProps))
	    tRes[1].pushBack(@mXoiMeta4CommonProps);
	if (!pMembers.hasKey(@mXoiCommonPropsDepth))
	    tRes[1].pushBack(@mXoiCommonPropsDepth);
	if (!pMembers.hasKey(@mXoiNonVisibleProps4Common))
	    tRes[1].pushBack(@mXoiNonVisibleProps4Common);
	if (!pMembers.hasKey(@mXoiROPropsEditable4Common))
	    tRes[1].pushBack(@mXoiROPropsEditable4Common);
	if (!pMembers.hasKey(@mXoiShowAsArticle))
	    tRes[1].pushBack(@mXoiShowAsArticle);
	if (!pMembers.hasKey(@mXoiIsPseudoArticle))
	    tRes[1].pushBack(@mXoiIsPseudoArticle);
	if (!pMembers.hasKey(@mXoiIsNonOrderArticle))
	    tRes[1].pushBack(@mXoiIsNonOrderArticle);
	if (!pMembers.hasKey(@mXoiIsNonOfferArticle))
	    tRes[1].pushBack(@mXoiIsNonOfferArticle);
	if (!pMembers.hasKey(@mXoiArticleFeatures4Common))
	    tRes[1].pushBack(@mXoiArticleFeatures4Common);
	if (!pMembers.hasKey(@mXoiFirstPos4CommonProps))
	    tRes[1].pushBack(@mXoiFirstPos4CommonProps);
	if (!pMembers.hasKey(@mXoiCategories))
	    tRes[1].pushBack(@mXoiCategories);
	if (!pMembers.hasKey(@mXoiCommonPropsPos))
	    tRes[1].pushBack(@mXoiCommonPropsPos);
	if (!pMembers.hasKey(@mXoiCommonPropsChLMode))
	    tRes[1].pushBack(@mXoiCommonPropsChLMode);
	if (!pMembers.hasKey(@mXoiSortUnionCommonProps))
	    tRes[1].pushBack(@mXoiSortUnionCommonProps);
	if (!pMembers.hasKey(@mXoiStartLayoutAtRecreation))
	    tRes[1].pushBack(@mXoiStartLayoutAtRecreation);
 	if (!pMembers.hasKey(@mXoiApplyOCDProps))
	    tRes[1].pushBack(@mXoiApplyOCDProps);

	return(tRes);
    }

    rule FINISH_EVAL(pArg)
    {
	if (mXoiSelectState4Layout == NULL)
	    mXoiSelectState4Layout = 1;
	if (mXoiSelectState4Other == NULL)
	    mXoiSelectState4Other = 1;
	if (mXoiCutState4Layout == NULL)
	    mXoiCutState4Layout = 1;
	if (mXoiCutState4Other == NULL)
	    mXoiCutState4Other = 1;
	if (mXoiIgnPlanDir4Rm == NULL)
	    mXoiIgnPlanDir4Rm = 0;
	if (mXoiCommonProps == NULL)
	    mXoiCommonProps = @();
	if (mXoiAllElmnts4SubArt == NULL)
	    mXoiAllElmnts4SubArt = 0;
	if (mXoiAllObjs4CommonProps == NULL)
	    mXoiAllObjs4CommonProps = 0;
	if (mXoiNonLayout4CommonProps == NULL)
	    mXoiNonLayout4CommonProps = 0;
	if (mXoiCommonPropsPos == NULL)
	    mXoiCommonPropsPos = Vector(0);
	if (mXoiCommonPropsChLMode == NULL)
	    mXoiCommonPropsChLMode = @FirstEl;
	if (mXoiSortUnionCommonProps == NULL)
	    mXoiSortUnionCommonProps = 0;
	if (mXoiClasses4CommonProps == NULL)
	    mXoiClasses4CommonProps = 0;
	if (mXoiMeta4CommonProps == NULL)
	    mXoiMeta4CommonProps = 0;
	if (mXoiNonVisibleProps4Common == NULL)
	    mXoiNonVisibleProps4Common = 0;
	if (mXoiROPropsEditable4Common == NULL)
	    mXoiROPropsEditable4Common = 0;
	if (mXoiShowAsArticle == NULL)
	    mXoiShowAsArticle = 0;
	if (mXoiIsPseudoArticle == NULL)
	    mXoiIsPseudoArticle = 0;
	if (mXoiIsNonOrderArticle == NULL)
	    mXoiIsNonOrderArticle = 0;
	if (mXoiIsNonOfferArticle == NULL)
	    mXoiIsNonOfferArticle = 0;
	if (mXoiArticleFeatures4Common == NULL)
	    mXoiArticleFeatures4Common = 0;
	if (mXoiFirstPos4CommonProps == NULL)
	    mXoiFirstPos4CommonProps = 0;
	if (mXoiCategories == NULL)
	    mXoiCategories = Hash();
	if (mXoiStartLayoutAtRecreation == NULL)
	    mXoiStartLayoutAtRecreation = 1;
	if (mXoiApplyOCDProps == NULL)
	    mXoiApplyOCDProps = [@None];

	return(0);
    }

    private func readDataTable(pPID)
    {
	xOiDebugStartFunc2("xOiJointPlGroup::readDataTable", pPID);

	var tTbl = xOiOpenDataTbl("jointplgroup", pPID,
				  xOiGetAppProgInfoDBPath(pPID));
	if (tTbl == NULL) {
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tValue;

	tValue = xOiGetMatchingEntryValue(tTbl, "@_2DMode", 
					  mXoiArticleSpec, Symbol,
					  @(@Elements, @Group, @Combined, 
					    @Rectangle));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@_2DMode");
	else {
	    mXoi2DMode = tValue;
	    xOiDebugPrint(@Info, 1, ["2D mode: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@InsertMode", 
					  mXoiArticleSpec, Int, @(0, 1, 2));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@InsertMode");
	else {
	    setInsertMode(tValue);
	    xOiDebugPrint(@Info, 1, ["insert mode: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@SelectableState4Layout", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@SelectableState4Layout");
	else {
	    mXoiSelectState4Layout = tValue;
	    xOiDebugPrint(@Info, 1, ["SelectableState4Layout: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@SelectableState4Other", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@SelectableState4Other");
	else {
	    mXoiSelectState4Other = tValue;
	    xOiDebugPrint(@Info, 1, ["SelectableState4Other: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@CutableState4Layout", 
					  mXoiArticleSpec, Int, @(-1, 0, 1, 2));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@CutableState4Layout");
	else {
	    mXoiCutState4Layout = tValue;
	    xOiDebugPrint(@Info, 1, ["CutableState4Layout: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@CutableState4Other", 
					  mXoiArticleSpec, Int, @(-1, 0, 1, 2));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@CutableState4Other");
	else {
	    mXoiCutState4Other = tValue;
	    xOiDebugPrint(@Info, 1, ["CutableState4Other: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@IgnorePlanDir4Remove", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@IgnorePlanDir4Remove");
	else {
	    mXoiIgnPlanDir4Rm = tValue;
	    xOiDebugPrint(@Info, 1, ["IgnorePlanDir4Remove: ", tValue]);
	}

	if (mXoiArticleSpec == NULL) {
	    
	    xOiCloseDataTbl(tTbl);
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@ShowAsArticle", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@ShowAsArticle");
	else {
	    mXoiShowAsArticle = tValue;
	    xOiDebugPrint(@Info, 1, ["ShowAsArticle: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@IsPseudoArticle", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL) {
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@IsPseudoArticle");
	    if (!mXoiShowAsArticle)
		mXoiIsPseudoArticle = 1;
	    xOiDebugPrint(@Info, 1, ["default for @IsPseudoArticle: ", 
				     mXoiIsPseudoArticle]);
	} else {
	    mXoiIsPseudoArticle = tValue;
	    xOiDebugPrint(@Info, 1, ["IsPseudoArticle: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@IsNonOrderArticle", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@IsNonOrderArticle");
	else {
	    mXoiIsNonOrderArticle = tValue;
	    xOiDebugPrint(@Info, 1, ["IsNonOrderArticle: ", tValue]);
	}

	if (!mXoiIsPseudoArticle && mXoiIsNonOrderArticle) {
	    xOiDebugPrint(@Warn, 1, 
			  "rejecting category @NonOrderArticle " +
			  "due to value 0 for option @IsPseudoArticle!");
	    mXoiIsNonOrderArticle = 0;
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@IsNonOfferArticle", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@IsNonOfferArticle");
	else {
	    mXoiIsNonOfferArticle = tValue;
	    xOiDebugPrint(@Info, 1, ["IsNonOfferArticle: ", tValue]);
	}

	if (!mXoiIsPseudoArticle && mXoiIsNonOfferArticle) {
	    xOiDebugPrint(@Warn, 1, 
			  "rejecting category @NonOfferArticle " +
			  "due to value 0 for option @IsPseudoArticle!");
	    mXoiIsNonOfferArticle = 0;
	}

	
	
	
	

	tValue = xOiGetMatchingEntryValue(tTbl, "@AllElements4SubArticle", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@AllElements4SubArticle");
	else {
	    mXoiAllElmnts4SubArt = tValue;
	    xOiDebugPrint(@Info, 1, ["AllElements4SubArticle: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@ApplyOCDProps", 
					  mXoiArticleSpec, NULL, NULL);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry for @ApplyOCDProps");
	else {
	    var tValidSlots = @(@None, @SetBaseSpec, @SetOtherSpec);
	    var tErr = NULL;
	    var tVal2 = NULL;
	    try {
		tVal2 = eval("["+tValue+"]");
	    }
	    catch(&e: Error) {
		tErr = e.what;
	    }
	    if (tVal2 instanceof Vector && tValidSlots.find(tVal2[0]) >= 0 &&
		(tVal2.size() == 1 || 
		 (tVal2[1] instanceof Int && tVal2[1] >= 0))) {
		mXoiApplyOCDProps = tVal2;
		xOiDebugPrint(@Info, 1, 
			      ["@ApplyOCDProps: ", mXoiApplyOCDProps]);
	    }
	    else
		xOiDebugPrint(@Warn, 1, ["invalid value for @ApplyOCDProps",
					 tErr == NULL ? "" : " ("+tErr+")"]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@StartLayoutAtRecreation", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@StartLayoutAtRecreation");
	else {
	    mXoiStartLayoutAtRecreation = tValue;
	    xOiDebugPrint(@Info, 1, ["StartLayoutAtRecreation: ", tValue]);
	}

	var tNoStartLayoutOption = 1;

	tValue = xOiGetMatchingEntryValue(tTbl, "@StartLayout", 
					  mXoiArticleSpec, Vector, NULL);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@StartLayout");
	else 
	if (!tValue.empty()) {
	    xOiDebugPrint(@Info, 1, ["start layout: ", tValue]);
	    tNoStartLayoutOption = 0;
	    if (isEmpty())
		_createStartLayout(tValue);
	    else
		xOiDebugPrint(@Warn, 1, "there already are layout elements?!");
	}

	if (isEmpty()) {
	    tValue = xOiGetMatchingEntryValue(tTbl, "@StartElement", 
					      mXoiArticleSpec, Vector, NULL);
	    if (tValue == NULL)
		xOiDebugPrint(@Info, 1, 
			      "no (matching) entry or valid value for " +
			      "@StartElement");
	    else 
	    if (tValue.size() >= 2) {
		xOiDebugPrint(@Info, 1, ["start element: ", tValue]);
		tNoStartLayoutOption = 0;
		_createElement(@Start, tValue, NULL);
	    }
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@FirstPos4CommonProps", 
					  mXoiArticleSpec, Int, NULL);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@FirstPos4CommonProps");
	else 
	if (tValue < 0)
	    xOiDebugPrint(@Warn, 1, 
			  ["invalid value for @FirstPos4CommonProps: ", tValue]);
	else {
	    mXoiFirstPos4CommonProps = tValue;
	    xOiDebugPrint(@Info, 1, ["FirstPos4CommonProps: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@CommonPropsPos", 
					  mXoiArticleSpec, Vector, NULL);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@CommonPropsPos");
	else {
	    mXoiCommonPropsPos = tValue;
	    xOiDebugPrint(@Info, 1, ["CommonPropsPos: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@AllObjs4CommonProps", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@AllObjs4CommonProps");
	else {
	    mXoiAllObjs4CommonProps = tValue;
	    xOiDebugPrint(@Info, 1, ["AllObjs4CommonProps: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@NonLayout4CommonProps", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@NonLayout4CommonProps");
	else {
	    mXoiNonLayout4CommonProps = tValue;
	    
	    
	    
	    
	    
	    
	    
	    
	    
	    xOiDebugPrint(@Info, 1, ["NonLayout4CommonProps: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@CommonPropsChLMode", 
					  mXoiArticleSpec, Symbol,
					  @(@FirstEl, @Intersection, @Union), 
					  1);
	if (tValue == NULL) {
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@CommonPropsChLMode");
	    if (mXoiAllObjs4CommonProps)
		mXoiCommonPropsChLMode = @Intersection;
	    else
		mXoiCommonPropsChLMode = @FirstEl;
	    xOiDebugPrint(@Info, 2, ["default CommonPropsChLMode: ", 
				     mXoiCommonPropsChLMode]);
	} else {
	    mXoiCommonPropsChLMode = tValue;
	    xOiDebugPrint(@Info, 1, ["CommonPropsChLMode: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@SortUnionCommonProps", 
					  mXoiArticleSpec, Int, @(0, 1), 1);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@SortUnionCommonProps");
	else {
	    mXoiSortUnionCommonProps = tValue;
	    xOiDebugPrint(@Info, 1, ["SortUnionCommonProps: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@Classes4CommonProps", 
					  mXoiArticleSpec, Int, @(0, 1), 1);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@Classes4CommonProps");
	else {
	    mXoiClasses4CommonProps = tValue;
	    xOiDebugPrint(@Info, 1, ["Classes4CommonProps: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@Meta4CommonProps", 
					  mXoiArticleSpec, Int, @(0, 1));
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@Meta4CommonProps");
	else {
	    mXoiMeta4CommonProps = tValue;
	    xOiDebugPrint(@Info, 1, ["Meta4CommonProps: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@CommonPropsDepth", 
					  mXoiArticleSpec, Int, NULL);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@CommonPropsDepth");
	else 
	if (tValue < 0)
	    xOiDebugPrint(@Warn, 1,
			  ["invalid value for @CommonPropsDepth: ", tValue]);
	else {
	    mXoiCommonPropsDepth = tValue;
	    xOiDebugPrint(@Info, 1, ["CommonPropsDepth: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@NonVisibleProps4Common", 
					  mXoiArticleSpec, Int, @(0, 1), 1);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@NonVisibleProps4Common");
	else {
	    mXoiNonVisibleProps4Common = tValue;
	    xOiDebugPrint(@Info, 1, ["NonVisibleProps4Common: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@ROPropsEditable4Common", 
					  mXoiArticleSpec, Int, @(0, 1), 1);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@ROPropsEditable4Common");
	else {
	    mXoiROPropsEditable4Common = tValue;
	    xOiDebugPrint(@Info, 1, ["ROPropsEditable4Common: ", tValue]);
	}

	tValue = xOiGetMatchingEntryValue(tTbl, "@ArticleFeatures4Common", 
					  mXoiArticleSpec, Int, @(0, 1), 1);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@ArticleFeatures4Common");
	else {
	    mXoiArticleFeatures4Common = tValue;
	    xOiDebugPrint(@Info, 1, ["ArticleFeatures4Common: ", tValue]);
	}

	mXoiCommonProps = @();

	tValue = xOiGetMatchingEntryValue(tTbl, "@CommonProps", 
					  mXoiArticleSpec, Vector, NULL);
	if (tValue == NULL)
	    xOiDebugPrint(@Info, 1, "no (matching) entry or valid value for " +
				    "@CommonProps");
	else {
	    xOiCopyAggr(tValue, mXoiCommonProps, 0);
	    xOiDebugPrint(@Info, 1, ["common props: ", mXoiCommonProps]);
	    if (tNoStartLayoutOption) {
		
		
		
		
		
		
		
		
		
		var tObjs4CommonProps = self.getObjs4OptionCommonProps();
		if (!mXoiCommonProps.empty() && !tObjs4CommonProps.empty()) {
		    xOiDebugPrint(@Info, 1, "generating common properties ...");
		    var tCPH = xOiCommonPropsHandler(self);
		    tCPH.initCommonProps(NULL, 0);
		}
	    } 
	}

	xOiCloseDataTbl(tTbl);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    private func _createStartLayout(pInfo)
    {
	xOiDebugStartFunc2("xOiJointPlGroup::_createStartLayout", NULL);

	var tRet = @();

	var tPlan = oiGetPlanning();

	if (tPlan == NULL) {
	    xOiDebugPrint(@Warn, 1, "no global planning instance!?");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	if (tPlan.getCreationMode() == 1 && !mXoiStartLayoutAtRecreation) {
	    xOiDebugPrint(@Info, 1, "nothing to do during re-creation");
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	var tPredecessor = NULL;
	var tElInfo;
	foreach(tElInfo; pInfo) {
	    if (!(tElInfo instanceof Vector && tElInfo.size() >= 3)) {
		xOiDebugPrint(@Warn, 1, ["wrong article info: ", tElInfo]);
		break;
	    }
	    var tEl = _createLayoutElement(tPlan, tPredecessor, tElInfo);
	    if (tEl == NULL) {
		xOiDebugPrint(@Warn, 1, "couldn't create all start elements!");
		tRet = NULL;
		break;
	    }
	    tRet.pushBack(tEl);
	    tPredecessor = tEl;
	}

	tPlan.setProgram(getProgram());

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    private func _createLayoutElement(pPlan, pPredecessor, pElInfo)
    {
	xOiDebugStartFunc2("xOiJointPlGroup::_createLayoutElement", 
			   [pPredecessor, pElInfo]);

	
	

	var tVCType = @VarCode;
	var tVarCode = "";
	if (pElInfo.size() >= 5) {
	    tVCType  = pElInfo[3];
	    tVarCode = pElInfo[4];
	}

	var tMode = @extended;
	if (pPredecessor == NULL && getFather() == getRoot())
	    tMode = @lite;

	var tNewEl = 
	     xOiCreateArticle(self, pPredecessor, pElInfo[0], pElInfo[1],
			      pElInfo[2], tVarCode, tVCType, tMode);

	if (tNewEl == NULL) {
	    xOiDebugPrint(@Warn, 1, "element could not be created/placed");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	if (pElInfo.size() >= 6 && pElInfo[5] instanceof Vector) {
	    var tP; 
	    foreach(tP; pElInfo[5])
		tNewEl.setPropValue(tP[0], tP[1]);
	}

	if (pElInfo.size() >= 7 && pElInfo[6] instanceof Vector) {
	    var tP; 
	    foreach(tP; pElInfo[6])
		tNewEl.setPropState2(tP[0], tP[1]);
	}

	xOiDebugFinishFunc(tNewEl);
	return(tNewEl);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    private func _createElement(pMode, pElInfo, pElName)
    {
	xOiDebugStartFunc2("xOiJointPlGroup::_createElement", 
			   [pMode, pElInfo, pElName]);

	

	var tPlan = oiGetPlanning();

	if (tPlan == NULL || !tPlan.isA(xOiBasePlanning)) {
	    xOiDebugPrint(@Warn, 1, "no or invalid planning instance!?");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	oiRegisterPackage(pElInfo[0]);

	var tClass = tPlan.article2Class(pElInfo[1], 1);

	if (tClass == NULL) {
	    xOiDebugPrint(@Warn, 1, ["could not determine class for article ", 
				     pElInfo[1]]);
	    tPlan.setProgram(getProgram());
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	xOiDebugPrint(@Info, 2, ["class for new element: ", tClass]);

	var tType;
	try {
	    tType = eval(tClass);
        }
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 1, e.what);
	    tPlan.setProgram(getProgram());
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tObj;
	try {
	    if (pMode == @Replace) {
		tObj = eval(tClass+"(self, @"+pElName+")");
		tObj.beAnElement();
	    }
	    else 
		tObj = self.add(tType);
	}
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 1, "instance creation failed!");
	    tPlan.setProgram(getProgram());
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	xOiDebugPrint(@Info, 2, ["created element: ", tObj]);

	var tVCType = @VarCode;
	var tVarCode = "";
	if (pElInfo.size() >= 4) {
	    tVCType  = pElInfo[2];
	    tVarCode = pElInfo[3];
	}

	tObj.setXArticleSpec(@Base, pElInfo[1]);
	tObj.setXArticleSpec(tVCType, tVarCode);

	if (pElInfo.size() >= 5 && pElInfo[4] instanceof Vector) {
	    var tP; 
	    foreach(tP; pElInfo[4])
		tObj.setPropValue(tP[0], tP[1]);
	}

	if (pMode == @Start)
	    elementCreated(tObj); 
	

	if (pElInfo.size() >= 6 && pElInfo[5] instanceof Vector) {
	    var tP; 
	    foreach(tP; pElInfo[5])
		tObj.setPropState2(tP[0], tP[1]);
	}

	var tMyProgram = self.getProgram();
	xOiDebugPrint(@Info, 2, ["my program: ", tMyProgram]);

	tPlan.setProgram(tMyProgram);

	xOiDebugFinishFunc(tObj);
	return(tObj);
    }

    public func isCat(pCat)
    {
	if (pCat == @IF_CompositeArticle) return(1);

	
	
	
	if (pCat == @ORDER_GROUP) 
	    return(mXoiArticleSpec == NULL || !mXoiShowAsArticle);

	if (pCat == @PseudoArticle)
	    
	    return(mXoiIsPseudoArticle);

	if (pCat == @NonOrderArticle)
	    return(mXoiIsPseudoArticle && mXoiIsNonOrderArticle);

	if (pCat == @NonOfferArticle)
	    return(mXoiIsPseudoArticle && mXoiIsNonOfferArticle);

	return(xOiLRPlGroup::isCat(pCat));
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setArticleSpec(pArticle)
    {
	xOiDebugStartFunc("xOiJointPlGroup::setArticleSpec", pArticle);

	xOiLRPlGroup::setArticleSpec(pArticle);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if (!(pArticle instanceof String) || pArticle.empty()) {
	    xOiDebugPrint(@Warn, 1, "rejecting invalid paramter!");
	    mXoiArticleSpec = NULL; 
	    mXoiIsPseudoArticle = 0;
	    mXoiIsNonOrderArticle = 0;
	    mXoiIsNonOfferArticle = 0;
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	mXoiArticleSpec = pArticle; 
	mXoiShowAsArticle = 0;
	mXoiIsPseudoArticle = 0;

	var tPlan = oiGetPlanning();
	var tAppMode = tPlan.getCreationMode();
	xOiDebugPrint(@Info, 1, ["application creation mode: ", tAppMode]);

	self.readDataTable(getProgram());
	    
	    
	    
	    

	var tOCDPropsSlot = mXoiApplyOCDProps[0];

	if (tOCDPropsSlot == @SetBaseSpec) {
	    var tPDM = self.getPDManager();
	    if (tPDM == NULL || !tPDM.isA(xOiPDManager))
		
		xOiDebugPrint(@Warn, 1, "no PD manager!?");
	    else {
		if (mXoiApplyOCDProps.size() > 1 && mXoiApplyOCDProps[1] > 0)
		    self.setExtPropOffset(mXoiApplyOCDProps[1]);
		tPDM.setupProps(self, pArticle);
	    }
	}

	var tObjs4CommonProps = self.getObjs4OptionCommonProps();

	if (tObjs4CommonProps.empty()) {
	    xOiDebugPrint(@Info, 1, "no initial elements");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	if (tAppMode == 0 || mXoiStartLayoutAtRecreation) {
	    
	    
	    
	    xOiDebugPrint(@Info, 1, "generating common properties ...");
	    var tCPH = xOiCommonPropsHandler(self);
	    tCPH.initCommonProps(NULL, 0);
	}

	xOiDebugFinishFunc(NULL);
    }

    
    public func getArticleSpec()
    {
	return(mXoiArticleSpec);
    }

    
    
    
    
    
    
    public func setXArticleSpec(pType, pSpec)
    { 
	xOiDebugStartFunc("xOiJointPlGroup::setXArticleSpec", 
			  [pType, pSpec]);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if (pType == @Base) {
	    setArticleSpec(pSpec);
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	xOiLRPlGroup::setXArticleSpec(pType, pSpec);

	var tOCDPropsSlot = mXoiApplyOCDProps[0];

	if (tOCDPropsSlot == @SetOtherSpec) {
	    var tPDM = self.getPDManager();
	    if (tPDM == NULL || !tPDM.isA(xOiPDManager))
		
		xOiDebugPrint(@Warn, 1, "no PD manager!?");
	    else {
		if (mXoiApplyOCDProps.size() > 1 && mXoiApplyOCDProps[1] > 0)
		    self.setExtPropOffset(mXoiApplyOCDProps[1]);
		if (pSpec.empty())
		    tPDM.setupProps(self, mXoiArticleSpec);
		else
		    tPDM.setupProps(self, mXoiArticleSpec, pSpec, pType);
	    }
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticleFeatures(pLanguage)
    {
	xOiDebugStartFunc("xOiJointPlGroup::getArticleFeatures", pLanguage);

	var tInh = xOiLRPlGroup::getArticleFeatures(pLanguage);

	xOiDebugPrint(@Info, 2, ["inherited: ", tInh]);

	if (pLanguage == NULL) {
	    xOiDebugFinishFunc(tInh);
	    return(tInh);
	}

	if (isEmpty()) {
	    xOiDebugPrint(@Warn, 1, "no layout elements?!");
	    xOiDebugFinishFunc(tInh);
	    return(tInh);
	}

	var tArticleFeatures4Common = mXoiArticleFeatures4Common;
	if (mXoiArticleFeatures4Common instanceof Vector) {
	    tArticleFeatures4Common = 1;
	    if (mXoiArticleFeatures4Common.size() == 1) {
		var tEntry = mXoiArticleFeatures4Common[0];
		var tValue = tEntry[0];
		var tConds = tEntry[1];
		if (tConds.empty() && !tValue)
		    tArticleFeatures4Common = 0;
	    }
	}

	if (!tArticleFeatures4Common) {
	    xOiDebugPrint(@Info, 1, 
			  "common props 4 article features not enabled");
	    xOiDebugFinishFunc(tInh);
	    return(tInh);
	}

	var tRet = @();
	if (tInh != NULL)
	    xOiCopyAggr(tInh, tRet, 1);

	
	xOiCopyAggr(getArticleFeatures4El(firstObj(), pLanguage),
		    tRet, 1);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getAddStateCode(pDomain)
    {
	xOiDebugStartFunc("xOiJointPlGroup::getAddStateCode", pDomain);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	var tRes = "";

	if (pDomain == NULL || pDomain != @xOiJointPlGroup)
	    tRes = xOiLRPlGroup::getAddStateCode(pDomain);

	if (pDomain != NULL && pDomain != @xOiJointPlGroup) {
	    xOiDebugFinishFunc(tRes);
	    return(tRes);
	 }

	var tSelfLen = self.getName().size();
	var tElements = @(); 

	var tTOrder = getElOrder();
	var tEL = xOiCopyAggr(tTOrder, NULL, 1);

	if (mXoiAllElmnts4SubArt) {
	    var tE;
	    foreach(tE; getElements()) {
		if (tTOrder.find(tE) >= 0)
		    continue;
		var tEl = tE; 
		if (!tEl.isCat(@IF_Article) ||
		    tEl.getArticleObj().getArticleSpec() == NULL)
		    xOiDebugPrint(@Warn, 1, ["ignoring non-article ", tEl]);
		else
		    tEL.pushBack(tEl);
	    }
	}

	xOiDebugPrint(@Info, 2, ["elements to be encoded: ", tEL]);

	var tEl;
	foreach(tEl; tEL) {
	    if (!tEl.isCat(@IF_Article)) {
		
		xOiDebugPrint(@Warn, 1, ["ignoring non-article ", tEl]);
		continue;
	    }

	    var tSubID   = tEl.getName().substr(tSelfLen+1);
	    var tPID     = tEl.getProgram();
	    var tArticle = tEl.getArticleObj().getArticleSpec();
	    var tPos     = tEl.getPosition();
	    var tRot     = tEl.getRotation(@PY);

	    tElements.pushBack([tSubID, tPID, tArticle, tPos, tRot]);
	}

	var tAddMembers = getAddStateMembers();
	var tAddMembersCode = [];
	if (!tAddMembers.empty())
	    tAddMembersCode = eval(xOiGetAddStateCode(self, @Dummy, NULL, 
						      @Dummy, tAddMembers, 0));

	
	
	
	
	
	

	
	
	
	
	
	var tAddState = [@V5, tElements, mXoiIgnPlanDir4Rm, mXoiCommonProps, 
			 mXoiAllElmnts4SubArt, tAddMembersCode,
			 xOiHash2List(mXoiCategories)];

	
	
	
	
        
	
	

	
	
	
	
	
	
	

	var tMyCode = xOiParam2Str(tAddState, 0, 1);

	if (pDomain == @xOiJointPlGroup)
	    tRes = tMyCode;
	else
	if (pDomain == NULL && !tMyCode.empty())
	    tRes.pushBack([@xOiJointPlGroup, tMyCode]);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    
    protected func getAddStateMembers()
    {
	return([]);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setAddStateCode(pDomain, pCode)
    {
	xOiDebugStartFunc("xOiJointPlGroup::setAddStateCode", [pDomain, pCode]);

	if (pDomain == NULL && !(pCode instanceof Vector)) {
	    xOiDebugPrint(@Warn, 1, "invalid code!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tMyCode, tBaseCode = Vector(0);

	xOiDebugPrint(@Info, 1, ["self: ", self]);

	if (pDomain == NULL) {
	    var tSection;
	    foreach(tSection; pCode) {
		var tDomain = tSection[0];
		var tCode   = tSection[1];
		if (tDomain == @xOiJointPlGroup)
		    tMyCode = tCode;
		else
		    tBaseCode.pushBack([tDomain, tCode]);
	    }
	}
	else {
	    if (pDomain != @xOiJointPlGroup)
		tBaseCode = pCode;
	    else
		tMyCode = pCode;
	}

	if (pDomain == NULL || pDomain != @xOiJointPlGroup)
	    xOiLRPlGroup::setAddStateCode(pDomain, tBaseCode);

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
 
	var tVersion = @V1;
	var tElements = tAddState;
	if (tAddState instanceof Vector && tAddState.size() > 1 &&
	    tAddState[0] instanceof Symbol) 
	    tVersion = tAddState[0];
	if (tVersion != @V1 && tAddState.size() >= 4) {
	    tElements = tAddState[1];
	    mXoiIgnPlanDir4Rm = tAddState[2];
	    
	    
	    
	    
	    
	    
	    
	    
	    if (!xOiCompareObjs(tAddState[3], mXoiCommonProps)) {
		xOiDebugPrint(@Info, 1, 
			      "option @CommonProps has been changed:");
		xOiDebugPrint(@Info, 1, ["old: ", tAddState[3]]);
		xOiDebugPrint(@Info, 1, ["new: ", mXoiCommonProps]);
	    }
	}
	
	if (tVersion == @V3 && tAddState.size() >= 5)
	    mXoiAllElmnts4SubArt = tAddState[4];

	sXoiDuringSetAddStateCode = 1;

	

	var t2BeRemoved = @();
	xOiCopyAggr(getElOrder(), t2BeRemoved, 0);

	var tE;
	if (mXoiAllElmnts4SubArt) {
	    foreach(tE; getElements()) {
		if (t2BeRemoved.find(tE) < 0 && 
		    tE.getArticleObj().getArticleSpec() != NULL) {
		    var tEl = tE; 
		    t2BeRemoved.pushBack(tEl);
		}
	    }
	}

	foreach(tE; t2BeRemoved) {
	    xOiDebugPrint(@Info, 1, ["removing initial ", tE]);
	    sXoiAllowRemoval = tE;
	    self.remove(tE);
	}
	sXoiAllowRemoval = NULL;

	var tPlan = oiGetPlanning();

	foreach(tE; tElements) {
	    if (!(tE instanceof Vector && tE.size() >= 5) ||
	        !(tE[0] instanceof String && tE[2] instanceof String) ||
		!(tE[1] instanceof Symbol)) {
		xOiDebugPrint(@Warn, 1, ["invalid element: ", tE]);
		continue;
	    }
	    xOiDebugPrint(@Info, 2, ["element: ", tE]);

	    var tSubID   = tE[0];
	    var tPID     = tE[1];
	    var tArticle = tE[2];
	    var tPos     = tE[3];
	    var tRot     = tE[4];

	    if (!oiRegisterPackage(tPID)) {
		xOiDebugPrint(@Warn, 1, ["couldn't register package ", tPID]);
		continue;
	    }

	    var tType, tSubArt;
	    try {
		if (tPlan.isA(xOiBasePlanning))
		    tType = tPlan.article2Class(tArticle, 1);
		else
		    tType = tPlan.article2Class(tArticle);
		tSubArt = eval(tType+"(self, @"+tSubID+")");
		tSubArt.beAnElement();	
					
	    }
	    catch (&e: Error) {
		xOiDebugPrint(@Warn, 1, ["couldn't create element ", tSubID, 
					 " with article numbber ", tArticle, 
					 "' (", e.where, ": ", e.what, ")"]);
		continue;
	    }

	    
	    
	    

	    tSubArt.setPosition(tPos);
	    xOiSetYRotation(tSubArt, tRot);
	}

	sXoiDuringSetAddStateCode = 0;

	if ((tVersion == @V4 || tVersion == @V5) && tAddState.size() >= 6) {
	    
	    var tAddMembers = tAddState[5];
	    if (!tAddMembers.empty()) {
		xOiDebugPrint(@Info, 2, ["derived members: ", tAddMembers]);
		xOiProcessAddStateCode(self, xOiParam2Str(tAddMembers));
	    }
	}

	if (tVersion == @V5 && tAddState.size() >= 7)
	    mXoiCategories = xOiList2Hash(tAddState[6]);

	tPlan.setProgram(getProgram());

	_xOiElCategoriesRestored(); 

	var tSavedInsertMode = self.getInsertMode();
	var tOptionInsertMode = self.readOptionInsertMode(self.getProgram(), 
							  "jointplgroup", 
							  mXoiArticleSpec);
	if (tOptionInsertMode != NULL && 
	    tOptionInsertMode != tSavedInsertMode) {
	    xOiDebugPrint(@Info, 1, 
			  ["assigning new insert mode ", tOptionInsertMode,
			   " according to option @InsertMode (was: ",
			   tSavedInsertMode, ")"]);
	    setInsertMode(tOptionInsertMode);
	}

	xOiDebugFinishFunc(NULL);
    }

    
    
    func duringSetAddStateCode()
    {
	return(sXoiDuringSetAddStateCode);
    }

    
    
    
    
    
    
    
    public func updateConfiguration()
    {
	xOiUpdateCompositeArticle(self);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func isOrderPos()
    {
	xOiDebugStartFunc("xOiJointPlGroup::isOrderPos", NULL);

	var tRet = (mXoiArticleSpec != NULL);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }
 
    
    
    
    
    
    public func getOrderGroupLabel2(pLanguage)
    {
	xOiDebugStartFunc("xOiJointPlGroup::getOrderGroupLabel2", pLanguage);

	var tRet = 
	    (mXoiArticleSpec != NULL ?
	     getArticleText(pLanguage, @s) :
	     oiGetStringResource("::ofml::xoi::@JointPlanning", pLanguage));

	if (tRet == NULL)
	    oiGetStringResource("::ofml::xoi::@JointPlanning", pLanguage);
	else
	if (tRet instanceof List || tRet instanceof Vector)
	    tRet = tRet.front();

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    public func getOrderGroupLabel()
    {
	xOiDebugStartFunc("xOiJointPlGroup::getOrderGroupLabel", NULL);

	var tRet = getOrderGroupLabel2(oiGetPlanning().getLanguage());

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func getSubArticleElements()
    {
	xOiDebugStartFunc2("xOiJointPlGroup::getSubArticleElements", NULL);

	var tRet = @();

	var tElOrder = getElOrder();

	var tEl;
	foreach(tEl; tElOrder) {
	    var tE = tEl; 
	    tRet.pushBack(tE);
	}

	if (!mXoiAllElmnts4SubArt) {
	    xOiDebugFinishFunc(tRet);
	    return(tRet);
	}

	xOiDebugPrint(@Info, 1, "considering non-topological elements");

	foreach(tEl; getElements()) {
	    if (tElOrder.find(tEl) < 0 && tEl.isCat(@IF_Article) && 
		tEl.getArticleObj().getArticleSpec() != NULL) {
		xOiDebugPrint(@Info, 2, 
			      ["adding non-topological element ", tEl]);
		var tE = tEl; 
		tRet.pushBack(tE);
	    }
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    public func getPropSubArticleIDs()
    {
	return(self.getSubArticleIDs());
    }

    
    public func getPropSubArticle(pID)
    {
	return(self.getSubArticle(pID));
    }

    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func replaceElement(pEl, pPID, pArticle, pOptions)
    {
        xOiDebugStartFunc("xOiJointPlGroup::replaceElement", 
			  [pEl, pPID, pArticle, pOptions]);

	var tEl = pEl;

	if (pEl == NULL && getElCount() == 1) {
	    xOiDebugPrint(@Info, 1, "replacing initial element");
	    tEl = firstObj();
	}

	if (!prepareReplacement(tEl)) {
	    xOiDebugPrint(@Warn, 1, "given element cannot be replaced!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRet = _replaceElement2(tEl, pPID, pArticle, pOptions, NULL, NULL);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    public func replaceElement2(pEl, pPID, pArticle, pOptions, pAttPt, ...)
    {
        xOiDebugStartFunc("xOiJointPlGroup::replaceElement2", 
			  [pEl, pPID, pArticle, pOptions, pAttPt]);

	var tAttPt = pAttPt[0];
	if (!(tAttPt instanceof Symbol)) {
	    xOiDebugPrint(@Warn, 1, "ignoring invalid attach point parameter!");
	    tAttPt = NULL;
	}

	var tRefObj = NULL;

	var tDir = @L;
	if (pAttPt.size() > 1) {
	    if (pAttPt[1] instanceof Symbol &&
		(pAttPt[1] == @L || pAttPt[1] == @R))
		tDir = pAttPt[1];
	    else
		xOiDebugPrint(@Warn, 1, "ignoring invalid optional parameter!");
	}

	if (tAttPt != NULL) {
	    tRefObj = neighbor(pEl, tDir);

	    if (tRefObj == NULL) {
		xOiDebugPrint(@Warn, 1, 
			      "ignoring attach pt for not existing ref obj!");
		tAttPt = NULL;
	    }
	}

	var tGo = (tRefObj != NULL && tAttPt != NULL ?
		   prepareReplacement(pEl, tRefObj, tAttPt) :
		   prepareReplacement(pEl));

	if (!tGo) {
	    xOiDebugPrint(@Warn, 1, "given element cannot be replaced!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tRet = _replaceElement2(pEl, pPID, pArticle, pOptions, 
				    tRefObj, tAttPt);

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    private func _replaceElement2(pEl, pPID, pArticle, pOptions, 
				  pRefObj, pAttPt)
    {
        xOiDebugStartFunc2("xOiJointPlGroup::_replaceElement2", 
			   [pEl, pPID, pArticle, pOptions, pRefObj, pAttPt]);

	if (!(pPID instanceof Symbol && pArticle instanceof String &&
	      (pOptions == NULL || 
	       (pOptions instanceof Vector && pOptions.size() >= 2)))) {
	    xOiDebugPrint(@Warn, 1, "invalid parameter(s)!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tElName = pEl.getName().substr(self.getName().size()+1);
	var tOldPos = pEl.getPosition();
	var tOldRot = pEl.getRotation(@PY);

	sXoiAllowRemoval = pEl; 
	remove(pEl);
	sXoiAllowRemoval = NULL;

	var tElInfo = Vector(6);

	tElInfo[0] = pPID;
	tElInfo[1] = pArticle;
	tElInfo[2] = @VarCode;
	tElInfo[3] = "";
	tElInfo[4] = NULL;
	tElInfo[5] = NULL;

	if (pOptions != NULL) {
	    if (pOptions[0] instanceof Symbol && 
		pOptions[1] instanceof String) {
		tElInfo[2] = pOptions[0];
		tElInfo[3] = pOptions[1];
	    }
	    if (pOptions.size() > 2 && pOptions[2] instanceof Vector)
		tElInfo[4] = pOptions[2];
	    if (pOptions.size() > 3 && pOptions[3] instanceof Vector)
		tElInfo[5] = pOptions[3];
	}

	var tNewEl = _createElement(@Replace, tElInfo, tElName);

	if (tNewEl == NULL) {
	    xOiDebugPrint(@Warn, 1, "new element couldn't be created!");
	    xOiDebugFinishFunc(NULL);
	    return(NULL);
	}

	var tElPos = tOldPos;
	var tElRot = tOldRot;

	if (pRefObj != NULL && pAttPt != NULL) {
	    var tCurrAttPt = NULL;
	    if (pRefObj.hasMember(@getActiveAttPt))
		tCurrAttPt = pRefObj.getActiveAttPt();
	    pRefObj.setActiveAttPt(pAttPt);
	    var tPR = oiGetPosRot4AttachPts(pRefObj, tNewEl, NULL, 0);
	    xOiDebugPrint(@Info, 2, ["new pos/rot: ", tPR]);
	    if (tCurrAttPt != NULL)
		pRefObj.setActiveAttPt(tCurrAttPt); 
	    if (tPR == NULL || tPR[0] == NULL || tPR[2] != NULL)
		xOiDebugPrint(@Warn, 1, "error during repositing!");
	    else {
		tElPos = tPR[0];
		tElRot = tPR[1];
	    }
	}

	tNewEl.setPosition(tElPos);
	xOiSetYRotation(tNewEl, tElRot);

	widthChanged(tNewEl);

	xOiDebugFinishFunc(tNewEl);
	return(tNewEl);
    }

    
    
    
    public func isBusyAttPt(pEl, pAttPt)
    {
	xOiDebugStartFunc("xOiJointPlGroup::isBusyAttPt", [pEl, pAttPt]);

	var tAttPtsOrder =  pEl.getAttPtsOrder();
	xOiDebugPrint(@Info, 2, ["current att pts order: ", tAttPtsOrder]);

	if (tAttPtsOrder.empty()) {
	    xOiDebugPrint(@Info, 1,
			  "currently, object has no defined attach points!");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	var tAttPt, tAttPts = @();
	foreach(tAttPt; tAttPtsOrder) {
	    if (tAttPt[0] == pAttPt) {
		tAttPts.pushBack(xOiCopyAggr(tAttPt, NULL, 0));
		break;
	    }
	}

	if (tAttPts.empty()) {
	    xOiDebugPrint(@Info, 1, "currently, there is no attach point " +
				    "defined with given key!");
	    xOiDebugFinishFunc(0);
	    return(0);
	}

	var tBusyAttPts = NULL;
	var tEps = 0.05;	
				
	try {
	    tBusyAttPts = oiGetBusyAttPts(pEl, tAttPts, NULL, tEps);
	}
	catch (&e: Error) {
	    xOiDebugPrint(@Warn, 1, e.where + ": " + e.what);
	}
	xOiDebugPrint(@Info, 2, ["oiGetBusyAttPts(): ", tBusyAttPts]);

	var tRes = (tBusyAttPts != NULL &&
		    tBusyAttPts.size() == 1 && tBusyAttPts.back() == pAttPt);

	xOiDebugFinishFunc(tRes);
	return(tRes);
    }

    
    
    
    
    
    
    
    protected func addElToOrderList(pEl)
    {
        xOiDebugStartFunc("xOiJointPlGroup::addElToOrderList", pEl);

	if (sXoiDuringSetAddStateCode) {
	    if (isValidForLRPlanning(pEl)) {
		var tElOrder = getElOrder();
		tElOrder.pushBack(pEl);
		xOiDebugPrint(@Info, 1, ["new order list: ", tElOrder]);
	    }
	    else
		xOiDebugPrint(@Info, 1, "element is not valid for LR planning");
	}
	else
	    xOiLRPlGroup::addElToOrderList(pEl);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    protected func handleNewElement(pEl)
    {
	if (pEl == NULL) return(0);

	var tIsLayoutEL = (getElPos(pEl) != NULL);

	if ((tIsLayoutEL  && !mXoiSelectState4Layout) ||
	    (!tIsLayoutEL && !mXoiSelectState4Other))
	    pEl.notSelectable();

	pEl.setCutable(tIsLayoutEL ? mXoiCutState4Layout : mXoiCutState4Other);

	return(0);
    }

    
    
    
    
    
    
    
    
    
    
    public func elRemoveValid(pEl)
    {
        xOiDebugStartFunc("xOiJointPlGroup::elRemoveValid", pEl);

	var tRet = 1;

	if (sXoiAllowRemoval == NULL || sXoiAllowRemoval != pEl) 
	    tRet = xOiLRPlGroup::elRemoveValid(pEl);
	else
	    xOiDebugPrint(@Info, 1, "allow internal removal");

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    protected func duringInternalRemoval()
    {
	return(sXoiAllowRemoval != NULL);
    }

    
    
    
    
    
    protected func rmElFromOrderList(pEl)
    {
	xOiDebugStartFunc("xOiJointPlGroup::rmElFromOrderList", pEl);

	if (sXoiDuringSetAddStateCode) {
	    var tElOrder = getElOrder();
	    tElOrder.remove(pEl);
	    xOiDebugPrint(@Info, 1, ["new order list: ", tElOrder]);
	}
	else
	    xOiLRPlGroup::rmElFromOrderList(pEl);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    protected func ignorePlanDir4Remove()
    {
	return(mXoiIgnPlanDir4Rm);
    }

    
    
    
 
    protected func _xOiGetOptionCommonProps()
    {
	return(mXoiCommonProps);
    }

    protected func _xOiGetOption1stPos4CommonProps()
    {
	return(mXoiFirstPos4CommonProps);
    }

    protected func _xOiGetOptionCommonPropsPos()
    {
	return(mXoiCommonPropsPos);
    }

    protected func getObjs4OptionCommonProps()
    {
	if (mXoiNonLayout4CommonProps)
	    return(self.getElements());

	return(self.getElOrder());
    }

    protected func _xOiGetOptionAllObjs4CommonProps()
    {
	return(mXoiAllObjs4CommonProps);
    }

    protected func _xOiGetOptionCommonPropsChLMode()
    {
	return(mXoiCommonPropsChLMode);
    }

    protected func _xOiGetOptionSortUnionCommonProps()
    {
	return(mXoiSortUnionCommonProps);
    }

    protected func _xOiGetOptionClasses4CommonProps()
    {
	return(mXoiClasses4CommonProps);
    }

    protected func _xOiGetOptionMeta4CommonProps()
    {
	return(mXoiMeta4CommonProps);
    }

    protected func _xOiGetOptionCommonPropsDepth()
    {
	return(mXoiCommonPropsDepth);
    }

    protected func _xOiGetOptionNonVisibleProps4Common()
    {
	return(mXoiNonVisibleProps4Common);
    }

    protected func _xOiGetOptionROPropsEditable4Common()
    {
	return(mXoiROPropsEditable4Common);
    }

    protected func _xOiGetOptionArticleFeatures4Common()
    {
	return(mXoiArticleFeatures4Common);
    }

    
    
    
    
    
    protected func getFixProperties()
    {
	xOiDebugStartFunc("xOiJointPlGroup::getFixProperties", NULL);

	var tRet = @();
	var tKey;
	foreach(tKey; mXoiCommonProps) {
	    var tPKey = tKey; 
	    if (self.hasProperty(tPKey))
		tRet.pushBack(tPKey);
	}

	xOiDebugFinishFunc(tRet);
	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func assignCommonPropValues(pEl)
    {
	xOiDebugStartFunc("xOiJointPlGroup::assignCommonPropValues", pEl);

	if (pEl instanceof Void) {
	    xOiDebugPrint(@Warn, 1, "invalid parameter!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tEL = self.getElOrder();
	if (mXoiNonLayout4CommonProps)
	    tEL = self.getElements();

	var tOL = pEl;
	if (!(pEl instanceof List || pEl instanceof Vector))
	    tOL = @(pEl);

	var tObjects = @();
	var tO;
	foreach(tO; tOL) {
	    var tObj = tO; 
	    if (tEL.find(tO) >= 0)
		tObjects.pushBack(tObj);
	}

	if (tObjects.empty()) {
	    xOiDebugPrint(@Warn, 1, "invalid parameter!");
	    xOiDebugFinishFunc(NULL);
	    return;
	}

	var tCPH = xOiCommonPropsHandler(self);
	tCPH.setCommonPropsDeep(NULL, tObjects);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func updateCommonProperties()
    {
	xOiDebugStartFunc("xOiJointPlGroup::updateCommonProperties", NULL);

	var tCPH = xOiCommonPropsHandler(self);
	tCPH.initCommonProps(NULL, 0);

	xOiDebugFinishFunc(NULL);
    }

    
    
    

    
    protected func _xOiGetElementCatHash()
    {
	return(mXoiCategories);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func assignElementCat(pObj, pCat)
    {
	_xOiAssignElementCat(pObj, pCat, mXoiCategories);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func removeElementCat(pObj, pCat)
    {
	_xOiRemoveElementCat(pObj, pCat, mXoiCategories);
    }

    
    
    
    public func isElementCat(pObj, pCat)
    {
	return(_xOiIsElementCat(pObj, pCat, mXoiCategories));
    }

    
    
    public func getCategoriesOfElement(pObj)
    {
	return(_xOiGetElementCategories(mXoiCategories, pObj));
    }

    
    public func clearElementCat(pCat)
    {
	_xOiClearElementCat(pCat, mXoiCategories);
    }

    
    public func getElementCategories()
    {
	return(_xOiGetElementCategories(mXoiCategories, NULL));
    }

    
    
    public func getCatElements(pCat)
    {
	return(_xOiGetCatElements(pCat, mXoiCategories));
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setSelectability4ElemCat(pCat, pState, ...)
    {
	xOiDebugStartFunc("xOiJointPlGroup::setSelectability4ElemCat", 
			  [pCat, pState]);

	var tState = pState[0];
	var tApplyHier = 0;
	if (pState.size() > 1 && pState[1] instanceof Int)
	    tApplyHier =  pState[1];

	_xOiSetSelectability4ElemCat(pCat, tState, tApplyHier, mXoiCategories);

	xOiDebugFinishFunc(NULL);
    }

    
    
    
    
    public func getSelectability4ElemCat(pCat)
    {
	_xOiGetSelectability4ElemCat(pCat);
    }
}

