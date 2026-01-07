//! 2D Representation System - Floor plan generation.
//!
//! This module implements 2D geometry primitives and SVG export
//! for floor plan views.

use std::f64::consts::PI;

/// 2D point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Create a new point.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Origin point (0, 0).
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Distance to another point.
    pub fn distance(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Apply a transform.
    pub fn transform(&self, t: &Transform2D) -> Point2D {
        // Apply scale
        let x = self.x * t.scale[0];
        let y = self.y * t.scale[1];

        // Apply rotation
        let cos_r = t.rotation.cos();
        let sin_r = t.rotation.sin();
        let rx = x * cos_r - y * sin_r;
        let ry = x * sin_r + y * cos_r;

        // Apply translation
        Point2D {
            x: rx + t.translation[0],
            y: ry + t.translation[1],
        }
    }
}

/// 2D line segment.
#[derive(Debug, Clone)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

impl Line2D {
    /// Create a new line.
    pub fn new(start: Point2D, end: Point2D) -> Self {
        Self { start, end }
    }

    /// Create from coordinates.
    pub fn from_coords(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            start: Point2D::new(x1, y1),
            end: Point2D::new(x2, y2),
        }
    }

    /// Get the length.
    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }
}

/// 2D rectangle.
#[derive(Debug, Clone)]
pub struct Rect2D {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect2D {
    /// Create a new rectangle.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Get the center point.
    pub fn center(&self) -> Point2D {
        Point2D::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Get the corner points.
    pub fn corners(&self) -> [Point2D; 4] {
        [
            Point2D::new(self.x, self.y),
            Point2D::new(self.x + self.width, self.y),
            Point2D::new(self.x + self.width, self.y + self.height),
            Point2D::new(self.x, self.y + self.height),
        ]
    }
}

/// 2D circle.
#[derive(Debug, Clone)]
pub struct Circle2D {
    pub center: Point2D,
    pub radius: f64,
}

impl Circle2D {
    /// Create a new circle.
    pub fn new(center: Point2D, radius: f64) -> Self {
        Self { center, radius }
    }

    /// Create from center coordinates.
    pub fn from_coords(cx: f64, cy: f64, radius: f64) -> Self {
        Self {
            center: Point2D::new(cx, cy),
            radius,
        }
    }
}

/// 2D ellipse.
#[derive(Debug, Clone)]
pub struct Ellipse2D {
    pub center: Point2D,
    pub rx: f64,
    pub ry: f64,
}

impl Ellipse2D {
    /// Create a new ellipse.
    pub fn new(center: Point2D, rx: f64, ry: f64) -> Self {
        Self { center, rx, ry }
    }
}

/// 2D arc.
#[derive(Debug, Clone)]
pub struct Arc2D {
    pub center: Point2D,
    pub radius: f64,
    /// Start angle in radians
    pub start_angle: f64,
    /// End angle in radians
    pub end_angle: f64,
}

impl Arc2D {
    /// Create a new arc.
    pub fn new(center: Point2D, radius: f64, start_angle: f64, end_angle: f64) -> Self {
        Self {
            center,
            radius,
            start_angle,
            end_angle,
        }
    }

    /// Get the start point.
    pub fn start_point(&self) -> Point2D {
        Point2D::new(
            self.center.x + self.radius * self.start_angle.cos(),
            self.center.y + self.radius * self.start_angle.sin(),
        )
    }

    /// Get the end point.
    pub fn end_point(&self) -> Point2D {
        Point2D::new(
            self.center.x + self.radius * self.end_angle.cos(),
            self.center.y + self.radius * self.end_angle.sin(),
        )
    }
}

/// Text anchor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAnchor {
    #[default]
    Start,
    Middle,
    End,
}

/// 2D text label.
#[derive(Debug, Clone)]
pub struct Text2D {
    pub position: Point2D,
    pub text: String,
    pub font_size: f64,
    pub anchor: TextAnchor,
}

impl Text2D {
    /// Create a new text label.
    pub fn new(position: Point2D, text: impl Into<String>, font_size: f64) -> Self {
        Self {
            position,
            text: text.into(),
            font_size,
            anchor: TextAnchor::default(),
        }
    }

    /// Set the anchor.
    pub fn with_anchor(mut self, anchor: TextAnchor) -> Self {
        self.anchor = anchor;
        self
    }
}

/// 2D primitive types for floor plan generation.
#[derive(Debug, Clone)]
pub enum G2DPrimitive {
    /// Point set
    Points(Vec<Point2D>),
    /// Individual line segments
    Lines(Vec<Line2D>),
    /// Connected line strip
    LineStrip(Vec<Point2D>),
    /// Closed line loop
    LineLoop(Vec<Point2D>),
    /// Convex polygon (filled)
    Polygon(Vec<Point2D>),
    /// Rectangle
    Rectangle(Rect2D),
    /// Circle
    Circle(Circle2D),
    /// Ellipse
    Ellipse(Ellipse2D),
    /// Arc
    Arc(Arc2D),
    /// Text label
    Text(Text2D),
}

/// Line style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

impl LineStyle {
    /// Get SVG dash array.
    pub fn to_svg_dasharray(&self) -> Option<&'static str> {
        match self {
            LineStyle::Solid => None,
            LineStyle::Dashed => Some("10,5"),
            LineStyle::Dotted => Some("2,2"),
            LineStyle::DashDot => Some("10,2,2,2"),
        }
    }
}

/// Fill style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillStyle {
    #[default]
    None,
    Solid,
    Hatched,
}

/// Visual attributes for 2D rendering.
#[derive(Debug, Clone)]
pub struct G2DAttributes {
    /// Stroke/line color [R, G, B, A]
    pub color: [f32; 4],
    /// Fill color (None = no fill)
    pub fill_color: Option<[f32; 4]>,
    /// Line width in mm
    pub line_width: f32,
    /// Line style
    pub line_style: LineStyle,
    /// Fill style
    pub fill_style: FillStyle,
    /// Layer name for visibility control
    pub layer: String,
}

impl G2DAttributes {
    /// Create new attributes with default values.
    pub fn new() -> Self {
        Self {
            color: [0.0, 0.0, 0.0, 1.0],
            fill_color: None,
            line_width: 1.0,
            line_style: LineStyle::Solid,
            fill_style: FillStyle::None,
            layer: "default".to_string(),
        }
    }

    /// Set stroke color.
    pub fn with_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = [r, g, b, a];
        self
    }

    /// Set fill color.
    pub fn with_fill(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.fill_color = Some([r, g, b, a]);
        self.fill_style = FillStyle::Solid;
        self
    }

    /// Set line width.
    pub fn with_line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// Set line style.
    pub fn with_line_style(mut self, style: LineStyle) -> Self {
        self.line_style = style;
        self
    }

    /// Set layer.
    pub fn with_layer(mut self, layer: impl Into<String>) -> Self {
        self.layer = layer.into();
        self
    }

    /// Get SVG stroke color string.
    pub fn svg_stroke(&self) -> String {
        format!(
            "rgb({},{},{})",
            (self.color[0] * 255.0) as u8,
            (self.color[1] * 255.0) as u8,
            (self.color[2] * 255.0) as u8
        )
    }

    /// Get SVG fill color string.
    pub fn svg_fill(&self) -> String {
        match self.fill_color {
            Some(c) => format!(
                "rgb({},{},{})",
                (c[0] * 255.0) as u8,
                (c[1] * 255.0) as u8,
                (c[2] * 255.0) as u8
            ),
            None => "none".to_string(),
        }
    }
}

impl Default for G2DAttributes {
    fn default() -> Self {
        Self::new()
    }
}

/// 2D affine transform.
#[derive(Debug, Clone)]
pub struct Transform2D {
    /// Translation [tx, ty]
    pub translation: [f64; 2],
    /// Rotation angle in radians
    pub rotation: f64,
    /// Scale factors [sx, sy]
    pub scale: [f64; 2],
}

