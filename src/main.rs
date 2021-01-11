use rust_embed::RustEmbed;
use rand::seq::IteratorRandom;
use warp::{http::header::HeaderValue,Filter, http::Response, Rejection, Reply};

#[derive(RustEmbed)]
#[folder = "src/images/"]
struct Asset;

#[tokio::main]
async fn main() {
    let random = warp::path("image").and_then(serve_random_image);
    
    println!("Running at 3030.");
    warp::serve(random)
        .run(([127, 0, 0, 1], 3030))    
        .await;
}

async fn serve_random_image() -> Result<impl Reply, Rejection> {
    let mut rng = rand::thread_rng();
    let random_file = Asset::iter().choose(&mut rng).unwrap();

    serve_image(random_file.as_ref())
}

fn serve_image(path: &str) -> Result<impl Reply, Rejection> {
    let image = Asset::get(path).ok_or_else(warp::reject::not_found)?;
    let b64 = base64::encode(image.as_ref());
    let imgsize = imagesize::blob_size(image.as_ref());
    let (width, height) = match imgsize {
      Ok(s) => (s.width, s.height),
      Err(_) => (400, 400)
    };
    let res = Response::builder()
    .header("content-type", HeaderValue::from_str("image/svg+xml").unwrap())
    .body(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{}\" height=\"{}\" viewBox=\"0 0\"><image xlink:href=\"data:image/jpeg;base64,{}\" /></svg>", width, height, b64));
    Ok(res)
}