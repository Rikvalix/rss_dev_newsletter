pub trait AiI {

    fn generate_resume(&self, file_path: &str, file_name: &str) -> impl Future<Output = String>;
}