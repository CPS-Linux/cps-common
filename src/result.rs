use std::fmt::Debug;

pub trait ResultExt<T, E> {
    fn unwrap_or_display(self) -> T;
    fn expect_with_end<A: AsRef<str>>(self, message: A) -> T;
}

impl<T, E: Debug> ResultExt<T, E> for Result<T, E> {
    #[inline]
    #[track_caller]
    fn unwrap_or_display(self) -> T {
        match self {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                eprintln!("Failure at file {}, line {}", file!(), line!());
                std::process::exit(1);
            }
        }
    }
    #[inline]
    #[track_caller]
    fn expect_with_end<A: AsRef<str>>(self, message: A) -> T {
        match self {
            Ok(o) => o,
            Err(_) => {
                eprintln!("Error: {}\nin {}:{}", message.as_ref(), file!(), line!());
                std::process::exit(1);
            }
        }
    }
}
