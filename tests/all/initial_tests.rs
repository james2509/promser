use crate::test_context::TestContext;

#[test]
pub fn validate_metrics_are_served() {
    // Arrange
    let context = TestContext::start();

    // Act
    let metrics = context.get_metrics();

    // Assert
    assert!(
        metrics.len() > 0,
        "length of metrics response expected to be greater than zero"
    );
}
