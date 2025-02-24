use serde::{Deserialize, Serialize};
use usecases::usecases::Usecases;

/// Represents different types of requests that can be made to the server.
///
/// The `Requests` enum is used to categorize and handle various actions
/// that the server can process. Each variant of the enum corresponds to
/// a specific type of request, with associated data as needed.
///
/// Example json request that turns off music:
///
/// ```json
/// {
///     "general": {
///         "action": "turnOffMusic"
///     }
/// }
/// ```
///
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Requests {
    /// A general request that includes an `Usecases` to be performed.
    Command {
        action: Usecases,
    },
    Human {
        message: String,
    },
}
