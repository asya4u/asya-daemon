use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum AiRecognizeMethod {
    Groq,
    AltaS,
    #[default]
    None,
}
