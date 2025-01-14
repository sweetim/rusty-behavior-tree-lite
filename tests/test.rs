// use std::convert::From;
use behavior_tree_lite::{
    hash_map, BehaviorCallback, BehaviorNode, BehaviorResult, Context, FallbackNode, SequenceNode,
    Symbol,
};

struct CheckMeNode;

impl BehaviorNode for CheckMeNode {
    fn tick(&mut self, _arg: BehaviorCallback, ctx: &mut Context) -> BehaviorResult {
        assert_eq!(Some(&"check me"), ctx.get(Symbol::from("check")));
        BehaviorResult::Success
    }
}

#[test]
fn test_check() {
    let mut ctx = Context::default();
    ctx.set(Symbol::from("check"), "check me");
    let mut print_arm = CheckMeNode;
    print_arm.tick(&mut |_| None, &mut ctx);
}

struct AlwaysSucceed;

impl BehaviorNode for AlwaysSucceed {
    fn tick(&mut self, _arg: BehaviorCallback, _ctx: &mut Context) -> BehaviorResult {
        BehaviorResult::Success
    }
}

struct AlwaysFail;

impl BehaviorNode for AlwaysFail {
    fn tick(&mut self, _arg: BehaviorCallback, _ctx: &mut Context) -> BehaviorResult {
        BehaviorResult::Fail
    }
}

#[test]
fn test_sequence() {
    let mut seq = SequenceNode::default();
    seq.add_child(Box::new(AlwaysSucceed), hash_map!()).unwrap();
    seq.add_child(Box::new(AlwaysSucceed), hash_map!()).unwrap();
    assert_eq!(
        seq.tick(&mut |_| None, &mut Context::default()),
        BehaviorResult::Success
    );
    seq.add_child(Box::new(AlwaysFail), hash_map!()).unwrap();
    assert_eq!(
        seq.tick(&mut |_| None, &mut Context::default()),
        BehaviorResult::Fail
    );
}

#[test]
fn test_fallback() {
    let mut seq = FallbackNode::default();
    seq.add_child(Box::new(AlwaysFail), hash_map!()).unwrap();
    seq.add_child(Box::new(AlwaysFail), hash_map!()).unwrap();
    assert_eq!(
        seq.tick(&mut |_| None, &mut Context::default()),
        BehaviorResult::Fail
    );
    seq.add_child(Box::new(AlwaysSucceed), hash_map!()).unwrap();
    assert_eq!(
        seq.tick(&mut |_| None, &mut Context::default()),
        BehaviorResult::Success
    );
}
