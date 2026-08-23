use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Query, QueryHandler};
use crate::{
    application::{
        services::member_service::MemberService,
        dto::{
            requests::member_request::SearchMembersRequest,
            responses::member_response::{
                MemberResponse,
                AuthorResponse,
                AdminResponse,
                ModeratorResponse,
                MemberSearchResponse,
                MemberStatsResponse,
            },
        },
    },
    utils::error::AppError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMemberQuery {
    pub member_id: Uuid,
    pub include_related: bool,
}

impl Query for GetMemberQuery {
    fn query_name(&self) -> &'static str {
        "GetMemberQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthorQuery {
    pub member_id: Uuid,
    pub include_stats: bool,
}

impl Query for GetAuthorQuery {
    fn query_name(&self) -> &'static str {
        "GetAuthorQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthorByOrcidQuery {
    pub orcid: String,
    pub include_stats: bool,
}

impl Query for GetAuthorByOrcidQuery {
    fn query_name(&self) -> &'static str {
        "GetAuthorByOrcidQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMembersQuery {
    pub query: Option<String>,
    pub status: Option<String>,
    pub role: Option<String>,
    pub page: u32,
    pub limit: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Query for SearchMembersQuery {
    fn query_name(&self) -> &'static str {
        "SearchMembersQuery"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMemberStatsQuery;

impl Query for GetMemberStatsQuery {
    fn query_name(&self) -> &'static str {
        "GetMemberStatsQuery"
    }
}

pub struct MemberQueryHandler {
    member_service: Box<dyn MemberService>,
}

impl MemberQueryHandler {
    pub fn new(member_service: Box<dyn MemberService>) -> Self {
        Self { member_service }
    }
}

#[async_trait]
impl QueryHandler<GetMemberQuery, MemberResponse> for MemberQueryHandler {
    async fn handle(&self, query: GetMemberQuery) -> Result<MemberResponse, AppError> {
        let response = self.member_service.get_member(query.member_id).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<GetAuthorQuery, AuthorResponse> for MemberQueryHandler {
    async fn handle(&self, query: GetAuthorQuery) -> Result<AuthorResponse, AppError> {
        let response = self.member_service.get_author(query.member_id).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<GetAuthorByOrcidQuery, AuthorResponse> for MemberQueryHandler {
    async fn handle(&self, query: GetAuthorByOrcidQuery) -> Result<AuthorResponse, AppError> {
        let response = self.member_service.get_author_by_orcid(&query.orcid).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<SearchMembersQuery, MemberSearchResponse> for MemberQueryHandler {
    async fn handle(&self, query: SearchMembersQuery) -> Result<MemberSearchResponse, AppError> {
        let request = SearchMembersRequest {
            query: query.query,
            status: query.status,
            role: query.role,
            page: Some(query.page),
            limit: Some(query.limit),
            sort_by: query.sort_by,
            sort_order: query.sort_order,
        };
        
        let response = self.member_service.search_members(request).await?;
        Ok(response)
    }
}

#[async_trait]
impl QueryHandler<GetMemberStatsQuery, MemberStatsResponse> for MemberQueryHandler {
    async fn handle(&self, _query: GetMemberStatsQuery) -> Result<MemberStatsResponse, AppError> {
        let response = self.member_service.get_member_stats().await?;
        Ok(response)
    }
}
