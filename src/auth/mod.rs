use axum::Router;
use route::AuthRoute;
use crate::auth::controller::AuthController;
use crate::auth::service::AuthService;
use crate::utils::{ControllerTrait, Module, ModuleTrait, RouterTrait, ServiceTrait};

pub mod service;
pub mod route;
pub mod controller;


pub struct Auth;

impl ModuleTrait for Auth {
    fn initialize() -> Module {
        let service = AuthService::initialize();
        let controller = AuthController::initialize(service);
        let router = AuthRoute::initialize(controller);

        Module {
            name: "auth".to_string(),
            router: Router::new().nest("/auth", router),
        }
    }
}


