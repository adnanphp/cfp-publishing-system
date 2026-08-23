use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::responses::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDonationRequest {
    pub amount: f64,
    pub text_id: Uuid,
    pub charity_id: Uuid,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DonationResponse {
    pub id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharityResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub async fn create_donation(_req: web::Json<CreateDonationRequest>) -> impl Responder {
    let donation = DonationResponse {
        id: Uuid::new_v4(),
        amount: 100.0,
        currency: "USD".to_string(),
        status: "completed".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Created().json(ApiResponse::new(donation, "Donation created".to_string()))
}

pub async fn get_donation(_path: web::Path<Uuid>) -> impl Responder {
    let donation = DonationResponse {
        id: Uuid::new_v4(),
        amount: 100.0,
        currency: "USD".to_string(),
        status: "completed".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(donation, "Donation found".to_string()))
}

pub async fn list_donations() -> impl Responder {
    let donations = vec![
        DonationResponse {
            id: Uuid::new_v4(),
            amount: 50.0,
            currency: "USD".to_string(),
            status: "completed".to_string(),
            created_at: chrono::Utc::now(),
        },
        DonationResponse {
            id: Uuid::new_v4(),
            amount: 25.0,
            currency: "EUR".to_string(),
            status: "pending".to_string(),
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(donations, "Donations list".to_string()))
}

pub async fn get_charity_donations(_path: web::Path<Uuid>) -> impl Responder {
    let donations = vec![
        DonationResponse {
            id: Uuid::new_v4(),
            amount: 30.0,
            currency: "USD".to_string(),
            status: "completed".to_string(),
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(donations, "Charity donations".to_string()))
}

pub async fn get_text_donations(_path: web::Path<Uuid>) -> impl Responder {
    let donations = vec![
        DonationResponse {
            id: Uuid::new_v4(),
            amount: 20.0,
            currency: "USD".to_string(),
            status: "completed".to_string(),
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(donations, "Text donations".to_string()))
}

pub async fn get_user_donations(_path: web::Path<Uuid>) -> impl Responder {
    let donations = vec![
        DonationResponse {
            id: Uuid::new_v4(),
            amount: 15.0,
            currency: "USD".to_string(),
            status: "completed".to_string(),
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(donations, "User donations".to_string()))
}

pub async fn process_donation_payment() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"payment_id": "pay_123", "status": "succeeded"}),
        "Payment processed".to_string()
    ))
}

pub async fn get_donation_stats() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "total_donations": 100,
            "total_amount": 5000.0,
            "avg_donation": 50.0
        }),
        "Donation stats".to_string()
    ))
}

pub async fn list_charities() -> impl Responder {
    let charities = vec![
        CharityResponse {
            id: Uuid::new_v4(),
            name: "Charity One".to_string(),
            description: "First charity".to_string(),
        },
        CharityResponse {
            id: Uuid::new_v4(),
            name: "Charity Two".to_string(),
            description: "Second charity".to_string(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(charities, "Charities list".to_string()))
}

pub async fn get_charity_stats(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "total_donations": 50,
            "total_amount": 2500.0,
            "donors_count": 30
        }),
        "Charity stats".to_string()
    ))
}
