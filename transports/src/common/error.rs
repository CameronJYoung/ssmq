use std::fmt;

#[derive(Debug)]
pub enum TransportError {
    StartError(String),
    StopError(String),
    EmitError(String),
    ProcessError(String),
    ProtocolSpecificError(String),
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TransportError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_error() {
        let error = TransportError::StartError("start error".to_string());
        assert_eq!(format!("{}", error), "StartError(\"start error\")");

        let error = TransportError::StopError("stop error".to_string());
        assert_eq!(format!("{}", error), "StopError(\"stop error\")");

        let error = TransportError::EmitError("emit error".to_string());
        assert_eq!(format!("{}", error), "EmitError(\"emit error\")");

        let error = TransportError::ProcessError("process error".to_string());
        assert_eq!(format!("{}", error), "ProcessError(\"process error\")");

        let error = TransportError::ProtocolSpecificError("protocol specific error".to_string());
        assert_eq!(format!("{}", error), "ProtocolSpecificError(\"protocol specific error\")");
    }
}