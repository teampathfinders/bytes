#[macro_export]
macro_rules! bail {
    ($err_type: ident, $fmt: expr, $($args:expr),+) => {
        return Err($crate::Error::new($crate::ErrorKind::$err_type, format!($fmt, $($args),+)))
    };

    ($err_type: ident, $fmt: expr) => {
        return Err($crate::Error::new($crate::ErrorKind::$err_type, format!($fmt)))
    };

    ($err_type: ident) => {
        $crate::bail!($err_type, "No description")
    };
}

/// Shorthand for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    /// Unexpected end of file
    UnexpectedEof,
    /// An unknown error
    Other,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    #[inline]
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    #[inline]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?} | {}", self.kind, self.message)
    }
}