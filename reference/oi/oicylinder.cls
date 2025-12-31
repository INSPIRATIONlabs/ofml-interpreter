// === ofml/oi/oicylinder.cls ===
package ::ofml::oi;
import ::egr::found::*;











public class OiCylinder: OiGeometry
{
    
    public func initialize(pFa, pNa, pLength, pRadius) 
    {
	OiGeometry::initialize(pFa, pNa);
	
	Cylinder(self, @geo, pLength, pRadius);
    }

    
    public func setLength(pLength) 
    {
	self.geo.setLength(pLength);
    }

    
    public func getLength() 
    {
	return(self.geo.getLength());
    }
    
    
    public func setRadius(pRadius) 
    {
	self.geo.setRadius(pRadius);
    }

    
    public func getRadius() 
    {
	return(self.geo.getRadius());
    }
    
}
