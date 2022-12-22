# Bevy in App
Integrate the [Bevy engine](https://github.com/bevyengine/bevy) into existing iOS | Android apps. 

## **Run on iOS**

```sh
# Add iOS target
rustup target add aarch64-apple-ios 

# Build for iOS device
sh ./ios_debug.sh
```

Then, Open `iOS/bevy_in_iOS.xcodeproj` with Xcode，connect an iOS device and run. 