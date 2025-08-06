# isiro

A calculator application built with [Slint UI](https://slint.dev/).

## Build and Run the Application

Requires [Rust](https://www.rust-lang.org/) to be installed.

```bash
cargo run --release
```

## Install the Application

Ensure the Rust path is included in your environment variables.

```bash
cargo install --path .
```

## isiro in Breeze Style (KDE Plasma, Linux)

| Dark Theme                                                                                                                                 | Light Theme                                                                                                                                      |
| ------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| <img width="457" height="683" alt="Breeze_Native" src="https://github.com/user-attachments/assets/3550d841-649a-466c-9ca9-753c3aa33454" /> | <img width="455" height="680" alt="Breeze_Native_Light" src="https://github.com/user-attachments/assets/2226aec1-2c61-459f-b75f-f6869bcf3dad" /> |


## isiro in Fluent Style (Windows 11)

| Dark Theme                                                                                                                                  | Light Theme                                                                                                                                       |
| ------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| <img width="432" height="669" alt="Windows_Native" src="https://github.com/user-attachments/assets/1bf98322-00e5-42dc-b238-72aa438ffe35" /> | <img width="438" height="673" alt="Windows_Native_Light" src="https://github.com/user-attachments/assets/4857db74-b9ca-41b9-861d-0fb25ea61272" /> |




## Build and Run on Android

To build and deploy `isiro` for Android, ensure you have the Android SDK, NDK (version 29.0.13846066 recommended), and platform 30 installed. The application uses the `backend-android-activity-06` feature of the Slint crate.


### Prerequisites

1. Install the Android toolchain for Rust:
   ```bash
   rustup target add aarch64-linux-android
   ```
2. Install `cargo-apk` for building and deploying Android APKs:
   ```bash
   cargo install cargo-apk
   ```
3. Set up the following environment variables:
   - `ANDROID_HOME`: The directory where your Android SDK is located (e.g., `$HOME/Android/Sdk`).
   - `ANDROID_NDK_ROOT`: The directory where your Android NDK is located (e.g., `$HOME/Android/Sdk/ndk/29.0.13846066`).
   - `JAVA_HOME`: The directory where your Java compiler (`javac`) is located (e.g., `/opt/android-studio/jbr`). Optional if `javac` is in your `$PATH`.

   For Bash/Zsh:
   ```bash
   export ANDROID_HOME=$HOME/Android/Sdk
   export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/29.0.13846066
   export JAVA_HOME=/opt/android-studio/jbr
   export PATH=$ANDROID_HOME/platform-tools:$PATH
   ```

   For Fish:
   ```fish
   set -x ANDROID_HOME $HOME/Android/Sdk
   set -x ANDROID_NDK_ROOT $ANDROID_HOME/ndk/29.0.13846066
   set -x JAVA_HOME /opt/android-studio/jbr
   set -x PATH $ANDROID_HOME/platform-tools $PATH
   ```

4. Ensure your `Cargo.toml` includes the Slint crate with the `backend-android-activity-06` feature and specifies a `cdylib` crate type:
   ```toml
   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   slint = { version = "1.6", features = ["backend-android-activity-06"] }
   ```

5. For release builds, generate a signing key:
   ```bash
   keytool -genkeypair \
       -v \
       -keystore ~/.android/isiro-release-key.jks \
       -keyalg RSA \
       -keysize 2048 \
       -validity 10000 \
       -alias isiro
   ```

### Building and Running

Ensure a physical device or emulator is running. Compile and run the application with:

```bash
cargo apk run --target aarch64-linux-android --lib
```

For a release build:

```bash
cargo apk run --target aarch64-linux-android --lib --release
```

### Notes

- The Android backend uses the `android-activity` crate, re-exported under `slint::android::android_activity`.
- The entry point for the Android application is the `android_main` function, which must be marked with `#[unsafe(no_mangle)]`. Example:
  ```rust
  #[unsafe(no_mangle)]
  fn android_main(app: slint::android::AndroidApp) {
      slint::android::init(app).unwrap();
      slint::slint! {
          export component MainWindow inherits Window {
              Text { text: "Hello World"; }
          }
      }
      MainWindow::new().unwrap().run().unwrap();
  }
  ```
- Refer to the [Android Developer’s Guide](https://developer.android.com/ndk/guides) for detailed setup instructions for the Android SDK and NDK.
- Alternative build tools like `xbuild` can be used, but `cargo-apk` is recommended for simplicity.

## License

This project is licensed under the [GNU GPL 3.0](LICENSE).

## Acknowledgments

- [Slint UI](https://slint.dev/): For providing the framework to build the user interface.
- [Rust](https://www.rust-lang.org/): For the programming language used to develop this application.
