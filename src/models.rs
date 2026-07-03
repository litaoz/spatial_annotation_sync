use uuid::Uuid;
use chrono::{Utc, DateTime};

use super::{Point, SpatialAnnotation, UserId, AnnotationId};

#[derive(Debug, Clone, PartialEq)]
struct LwwRegister<T> {
    value: T,
    last_modified_user: UserId,
    last_modified_time: DateTime<Utc>
}

impl<T: Clone + PartialEq> LwwRegister<T> {
    // fn new(value: T, user: UserId) -> Self {
    //     Self::new_with_datetime(value, user, Utc::now())
    // }

    const fn new_with_datetime(value: T, user: UserId, datetime: DateTime<Utc>) -> Self {
        Self {
            value,
            last_modified_user: user,
            last_modified_time: datetime
        }
    }

    fn clone_value(&self) -> T {
        self.value.clone()
    }

    fn update(&mut self, value: T, user: UserId) {
        if self.value == value { return; }
        self.value = value;
        self.last_modified_user = user;
        self.last_modified_time = Utc::now();
    }

    fn merge(&mut self, other: Self) {
        let other_is_newer = other.last_modified_time > self.last_modified_time;
        let other_is_equal_and_lower_user_id = other.last_modified_time == self.last_modified_time && other.last_modified_user < self.last_modified_user;
        if other_is_newer || other_is_equal_and_lower_user_id {
            self.value = other.value;
            self.last_modified_time = other.last_modified_time;
            self.last_modified_user = other.last_modified_user;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpatialAnnotationInternal {
    pub id: AnnotationId,
    coord: LwwRegister<Option<Point>>,
    text: LwwRegister<Option<String>>,
}

impl SpatialAnnotationInternal {
    // pub fn new(value: SpatialAnnotation, user: UserId) -> Self {
    //     let coord = LwwRegister::new(value.coord, user);
    //     let text = LwwRegister::new(value.text, user);
    //     Self {
    //         id: AnnotationId(Uuid::new_v4()),
    //         coord,
    //         text,
    //     }
    // }

    pub(super) fn new(value: SpatialAnnotation, user: UserId, datetime: DateTime<Utc>) -> Self {
        let id = value.id
            .unwrap_or_else(|| AnnotationId(Uuid::new_v4()));
        let coord = LwwRegister::new_with_datetime(value.coord, user, datetime);
        let text = LwwRegister::new_with_datetime(value.text, user, datetime);
        Self { id, coord, text }
    }

    pub fn update(&mut self, other: SpatialAnnotation, user: UserId) {
        self.coord.update(other.coord, user);
        self.text.update(other.text, user);
    }

    pub fn merge(&mut self, other: Self) {
        self.coord.merge(other.coord);
        self.text.merge(other.text);
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



#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Timelike};

    use super::*;

    #[test]
    fn test_merge() {
        let annotation_id = Some(AnnotationId(Uuid::from_u128(1)));
        let left_values = (Point(0, 0), String::from("A"));
        let right_values = (Point(5, 5), String::from("B"));

        let baseline_user = UserId(Uuid::from_u128(1));
        let earlier_user = UserId(Uuid::from_u128(0));
        let later_user = UserId(Uuid::from_u128(2));
        let baseline_date: DateTime<Utc> = Utc
            .with_ymd_and_hms(2026, 1, 1, 0, 0, 1)
            .single()
            .expect("A valid UTC datetime");
        let earlier_date = baseline_date.with_second(0).expect("A valid UTC datetime");
        let later_date = baseline_date.with_second(2).expect("A valid UTC datetime");

        let tests = [
            ((baseline_user, baseline_date), (earlier_user,  earlier_date),  "left"),
            ((baseline_user, baseline_date), (earlier_user,  baseline_date), "right"),
            ((baseline_user, baseline_date), (earlier_user,  later_date),    "right"),
            ((baseline_user, baseline_date), (baseline_user, earlier_date),  "left"),
            ((baseline_user, baseline_date), (baseline_user, baseline_date), "left"),
            ((baseline_user, baseline_date), (baseline_user, later_date),    "right"),
            ((baseline_user, baseline_date), (later_user,    earlier_date),  "left"),
            ((baseline_user, baseline_date), (later_user,    baseline_date), "left"),
            ((baseline_user, baseline_date), (later_user,    later_date),    "right"),
        ];
        for scenario in tests {
            let ((left_user, left_date), (right_user, right_date), expected) = scenario;
            let (left_coord, left_str) = left_values.clone();
            let (right_coord, right_str) = right_values.clone();
            let mut left = SpatialAnnotationInternal::new(
                SpatialAnnotation::new(annotation_id, left_coord, left_str),
                left_user,
                left_date
            );
            let right = SpatialAnnotationInternal::new(
                SpatialAnnotation::new(annotation_id, right_coord, right_str),
                right_user,
                right_date
            );
            let expected = match expected {
                "left" => left.clone(),
                "right" => right.clone(),
                _ => panic!("Bad expected value")
            };

            left.merge(right);
            assert_eq!(left, expected);
        }
    }
}