impl Transform2D {
    /// Create an identity transform.
    pub fn identity() -> Self {
        Self::default()
    }

    /// Create a translation transform.
    pub fn translate(x: f64, y: f64) -> Self {
        Self {
            translation: [x, y],
            rotation: 0.0,
            scale: [1.0, 1.0],
        }
    }

    /// Create a rotation transform.
    pub fn rotate(angle: f64) -> Self {
        Self {
            translation: [0.0, 0.0],
            rotation: angle,
            scale: [1.0, 1.0],
        }
    }

    /// Create a scale transform.
    pub fn scale(sx: f64, sy: f64) -> Self {
        Self {
            translation: [0.0, 0.0],
            rotation: 0.0,
            scale: [sx, sy],
        }
    }

    /// Compose with another transform.
    pub fn then(&self, other: &Transform2D) -> Transform2D {
        // Simplified composition: apply self, then other
        Transform2D {
            translation: [
                self.translation[0] + other.translation[0],
                self.translation[1] + other.translation[1],
            ],
            rotation: self.rotation + other.rotation,
            scale: [
                self.scale[0] * other.scale[0],
                self.scale[1] * other.scale[1],
            ],
        }
    }

    /// Get SVG transform string.
    pub fn to_svg(&self) -> String {
        let mut parts = Vec::new();

        if self.translation[0] != 0.0 || self.translation[1] != 0.0 {
            parts.push(format!(
                "translate({:.3},{:.3})",
                self.translation[0], self.translation[1]
            ));
        }

        if self.rotation != 0.0 {
            parts.push(format!("rotate({:.3})", self.rotation * 180.0 / PI));
        }

        if self.scale[0] != 1.0 || self.scale[1] != 1.0 {
            parts.push(format!("scale({:.3},{:.3})", self.scale[0], self.scale[1]));
        }

        parts.join(" ")
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            translation: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
        }
    }
}

/// Compound 2D object containing multiple primitives.
#[derive(Debug, Clone)]
pub struct G2DCompound {
    /// Child primitives
    pub primitives: Vec<G2DPrimitive>,
    /// Child compounds (nested groups)
    pub children: Vec<G2DCompound>,
    /// Local transform
    pub transform: Transform2D,
    /// Visual attributes (inherited by children unless overridden)
    pub attributes: G2DAttributes,
}

