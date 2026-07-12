mod model;

use core::fmt;
use std::{collections::{HashMap, hash_map::Entry}, str::FromStr};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::{Context, Result};

use self::model::SpatialAnnotationInternal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd,  Hash, serde::Serialize, serde::Deserialize)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, serde::Serialize, serde::Deserialize)]
pub struct AnnotationId(pub Uuid);

impl AnnotationId {
    pub fn new(id: u128) -> Self {
        AnnotationId(Uuid::from_u128(id))
    }
}

impl fmt::Display for AnnotationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id-{}",
            &self.0.simple().to_string()[(32-2)..32]
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Point(pub i32, pub i32);

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let inner = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .with_context(|| format!("expected format (x, y), got '{s}'"))?;

        let (x_str, y_str) = inner.split_once(',')
            .with_context(|| format!("expected format (x, y), got '{s}'"))?;

        let x = x_str.trim().parse::<i32>()?;
        let y = y_str.trim().parse::<i32>()?;

        Ok(Point(x, y))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialAnnotation {
    id: Option<AnnotationId>,
    coord: Option<Point>,
    text: Option<String>,
}

impl SpatialAnnotation {
    #[must_use]
    pub const fn new(id: Option<AnnotationId>, coord: Point, text: String) -> Self {
        Self {id, coord: Some(coord), text: Some(text)}
    }

    const fn new_internal(id: Option<AnnotationId>, coord: Option<Point>, text: Option<String>) -> Self {
        Self {id, coord, text}
    }

    #[must_use]
    pub const fn get_id(&self) -> Option<AnnotationId> {
        self.id
    }

    #[must_use]
    pub const fn get_coord(&self) -> Option<&Point> {
        self.coord.as_ref()
    }

    pub const fn update_coord(&mut self, coord: Option<Point>) {
        self.coord = coord;
    }

    #[must_use]
    pub const fn get_text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    pub fn update_text(&mut self, text: Option<String>) {
        self.text = text;
    }
}

