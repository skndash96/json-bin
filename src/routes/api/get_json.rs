use actix_web::{ get, web, HttpMessage, HttpRequest, HttpResponse, Responder };
use crate::error::CustomError;
use crate::lib::schema::{Bin, UserID};
use mongodb::Client;
use mongodb::bson::doc;
use serde_json::Value;

#[derive(serde::Deserialize)]
struct GetJSONOptions {
    pub _id: String,
}

/// Get JSON
/// Path: /api/get-json
/// Description: Retrieve previously added JSON (Anyone with the ID can read)
/// Usage:  curl -X GET http://127.0.0.1:8080/api/get-json -d "_id: your-json-id" -H "Api-Key: your-api-key"
///         [or]
///         curl -X GET http://127.0.0.1:8080/api/get-json?_id=your-json-id -H "Api-Key: your-api-key"
/// Returns: JSON body { "json": {your JSON content as Object}, owner: "your id" }
/// Throws: JSON Not Found

#[get("/get-json")]
async fn get_json(
    mongo_client: web::Data<Client>,
    params: web::Query<GetJSONOptions>,
    req: HttpRequest
) -> impl Responder {
    let params = params.into_inner();

    let extensions = req.extensions();
    let uid = extensions.get::<UserID>().unwrap();
    
    if let Ok(Some(data)) = mongo_client
        .database("app")
        .collection::<Value>("bin")
        .find_one(
            doc! { "_id": params._id },
            None
        )
        .await
    {
        let bin = Bin::new(data, uid.clone());
        return HttpResponse::Ok().json(bin);
    } else {
        return CustomError::NotFoundJSON.into();
    }
}