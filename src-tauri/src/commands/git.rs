use std::path::PathBuf;
use std::process::Command;

use crate::types::{FileStatus, RepoInfo};

fn run_git(path: &str, args: &[String]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(PathBuf::from(path))
        .output()
        .map_err(|e| format!("Failed to run git: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim_end().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(if err.is_empty() {
            format!(
                "git {} failed",
                args.first().map(String::as_str).unwrap_or("command")
            )
        } else {
            err
        })
    }
}

fn parse_status(raw: &str) -> Vec<FileStatus> {
    raw.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (status, path) = l.split_at(2);
            FileStatus {
                status: status.trim().to_string(),
                path: path.trim_start().to_string(),
            }
        })
        .collect()
}

fn open_repo_sync(path: String) -> Result<RepoInfo, String> {
    let p = PathBuf::from(&path);
    if !p.is_dir() {
        return Err(format!("Not a directory: {path}"));
    }
    let inside = run_git(&path, &[String::from("rev-parse"), String::from("--is-inside-work-tree")])?;
    if inside.trim() != "true" {
        return Err(format!("{path} is not a git repository"));
    }

    let branch = run_git(&path, &[String::from("branch"), String::from("--show-current")])
        .ok()
        .filter(|b| !b.is_empty())
        .unwrap_or_else(|| String::from("detached"));

    let statuses = parse_status(&run_git(
        &path,
        &[String::from("status"), String::from("--porcelain")],
    )?);
    let is_clean = statuses.is_empty();

    Ok(RepoInfo {
        branch,
        statuses,
        is_clean,
    })
}

#[tauri::command]
pub async fn open_repo(path: String) -> Result<RepoInfo, String> {
    tauri::async_runtime::spawn_blocking(move || open_repo_sync(path))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_status(path: String) -> Result<Vec<FileStatus>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let raw = run_git(&path, &[String::from("status"), String::from("--porcelain")])?;
        Ok(parse_status(&raw))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_diff(path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let diff = run_git(
            &path,
            &[String::from("diff"), String::from("HEAD")],
        )?;
        let capped: String = diff.chars().take(12_000).collect();
        Ok(capped)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn stage_all(path: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        run_git(&path, &[String::from("add"), String::from("-A")])?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn commit(path: String, message: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let out = run_git(&path, &[String::from("commit"), String::from("-m"), message])?;
        Ok(out)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn push(path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let out = run_git(&path, &[String::from("push")])?;
        Ok(out)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn reset_soft(path: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        run_git(
            &path,
            &[
                String::from("reset"),
                String::from("--soft"),
                String::from("HEAD~1"),
            ],
        )?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
