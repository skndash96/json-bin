use actix_web::{ post, web, Responder, HttpResponse };
use uuid::Uuid;
use crate::{error::CustomError, lib::schema::{User, UserCredential}};
use mongodb::Client;

/// Add User
/// Path: /add-user
/// Description: Creates a new user with given unique _id and password if the user not already exists.
/// Usage: curl -X POST http://127.0.0.1:8080/add-user -d "_id=test&pwd=testpwd"
/// Returns: A body message containing Api-Key
/// Throws: User Already Exists, Mongo WriteConcern Error

#[post("/add-user")]
pub async fn add_user(mongo_client: web::Data<Client>, form: web::Form<UserCredential>) -> impl Responder {
    let apikey = Uuid::new_v4().to_string();

    let cred = form.into_inner();
    let u = User {
        _id: cred._id,
        pwd: cred.pwd,
        key: apikey
    };

    if let Err(e) = mongo_client
        .database("app")
        .collection::<User>("users")
        .insert_one(&u, None) //TODO: salt & hash password
        .await
    {
        println!("{}", e);
        return CustomError::UserExists.into();
    } else {
        return HttpResponse::Ok().json(
            format!(
                "User created. Please make note of the APIKEY for usage in subsequent requests made: {}",
                u.key
            )
        );
    }
}
