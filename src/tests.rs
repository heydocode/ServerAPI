#[cfg(test)]
mod tests {
    use crate::*;
    use async_std::path::Path;
    use async_std::prelude::*;
    use async_std::fs::{self, File, OpenOptions};
    use async_std::task;

    #[async_std::test]
    async fn test_write_to_file() {
        let test_file_path = "test_write_to_file.txt";

        // Ensure the test file is clean
        if Path::new(test_file_path).exists().await {
            fs::remove_file(test_file_path).await.expect("Failed to remove test file");
        }

        // Write to the file
        write_to_file("Hello, world!".to_string(), test_file_path)
            .await
            .expect("Failed to write to file");

        // Read the file content
        let mut file = File::open(test_file_path).await.expect("Failed to open test file");
        let mut content = String::new();
        file.read_to_string(&mut content).await.expect("Failed to read test file");

        // Check the content
        assert_eq!(content, "Hello, world!");

        // Clean up
        fs::remove_file(test_file_path).await.expect("Failed to remove test file");
    }

    #[async_std::test]
    async fn test_update_credentials() {
        let test_file_path = "config/test_smtp_account.txt";

        // Ensure the config directory exists
        fs::create_dir_all("config").await.expect("Failed to create config directory");

        // Prepare the test file
        let initial_content = "username:example@gmail.com\npassword:password123123\n";
        fs::write(test_file_path, initial_content).await.expect("Failed to write initial content");

        // Update credentials
        update_credentials("new_username".to_string(), "new_password".to_string())
            .await
            .expect("Failed to update credentials");

        // Read the updated file content
        let mut file = File::open(test_file_path).await.expect("Failed to open test file");
        let mut content = String::new();
        file.read_to_string(&mut content).await.expect("Failed to read test file");

        // Check the updated content
        let expected_content = "username:new_username\npassword:new_password\n";
        assert_eq!(content, expected_content);

        // Clean up
        fs::remove_file(test_file_path).await.expect("Failed to remove test file");
    }
}
