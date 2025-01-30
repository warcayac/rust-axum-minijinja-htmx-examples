mod config;
mod init;
mod routes;
mod functions;
mod errors;
mod state;

mod domain; // domain layer (entities, value objects, etc.): Core Business Logic
mod application; // application layer (use cases or business logic): Use Cases
mod infrastructure; // infrastructure layer (database connection and ORM, etc.): External Systems
mod presentation; // presentation layer (view, controller, etc.): UI/Endpoints
mod middleware; // middleware layer (logging, authentication, etc.): Middleware

// re-export
pub use init::*;
pub use routes::*;
pub use functions::*;
pub use errors::*;
pub use state::*;

mod templates;
pub use templates::*;
pub use middleware::*;
