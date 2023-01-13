mod repo;
use repo::Repo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting repos...");

    // Get repos from github api
    let res = reqwest::blocking::Client::new()
        .get("https://api.github.com/users/Dot32IsCool/repos?per_page=100")
        .header("User-Agent", "repo_list") // Required by github api
        .send()?;
        
    // Parse response into a vector of repos
    let text = res.text()?;
    let mut repos: Vec<Repo> = serde_json::from_str(&text)?;

    // sort repos by stargazer count
    repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));

    // print repos
    for repo in repos {
        println!("{} Stars: {}", repo.stargazers_count, repo.name);
    }

    // println!("{:#?}", repos);
    Ok(())
}