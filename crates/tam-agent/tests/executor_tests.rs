use tam_agent::pm2::executor::{Pm2Executor, CommandExecutionResult};

#[tokio::test]
async fn test_pm2_command_validation_and_dry_run() {
    let executor = Pm2Executor::new_dry_run();

    let res = executor.execute("ecommerce-api", "restart").await;
    assert!(res.is_ok());
    let outcome: CommandExecutionResult = res.unwrap();
    assert_eq!(outcome.status, "success");
    assert!(outcome.output.contains("ecommerce-api"));

    // Test invalid command action
    let invalid = executor.execute("ecommerce-api", "destroy_everything").await;
    assert!(invalid.is_err());
}

