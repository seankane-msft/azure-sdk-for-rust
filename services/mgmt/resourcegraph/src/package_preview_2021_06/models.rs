#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
    #[serde(rename = "managementGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<String>,
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<QueryRequestOptions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<FacetRequest>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRequestOptions {
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[serde(rename = "$skip", default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(rename = "resultFormat", default, skip_serializing_if = "Option::is_none")]
    pub result_format: Option<query_request_options::ResultFormat>,
    #[serde(rename = "allowPartialScopes", default, skip_serializing_if = "Option::is_none")]
    pub allow_partial_scopes: Option<bool>,
    #[serde(rename = "authorizationScopeFilter", default, skip_serializing_if = "Option::is_none")]
    pub authorization_scope_filter: Option<query_request_options::AuthorizationScopeFilter>,
}
pub mod query_request_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultFormat {
        #[serde(rename = "table")]
        Table,
        #[serde(rename = "objectArray")]
        ObjectArray,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthorizationScopeFilter {
        AtScopeAndBelow,
        AtScopeAndAbove,
        AtScopeExact,
        AtScopeAboveAndBelow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetRequest {
    pub expression: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<FacetRequestOptions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetRequestOptions {
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<facet_request_options::SortOrder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
}
pub mod facet_request_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SortOrder {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResponse {
    #[serde(rename = "totalRecords")]
    pub total_records: i64,
    pub count: i64,
    #[serde(rename = "resultTruncated")]
    pub result_truncated: query_response::ResultTruncated,
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    pub data: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<Facet>,
}
pub mod query_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultTruncated {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: ColumnDataType,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ColumnDataType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "datetime")]
    Datetime,
}
pub type Row = Vec<serde_json::Value>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Facet {
    pub expression: String,
    #[serde(rename = "resultType")]
    pub result_type: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetResult {
    #[serde(flatten)]
    pub facet: Facet,
    #[serde(rename = "totalRecords")]
    pub total_records: i64,
    pub count: i32,
    pub data: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetError {
    #[serde(flatten)]
    pub facet: Facet,
    pub errors: Vec<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: Error,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcesHistoryRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<ResourcesHistoryRequestOptions>,
    #[serde(rename = "managementGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcesHistoryRequestOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<DateTimeInterval>,
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[serde(rename = "$skip", default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[serde(rename = "resultFormat", default, skip_serializing_if = "Option::is_none")]
    pub result_format: Option<resources_history_request_options::ResultFormat>,
}
pub mod resources_history_request_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultFormat {
        #[serde(rename = "table")]
        Table,
        #[serde(rename = "objectArray")]
        ObjectArray,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateTimeInterval {
    pub start: String,
    pub end: String,
}
