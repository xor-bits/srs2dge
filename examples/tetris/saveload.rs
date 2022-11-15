#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs::File,
    io::{Read, Write},
};

//

#[cfg(not(target_arch = "wasm32"))]
pub fn save_highscore(highscore: usize) {
    if let Err(err) = (|| -> Result<_, String> {
        let mut highscore_file = File::options()
            .create(true)
            .write(true)
            .open("highscore")
            .map_err(|err| err.to_string())?;
        write!(
            highscore_file,
            "{}",
            ron::to_string(&highscore).map_err(|err| err.to_string())?
        )
        .map_err(|err| err.to_string())?;
        Ok(())
    })() {
        tracing::warn!("Failed to write highscore file: {err}");
    }
}

#[cfg(target_arch = "wasm32")]
pub fn save_highscore(highscore: usize) {
    if let Err(err) = (|| -> Result<(), &str> {
        web_sys::window()
            .ok_or("Failed to get window")?
            .local_storage()
            .map_err(|_| "Failed to load local storage")?
            .ok_or("Failed to load local storage")?
            .set("highscore", highscore.to_string().as_str())
            .map_err(|_| "Failed to insert highscore to local storage")
    })() {
        tracing::warn!("Failed to write highscore file: {err}");
    };
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_highscore() -> usize {
    use std::error::Error;
    match (|| -> Result<usize, Box<dyn Error>> {
        let mut highscore_file = File::options().read(true).open("highscore")?;
        let mut buf = String::new();
        highscore_file.read_to_string(&mut buf)?;
        Ok(ron::from_str(&buf)?)
    })() {
        Ok(s) => s,
        Err(err) => {
            tracing::warn!("Failed to read highscore file: {err}");
            0
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load_highscore() -> usize {
    match (|| -> Result<usize, &str> {
        web_sys::window()
            .ok_or("Failed to get window")?
            .local_storage()
            .map_err(|_| "Failed to load local storage")?
            .ok_or("Failed to load local storage")?
            .get("highscore")
            .map_err(|_| "Failed to insert highscore to local storage")?
            .unwrap_or("0".to_owned())
            .parse()
            .map_err(|err| "Failed to parse highscore: {err}")
    })() {
        Ok(s) => s,
        Err(err) => {
            tracing::warn!("Failed to read highscore file: {err}");
            0
        }
    }
}
