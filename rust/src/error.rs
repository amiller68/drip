//! Common error modules for the Blossom library

/// A common catch all minimal error type for the Blossom library.
#[derive(Debug, thiserror::Error)]
pub enum BlossomError {}

/// Convenience type for any fallible method that can produce a [`BlossomError`].
pub type BlossomResult<T> = Result<T, BlossomError>;
