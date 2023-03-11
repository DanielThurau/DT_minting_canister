use ic_canister_log::declare_log_buffer;

// Info log messages
declare_log_buffer!(name = INFO, capacity = 1000);

// Error log messages.
declare_log_buffer!(name = ERROR, capacity = 1000);

/// Adds a new record to a canister log buffer and print from ic_cdk
#[macro_export]
macro_rules! c_log {
    ($sink:expr, $message:expr $(,$args:expr)* $(,)*) => {{
        let message = std::format!($message $(,$args)*);
        // Print the message for convenience for local development (e.g. integration tests)
        ic_cdk::println!("{}", &message);
        ic_canister_log::log!($sink, $message $(,$args)*);
    }}
}
