#[derive(Debug, Clone)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Clone)]
pub struct MemberStats {
    pub total_members: i64,
    pub active_members: i64,
    pub new_members_today: i64,
}

#[derive(Debug, Clone)]
pub struct DownloadStats {
    pub total_downloads: i64,
    pub downloads_today: i64,
    pub unique_downloaders: i64,
}
