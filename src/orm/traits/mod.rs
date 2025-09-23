//! A suite of traits that offer ORM capabilities to the types implementing them

mod with_id;
pub use with_id::WithId;

mod with_table;
pub use with_table::WithTable;

mod with_db;
pub use with_db::WithDb;

mod with_binding;
pub use with_binding::WithBinding;

mod with_crud_events;
pub use with_crud_events::WithCrudEvents;

mod with_db_model;
pub use with_db_model::WithDbModel;
