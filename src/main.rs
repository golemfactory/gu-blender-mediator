use actix::prelude::*;
use actix_web::{
    dev::JsonBody, dev::Path, http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use futures::prelude::*;
use structopt::StructOpt;

use gateway::Gateway;
use gu_actix::flatten::FlattenFuture;
use log::Metadata;
use serde_derive::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

mod args;

mod blender;
mod dav;
mod error;
mod gateway;
mod joinact;
mod subtask_worker;
mod task_worker;
mod workman;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SessionConfig {
    // "account":"0xb2bbb75241939e50b5ba6f698415bbb5ca54610d","davUrl":"http://127.0.0.1:55011","docker":true,"gwUrl":"http://127.0.0.1:55001/"
    account: String,
    dav_url: String,
    gw_url: String,
    docker: bool,
    #[serde(default)]
    subscription_id: String,
}

fn main() {
    env_logger::init();
    let args = args::Args::from_args();

    let local = args.local;

    if !local {
        Arbiter::spawn_fn(|| {
            eprintln!("Starting registration");
            gu_plugin_api::register_service("http://127.0.0.1:33433/", "gu-blender-mediator")
        })
    } else {
        eprintln!("registration skipped");
    }

    let gateways: Arc<RwLock<HashMap<Option<u64>, _>>> = Arc::new(RwLock::new(HashMap::new()));

    if !args.gw_addr.is_empty() && !args.dav_addr.is_empty() {
        let gw = Gateway::new(None, args.dav_addr, args.gw_addr).start();

        gateways.write().unwrap().insert(None, gw);
    }

    //let client = ::hyper::client::Client::
    //golem_gw_api::apis::configuration::Configuration::new()

    HttpServer::new(move || {
        let gateways_to_add = gateways.clone();
        let gateways_to_get = gateways.clone();
        let gateways_to_get2 = gateways.clone();

        App::new()
            .service(
                web::resource("/gw")
                    .route(web::post().to_async(move |b: web::Json<u64>| {
                        let gateways = gateways_to_add.clone();
                        use gu_client::r#async::*;
                        let gu_api = HubConnection::default();
                        let session_id = b.into_inner();

                        let session = gu_api.hub_session(session_id);
                        session
                            .config()
                            .map_err(|e| actix_web::error::ErrorInternalServerError(e))
                            .and_then(move |m| {
                                fn extract_config(
                                    m: gu_client::model::session::Metadata,
                                ) -> Result<SessionConfig, serde_json::Error>
                                {
                                    serde_json::from_value(serde_json::to_value(m.entry)?)
                                }

                                let config: SessionConfig = match extract_config(m) {
                                    Ok(c) => c,
                                    Err(e) => {
                                        return Err(actix_web::error::ErrorInternalServerError(e));
                                    }
                                };

                                let gw =
                                    Gateway::new(Some(session_id), config.dav_url, config.gw_url)
                                        .start();
                                gateways.write().unwrap().insert(Some(session_id), gw);
                                Ok(HttpResponse::Ok().json("ok"))
                            })
                    }))
                    .route(web::get().to(move |_: ()| {
                        let sessions: Vec<Option<u64>> =
                            gateways_to_get.read().unwrap().keys().cloned().collect();

                        web::Json(sessions)
                    })),
            )
            .service(web::resource("/gw/{session_id}").route(web::get().to_async(
                move |p: web::Path<(u64,)>| {
                    let gw = {
                        gateways_to_get2
                            .read()
                            .unwrap()
                            .get(&Some(p.0))
                            .ok_or(error::other("missing id"))
                            .map(|v| v.clone())
                    };
                    let request = {
                        gw.into_future()
                            .and_then(|a| a.send(gateway::Stats).flatten_fut())
                    };

                    request
                        .map_err(|e| actix_web::error::ErrorInternalServerError(e))
                        .and_then(|stats| Ok(HttpResponse::Ok().json(stats)))
                },
            )))
    })
    .bind("127.0.0.1:33433")
    .unwrap()
    .run()
    .unwrap()
}
