// Bordbar Mobile Bar Trolley Assembly
// Premium airline-style bar cart on wheels
//
// Approximate dimensions: 30x93x41cm (WxHxD)
// Components loaded from OBJ geometry

// Main body frame - the structural backbone
var body_frame = OffPolygon(NULL, @body_frame, "bordbar_body_frame_fix");
body_frame.setPosition([0.0, 0.0, 0.0]);

// Inside face - visible interior panels
var inside = OffPolygon(NULL, @inside, "bordbar_body_inside_face");
inside.setPosition([0.0, 0.0, 0.0]);

// Outside panels
var back = OffPolygon(NULL, @back, "bordbar_body_outside_back");
back.setPosition([0.0, 0.0, 0.0]);

var top_panel = OffPolygon(NULL, @top_panel, "bordbar_body_outside_top");
top_panel.setPosition([0.0, 0.0, 0.0]);

var bottom_panel = OffPolygon(NULL, @bottom_panel, "bordbar_body_outside_bottom");
bottom_panel.setPosition([0.0, 0.0, 0.0]);

var left_panel = OffPolygon(NULL, @left_panel, "bordbar_body_outside_left");
left_panel.setPosition([0.0, 0.0, 0.0]);

var right_panel = OffPolygon(NULL, @right_panel, "bordbar_body_outside_right");
right_panel.setPosition([0.0, 0.0, 0.0]);

// Door assembly
var door_frame = OffPolygon(NULL, @door_frame, "bordbar_door_frame");
door_frame.setPosition([0.0, 0.0, 0.0]);

var door_panel = OffPolygon(NULL, @door_panel, "bordbar_door_panel");
door_panel.setPosition([0.0, 0.0, 0.0]);

// Top handle
var top_handle = OffPolygon(NULL, @top_handle, "bordbar_top_handle");
top_handle.setPosition([0.0, 0.0, 0.0]);

// Front handle
var handle_front = OffPolygon(NULL, @handle_front, "bordbar_handle_front");
handle_front.setPosition([0.0, 0.0, 0.0]);

var handle_support = OffPolygon(NULL, @handle_support, "bordbar_handle_support");
handle_support.setPosition([0.0, 0.0, 0.0]);

// Locker
var locker = OffPolygon(NULL, @locker, "bordbar_locker_body");
locker.setPosition([0.0, 0.0, 0.0]);

var locker_lock = OffPolygon(NULL, @locker_lock, "bordbar_locker_lock");
locker_lock.setPosition([0.0, 0.0, 0.0]);

// Castors (wheels) - 4 corners
var castor_fl = OffPolygon(NULL, @castor_fl, "bordbar_castor_outside");
castor_fl.setPosition([0.0, 0.0, 0.0]);

var castor_fr = OffPolygon(NULL, @castor_fr, "bordbar_castor_outside");
castor_fr.setPosition([0.0, 0.0, 0.0]);

var castor_bl = OffPolygon(NULL, @castor_bl, "bordbar_castor_outside");
castor_bl.setPosition([0.0, 0.0, 0.0]);

var castor_br = OffPolygon(NULL, @castor_br, "bordbar_castor_outside");
castor_br.setPosition([0.0, 0.0, 0.0]);

// Brake system
var brake_left = OffPolygon(NULL, @brake_left, "bordbar_brake_pedal_left");
brake_left.setPosition([0.0, 0.0, 0.0]);

var brake_right = OffPolygon(NULL, @brake_right, "bordbar_brake_pedal_right");
brake_right.setPosition([0.0, 0.0, 0.0]);

// Logo
var logo = OffPolygon(NULL, @logo, "bordbar_logo");
logo.setPosition([0.0, 0.0, 0.0]);
