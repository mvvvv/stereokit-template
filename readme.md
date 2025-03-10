## Template for a basic stereokit-rust program
There are two other templates (see the branches) if you prefer to use gradle for android or if you do not want to use winit and want to have an android version (gradle then being the only solution available)

### Download the source project then this template:
* git clone --recursive https://github.com/mvvvv/StereoKit-rust/
* git clone https://github.com/mvvvv/stereokit-template/
* For Windows only add to the PATH environment variable the directory ./target/debug/deps

First, check that you can launch the Stereokit-rust demos as described here https://github.com/mvvvv/StereoKit-rust/blob/master/README.md

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

## Compile shaders:
If you want to create your own shaders, you'll need the binary `compile_sks` of the stereokit-rust project and so you have to 'install' the project: 
* `cargo install --path <path to git directory of Stereokit-rust>`

`compile_sks` calls the stereokit binary `skshaderc` using the following configuration:
* The shaders (*.hlsl files) must be created inside the shaders_src directory inside the root directory of your project. 
* The result (*.hlsl.sks files) will be produced inside the assets/shaders directory inside the root directory of your project.

To compile the *.hlsl files, go to the root directory of your project then launch `cargo compile_sks`

## Troubleshooting
Submit bugs on the [Issues tab](https://github.com/mvvvv/StereoKit-rust/issues), and ask questions in the [Discussions tab](https://github.com/mvvvv/StereoKit-rust/discussions)!

The project <https://github.com/StereoKit/StereoKit/> will give you many useful links (Discord/Twitter/Blog)
