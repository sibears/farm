use rocket::local::asynchronous::Client;
use rocket::uri;
use sibears_farm::rocket_init::rocket;

#[rocket::async_test]
async fn ping() {
    let client = Client::tracked(rocket()).await.unwrap();
    let response = client.get(uri!(sibears_farm::rocket_init::hello)).dispatch().await;
    assert_eq!(response.into_string().await.unwrap(), "Hello, SiBears Farm!");
}