use generic_crud_proc_macro::CRUDControllerImpl;

#[derive(CRUDControllerImpl)]
#[module = "car"]
pub struct CarController;