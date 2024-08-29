use axum::Router;
use crate::auth::controller::AuthController;
use crate::utils::RouterTrait;

pub struct AuthRoute;

impl RouterTrait for AuthRoute {
    fn initialize(controller: AuthController) -> Router {
        let router = Router::new();

        router.route("/login", axum::routing::get(|| {
            controller.login()
        }))
    }
}
