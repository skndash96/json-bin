use actix_web::{ post, web, HttpMessage, HttpRequest, HttpResponse, Responder };
use crate::{error::CustomError, lib::schema::{Bin, UserID, Value}};
use mongodb::{bson::to_document, Client};

/// Add JSON
/// Path: /api/add-json
/// Description: Adds new JSON 
/// Usage: curl -X POST http://127.0.0.1:8080/api/add-json -d " your stringified JSON " -H "Api-Key: your-api-key"
/// Returns: A body message that says Hello with your user  id.
/// Throwws: JSON Invalid Error

#[post("/add-json")]
async fn add_json(
    mongo_client: web::Data<Client>,
    json: web::Json<Value>,
    req: HttpRequest
) -> impl Responder {
    
    let extensions = req.extensions();
    let uid = extensions.get::<UserID>().unwrap().to_owned();
    
    let bin = Bin::new(json.into_inner(), uid);
    let doc = to_document(&bin);

    if doc.is_err() {
        return CustomError::InvalidJSON.into();
    }
    let doc = doc.unwrap();
    
    if let Ok(res) = mongo_client
        .database("app")
        .collection("bin")
        .insert_one(
            doc,
            None
        )
        .await
    {
        return HttpResponse::Ok().body(format!("Added JSON with _id {}", res.inserted_id));
    } else {
        return CustomError::JSONAddFail.into();
    }
}
