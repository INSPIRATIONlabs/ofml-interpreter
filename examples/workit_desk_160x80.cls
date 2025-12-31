// Vitra Workit Desk 160x80cm Assembly
// Uses actual 3DS geometry from the workit ALB
//
// Dimensions:
// - Tabletop: 160cm x 80cm (1.6m x 0.8m)
// - Table height: 74cm (legs + tabletop thickness ~2.5cm)
// - Legs inset: ~8cm from edges
//
// Alignment methods:
// setAlignment([@C, @I, @C]) = center X, min Y (grounded), center Z
// setFootAlignment() = auto-detect ground contact point for L-shaped parts

// Load tabletop - 160x80cm
// Align to center XZ, ground Y, then position at tabletop height
var tabletop = OffPolygon(NULL, @tabletop, "wk_tt_160_080");
tabletop.setAlignment([@C, @I, @C]);
tabletop.setPosition([0.8, 0.715, 0.4]);

// Load 4 legs at corner positions
// Legs are L-shaped, so use setFootAlignment() to detect ground contact point
// Position at corners inset 8cm from edges

// Front-left leg (X=0.08, Z=0.08)
var leg_fl = OffPolygon(NULL, @leg_fl, "wk_leg1_74");
leg_fl.setFootAlignment();
leg_fl.setPosition([0.08, 0, 0.08]);

// Front-right leg (X=1.52, Z=0.08)
var leg_fr = OffPolygon(NULL, @leg_fr, "wk_leg1_74");
leg_fr.setFootAlignment();
leg_fr.setPosition([1.52, 0, 0.08]);

// Back-left leg (X=0.08, Z=0.72)
var leg_bl = OffPolygon(NULL, @leg_bl, "wk_leg1_74");
leg_bl.setFootAlignment();
leg_bl.setPosition([0.08, 0, 0.72]);

// Back-right leg (X=1.52, Z=0.72)
var leg_br = OffPolygon(NULL, @leg_br, "wk_leg1_74");
leg_br.setFootAlignment();
leg_br.setPosition([1.52, 0, 0.72]);

// Add gliders under each leg
// Gliders use same position as legs since legs are positioned by their foot

var glider_fl = OffPolygon(NULL, @glider_fl, "wk_glider");
glider_fl.setAlignment([@C, @I, @C]);
glider_fl.setPosition([0.08, 0, 0.08]);

var glider_fr = OffPolygon(NULL, @glider_fr, "wk_glider");
glider_fr.setAlignment([@C, @I, @C]);
glider_fr.setPosition([1.52, 0, 0.08]);

var glider_bl = OffPolygon(NULL, @glider_bl, "wk_glider");
glider_bl.setAlignment([@C, @I, @C]);
glider_bl.setPosition([0.08, 0, 0.72]);

var glider_br = OffPolygon(NULL, @glider_br, "wk_glider");
glider_br.setAlignment([@C, @I, @C]);
glider_br.setPosition([1.52, 0, 0.72]);
