use actix_web::{ post, web, HttpResponse, Responder };
use uuid::Uuid;
use crate::error::CustomError;
use crate::lib::schema::{User, UserCredential};
use mongodb::Client;
use mongodb::bson::doc;

/// New Key
/// Path: /new-key
/// Description: Generates a new Api-Key for a user for the given _id and passsword.
/// Usage: curl -X POST http://127.0.0.1:8080/new-key -d "_id=testuser&pwd=testuserpassword"
/// Returns: A body message containing the New Api-Key
/// Throws: User Not Found, Mongo WriteConcern Error

#[post("/new-key")]
pub async fn new_key(mongo_client: web::Data<Client>, form: web::Form<UserCredential>) -> impl Responder {
    let apikey = Uuid::new_v4().to_string();

    let cred = form.into_inner();
    
    if let Err(e) = mongo_client
        .database("app")
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "_id": cred._id, "pwd": cred.pwd },
            doc! { "key": apikey.clone() },
            None
        ).await
    {
        println!("{:?}", e);
        return CustomError::UserNotMatched.into();
    } else {
        return HttpResponse::Ok().json(
            format!(
                "Please make note of the APIKEY for usage in subsequent requests made: {}",
                apikey
            )
        );
    }
}
