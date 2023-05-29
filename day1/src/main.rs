
fn main(){

    let mut totals: Vec<u64> = Vec::new();
    //group lines    
    for group in include_str!("input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
    {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>().unwrap();
            sum += value;
        }
        totals.push(sum);
    }

    totals.sort();
    totals.reverse();
    let max = totals[0];

    let maxes = totals[0..3].to_vec();
    let mut sum = 0;
    for val in maxes{
        sum += val;
    }
    println!("max value: {max}");
    println!("max value of three groups added: {sum}");
}
