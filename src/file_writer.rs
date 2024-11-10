use async_std::fs::File;
use async_std::fs::OpenOptions;
use async_std::path::Path;
use async_std::prelude::*;

pub async fn write_to_file(text: String, file_path: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .await
        .map_err(|e| e.to_string())?;

    file.write_all(text.as_bytes())
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_credentials(
    username: String,
    password: String,
    receiver: String,
) -> Result<(), String> {
    let file_path = "config/smtp_account.txt";
    let path = Path::new(file_path);

    let mut content = String::new();
    {
        let mut file = File::open(path).await.map_err(|e| e.to_string())?;
        file.read_to_string(&mut content)
            .await
            .map_err(|e| e.to_string())?;
    }

    let updated_content = content
        .replace(
            "username:example@gmail.com",
            &format!("username:{}", username),
        )
        .replace("password:password123123", &format!("password:{}", password))
        .replace(
            "receiver:example@gmail.com",
            &format!("receiver:{}", receiver),
        );

    {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
            .await
            .map_err(|e| e.to_string())?;
        file.write_all(updated_content.as_bytes())
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
