use std::sync::LazyLock;
use std::env;

pub static IS_CLIENT_PAGES_DEV: LazyLock<bool> = LazyLock::new(|| {
    env::args().any(|arg| arg == "client-pages-dev")
});

pub static IS_API_DEV: LazyLock<bool> = LazyLock::new(|| {
    env::args().any(|arg| arg == "api-dev")
});
