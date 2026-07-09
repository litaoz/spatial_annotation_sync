use proptest::prelude::*;
use spatial_annotation_sync::crdt::{AnnotationId, Point, SpatialAnnotation, SpatialEnvironment};
use uuid::Uuid;


#[derive(Debug, Clone)]
enum Op {
    Create,
    // Read,
    Update,
    Delete
}

#[derive(Debug, Clone)]
enum Operation {
    Create(AnnotationId, SpatialAnnotation),
    // Read,
    Update(AnnotationId, SpatialAnnotation),
    Delete(AnnotationId)
}


prop_compose!{
    fn arb_point(max_point: i32) (i in -max_point..=max_point, j in -max_point..=max_point) -> Point {
        Point(i, j)
    }
}

fn one_of_ops_strategy() -> impl Strategy<Value = Op> {
    prop_oneof![
        Just(Op::Create),
        // Just(Op::Read),
        Just(Op::Update),
        Just(Op::Delete)
    ].boxed()
}

fn ops_strategy() -> impl Strategy<Value = Vec<Op>> {
    prop::collection::vec(one_of_ops_strategy(), 1..10)
}

fn fix_up_ops(peer_tag: u128, ops: Vec<Op>, annotation_parts: Vec<(Point, String)>, selections: Vec<usize>, reuse_ids: Vec<bool>) -> Vec<Operation> {
    let mut known_ids: Vec<AnnotationId> = Vec::new();
    let mut result = Vec::new();

    let mut parts_iter = annotation_parts.into_iter();
    let mut selections_iter = selections.into_iter();
    let mut reuse_ids_iter = reuse_ids.into_iter();
    let mut next_id = 0_u128;

    for op in ops {
        match op {
            Op::Create => {
                let reuse_id = reuse_ids_iter
                    .next()
                    .expect("reuse_ids should be same length as ops");

                let id = match reuse_id {
                    true => {next_id += 1; AnnotationId(Uuid::from_u128(next_id))}
                    false => {AnnotationId(Uuid::from_u128((peer_tag as u128) << 96 | next_id as u128))}
                };


                let (coord, text) = parts_iter
                    .next()
                    .expect("annotation_parts should be same length as ops");

                let item = SpatialAnnotation::new(Some(id), coord, text);
                known_ids.push(id);
                result.push(Operation::Create(id, item));
            }
            // Op::Read | Op::Update | Op::Delete => {
            Op::Update => {
                if known_ids.is_empty() {
                    // no valid target yet — skip this intent
                    continue;
                }
                let selection = selections_iter.next().expect("selections iter should be same length as ops");
                let idx = selection % known_ids.len();
                let target_id = known_ids[idx];

                let (coord, text) = parts_iter
                    .next()
                    .expect("annotation_parts must be at least as long as ops");
                let item = SpatialAnnotation::new(Some(target_id), coord, text);
                result.push(Operation::Update(target_id, item));
            },
            Op::Delete => {
                if known_ids.is_empty() {
                    // no valid target yet — skip this intent
                    continue;
                }
                let selection = selections_iter.next().unwrap_or(0);
                let idx = selection % known_ids.len();
                let target_id = known_ids[idx];
                result.push(Operation::Delete(target_id))
            }
        }
    }

    result
}

fn annotation_parts_strategy() -> impl Strategy<Value = (Point, String)> {
    (arb_point(100), "[a-zA-Z0-9 ]{0,20}")
}

fn operations_strategy(peer_tag: u128) -> impl Strategy<Value = Vec<Operation>> {
    ops_strategy().prop_flat_map(move |ops |{
        let len = ops.len();
        (
            Just(ops),
            prop::collection::vec(annotation_parts_strategy(), len),
            prop::collection::vec(any::<usize>(), len),
            prop::collection::vec(prop::bool::weighted(0.8), len)
        ).prop_map(move |(ops, annotation_parts, selections, reuse_ids)| {
            fix_up_ops(peer_tag, ops, annotation_parts, selections, reuse_ids)
        })
    })
}

fn load_env(ops: Vec<Operation>, env: &mut SpatialEnvironment) {
    for op in ops {
            match op {
                Operation::Create(_id, ann) => {env.create_annotation(ann);}
                Operation::Update(_id, ann) => {env.update_annotation(ann);}
                Operation::Delete(id) => {env.delete_annotation(id);}
            }
        }

}

proptest! {
    #[test]
    fn commutativity(ops_a in operations_strategy(1), ops_b in operations_strategy(2)) {
        let mut env_a = SpatialEnvironment::new();
        let mut env_b = SpatialEnvironment::new();

        load_env(ops_a, &mut env_a);
        load_env(ops_b, &mut env_b);

        let mut a_b = env_a.clone();
        a_b.merge(env_b.clone());

        let mut b_a = env_b.clone();
        b_a.merge(env_a.clone());

        assert!(a_b.has_same_data(&b_a));
    }

    #[test]
    fn associativity(ops_a in operations_strategy(1), ops_b in operations_strategy(2), ops_c in operations_strategy(3)) {
        let mut env_a = SpatialEnvironment::new();
        let mut env_b = SpatialEnvironment::new();
        let mut env_c = SpatialEnvironment::new();

        load_env(ops_a, &mut env_a);
        load_env(ops_b, &mut env_b);
        load_env(ops_c, &mut env_c);

        let mut ab_c = env_a.clone();
        ab_c.merge(env_b.clone());
        ab_c.merge(env_c.clone());

        let mut bc = env_b.clone();
        bc.merge(env_c.clone());
        let mut a_bc = env_a.clone();
        a_bc.merge(bc);

        assert!(ab_c.has_same_data(&a_bc), "'merge(merge(a b), c)' should be the same as 'merge(a, merge(b, c)'");
    }

    #[test]
    fn idempotence(ops_a in operations_strategy(1)) {
        let mut env_a = SpatialEnvironment::new();

        load_env(ops_a, &mut env_a);

        let mut a_a = env_a.clone();
        a_a.merge(env_a.clone());

        assert!(env_a.has_same_data(&a_a));
    }
}