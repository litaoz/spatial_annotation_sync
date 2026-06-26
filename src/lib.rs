mod models;

use std::collections::HashMap;
use uuid::Uuid;
use models::SpatialAnnotationInternal;

// pub struct UserId(pub Uuid);
// pub struct AnnotationId(pub Uuid);

pub struct Point(pub i32, pub i32);

pub struct SpatialAnnotation {
    pub id: Option<Uuid>,
    pub coord: Point,
    pub text: String,
}

impl SpatialAnnotation {
    pub const fn new(id: Option<Uuid>, coord: Point, text: String) -> Self {
        Self {id, coord, text}
    }
}

pub struct SpatialEnvironment {
    user: Uuid,
    data: HashMap<Uuid, SpatialAnnotationInternal>
}

impl SpatialEnvironment {
    pub fn new() -> Self {
        Self {
            user: Uuid::new_v4(),
            data: HashMap::new()
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn create_annotation(&mut self, annotation: SpatialAnnotation) -> Uuid {
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