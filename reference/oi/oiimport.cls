// === ofml/oi/oiimport.cls ===
package ::ofml::oi;
import ::egr::found::*;











public class OiImport: OiGeometry
{
    
    public func initialize(pFa, pNa, pMode, pFile) 
    {
	OiGeometry::initialize(pFa, pNa);
	
	if (pMode == @OFF || pMode == @G3DS) OffPolygon(self, @geo, pFile);
    }

    
    public func setScale(pS) 
    {
	self.geo.setScale(pS);
    }

    
    public func getScale() 
    {
	return(self.geo.getScale());
    }
}

