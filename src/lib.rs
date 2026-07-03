mod models;

use std::collections::HashMap;
use uuid::Uuid;
use models::SpatialAnnotationInternal;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd,  Hash)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct AnnotationId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

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
}

#[cfg(test)]
impl SpatialAnnotation {
    fn without_id(self) -> Self {
        Self::new_internal(
            None, self.coord, self.text
        )
    }
}

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
        let annotation = SpatialAnnotationInternal::new(annotation, self.user);
        let id = annotation.id;
        self.data.insert(id, annotation);
        id
    }

    pub fn read_annotation(&self, annotation_id: AnnotationId) -> Option<SpatialAnnotation>{
        self.data
            .get(&annotation_id)
            .map(From::from)
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
    }

    #[test]
    fn test_delete_annotation() {
    }
}