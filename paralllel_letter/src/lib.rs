use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;



pub fn countL(text: &str) -> HashMap<char, usize> {
    let mut myletters: HashMap<char, usize> = HashMap::new();
    for index in text.to_lowercase().chars(){
        if index.is_alphabetic()
        {
            let counter = myletters.entry(index).or_insert(0);
            *counter +=1;
        }
    }
    myletters
}



pub fn countLetters(text: &'static [&str]) -> Result<Vec<(char, usize)>, String>{
    if text.is_empty() {
        return Err("There is no text".to_string());
    }
	// determine lines number in order to determine thread numbers
    let lines_number = text.len();
	// store results of counting letters frequency in hashmap
    let mut myletters: HashMap<char, usize> = HashMap::new();
    let (tx, rx) = mpsc::channel();

    for index in 0..lines_number{
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let mut letters = countL(&text[index]);
            thread_tx.send(letters).unwrap();
           // thread::sleep(Duration::from_secs(1));
            });

        let mut received = rx.recv().unwrap();
		// compare recently computed hashmap with final one
		// updated values in final one if there are any in recently computed  
        for (k,v) in received.iter(){
            if myletters.get(k).is_some()
            {
                *myletters.get_mut(k).unwrap() += *v;

            }
		// if there is no key in final hashmap, add it to it
            else {
                myletters.insert(*k, *v);
            }
        };


    };
	//sort final hashmap
    let mut sorted_myletters: Vec<_> = myletters.into_iter().collect();
    sorted_myletters.sort_by(|x,y| x.0.cmp(&y.0));
    Ok(sorted_myletters)
}

