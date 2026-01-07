//! Attachment Points System - Product composition and connection points.
//!
//! This module implements attachment points for connecting products together.
//! Attachment points are loaded from EBASE tables: attpt, oppattpt, stdattpt.

use crate::errors::AttachmentError;

/// Attachment point type.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AttachmentType {
    /// Standard attachment point (from stdattpt table)
    #[default]
    Standard,
    /// Opposite attachment point (from oppattpt table)
    Opposite,
    /// Product-specific attachment point (from attpt table)
    Custom,
}

impl AttachmentType {
    /// Get the table name for this type.
    pub fn table_name(&self) -> &'static str {
        match self {
            AttachmentType::Standard => "stdattpt",
            AttachmentType::Opposite => "oppattpt",
            AttachmentType::Custom => "attpt",
        }
    }
}

/// Attachment point definition.
#[derive(Debug, Clone)]
pub struct AttachmentPoint {
    /// Point name/identifier
    pub name: String,
    /// Local position [x, y, z]
    pub position: [f64; 3],
    /// Attachment direction [dx, dy, dz] (unit vector)
    pub direction: [f64; 3],
    /// Point type
    pub point_type: AttachmentType,
    /// Compatibility tags for matching
    pub tags: Vec<String>,
}

impl AttachmentPoint {
    /// Create a new attachment point.
    pub fn new(name: impl Into<String>, position: [f64; 3], direction: [f64; 3]) -> Self {
        Self {
            name: name.into(),
            position,
            direction,
            point_type: AttachmentType::default(),
            tags: Vec::new(),
        }
    }

    /// Set the point type.
    pub fn with_type(mut self, point_type: AttachmentType) -> Self {
        self.point_type = point_type;
        self
    }

    /// Add a compatibility tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add multiple compatibility tags.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }

    /// Check if this point is compatible with another.
    ///
    /// Points are compatible if they share at least one tag or if
    /// one is a standard point and the other is opposite.
    pub fn is_compatible(&self, other: &AttachmentPoint) -> bool {
        // Check type compatibility
        let type_compatible = matches!(
            (&self.point_type, &other.point_type),
            (AttachmentType::Standard, AttachmentType::Opposite)
                | (AttachmentType::Opposite, AttachmentType::Standard)
        );

        if type_compatible {
            return true;
        }

        // Check tag compatibility
        for tag in &self.tags {
            if other.tags.contains(tag) {
                return true;
            }
        }

        false
    }

    /// Validate the direction vector (should be unit length).
    pub fn validate(&self) -> Result<(), AttachmentError> {
        let len =
            (self.direction[0].powi(2) + self.direction[1].powi(2) + self.direction[2].powi(2))
                .sqrt();

        if (len - 1.0).abs() > 0.01 {
            return Err(AttachmentError::InvalidDirection);
        }

        Ok(())
    }

    /// Normalize the direction vector.
    pub fn normalize_direction(&mut self) {
        let len =
            (self.direction[0].powi(2) + self.direction[1].powi(2) + self.direction[2].powi(2))
                .sqrt();

        if len > 0.0 {
            self.direction[0] /= len;
            self.direction[1] /= len;
            self.direction[2] /= len;
        }
    }

    /// Transform position to world coordinates.
    pub fn world_position(&self, transform: &[[f64; 4]; 4]) -> [f64; 3] {
        let x = self.position[0];
        let y = self.position[1];
        let z = self.position[2];

        [
            transform[0][0] * x + transform[0][1] * y + transform[0][2] * z + transform[0][3],
            transform[1][0] * x + transform[1][1] * y + transform[1][2] * z + transform[1][3],
            transform[2][0] * x + transform[2][1] * y + transform[2][2] * z + transform[2][3],
        ]
    }

    /// Transform direction to world coordinates.
    pub fn world_direction(&self, transform: &[[f64; 4]; 4]) -> [f64; 3] {
        let dx = self.direction[0];
        let dy = self.direction[1];
        let dz = self.direction[2];

        // Transform direction (no translation)
        let mut result = [
            transform[0][0] * dx + transform[0][1] * dy + transform[0][2] * dz,
            transform[1][0] * dx + transform[1][1] * dy + transform[1][2] * dz,
            transform[2][0] * dx + transform[2][1] * dy + transform[2][2] * dz,
        ];

        // Normalize
        let len = (result[0].powi(2) + result[1].powi(2) + result[2].powi(2)).sqrt();
        if len > 0.0 {
            result[0] /= len;
            result[1] /= len;
            result[2] /= len;
        }

        result
    }
}

