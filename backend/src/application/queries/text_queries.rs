use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Query, QueryHandler};
use crate::{
    application::{
        services::text_service::TextService,
        dto::{
            requests::text_request::SearchTextsRequest,
            responses::text_response::{
                TextResponse,
                TextVersionResponse,
                TextSearchResponse,
                TextStatsResponse,
                TextAnalyticsResponse,
            },
        },
    },
    utils::error::AppError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTextQuery {
    pub text_id: Uuid,
    pub include_versions: bool,
    pub include_comments: bool,
    pub include_stats: bool,
}

impl Query for GetTextQuery {
    fn query_name(&self) -> &'static str {
        "GetTextQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTextVersionQuery {
    pub version_id: Uuid,
    pub include_text_info: bool,
}

impl Query for GetTextVersionQuery {
    fn query_name(&self) -> &'static str {
        "GetTextVersionQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTextsQuery {
    pub query: Option<String>,
    pub author_orcid: Option<String>,
    pub topic: Option<String>,
    pub status: Option<String>,
    pub min_rating: Option<f64>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: u32,
    pub limit: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Query for SearchTextsQuery {
    fn query_name(&self) -> &'static str {
        "SearchTextsQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTextStatsQuery {
    pub text_id: Uuid,
    pub period_days: Option<u32>,
}

impl Query for GetTextStatsQuery {
    fn query_name(&self) -> &'static str {
        "GetTextStatsQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTextAnalyticsQuery {
    pub include_top_lists: bool,
    pub period_days: Option<u32>,
}

impl Query for GetTextAnalyticsQuery {
    fn query_name(&self) -> &'static str {
        "GetTextAnalyticsQuery"
    }
}

pub struct TextQueryHandler {
    text_service: Box<dyn TextService>,
}

impl TextQueryHandler {
    pub fn new(text_service: Box<dyn TextService>) -> Self {
        Self { text_service }
    }
}

#[async_trait]
impl QueryHandler<GetTextQuery, TextResponse> for TextQueryHandler {
    async fn handle(&self, query: GetTextQuery) -> Result<TextResponse, AppError> {
        let response = self.text_service.get_text(query.text_id).await?;
        
        // TODO: Include versions, comments, stats if requested
        
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<GetTextVersionQuery, TextVersionResponse> for TextQueryHandler {
    async fn handle(&self, query: GetTextVersionQuery) -> Result<TextVersionResponse, AppError> {
        let response = self.text_service.get_version(query.version_id).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<SearchTextsQuery, TextSearchResponse> for TextQueryHandler {
    async fn handle(&self, query: SearchTextsQuery) -> Result<TextSearchResponse, AppError> {
        let request = SearchTextsRequest {
            query: query.query,
            author_orcid: query.author_orcid,
            topic: query.topic,
            status: query.status,
            min_rating: query.min_rating,
            date_from: query.date_from,
            date_to: query.date_to,
            page: Some(query.page),
            limit: Some(query.limit),
            sort_by: query.sort_by,
            sort_order: query.sort_order,
        };
        
        let response = self.text_service.search_texts(request).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<GetTextStatsQuery, TextStatsResponse> for TextQueryHandler {
    async fn handle(&self, query: GetTextStatsQuery) -> Result<TextStatsResponse, AppError> {
        let response = self.text_service.get_text_stats(query.text_id).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<GetTextAnalyticsQuery, TextAnalyticsResponse> for TextQueryHandler {
    async fn handle(&self, query: GetTextAnalyticsQuery) -> Result<TextAnalyticsResponse, AppError> {
        let response = self.text_service.get_text_analytics().await?;
        Ok(response)
    }
}
