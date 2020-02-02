use actix_web::{HttpResponse};
use crate::model::User;

pub async fn get_users() -> HttpResponse {
    let users = vec![ 
                User { id : 1, name: String::from("Test user 1")}
            ,   User { id : 2, name: String::from("Test user 2")}
            ,   User { id : 3, name: String::from("Test user 3")}
            ,   User { id : 4, name: String::from("Test user 4")}
        ];
    HttpResponse::Ok().json(users)
}
