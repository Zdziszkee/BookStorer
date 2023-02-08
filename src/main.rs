use std::io;
use rand::Rng;


fn main() {
    let mut rng = rand::thread_rng();


    let mut tab = [1, rng.gen::<i32>(), 1, 1, 1, 1, 1, 1, 1, 1];
    //tab.shuffle(&mut rng);
    print_tab(&tab);
    println!("Sort\n");

    print_tab(&tab);

    /*for element in tab{
        println!("{element}");
    }*/

    /*
        for (i) in tab.iter() {
            let x = i.to_u8().unwrap();
            numbers[i] = x + numbers[i];
        }
     */


    /*
    let mut string = String::new();
    string = String::from("Jebac zydow");
    string.push('a');

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    */

    //println!("{string}");
}

fn print_tab(arr: &[i32]) {
    for element in arr {
        println!("{element}");
    }
}

fn buble_sort() {
    loop {
        break;
    }
}

