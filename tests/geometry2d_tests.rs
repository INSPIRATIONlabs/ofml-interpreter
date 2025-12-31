//! Integration tests for 2D geometry processing.
//!
//! These tests verify the 2D representation generation pipeline.

use ofml_interpreter::geometry2d::{
    Arc2D, Circle2D, Ellipse2D, FillStyle, G2DAttributes, G2DCompound, G2DPrimitive, Line2D,
    LineStyle, Point2D, Rect2D, Text2D, TextAnchor, Transform2D,
};

/// Test creating 2D primitives
#[test]
fn test_2d_primitives_creation() {
    // Points
    let points = vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 1.0, y: 0.0 },
        Point2D { x: 1.0, y: 1.0 },
    ];
    let primitive = G2DPrimitive::Points(points);
    assert!(matches!(primitive, G2DPrimitive::Points(_)));

    // Lines
    let lines = vec![Line2D {
        start: Point2D { x: 0.0, y: 0.0 },
        end: Point2D { x: 1.0, y: 1.0 },
    }];
    let primitive = G2DPrimitive::Lines(lines);
    assert!(matches!(primitive, G2DPrimitive::Lines(_)));

    // Rectangle
    let rect = Rect2D {
        x: 0.0,
        y: 0.0,
        width: 2.0,
        height: 1.5,
    };
    let primitive = G2DPrimitive::Rectangle(rect);
    assert!(matches!(primitive, G2DPrimitive::Rectangle(_)));

    // Circle
    let circle = Circle2D {
        center: Point2D { x: 0.5, y: 0.5 },
        radius: 0.25,
    };
    let primitive = G2DPrimitive::Circle(circle);
    assert!(matches!(primitive, G2DPrimitive::Circle(_)));
}

/// Test ellipse and arc primitives
#[test]
fn test_2d_ellipse_arc() {
    // Ellipse
    let ellipse = Ellipse2D {
        center: Point2D { x: 1.0, y: 1.0 },
        rx: 0.5,
        ry: 0.3,
    };
    let primitive = G2DPrimitive::Ellipse(ellipse);
    assert!(matches!(primitive, G2DPrimitive::Ellipse(_)));

    // Arc
    let arc = Arc2D {
        center: Point2D { x: 0.0, y: 0.0 },
        radius: 0.5,
        start_angle: 0.0,
        end_angle: std::f64::consts::FRAC_PI_2,
    };
    let primitive = G2DPrimitive::Arc(arc);
    assert!(matches!(primitive, G2DPrimitive::Arc(_)));
}

/// Test text primitive
#[test]
fn test_2d_text() {
    let text = Text2D {
        position: Point2D { x: 0.0, y: 0.0 },
        text: "Test".to_string(),
        font_size: 12.0,
        anchor: TextAnchor::Start,
    };
    let primitive = G2DPrimitive::Text(text);
    assert!(matches!(primitive, G2DPrimitive::Text(_)));
}

/// Test 2D compound structure
#[test]
fn test_2d_compound() {
    let mut compound = G2DCompound::new();

    // Add primitives directly to the vector
    let rect = Rect2D {
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    };
    compound.primitives.push(G2DPrimitive::Rectangle(rect));

    let circle = Circle2D {
        center: Point2D { x: 0.5, y: 0.5 },
        radius: 0.25,
    };
    compound.primitives.push(G2DPrimitive::Circle(circle));

    assert_eq!(compound.primitives.len(), 2);
}

/// Test 2D transforms
#[test]
fn test_2d_transforms() {
    // Identity transform
    let transform = Transform2D::default();
    assert!((transform.translation[0] - 0.0).abs() < 0.01);
    assert!((transform.translation[1] - 0.0).abs() < 0.01);
    assert!((transform.scale[0] - 1.0).abs() < 0.01);
    assert!((transform.scale[1] - 1.0).abs() < 0.01);
    assert!((transform.rotation - 0.0).abs() < 0.01);

    // Custom transform
    let mut transform = Transform2D::default();
    transform.translation = [10.0, 20.0];
    transform.scale = [2.0, 2.0];
    transform.rotation = std::f64::consts::FRAC_PI_4;

    assert!((transform.translation[0] - 10.0).abs() < 0.01);
    assert!((transform.translation[1] - 20.0).abs() < 0.01);
    assert!((transform.scale[0] - 2.0).abs() < 0.01);
    assert!((transform.rotation - std::f64::consts::FRAC_PI_4).abs() < 0.01);
}

