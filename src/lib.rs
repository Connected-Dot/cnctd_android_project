use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use colored::*;
use regex::Regex;

pub struct AndroidProject;

impl AndroidProject {
    pub fn bump_version(version_part: &str) -> Result<(), anyhow::Error> {
        // Open the build.gradle file
        let path = "app/build.gradle";
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
    
        // Read the file line by line and modify versionName where necessary
        let mut output = String::new();
        let mut version_name_found = false;
        let mut major_version;
        let mut minor_version;
        let mut patch_version;
        for line in reader.lines() {
            let line = line?;
            if line.contains("versionName") {
                let line = line.trim_end_matches('\n');
                let version_parts = line.split('=').collect::<Vec<&str>>();
                let version_prefix = version_parts[0].trim_end_matches(' ').to_string();
                let version_value = version_parts[1]
                    .trim_start_matches(' ')
                    .trim_start_matches('"')
                    .trim_end_matches('"')
                    .to_string();
    
                if !version_name_found {
                    version_name_found = true;
    
                    // Split the version string into major, minor, and patch parts
                    let version_parts = version_value.split('.').collect::<Vec<&str>>();
                    if version_parts.len() < 3 {
                        return Err(anyhow::anyhow!("Unable to parse version"));
                    }
                    let major_part = version_parts[0];
                    let minor_part = version_parts[1];
                    let patch_part = version_parts[2];
                    major_version = Some(
                        major_part
                            .parse::<u32>()
                            .map_err(|_| anyhow::anyhow!("Unable to parse major version"))?,
                    );
                    minor_version = Some(
                        minor_part
                            .parse::<u32>()
                            .map_err(|_| anyhow::anyhow!("Unable to parse minor version"))?,
                    );
                    patch_version = Some(
                        patch_part
                            .parse::<u32>()
                            .map_err(|_| anyhow::anyhow!("Unable to parse patch version"))?,
                    );
    
                    // Determine the new version
                    let new_major_version = if version_part == "major" {
                        major_version.unwrap() + 1
                    } else {
                        major_version.unwrap()
                    };
                    let new_minor_version = if version_part == "major" || version_part == "minor" {
                        0
                    } else {
                        minor_version.unwrap() + 1
                    };
                    let new_patch_version = if version_part == "major" || version_part == "minor" {
                        0
                    } else {
                        patch_version.unwrap()
                    };
                    let new_version = format!(
                        "{} = \"{}.{}.{}\"",
                        version_prefix, new_major_version, new_minor_version, new_patch_version
                    );
                    output.push_str(&new_version);
                } else {
                    output.push_str(&line);
                }
                output.push_str("\n");
            } else {
                output.push_str(&line);
                output.push_str("\n");
            }
        }
    
        // Write the modified file back to disk
        println!("{}", "Updating Android app version".blue());
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)?;
    
        Ok(())
       
    }

    
    pub fn get_version(base_path: &str) -> Result<String, anyhow::Error> {
        // Open the build.gradle file
        let path = format!("{}/app/build.gradle", base_path);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
    
        // Define a regular expression to match the versionName attribute
        let re = Regex::new(r#"versionName\s+"([^"]+)""#)?;
    
        // Read the file line by line and search for the versionName attribute
        for line in reader.lines() {
            let line = line?;
            if let Some(captures) = re.captures(&line) {
                let version_value = captures.get(1).unwrap().as_str().to_string();
                return Ok(version_value);
            }
        }
    
        Err(anyhow::anyhow!("Unable to find versionName in build.gradle file"))
    }
    
}

