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
use std::borrow::Borrow;
use itertools::Itertools;
use std::hash::Hash;

/*
mod hashed_stack;
use hashed_stack::HashedStack;
*/

type Pair<'a> = (&'a str, &'a str);

#[derive(PartialEq, Eq, Debug)]
struct Element<T>( T, isize);

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

fn find_all_names(string: &str) -> HashSet<&str> {
    let re = Regex::new(r"([A-Z][a-z]*)[\s-]([A-Z][a-z]*)").unwrap();
    re.find_iter(string).map(|mat| mat.as_str()).collect()
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
                *histogram.entry(i- stack[0].1 as usize)
                    .or_insert(0) += 1;
                *histogram.entry(i- stack[1].1 as usize)
                    .or_insert(0) -= 1;
            }

            match stack.iter().position(|x| x.0 == current) {
                Some(index) => {
                    let mut removed = stack.remove(index);
                    removed.1 = (i + 1) as isize;
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

// Counts of cooccurence for each window length to csv
fn to_file(counts: &HashMap<usize, isize>, file_path: &str) -> File {
    let mut file = File::create(file_path).expect("No file created");
    let mut out: String = String::new();

    for i in 1..574819 {
        fmt::write(&mut out,
                   format_args!("{}, {}\n", i, counts[&i]))
            .expect("No file");
    }

    file.write(out.as_ref()).expect("No file found");
    file
}

// Given counts, outputs percentage of cooccurrence in all window lengths to csv
fn percent_to_file(counts: &HashMap<usize, isize>, trace_len: usize, file_path: &str) -> File {
    let mut file = File::create(file_path).expect("No file created");
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

// Calculate conditional cooccurence for all pairs
fn pair_cooccurrence(characters: &'static str, trace: &Vec<String>) {
    let mut pairs: Vec<Pair> = get_pairs(characters);
    let mut li: Vec<Element<Pair>> = Vec::new();

    for pair in pairs {
        let window_length = get_min_window_length(pair.0, pair.1 , trace);
        println!("{:?} {}", pair, window_length);
        li.push(Element(pair, window_length as isize));
    }
    li.sort_by_key(|k| k.1);


    let mut file = File::create("pair_rankings").expect("No file created");
    let mut out: String = String::new();

    for i in 0.. li.len() {
        fmt::write(&mut out,
                   format_args!("{:?}: {}\n", li[i].0, li[i].1))
            .expect("No file");
    }

    file.write(out.as_ref()).expect("No file found");
}

fn get_min_window_length(first: &'static str, second: &'static str, trace: &Vec<String>) -> usize{
    let mut a: HashSet<&'static str> = HashSet::new();
    a.insert(first);
    let mut b: HashSet<&'static str> = HashSet::new();
    b.insert(second);

    let trace_len = trace.len();
    let joint: HashSet<&str> = a.union(&b).cloned().collect();

    let histA = get_histogram(&trace, a);
    let hist_joint = get_histogram(&trace, joint);

    let coocA = count_cooccurrence(histA, &trace_len);
    let cooc_joint = count_cooccurrence(hist_joint, &trace_len);

    let mut min_length = usize::max_value();

    for i in 1..trace.len() {
        let prob = cooc_joint[&i] as f64 / coocA[&i] as f64;
        if prob > 0.9 {
            min_length = i;
            break
        }
    }

    min_length
}

// Given a list of items, returns all pairs
fn get_pairs(characters: &'static str) -> Vec<Pair<'static>> {
    let char_list: Vec<&'static str> = characters.split("\r\n").collect();
    let mut pairs: Vec<Pair<'static>> = Vec::new();

    for perm in char_list.into_iter().permutations(2) {
        let curr: Pair = (&perm[0], &perm[1]);
        pairs.push(curr);
    }

    pairs
}

// Reads a csv and puts it into a vector of vector, where each index represents a given time
fn read_csv() {

}

fn multiple_event_cooccurrence<T: Eq + Hash + Clone + std::fmt::Debug>
    (I: HashSet<T>, trace: Vec<Vec<T>>, ) {
    let trace_len = trace.len();

    // Put csv into a vector which has time indices
    // Calculate histogram for a given itemset
    let hist = histogram_timestamped(I,trace);

    // Count co-occurrence on the trace for every window length
    let counts = count_cooccurrence(hist, &trace_len);
    // Write co-occurrence to file
}

fn histogram_timestamped<T: Eq + Hash + Clone + std::fmt::Debug>
    (word_set: HashSet<T>, trace: Vec<Vec<T>>) -> HashMap<usize, isize> {

    let mut histogram: HashMap<usize, isize> = HashMap::new();

    let n = trace.len();

    // Each element of the stack is a set of events and a tesla
    let mut stack:Vec<Element<HashSet<T>>> = Vec::new();

    // Add all elements of I to stack
    stack.push(Element(word_set.clone(),-1));

    // Iterate through the trace
    for t in 0..n {
        println!("{} {:?}", t, stack);
        // Let X be the set of events that occur at time = t
        let curr_element = &trace[t];
        let X:HashSet<T> =curr_element.iter().cloned().collect();

        // We are interested in the intersection of I and X, called X'
        let Xprime:HashSet<_> = X.intersection(&word_set).cloned().collect();

        if !Xprime.is_empty() {
            // Update if the intersection of bottom of stack with Xprime is non-empty
            let bottom = &stack[0];
            let Y = &bottom.0;

            if !Xprime.intersection(Y).cloned().collect::<HashSet<T>>().is_empty() {
                update_histogram(&stack, &Xprime, &Y, &word_set, &mut histogram, t);
            }

            move_set_to_top(&mut stack, Xprime, t)
        }
    }

    // Update histogram for last gap which is an odd subset so plus one
    *histogram.entry(trace.len() - stack[0].1 as usize - 1).or_insert(0) += 1;

    println!("{:?}", &histogram);

    histogram
}

fn move_set_to_top<T: Eq + Hash + Clone + std::fmt::Debug>
    (stack: &mut Vec<Element<HashSet<T>>>, X: HashSet<T>, time: usize) {
    // Remove all elements in Xprime from stack
    let mut U = X.clone();
    let mut to_remove: Vec<usize> = Vec::new();
    for i in 0..stack.len() {
        let element = &mut stack[i];
        let C = &element.0;
        let R: HashSet<_> = C.intersection(&X).cloned().collect();

        if !R.is_empty() {
            // Two cases
            if &R == C {
                // Must remove the entire element
                to_remove.push(i)
            }
            else if R.is_subset(C) {
                // Updates last seen time of certain events while leaving others
                element.0 = C.difference(&R).cloned().collect();
            }

            U = U.difference(&R).cloned().collect();
            if U.is_empty() {break}
        }
    }

    for index in to_remove.into_iter().rev() {
        stack.remove(index);
    }

    // Add new element on top
    let new: Element<HashSet<_>> = Element(X, time as isize);
    stack.push(new);
}

fn update_histogram<T: Eq + Hash + Clone + std::fmt::Debug>
    (stack: &Vec<Element<HashSet<T>>>, Xprime: &HashSet<T>, Y: &HashSet<T>,
     I: &HashSet<T>, hist: & mut HashMap<usize, isize>, time: usize) {

    println!("X {:?} Y {:?}", Xprime, Y);
    /*
     If Y is a subset of Xprime we only need to update if the cardinality of Y is 1.
     In a case where we do update we must update the gap for the next element in the stack which
     was not seen in I
     */

    if !(Y.is_subset(Xprime) && Y.len() != 1) {
        let k0 = (time as isize - stack[0].1 - 1) as usize; // Gap from bottom of the stack
        *hist.entry(k0).or_insert(0) += 1;

        for element in stack.iter() {
            if !element.0.is_subset(Xprime) {
                let kn = (time as isize - element.1 - 1) as usize;
                *hist.entry(kn).or_insert(0) -= 1;
                break;
            }
        }

        println!("Histogram: {:?}", hist)

    }

}

fn main() {

    /*
        Example stream
        1, a,
        2, x,
        3, x,
        4, b, x,
        5, c, x,
        6, x,
        7, d, a
        8, x,
        10,
        9, a, b,
        10, c,
        11, a,
     */

    let ex_stream = vec![vec!['a'],vec!['x'],vec!['x'],vec!['b','x'],vec!['c','x'],vec!['x'],vec!['d','a'],vec!['x'],vec![],vec!['a','b'],vec!['c'],vec!['d']];
    let filename = "traces/ex_stream_timestamped.csv";
    let I = ['a','b'].iter().cloned().collect();
    multiple_event_cooccurrence(I, ex_stream);

}