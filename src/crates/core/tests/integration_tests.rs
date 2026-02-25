use aether_core::{process_manager::ProcessManager, Runtime};

#[test]
fn test_python_command() {
    let runtime = Runtime::Python3;
    let (prog, args) = runtime.build_command("app.py");
    assert_eq!(prog, "python3");
    assert_eq!(args, vec!["-u", "app.py"]);
}

#[test]
fn test_nodejs_command() {
    let runtime = Runtime::NodeJS;
    let (prog, args) = runtime.build_command("index.js");
    assert_eq!(prog, "node");
    assert_eq!(args, vec!["index.js"]);
}

#[test]
fn test_native_command() {
    let runtime = Runtime::Native;
    let (prog, args) = runtime.build_command("./binary");
    assert_eq!(prog, "./binary");
    assert!(args.is_empty());
}

#[test]
fn test_remote_api_command() {
    let runtime = Runtime::RemoteApi {
        endpoint: "http://localhost".to_string(),
        method: "POST".to_string(),
    };
    let (prog, _args) = runtime.build_command("");
    assert_eq!(prog, "network_call");
}

#[tokio::test]
async fn test_spawn_and_communicate() {
    let manager = ProcessManager::new();
    let agent_id = "test_agent".to_string();

    // Use 'cat' as a simple echo agent
    let result = manager
        .spawn_agent(
            agent_id.clone(),
            Runtime::Native,
            "cat".to_string(),
            ".".to_string(),
        )
        .await;

    assert!(result.is_ok(), "Failed to spawn cat: {:?}", result.err());

    let message = "hello aether";
    let response = manager.send_to_agent(&agent_id, message).await;

    assert!(
        response.is_ok(),
        "Failed to communicate: {:?}",
        response.err()
    );
    assert_eq!(response.unwrap(), message);
}

#[tokio::test]
async fn test_agent_not_found() {
    let manager = ProcessManager::new();
    let response = manager.send_to_agent("non_existent", "ping").await;
    assert!(response.is_err());
    assert!(response.err().unwrap().contains("not found"));
}
