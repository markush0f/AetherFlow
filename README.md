# AetherFlow

AetherFlow is an Agent Management API and remote execution engine. It allows you to register decentralized agents and seamlessly execute tasks against them via a robust, highly-concurrent Rust gateway, acting as a transparent proxy.

## Architecture

AetherFlow is composed of two primary parts:

1. **The Gateway (Rust):** Built using `Axum`, `SeaORM`, and `tokio`. It serves as the main API to manage agents in a PostgreSQL database and dynamically proxies tasks directly to the agents using an optimized, persistent `reqwest` HTTP client. Documentation automatically generated via Swagger/OpenAPI.
2. **The Agent Farm (TypeScript/Node.js):** A clean-architecture microservice ecosystem where intelligent agents live. The agents currently utilize a local instance of `Ollama` (`llama3.2`) to execute semantic tasks (translation, summarization, etc.).

## Components & Agents

The `agents/translator` folder acts as an independent farm running on TypeScript. Currently, three agents are implemented:

* **Translator Agent (Port 4000):** Translates incoming text into a specified target language. 
* **Summarizer Agent (Port 4001):** Provides concise summaries of incoming text blocks.
* **Sentiment Agent (Port 4002):** Analyzes text to determine if its sentiment is POSITIVE, NEGATIVE, or NEUTRAL.

## Prerequisites

* **Rust:** `>= 1.80` to compile and run the backend gateway.
* **Node.js:** `>= 18.x` and `npm` for the TypeScript agents.
* **Database:** PostgreSQL running and configured via `.env` (using the `DATABASE_URL` format).
* **Ollama:** A local instance of Ollama running on `http://127.0.0.1:11434` with the `llama3.2` model available (or configured otherwise).

## Getting Started

### 1. Database Migrations
Ensure your PostgreSQL instance is running and the `DATABASE_URL` is set in the `.env` file located in `src/crates/server`. AetherFlow will run migrations automatically upon startup.

### 2. Managing the Agents
We provide simple scripts in the root directory to manage the Node.js agent farm:

* **Start the Agents:** `./start_agents.sh` (Initializes the TypeScript server hosting all three agents).
* **Stop the Agents:** `./stop_agents.sh` (Kills any process bound to the agent ports cleanly).

Make sure you've installed their dependencies first:
```bash
cd agents/translator && npm install
```

### 3. Running the Gateway
Start the main Rust server (by default hosts on `http://127.0.0.1:8080/`):

```bash
cd src/crates/server
cargo run aether-server
```

## API Usage (Gateway)

You can explore and interact with the endpoints through the auto-generated Swagger UI:
👉 **[Swagger Docs](http://127.0.0.1:8080/docs)** 

### Register an Agent Example
To register the local Translation agent into the AetherFlow database:
```bash
curl -X POST http://127.0.0.1:8080/agents \
-H "Content-Type: application/json" \
-d '{
    "slug": "translator-ai",
    "endpoint": "http://127.0.0.1:4000/api/translate"
}'
```

### Execute a Task via the Gateway Example
After obtaining the UUID from the creation step:
```bash
curl -X POST http://127.0.0.1:8080/agents/<AGENT_UUID>/execute \
-H "Content-Type: application/json" \
-d '{
    "payload": {
        "text": "Hello world, AetherFlow is fully operational!",
        "target_language": "Spanish"
    }
}'
```

The Gateway acts as a transparent proxy, accepting JSON payloads and routing them efficiently directly to the underlying agents. 
