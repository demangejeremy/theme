mod lem;
mod token;
mod theme;

use std::time::{Instant};

fn main() {
    let now = Instant::now();

    // lem::hello();
    let mut result = token::stop("Je joue au tennis avec une balle dure.");
    result = lem::lematizer(&result);
    result = theme::detection(&result);

    println!("=========");
    if result != "" {
        println!("Thematique : {}", result);
    } else {
        println!("Thematique : none");
    }

    println!("(Execution time: {}ms)", now.elapsed().as_millis());
}