
use std::{fs::File, io::Write, path::Path};
use tauri::api::dialog::message;
use mailbox::mbox::Mbox;

#[tauri::command]
fn convert_mbox(file_path: String) -> Result<(), String> {
    let mbox = Mbox::from_path(&file_path).map_err(|e| e.to_string())?;
    let parent_dir = Path::new(&file_path).parent().unwrap();
    let output_path = parent_dir.join("converted.csv");
    let mut wtr = csv::Writer::from_path(output_path).map_err(|e| e.to_string())?;
    wtr.write_record(&["Date", "From", "To", "Subject"]).unwrap();
    for msg in mbox.iter() {
        let date = msg.headers().get("Date").unwrap_or("").to_string();
        let from = msg.headers().get("From").unwrap_or("").to_string();
        let to = msg.headers().get("To").unwrap_or("").to_string();
        let subject = msg.headers().get("Subject").unwrap_or("").to_string();
        wtr.write_record(&[date, from, to, subject]).unwrap();
    }
    wtr.flush().unwrap();
    Ok(())
}
