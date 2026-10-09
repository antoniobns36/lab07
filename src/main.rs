use rand::RngExt;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

struct Reading {
    day: String,
    high: i32,
}

struct Summary {
    average: f64,
    hottest_day: String,
    days_above: usize,
}

fn main() {
    let mut temp_ ighs: Vec<i32> = vec![72, 68, 75, 81, 79];

    let mut rng = rand::rng();
    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));

    let mut log: Vec<Reading> = Vec::new();
    for i in 0..DAYS.len() {
        log.push(Reading {
            days: DAYS[i].to_string(),
	    high: temp_highs[i],
        });
    }

    println!("\n--- Updated Highs ---");
    for reading in &log  {
        println!("{}: {}", reading.day, reading.high);
    }

    let summary = summarize(&log, 75);

    println!("\n--- Analysis Summary ---");
    println!("Average Temperature: {:.2}", summary.average);
    println!("Hottest Day: {}", summary.hottest_day);
    println!("Days Above 75: {}", summary.days_above);
}

    fn average_temp(log: &Vec<Reading>) -> f64 {
	if log.is_empty() {
	    return 0.0;
	}
	let sum: i32 = log.iter().map(|r| r.high).sum();
	sum as f64 / log.len() as f64
     }

     fn hottest_day(log: &Vec<Reading>) -> usize {
    log.iter()
        .enumerate()
        .max_by_key(|&(_, r)| r.high)
        .map(|(idx, _)| idx)
        .unwrap_or(0)
     }
     
     fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
    log.iter().filter(|r| r.high  > threshold).count()
    }

    fn summarize(log: &Vec<Reading>, threshold: i32) -> Summarty {
        Summary {
	    average: average_temp(log),
	    hottest_day: log[hottest_day(log)].day.clone(),
  	    days_above: count_above(log, threshold),
        } 
}
