## Template for a basic stereokit-rust program

### Download the source project then this template:
* git clone --recursive https://github.com/mvvvv/StereoKit-rust/
* git clone -b gradle https://github.com/mvvvv/stereokit-gradle-template/

First, check that you can launch the Stereokit-rust demos as described here https://github.com/mvvvv/StereoKit-rust/blob/master/README.md

Then, go to the Stereokit-gradle-template project and transform it to your project:
- by renaming the package.name in Cargo.toml, 
- by renaming cargo.libName (same as package.name from Cargo.toml), android.applicationId and android.main in gradle.properties,
- by deleting or modifying the path and package's name of MainActivity.java (your choice impacts android.main &uarr; and android:hasCode attribute in AndroidManifest.xml),
- and removing the .git folder in order to create yours.

The file AndroidManifest.xml is under the single Android module `./app` [where you could add some Java classes if needed](https://developer.android.com/studio/projects#ProjectView except for res and assets which are here at the root of the project)

### Run your project on your PC's headset :
* Make sure you have [OpenXR installed](https://www.khronos.org/openxr/) with an active runtine.
* Launch: `cargo run`

### Run your project on your PC using the [simulator](https://stereokit.net/Pages/Guides/Using-The-Simulator.html) 
* Launch: `cargo run  -- --test`

If you're using VsCode you'll see two launchers in launch.json to debug the project.


## Run the project on your Android headset thanks to gradle:
* install openjdk v8 or v17
* install [gradle](https://gradle.org/install/) v8.9 or more
* launch `cargo install cargo-ndk`
* launch `rustup target add aarch64-linux-android `
* create a [keystore](https://developer.android.com/studio/publish/app-signing) then a file [.gradle/gradle.properties](https://www.repeato.app/creating-a-release-signed-apk-file-using-gradle/) to store and forget the confidential values
* Run the debug then show the Android log for this project
    - On Windows launch: `./gradlew run && cmd /c logcat.cmd`
    - On others launch: `./gradlew run && sh logcat.cmd`

## Build the release versions of your project:
* Desktop : `cargo build --release` &rarr; Binaries are produced under ./target/release
* Android : `./gradlew buildRelease` &rarr; APK archive is produced under ./app/build/outputs/apk. You can install it with './gradlew installRelease'

## Compile shaders
If you want to create your own shaders, you'll need the binary `compile_sks` of the stereokit-rust project and so you have to 'install' the project: 
* `cargo install --path <path to git directory of StereoKit-rust>`

`compile_sks` calls the stereokit binary `skshaderc` using the following configuration:
* The shaders (*.hlsl files) must be created inside the shaders_src directory inside the root directory of your project. 
* The result (*.hlsl.sks files) will be produced inside the assets/shaders directory inside the root directory of your project.

To compile the *.hlsl files, go to the root directory of your project then launch `cargo compile_sks`

## Troubleshooting
Submit bugs on the [Issues tab](https://github.com/mvvvv/StereoKit-rust/issues), and ask questions in the [Discussions tab](https://github.com/mvvvv/StereoKit-rust/discussions)!

The project <https://github.com/StereoKit/StereoKit/> will give you many useful links (Discord/Twitter/Blog)
