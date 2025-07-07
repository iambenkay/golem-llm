use crate::vector::types;

pub fn from_reqwest_error(e: reqwest::Error) -> types::VectorError {
    if e.is_request() || e.is_timeout() || e.is_redirect() {
        return types::VectorError::ConnectionError(format!(
            "The connection could not be established: {e}",
        ));
    }

    if e.is_decode() {
        return types::VectorError::ProviderError(format!(
            "The provider returned an invalid response: {}",
            e
        ));
    }

    types::VectorError::ProviderError(format!("An unknown error occurred: {e}"))
}
