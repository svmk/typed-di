#![feature(const_type_name)]
#![feature(const_fn)]
#![feature(raw)]
pub mod service;
pub mod macros;
pub mod service_access;
pub mod container_declaration;
pub mod service_instance;
pub mod argument_instance;
pub mod service_id;
pub mod service_id_resolver;
pub mod argument_id;
pub mod argument_id_resolver;
pub mod async_di_container;
pub mod async_blocking_context;
pub mod sync_context;
pub mod service_builder;
pub mod di_container;
pub mod error;