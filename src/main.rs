use actix_web::{dev::Service, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use mongodb::Client;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};

pub mod routes {
    pub mod add_user;
    pub mod get_key;
    pub mod new_key;
    pub mod api {
        pub mod add_json;
        pub mod get_json;
        pub mod hello;
    }
}
pub mod lib {
    pub mod schema;
    pub mod auth;
}

pub mod error;

use routes::{
    add_user,
    api::{add_json, get_json, hello},
    get_key, new_key,
};

use lib::auth::token_auth;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "127.0.0.1";
    let port = 8080;

    println!("Establishing Mongo Connection");

    let mongo_client = mongo().await?;

    println!("Mongo Connection Established");

    println!("Listening at {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                println!("{} at {}", req.method(), req.path());

                srv.call(req)
            })
            .app_data(web::Data::new(mongo_client.clone()))
            .service(add_user::add_user)
            .service(new_key::new_key)
            .service(get_key::get_key)
            .service(
                web::scope("/api")
                    .wrap(from_fn(token_auth))
                    .service(hello::hello)
                    .service(get_json::get_json)
                    .service(add_json::add_json),
            )
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}

async fn mongo() -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(
        format!(
            "mongodb+srv://{}:{}@cluster0.mogohzh.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
            "skndash96",
            "admin"
        )
    ).await?;

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(client)
}

/*
Flatten an Object
{
    a: {
        b: 1,
        c: {d:2}
    },
    e: 3,
    f: [4,5,6]
}
TO
{
    a.b  : 1
    a.c.d: 2,
    e    : 3,
    f.0  : 4,
    f.1  : 5,
    f.6  : 6
}

let res = []; //Vec<&str, String|Number>

//Use this in FRONTEND JS
function flatten_obj(obj, ok="") {
  for (let [k,v] of Object.entries(obj)) {
    if (typeof(v) === "object") {
      flatten_obj(v, ok+k+".");
    } else {
      res.push([ok+k, v]);
    }
  }
}
*/
