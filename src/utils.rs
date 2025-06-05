pub fn open_link(url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        if let Some(w) = window() {
            let _ = w.open_with_url(url);
        }
    }
}
