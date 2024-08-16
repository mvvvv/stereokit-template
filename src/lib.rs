use stereokit_rust::{
    maths::{Pose, Quat, Vec3},
    ui::Ui,
};
use stereokit_rust::{sk::Sk, system::Log};

//use crate::launch;
#[cfg(target_os = "android")]
use android_activity::AndroidApp;

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
/// The main function for android app
fn android_main(app: AndroidApp) {
    use stereokit_rust::sk::{DepthMode, OriginMode, SkSettings};
    use stereokit_rust::system::LogLevel;
    let mut settings = SkSettings::default();
    settings
        .app_name("stereokit-rust")
        .assets_folder("assets")
        .origin(OriginMode::Floor)
        .render_multisample(4)
        .render_scaling(2.0)
        .depth_mode(DepthMode::Stencil)
        .log_filter(LogLevel::Diagnostic);

    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Debug));
    stereokit_rust::system::BackendOpenXR::request_ext("XR_FB_passthrough");

    _main(settings.init(app).unwrap());
}

pub fn _main(sk: Sk) {
    let is_testing = false;
    Log::diag("Launch my_vr_program");
    launch(is_testing, sk);
    Sk::shutdown();
}

pub fn launch(_is_testing: bool, sk: Sk) {
    let mut window_pose = Pose::new(Vec3::new(0.0, 1.5, -0.5), Some(Quat::from_angles(0.0, 180.0, 0.0)));
    while let Some(_token) = sk.step() {
        Ui::window_begin("test window", &mut window_pose, None, None, None);
        if Ui::button("quit lel", None) {
            Log::diag("See You !!");
            break;
        }
        Ui::window_end();
    }
}
