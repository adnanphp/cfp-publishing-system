//use actix_web::web;
//use crate::api::handlers::downloads_handler;

//pub fn download_routes(cfg: &mut web::ServiceConfig) {
  //  cfg.service(
     //   web::scope("/downloads")
      //      .route("", web::get().to(downloads_handler::get_downloads))
      //      .route("/count", web::get().to(downloads_handler::get_download_count))
      //      .route("/stats", web::get().to(downloads_handler::get_download_stats))
    //        .route("/by-text/{text_id}", web::get().to(downloads_handler::get_downloads_by_text))
   //         .route("/top-texts", web::get().to(downloads_handler::get_top_downloaded_texts))
   //         .route("/by-member/{member_id}", web::get().to(downloads_handler::get_downloads_by_member))
 //   );
//}