impl G2DCompound {
    /// Create a new empty compound.
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            children: Vec::new(),
            transform: Transform2D::default(),
            attributes: G2DAttributes::default(),
        }
    }

    /// Add a primitive.
    pub fn add_primitive(&mut self, primitive: G2DPrimitive) {
        self.primitives.push(primitive);
    }

    /// Add a child compound.
    pub fn add_child(&mut self, child: G2DCompound) {
        self.children.push(child);
    }

    /// Set the transform.
    pub fn with_transform(mut self, transform: Transform2D) -> Self {
        self.transform = transform;
        self
    }

    /// Set the attributes.
    pub fn with_attributes(mut self, attributes: G2DAttributes) -> Self {
        self.attributes = attributes;
        self
    }

    /// Export to SVG string.
    pub fn to_svg(&self) -> String {
        let mut svg = String::new();
        svg.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        svg.push('\n');
        svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1">"#);
        svg.push('\n');

        self.write_svg_content(&mut svg, &Transform2D::default());

        svg.push_str("</svg>\n");
        svg
    }

    /// Write SVG content recursively.
    fn write_svg_content(&self, svg: &mut String, parent_transform: &Transform2D) {
        let combined = parent_transform.then(&self.transform);
        let transform_attr = combined.to_svg();
        let group_start = if transform_attr.is_empty() {
            "<g>".to_string()
        } else {
            format!(r#"<g transform="{}">"#, transform_attr)
        };

        svg.push_str(&group_start);
        svg.push('\n');

        // Write primitives
        for primitive in &self.primitives {
            self.write_primitive_svg(svg, primitive);
        }

        // Write children
        for child in &self.children {
            child.write_svg_content(svg, &combined);
        }

        svg.push_str("</g>\n");
    }

    /// Write a single primitive to SVG.
    fn write_primitive_svg(&self, svg: &mut String, primitive: &G2DPrimitive) {
        let stroke = self.attributes.svg_stroke();
        let fill = self.attributes.svg_fill();
        let width = self.attributes.line_width;
        let dash = self.attributes.line_style.to_svg_dasharray();
        let dash_attr = dash
            .map(|d| format!(r#" stroke-dasharray="{}""#, d))
            .unwrap_or_default();

        match primitive {
            G2DPrimitive::Points(points) => {
                for p in points {
                    svg.push_str(&format!(
                        r#"  <circle cx="{:.3}" cy="{:.3}" r="1" fill="{}" />"#,
                        p.x, p.y, stroke
                    ));
                    svg.push('\n');
                }
            }
            G2DPrimitive::Lines(lines) => {
                for line in lines {
                    svg.push_str(&format!(
                        r#"  <line x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" stroke="{}" stroke-width="{:.2}"{} />"#,
                        line.start.x, line.start.y, line.end.x, line.end.y, stroke, width, dash_attr
                    ));
                    svg.push('\n');
                }
            }
            G2DPrimitive::LineStrip(points) => {
                if points.len() >= 2 {
                    let path_data: String = points
                        .iter()
                        .enumerate()
                        .map(|(i, p)| {
                            if i == 0 {
                                format!("M{:.3},{:.3}", p.x, p.y)
                            } else {
                                format!("L{:.3},{:.3}", p.x, p.y)
                            }
                        })
                        .collect();
                    svg.push_str(&format!(
                        r#"  <path d="{}" fill="none" stroke="{}" stroke-width="{:.2}"{} />"#,
                        path_data, stroke, width, dash_attr
                    ));
                    svg.push('\n');
                }
            }
            G2DPrimitive::LineLoop(points) => {
                if points.len() >= 3 {
                    let path_data: String = points
                        .iter()
                        .enumerate()
                        .map(|(i, p)| {
                            if i == 0 {
                                format!("M{:.3},{:.3}", p.x, p.y)
                            } else {
                                format!("L{:.3},{:.3}", p.x, p.y)
                            }
                        })
                        .collect::<String>()
                        + "Z";
                    svg.push_str(&format!(
                        r#"  <path d="{}" fill="none" stroke="{}" stroke-width="{:.2}"{} />"#,
                        path_data, stroke, width, dash_attr
                    ));
                    svg.push('\n');
                }
            }
            G2DPrimitive::Polygon(points) => {
                if points.len() >= 3 {
                    let points_str: String = points
                        .iter()
                        .map(|p| format!("{:.3},{:.3}", p.x, p.y))
                        .collect::<Vec<_>>()
                        .join(" ");
                    svg.push_str(&format!(
                        r#"  <polygon points="{}" fill="{}" stroke="{}" stroke-width="{:.2}" />"#,
                        points_str, fill, stroke, width
                    ));
                    svg.push('\n');
                }
            }
            G2DPrimitive::Rectangle(rect) => {
                svg.push_str(&format!(
                    r#"  <rect x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" fill="{}" stroke="{}" stroke-width="{:.2}"{} />"#,
                    rect.x, rect.y, rect.width, rect.height, fill, stroke, width, dash_attr
                ));
                svg.push('\n');
            }
            G2DPrimitive::Circle(circle) => {
                svg.push_str(&format!(
                    r#"  <circle cx="{:.3}" cy="{:.3}" r="{:.3}" fill="{}" stroke="{}" stroke-width="{:.2}" />"#,
                    circle.center.x, circle.center.y, circle.radius, fill, stroke, width
                ));
                svg.push('\n');
            }
            G2DPrimitive::Ellipse(ellipse) => {
                svg.push_str(&format!(
                    r#"  <ellipse cx="{:.3}" cy="{:.3}" rx="{:.3}" ry="{:.3}" fill="{}" stroke="{}" stroke-width="{:.2}" />"#,
                    ellipse.center.x, ellipse.center.y, ellipse.rx, ellipse.ry, fill, stroke, width
                ));
                svg.push('\n');
            }
            G2DPrimitive::Arc(arc) => {
                let start = arc.start_point();
                let end = arc.end_point();
                let large_arc = if (arc.end_angle - arc.start_angle).abs() > PI {
                    1
                } else {
                    0
                };
                let sweep = if arc.end_angle > arc.start_angle {
                    1
                } else {
                    0
                };

                svg.push_str(&format!(
                    r#"  <path d="M{:.3},{:.3} A{:.3},{:.3} 0 {} {} {:.3},{:.3}" fill="none" stroke="{}" stroke-width="{:.2}" />"#,
                    start.x, start.y, arc.radius, arc.radius, large_arc, sweep, end.x, end.y, stroke, width
                ));
                svg.push('\n');
            }
            G2DPrimitive::Text(text) => {
                let anchor = match text.anchor {
                    TextAnchor::Start => "start",
                    TextAnchor::Middle => "middle",
                    TextAnchor::End => "end",
                };
                svg.push_str(&format!(
                    r#"  <text x="{:.3}" y="{:.3}" font-size="{:.1}" text-anchor="{}" fill="{}">{}</text>"#,
                    text.position.x, text.position.y, text.font_size, anchor, stroke, text.text
                ));
                svg.push('\n');
            }
        }
    }

    /// Calculate bounding box.
    pub fn bounding_box(&self) -> Option<(Point2D, Point2D)> {
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut has_points = false;

        fn update_bounds(
            x: f64,
            y: f64,
            min_x: &mut f64,
            min_y: &mut f64,
            max_x: &mut f64,
            max_y: &mut f64,
            has_points: &mut bool,
        ) {
            *min_x = min_x.min(x);
            *min_y = min_y.min(y);
            *max_x = max_x.max(x);
            *max_y = max_y.max(y);
            *has_points = true;
        }

        for primitive in &self.primitives {
            match primitive {
                G2DPrimitive::Points(points)
                | G2DPrimitive::LineStrip(points)
                | G2DPrimitive::LineLoop(points)
                | G2DPrimitive::Polygon(points) => {
                    for p in points {
                        update_bounds(
                            p.x,
                            p.y,
                            &mut min_x,
                            &mut min_y,
                            &mut max_x,
                            &mut max_y,
                            &mut has_points,
                        );
                    }
                }
                G2DPrimitive::Lines(lines) => {
                    for line in lines {
                        update_bounds(
                            line.start.x,
                            line.start.y,
                            &mut min_x,
                            &mut min_y,
                            &mut max_x,
                            &mut max_y,
                            &mut has_points,
                        );
                        update_bounds(
                            line.end.x,
                            line.end.y,
                            &mut min_x,
                            &mut min_y,
                            &mut max_x,
                            &mut max_y,
                            &mut has_points,
                        );
                    }
                }
                G2DPrimitive::Rectangle(rect) => {
                    update_bounds(
                        rect.x,
                        rect.y,
                        &mut min_x,
                        &mut min_y,
                        &mut max_x,
                        &mut max_y,
                        &mut has_points,
                    );
                    update_bounds(
                        rect.x + rect.width,
                        rect.y + rect.height,
                        &mut min_x,
                        &mut min_y,
                        &mut max_x,
                        &mut max_y,
                        &mut has_points,
                    );
                }
                G2DPrimitive::Circle(circle) => {
                    update_bounds(
                        circle.center.x - circle.radius,
                        circle.center.y - circle.radius,
                        &mut min_x,
                        &mut min_y,
                        &mut max_x,
                        &mut max_y,
                        &mut has_points,
                    );
                    update_bounds(
                        circle.center.x + circle.radius,
                        circle.center.y + circle.radius,
                        &mut min_x,
                        &mut min_y,
                        &mut max_x,
                        &mut max_y,
                        &mut has_points,
                    );
                }
                G2DPrimitive::Ellipse(ellipse) => {
                    update_bounds(
                        ellipse.center.x - ellipse.rx,
                        ellipse.center.y - ellipse.ry,
                        &mut min_x,
                        &mut min_y,
                        &mut max_x,
                        &mut max_y,
                        &mut has_points,
                    );
                    update_bounds(
                        ellipse.center.x + ellipse.rx,
                        ellipse.center.y + ellipse.ry,
                        &mut min_x,
                        &mut min_y,
                        &mut max_x,
                        &mut max_y,
                        &mut has_points,
                    );
                }
                G2DPrimitive::Arc(arc) => {
                    update_bounds(
                        arc.center.x - arc.radius,
                        arc.center.y - arc.radius,
                        &mut min_x,
                        &mut min_y,
                        &mut max_x,
                        &mut max_y,
                        &mut has_points,
                    );
                    update_bounds(
                        arc.center.x + arc.radius,
                        arc.center.y + arc.radius,
                        &mut min_x,
                        &mut min_y,
                        &mut max_x,
                        &mut max_y,
                        &mut has_points,
                    );
                }
                G2DPrimitive::Text(text) => {
                    update_bounds(
                        text.position.x,
                        text.position.y,
                        &mut min_x,
                        &mut min_y,
                        &mut max_x,
                        &mut max_y,
                        &mut has_points,
                    );
                }
            }
        }

        if has_points {
            Some((Point2D::new(min_x, min_y), Point2D::new(max_x, max_y)))
        } else {
            None
        }
    }
}

impl Default for G2DCompound {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert an Odb2dRecord to a G2DPrimitive.
///
/// This handles the conversion of EBASE odb2d records into 2D geometry primitives
/// for floor plan rendering and SVG export.
pub fn odb2d_to_primitive(record: &crate::ebase::Odb2dRecord) -> Option<G2DPrimitive> {
    let prim_type = record.prim_type.to_lowercase();
    let x = record.parse_x_coords();
    let y = record.parse_y_coords();

    match prim_type.as_str() {
        "point" | "points" => {
            let points: Vec<Point2D> = x
                .iter()
                .zip(y.iter())
                .map(|(&x, &y)| Point2D::new(x, y))
                .collect();
            if points.is_empty() {
                None
            } else {
                Some(G2DPrimitive::Points(points))
            }
        }
        "line" | "lines" => {
            // Lines come in pairs of points
            if x.len() < 2 || y.len() < 2 {
                return None;
            }
            let mut lines = Vec::new();
            for i in (0..x.len().min(y.len())).step_by(2) {
                if i + 1 < x.len() && i + 1 < y.len() {
                    lines.push(Line2D::from_coords(x[i], y[i], x[i + 1], y[i + 1]));
                }
            }
            if lines.is_empty() {
                None
            } else {
                Some(G2DPrimitive::Lines(lines))
            }
        }
        "linestrip" | "line_strip" | "polyline" => {
            let points: Vec<Point2D> = x
                .iter()
                .zip(y.iter())
                .map(|(&x, &y)| Point2D::new(x, y))
                .collect();
            if points.len() < 2 {
                None
            } else {
                Some(G2DPrimitive::LineStrip(points))
            }
        }
        "lineloop" | "line_loop" | "closed_polyline" => {
            let points: Vec<Point2D> = x
                .iter()
                .zip(y.iter())
                .map(|(&x, &y)| Point2D::new(x, y))
                .collect();
            if points.len() < 3 {
                None
            } else {
                Some(G2DPrimitive::LineLoop(points))
            }
        }
        "polygon" | "poly" | "filled_polygon" => {
            let points: Vec<Point2D> = x
                .iter()
                .zip(y.iter())
                .map(|(&x, &y)| Point2D::new(x, y))
                .collect();
            if points.len() < 3 {
                None
            } else {
                Some(G2DPrimitive::Polygon(points))
            }
        }
        "rect" | "rectangle" => {
            let px = x.first().copied().unwrap_or(0.0);
            let py = y.first().copied().unwrap_or(0.0);
            let width = record.parse_width();
            let height = record.parse_height();
            Some(G2DPrimitive::Rectangle(Rect2D::new(px, py, width, height)))
        }
        "circle" => {
            let cx = x.first().copied().unwrap_or(0.0);
            let cy = y.first().copied().unwrap_or(0.0);
            let radius = record.parse_radius();
            Some(G2DPrimitive::Circle(Circle2D::from_coords(cx, cy, radius)))
        }
        "ellipse" => {
            let cx = x.first().copied().unwrap_or(0.0);
            let cy = y.first().copied().unwrap_or(0.0);
            let rx = record.parse_width() / 2.0;
            let ry = record.parse_height() / 2.0;
            Some(G2DPrimitive::Ellipse(Ellipse2D::new(
                Point2D::new(cx, cy),
                rx,
                ry,
            )))
        }
        "arc" => {
            let cx = x.first().copied().unwrap_or(0.0);
            let cy = y.first().copied().unwrap_or(0.0);
            let radius = record.parse_radius();
            let start_angle = record.parse_start_angle().to_radians();
            let end_angle = record.parse_end_angle().to_radians();
            Some(G2DPrimitive::Arc(Arc2D::new(
                Point2D::new(cx, cy),
                radius,
                start_angle,
                end_angle,
            )))
        }
        "text" | "label" => {
            let px = x.first().copied().unwrap_or(0.0);
            let py = y.first().copied().unwrap_or(0.0);
            let text = &record.text;
            let font_size = record.parse_font_size();
            Some(G2DPrimitive::Text(Text2D::new(
                Point2D::new(px, py),
                text.clone(),
                font_size,
            )))
        }
        _ => None,
    }
}

/// Get G2DAttributes from an Odb2dRecord.
pub fn odb2d_to_attributes(record: &crate::ebase::Odb2dRecord) -> G2DAttributes {
    let color = record.parse_color();
    let fill_color = record.parse_fill_color();
    let line_width = record.parse_line_width();
    let line_style = match record.get_line_style() {
        "dashed" => LineStyle::Dashed,
        "dotted" => LineStyle::Dotted,
        "dashdot" => LineStyle::DashDot,
        _ => LineStyle::Solid,
    };

    let mut attrs = G2DAttributes::new()
        .with_color(color[0], color[1], color[2], color[3])
        .with_line_width(line_width)
        .with_line_style(line_style)
        .with_layer(&record.layer);

    if let Some(fill) = fill_color {
        attrs = attrs.with_fill(fill[0], fill[1], fill[2], fill[3]);
    }

    attrs
}

/// Process multiple Odb2dRecords into a G2DCompound.
///
/// Groups primitives by layer and applies attributes.
pub fn process_odb2d_records(records: &[crate::ebase::Odb2dRecord]) -> G2DCompound {
    let mut compound = G2DCompound::new();

    for record in records {
        if let Some(primitive) = odb2d_to_primitive(record) {
            // Create a sub-compound for each primitive with its attributes
            let attrs = odb2d_to_attributes(record);
            let mut prim_compound = G2DCompound::new().with_attributes(attrs);
            prim_compound.add_primitive(primitive);
            compound.add_child(prim_compound);
        }
    }

    compound
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        assert!((p1.distance(&p2) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_point_transform() {
        let p = Point2D::new(1.0, 0.0);
        let t = Transform2D::rotate(std::f64::consts::FRAC_PI_2);
        let tp = p.transform(&t);
        assert!(tp.x.abs() < 0.001);
        assert!((tp.y - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_rect2d() {
        let rect = Rect2D::new(0.0, 0.0, 100.0, 50.0);
        let center = rect.center();
        assert_eq!(center.x, 50.0);
        assert_eq!(center.y, 25.0);

        let corners = rect.corners();
        assert_eq!(corners.len(), 4);
    }

    #[test]
    fn test_transform2d() {
        let t = Transform2D::translate(10.0, 20.0);
        assert_eq!(t.translation, [10.0, 20.0]);

        let svg = t.to_svg();
        assert!(svg.contains("translate"));
    }

    #[test]
    fn test_g2d_compound() {
        let mut compound = G2DCompound::new();

        compound.add_primitive(G2DPrimitive::Rectangle(Rect2D::new(0.0, 0.0, 100.0, 50.0)));
        compound.add_primitive(G2DPrimitive::Circle(Circle2D::from_coords(
            50.0, 25.0, 10.0,
        )));

        let svg = compound.to_svg();
        assert!(svg.contains("<rect"));
        assert!(svg.contains("<circle"));
    }

    #[test]
    fn test_g2d_bounding_box() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Rectangle(Rect2D::new(
            10.0, 20.0, 100.0, 50.0,
        )));

        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 10.0);
        assert_eq!(bbox.0.y, 20.0);
        assert_eq!(bbox.1.x, 110.0);
        assert_eq!(bbox.1.y, 70.0);
    }

    #[test]
    fn test_g2d_attributes() {
        let attrs = G2DAttributes::new()
            .with_color(1.0, 0.0, 0.0, 1.0)
            .with_fill(0.0, 1.0, 0.0, 1.0)
            .with_line_width(2.0)
            .with_line_style(LineStyle::Dashed);

        assert_eq!(attrs.svg_stroke(), "rgb(255,0,0)");
        assert_eq!(attrs.svg_fill(), "rgb(0,255,0)");
        assert_eq!(attrs.line_style.to_svg_dasharray(), Some("10,5"));
    }

    #[test]
    fn test_line_style() {
        assert_eq!(LineStyle::Solid.to_svg_dasharray(), None);
        assert_eq!(LineStyle::Dashed.to_svg_dasharray(), Some("10,5"));
        assert_eq!(LineStyle::Dotted.to_svg_dasharray(), Some("2,2"));
    }

    #[test]
    fn test_odb2d_to_polygon() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert(
            "prim_type".to_string(),
            Value::String("polygon".to_string()),
        );
        record_map.insert(
            "x_coords".to_string(),
            Value::String("0,100,100,0".to_string()),
        );
        record_map.insert(
            "y_coords".to_string(),
            Value::String("0,0,50,50".to_string()),
        );

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::Polygon(points) = primitive {
            assert_eq!(points.len(), 4);
            assert_eq!(points[0].x, 0.0);
            assert_eq!(points[1].x, 100.0);
            assert_eq!(points[2].y, 50.0);
        } else {
            panic!("Expected Polygon primitive");
        }
    }

    #[test]
    fn test_odb2d_to_rect() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("rect".to_string()));
        record_map.insert("x".to_string(), Value::String("10".to_string()));
        record_map.insert("y".to_string(), Value::String("20".to_string()));
        record_map.insert("width".to_string(), Value::String("100".to_string()));
        record_map.insert("height".to_string(), Value::String("50".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::Rectangle(rect) = primitive {
            assert_eq!(rect.x, 10.0);
            assert_eq!(rect.y, 20.0);
            assert_eq!(rect.width, 100.0);
            assert_eq!(rect.height, 50.0);
        } else {
            panic!("Expected Rectangle primitive");
        }
    }

    #[test]
    fn test_odb2d_to_circle() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("circle".to_string()));
        record_map.insert("x".to_string(), Value::String("50".to_string()));
        record_map.insert("y".to_string(), Value::String("50".to_string()));
        record_map.insert("r".to_string(), Value::String("25".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::Circle(circle) = primitive {
            assert_eq!(circle.center.x, 50.0);
            assert_eq!(circle.center.y, 50.0);
            assert_eq!(circle.radius, 25.0);
        } else {
            panic!("Expected Circle primitive");
        }
    }

    #[test]
    fn test_odb2d_to_text() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("text".to_string()));
        record_map.insert("x".to_string(), Value::String("10".to_string()));
        record_map.insert("y".to_string(), Value::String("20".to_string()));
        record_map.insert("text".to_string(), Value::String("Hello".to_string()));
        record_map.insert("font_size".to_string(), Value::String("12".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::Text(text) = primitive {
            assert_eq!(text.position.x, 10.0);
            assert_eq!(text.text, "Hello");
            assert_eq!(text.font_size, 12.0);
        } else {
            panic!("Expected Text primitive");
        }
    }

    #[test]
    fn test_odb2d_to_lines() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("lines".to_string()));
        record_map.insert(
            "x_coords".to_string(),
            Value::String("0,100,50,150".to_string()),
        );
        record_map.insert(
            "y_coords".to_string(),
            Value::String("0,0,50,50".to_string()),
        );

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::Lines(lines) = primitive {
            assert_eq!(lines.len(), 2);
            assert_eq!(lines[0].start.x, 0.0);
            assert_eq!(lines[0].end.x, 100.0);
            assert_eq!(lines[1].start.x, 50.0);
            assert_eq!(lines[1].end.x, 150.0);
        } else {
            panic!("Expected Lines primitive");
        }
    }

    #[test]
    fn test_odb2d_to_attributes() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("rect".to_string()));
        record_map.insert(
            "color".to_string(),
            Value::String("0.8 0.4 0.2".to_string()),
        );
        record_map.insert(
            "fill_color".to_string(),
            Value::String("0.1 0.9 0.1".to_string()),
        );
        record_map.insert("line_width".to_string(), Value::String("2.5".to_string()));
        record_map.insert(
            "line_style".to_string(),
            Value::String("dashed".to_string()),
        );
        record_map.insert("layer".to_string(), Value::String("floor".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let attrs = odb2d_to_attributes(&odb_record);

        assert!((attrs.color[0] - 0.8).abs() < 0.001);
        assert!((attrs.color[1] - 0.4).abs() < 0.001);
        assert!(attrs.fill_color.is_some());
        let fill = attrs.fill_color.unwrap();
        assert!((fill[1] - 0.9).abs() < 0.001);
        assert!((attrs.line_width - 2.5).abs() < 0.001);
        assert_eq!(attrs.line_style, LineStyle::Dashed);
        assert_eq!(attrs.layer, "floor");
    }

    #[test]
    fn test_process_odb2d_records() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut records = Vec::new();

        // Add a rectangle
        let mut r1 = Record::new();
        r1.insert("prim_type".to_string(), Value::String("rect".to_string()));
        r1.insert("x".to_string(), Value::String("0".to_string()));
        r1.insert("y".to_string(), Value::String("0".to_string()));
        r1.insert("width".to_string(), Value::String("100".to_string()));
        r1.insert("height".to_string(), Value::String("50".to_string()));
        records.push(Odb2dRecord::from_record(&r1).unwrap());

        // Add a circle
        let mut r2 = Record::new();
        r2.insert("prim_type".to_string(), Value::String("circle".to_string()));
        r2.insert("x".to_string(), Value::String("50".to_string()));
        r2.insert("y".to_string(), Value::String("25".to_string()));
        r2.insert("r".to_string(), Value::String("10".to_string()));
        records.push(Odb2dRecord::from_record(&r2).unwrap());

        let compound = process_odb2d_records(&records);
        assert_eq!(compound.children.len(), 2);

        // Verify SVG export works
        let svg = compound.to_svg();
        assert!(svg.contains("<rect"));
        assert!(svg.contains("<circle"));
    }

    // ========== Additional Coverage Tests ==========

    #[test]
    fn test_point2d_origin() {
        let origin = Point2D::origin();
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
    }

    #[test]
    fn test_point2d_debug_clone() {
        let p = Point2D::new(1.0, 2.0);
        let debug_str = format!("{:?}", p);
        assert!(debug_str.contains("Point2D"));

        let cloned = p.clone();
        assert_eq!(cloned.x, 1.0);
        assert_eq!(cloned.y, 2.0);
    }

    #[test]
    fn test_point2d_partial_eq() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(1.0, 2.0);
        let p3 = Point2D::new(3.0, 4.0);
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_line2d_new() {
        let start = Point2D::new(0.0, 0.0);
        let end = Point2D::new(10.0, 10.0);
        let line = Line2D::new(start, end);
        assert_eq!(line.start.x, 0.0);
        assert_eq!(line.end.x, 10.0);
    }

    #[test]
    fn test_line2d_from_coords() {
        let line = Line2D::from_coords(1.0, 2.0, 3.0, 4.0);
        assert_eq!(line.start.x, 1.0);
        assert_eq!(line.start.y, 2.0);
        assert_eq!(line.end.x, 3.0);
        assert_eq!(line.end.y, 4.0);
    }

    #[test]
    fn test_line2d_length() {
        let line = Line2D::from_coords(0.0, 0.0, 3.0, 4.0);
        assert!((line.length() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_line2d_debug_clone() {
        let line = Line2D::from_coords(0.0, 0.0, 1.0, 1.0);
        let debug_str = format!("{:?}", line);
        assert!(debug_str.contains("Line2D"));

        let cloned = line.clone();
        assert_eq!(cloned.start.x, 0.0);
    }

    #[test]
    fn test_rect2d_new() {
        let rect = Rect2D::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
    }

    #[test]
    fn test_rect2d_corners() {
        let rect = Rect2D::new(10.0, 20.0, 100.0, 50.0);
        let corners = rect.corners();
        assert_eq!(corners[0].x, 10.0);
        assert_eq!(corners[0].y, 20.0);
        assert_eq!(corners[2].x, 110.0);
        assert_eq!(corners[2].y, 70.0);
    }

    #[test]
    fn test_rect2d_debug_clone() {
        let rect = Rect2D::new(0.0, 0.0, 10.0, 10.0);
        let debug_str = format!("{:?}", rect);
        assert!(debug_str.contains("Rect2D"));

        let cloned = rect.clone();
        assert_eq!(cloned.width, 10.0);
    }

    #[test]
    fn test_circle2d_new() {
        let circle = Circle2D::new(Point2D::new(50.0, 50.0), 25.0);
        assert_eq!(circle.center.x, 50.0);
        assert_eq!(circle.radius, 25.0);
    }

    #[test]
    fn test_circle2d_from_coords() {
        let circle = Circle2D::from_coords(100.0, 200.0, 30.0);
        assert_eq!(circle.center.x, 100.0);
        assert_eq!(circle.center.y, 200.0);
        assert_eq!(circle.radius, 30.0);
    }

    #[test]
    fn test_circle2d_debug_clone() {
        let circle = Circle2D::from_coords(0.0, 0.0, 10.0);
        let debug_str = format!("{:?}", circle);
        assert!(debug_str.contains("Circle2D"));

        let cloned = circle.clone();
        assert_eq!(cloned.radius, 10.0);
    }

    #[test]
    fn test_arc2d_new() {
        let arc = Arc2D::new(Point2D::new(50.0, 50.0), 25.0, 0.0, PI / 2.0);
        assert_eq!(arc.center.x, 50.0);
        assert_eq!(arc.radius, 25.0);
        assert_eq!(arc.start_angle, 0.0);
        assert!((arc.end_angle - PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_arc2d_debug_clone() {
        let arc = Arc2D::new(Point2D::origin(), 10.0, 0.0, PI);
        let debug_str = format!("{:?}", arc);
        assert!(debug_str.contains("Arc2D"));

        let cloned = arc.clone();
        assert_eq!(cloned.radius, 10.0);
    }

    #[test]
    fn test_text2d_new() {
        let text = Text2D::new(Point2D::new(10.0, 20.0), "Hello", 12.0);
        assert_eq!(text.text, "Hello");
        assert_eq!(text.position.x, 10.0);
        assert_eq!(text.font_size, 12.0);
    }

    #[test]
    fn test_text2d_debug_clone() {
        let text = Text2D::new(Point2D::origin(), "Test", 14.0);
        let debug_str = format!("{:?}", text);
        assert!(debug_str.contains("Text2D"));

        let cloned = text.clone();
        assert_eq!(cloned.text, "Test");
    }

    #[test]
    fn test_transform2d_identity() {
        let t = Transform2D::identity();
        assert_eq!(t.translation, [0.0, 0.0]);
        assert_eq!(t.rotation, 0.0);
        assert_eq!(t.scale, [1.0, 1.0]);
    }

    #[test]
    fn test_transform2d_translate() {
        let t = Transform2D::translate(10.0, 20.0);
        assert_eq!(t.translation, [10.0, 20.0]);
        assert_eq!(t.rotation, 0.0);
    }

    #[test]
    fn test_transform2d_rotate() {
        let t = Transform2D::rotate(PI / 2.0);
        assert!((t.rotation - PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_transform2d_scale() {
        let t = Transform2D::scale(2.0, 3.0);
        assert_eq!(t.scale, [2.0, 3.0]);
    }

    #[test]
    fn test_transform2d_then() {
        let t1 = Transform2D::translate(10.0, 0.0);
        let t2 = Transform2D::translate(0.0, 20.0);
        let combined = t1.then(&t2);
        assert_eq!(combined.translation, [10.0, 20.0]);
    }

    #[test]
    fn test_transform2d_debug_clone() {
        let t = Transform2D::translate(5.0, 5.0);
        let debug_str = format!("{:?}", t);
        assert!(debug_str.contains("Transform2D"));

        let cloned = t.clone();
        assert_eq!(cloned.translation, [5.0, 5.0]);
    }

    #[test]
    fn test_g2d_attributes_default() {
        let attrs = G2DAttributes::default();
        assert_eq!(attrs.layer, "default");
    }

    #[test]
    fn test_g2d_attributes_debug_clone() {
        let attrs = G2DAttributes::new().with_line_width(3.0);
        let debug_str = format!("{:?}", attrs);
        assert!(debug_str.contains("G2DAttributes"));

        let cloned = attrs.clone();
        assert_eq!(cloned.line_width, 3.0);
    }

    #[test]
    fn test_line_style_dotted() {
        assert_eq!(LineStyle::Dotted.to_svg_dasharray(), Some("2,2"));
    }

    #[test]
    fn test_line_style_dash_dot() {
        assert_eq!(LineStyle::DashDot.to_svg_dasharray(), Some("10,2,2,2"));
    }

    #[test]
    fn test_line_style_debug_clone() {
        let style = LineStyle::Dashed;
        let debug_str = format!("{:?}", style);
        assert!(debug_str.contains("Dashed"));

        let cloned = style.clone();
        assert_eq!(cloned, LineStyle::Dashed);
    }

    #[test]
    fn test_g2d_primitive_debug_clone() {
        let prim = G2DPrimitive::Rectangle(Rect2D::new(0.0, 0.0, 10.0, 10.0));
        let debug_str = format!("{:?}", prim);
        assert!(debug_str.contains("Rectangle"));

        let cloned = prim.clone();
        if let G2DPrimitive::Rectangle(rect) = cloned {
            assert_eq!(rect.width, 10.0);
        }
    }

    #[test]
    fn test_g2d_compound_empty() {
        let compound = G2DCompound::new();
        assert!(compound.children.is_empty());
        assert!(compound.bounding_box().is_none());
    }

    #[test]
    fn test_g2d_compound_with_transform() {
        let t = Transform2D::translate(10.0, 20.0);
        let compound = G2DCompound::new().with_transform(t);
        assert_eq!(compound.transform.translation, [10.0, 20.0]);
    }

    #[test]
    fn test_g2d_compound_with_attributes() {
        let attrs = G2DAttributes::new().with_line_width(5.0);
        let compound = G2DCompound::new().with_attributes(attrs);
        assert_eq!(compound.attributes.line_width, 5.0);
    }

    #[test]
    fn test_g2d_compound_debug_clone() {
        let compound = G2DCompound::new();
        let debug_str = format!("{:?}", compound);
        assert!(debug_str.contains("G2DCompound"));

        let cloned = compound.clone();
        assert!(cloned.children.is_empty());
    }

    // ========== SVG Generation Tests for Uncovered Primitives ==========

    #[test]
    fn test_svg_points() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Points(vec![
            Point2D::new(10.0, 20.0),
            Point2D::new(30.0, 40.0),
        ]));
        let svg = compound.to_svg();
        assert!(svg.contains("<circle cx="));
        assert!(svg.contains("10.000"));
        assert!(svg.contains("20.000"));
    }

    #[test]
    fn test_svg_lines() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Lines(vec![
            Line2D::from_coords(0.0, 0.0, 100.0, 100.0),
            Line2D::from_coords(50.0, 0.0, 50.0, 100.0),
        ]));
        let svg = compound.to_svg();
        assert!(svg.contains("<line"));
        assert!(svg.contains("x1="));
        assert!(svg.contains("y2="));
    }

    #[test]
    fn test_svg_line_strip() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::LineStrip(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(50.0, 50.0),
            Point2D::new(100.0, 0.0),
        ]));
        let svg = compound.to_svg();
        assert!(svg.contains("<path"));
        assert!(svg.contains("M0.000,0.000"));
        assert!(svg.contains("L50.000,50.000"));
    }

    #[test]
    fn test_svg_line_loop() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::LineLoop(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(100.0, 0.0),
            Point2D::new(50.0, 100.0),
        ]));
        let svg = compound.to_svg();
        assert!(svg.contains("<path"));
        assert!(svg.contains("Z")); // Closed path
    }

    #[test]
    fn test_svg_polygon() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Polygon(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(100.0, 0.0),
            Point2D::new(100.0, 100.0),
            Point2D::new(0.0, 100.0),
        ]));
        let svg = compound.to_svg();
        assert!(svg.contains("<polygon"));
        assert!(svg.contains("points="));
    }

    #[test]
    fn test_svg_ellipse() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Ellipse(Ellipse2D::new(
            Point2D::new(50.0, 50.0),
            30.0,
            20.0,
        )));
        let svg = compound.to_svg();
        assert!(svg.contains("<ellipse"));
        assert!(svg.contains("cx=\"50.000\""));
        assert!(svg.contains("rx=\"30.000\""));
        assert!(svg.contains("ry=\"20.000\""));
    }

    #[test]
    fn test_svg_arc() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Arc(Arc2D::new(
            Point2D::new(50.0, 50.0),
            25.0,
            0.0,
            PI / 2.0,
        )));
        let svg = compound.to_svg();
        assert!(svg.contains("<path"));
        assert!(svg.contains(" A")); // Arc command
    }

    #[test]
    fn test_svg_arc_large() {
        // Test with a large arc (more than PI radians)
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Arc(Arc2D::new(
            Point2D::new(50.0, 50.0),
            25.0,
            0.0,
            PI * 1.5, // 270 degrees
        )));
        let svg = compound.to_svg();
        assert!(svg.contains("<path"));
    }

    #[test]
    fn test_svg_arc_negative_sweep() {
        // Test with negative sweep (end_angle < start_angle)
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Arc(Arc2D::new(
            Point2D::new(50.0, 50.0),
            25.0,
            PI / 2.0,
            0.0,
        )));
        let svg = compound.to_svg();
        assert!(svg.contains("<path"));
    }

    #[test]
    fn test_svg_text() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Text(
            Text2D::new(Point2D::new(10.0, 20.0), "Hello World", 14.0),
        ));
        let svg = compound.to_svg();
        assert!(svg.contains("<text"));
        assert!(svg.contains("Hello World"));
        assert!(svg.contains("font-size=\"14.0\""));
    }

    #[test]
    fn test_svg_text_anchor_middle() {
        let mut compound = G2DCompound::new();
        let text = Text2D::new(Point2D::new(50.0, 50.0), "Centered", 12.0)
            .with_anchor(TextAnchor::Middle);
        compound.add_primitive(G2DPrimitive::Text(text));
        let svg = compound.to_svg();
        assert!(svg.contains("text-anchor=\"middle\""));
    }

    #[test]
    fn test_svg_text_anchor_end() {
        let mut compound = G2DCompound::new();
        let text = Text2D::new(Point2D::new(100.0, 50.0), "Right", 12.0)
            .with_anchor(TextAnchor::End);
        compound.add_primitive(G2DPrimitive::Text(text));
        let svg = compound.to_svg();
        assert!(svg.contains("text-anchor=\"end\""));
    }

    #[test]
    fn test_svg_with_transform() {
        let transform = Transform2D::translate(100.0, 50.0);
        let compound = G2DCompound::new().with_transform(transform);
        let svg = compound.to_svg();
        assert!(svg.contains("translate(100.000,50.000)"));
    }

    #[test]
    fn test_svg_with_rotation() {
        let transform = Transform2D::rotate(PI / 4.0);
        let compound = G2DCompound::new().with_transform(transform);
        let svg = compound.to_svg();
        assert!(svg.contains("rotate("));
    }

    #[test]
    fn test_svg_with_scale() {
        let transform = Transform2D::scale(2.0, 3.0);
        let compound = G2DCompound::new().with_transform(transform);
        let svg = compound.to_svg();
        assert!(svg.contains("scale(2.000,3.000)"));
    }

    #[test]
    fn test_svg_dashed_line() {
        let attrs = G2DAttributes::new().with_line_style(LineStyle::Dashed);
        let mut compound = G2DCompound::new().with_attributes(attrs);
        compound.add_primitive(G2DPrimitive::Lines(vec![
            Line2D::from_coords(0.0, 0.0, 100.0, 100.0),
        ]));
        let svg = compound.to_svg();
        assert!(svg.contains("stroke-dasharray=\"10,5\""));
    }

    #[test]
    fn test_svg_nested_compound() {
        let mut child = G2DCompound::new();
        child.add_primitive(G2DPrimitive::Circle(Circle2D::from_coords(50.0, 50.0, 10.0)));

        let mut parent = G2DCompound::new();
        parent.add_child(child);

        let svg = parent.to_svg();
        assert!(svg.contains("<circle"));
    }

    // ========== Bounding Box Tests for Uncovered Primitives ==========

    #[test]
    fn test_bounding_box_points() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Points(vec![
            Point2D::new(10.0, 20.0),
            Point2D::new(30.0, 40.0),
        ]));
        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 10.0);
        assert_eq!(bbox.0.y, 20.0);
        assert_eq!(bbox.1.x, 30.0);
        assert_eq!(bbox.1.y, 40.0);
    }

    #[test]
    fn test_bounding_box_lines() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Lines(vec![
            Line2D::from_coords(0.0, 0.0, 100.0, 50.0),
        ]));
        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 0.0);
        assert_eq!(bbox.0.y, 0.0);
        assert_eq!(bbox.1.x, 100.0);
        assert_eq!(bbox.1.y, 50.0);
    }

    #[test]
    fn test_bounding_box_line_strip() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::LineStrip(vec![
            Point2D::new(0.0, 50.0),
            Point2D::new(50.0, 0.0),
            Point2D::new(100.0, 50.0),
        ]));
        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 0.0);
        assert_eq!(bbox.0.y, 0.0);
        assert_eq!(bbox.1.x, 100.0);
        assert_eq!(bbox.1.y, 50.0);
    }

    #[test]
    fn test_bounding_box_line_loop() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::LineLoop(vec![
            Point2D::new(10.0, 10.0),
            Point2D::new(90.0, 10.0),
            Point2D::new(50.0, 80.0),
        ]));
        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 10.0);
        assert_eq!(bbox.0.y, 10.0);
        assert_eq!(bbox.1.x, 90.0);
        assert_eq!(bbox.1.y, 80.0);
    }

    #[test]
    fn test_bounding_box_polygon() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Polygon(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(100.0, 0.0),
            Point2D::new(100.0, 100.0),
            Point2D::new(0.0, 100.0),
        ]));
        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 0.0);
        assert_eq!(bbox.0.y, 0.0);
        assert_eq!(bbox.1.x, 100.0);
        assert_eq!(bbox.1.y, 100.0);
    }

    #[test]
    fn test_bounding_box_circle() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Circle(Circle2D::from_coords(50.0, 50.0, 25.0)));
        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 25.0);
        assert_eq!(bbox.0.y, 25.0);
        assert_eq!(bbox.1.x, 75.0);
        assert_eq!(bbox.1.y, 75.0);
    }

    #[test]
    fn test_bounding_box_ellipse() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Ellipse(Ellipse2D::new(
            Point2D::new(50.0, 50.0),
            30.0,
            20.0,
        )));
        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 20.0); // 50 - 30
        assert_eq!(bbox.0.y, 30.0); // 50 - 20
        assert_eq!(bbox.1.x, 80.0); // 50 + 30
        assert_eq!(bbox.1.y, 70.0); // 50 + 20
    }

    #[test]
    fn test_bounding_box_arc() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Arc(Arc2D::new(
            Point2D::new(50.0, 50.0),
            25.0,
            0.0,
            PI / 2.0,
        )));
        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 25.0); // 50 - 25
        assert_eq!(bbox.0.y, 25.0); // 50 - 25
        assert_eq!(bbox.1.x, 75.0); // 50 + 25
        assert_eq!(bbox.1.y, 75.0); // 50 + 25
    }

    #[test]
    fn test_bounding_box_text() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Text(
            Text2D::new(Point2D::new(100.0, 200.0), "Test", 12.0),
        ));
        let bbox = compound.bounding_box().unwrap();
        assert_eq!(bbox.0.x, 100.0);
        assert_eq!(bbox.0.y, 200.0);
        assert_eq!(bbox.1.x, 100.0);
        assert_eq!(bbox.1.y, 200.0);
    }

    // ========== Arc Point Calculation Tests ==========

    #[test]
    fn test_arc_start_point() {
        let arc = Arc2D::new(Point2D::new(50.0, 50.0), 25.0, 0.0, PI / 2.0);
        let start = arc.start_point();
        assert!((start.x - 75.0).abs() < 0.001); // 50 + 25 * cos(0)
        assert!((start.y - 50.0).abs() < 0.001); // 50 + 25 * sin(0)
    }

    #[test]
    fn test_arc_end_point() {
        let arc = Arc2D::new(Point2D::new(50.0, 50.0), 25.0, 0.0, PI / 2.0);
        let end = arc.end_point();
        assert!((end.x - 50.0).abs() < 0.001); // 50 + 25 * cos(PI/2)
        assert!((end.y - 75.0).abs() < 0.001); // 50 + 25 * sin(PI/2)
    }

    // ========== Additional ODB2D Conversion Tests ==========

    #[test]
    fn test_odb2d_to_ellipse() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("ellipse".to_string()));
        record_map.insert("x".to_string(), Value::String("100".to_string()));
        record_map.insert("y".to_string(), Value::String("100".to_string()));
        record_map.insert("width".to_string(), Value::String("60".to_string()));
        record_map.insert("height".to_string(), Value::String("40".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::Ellipse(ellipse) = primitive {
            assert_eq!(ellipse.center.x, 100.0);
            assert_eq!(ellipse.center.y, 100.0);
            assert_eq!(ellipse.rx, 30.0); // width/2
            assert_eq!(ellipse.ry, 20.0); // height/2
        } else {
            panic!("Expected Ellipse primitive");
        }
    }

    #[test]
    fn test_odb2d_to_arc() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("arc".to_string()));
        record_map.insert("x".to_string(), Value::String("50".to_string()));
        record_map.insert("y".to_string(), Value::String("50".to_string()));
        record_map.insert("r".to_string(), Value::String("25".to_string()));
        record_map.insert("start_angle".to_string(), Value::String("0".to_string()));
        record_map.insert("end_angle".to_string(), Value::String("90".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::Arc(arc) = primitive {
            assert_eq!(arc.center.x, 50.0);
            assert_eq!(arc.radius, 25.0);
            assert!((arc.start_angle - 0.0).abs() < 0.001);
            assert!((arc.end_angle - (90.0_f64.to_radians())).abs() < 0.001);
        } else {
            panic!("Expected Arc primitive");
        }
    }

    #[test]
    fn test_odb2d_to_linestrip() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("linestrip".to_string()));
        record_map.insert(
            "x_coords".to_string(),
            Value::String("0,50,100".to_string()),
        );
        record_map.insert(
            "y_coords".to_string(),
            Value::String("0,50,0".to_string()),
        );

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::LineStrip(points) = primitive {
            assert_eq!(points.len(), 3);
            assert_eq!(points[0].x, 0.0);
            assert_eq!(points[1].x, 50.0);
            assert_eq!(points[2].x, 100.0);
        } else {
            panic!("Expected LineStrip primitive");
        }
    }

    #[test]
    fn test_odb2d_to_lineloop() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("lineloop".to_string()));
        record_map.insert(
            "x_coords".to_string(),
            Value::String("0,100,50".to_string()),
        );
        record_map.insert(
            "y_coords".to_string(),
            Value::String("0,0,100".to_string()),
        );

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::LineLoop(points) = primitive {
            assert_eq!(points.len(), 3);
        } else {
            panic!("Expected LineLoop primitive");
        }
    }

    #[test]
    fn test_odb2d_to_points() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("points".to_string()));
        record_map.insert(
            "x_coords".to_string(),
            Value::String("10,20,30".to_string()),
        );
        record_map.insert(
            "y_coords".to_string(),
            Value::String("15,25,35".to_string()),
        );

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record).unwrap();

        if let G2DPrimitive::Points(points) = primitive {
            assert_eq!(points.len(), 3);
            assert_eq!(points[0].x, 10.0);
            assert_eq!(points[0].y, 15.0);
        } else {
            panic!("Expected Points primitive");
        }
    }

    #[test]
    fn test_odb2d_unknown_primitive() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("unknown".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record);
        assert!(primitive.is_none());
    }

    #[test]
    fn test_odb2d_empty_points() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("points".to_string()));
        record_map.insert("x_coords".to_string(), Value::String("".to_string()));
        record_map.insert("y_coords".to_string(), Value::String("".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record);
        assert!(primitive.is_none());
    }

    #[test]
    fn test_odb2d_insufficient_line_points() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("lines".to_string()));
        record_map.insert("x_coords".to_string(), Value::String("10".to_string()));
        record_map.insert("y_coords".to_string(), Value::String("20".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record);
        assert!(primitive.is_none());
    }

    #[test]
    fn test_odb2d_insufficient_linestrip_points() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("linestrip".to_string()));
        record_map.insert("x_coords".to_string(), Value::String("10".to_string()));
        record_map.insert("y_coords".to_string(), Value::String("20".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record);
        assert!(primitive.is_none());
    }

    #[test]
    fn test_odb2d_insufficient_lineloop_points() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("lineloop".to_string()));
        record_map.insert("x_coords".to_string(), Value::String("10,20".to_string()));
        record_map.insert("y_coords".to_string(), Value::String("10,20".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record);
        assert!(primitive.is_none());
    }

    #[test]
    fn test_odb2d_insufficient_polygon_points() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("polygon".to_string()));
        record_map.insert("x_coords".to_string(), Value::String("10,20".to_string()));
        record_map.insert("y_coords".to_string(), Value::String("10,20".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let primitive = odb2d_to_primitive(&odb_record);
        assert!(primitive.is_none());
    }

    #[test]
    fn test_odb2d_attributes_dotted() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("rect".to_string()));
        record_map.insert("line_style".to_string(), Value::String("dotted".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let attrs = odb2d_to_attributes(&odb_record);
        assert_eq!(attrs.line_style, LineStyle::Dotted);
    }

    #[test]
    fn test_odb2d_attributes_dashdot() {
        use crate::ebase::{Odb2dRecord, Record, Value};

        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("rect".to_string()));
        record_map.insert("line_style".to_string(), Value::String("dashdot".to_string()));

        let odb_record = Odb2dRecord::from_record(&record_map).unwrap();
        let attrs = odb2d_to_attributes(&odb_record);
        assert_eq!(attrs.line_style, LineStyle::DashDot);
    }

    #[test]
    fn test_ellipse2d_new() {
        let ellipse = Ellipse2D::new(Point2D::new(100.0, 100.0), 50.0, 30.0);
        assert_eq!(ellipse.center.x, 100.0);
        assert_eq!(ellipse.rx, 50.0);
        assert_eq!(ellipse.ry, 30.0);
    }

    #[test]
    fn test_ellipse2d_debug_clone() {
        let ellipse = Ellipse2D::new(Point2D::origin(), 10.0, 5.0);
        let debug_str = format!("{:?}", ellipse);
        assert!(debug_str.contains("Ellipse2D"));

        let cloned = ellipse.clone();
        assert_eq!(cloned.rx, 10.0);
    }

    #[test]
    fn test_fill_style_debug_clone() {
        let style = FillStyle::Hatched;
        let debug_str = format!("{:?}", style);
        assert!(debug_str.contains("Hatched"));

        let cloned = style.clone();
        assert_eq!(cloned, FillStyle::Hatched);
    }

    #[test]
    fn test_text_anchor_debug_clone() {
        let anchor = TextAnchor::Middle;
        let debug_str = format!("{:?}", anchor);
        assert!(debug_str.contains("Middle"));

        let cloned = anchor.clone();
        assert_eq!(cloned, TextAnchor::Middle);
    }

    #[test]
    fn test_g2d_attributes_no_fill() {
        let attrs = G2DAttributes::new();
        assert_eq!(attrs.svg_fill(), "none");
    }

    #[test]
    fn test_g2d_compound_default() {
        let compound = G2DCompound::default();
        assert!(compound.primitives.is_empty());
        assert!(compound.children.is_empty());
    }

    #[test]
    fn test_svg_line_strip_too_few_points() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::LineStrip(vec![Point2D::new(0.0, 0.0)]));
        let svg = compound.to_svg();
        // Should not contain path since we need at least 2 points
        assert!(!svg.contains("<path d=\"M"));
    }

    #[test]
    fn test_svg_line_loop_too_few_points() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::LineLoop(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(10.0, 10.0),
        ]));
        let svg = compound.to_svg();
        // Should not contain closed path since we need at least 3 points
        assert!(!svg.contains("Z"));
    }

    #[test]
    fn test_svg_polygon_too_few_points() {
        let mut compound = G2DCompound::new();
        compound.add_primitive(G2DPrimitive::Polygon(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(10.0, 10.0),
        ]));
        let svg = compound.to_svg();
        // Should not contain polygon since we need at least 3 points
        assert!(!svg.contains("<polygon"));
    }
}
