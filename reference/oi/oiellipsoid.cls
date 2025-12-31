// === ofml/oi/oiellipsoid.cls ===
package ::ofml::oi;
import ::egr::found::*;











public class OiEllipsoid: OiGeometry
{
    
    public func initialize(pFa, pNa, pDimensions) 
    {
	OiGeometry::initialize(pFa, pNa);
	
	Ellipsoid(self, @geo, pDimensions);
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
