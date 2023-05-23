#[tauri::command]
pub fn get_folder_count() -> Result<u32, String> {
    let current_dir = std::env::current_dir().unwrap();
    let images_dir = current_dir.parent().unwrap().join(format!("images"));
    let folders_count = std::fs::read_dir(images_dir)
        .map_err(|_| "Error reading directory".to_string())?
        .count();
    Ok(folders_count as u32)
}

#[tauri::command]
pub fn get_images(dir_number: &str) -> Result<Vec<String>, String> {
    let mut images = vec![];

    let current_dir = std::env::current_dir().unwrap();
    let images_dir = current_dir
        .parent()
        .unwrap()
        .join(format!("images/{}", dir_number));
    let files = std::fs::read_dir(images_dir).map_err(|e| e.to_string())?;

    for file in files {
        let filename = file
            .as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let parent = file
            .unwrap()
            .path()
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        images.push(format!("{}/{}", parent, filename));
    }
    images.reverse();
    Ok(images)
}
