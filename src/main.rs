// A regex to match a valid token of time expression in the input is this.
// [0-9]+[ ]*[hms]+(inutes|econds|ours|our|econd|inute|ins|ecs)*
// This regex matches the following strings:
// 10m20s (splits up into 10m and 20s)
// 10minutes is allowed as well.
// 40 hours 30 minutes and 20 seconds is allowed too.

use regex::Regex;

// this function gets a line of input and returns it
fn get_line() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(n) => n,
        Err(error) => panic!("Error reading input: {}", error),
    };
    input
}
fn main() {
    'main_loop: loop {
        println!("How long should this timer go for? ");
        // get a line from input
        let input: String = get_line();
        // init number of seconds to sleep for
        let mut num_seconds = 0;

        // create a regex to match the input
        let token_re =
            match Regex::new("[0-9]+[ ]*[hms]+(inutes|econds|ours|our|econd|inute|ins|ecs)*") {
                Ok(re) => re,
                Err(error) => panic!("Error creating regex: {}", error),
            };

        // create a regex for each type of token.
        let hours_re = Regex::new("[0-9]+[ ]*[h]+(ours|our)*").unwrap();
        let mins_re = Regex::new("[0-9]+[ ]*[m]+(inutes|inute|ins)*").unwrap();
        let secs_re = Regex::new("[0-9]+[ ]*[s]+(econds|econd|ecs)*").unwrap(); // completely made up by Copilot. Insane

        // check for the case that none match
        if token_re.is_match(&input) == false {
            println!("Invalid input. Please try again.");
            continue 'main_loop;
        }

        // split the input into tokens
        for token_match in token_re.find_iter(&input) {
            // check if it refers to hours, minutes or seconds.
            // we'll use another regex to check for each case.
            let token_text = token_match.as_str();
            // then we have to get the number from the token
            // this is an incredibly 'best practices' way dreamt up by Copilot.
            // I was about to use another regex lol.
            let time = match token_text
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u32>()
            {
                Ok(n) => n,
                Err(error) => panic!("Error parsing number from token: {}", error),
            };
            if hours_re.is_match(token_text) {
                num_seconds += time * 60 * 60;
                continue;
            }
            if mins_re.is_match(token_text) {
                num_seconds += time * 60;
                continue;
            }
            if secs_re.is_match(token_text) {
                num_seconds += time;
                continue;
            }

            panic!("Invalid token: {}", token_text);
        }

        // to test, print the number of hours, minutes and seconds to sleep { COPILOT IS MAKING ME LAZY LOL}
        let hours = num_seconds / 60 / 60;
        let minutes = (num_seconds - hours * 60 * 60) / 60;
        let seconds = num_seconds - hours * 60 * 60 - minutes * 60;
        println!(
            "Sleeping for {} hours, {} minutes and {} seconds",
            hours, minutes, seconds
        );
    }
}
