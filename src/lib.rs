/// Android Entry point
mod shared_logic;

#[allow(improper_ctypes_definitions)]
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_main(app: slint::android::AndroidApp) {
    use slint::android::init;
    init(app).unwrap();

    shared_logic::run().unwrap();
}
