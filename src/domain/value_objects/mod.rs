pub mod state_code;
pub mod lga_code;
pub mod ward_code;
pub mod postal_code;
pub mod coordinates;

// Re-exports for convenience
pub use state_code::{StateCode, StateCodeError};
pub use lga_code::{LgaCode, LgaCodeError};
pub use ward_code::{WardCode, WardCodeError};
pub use postal_code::{PostalCode, PostalCodeError};
pub use coordinates::{Coordinates, CoordinatesError};