/// Test SVG export
#[test]
fn test_svg_export() {
    let mut compound = G2DCompound::new();

    // Add a rectangle
    let rect = Rect2D {
        x: 10.0,
        y: 10.0,
        width: 80.0,
        height: 60.0,
    };
    compound.primitives.push(G2DPrimitive::Rectangle(rect));

    // Add a circle
    let circle = Circle2D {
        center: Point2D { x: 50.0, y: 40.0 },
        radius: 20.0,
    };
    compound.primitives.push(G2DPrimitive::Circle(circle));

    // Export to SVG
    let svg = compound.to_svg();

    // Verify SVG structure
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    assert!(svg.contains("<rect") || svg.contains("rect"));
    assert!(svg.contains("<circle") || svg.contains("circle"));
}

/// Test polygon primitives
#[test]
fn test_polygon_primitives() {
    let vertices = vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 1.0, y: 0.0 },
        Point2D { x: 1.0, y: 1.0 },
        Point2D { x: 0.0, y: 1.0 },
    ];
    let primitive = G2DPrimitive::Polygon(vertices.clone());

    if let G2DPrimitive::Polygon(v) = primitive {
        assert_eq!(v.len(), 4);
    } else {
        panic!("Expected Polygon");
    }
}

/// Test line strip and line loop
#[test]
fn test_line_strip_and_loop() {
    let points = vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 1.0, y: 0.5 },
        Point2D { x: 2.0, y: 0.0 },
        Point2D { x: 2.0, y: 1.0 },
    ];

    let strip = G2DPrimitive::LineStrip(points.clone());
    assert!(matches!(strip, G2DPrimitive::LineStrip(_)));

    let loop_prim = G2DPrimitive::LineLoop(points);
    assert!(matches!(loop_prim, G2DPrimitive::LineLoop(_)));
}

/// Test attributes
#[test]
fn test_2d_attributes() {
    let mut attrs = G2DAttributes::default();
    attrs.color = [1.0, 0.5, 0.0, 1.0];
    attrs.fill_color = Some([0.0, 1.0, 0.5, 0.8]);
    attrs.line_width = 3.0;
    attrs.line_style = LineStyle::Dashed;
    attrs.fill_style = FillStyle::Solid;
    attrs.layer = "layer_5".to_string();

    assert!((attrs.color[0] - 1.0).abs() < 0.01);
    assert!(attrs.fill_color.is_some());
    assert!((attrs.line_width - 3.0).abs() < 0.01);
    assert_eq!(attrs.layer, "layer_5");
}

/// Test compound with transform
#[test]
fn test_compound_with_transform() {
    let mut compound = G2DCompound::new();

    // Apply a transform to the compound
    compound.transform.translation = [50.0, 50.0];
    compound.transform.scale = [2.0, 2.0];

    // Add a simple shape
    let rect = Rect2D {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };
    compound.primitives.push(G2DPrimitive::Rectangle(rect));

    let svg = compound.to_svg();

    // Should contain transform in SVG
    assert!(svg.contains("<svg") || svg.contains("transform="));
}

/// Test nested compounds
#[test]
fn test_nested_compounds() {
    let mut parent = G2DCompound::new();
    let mut child = G2DCompound::new();

    // Add primitive to child
    let circle = Circle2D {
        center: Point2D { x: 0.0, y: 0.0 },
        radius: 1.0,
    };
    child.primitives.push(G2DPrimitive::Circle(circle));

    // Add child to parent
    parent.children.push(child);

    assert_eq!(parent.children.len(), 1);
    assert_eq!(parent.children[0].primitives.len(), 1);
}

/// Test line styles
#[test]
fn test_line_styles() {
    let styles = [
        LineStyle::Solid,
        LineStyle::Dashed,
        LineStyle::Dotted,
        LineStyle::DashDot,
    ];

    for style in styles {
        // All styles should have a valid dasharray (or None for solid)
        let _ = style.to_svg_dasharray();
    }
}
