// use tokio;

// #[tokio::main]
// async fn main(){
//     println!("Aync task starting");
//     let task1 = tokio::spawn(async {
//         tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
//         println!("task 1 complted");
//     });

//     let task2 = tokio::spawn(async {
//         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//         println!("task 2 complted");
//     });

//     task1.await.unwrap();
//     task2.await.unwrap();

//     println!("all tasks completed!")
    

// }

// Practical Example: Building a Simple Web Server with Concurrency
use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{body::Incoming as RequestBody, Request, Response};
use hyper_util::rt::TokioIo; // TokioIo is translator means translate between friends Ali speck urdu Hassan speck french
use std::future::Future;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

type Counter = i32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127,0,0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port on http://{}", addr);

    let svc = Svc {
        counter: Arc::new(Mutex::new(0))
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let svc_clone = svc.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, svc_clone).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    };
}

#[derive(Debug, Clone)]
struct Svc {
    counter: Arc<Mutex<Counter>>,
}

impl<IncomingBody> Service<Request<IncomingBody>> for Svc {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send >>;
    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        if req.uri().path() != "/favicon.ico" {
            *self.counter.lock().expect("lock poisoned") += 1
        }
        let path = req.uri().path().to_string();
        let counter = self.counter.clone();

        Box::pin(async move {
            match path.as_str() {
                "/" => {
                    mk_response(format!("Welcome to the Rust Web Server! counter = {:?}", counter))
                }
                "/slow" => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    mk_response(format!("This took the while, but we're still concurrent! counter = {:?}", counter))
                }
                "/fast" => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    mk_response(format!("This took the long time, but we're still concurrent! counter = {:?}", counter))
                }
                _ => mk_response("oh no! not found".into()),
            }
        })

    }


}