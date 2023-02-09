use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum OrganismError {
    #[error("failed to load organism")]
    LoadOrganismError,
    #[error("failed to parse organism or config from json file")]
    ParseJSONError(#[from] serde_json::Error),
    #[error("failed to open organism or config file")]
    IOError(#[from] std::io::Error),
    #[error("failed to parse recognizer object")]
    RecognizerError(#[from] RecognizerError),
    #[error("failed to parse connector object")]
    ConnectorError(#[from] ConnectorError),
}

#[derive(thiserror::Error, Debug)]
pub enum RecognizerError {
    #[error("failed to load recognizer")]
    LoadRecognizerError,

    #[error("failed to parse recognizer from JSON value")]
    ParseJSONError(#[from] serde_json::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ConnectorError {
    #[error("failed to load connector")]
    LoadConnectorError,
    #[error("failed to parse connector from json file")]
    ParseJSONError(#[from] serde_json::Error),
}