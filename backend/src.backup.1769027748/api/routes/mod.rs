pub mod text_routes;
pub mod donation_routes;
pub mod plagiarism_routes;
pub mod committee_routes;
pub mod notification_routes;
pub mod websocket_routes;

use actix_web::web;
use crate::api::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Text routes
            .service(
                web::scope("/texts")
                    .route("", web::get().to(handlers::text_handler::get_texts))
                    .route("/{id}", web::get().to(handlers::text_handler::get_text))
                    .route("", web::post().to(handlers::text_handler::create_text))
                    .route("/{id}", web::put().to(handlers::text_handler::update_text))
                    .route("/{id}", web::delete().to(handlers::text_handler::delete_text))
                    .route("/{id}/download", web::post().to(handlers::text_handler::download_text))
                    .route("/{id}/comments", web::get().to(handlers::text_handler::get_text_comments))
                    .route("/{id}/versions", web::get().to(handlers::text_handler::get_text_versions))
            )
            // Donation routes
            .service(
                web::scope("/donations")
                    .route("", web::get().to(handlers::donation_handler::get_donations))
                    .route("/{id}", web::get().to(handlers::donation_handler::get_donation))
                    .route("", web::post().to(handlers::donation_handler::create_donation))
                    .route("/{id}/confirm", web::post().to(handlers::donation_handler::confirm_donation))
                    .route("/{id}/refund", web::post().to(handlers::donation_handler::refund_donation))
                    .route("/stats", web::get().to(handlers::donation_handler::get_donation_stats))
                    .route("/charities", web::get().to(handlers::donation_handler::get_charities))
                    .route("/charities/{id}", web::get().to(handlers::donation_handler::get_charity))
            )
            // Plagiarism routes
            .service(
                web::scope("/plagiarism")
                    .route("/cases", web::get().to(handlers::plagiarism_handler::get_cases))
                    .route("/cases/{id}", web::get().to(handlers::plagiarism_handler::get_case))
                    .route("/cases", web::post().to(handlers::plagiarism_handler::create_case))
                    .route("/cases/{id}/vote", web::post().to(handlers::plagiarism_handler::vote_on_case))
                    .route("/cases/{id}/appeal", web::post().to(handlers::plagiarism_handler::appeal_case))
                    .route("/cases/{id}/close", web::post().to(handlers::plagiarism_handler::close_case))
                    .route("/check", web::post().to(handlers::plagiarism_handler::check_plagiarism))
                    .route("/stats", web::get().to(handlers::plagiarism_handler::get_plagiarism_stats))
            )
            // Committee routes
            .service(
                web::scope("/committees")
                    .route("", web::get().to(handlers::committee_handler::get_committees))
                    .route("/{id}", web::get().to(handlers::committee_handler::get_committee))
                    .route("", web::post().to(handlers::committee_handler::create_committee))
                    .route("/{id}/members", web::get().to(handlers::committee_handler::get_committee_members))
                    .route("/{id}/members", web::post().to(handlers::committee_handler::add_committee_member))
                    .route("/{id}/members/{member_id}", web::delete().to(handlers::committee_handler::remove_committee_member))
                    .route("/{id}/meetings", web::get().to(handlers::committee_handler::get_committee_meetings))
                    .route("/{id}/meetings", web::post().to(handlers::committee_handler::schedule_meeting))
                    .route("/{id}/decisions", web::get().to(handlers::committee_handler::get_committee_decisions))
            )
            // Notification routes
            .service(
                web::scope("/notifications")
                    .route("", web::get().to(handlers::notification_handler::get_notifications))
                    .route("/unread", web::get().to(handlers::notification_handler::get_unread_notifications))
                    .route("/{id}/read", web::put().to(handlers::notification_handler::mark_as_read))
                    .route("/{id}", web::delete().to(handlers::notification_handler::delete_notification))
                    .route("/preferences", web::get().to(handlers::notification_handler::get_notification_preferences))
                    .route("/preferences", web::put().to(handlers::notification_handler::update_notification_preferences))
                    .route("/broadcast", web::post().to(handlers::notification_handler::broadcast_notification))
            )
            // WebSocket/SSE routes
            .service(
                web::scope("/ws")
                    .route("/connect", web::get().to(websocket_routes::websocket_handler))
                    .route("/sse", web::get().to(websocket_routes::sse_handler))
            )
            // Member routes (assuming they exist in member_handler)
            .service(
                web::scope("/members")
                    .route("/profile", web::get().to(handlers::member_handler::get_profile))
                    .route("/profile", web::put().to(handlers::member_handler::update_profile))
                    .route("/messages", web::get().to(handlers::member_handler::get_messages))
                    .route("/messages/{id}", web::get().to(handlers::member_handler::get_message))
                    .route("/messages", web::post().to(handlers::member_handler::send_message))
                    .route("/downloads", web::get().to(handlers::member_handler::get_downloads))
                    .route("/donations", web::get().to(handlers::member_handler::get_member_donations))
                    .route("/comments", web::get().to(handlers::member_handler::get_member_comments))
            )
    );
}

// Route-specific configuration functions
pub fn text_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/texts/search")
            .route(web::get().to(text_routes::search_texts))
    );
}

pub fn donation_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/donations/verify")
            .route(web::post().to(donation_routes::verify_payment))
    );
}

pub fn plagiarism_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/plagiarism/bulk-check")
            .route(web::post().to(plagiarism_routes::bulk_check_plagiarism))
    );
}

pub fn committee_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/committees/{id}/assign-case")
            .route(web::post().to(committee_routes::assign_case_to_committee))
    );
}

pub fn notification_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/notifications/test")
            .route(web::post().to(notification_routes::test_notification))
    );
}

pub fn websocket_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/ws/status")
            .route(web::get().to(websocket_routes::get_connection_status))
    );
}
