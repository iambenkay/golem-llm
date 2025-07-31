use golem_vector::{
    error::from_reqwest_error,
    vector::{collections, connection, search, search_extended, types},
};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::{json, Map};

use crate::client::QdrantPoint;

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
        let message = response.json::<serde_json::Value>().map(|v| {
            if let Some(error) = v
                .as_object()
                .and_then(|v| v.get("status")?.as_object()?.get("error")?.as_str())
            {
                return error.to_string();
            }
            return "The requested resource does not exist".to_string();
        });
        return Err(types::VectorError::NotFound(message.unwrap_or(
            "Unspecified error message returned from Qdrant".to_string(),
        )));
    }

    response.json().map_err(from_reqwest_error)
}

pub fn json_to_metadata(json: &serde_json::Value) -> types::MetadataValue {
    match json {
        serde_json::Value::Null => types::MetadataValue::NullVal,
        serde_json::Value::Bool(boolean) => types::MetadataValue::BooleanVal(*boolean),
        serde_json::Value::Number(number) => {
            if let Some(num) = number.as_f64() {
                types::MetadataValue::NumberVal(num)
            } else if let Some(num) = number.as_i64() {
                types::MetadataValue::IntegerVal(num)
            } else {
                types::MetadataValue::NullVal
            }
        }
        serde_json::Value::String(string) => types::MetadataValue::StringVal(string.clone()),
        serde_json::Value::Array(arr) => types::MetadataValue::ArrayVal(
            arr.into_iter()
                .map(|v| types::LazyMetadataValue::new(json_to_metadata(v)))
                .collect(),
        ),
        serde_json::Value::Object(map) => types::MetadataValue::ObjectVal(
            map.into_iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        types::LazyMetadataValue::new(json_to_metadata(v)),
                    )
                })
                .collect(),
        ),
    }
}

pub fn metadata_to_json(metadata: &types::MetadataValue) -> serde_json::Value {
    match metadata {
        types::MetadataValue::NullVal => serde_json::Value::Null,
        types::MetadataValue::BooleanVal(boolean) => json!(boolean),
        types::MetadataValue::NumberVal(number) => json!(number),
        types::MetadataValue::IntegerVal(number) => json!(number),
        types::MetadataValue::StringVal(string) => json!(string),
        types::MetadataValue::ArrayVal(arr) => json!(arr
            .into_iter()
            .map(|v| metadata_to_json(v.get::<types::MetadataValue>()))
            .collect::<Vec<serde_json::Value>>()),
        types::MetadataValue::ObjectVal(map) => json!(map.into_iter().fold(
            Map::<String, serde_json::Value>::new(),
            |mut acc, (k, v)| {
                acc.insert(k.clone(), metadata_to_json(v.get::<types::MetadataValue>()));
                acc
            },
        )),
        types::MetadataValue::GeoVal(geo_coordinates) => {
            let mut map = serde_json::Map::<String, serde_json::Value>::new();
            map["latitude"] = json!(geo_coordinates.latitude);
            map["longitude"] = json!(geo_coordinates.longitude);
            json!(map)
        }
        types::MetadataValue::DatetimeVal(date) => json!(date),
        types::MetadataValue::BlobVal(items) => json!(items
            .into_iter()
            .map(|v| json!(v))
            .collect::<Vec<serde_json::Value>>()),
    }
}

pub fn to_create_collection_body(
    size: u32,
    distance_metric: types::DistanceMetric,
    index_config: Option<collections::IndexConfig>,
    metadata: Option<types::Metadata>,
) -> Result<serde_json::Value, types::VectorError> {
    let distance = to_supported_distance_metric(distance_metric).ok_or(
        types::VectorError::UnsupportedFeature(format!(
            "Distance metric is not supported by Qdrant: {distance_metric:?}"
        )),
    )?;

    let mut body = json!({
        "vectors": {
            "size": size,
            "distance": distance,
        }
    });

    let map = body.as_object_mut().unwrap();
    if let Some(mut json) = metadata.map(metadata_to_json_map) {
        map.append(&mut json);
    }

    let index_type = index_config.and_then(|index| index.index_type);

    if let Some(index_type) = index_type {
        if let Some(index) = map
            .entry("sparse_vectors")
            .or_insert(json!({}))
            .as_object_mut()
            .and_then(|f| f.entry("index").or_insert(json!({})).as_object_mut())
        {
            index["datatype"] = json!(index_type);
        }
    }

    Ok(body)
}

fn to_supported_distance_metric(distance_metric: types::DistanceMetric) -> Option<String> {
    match distance_metric {
        types::DistanceMetric::Cosine => Some("Cosine".to_string()),
        types::DistanceMetric::Euclidean => Some("Euclid".to_string()),
        types::DistanceMetric::DotProduct => Some("Dot".to_string()),
        types::DistanceMetric::Manhattan => Some("Manhattan".to_string()),
        _ => None,
    }
}

