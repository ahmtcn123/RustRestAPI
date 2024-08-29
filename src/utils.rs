use axum::Router;

pub trait RouterTrait {
    fn initialize<T>(
        controller: T
    ) -> Router;
}

pub trait ControllerTrait {
    fn initialize<T: ServiceTrait>(
        service: T
    ) -> Self;
}

pub trait ServiceTrait {
    fn initialize() -> Self;
}

pub struct Module {
    pub name: String,
    pub router: Router,
}


pub trait ModuleTrait {
    fn initialize() -> Module;
}