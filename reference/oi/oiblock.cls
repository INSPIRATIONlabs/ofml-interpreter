// === ofml/oi/oiblock.cls ===
package ::ofml::oi;
import ::egr::found::*;











public class OiBlock: OiGeometry
{
    
    public func initialize(pFa, pNa, pDimensions) 
    {
	OiGeometry::initialize(pFa, pNa);

	if (pDimensions instanceof OiBlock)
	afError("OiBlock: "+pFa.getName()+" ("+pFa.getClass()+") @"+String(pNa)
		+" dimension is a OiBlock");
	Block(self, @geo, pDimensions);
    }

    
    public func setDimensions(pDimensions) 
    {
	self.geo.setDimensions(pDimensions);
    }

    
    public func getDimensions() 
    {
	return(self.geo.getDimensions());
    }
}

