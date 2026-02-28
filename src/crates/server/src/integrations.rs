use sea_orm::prelude::async_trait::async_trait;
use serde_json::Value;

// Standardized interface for any AI provider integration.

#[async_trait]
pub trait AiProvider: Send + Sync {
    // Transforms internal task data into the specific provider's format.
    async fn build_payload(
        &self,
        model: &str,
        system_prompt: &str,
        input: &Value,
    ) -> Result<Value, String>;

    // Parses the raw JSON response from the provider into a clean string.
    async fn extract_response(&self, response: Value) -> Result<Value, String>;
}

pub mod anthropic;
pub mod gemini;
pub mod ollama;
pub mod openai;

// Factory to get the appropriate integration instance.
// pub fn get_integration(provider_name: &str) -> Box<dyn AiProvider> {
//     match provider_name.to_lowercase().as_str() {
//         "openai" => Box::new(openai::OpenAiIntegration),
//         "anthropic" => Box::new(anthropic::AnthropicIntegration),
//         "gemini" => Box::new(gemini::GeminiIntegration),
//         _ => Box::new(ollama::OllamaIntegration),
//     }
// }
