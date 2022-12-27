use generic_crud_proc_macro::CRUDControllerImpl;

#[derive(CRUDControllerImpl)]
#[module = "animal"]
pub struct AnimalController;