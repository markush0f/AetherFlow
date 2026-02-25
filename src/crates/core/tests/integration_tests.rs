use aether_core::{Director, Runtime};
use std::fs::File;
use std::io::Write;

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
async fn test_director_spawn_and_communicate() {
    let director = Director::new();
    let agent_id = "test_agent".to_string();

    // Configurar un entorno temporal
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_agent.py");
    let mut file = File::create(&script_path).unwrap();

    // Creamos un script en python que hace un echo de la entrada y luego el delimitador
    file.write_all(b"import sys\n").unwrap();
    file.write_all(b"input_data = sys.stdin.readline().strip()\n")
        .unwrap();
    file.write_all(b"print(f'ECHO: {input_data}')\n").unwrap();
    file.write_all(b"print('__AETHER_DONE__')\n").unwrap();

    let message = "hello aether".to_string();

    let result = director
        .execute_task(
            agent_id.clone(),
            Runtime::Python3,
            script_path.to_str().unwrap().to_string(),
            temp_dir.to_str().unwrap().to_string(),
            message.clone(),
        )
        .await;

    assert!(result.is_ok(), "Failed to communicate: {:?}", result.err());
    let response = result.unwrap();
    assert!(response.contains("ECHO: hello aether"));
}

#[tokio::test]
async fn test_director_spawn_failure() {
    let director = Director::new();

    // Spawning un script que no existe provocará que el proceso termine inmediatamente
    // y no envíe el delimitador.
    let response = director
        .execute_task(
            "broken_agent".to_string(),
            Runtime::Python3,
            "does_not_exist_in_the_universe.py".to_string(),
            ".".to_string(),
            "ping".to_string(),
        )
        .await;

    assert!(response.is_err());
}
