use casbin::{CoreApi, DefaultModel, Enforcer, FileAdapter, RbacApi};
use std::io;
use tokio::sync::RwLock;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

/// simple handle
async fn success(enforcer: web::Data<RwLock<Enforcer>>, req: HttpRequest) -> HttpResponse {
    let mut e = enforcer.write().await;
    println!("{req:?}");
    assert_eq!(vec!["data2_admin"], e.get_roles_for_user("alice", None));

    HttpResponse::Ok().body("Success: alice is data2_admin.")
}

async fn fail(enforcer: web::Data<RwLock<Enforcer>>, req: HttpRequest) -> HttpResponse {
    let mut e = enforcer.write().await;
    println!("{req:?}");
    assert_eq!(vec!["data1_admin"], e.get_roles_for_user("alice", None));

    HttpResponse::Ok().body("Fail: alice is not data1_admin.") // In fact, it can't be displayed.
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("LOGE_FORMAT", "target");

    loge::init();

    let model = DefaultModel::from_file("rbac/rbac_model.conf")
        .await
        .unwrap();
    let adapter = FileAdapter::new("rbac/rbac_policy.csv");

    let e = Enforcer::new(model, adapter).await.unwrap();
    let e = web::Data::new(RwLock::new(e)); // wrap enforcer into actix-state

    //move is necessary to give closure below ownership of counter
    HttpServer::new(move || {
        App::new()
            .app_data(e.clone()) // <- create app with shared state
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
            .service(web::resource("/success").to(success))
            .service(web::resource("/fail").to(fail))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
