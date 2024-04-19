use actix_web::{ get, HttpMessage, HttpRequest, HttpResponse, Responder };

use crate::lib::schema::UserID;


/// Hello
/// Path: /api/hello
/// Description: Verifies Api-Key and sends a Hello message
/// Usage: curl -X GET http://127.0.0.1:8080/api/hello -H "Api-Key: your-api-key"
/// Returns: A body message that says Hello with your user  id.
// Throws: Nope

#[get("/hello")]
pub async fn hello(
    req: HttpRequest
) -> impl Responder {
    let extensions = req.extensions();

    let uid = extensions.get::<UserID>().unwrap();
    
    HttpResponse::Ok().body(format!(
        "Hello Client - {}", uid
    ))
}