/// Collection of attachment points.
#[derive(Debug, Clone, Default)]
pub struct AttachmentPointSet {
    /// All attachment points
    points: Vec<AttachmentPoint>,
}

impl AttachmentPointSet {
    /// Create a new empty set.
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Add an attachment point.
    pub fn add(&mut self, point: AttachmentPoint) {
        self.points.push(point);
    }

    /// Get a point by name.
    pub fn get(&self, name: &str) -> Option<&AttachmentPoint> {
        self.points.iter().find(|p| p.name == name)
    }

    /// Get a mutable point by name.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut AttachmentPoint> {
        self.points.iter_mut().find(|p| p.name == name)
    }

    /// Get all points.
    pub fn all(&self) -> &[AttachmentPoint] {
        &self.points
    }

    /// Get points by type.
    pub fn by_type(&self, point_type: AttachmentType) -> Vec<&AttachmentPoint> {
        self.points
            .iter()
            .filter(|p| p.point_type == point_type)
            .collect()
    }

    /// Get points with a specific tag.
    pub fn by_tag(&self, tag: &str) -> Vec<&AttachmentPoint> {
        self.points
            .iter()
            .filter(|p| p.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Find compatible point pairs between this set and another.
    pub fn find_compatible_pairs<'a>(
        &'a self,
        other: &'a AttachmentPointSet,
    ) -> Vec<(&'a AttachmentPoint, &'a AttachmentPoint)> {
        let mut pairs = Vec::new();

        for self_point in &self.points {
            for other_point in &other.points {
                if self_point.is_compatible(other_point) {
                    pairs.push((self_point, other_point));
                }
            }
        }

        pairs
    }

    /// Number of points.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Clear all points.
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Remove a point by name.
    pub fn remove(&mut self, name: &str) -> Option<AttachmentPoint> {
        if let Some(pos) = self.points.iter().position(|p| p.name == name) {
            Some(self.points.remove(pos))
        } else {
            None
        }
    }

    /// Get point names.
    pub fn names(&self) -> Vec<&str> {
        self.points.iter().map(|p| p.name.as_str()).collect()
    }
}

