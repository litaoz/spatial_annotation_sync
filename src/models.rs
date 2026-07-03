use uuid::Uuid;
use chrono::{Utc, DateTime};

use super::{Point, SpatialAnnotation, UserId, AnnotationId};

struct LwwRegister<T> {
    value: T,
    last_modified_user: UserId,
    last_modified_time: DateTime<Utc>
}

impl<T: Clone> LwwRegister<T> {
    fn new(value: T, user: UserId) -> Self {
        Self {
            value,
            last_modified_user: user,
            last_modified_time: Utc::now()
        }
    }

    fn clone_value(&self) -> T {
        self.value.clone()
    }
}

pub struct SpatialAnnotationInternal {
    pub id: AnnotationId,
    coord: LwwRegister<Option<Point>>,
    text: LwwRegister<Option<String>>,
}

impl SpatialAnnotationInternal {
    pub fn new(value: SpatialAnnotation, user: UserId) -> Self {
        let coord = LwwRegister::new(value.coord, user);
        let text = LwwRegister::new(value.text, user);
        Self {
            id: AnnotationId(Uuid::new_v4()),
            coord,
            text,
        }
    }
}

impl From<&SpatialAnnotationInternal> for SpatialAnnotation {
    fn from(value: &SpatialAnnotationInternal) -> Self {
        Self::new_internal(
            Some(value.id),
            value.coord.clone_value(),
            value.text.clone_value()
        )
    }
}