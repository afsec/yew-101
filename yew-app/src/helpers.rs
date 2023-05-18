pub fn console_log<T: AsRef<str>>(msg: T) {
    use gloo_console::log;
    #[cfg(debug_assertions)]
    log!(msg.as_ref());
}
