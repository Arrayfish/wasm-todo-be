// Github Copilotから出てきたものをコピペしただけなので、動作するかもわからない
use actix_identity::{Identity, IdentityExt};
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http,
    web::Redirect,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    fmt::Debug,
    future::{ready, Ready},
};

pub struct CheckLogin {
    exclude_path_list: Vec<String>,
}

impl CheckLogin {
    // この関数は、exclude_path_listに含まれるパスにはログインチェックを行わない
    pub fn new(exclude_path_list: Vec<String>) -> Self {
        CheckLogin { exclude_path_list }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware {
            service,
            exclude_path_list: self.exclude_path_list.clone(),
        }))
    }
}

pub struct CheckLoginMiddleware<S> {
    service: S,
    exclude_path_list: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 適用するパスかどうかの判定(前方一致)
        let mut is_apply = true;
        for path in &self.exclude_path_list {
            if req.path().starts_with(path) {
                is_apply = false;
                break;
            }
        }
        let mut is_logged_in = false;
        if let Ok(user) = &req.get_identity() {
            println!("User: {:?}", user.id().unwrap());
            is_logged_in = true;
        }
        if is_apply && !is_logged_in {
            // ログインしていない場合は、ログイン画面にリダイレクト
            let (request, _pl) = req.into_parts();

            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/login"))
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }
        let res = self.service.call(req);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
