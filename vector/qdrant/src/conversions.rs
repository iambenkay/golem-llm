use golem_vector::{
    error::from_reqwest_error,
    vector::{connection, types},
};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub fn get_api_key(
    credentials: Option<connection::Credentials>,
) -> Result<Option<String>, types::VectorError> {
    match credentials {
        Some(connection::Credentials::ApiKey(api_key)) => Ok(Some(api_key)),
        None => Ok(None),
        Some(..) => Err(types::VectorError::InvalidParams(
            "Invalid credential type supplied. Requires `ApiKey`".to_string(),
        )),
    }
}

pub fn get_response_body<T>(response: reqwest::Response) -> Result<T, types::VectorError>
where
    T: DeserializeOwned,
{
    if matches!(response.status(), StatusCode::UNAUTHORIZED) {
        return Err(types::VectorError::Unauthorized(
            "The API key you supplied is not valid".to_string(),
        ));
    }

    if matches!(response.status(), StatusCode::NOT_FOUND) {
        let message = response
            .json::<serde_json::Value>()
            .map(|v| {
                if let Some(v) = v.as_object() {
                    if let Some(status) = v.get("status").and_then(serde_json::Value::as_object) {
                        if let Some(error) = status.get("error").and_then(serde_json::Value::as_str)
                        {
                            return error.to_string();
                        }
                    }
                }
                return "The requested resource does not exist".to_string();
            })
            .unwrap();
        return Err(types::VectorError::NotFound(message));
    }

    response.json().map_err(from_reqwest_error)
}

pub fn json_to_metadata(json: serde_json::Value) -> types::MetadataValue {
    match json {
        serde_json::Value::Null => types::MetadataValue::NullVal,
        serde_json::Value::Bool(boolean) => types::MetadataValue::BooleanVal(boolean),
        serde_json::Value::Number(number) => {
            if number.is_f64() {
                types::MetadataValue::NumberVal(number.as_f64().unwrap())
            } else {
                types::MetadataValue::IntegerVal(
                    number.as_i64().unwrap_or(number.as_u64().unwrap() as i64),
                )
            }
        }
        serde_json::Value::String(string) => types::MetadataValue::StringVal(string),
        serde_json::Value::Array(arr) => types::MetadataValue::ArrayVal(
            arr.into_iter()
                .map(|v| types::LazyMetadataValue::new(json_to_metadata(v)))
                .collect(),
        ),
        serde_json::Value::Object(map) => types::MetadataValue::ObjectVal(
            map.into_iter()
                .map(|(k, v)| (k, types::LazyMetadataValue::new(json_to_metadata(v))))
                .collect(),
        ),
    }
}
