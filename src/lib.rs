mod sync;

use std::collections::HashMap;

use uuid::Uuid;
use chrono::{Utc, DateTime};

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Point(pub i32, pub i32);

pub struct SpatialAnnotation {
    pub coord: Point,
    pub text: String,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct LwwRegister<T> {
    value: T,
    last_modified_user: Uuid,
    last_modified_time: DateTime<Utc>
}

impl<T> LwwRegister<T> {
    fn new(value: T, user: Uuid) -> Self {
        Self {
            value,
            last_modified_user: user,
            last_modified_time: Utc::now()
        }
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct SpatialAnnotationInternal {
    id: Uuid,
    coord: Option<LwwRegister<Point>>,
    text: Option<LwwRegister<String>>,
}

impl SpatialAnnotationInternal {
    fn new(value: SpatialAnnotation, user: Uuid) -> Self {
        let coord = Some(LwwRegister::new(value.coord, user));
        let text = Some(LwwRegister::new(value.text, user));
        Self {
            id: Uuid::new_v4(),
            coord,
            text,
        }
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

    pub fn create_annotation(&mut self, annotation: SpatialAnnotation) -> Uuid {
        let annotation = SpatialAnnotationInternal::new(annotation, self.user);
        let id = annotation.id;
        self.data.insert(id, annotation);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_annotation() {
        let mut env = SpatialEnvironment::new();
        let a1 = SpatialAnnotation{
            coord: Point(0, 0),
            text: String::from("Home")
        };
        let a2 = SpatialAnnotation{
            coord: Point(1, 0),
            text: String::from("Neighbor 1")
        };
        let _a1 = env.create_annotation(a1);
        let _a2 = env.create_annotation(a2);
        assert_eq!(env.len(), 2);
    }

    #[test]
    fn test_read_annotation() {
    }

    #[test]
    fn test_update_annotation() {
    }

    #[test]
    fn test_delete_annotation() {
    }
}