pub fn from_vector_data_to_qdrant_vector(
    vector: types::VectorData,
) -> Result<serde_json::Value, types::VectorError> {
    let vectors = match vector {
        types::VectorData::Dense(dense_vector) => json!(dense_vector),
        types::VectorData::Sparse(sparse_vector) => {
            if sparse_vector.indices.len() != sparse_vector.values.len() {
                return Err(types::VectorError::InvalidParams("For sparse vectors, the number of indices must be the same as the number of values".to_string()));
            }
            json!({
                "indices": sparse_vector.indices,
                "values": sparse_vector.values,
            })
        }
        types::VectorData::Named(items) => {
            let map = items
                .into_iter()
                .fold(serde_json::Map::new(), |mut acc, (k, v)| {
                    acc.insert(k, json!(v));
                    acc
                });
            json!(map)
        }
        vector_type => {
            let feature_name = match vector_type {
                types::VectorData::Binary(..) => "Binary",
                types::VectorData::Half(..) => "Half",
                types::VectorData::Hybrid(..) => "Hybrid",
                _ => "UnknownType",
            };

            return Err(types::VectorError::UnsupportedFeature(format!(
                "{feature_name} vectors are not supported by Qdrant"
            )));
        }
    };
    Ok(vectors)
}

pub fn from_vector_record_to_qdrant_point(
    vector_record: types::VectorRecord,
) -> Result<QdrantPoint, types::VectorError> {
    let vectors = from_vector_data_to_qdrant_vector(vector_record.vector)?;
    Ok(QdrantPoint {
        id: from_id_to_json_value(vector_record.id),
        payload: vector_record
            .metadata
            .map(|m| json!(metadata_to_json_map(m))),
        vector: vectors,
    })
}

pub fn from_qdrant_vector_to_vector_data(
    vectors: serde_json::Value,
) -> Result<types::VectorData, types::VectorError> {
    let vector_data = match vectors {
        serde_json::Value::Array(values) => {
            if !values.is_empty() {
                if values.iter().all(serde_json::Value::is_f64) {
                    types::VectorData::Dense(
                        values
                            .into_iter()
                            .map(|v| v.as_f64().unwrap() as f32)
                            .collect(),
                    )
                } else if values
                    .iter()
                    .all(|v| v.is_array() && v.as_array().unwrap().len() == 2)
                {
                    unimplemented!()
                } else {
                    unimplemented!()
                }
            } else {
                types::VectorData::Dense(vec![])
            }
        }
        serde_json::Value::Object(_map) => unimplemented!(),
        value => {
            return Err(types::VectorError::ProviderError(format!(
                "Invalid vector value received from Qdrant: {value}"
            )))
        }
    };

    Ok(vector_data)
}

pub fn from_qdrant_point_to_vector_record(
    point: QdrantPoint,
) -> Result<types::VectorRecord, types::VectorError> {
    let vector =
        from_qdrant_vector_to_vector_data(point.vector).unwrap_or(types::VectorData::Dense(vec![]));

    Ok(types::VectorRecord {
        id: from_json_value_to_id(point.id)?,
        vector,
        metadata: point
            .payload
            .and_then(|v| v.as_object().map(json_map_to_metadata)),
    })
}

pub fn from_id_to_json_value(id: types::Id) -> serde_json::Value {
    match id {
        types::Id::Uint(uint) => json!(uint),
        types::Id::Str(str) => json!(str),
    }
}

pub fn from_json_value_to_id(value: serde_json::Value) -> Result<types::Id, types::VectorError> {
    match value {
        serde_json::Value::Number(number) => {
            if let Some(id) = number.as_u64() {
                Ok(types::Id::Uint(id))
            } else {
                Err(types::VectorError::ProviderError(format!(
                    "Invalid id type received from Qdrant: {number}"
                )))
            }
        }
        serde_json::Value::String(str) => Ok(types::Id::Str(str)),
        value => Err(types::VectorError::ProviderError(format!(
            "Invalid id type received from Qdrant: {value}"
        ))),
    }
}

pub fn metadata_to_json_map(
    metadata: types::Metadata,
) -> serde_json::Map<String, serde_json::Value> {
    metadata.into_iter().fold(Map::new(), |mut acc, (k, v)| {
        acc.insert(k, metadata_to_json(&v));
        acc
    })
}

pub fn json_map_to_metadata(json: &serde_json::Map<String, serde_json::Value>) -> types::Metadata {
    json.into_iter()
        .map(|(k, v)| (k.clone(), json_to_metadata(v)))
        .collect()
}

