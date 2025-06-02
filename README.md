Cross platform Rust Android Examples
===

Cargo examples cannot build lib/shared object.  Android requires a shared object with android_main().  Therefore, each
example is a workspace.

WGPU
---

Desktop

```
cd wgpu
cargo build
../target/debug/android-rust-example-apk-wgpu
```

Android

```
cd wgpu

cargo build --target=aarch64-linux-android
cp ../target/aarch64-linux-android/debug/libandroid_rust_example_apk_wgpu.so ./android/app/src/main/jniLibs/arm64-v8a/

# arm7 ?
# cargo build --target=armv7-linux-androideabi
# cp ../target/armv7-linux-androideabi/debug/libandroid_rust_example_apk_wgpu.so ./android/app/src/main/jniLibs/armeabi-v7a/

# x86_64 emu?
# cargo build --target=x86_64-linux-android
# cp ../target/x86_64-linux-android/debug/libandroid_rust_example_apk_wgpu.so ./android/app/src/main/jniLibs/x86_64/

cd android
./gradlew assembleDebug
./gradlew installDebug
```

> * In `android/app/src/main/AndroidManifest.xml` the meta-data tag with name="android.app.lib_name" and value="android_rust_example_apk_wgpu" specifies which shared object the ndk-glue should load
> * In `android/app/build.gradle` the `applicationId` is set to `co.realfit.nawinitwgpu` from `android-activity-examples` project


JNI
---
This shows how to interact with JNI after starting an app from a Rust shared library.  This requires a fork of
`android-activity` to support access to the main thread on Android.
