#[macro_export]
macro_rules! service_ref {
    ($service:expr => &dyn $trait_reference: path) => {
        {
            use ::typed_di::service::service_instance::ServiceInstance;
            use ::typed_di::service::service_access::ServiceAccess;
            let service = $service;
            let instance = service.get_instance();
            let trait_reference = service.as_ref() as &dyn $trait_reference;
            let trait_reference: ::std::raw::TraitObject = unsafe {
                ::std::mem::transmute(trait_reference)
            };
            let trait_reference: &dyn $trait_reference = unsafe {
                ::std::mem::transmute(trait_reference)
            };
            ::typed_di::service::service::Service::from_trait_object(instance, trait_reference)
        }
    };
    ($service:expr => &'static dyn $trait_reference: path) => {
        {
            use ::typed_di::service::service_instance::ServiceInstance;
            use ::typed_di::service::service_access::ServiceAccess;
            let service = $service;
            let instance = service.get_instance();
            let trait_reference = service.as_ref() as &dyn $trait_reference;
            let trait_reference: ::std::raw::TraitObject = unsafe {
                ::std::mem::transmute(trait_reference)
            };
            let trait_reference: &'static dyn $trait_reference = unsafe {
                ::std::mem::transmute(trait_reference)
            };
            ::typed_di::service::service::Service::from_trait_object(instance, trait_reference)
        }
    };
}