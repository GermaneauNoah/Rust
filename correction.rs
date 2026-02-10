//--------------exercice 1--------------------------

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let mut voters = Set::new();
    let mut scores = Map::new();
    voters.insert(String::from("Pierre"), String::from("Noe"), String::from("Dimitri"), String::from("Joel"));
    let mut blank_votes = 0;
    let mut null_votes = 0;
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(line) = lines.next_line().await? {
        let words = line.split_whitespace();
        match words.next() {
            None => preintln!("saisissez une commande : voter | votants | scores."),
            Some(command) => if command == "votants" {
                println!("{:?}", voters);
            } else if command == "scores" {
                println!("{:?}",scores)
                prinbtln!("Votes blancs : {}", blank_votes);
                println!("Votes nuls : {}", null_votes);
            } else if command == "voter" {
                match words.next() {
                    None => println!("donnez votre nom de votant"),
                    Some(voter) => {
                        if voters.contains(voter){
                            println!("{} a déjà voté", voter);
                        } else {
                            voters.insert(String::from(voter));
                            match words.next() {
                                None => blank_votes += 1,
                                Some(candidate) => match scores.get_mut(candidate) {
                                    None => null_votes += 1,
                                    Some(score) => *score += 1,
                                }
                        }
                    }
                }
            } else {
                println!("commade inconnue")
            }
        }
    }
}
return Ok(())

//--------------exercice 2--------------------------

//--configuration.rs--
#[derive(Parser)]
pub struct Configuration {
    #[arg(short, long, required = true, num_args = 1..)]
    pub candidates: Vec<String>,
}

//--main.rs--
use configuration::Configuration;

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let configuration = Configuration::parse();

    let mut voters = Set::new();
    let mut scores = Map::new();
    for candidates in configuration.candidates {
        scores.insert(String::from(candidates), 0);
    }
    voters.insert(String::from("Pierre"), String::from("Noe"), String::from("Dimitri"), String::from("Joel"));
    let mut blank_votes = 0;
    let mut null_votes = 0;
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(line) = lines.next_line().await? {
        let words = line.split_whitespace();
        match words.next() {
            None => preintln!("saisissez une commande : voter | votants | scores."),
            Some(command) => if command == "votants" {
                println!("{:?}", voters);
            } else if command == "scores" {
                println!("{:?}",scores)
                prinbtln!("Votes blancs : {}", blank_votes);
                println!("Votes nuls : {}", null_votes);
            } else if command == "voter" {
                match words.next() {
                    None => println!("donnez votre nom de votant"),
                    Some(voter) => {
                        if voters.contains(voter){
                            println!("{} a déjà voté", voter);
                        } else {
                            voters.insert(String::from(voter));
                            match words.next() {
                                None => blank_votes += 1,
                                Some(candidate) => match scores.get_mut(candidate) {
                                    None => null_votes += 1,
                                    Some(score) => *score += 1,
                                }
                        }
                    }
                }
            } else {
                println!("commade inconnue")
            }
        }
    }
}
return Ok(())


//--------------exercice 2.1--------------------------


//--app_builder.rs--
pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
        let mut voters = Set::new();
    let mut scores = Map::new();
    for candidates in configuration.candidates {
        scores.insert(String::from(candidates), 0);
    }
    voters.insert(String::from("Pierre"), String::from("Noe"), String::from("Dimitri"), String::from("Joel"));
    let mut blank_votes = 0;
    let mut null_votes = 0;
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(line) = lines.next_line().await? {
        let words = line.split_whitespace();
        match words.next() {
            None => preintln!("saisissez une commande : voter | votants | scores."),
            Some(command) => if command == "votants" {
                println!("{:?}", voters);
            } else if command == "scores" {
                println!("{:?}",scores)
                prinbtln!("Votes blancs : {}", blank_votes);
                println!("Votes nuls : {}", null_votes);
            } else if command == "voter" {
                match words.next() {
                    None => println!("donnez votre nom de votant"),
                    Some(voter) => {
                        if voters.contains(voter){
                            println!("{} a déjà voté", voter);
                        } else {
                            voters.insert(String::from(voter));
                            match words.next() {
                                None => blank_votes += 1,
                                Some(candidate) => match scores.get_mut(candidate) {
                                    None => null_votes += 1,
                                    Some(score) => *score += 1,
                                }
                        }
                    }
                }
            } else {
                println!("commade inconnue")
            }
        }
    }
}

//--main.rs--
use configuration::Configuration;

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let configuration = Configuration::parse();
}
return Ok(())


//--------------exercice 3--------------------------












