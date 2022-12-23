# Bevy in App
Integrate the [Bevy engine](https://github.com/bevyengine/bevy) into existing iOS | Android apps. 

## **iOS**

```sh
# Add iOS target
rustup target add aarch64-apple-ios 

# Build for iOS device
sh ./ios_debug.sh
```

Then, Open `iOS/bevy_in_iOS.xcodeproj` with Xcode，connect an iOS device and run. 

## **Android**

### Set up Android environment

Assuming your computer already has Android Studio installed, go to `Android Studio` > `Tools` > `SDK Manager` > `Android SDK` > `SDK Tools`. Check the following options for installation and click OK. 

- [x] Android SDK Build-Tools
- [x] Android SDK Command-line Tools
- [x] NDK(Side by side)

Then, set two following environment variables:

```sh
export ANDROID_SDK_ROOT=$HOME/Library/Android/sdk
# Replace the NDK version number with the version you installed 
export NDK_HOME=$ANDROID_SDK_ROOT/ndk/23.1.7779620
```

### Add build targets

```sh
# Since simulator and virtual devices only support GLES, 
# `x86_64-linux-android` and `i686-linux-android` targets are not necessary
rustup target add aarch64-linux-android
```

### Build
```sh
# Install cargo-so subcommand
cargo install cargo-so

# Build
sh ./android_debug.sh
```