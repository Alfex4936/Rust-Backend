use crate::utils::parser::notice_parse;

use actix_web::{get, web, HttpResponse, Responder};

// #[get("/")]
// pub async fn hello() -> impl Responder {
//     let notice = Notice {
//         id: 12345,
//         title: "공지1".to_string(),
//         date: "2021-07-09".to_string(),
//         link: "https://".to_string(),
//         writer: "CSW".to_string(),
//     };

//     HttpResponse::Ok().json(notice)
// }

// #[get("/db")]
// pub async fn db_test(conn: web::Data<DbPool>) -> Result<HttpResponse, Error> {
//     let result = query::show_scheds(&conn.get().unwrap())
//         .await
//         .map(|sched| HttpResponse::Ok().json(sched))
//         .map_err(|_| HttpResponse::InternalServerError());

//     for row in query::show_scheds(&conn.get().unwrap()).await.unwrap() {
//         println!("id: {}, content: {}", row.id, row.content);
//     }

//     Ok(result.ok().unwrap())
// }
