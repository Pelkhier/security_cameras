use std::{
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use opencv::{
    core::{Scalar, Size},
    highgui,
    imgcodecs::imwrite,
    imgproc, objdetect,
    prelude::*,
    types, videoio,
};

pub fn how_many_cameras() -> Result<i32, String> {
    let mut count = 0;

    loop {
        let mut cam =
            videoio::VideoCapture::new(count, videoio::CAP_ANY).map_err(|e| e.to_string())?;
        let is_open = cam
            .open(count, videoio::CAP_ANY)
            .map_err(|e| e.to_string())?;
        if !is_open {
            break;
        }
        count += 1;
    }
    Ok(count)
}

#[tauri::command]
pub fn start_threads(state: tauri::State<Arc<AtomicBool>>) -> Result<(), String> {
    // let mut handles = vec![];
    for i in 0..how_many_cameras().map_err(|e| e.to_string())? {
        let thread_stop_signal = state.inner().clone();
        thread_stop_signal.store(false, Ordering::Relaxed);
        let _handle = thread::spawn(move || -> Result<(), String> {
            let filename = format!("../images/{}/image_{}_", i + 1, i + 1);
            let dir_path = std::env::current_dir().unwrap();
            let image_dirs = dir_path
                .parent()
                .unwrap()
                .join::<PathBuf>(format!("images/{}/", i + 1).into());
            std::fs::create_dir_all(image_dirs).ok();
            let xml =
                "C:\\tools\\opencv\\build\\etc\\haarcascades\\haarcascade_frontalface_default.xml";
            let mut face_detector =
                objdetect::CascadeClassifier::new(xml).map_err(|e| e.to_string())?;
            let mut cam =
                match videoio::VideoCapture::new(i, videoio::CAP_ANY).map_err(|e| e.to_string()) {
                    Ok(cam) => cam,
                    _ => return Err("Could not open camera".to_string()),
                };
            let mut frame = Mat::default();
            while !thread_stop_signal.load(Ordering::Relaxed) {
                let time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                cam.read(&mut frame).map_err(|e| e.to_string())?;

                let mut gray = Mat::default();

                imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)
                    .map_err(|e| e.to_string())?;

                // highgui::imshow("window", &mut frame).map_err(|e| e.to_string())?;
                let mut faces = types::VectorOfRect::new();
                face_detector
                    .detect_multi_scale(
                        &gray,
                        &mut faces,
                        1.1,
                        10,
                        objdetect::CASCADE_SCALE_IMAGE,
                        Size::new(30, 30),
                        Size::new(0, 0),
                    )
                    .map_err(|e| e.to_string())?;

                if faces.len() > 0 {
                    for face in faces.iter() {
                        imgproc::rectangle(
                            &mut frame,
                            face,
                            Scalar::new(0.0, 255.0, 0.0, 0.0),
                            2,
                            imgproc::LINE_8,
                            0,
                        )
                        .map_err(|e| e.to_string())?;
                        imwrite(
                            &format!("{}{}.jpg", filename, time),
                            &frame,
                            &types::VectorOfi32::new(),
                        )
                        .map_err(|e| e.to_string())?;
                        // std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                }

                if highgui::wait_key(1).map_err(|e| e.to_string())? == i + 49 {
                    cam.release().map_err(|e| e.to_string())?;
                    break;
                }
            }
            cam.release().map_err(|e| e.to_string())?;
            Ok(())
        });

        // handles.push(handle);
    }

    // for handle in handles {
    //     match handle.join().unwrap() {
    //         Ok(h) => h,
    //         Err(e) => println!("{:?}", e),
    //     }
    // }

    Ok(())
}

#[tauri::command]
pub fn stop_thread(state: tauri::State<Arc<AtomicBool>>) {
    state.inner().store(true, Ordering::Relaxed);
}
