use crate::auth::service::AuthService;
use crate::utils::ControllerTrait;

pub struct AuthController;


impl AuthController {
    pub fn login(&self) {
        todo!()
    }
}

impl ControllerTrait for AuthController {
    fn initialize<AuthService>(service: AuthService) -> Self {
        todo!()
    }
}