#[cfg(test)]
impl SpatialAnnotation {
    fn without_id(self) -> Self {
        Self::new_internal(
            None, self.coord, self.text
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SpatialEnvironment {
    user: UserId,
    data: HashMap<AnnotationId, SpatialAnnotationInternal>
}

impl SpatialEnvironment {
    #[must_use]
    pub fn new() -> Self {
        Self {
            user: UserId(Uuid::new_v4()),
            data: HashMap::new()
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn create_annotation(&mut self, annotation: SpatialAnnotation) -> AnnotationId {
        Self::create_annotation_with_datetime(self, annotation, Utc::now())
    }

    fn create_annotation_with_datetime(&mut self, annotation: SpatialAnnotation, datetime: DateTime<Utc>) -> AnnotationId {
        let annotation = SpatialAnnotationInternal::new(annotation, self.user, datetime);
        let id = annotation.id;
        self.data.insert(id, annotation);
        id
    }

    pub fn read_annotation(&self, annotation_id: AnnotationId) -> Option<SpatialAnnotation>{
        self.data
            .get(&annotation_id)
            .map(From::from)
    }

    pub fn list_annotation(&self) -> Vec<SpatialAnnotation> {
        self.data.values().take(100).map(From::from).collect()
    }

    pub fn update_annotation(&mut self, annotation: SpatialAnnotation) -> Option<SpatialAnnotation> {
        let a_clone = annotation.clone();
        let Some(id) = annotation.id else {
            self.create_annotation(annotation);
            return Some(a_clone);

        };
        // let annotation = SpatialAnnotationInternal::new(annotation, self.user);
        let _entry = self.data
            .entry(id).and_modify(|a| a.update(annotation, self.user));
        Some(a_clone)
    }

    pub fn delete_annotation(&mut self, annotation_id: AnnotationId) -> Option<SpatialAnnotation> {
        let mut a = self.read_annotation(annotation_id)?;
        a.coord = None;
        a.text = None;
        self.update_annotation(a)
    }

    pub fn merge(&mut self, other: Self) {
        // for every annotiation in the other env, merge
        for (id, incoming) in other.data {
            match self.data.entry(id) {
                Entry::Occupied(mut e) => e.get_mut().merge(incoming),
                Entry::Vacant(e) => {
                    e.insert(incoming);
                }
            }
        }
    }

    #[must_use]
    pub fn has_same_data(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Default for SpatialEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_create_annotation() {
        let mut env = SpatialEnvironment::new();
        assert!(env.is_empty());
        let a1 = SpatialAnnotation::new(
            None,
            Point(0, 0),
            String::from("Home")
        );
        let a2 = SpatialAnnotation::new(
            None,
            Point(1, 0),
            String::from("Neighbor 1")
        );
        let _a1 = env.create_annotation(a1);
        let _a2 = env.create_annotation(a2);
        assert_eq!(env.len(), 2);
    }

    #[test]
    fn test_read_annotation() {
        let mut env = SpatialEnvironment::new();
        let a1 = SpatialAnnotation::new(
            None,
            Point(0, 0),
            String::from("Home")
        );
        let a1_clone = a1.clone();

        let a1_id = env.create_annotation(a1);
        let a1 = env.read_annotation(a1_id)
            .expect("read_annotation should return an existing id")
            .without_id();
        assert_eq!(a1, a1_clone);
    }

    #[test]
    fn test_update_annotation() {
        let mut env = SpatialEnvironment::new();
        let a1 = SpatialAnnotation::new(
            None,
            Point(0, 0),
            String::from("Home")
        );
        let a1_expected = SpatialAnnotation{
            coord: Some(Point(1, 0)),
            ..a1.clone()
        };

        let _ = env.create_annotation(a1);
        let a1 = env.update_annotation(a1_expected.clone())
            .expect("update_annotation should return OK");
        assert_eq!(a1, a1_expected);
    }

    #[test]
    fn test_delete_annotation() {
        let mut env = SpatialEnvironment::new();
        let a1 = SpatialAnnotation::new(
            None,
            Point(0, 0),
            String::from("Home")
        );
        let a1_expected = SpatialAnnotation{
            coord: None,
            text: None,
            ..a1.clone()
        };

        let a1_id = env.create_annotation(a1);
        let _ = env.delete_annotation(a1_id)
            .expect("delete_annotation should return OK");
        assert_eq!(env.len(), 1);
        let a1 = env.read_annotation(a1_id)
            .expect("read_annotation should return an existing id")
            .without_id();

        assert_eq!(a1, a1_expected);
    }

    #[test]
    fn test_merge() {
        let mut env_a = SpatialEnvironment::new();
        let mut env_b = SpatialEnvironment::new();

        let shared_id = AnnotationId(Uuid::from_u128(1));
        let a_only_id = AnnotationId(Uuid::from_u128(2));
        let b_only_id = AnnotationId(Uuid::from_u128(3));

        let earlier = Utc
            .with_ymd_and_hms(2026, 1, 1, 0, 0, 0)
            .single()
            .expect("A valid UTC datetime");
        let later = Utc
            .with_ymd_and_hms(2026, 1, 1, 0, 0, 1)
            .single()
            .expect("A valid UTC datetime");

        env_a.create_annotation_with_datetime(
            SpatialAnnotation::new(
                Some(shared_id),
                Point(0, 0),
                String::from("left-shared")
            ),
            earlier
        );
        env_a.create_annotation_with_datetime(
            SpatialAnnotation::new(
                Some(a_only_id),
                Point(1, 1),
                String::from("left-only")
            ),
            earlier
        );

        env_b.create_annotation_with_datetime(
            SpatialAnnotation::new(
                Some(shared_id),
                Point(5, 5),
                String::from("right-shared")
            ),
            later
        );
        env_b.create_annotation_with_datetime(
            SpatialAnnotation::new(
                Some(b_only_id),
                Point(9, 9),
                String::from("right-only")
            ),
            later
        );

        env_a.merge(env_b);

        assert_eq!(env_a.len(), 3);

        assert_eq!(
            env_a.read_annotation(shared_id),
            Some(SpatialAnnotation::new(
                Some(shared_id),
                Point(5, 5),
                String::from("right-shared")
            ))
        );
        assert_eq!(
            env_a.read_annotation(a_only_id),
            Some(SpatialAnnotation::new(
                Some(a_only_id),
                Point(1, 1),
                String::from("left-only")
            ))
        );
        assert_eq!(
            env_a.read_annotation(b_only_id),
            Some(SpatialAnnotation::new(
                Some(b_only_id),
                Point(9, 9),
                String::from("right-only")
            ))
        );
    }

    #[test]
    fn test_merge_commutativity() {
        let mut env_a = SpatialEnvironment::new();
        let mut env_b = SpatialEnvironment::new();

        env_a.create_annotation(
            SpatialAnnotation::new(
                Some(AnnotationId(Uuid::from_u128(1))),
                Point(0, 0),
                String::from("")
            )
        );

        env_b.create_annotation(
            SpatialAnnotation::new(
                Some(AnnotationId(Uuid::from_u128(1))),
                Point(0, 0),
                String::from("")
            )
        );

        let mut a_b = env_a.clone();
        a_b.merge(env_b.clone());

        let mut b_a = env_b.clone();
        b_a.merge(env_a.clone());

        assert!(a_b.has_same_data(&b_a), "a merge b should be the same as b merge a");
    }

    #[test]
    fn test_from_str_point() {
        let s = "(1, 2)";
        let res = Point::from_str(s).expect("Point was not parsed correctly");
        assert_eq!(res, Point(1, 2));

    }
}