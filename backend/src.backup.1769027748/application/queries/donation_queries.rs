use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Query, QueryHandler};
use crate::{
    application::{
        services::donation_service::DonationService,
        dto::{
            requests::donation_request::{
                SearchDonationsRequest,
                DonationSummaryRequest,
            },
            responses::donation_response::{
                DonationResponse,
                DonationSearchResponse,
                DonationSummaryResponse,
            },
        },
    },
    utils::error::AppError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDonationQuery {
    pub donation_id: Uuid,
    pub include_distribution: bool,
}

impl Query for GetDonationQuery {
    fn query_name(&self) -> &'static str {
        "GetDonationQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDonationsQuery {
    pub member_id: Option<String>,
    pub text_id: Option<String>,
    pub charity_id: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub page: u32,
    pub limit: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Query for SearchDonationsQuery {
    fn query_name(&self) -> &'static str {
        "SearchDonationsQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDonationSummaryQuery {
    pub period: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub group_by: Option<String>,
    pub include_breakdown: bool,
}

impl Query for GetDonationSummaryQuery {
    fn query_name(&self) -> &'static str {
        "GetDonationSummaryQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMemberDonationsQuery {
    pub member_id: Uuid,
    pub include_text_info: bool,
    pub page: u32,
    pub limit: u32,
}

impl Query for GetMemberDonationsQuery {
    fn query_name(&self) -> &'static str {
        "GetMemberDonationsQuery"
    }
}

pub struct DonationQueryHandler {
    donation_service: Box<dyn DonationService>,
}

impl DonationQueryHandler {
    pub fn new(donation_service: Box<dyn DonationService>) -> Self {
        Self { donation_service }
    }
}

#[async_trait]
impl QueryHandler<GetDonationQuery, DonationResponse> for DonationQueryHandler {
    async fn handle(&self, query: GetDonationQuery) -> Result<DonationResponse, AppError> {
        let response = self.donation_service.get_donation(query.donation_id).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<SearchDonationsQuery, DonationSearchResponse> for DonationQueryHandler {
    async fn handle(&self, query: SearchDonationsQuery) -> Result<DonationSearchResponse, AppError> {
        let request = SearchDonationsRequest {
            member_id: query.member_id,
            text_id: query.text_id,
            charity_id: query.charity_id,
            status: query.status,
            date_from: query.date_from,
            date_to: query.date_to,
            min_amount: query.min_amount,
            max_amount: query.max_amount,
            page: Some(query.page),
            limit: Some(query.limit),
            sort_by: query.sort_by,
            sort_order: query.sort_order,
        };
        
        let response = self.donation_service.search_donations(request).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<GetDonationSummaryQuery, DonationSummaryResponse> for DonationQueryHandler {
    async fn handle(&self, query: GetDonationSummaryQuery) -> Result<DonationSummaryResponse, AppError> {
        let request = DonationSummaryRequest {
            period: query.period,
            start_date: query.start_date,
            end_date: query.end_date,
            group_by: query.group_by,
        };
        
        let response = self.donation_service.get_donation_summary(request).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<GetMemberDonationsQuery, Vec<DonationResponse>> for DonationQueryHandler {
    async fn handle(&self, query: GetMemberDonationsQuery) -> Result<Vec<DonationResponse>, AppError> {
        let request = SearchDonationsRequest {
            member_id: Some(query.member_id.to_string()),
            text_id: None,
            charity_id: None,
            status: None,
            date_from: None,
            date_to: None,
            min_amount: None,
            max_amount: None,
            page: Some(query.page),
            limit: Some(query.limit),
            sort_by: Some("date".to_string()),
            sort_order: Some("desc".to_string()),
        };
        
        let response = self.donation_service.search_donations(request).await?;
        Ok(response.donations)
    }
}
