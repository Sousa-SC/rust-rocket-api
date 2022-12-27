use generic_crud_proc_macro::CRUDServiceImpl;

#[derive(CRUDServiceImpl)]
#[module = "animal"]
pub struct AnimalService;