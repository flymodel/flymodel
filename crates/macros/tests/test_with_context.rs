use flymodel_macros::WithContext;

#[derive(WithContext, PartialEq, Eq, Debug)]
#[context_needs(
    #[derive(Clone)]
)]
pub struct Test {
    pub abc: i64,
    #[context]
    pub c: usize,
}

#[test]
fn test_needs_context() {
    let t = TestWithContext::new(2);
    let t = t.clone();
    let y = t.with_context(4);

    assert_eq!(y, Test { abc: 2, c: 4 });
}
