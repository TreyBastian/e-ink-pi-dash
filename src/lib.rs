pub mod ui;
pub mod shopify;

#[cfg(feature = "simulator")]
#[cfg(not(feature = "wavshare"))]
pub mod simulator;
pub mod app;
#[cfg(not(feature = "simulator"))]
#[cfg(feature = "wavshare")]
pub mod wavshare;