/// Load attachment points from an EBASE record.
///
/// Parses attachment point data from attpt, oppattpt, or stdattpt tables.
pub fn load_attachment_from_record(
    record: &crate::ebase::Record,
    point_type: AttachmentType,
) -> Option<AttachmentPoint> {
    use crate::ebase::Value;

    let name = record
        .get("name")
        .or_else(|| record.get("attpt_name"))
        .and_then(|v| v.as_str())?
        .to_string();

    let get_float = |key: &str| -> f64 { record.get(key).and_then(|v| v.as_f64()).unwrap_or(0.0) };

    let x = get_float("x").max(get_float("pos_x"));
    let y = get_float("y").max(get_float("pos_y"));
    let z = get_float("z").max(get_float("pos_z"));

    let dx = record
        .get("dx")
        .or_else(|| record.get("dir_x"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let dy = record
        .get("dy")
        .or_else(|| record.get("dir_y"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let mut dz = record
        .get("dz")
        .or_else(|| record.get("dir_z"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    // Default direction to +Z if all components are 0
    if dx == 0.0 && dy == 0.0 && dz == 0.0 {
        dz = 1.0;
    }

    let mut point = AttachmentPoint::new(name, [x, y, z], [dx, dy, dz]).with_type(point_type);
    point.normalize_direction();

    // Load tags if present
    if let Some(Value::String(tags)) = record.get("tags").or_else(|| record.get("tag")) {
        for tag in tags.split(',') {
            let tag = tag.trim();
            if !tag.is_empty() {
                point.tags.push(tag.to_string());
            }
        }
    }

    Some(point)
}

/// Load attachment points from EBASE attpt table.
pub fn load_attpt_table(reader: &mut crate::ebase::EBaseReader) -> Vec<AttachmentPoint> {
    load_attachment_table(reader, "attpt", AttachmentType::Custom)
}

/// Load attachment points from EBASE oppattpt table.
pub fn load_oppattpt_table(reader: &mut crate::ebase::EBaseReader) -> Vec<AttachmentPoint> {
    load_attachment_table(reader, "oppattpt", AttachmentType::Opposite)
}

/// Load attachment points from EBASE stdattpt table.
pub fn load_stdattpt_table(reader: &mut crate::ebase::EBaseReader) -> Vec<AttachmentPoint> {
    load_attachment_table(reader, "stdattpt", AttachmentType::Standard)
}

/// Load attachment points from a specific EBASE table.
fn load_attachment_table(
    reader: &mut crate::ebase::EBaseReader,
    table_name: &str,
    point_type: AttachmentType,
) -> Vec<AttachmentPoint> {
    if !reader.tables.contains_key(table_name) {
        return Vec::new();
    }

    match reader.read_records(table_name, None) {
        Ok(records) => records
            .iter()
            .filter_map(|r| load_attachment_from_record(r, point_type.clone()))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Load all attachment points from an EBASE file into an AttachmentPointSet.
pub fn load_all_attachments(reader: &mut crate::ebase::EBaseReader) -> AttachmentPointSet {
    let mut set = AttachmentPointSet::new();

    // Load from all three tables
    for point in load_attpt_table(reader) {
        set.add(point);
    }
    for point in load_oppattpt_table(reader) {
        set.add(point);
    }
    for point in load_stdattpt_table(reader) {
        set.add(point);
    }

    set
}

/// Builder for creating attachment points from EBASE data.
pub struct AttachmentPointBuilder {
    point: AttachmentPoint,
}

impl AttachmentPointBuilder {
    /// Start building a new attachment point.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            point: AttachmentPoint::new(name, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        }
    }

    /// Set position.
    pub fn position(mut self, x: f64, y: f64, z: f64) -> Self {
        self.point.position = [x, y, z];
        self
    }

    /// Set direction.
    pub fn direction(mut self, dx: f64, dy: f64, dz: f64) -> Self {
        self.point.direction = [dx, dy, dz];
        self.point.normalize_direction();
        self
    }

    /// Set type.
    pub fn point_type(mut self, point_type: AttachmentType) -> Self {
        self.point.point_type = point_type;
        self
    }

    /// Add tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.point.tags.push(tag.into());
        self
    }

    /// Build the attachment point.
    pub fn build(self) -> AttachmentPoint {
        self.point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attachment_point_creation() {
        let point = AttachmentPoint::new("test_point", [10.0, 20.0, 30.0], [0.0, 0.0, 1.0]);

        assert_eq!(point.name, "test_point");
        assert_eq!(point.position, [10.0, 20.0, 30.0]);
        assert_eq!(point.direction, [0.0, 0.0, 1.0]);
        assert_eq!(point.point_type, AttachmentType::Standard);
    }

    #[test]
    fn test_attachment_point_builder() {
        let point = AttachmentPointBuilder::new("builder_point")
            .position(100.0, 200.0, 300.0)
            .direction(1.0, 0.0, 0.0)
            .point_type(AttachmentType::Custom)
            .tag("connector")
            .build();

        assert_eq!(point.name, "builder_point");
        assert_eq!(point.position, [100.0, 200.0, 300.0]);
        assert_eq!(point.point_type, AttachmentType::Custom);
        assert!(point.tags.contains(&"connector".to_string()));
    }

    #[test]
    fn test_attachment_point_validation() {
        let valid = AttachmentPoint::new("valid", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(valid.validate().is_ok());

        let invalid = AttachmentPoint::new("invalid", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_attachment_point_normalize() {
        let mut point = AttachmentPoint::new("test", [0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
        point.normalize_direction();

        assert!((point.direction[0] - 0.6).abs() < 0.001);
        assert!((point.direction[1] - 0.8).abs() < 0.001);
        assert!(point.validate().is_ok());
    }

    #[test]
    fn test_attachment_compatibility() {
        let standard = AttachmentPoint::new("std", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Standard);
        let opposite = AttachmentPoint::new("opp", [0.0, 0.0, 0.0], [0.0, 0.0, -1.0])
            .with_type(AttachmentType::Opposite);
        let custom1 = AttachmentPoint::new("c1", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Custom)
            .with_tag("power");
        let custom2 = AttachmentPoint::new("c2", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Custom)
            .with_tag("power");

        // Standard and Opposite are compatible
        assert!(standard.is_compatible(&opposite));
        assert!(opposite.is_compatible(&standard));

        // Custom with same tag are compatible
        assert!(custom1.is_compatible(&custom2));

        // Standard and Custom (no matching tags) are not compatible
        assert!(!standard.is_compatible(&custom1));
    }

    #[test]
    fn test_attachment_point_set() {
        let mut set = AttachmentPointSet::new();

        set.add(
            AttachmentPoint::new("p1", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
                .with_type(AttachmentType::Standard),
        );
        set.add(
            AttachmentPoint::new("p2", [100.0, 0.0, 0.0], [0.0, 0.0, 1.0])
                .with_type(AttachmentType::Custom)
                .with_tag("connector"),
        );

        assert_eq!(set.len(), 2);
        assert!(set.get("p1").is_some());
        assert!(set.get("p3").is_none());

        let standard = set.by_type(AttachmentType::Standard);
        assert_eq!(standard.len(), 1);

        let tagged = set.by_tag("connector");
        assert_eq!(tagged.len(), 1);
    }

    #[test]
    fn test_find_compatible_pairs() {
        let mut set1 = AttachmentPointSet::new();
        set1.add(
            AttachmentPoint::new("std1", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
                .with_type(AttachmentType::Standard),
        );

        let mut set2 = AttachmentPointSet::new();
        set2.add(
            AttachmentPoint::new("opp1", [0.0, 0.0, 0.0], [0.0, 0.0, -1.0])
                .with_type(AttachmentType::Opposite),
        );

        let pairs = set1.find_compatible_pairs(&set2);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].0.name, "std1");
        assert_eq!(pairs[0].1.name, "opp1");
    }

    #[test]
    fn test_world_transform() {
        let point = AttachmentPoint::new("test", [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);

        // Translation matrix
        let transform = [
            [1.0, 0.0, 0.0, 10.0],
            [0.0, 1.0, 0.0, 20.0],
            [0.0, 0.0, 1.0, 30.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let world_pos = point.world_position(&transform);
        assert!((world_pos[0] - 11.0).abs() < 0.001);
        assert!((world_pos[1] - 20.0).abs() < 0.001);
        assert!((world_pos[2] - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_load_attachment_from_record() {
        use crate::ebase::{Record, Value};

        let mut record = Record::new();
        record.insert("name".to_string(), Value::String("connector_1".to_string()));
        record.insert("x".to_string(), Value::Float(10.0));
        record.insert("y".to_string(), Value::Float(20.0));
        record.insert("z".to_string(), Value::Float(30.0));
        record.insert("dx".to_string(), Value::Float(1.0));
        record.insert("dy".to_string(), Value::Float(0.0));
        record.insert("dz".to_string(), Value::Float(0.0));
        record.insert("tags".to_string(), Value::String("power, data".to_string()));

        let point = load_attachment_from_record(&record, AttachmentType::Custom).unwrap();

        assert_eq!(point.name, "connector_1");
        assert_eq!(point.position, [10.0, 20.0, 30.0]);
        assert!((point.direction[0] - 1.0).abs() < 0.001);
        assert_eq!(point.point_type, AttachmentType::Custom);
        assert!(point.tags.contains(&"power".to_string()));
        assert!(point.tags.contains(&"data".to_string()));
    }

    #[test]
    fn test_load_attachment_missing_name() {
        use crate::ebase::{Record, Value};

        let mut record = Record::new();
        record.insert("x".to_string(), Value::Float(10.0));

        // Should return None when name is missing
        let point = load_attachment_from_record(&record, AttachmentType::Standard);
        assert!(point.is_none());
    }

    #[test]
    fn test_attachment_type_table_name() {
        assert_eq!(AttachmentType::Standard.table_name(), "stdattpt");
        assert_eq!(AttachmentType::Opposite.table_name(), "oppattpt");
        assert_eq!(AttachmentType::Custom.table_name(), "attpt");
    }

    #[test]
    fn test_attachment_type_default() {
        let default_type: AttachmentType = Default::default();
        assert_eq!(default_type, AttachmentType::Standard);
    }

    #[test]
    fn test_attachment_type_clone() {
        let original = AttachmentType::Opposite;
        let cloned = original.clone();
        assert_eq!(cloned, AttachmentType::Opposite);
    }

    #[test]
    fn test_attachment_type_debug() {
        let std = AttachmentType::Standard;
        assert_eq!(format!("{:?}", std), "Standard");
        let opp = AttachmentType::Opposite;
        assert_eq!(format!("{:?}", opp), "Opposite");
        let cust = AttachmentType::Custom;
        assert_eq!(format!("{:?}", cust), "Custom");
    }

    #[test]
    fn test_attachment_point_with_type() {
        let point = AttachmentPoint::new("p", [0.0; 3], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Opposite);
        assert_eq!(point.point_type, AttachmentType::Opposite);
    }

    #[test]
    fn test_attachment_point_with_tags() {
        let point = AttachmentPoint::new("p", [0.0; 3], [0.0, 0.0, 1.0])
            .with_tags(vec!["tag1".to_string(), "tag2".to_string()]);
        assert_eq!(point.tags.len(), 2);
        assert!(point.tags.contains(&"tag1".to_string()));
        assert!(point.tags.contains(&"tag2".to_string()));
    }

    #[test]
    fn test_attachment_point_clone() {
        let point = AttachmentPoint::new("cloneable", [1.0, 2.0, 3.0], [0.0, 1.0, 0.0])
            .with_type(AttachmentType::Custom)
            .with_tag("clone_tag");
        let cloned = point.clone();

        assert_eq!(cloned.name, "cloneable");
        assert_eq!(cloned.position, [1.0, 2.0, 3.0]);
        assert_eq!(cloned.direction, [0.0, 1.0, 0.0]);
        assert_eq!(cloned.point_type, AttachmentType::Custom);
        assert!(cloned.tags.contains(&"clone_tag".to_string()));
    }

    #[test]
    fn test_attachment_point_debug() {
        let point = AttachmentPoint::new("debug_test", [0.0; 3], [0.0, 0.0, 1.0]);
        let debug_str = format!("{:?}", point);
        assert!(debug_str.contains("debug_test"));
    }

    #[test]
    fn test_validate_non_unit_direction() {
        // Direction too long
        let long_dir = AttachmentPoint::new("test", [0.0; 3], [2.0, 0.0, 0.0]);
        assert!(long_dir.validate().is_err());

        // Direction too short
        let short_dir = AttachmentPoint::new("test", [0.0; 3], [0.1, 0.0, 0.0]);
        assert!(short_dir.validate().is_err());
    }

    #[test]
    fn test_normalize_zero_direction() {
        let mut point = AttachmentPoint::new("zero", [0.0; 3], [0.0, 0.0, 0.0]);
        point.normalize_direction();
        // Direction should remain zero (no normalization possible)
        assert_eq!(point.direction, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_world_direction_transform() {
        // Test a simple 90-degree rotation around Z axis
        let point = AttachmentPoint::new("test", [0.0; 3], [1.0, 0.0, 0.0]);

        // 90-degree rotation around Z
        let transform = [
            [0.0, -1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let world_dir = point.world_direction(&transform);
        // [1, 0, 0] rotated 90 degrees around Z should give [0, 1, 0]
        assert!((world_dir[0] - 0.0).abs() < 0.001);
        assert!((world_dir[1] - 1.0).abs() < 0.001);
        assert!((world_dir[2] - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_world_direction_zero_result() {
        let point = AttachmentPoint::new("test", [0.0; 3], [0.0, 0.0, 1.0]);

        // Transform that projects onto xy plane (zeroing the direction if it was Z)
        let transform = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let world_dir = point.world_direction(&transform);
        // Result should be [0, 0, 0] since normalization can't be done
        assert_eq!(world_dir, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_is_compatible_no_tags() {
        let p1 = AttachmentPoint::new("p1", [0.0; 3], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Custom);
        let p2 = AttachmentPoint::new("p2", [0.0; 3], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Custom);

        // Custom types without shared tags are not compatible
        assert!(!p1.is_compatible(&p2));
    }

    #[test]
    fn test_is_compatible_different_tags() {
        let p1 = AttachmentPoint::new("p1", [0.0; 3], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Custom)
            .with_tag("power");
        let p2 = AttachmentPoint::new("p2", [0.0; 3], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Custom)
            .with_tag("data");

        assert!(!p1.is_compatible(&p2));
    }

    #[test]
    fn test_is_compatible_multiple_tags() {
        let p1 = AttachmentPoint::new("p1", [0.0; 3], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Custom)
            .with_tags(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let p2 = AttachmentPoint::new("p2", [0.0; 3], [0.0, 0.0, 1.0])
            .with_type(AttachmentType::Custom)
            .with_tags(vec!["x".to_string(), "y".to_string(), "c".to_string()]);

        // Share tag "c"
        assert!(p1.is_compatible(&p2));
    }

    #[test]
    fn test_attachment_point_set_default() {
        let set: AttachmentPointSet = Default::default();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_attachment_point_set_clone() {
        let mut set = AttachmentPointSet::new();
        set.add(AttachmentPoint::new("p1", [0.0; 3], [0.0, 0.0, 1.0]));

        let cloned = set.clone();
        assert_eq!(cloned.len(), 1);
        assert!(cloned.get("p1").is_some());
    }

    #[test]
    fn test_attachment_point_set_debug() {
        let set = AttachmentPointSet::new();
        let debug_str = format!("{:?}", set);
        assert!(debug_str.contains("AttachmentPointSet"));
    }

    #[test]
    fn test_attachment_point_set_get_mut() {
        let mut set = AttachmentPointSet::new();
        set.add(AttachmentPoint::new("mutable", [0.0; 3], [0.0, 0.0, 1.0]));

        if let Some(point) = set.get_mut("mutable") {
            point.position = [1.0, 2.0, 3.0];
        }

        let point = set.get("mutable").unwrap();
        assert_eq!(point.position, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_attachment_point_set_get_mut_not_found() {
        let mut set = AttachmentPointSet::new();
        assert!(set.get_mut("nonexistent").is_none());
    }

    #[test]
    fn test_attachment_point_set_all() {
        let mut set = AttachmentPointSet::new();
        set.add(AttachmentPoint::new("p1", [0.0; 3], [0.0, 0.0, 1.0]));
        set.add(AttachmentPoint::new("p2", [1.0; 3], [0.0, 0.0, 1.0]));

        let all = set.all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_attachment_point_set_clear() {
        let mut set = AttachmentPointSet::new();
        set.add(AttachmentPoint::new("p1", [0.0; 3], [0.0, 0.0, 1.0]));
        set.add(AttachmentPoint::new("p2", [1.0; 3], [0.0, 0.0, 1.0]));

        assert_eq!(set.len(), 2);
        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn test_attachment_point_set_remove() {
        let mut set = AttachmentPointSet::new();
        set.add(AttachmentPoint::new("removable", [0.0; 3], [0.0, 0.0, 1.0]));

        let removed = set.remove("removable");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().name, "removable");
        assert!(set.is_empty());
    }

    #[test]
    fn test_attachment_point_set_remove_not_found() {
        let mut set = AttachmentPointSet::new();
        set.add(AttachmentPoint::new("keep", [0.0; 3], [0.0, 0.0, 1.0]));

        let removed = set.remove("nonexistent");
        assert!(removed.is_none());
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_attachment_point_set_names() {
        let mut set = AttachmentPointSet::new();
        set.add(AttachmentPoint::new("alpha", [0.0; 3], [0.0, 0.0, 1.0]));
        set.add(AttachmentPoint::new("beta", [1.0; 3], [0.0, 0.0, 1.0]));

        let names = set.names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"alpha"));
        assert!(names.contains(&"beta"));
    }

    #[test]
    fn test_attachment_point_set_by_type_empty() {
        let set = AttachmentPointSet::new();
        let result = set.by_type(AttachmentType::Standard);
        assert!(result.is_empty());
    }

    #[test]
    fn test_attachment_point_set_by_tag_empty() {
        let set = AttachmentPointSet::new();
        let result = set.by_tag("nonexistent");
        assert!(result.is_empty());
    }

    #[test]
    fn test_attachment_point_set_by_tag_no_match() {
        let mut set = AttachmentPointSet::new();
        set.add(
            AttachmentPoint::new("p1", [0.0; 3], [0.0, 0.0, 1.0])
                .with_tag("power")
        );

        let result = set.by_tag("data");
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_compatible_pairs_empty_sets() {
        let set1 = AttachmentPointSet::new();
        let set2 = AttachmentPointSet::new();

        let pairs = set1.find_compatible_pairs(&set2);
        assert!(pairs.is_empty());
    }

    #[test]
    fn test_find_compatible_pairs_no_matches() {
        let mut set1 = AttachmentPointSet::new();
        set1.add(
            AttachmentPoint::new("p1", [0.0; 3], [0.0, 0.0, 1.0])
                .with_type(AttachmentType::Custom)
                .with_tag("type_a")
        );

        let mut set2 = AttachmentPointSet::new();
        set2.add(
            AttachmentPoint::new("p2", [0.0; 3], [0.0, 0.0, 1.0])
                .with_type(AttachmentType::Custom)
                .with_tag("type_b")
        );

        let pairs = set1.find_compatible_pairs(&set2);
        assert!(pairs.is_empty());
    }

    #[test]
    fn test_find_compatible_pairs_multiple() {
        let mut set1 = AttachmentPointSet::new();
        set1.add(
            AttachmentPoint::new("std1", [0.0; 3], [0.0, 0.0, 1.0])
                .with_type(AttachmentType::Standard)
        );
        set1.add(
            AttachmentPoint::new("std2", [1.0; 3], [0.0, 0.0, 1.0])
                .with_type(AttachmentType::Standard)
        );

        let mut set2 = AttachmentPointSet::new();
        set2.add(
            AttachmentPoint::new("opp1", [0.0; 3], [0.0, 0.0, -1.0])
                .with_type(AttachmentType::Opposite)
        );

        let pairs = set1.find_compatible_pairs(&set2);
        // 2 standard x 1 opposite = 2 pairs
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn test_load_attachment_from_record_with_attpt_name() {
        use crate::ebase::{Record, Value};

        let mut record = Record::new();
        record.insert("attpt_name".to_string(), Value::String("alt_name".to_string()));
        record.insert("pos_x".to_string(), Value::Float(5.0));
        record.insert("pos_y".to_string(), Value::Float(10.0));
        record.insert("pos_z".to_string(), Value::Float(15.0));
        record.insert("dir_x".to_string(), Value::Float(0.0));
        record.insert("dir_y".to_string(), Value::Float(1.0));
        record.insert("dir_z".to_string(), Value::Float(0.0));

        let point = load_attachment_from_record(&record, AttachmentType::Standard).unwrap();

        assert_eq!(point.name, "alt_name");
        assert_eq!(point.position, [5.0, 10.0, 15.0]);
        assert!((point.direction[1] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_load_attachment_from_record_zero_direction() {
        use crate::ebase::{Record, Value};

        let mut record = Record::new();
        record.insert("name".to_string(), Value::String("zero_dir".to_string()));
        // No direction specified - should default to +Z

        let point = load_attachment_from_record(&record, AttachmentType::Custom).unwrap();

        // Direction should be normalized +Z
        assert!((point.direction[2] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_load_attachment_from_record_with_tag_singular() {
        use crate::ebase::{Record, Value};

        let mut record = Record::new();
        record.insert("name".to_string(), Value::String("tagged".to_string()));
        record.insert("tag".to_string(), Value::String("single_tag".to_string()));

        let point = load_attachment_from_record(&record, AttachmentType::Custom).unwrap();

        assert!(point.tags.contains(&"single_tag".to_string()));
    }

    #[test]
    fn test_builder_direction_normalizes() {
        let point = AttachmentPointBuilder::new("normalized")
            .direction(3.0, 4.0, 0.0)  // Should be normalized to [0.6, 0.8, 0]
            .build();

        assert!((point.direction[0] - 0.6).abs() < 0.001);
        assert!((point.direction[1] - 0.8).abs() < 0.001);
        assert!((point.direction[2]).abs() < 0.001);
    }

    #[test]
    fn test_builder_complete() {
        let point = AttachmentPointBuilder::new("complete")
            .position(10.0, 20.0, 30.0)
            .direction(0.0, 0.0, 1.0)
            .point_type(AttachmentType::Opposite)
            .tag("power")
            .tag("data")
            .build();

        assert_eq!(point.name, "complete");
        assert_eq!(point.position, [10.0, 20.0, 30.0]);
        assert_eq!(point.direction, [0.0, 0.0, 1.0]);
        assert_eq!(point.point_type, AttachmentType::Opposite);
        assert_eq!(point.tags.len(), 2);
    }
}
