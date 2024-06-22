use rocket::local::asynchronous::Client;
use rocket::uri;
use sibears_farm::config::get_config;
use sibears_farm::rocket_init::rocket;
use std::sync::Arc;

#[rocket::async_test]
async fn ping() {
    let config = Arc::new(get_config("./config_test.json"));
    let client = Client::tracked(rocket(config)).await.unwrap();
    let response = client
        .get(uri!(sibears_farm::rocket_init::hello))
        .dispatch()
        .await;
    assert_eq!(
        response.into_string().await.unwrap(),
        "Hello, SiBears Farm!"
    );
}
