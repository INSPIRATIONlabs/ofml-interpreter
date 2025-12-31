// === ofml/oi/oisphere.cls ===
package ::ofml::oi;
import ::egr::found::*;











public class OiSphere: OiGeometry
{
    
    public func initialize(pFa, pNa, pRad) 
    {
	OiGeometry::initialize(pFa, pNa);
	
	Sphere(self, @geo, pRad);
    }

    
    public func setRadius(pRadius) 
    {
	if (pRadius <= 0.0) afError("::ofml::oi::@OIbadRadius");
	self.geo.setRadius(pRadius);
    }

    
    public func getRadius() 
    {
	return(self.geo.getRadius());
    }
}


