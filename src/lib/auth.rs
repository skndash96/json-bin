use actix_web::{
    body::BoxBody, dev::{ServiceRequest, ServiceResponse}, web, Error, HttpMessage, HttpResponse
};
use actix_web_lab::middleware::Next;
use mongodb::{bson::doc, Client};

use super::schema::{User, UserID};

pub async fn token_auth(
    mongo_client: web::Data<Client>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let mongo_client = mongo_client.into_inner();

    let apikey = req.headers().get("Api-Key").map(|k| k.to_str().unwrap_or(""));

    let noauth_res = || Ok(
        ServiceResponse::new(
            req.request().clone(),
            HttpResponse::Unauthorized().body("Invalid Api-Key Found")
        ).map_into_boxed_body()
    );

    if apikey.is_none() {
        noauth_res()
    }
    
    else if let Ok(Some(user)) = mongo_client
        .database("app")
        .collection::<User>("users")
        .find_one(
            doc! { "key": apikey.unwrap() },
            None
        )
        .await
    {
        req.extensions_mut().insert::<UserID>(user._id);

        let res = next.call(req).await?;
        
        Ok(res.map_into_boxed_body())
    }

    else {
        noauth_res()
    }
}
