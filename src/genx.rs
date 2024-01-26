use rand::prelude::*;

pub fn main() {
    
    const PWIDTH: usize = 64;
    const PHEIGHT: usize = 256;

    const MAX_RNG_NUMBER: usize = 1000;

    const NUMBER_PROB: f32 = 0.10;
    const SYMBOL_PROB: f32 = 0.05;

    let mut buffer = String::new();
    for i in 1..PHEIGHT {

        buffer.clear();

        // let mut j: usize = 1;
        while buffer.len() <= PWIDTH {

            let v = rand::random::<f32>();

            match v {
                x if x < SYMBOL_PROB => {
                    buffer.push('A');
                },
                x if x < SYMBOL_PROB + NUMBER_PROB => {
                    let rng = rand::random::<usize>() % MAX_RNG_NUMBER;
                    
                    if buffer.ends_with(char::is_numeric) {
                        buffer.push('.');
                    }
                    buffer.push_str(rng.to_string().as_str());
                },
                _ => {
                    buffer.push('.');
                }
            }
        }

        buffer.truncate(PWIDTH);
        println!("{}", &buffer);
    }
}