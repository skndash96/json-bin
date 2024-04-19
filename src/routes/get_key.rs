use actix_web::{ web, get, Responder, HttpResponse };
use mongodb::{ bson::doc, Client };
use crate::{error::CustomError, lib::schema::{User, UserCredential}};

/// Get Key
/// Path: /get-key
/// Description: Fetches the Api-Key of the user for the given _id and password.
/// Usage: curl -X GET http://127.0.0.1:8080/get-key -d "_id=test&pwd=testpwd"
/// Returns: A body message containing only the Api-Key
/// Throws: User not found

#[get("/get-key")]
pub async fn get_key(mongo_client: web::Data<Client>, form: web::Form<UserCredential>) -> impl Responder {
    let cred = form.into_inner();
    
    if let Ok(Some(u)) = mongo_client
        .database("app")
        .collection::<User>("users")
        .find_one(doc! { "_id": cred._id, "pwd": cred.pwd }, None)
        .await
    {
        return HttpResponse::Ok().body(u.key);
    } else {
        return CustomError::UserNotMatched.into();
    }
}