pub mod connection_manager;
pub mod message_router;
pub mod message_validator;
pub mod rate_limiter;
pub mod room_service;

// Tests disabled - require full DB setup
// #[cfg(test)]
// mod connection_manager_tests;
//
// #[cfg(test)]
// mod message_router_tests;
//
// #[cfg(test)]
// mod room_service_tests;
//
// #[cfg(test)]
// mod rate_limiter_tests;
//
// #[cfg(test)]
// mod message_validator_tests;
