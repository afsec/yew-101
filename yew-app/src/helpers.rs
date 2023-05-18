pub fn console_log<T: AsRef<str>>(msg: T) {
    use gloo_console::log;
    log!(msg.as_ref());
}
