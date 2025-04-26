## Template for a basic stereokit-rust program
There are two other templates (see the branches) if you prefer to use gradle for android or if you do not want to use winit and want to have an android version (gradle then being the only solution available)

### If you never used stereokit-rust before, you have to install the prerequisites:
See [installation](https://docs.rs/stereokit-rust/latest/stereokit_rust/#installation) and [how to build and test your application](https://docs.rs/stereokit-rust/latest/stereokit_rust/#how-to-build-and-test-your-application)

In order to use the build_sk_rs or compile_sks command you have to 'install' the project: `cargo install -F event-loop stereokit-rust`

### Download this template:
* git clone https://github.com/mvvvv/stereokit-template/
* For Windows only and if you want to get DLLs add to the PATH environment variable the directory ./target/debug/deps

Then, go to the Stereokit-template project and transform it to your project:
- by renaming the name, package and labels in Cargo.toml, 
- and removing the .git in order to create yours,

### Run your project on your PC's headset:
* Make sure you have [OpenXR installed](https://www.khronos.org/openxr/) with an active runtine.
* Launch: `cargo run`

### Run your project on your PC using the [simulator](https://stereokit.net/Pages/Guides/Using-The-Simulator.html): 
* Launch: `cargo run  -- --test`

If you're using VsCode you'll see two launchers in launch.json to debug the project.

### Build and create an exportable repository of your PCVR program:
`cargo build_sk_rs <the path of your exportable repository>`

## Run the project on your Android headset:
* Launch: `cargo apk run --lib`

## Build the release versions of your project:
For Android you'll have to set a [keystore](https://developer.android.com/studio/publish/app-signing). See [cargo-apk](https://github.com/rust-mobile/cargo-apk) to store the path and keyword
* Desktop : `cargo build --release`
* Android : `cargo apk build --lib --release`

Binaries and APK archives are produced under ./target/release

## Compile shaders 
Important: By default build_sk_rs compiles the shaders optimally for the target platform.

`compile_sks` calls the stereokit binary `skshaderc` using the following configuration:
* The shaders (*.hlsl files) must be created inside the `shaders_src` directory inside the root directory of your project. 
* The result (*.hlsl.sks files) will be produced inside the `assets/shaders` directory inside the root directory of your project.

To compile the *.hlsl files, go to the root directory of your project then launch `cargo compile_sks`

## Troubleshooting
Submit bugs on the [Issues tab](https://github.com/mvvvv/StereoKit-rust/issues), and ask questions in the [Discussions tab](https://github.com/mvvvv/StereoKit-rust/discussions)!

The project <https://github.com/StereoKit/StereoKit/> will give you many useful links (Discord/Twitter/Blog)
