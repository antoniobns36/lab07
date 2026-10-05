use rand::RngExt;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    println!("--- Initial Highs ---");
    for (i, &temp) in highs.iter().enumerate() {
        println!("{}: {}", DAYS[i], temp);
    }
     let mut rng = rand::rng();
    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));

     println!("\n--- Updated Highs ---");
    for (i, &temp) in highs.iter().enumerate() {
        println!("{}: {}", DAYS[i], temp);
    }

    let avg = average_temp(&highs);
    let hot_idx = hottest_day(&highs);
    let count = count_above(&highs, 75);

    println!("\n--- Analysis ---");
    println!("Average Temperature: {:.2}", avg);
    println!("Hottest Day: {}", DAYS[hot_idx]);
    println!("Days Above 75: {}", count);
}

    fn average_temp(log: &Vec<i32>) -> f64 {
	if log.is_empty() {
	    return 0.0;
	}
	let sum: i32 = log.iter().sum();
	sum as f64 / log.len() as f64
     }

     fn hottest_day(log: &Vec<i32>) -> usize {
    log.iter()
        .enumerate()
        .max_by_key(|&(_, &temp)| temp)
        .map(|(idx, _)| idx)
        .unwrap_or(0)
      }


