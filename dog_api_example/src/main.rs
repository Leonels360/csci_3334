/*
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);
            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }

    Ok(())
}
*/
use serde::Deserialize;
use std::error::Error;
use std::fmt::{self, Display};
use std::io;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum DogFetcherError {
    NetworkError(String),
    ApiError(String),
    ParseError(String),
    IoError(io::Error),
}

impl Display for DogFetcherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DogFetcherError::NetworkError(e) => write!(f, "Network connectivity error: {}", e),
            DogFetcherError::ApiError(e) => write!(f, "API or HTTP error: {}", e),
            DogFetcherError::ParseError(e) => write!(f, "Response parsing error: {}", e),
            DogFetcherError::IoError(e) => write!(f, "File system I/O error: {}", e),
        }
    }
}

impl Error for DogFetcherError {}

impl From<ureq::Error> for DogFetcherError {
    fn from(err: ureq::Error) -> Self {
        DogFetcherError::NetworkError(err.to_string())
    }
}

impl From<io::Error> for DogFetcherError {
    fn from(err: io::Error) -> Self {
        DogFetcherError::IoError(err)
    }
}

fn fetch_random_dog_image(filename: &str) -> Result<(), DogFetcherError> {
    let api_url = "https://dog.ceo/api/breeds/image/random";
    
    let dog_image: DogImage = {
        let response = ureq::get(api_url).call()?;

        if response.status() == 200 {
            match response.into_json::<DogImage>() {
                Ok(img) => img,
                Err(e) => return Err(DogFetcherError::ParseError(format!("Failed to parse JSON: {}", e))),
            }
        } else {
            return Err(DogFetcherError::ApiError(format!("API responded with HTTP status: {}", response.status())));
        }
    };
    
    println!("🖼️ Image URL: {}", dog_image.message);
    println!("💾 Target Filename: {}", filename);

    let image_url = dog_image.message.as_str();

    let mut response = ureq::get(image_url).call()?;

    if response.status() != 200 {
        return Err(DogFetcherError::ApiError(format!("Image download failed with HTTP status: {}", response.status())));
    }

    let mut dest = std::fs::File::create(filename)?;

    io::copy(&mut response.into_reader(), &mut dest)?;

    Ok(())
}

fn main() -> Result<(), DogFetcherError> {
    println!("Dog Image Downloader");
    println!("===================\n");

    for i in 1..=5 {
        let filename = format!("dog_image_{}.jpg", i);
        println!("--- Fetching and downloading image #{} ---", i);

        match fetch_random_dog_image(&filename) {
            Ok(_) => {
                println!("✅ Success! Image downloaded and saved as {}.\n", filename);
            },
            Err(e) => {
                println!("❌ Operation Failed: {}\n", e);
            }
        }
    }

    Ok(())
}