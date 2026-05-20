use sea_orm::DatabaseConnection;
// Estructura que usaremos para compartir el contexto de la aplicación, como la conexión a la base de datos, entre los diferentes handlers.
#[derive(Clone)]
pub struct AppContext {
    pub conn: DatabaseConnection,
    pub app_name: String,
    pub app_version: String
}