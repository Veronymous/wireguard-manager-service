#[macro_export]
macro_rules! try_wg_function {
    ($function:expr) => {
        match $function {
            Ok(result) => result,
            Err(err) => return Err(Status::aborted(err.to_string())),
        };
    };
}
