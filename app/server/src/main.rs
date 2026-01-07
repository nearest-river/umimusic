
use tokio::net::TcpListener;

use axum::{
  Router,
  routing::get,
};




#[tokio::main]
async fn main()-> anyhow::Result<()> {
  let app=Router::new()
  .route("/",get(home));




  let listener=TcpListener::bind("0.0.0.0:3000").await?;
  axum::serve(listener,app)
  .await?;


  Ok(())
}


async fn home()-> String {
  "hello world".to_owned()
}






