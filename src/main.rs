use std::fs;
use std::fmt;
use std::iter;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/*
mod hashed_stack;
use hashed_stack::HashedStack;
*/

#[derive(Debug)]
struct Element<T>(T, usize);

impl fmt::Display for Element<&str> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn split_string(string: &str) -> Vec<String>{
    let re = Regex::new(r"[^A-za-z0-9 ]").unwrap();
    let mut result = String::from(re.replace_all(&string, ""));
    result.make_ascii_lowercase();

    let text: Vec<String> = result.split(" ").map(|s| s.to_string()).collect();
    text.into_iter().filter(|i| i != "").collect::<Vec<String>>()
}

// ALWCO
fn get_histogram(trace: Vec<String>, word_set: HashSet<&'static str>) -> HashMap<usize, isize>{
    let mut histogram: HashMap<usize, isize> = HashMap::new();

    let n = trace.len();
    // let I = word_set.len();
    // Empty stack of tuples
    let mut stack = Vec::new();
    // Add all elements to stack
    for item in word_set.iter() {
        stack.push(Element(*item,0));
    }

    // Iterate through the trace
        for i in 0..n-1 {
        let current:&str = trace[i].as_ref();

        if word_set.contains(current) {
            if stack[0].0 == current {
                *histogram.entry(i-stack[0].1)
                    .or_insert(0) += 1;
                *histogram.entry(i-stack[1].1)
                    .or_insert(0) -= 1;
            }

            match stack.iter().position(|x| x.0 == current) {
                Some(index) => {
                    let mut removed = stack.remove(index);
                    removed.1 = i + 1;
                    stack.push(removed);
                },
                None => println!("No value present")
            }
        }
    }
    // Update final gap
    *histogram.entry(trace.len() - stack[0].1 as usize).or_insert(0) += 1;
    histogram

    // Using hashed_stack
    // let stack: hashed_stack::HashedStack<hashed_stack::Element<&str>, String> = HashedStack::new(word_set);
    // println!("{}", stack);
}

fn count_cooccurrence(hist: HashMap<usize, isize>, trace_len: usize) -> HashMap<usize, isize> {

    let mut cooc: HashMap<usize, isize> = HashMap::new();
    let mut count_1: isize = 0;
    let mut count_2: isize = 0;

    for i in (1..trace_len+1).rev() {
        let curr: isize = match hist.get(&i) {
            None => 0,
            Some(x) => *x,
        };
        count_1 += curr;
        count_2 += curr*(i + 1) as isize;

        cooc.insert(i, ((trace_len-i + 1) as isize)
            - (count_2 - (i as isize * count_1)));

        //println!("{} {:#?}",i, cooc[&i])
    }

    cooc

}

fn to_file(counts: &HashMap<usize, isize>) -> File {
    let mut file = File::create("cooc.txt").expect("No file created");
    let mut out: String = String::new();

    for i in 1..500 {
        fmt::write(&mut out,
                   format_args!("{} {}\n", i, counts[&i]))
            .expect("No file");
    }

    file.write(out.as_ref()).expect("No file found");
    file
}

fn percent_to_file(counts: &HashMap<usize, isize>, trace_len: usize) -> File {
    let mut file = File::create("percentage.csv").expect("No file created");
    let mut out: String = String::new();
    fmt::write(&mut out, format_args!("Window Length,Co-occurrence\n"));

    for i in 1..500 {
        let percent:f64 = (counts[&i] as f64/ (trace_len - i + 1) as f64);
        fmt::write(&mut out,
                   format_args!("{},{:.7}\n", i, percent))
            .expect("No file");
    }

    file.write(out.as_ref()).expect("No file found");
    file
}


fn main() {
    let filename = "text/cnus.txt";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Error reading file");
    let trace = split_string(&contents);
    let trace_len = &trace.len();
    let word_set:HashSet<&'static str> =
        vec!["sherlock", "holmes", "watson"]
            .iter().cloned().collect();

    let hist = get_histogram(trace, word_set);
    let cooccurrence = count_cooccurrence(hist, *trace_len);
    to_file(&cooccurrence);
    percent_to_file(&cooccurrence, *trace_len);

}