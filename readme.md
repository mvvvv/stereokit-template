## Template for a basic stereokit-rust program
This template is for a basic stereokit-rust program using winit for the event loop and gradle for the Android build.

There are two other templates (see the branches) if you prefer to not use gradle for android (main) or if you do not want to use winit and want to have an android version (gradle then being the only solution available).

### If you never used stereokit-rust before, you have to install the prerequisites:
See [installation](https://docs.rs/stereokit-rust/latest/stereokit_rust/#installation) and [how to build and test your application](https://docs.rs/stereokit-rust/latest/stereokit_rust/#how-to-build-and-test-your-application)

In order to use the build_sk_rs or compile_sks command you have to 'install' the project: `cargo install -F event-loop stereokit-rust`

### Download the source project then this template:
* git clone -b gradle https://github.com/mvvvv/stereokit-gradle-template/
* For Windows only add to the PATH environment variable the directory ./target/debug/deps

Then, go to the Stereokit-gradle-template project and transform it to your project:
- by renaming the package.name in Cargo.toml, 
- by renaming cargo.libName (same as package.name from Cargo.toml), android.applicationId and android.main in gradle.properties,
- by deleting or modifying the path and package's name of MainActivity.java (your choice impacts android.main &uarr; and android:hasCode attribute in AndroidManifest.xml),
- and removing the .git folder in order to create yours.

The file AndroidManifest.xml is under the single Android module `./app` [where you could add some Java classes if needed](https://developer.android.com/studio/projects#ProjectView except for res and assets which are here at the root of the project)

### Run your project on your PC's headset:
* Make sure you have [OpenXR installed](https://www.khronos.org/openxr/) with an active runtine.
* Launch: `cargo run` (using Wayland on Linux may require to unset temporarily the DISPLAY variable: `DISPLAY= cargo run`)

### Run your project on your PC using the [simulator](https://stereokit.net/Pages/Guides/Using-The-Simulator.html): 
* Launch: `cargo run  -- --test`

If you're using VsCode you'll see two launchers in launch.json to debug the project.

### Build and create an exportable repository of your PC VR/MR program:
`cargo build_sk_rs <the path of your exportable repository>`


## Run the project on your Android headset thanks to gradle:
* Run the debug then show the Android log for this project
    - On Windows launch: `./gradlew.bat run && cmd /c logcat.cmd` or `(./gradlew.bat run) -and (cmd /c logcat.cmd)`
    - On others launch: `./gradlew run && sh logcat.cmd`

## Build the release versions of your project:
* Desktop : `cargo build --release` &rarr; Binaries are produced under ./target/release
* Android : `./gradlew buildRelease` &rarr; APK archive is produced under ./app/build/outputs/apk. You can install it with './gradlew installRelease'

## Compile shaders:
Important: By default build_sk_rs compiles the shaders optimally for the target platform.

`compile_sks` calls the stereokit binary `skshaderc` using the following configuration:
* The shaders (*.hlsl files) must be created inside the shaders_src directory inside the root directory of your project. 
* The result (*.hlsl.sks files) will be produced inside the assets/shaders directory inside the root directory of your project.

To compile the *.hlsl files, go to the root directory of your project then launch `cargo compile_sks`

## Run the documentation tests:

This is the regular way to run the doc tests: `cargo test --doc` or `cargo test` but screenshots will fail on Wayland because of the DISPLAY variable. So you have to unset it before running the tests: `DISPLAY= cargo test`


## Troubleshooting
Submit bugs on the [Issues tab](https://github.com/mvvvv/StereoKit-rust/issues), and ask questions in the [Discussions tab](https://github.com/mvvvv/StereoKit-rust/discussions)!

The project <https://github.com/StereoKit/StereoKit/> will give you many useful links (Discord/Twitter/Blog)
