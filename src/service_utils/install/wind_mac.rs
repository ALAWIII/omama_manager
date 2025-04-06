use crate::{Result, anyhow};
use ollama_td::{
    OllamaDownload, OllamaDownloadBuilder, Platform, TVersion, Unix, Windows, download,
    download_customize,
};

use reqwest::Response;
use std::path::PathBuf;
use std::{fs::File, path::Path};
use tokio::fs::remove_file;
use zip_extract::extract;

pub(super) async fn install_windows_arm_tool<T>(f_stream: T) -> Result<()>
where
    T: AsyncFnOnce(Response, &mut Path) -> Result<PathBuf>,
{
    let platform = Platform::Windows(Windows::Arm);
    let path_zip = download_customize(ollama_build(platform)?, f_stream).await?;
    unpack_zip(
        path_zip.as_path(),
        &path_zip
            .parent()
            .ok_or("failed to get the parent of current directory , arm ollama tool")
            .map_err(|e| anyhow!(format!("{e}")))?
            .join("ollama-windows"),
    )
    .await?;
    Ok(())
}

pub(super) async fn install_windows_x86_tool<T>(f_stream: T) -> Result<()>
where
    T: AsyncFnOnce(Response, &mut Path) -> Result<PathBuf>,
{
    let platform = Platform::Windows(Windows::X86);
    let path_zip = download_customize(ollama_build(platform)?, f_stream).await?;
    unpack_zip(
        path_zip.as_path(),
        &path_zip
            .parent()
            .ok_or("failed to get the parent of current directory , x86 ollama tool")
            .map_err(|e| anyhow!(format!("{e}")))?
            .join("ollama-windows"),
    )
    .await?;
    Ok(())
}

pub(super) async fn install_macos_tool<T>(f_stream: T) -> Result<()>
where
    T: AsyncFnOnce(Response, &mut Path) -> Result<PathBuf>,
{
    let platform = Platform::Unix(Unix::DarwinZip);
    let path_zip = download_customize(ollama_build(platform)?, f_stream).await?;
    unpack_zip(
        path_zip.as_path(),
        path_zip
            .parent()
            .ok_or("failed to get the parent of current directory , mac ollama tool")
            .map_err(|e| anyhow!(format!("{e}")))?,
    )
    .await?;
    Ok(())
}

fn ollama_build(platform: Platform) -> Result<OllamaDownload> {
    OllamaDownloadBuilder::new()?
        .platform(platform)
        .tag_version(TVersion::Latest)
        .build()
}

async fn unpack_zip(path_zip: &Path, destination: &Path) -> Result<()> {
    extract(File::open(path_zip)?, destination, false)?;
    remove_file(path_zip).await?;
    Ok(())
}

//--------------------------------tests-----------------------------
#[cfg(test)]
mod quick_test {
    use super::unpack_zip;
    use ollama_td::Result;
    use std::io::{BufWriter, Read, Write};
    use std::{
        fs::{File, OpenOptions},
        path::Path,
    };
    use tokio::fs::remove_file;
    use zip::{ZipWriter, write::SimpleFileOptions};
    fn compress_file(file_path: &str, zip_path: &str) -> zip::result::ZipResult<()> {
        //let file = File::open(file_path)?;
        let zip_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(zip_path)?;
        let mut zip = ZipWriter::new(BufWriter::new(zip_file));

        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        zip.start_file(file_path, options)?;

        let mut buffer = Vec::new();
        File::open(file_path)?.read_to_end(&mut buffer)?;
        zip.write_all(&buffer)?;

        zip.finish()?;
        println!("✅ File compressed into {}", zip_path);

        Ok(())
    }

    #[tokio::test]
    async fn check_unpack_zip() -> Result<()> {
        let test_dir = Path::new("./loloa.txt");
        File::options()
            .create(true)
            .read(true)
            .truncate(true)
            .write(true)
            .open(test_dir)?;
        compress_file("./loloa.txt", "./potato.zip")?;
        remove_file("./loloa.txt").await.unwrap();
        let path = Path::new("./potato.zip");
        let status = unpack_zip(path, path.parent().unwrap()).await;
        //dbg!(&process);
        assert!(status.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn check_unpack_zip_create_child_dir() -> Result<()> {
        let test_dir = Path::new("./loloa.txt");
        File::options()
            .create(true)
            .read(true)
            .truncate(true)
            .write(true)
            .open(test_dir)?;
        compress_file("./loloa.txt", "./potato.zip")?;
        remove_file("./loloa.txt").await.unwrap();
        let path = Path::new("./potato.zip");
        let status = unpack_zip(path, &path.parent().unwrap().join("./flafel")).await;
        //dbg!(&process);
        assert!(status.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn check_unpack_zip_parent_not_exist() -> Result<()> {
        let test_dir = Path::new("./loloa.txt");
        File::options()
            .create(true)
            .read(true)
            .truncate(true)
            .write(true)
            .open(test_dir)?;
        compress_file("./loloa.txt", "./potato.zip")?;
        remove_file("./loloa.txt").await.unwrap();
        let path = Path::new("./potato.zip");
        let status = unpack_zip(path, &path.parent().unwrap().join("./muhlabieh/flafel")).await;
        //dbg!(&process);
        assert!(status.is_err());
        Ok(())
    }
}
