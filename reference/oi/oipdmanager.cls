// === ofml/oi/oipdmanager.cls ===
package ::ofml::oi;
import ::egr::found::*;















public class OiPDManager: xxStdPDManager
{
    
    public func initialize(pFa, pNa)
    {
	xxStdPDManager::initialize(pFa, pNa);
    }

    public func isCat(pCat)
    {
	if (pCat == @IF_MObject) return(1);

	return(0);
    }

    public func getPlanning()
    {
	
	

	var tF = getFather();

	if (tF.isA(OiPlanning)) return(tF);

	return(NULL);
    }

    
    public func article2Class(pArticle)
    {
	return(NULL);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func article2ODBName(pArticle, ...)
    {
	var tRet = NULL;

	if (self.hasMember(@article2ODBType))
	    
	    tRet = self.article2ODBType(NULL, pArticle[0]);

        return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkPDB4Article(pProgram, pRegion, pVersion, pBaseArticle)
    {
	oiDebugStartFunc("OiPDManager::checkPDB4Article", 
			 [pProgram, pRegion, pVersion, pBaseArticle]);

	var tPDB = self.getProgPDB(pProgram);

	var tOk = 0;

	if (tPDB == NULL || pBaseArticle == NULL || pBaseArticle.empty()) {
	    if (tPDB == NULL)
		oiDebugPrint(@Warn, 1, "no product database!");
	    else
		oiDebugPrint(@Warn, 1, "no article!");
	    oiDebugFinishFunc(tOk);
	    return(tOk);
	}

	var tMID  = oiGetAppManufacturerID(pProgram);
	var tSIDs = oiParseString2List(oiGetAppSeriesID(pProgram), ';');

	
	

	oiDebugPrint(@Info, 2, ["manufacturer: ", tMID]);
	oiDebugPrint(@Info, 2, ["series: ", tSIDs]);

	if (tMID == NULL || tSIDs == NULL) {
	    if (tMID == NULL)
		oiDebugPrint(@Warn, 1,
			      ["no manufacturer ID for program ", pProgram]);
	    if (tSIDs == NULL)
		oiDebugPrint(@Warn, 1,
			      ["no series ID for program ", pProgram]);
	}
	else {
	    var tSID;
	    foreach(tSID; tSIDs)
		if (tPDB.containsArticle(tMID, tSID, pBaseArticle)) {
		    tOk = 1;
		    break;
		}
	}
	oiDebugFinishFunc(tOk);
	return(tOk);
    }

    public func resetProps(pObj)
    {
    }

    public func getAllPropSpecs(pObj, pRequiredState)
    {
	return(@());
    }

    public func getAllPropValueInfos(pObj, pPKey, pPClass)
    {
	return(NULL);
    }

    public func getDefaultPropGroupSpecs(pObj)
    {
	return(@());
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkUpdatability(pProgram, pRegion, pVersion, pBaseArticle, 
				  pArticleCode, pCodeType)
    {
	return(1);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func checkObjUpdatability(pObj, pCodeType)
    {
	return(1);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func setupConfiguration(pObject, pBaseArticle, pArticleCode,
                                   pCodeType, pMigration)
    {
	return(1);
    }

    
    
    
    
    
    
    
    
    
    
    public func updateConfiguration(pObj)
    {
    }

    
    public func checkConsistency(pObj, pLanguage, pReport)
    {
	return(1);
    }

    
    public func getArticleFeatures2(pObj, pLanguage)
    {
	return(NULL);
    }

    
    public func getArticleFeaturesDescr(pObj, pType, pLanguage)
    {
	return(NULL);
    }

    
    public func getAllArticleFeatures(pObj, pLanguage)
    {
	return(NULL);
    }

    
    public func getXArticleSpec(pObj, pMode)
    {
	if (pMode != @EAN_UCC)
	    
	    return(NULL);

	var tPDB = getPDBFor(pObj);
	var tArticle = pObj.getArticleSpec();
	var tFinal   = pObj.getXArticleSpec(@Final);

	var tSpec = NULL;
	if (tPDB != NULL && tArticle != NULL)
	    tSpec = tPDB.getArticleIdentification(tArticle, tFinal, @EAN_UCC);

	return(tSpec);
    }

    
    public func setXArticleSpec(pObj, pMode, pSpec)
    {
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticleClassifications(pObj, pLanguages) 
    {
	var tPDB = getPDBFor(pObj);
	var tArticle = pObj.getArticleSpec();

	

	var tRet = NULL;

	if (tPDB != NULL && tArticle != NULL)
	    tRet = tPDB.getArticleClassifications(tArticle, pLanguages);

	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    public func getArticleAttribute(pObj, pAttr) 
    {
	var tArticle, tPDB;

	if (pObj instanceof Vector) {
	    oiRegisterPackage(pObj[0]); 
	    tPDB     = getProgPDB(pObj[0]);
	    tArticle = pObj[1];
	}
	else {
	    tPDB = getPDBFor(pObj);
	    tArticle = pObj.getArticleSpec();
	}
	
	var tRet = NULL;
	if (tPDB != NULL && tArticle != NULL)
	    tRet = tPDB.getArticleAttribute(tArticle, pAttr);

	return(tRet);
    }

    
    
    
    
    
    
    
    
    
    
    
    public func getOrderUnit(pObj) 
    {
	var tPDB = getPDBFor(pObj);
	var tArticle = pObj.getArticleSpec();

	var tUnit = "C62";
	if (tPDB != NULL && tArticle != NULL)
	    tUnit = tPDB.getOrderUnit(tArticle);

	return(tUnit);
    }
}

OiGlobal();

