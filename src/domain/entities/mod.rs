pub mod state;
pub mod lga;
pub mod ward;
pub mod postal_code;
pub mod address;

// Re-exports for convenience
pub use state::{State, CreateStateRequest, UpdateStateRequest};
pub use lga::{Lga, CreateLgaRequest, UpdateLgaRequest};
pub use ward::{Ward, CreateWardRequest, UpdateWardRequest};
pub use postal_code::{PostalCode, CreatePostalCodeRequest, UpdatePostalCodeRequest};
pub use address::{Address, AddressValidationRequest, AddressValidationResponse, AddressSuggestion};
