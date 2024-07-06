use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{error::FluxyResult, lazy::APP_CONFIG_DIR};

#[tauri::command]
pub async fn get_star_state() -> FluxyResult<bool> {
    let file_path = APP_CONFIG_DIR.join("STAR");
    if !file_path.exists() || file_path.is_dir() {
        info!("star 文件不存在");
        return Ok(false);
    }

    let mut file = File::open(&file_path).await.map_err(|e| {
        error!(
            message = "只读打开文件时出错",
            path = ?file_path,
            error = ?e
        );
        e
    })?;
    let mut buf = [0u8; 1];
    file.read_exact(&mut buf).await?;

    info!(message = "保存的 star 状态", state = buf[0] == 1);

    Ok(buf[0] == 1)
}

#[tauri::command]
pub async fn stared() -> FluxyResult<()> {
    let file_path = APP_CONFIG_DIR.join("STAR");
    let mut file = File::create(&file_path).await.map_err(|e| {
        error!(
            message = "写入打开文件时出错",
            path = ?file_path,
            error = ?e
        );
        e
    })?;
    file.write_all(&[1]).await.map_err(|e| {
        error!(message = "向文件写入数据时出错", error = ?e);
        e
    })?;
    info!("已保存 star 状态");

    Ok(())
}
