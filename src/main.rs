mod repo;
use repo::Repo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting repos...");

    // Get repos from github api
    let res = reqwest::blocking::Client::new()
        .get("https://api.github.com/users/Dot32Dev/repos?per_page=100")
        .header("User-Agent", "repo_list") // Required by github api
        .send()?;
        
    // Parse response into a vector of repos
    let text = res.text()?;
    let mut repos: Vec<Repo> = serde_json::from_str(&text)?;

    // Sort repos by stargazer count
    repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));

    // Total stars
    let mut total_stars = 0;

    // Hidden repos
    let mut hidden_repos = repos.len();

    // Print repos
    for repo in repos.iter().filter(|&repo| repo.stargazers_count > 0) {
        println!("{} Stars: {}", repo.stargazers_count, repo.name);
        total_stars += repo.stargazers_count;
        hidden_repos -= 1;
    }

    println!("Total Stars: {}", total_stars);
    println!("There are {} repos with 0 stars", hidden_repos);
    Ok(())
}