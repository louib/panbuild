pub struct Database {
    pub projects: Vec<crate::projects::project::Project>,
    pub modules: Vec<crate::modules::module::SoftwareModule>,
}
impl Database {
    pub fn get_loaded_database() -> Option<Vec<Database>> {
        None
    }
}
