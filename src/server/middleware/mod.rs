use http::{Request, Response, StatusCode};
use pin_project_lite::pin_project;
use std::{env, future::Future, pin::Pin, task::{Context, Poll}};
use cookie::Cookie;
use dotenvy::dotenv;
use http::header::COOKIE;
use tower_layer::Layer;
use tower_service::Service;

#[derive(Debug, Clone, Copy)]
pub struct AuthLayer {}

impl AuthLayer {
    pub fn new() -> Self {
        AuthLayer {}
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = Auth<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Auth::new(inner)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Auth<S> {
    inner: S,
}

impl<S> Auth<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }

    pub fn layer() -> AuthLayer {
        AuthLayer::new()
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for Auth<S>
where
    S: Service<Request<ReqBody>, Response=Response<ResBody>>,
    ResBody: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let cookies = req.headers()
            .get(COOKIE)
            .and_then(|header| header.to_str().ok())
            .map(Cookie::split_parse_encoded);

        let token = cookies.and_then(
            |cookies| cookies
                .filter_map(|cookie| cookie.ok())
                .find(|cookie| cookie.name() == "auth_token")
                .map(|cookie| cookie.value().to_string())
        );

        ResponseFuture {
            inner: self.inner.call(req),
            token,
        }
    }
}

pin_project! {
    pub struct ResponseFuture<F> {
        #[pin]
        inner: F,
        #[pin]
        token: Option<String>,
    }
}

impl<F, B, E> Future for ResponseFuture<F>
where
    F: Future<Output=Result<Response<B>, E>>,
    B: Default,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        dotenv().expect("Could not load environment variables from the .env file.");

        let this = self.project();

        if !(*this.token).as_ref().is_some_and(
            |token| token == env::var("AUTH_TOKEN")
                .expect("The environment variable DATABASE_URL has to be set.").as_str()
        ) {
            let mut res = Response::new(B::default());
            *res.status_mut() = StatusCode::UNAUTHORIZED;
            return Poll::Ready(Ok(res));
        }

        this.inner.poll(cx)
    }
}
