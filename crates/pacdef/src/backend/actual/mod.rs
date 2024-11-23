#[cfg(feature = "arch")]
pub mod arch;
#[cfg(feature = "debian")]
pub mod debian;
pub mod fedora;
pub mod flatpak;
pub mod python;
pub mod rust;
pub mod rustup;
pub mod snap;
pub mod void;
