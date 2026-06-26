use uuid::Uuid;
use chrono::{Utc, DateTime};

use super::{Point, SpatialAnnotation};

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

pub struct SpatialAnnotationInternal {
    pub id: Uuid,
    coord: LwwRegister<Option<Point>>,
    text: LwwRegister<Option<String>>,
}

impl SpatialAnnotationInternal {
    pub fn new(value: SpatialAnnotation, user: Uuid) -> Self {
        let coord = LwwRegister::new(Some(value.coord), user);
        let text = LwwRegister::new(Some(value.text), user);
        Self {
            id: Uuid::new_v4(),
            coord,
            text,
        }
    }
}
