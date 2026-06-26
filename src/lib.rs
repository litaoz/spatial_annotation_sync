mod models;

use std::collections::HashMap;
use uuid::Uuid;
use models::SpatialAnnotationInternal;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct UserId(pub Uuid);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct AnnotationId(pub Uuid);

pub struct Point(pub i32, pub i32);

pub struct SpatialAnnotation {
    pub id: Option<AnnotationId>,
    pub coord: Point,
    pub text: String,
}

impl SpatialAnnotation {
    #[must_use]
    pub const fn new(id: Option<AnnotationId>, coord: Point, text: String) -> Self {
        Self {id, coord, text}
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
        let a1 = SpatialAnnotation{
            id: None,
            coord: Point(0, 0),
            text: String::from("Home")
        };
        let a2 = SpatialAnnotation{
            id: None,
            coord: Point(1, 0),
            text: String::from("Neighbor 1")
        };
        let _a1 = env.create_annotation(a1);
        let _a2 = env.create_annotation(a2);
        assert_eq!(env.len(), 2);
    }

    #[test]
    fn test_read_annotation() {
        // let mut env = SpatialEnvironment::new();
        // let a1 = SpatialAnnotation{
        //     coord: Point(0, 0),
        //     text: String::from("Home")
        // };
        // let a1_clone = a1.clone();
        // let a2 = SpatialAnnotation{
        //     coord: Point(1, 0),
        //     text: String::from("Neighbor 1")
        // };
        // let _a1 = env.create_annotation(a1);
        // let _a2 = env.create_annotation(a2);
    }

    #[test]
    fn test_update_annotation() {
    }

    #[test]
    fn test_delete_annotation() {
    }
}