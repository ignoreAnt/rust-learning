use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

/// Starts an HTTP server using Actix-web framework.
///
/// The server listens on localhost at port 3000 and has a single route "/"
/// which is handled by the `get_index` function. The server will print a 
/// message indicating that it has started and is ready to accept requests.
///
/// # Returns
///
/// A `Result` which is `Ok` if the server runs successfully, or an error
/// if the server encounters any issues during startup or while running.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Starting server at http://localhost:3000..");
    server
        .bind("127.0.0.1:3000")?
        .run()
        .await;
    Ok(())
}

/// Handles GET requests to the root URL ("/") and returns an HTML form
/// prompting the user for two numbers.  The form will POST to the "/gcd"
/// URL.
async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"<title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,)
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

/// Handles POST requests to the "/gcd" URL and returns an HTML page
/// containing the greatest common divisor of the two numbers provided in
/// the form data.
///
/// If either of the two input numbers is zero, the function responds with
/// a 400 Bad Request error, indicating that the request was invalid.
///
/// The function returns a `Result` which is `Ok` if the request was
/// successful, or an error if any issues were encountered while handling
/// the request.
async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}


/// Computes the greatest common divisor (GCD) of two non-zero unsigned
/// integers using Euclid's algorithm.
///
/// # Arguments
///
/// * `n` - An unsigned integer input (must be non-zero).
/// * `m` - Another unsigned integer input (must be non-zero).
///
/// # Returns
///
/// The GCD of `n` and `m`.
///
/// # Panics
///
/// Panics if either `n` or `m` is zero.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    n
}
