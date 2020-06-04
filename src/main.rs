use std::fs;
use std::fmt;
use std::iter;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use rand::Rng;
use rand::seq::SliceRandom;

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

// Splits text
fn split_string(string: &str) -> Vec<String>{
    let re = Regex::new(r"[^A-za-z0-9 ]").unwrap();
    let mut result = String::from(re.replace_all(&string, ""));
    result.make_ascii_lowercase();

    let text: Vec<String> = result.split(" ").map(|s| s.to_string()).collect();
    text.into_iter().filter(|i| i != "").collect::<Vec<String>>()
}

// Histogram for one word
fn get_freq_hist (trace: &Vec<String>, word: &'static str) -> HashMap<usize, isize> {
    let mut freq:HashMap<usize,isize> = HashMap::new();
    let mut last_access: usize = 0;

    for i in 0.. trace.len() {
        if trace[i] == word {
            *freq.entry(i-last_access).or_insert(0) += 1;
            // println!("{}", &last_access);
            last_access = i+1;
        }
    }

    freq
}

// ALWCO
fn get_histogram(trace: &Vec<String>, word_set: HashSet<&'static str>) -> HashMap<usize, isize>{

    // Cases when counting the occurrence of one word
    if word_set.len() < 2 {
        let mut word = "";
        for x in word_set.iter() {
           word = *x;
        }
        return get_freq_hist(trace, word)
    }

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

// Probability that A and B cooccur given A occurs in a
// given window length calculated for every window length

fn conditional_cooccurrence(trace: &Vec<String>, A: HashSet<&'static str>, B: HashSet<&'static str>)
    -> HashMap<usize, f64> {

    let trace_len = trace.len();
    let joint: HashSet<&'static str> = A.union(&B).cloned().collect();

    let histA = get_histogram(&trace, A);
    let hist_joint = get_histogram(&trace, joint);

    let coocA = count_cooccurrence(histA, &trace_len);
    let cooc_joint = count_cooccurrence(hist_joint, &trace_len);

    let mut cond_prob: HashMap<usize, f64> = HashMap::new();

    for i in 1..500 {
        let prob = cooc_joint[&i] as f64 / coocA[&i] as f64;
        cond_prob.insert(i, prob);
        println!("{} {}", i, prob)
    }

    cond_prob
}


fn count_cooccurrence(hist: HashMap<usize, isize>, trace_len: &usize) -> HashMap<usize, isize> {

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

fn to_file(counts: &HashMap<usize, isize>, file_path: &str) -> File {
    let mut file = File::create(file_path).expect("No file created");
    let mut out: String = String::new();

    for i in 1..574819 {
        fmt::write(&mut out,
                   format_args!("{} {}\n", i, counts[&i]))
            .expect("No file");
    }

    file.write(out.as_ref()).expect("No file found");
    file
}

fn percent_to_file(counts: &HashMap<usize, isize>, trace_len: usize) -> File {
    let mut file = File::create("shuffled_percents.csv").expect("No file created");
    let mut out: String = String::new();
    fmt::write(&mut out, format_args!("Window Length,Co-occurrence\n"));

    for i in 1..trace_len {
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

    let mut rng = rand::thread_rng();

    let trace = split_string(&contents);

    /*
    let mut shuffled_trace = trace.clone();
    shuffled_trace.shuffle(&mut rng);
    // println!("{:#?}", &shuffled_trace);

    let trace_len = &trace.len();

    let word_set:HashSet<&'static str> =
        vec!["sherlock", "holmes", "watson"]
            .iter().cloned().collect();

    let i1 = rng.gen_range(0,trace_len-1);
    let i2 = rng.gen_range(0,trace_len-1);
    let i3 = rng.gen_range(0,trace_len-1);

    let str1 = &trace[i1];
    let str2 = &trace[i2];
    let str3 = &trace[i3];

    let random_set:HashSet<&'static str> =
        vec!["confess", "about", "is"]
            .iter().cloned().collect();

    println!("{:#?}", random_set);

    let hist = get_histogram(&shuffled_trace, word_set);
    let cooccurrence = count_cooccurrence(hist, trace_len);
    // to_file(&cooccurrence);
    percent_to_file(&cooccurrence, *trace_len);
    */

    let A:HashSet<&'static str> = vec!["confess"].iter().cloned().collect();
    let B = vec!["confess", "sherlock"].iter().cloned().collect();

    conditional_cooccurrence(&trace,A, B);

}