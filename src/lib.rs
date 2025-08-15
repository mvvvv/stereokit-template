use stereokit_rust::{
    maths::{Pose, Quat, Vec3},
    prelude::*,
    ui::Ui,
};

//use crate::launch;
#[cfg(target_os = "android")]
use android_activity::AndroidApp;

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
/// The main function for android app
fn android_main(app: AndroidApp) {
    let app_loop = app.clone();
    use std::thread;
    // In the main thread we purge the first events.
    Sk::poll_first_events(&app);

    // Then we create the thread_SK
    if let Ok(thread) = thread::Builder::new().name("thread_SK".to_string()).spawn(move || {
        loop {
            use stereokit_rust::sk::{DepthMode, OriginMode, SkSettings};
            use stereokit_rust::system::LogLevel;
            let mut settings = SkSettings::default();
            settings
                .app_name("rust_gradle")
                .origin(OriginMode::Floor)
                .render_multisample(4)
                .render_scaling(2.0)
                .depth_mode(DepthMode::D32)
                .log_filter(LogLevel::Diagnostic);

            android_logger::init_once(
                android_logger::Config::default().with_max_level(log::LevelFilter::Debug).with_tag("SKit-rs"),
            );

            let app_init = app.clone();
            let sk = settings.init(app_init).unwrap();

            _main(sk);
        }
    }) {
        while !thread.is_finished() {
            app_loop.poll_events(None, |poll_event| Log::diag(format!("!!!!!!Event : {:?}", poll_event)));
        }
    }
}

pub fn _main(sk: Sk) {
    let is_testing = false;
    Log::diag("Launch my_vr_program");
    launch(sk, is_testing);
    Sk::shutdown();
}

pub fn launch(sk: Sk, _is_testing: bool) {
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
