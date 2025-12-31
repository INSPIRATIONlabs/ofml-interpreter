// === ofml/xoi/xoiglobal.cls ===
package ::ofml::xoi;




    






public class xOiGlobal
{
    
    static var sConfProfileTE = @(
	["manufacturer", 0, 0+0, @s, ""+"", "18"],
	["series", 0, 0+0, @s, ""+"", "18"],
	["pclass", 0, 0+0, @s, ""+"", "18"],
	["article", 0, 0+0, @s, ""+"", "18"],
	["property", 0, 0+0, @s, ""+"", "30"],
	["value", 0, 0+0, @s, ""+"", "30"]);

    
    static var sConfProfileIndices = @(
	["scope", ["manufacturer", "series", "pclass", "article", "property"], 
	 1, 0],
	["manu_prop", ["manufacturer", "property"], 0, 0]);
}


