use common::RepoInfo;

fn main() {
    match reqwest::blocking::get("http://localhost:8080/repo") {
        Ok(response) => {
            if let Ok(info) = response.json::<RepoInfo>() {
                println!("Repository: {}", info.name);
                println!("Primary Language: {:#?}", info.primary_language);
                if let Some(desc) = info.description {
                    println!("Description: {}", desc);
                }
                if let Some(lang) = info.primary_language {
                    println!("Language: {}", lang);
                }
                return;
            }
        }
        Err(e) => println!("Could not fetch repo info: {}", e),
    }
    
    println!("Web service not available");
}