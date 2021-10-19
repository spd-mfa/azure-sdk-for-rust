#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, models::*};
pub mod metrics {
    use super::{models, models::*};
    pub async fn list_at_subscription_scope(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        region: &str,
        timespan: Option<&str>,
        interval: Option<&str>,
        metricnames: Option<&str>,
        aggregation: Option<&str>,
        top: Option<i32>,
        orderby: Option<&str>,
        filter: Option<&str>,
        result_type: Option<&str>,
        metricnamespace: Option<&str>,
        auto_adjust_timegrain: Option<bool>,
        validate_dimensions: Option<bool>,
    ) -> std::result::Result<SubscriptionScopeMetricResponse, list_at_subscription_scope::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Insights/metrics",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).map_err(list_at_subscription_scope::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_at_subscription_scope::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        url.query_pairs_mut().append_pair("region", region);
        if let Some(timespan) = timespan {
            url.query_pairs_mut().append_pair("timespan", timespan);
        }
        if let Some(interval) = interval {
            url.query_pairs_mut().append_pair("interval", interval);
        }
        if let Some(metricnames) = metricnames {
            url.query_pairs_mut().append_pair("metricnames", metricnames);
        }
        if let Some(aggregation) = aggregation {
            url.query_pairs_mut().append_pair("aggregation", aggregation);
        }
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("top", top.to_string().as_str());
        }
        if let Some(orderby) = orderby {
            url.query_pairs_mut().append_pair("orderby", orderby);
        }
        if let Some(filter) = filter {
            url.query_pairs_mut().append_pair("$filter", filter);
        }
        if let Some(result_type) = result_type {
            url.query_pairs_mut().append_pair("resultType", result_type);
        }
        if let Some(metricnamespace) = metricnamespace {
            url.query_pairs_mut().append_pair("metricnamespace", metricnamespace);
        }
        if let Some(auto_adjust_timegrain) = auto_adjust_timegrain {
            url.query_pairs_mut()
                .append_pair("AutoAdjustTimegrain", auto_adjust_timegrain.to_string().as_str());
        }
        if let Some(validate_dimensions) = validate_dimensions {
            url.query_pairs_mut()
                .append_pair("ValidateDimensions", validate_dimensions.to_string().as_str());
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(list_at_subscription_scope::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_at_subscription_scope::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: SubscriptionScopeMetricResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_at_subscription_scope::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_at_subscription_scope::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list_at_subscription_scope::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_at_subscription_scope {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn list_at_subscription_scope_post(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        region: &str,
        timespan: Option<&str>,
        interval: Option<&str>,
        metricnames: Option<&str>,
        aggregation: Option<&str>,
        top: Option<i32>,
        orderby: Option<&str>,
        filter: Option<&str>,
        result_type: Option<&str>,
        metricnamespace: Option<&str>,
        auto_adjust_timegrain: Option<bool>,
        validate_dimensions: Option<bool>,
        body: Option<&SubscriptionScopeMetricsRequestBodyParameters>,
    ) -> std::result::Result<SubscriptionScopeMetricResponse, list_at_subscription_scope_post::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Insights/metrics",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).map_err(list_at_subscription_scope_post::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_at_subscription_scope_post::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        url.query_pairs_mut().append_pair("region", region);
        if let Some(timespan) = timespan {
            url.query_pairs_mut().append_pair("timespan", timespan);
        }
        if let Some(interval) = interval {
            url.query_pairs_mut().append_pair("interval", interval);
        }
        if let Some(metricnames) = metricnames {
            url.query_pairs_mut().append_pair("metricnames", metricnames);
        }
        if let Some(aggregation) = aggregation {
            url.query_pairs_mut().append_pair("aggregation", aggregation);
        }
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("top", top.to_string().as_str());
        }
        if let Some(orderby) = orderby {
            url.query_pairs_mut().append_pair("orderby", orderby);
        }
        if let Some(filter) = filter {
            url.query_pairs_mut().append_pair("$filter", filter);
        }
        if let Some(result_type) = result_type {
            url.query_pairs_mut().append_pair("resultType", result_type);
        }
        if let Some(metricnamespace) = metricnamespace {
            url.query_pairs_mut().append_pair("metricnamespace", metricnamespace);
        }
        if let Some(auto_adjust_timegrain) = auto_adjust_timegrain {
            url.query_pairs_mut()
                .append_pair("AutoAdjustTimegrain", auto_adjust_timegrain.to_string().as_str());
        }
        if let Some(validate_dimensions) = validate_dimensions {
            url.query_pairs_mut()
                .append_pair("ValidateDimensions", validate_dimensions.to_string().as_str());
        }
        let req_body = if let Some(body) = body {
            req_builder = req_builder.header("content-type", "application/json");
            azure_core::to_json(body).map_err(list_at_subscription_scope_post::Error::SerializeError)?
        } else {
            bytes::Bytes::from_static(azure_core::EMPTY_BODY)
        };
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(list_at_subscription_scope_post::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_at_subscription_scope_post::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: SubscriptionScopeMetricResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_at_subscription_scope_post::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_at_subscription_scope_post::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list_at_subscription_scope_post::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_at_subscription_scope_post {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        timespan: Option<&str>,
        interval: Option<&str>,
        metricnames: Option<&str>,
        aggregation: Option<&str>,
        top: Option<i32>,
        orderby: Option<&str>,
        filter: Option<&str>,
        result_type: Option<&str>,
        metricnamespace: Option<&str>,
        auto_adjust_timegrain: Option<bool>,
        validate_dimensions: Option<bool>,
    ) -> std::result::Result<Response, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.Insights/metrics",
            operation_config.base_path(),
            resource_uri
        );
        let mut url = url::Url::parse(url_str).map_err(list::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        if let Some(timespan) = timespan {
            url.query_pairs_mut().append_pair("timespan", timespan);
        }
        if let Some(interval) = interval {
            url.query_pairs_mut().append_pair("interval", interval);
        }
        if let Some(metricnames) = metricnames {
            url.query_pairs_mut().append_pair("metricnames", metricnames);
        }
        if let Some(aggregation) = aggregation {
            url.query_pairs_mut().append_pair("aggregation", aggregation);
        }
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("top", top.to_string().as_str());
        }
        if let Some(orderby) = orderby {
            url.query_pairs_mut().append_pair("orderby", orderby);
        }
        if let Some(filter) = filter {
            url.query_pairs_mut().append_pair("$filter", filter);
        }
        if let Some(result_type) = result_type {
            url.query_pairs_mut().append_pair("resultType", result_type);
        }
        if let Some(metricnamespace) = metricnamespace {
            url.query_pairs_mut().append_pair("metricnamespace", metricnamespace);
        }
        if let Some(auto_adjust_timegrain) = auto_adjust_timegrain {
            url.query_pairs_mut()
                .append_pair("AutoAdjustTimegrain", auto_adjust_timegrain.to_string().as_str());
        }
        if let Some(validate_dimensions) = validate_dimensions {
            url.query_pairs_mut()
                .append_pair("ValidateDimensions", validate_dimensions.to_string().as_str());
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Response =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
}
pub mod metric_definitions {
    use super::{models, models::*};
    pub async fn list_at_subscription_scope(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        region: &str,
        metricnamespace: Option<&str>,
    ) -> std::result::Result<SubscriptionScopeMetricDefinitionCollection, list_at_subscription_scope::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Insights/metricDefinitions",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).map_err(list_at_subscription_scope::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_at_subscription_scope::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        url.query_pairs_mut().append_pair("region", region);
        if let Some(metricnamespace) = metricnamespace {
            url.query_pairs_mut().append_pair("metricnamespace", metricnamespace);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(list_at_subscription_scope::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_at_subscription_scope::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: SubscriptionScopeMetricDefinitionCollection = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_at_subscription_scope::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_at_subscription_scope::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list_at_subscription_scope::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_at_subscription_scope {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        metricnamespace: Option<&str>,
    ) -> std::result::Result<MetricDefinitionCollection, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.Insights/metricDefinitions",
            operation_config.base_path(),
            resource_uri
        );
        let mut url = url::Url::parse(url_str).map_err(list::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        if let Some(metricnamespace) = metricnamespace {
            url.query_pairs_mut().append_pair("metricnamespace", metricnamespace);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: MetricDefinitionCollection =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
}
pub mod operations {
    use super::{models, models::*};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.Insights/operations", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(list::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: OperationListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorContract =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
}