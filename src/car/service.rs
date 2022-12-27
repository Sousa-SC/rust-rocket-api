use generic_crud_proc_macro::CRUDServiceImpl;

#[derive(CRUDServiceImpl)]
#[module = "car"]
pub struct CarService;