pub fn from_filter_expression_to_qdrant_filter(
    filter: &types::FilterExpression,
) -> Result<serde_json::Value, types::VectorError> {
    match filter {
        types::FilterExpression::Condition(filter_condition) => {
            from_filter_condition_to_qdrant_condition(filter_condition)
        }
        types::FilterExpression::And(lazy_filter_expressions) => {
            let mut conditions = vec![];

            for lazy_expression in lazy_filter_expressions {
                conditions.push(from_filter_expression_to_qdrant_filter(
                    lazy_expression.get::<types::FilterExpression>(),
                )?);
            }

            Ok(json!({"must": conditions}))
        }
        types::FilterExpression::Or(lazy_filter_expressions) => {
            let mut conditions = vec![];

            for lazy_expression in lazy_filter_expressions {
                conditions.push(from_filter_expression_to_qdrant_filter(
                    lazy_expression.get::<types::FilterExpression>(),
                )?);
            }

            Ok(json!({"should": conditions}))
        }
        types::FilterExpression::Not(lazy_filter_expression) => {
            let expr = lazy_filter_expression.get::<types::FilterExpression>();

            match expr {
                types::FilterExpression::Condition(filter_condition) => {
                    Ok(json!({"must_not": from_filter_condition_to_qdrant_condition(filter_condition)?}))
                },
                _ => Err(types::VectorError::UnsupportedFeature(format!(
                    "Nesting operator {expr:?} in a logical Not operator is not supported by Qdrant. Use Ne in filter condition instead"
                )))
            }
        }
    }
}

pub fn from_filter_condition_to_qdrant_condition(
    condition: &types::FilterCondition,
) -> Result<serde_json::Value, types::VectorError> {
    let result = match condition.operator {
        types::FilterOperator::Eq => match condition.value {
            types::MetadataValue::NullVal => json!({
                "key": condition.field,
                "is_null": true,
            }),
            _ => json!({
                    "key": condition.field,
                    "match": {
                        "value": metadata_to_json(&condition.value)
                    }
            }),
        },
        types::FilterOperator::Ne => json!({
            "key": condition.field,
            "match": {
                "except": metadata_to_json(&condition.value)
            }
        }),
        types::FilterOperator::Gt => json!({
            "key": condition.field,
            "range": {
                "gt": metadata_to_json(&condition.value)
            }
        }),
        types::FilterOperator::Gte => json!({
            "key": condition.field,
            "range": {
                "gte": metadata_to_json(&condition.value)
            }
        }),
        types::FilterOperator::Lt => json!({
            "key": condition.field,
            "range": {
                "lt": metadata_to_json(&condition.value)
            }
        }),
        types::FilterOperator::Lte => json!({
            "key": condition.field,
            "range": {
                "lte": metadata_to_json(&condition.value)
            }
        }),
        types::FilterOperator::GeoWithin => json!({
            "key": condition.field,
            "geo_radius": metadata_to_json(&condition.value)
        }),
        types::FilterOperator::GeoBbox => json!({
            "key": condition.field,
            "geo_bounding_box": metadata_to_json(&condition.value)
        }),
        operator => {
            return Err(types::VectorError::UnsupportedFeature(format!(
                "The filter operator {operator:?} is not supported by Qdrant"
            )))
        }
    };

    Ok(result)
}

pub fn from_search_query_to_qdrant_query(
    search_query: search::SearchQuery,
) -> Result<serde_json::Value, types::VectorError> {
    let value = match search_query {
        search::SearchQuery::ById(id) => from_id_to_json_value(id),
        search::SearchQuery::Vector(vector_data) => from_vector_data_to_qdrant_vector(vector_data)?,
        search::SearchQuery::MultiVector(items) => {
            let mut map = serde_json::Map::new();
            for (key, value) in items {
                map.insert(key, from_vector_data_to_qdrant_vector(value)?);
            }
            json!(map)
        }
    };

    Ok(json!({
        "nearest": value,
    }))
}

pub fn from_recommendation_example_to_qdrant_query(
    recommendation_example: search_extended::RecommendationExample,
) -> Result<serde_json::Value, types::VectorError> {
    match recommendation_example {
        search_extended::RecommendationExample::VectorId(id) => Ok(from_id_to_json_value(id)),
        search_extended::RecommendationExample::VectorData(vector_data) => {
            from_vector_data_to_qdrant_vector(vector_data)
        }
    }
}

pub fn from_recommendation_strategy_to_qdrant_strategy(
    strategy: search_extended::RecommendationStrategy,
) -> String {
    match strategy {
        search_extended::RecommendationStrategy::AverageVector => "average_vector".to_string(),
        search_extended::RecommendationStrategy::BestScore => "best_score".to_string(),
        search_extended::RecommendationStrategy::Centroid => "sum_scores".to_string(),
    }
}
