use rug::{Complete, Integer};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//  A 'patch' covers a set of positions
//
#[derive(Debug, Clone, Copy)]
struct Patch {
    row: usize,
    col: usize,
    len: usize,

    tick: usize,

    gear: bool,
}

#[derive(Debug, Clone, Copy)]
struct Digit {
    location: Patch,
    value: usize,

    tick: usize,
    available: bool
}

fn overlaps(dx: &Digit, px: &Patch) -> bool {

    if dx.location.row != px.row { return false; }

    let dxa = dx.location.col;
    let dxb = dx.location.col + dx.location.len - 1;

    let pxa = px.col;
    let pxb = px.col + px.len - 1;

    if dxb >= pxa && dxb <= pxb { return true; }
    if dxa >= pxa && dxa <= pxb { return true; }

    false
}

fn overlaps_dilation(dx: &Digit, px: &Patch) -> bool {
    let mut candidate = px.clone();
    candidate.row = candidate.row - 1;

    if overlaps(dx, &candidate) { return true; }
    candidate.row += 1;
    if overlaps(dx, &candidate) { return true; }
    candidate.row += 1;
    if overlaps(dx, &candidate) { return true; }

    return false;
}

#[derive(Debug)]
pub struct Engine {
    digits: Vec<Digit>,
    patches: Vec<Patch>,

    clock: usize,

    row: usize,

    rx: Integer,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            digits: Vec::new(), patches: Vec::new(), clock: 0, row: 0, rx: Integer::from(0)
        }
    }

    pub fn signature(&self) -> (usize,usize,usize,&Integer) {
        (self.digits.len(), self.patches.len(), self.clock, &self.rx)
    }
    
    pub fn run<'a, I>(lines: I) -> Engine where I: Iterator<Item = &'a String> {
        
        let mut eng = Engine {
            digits: Vec::new(), patches: Vec::new(), clock: 0, row: 0, rx: Integer::from(0)
        };
        
        for lx in lines {
            eng.ack(lx);
            // dbg!(&eng);
        }
        
        eng
    }
    
    pub fn ack(&mut self, lx: &String) {
        
        // let lx = String::from("A....123...567..A890A..");
        
        let mut col = 0usize;
        
        self.row = self.row + 1;
        
        let chars: Vec<char> = lx.chars().into_iter().collect();
        
        let mut k = 0usize;
        
        println!(" tick at start {:?}", self.clock);
        println!(" {}", lx);
        
        while k < lx.len() {

            col = col + 1;
            
            //  detects a digit; store it with current clock
            if char::is_ascii_digit(&chars[k]) {
                //  value
                let mut vx = chars[k].to_string();
                //  location
                let mut cloc = Patch { row: self.row, col, len: 1, tick: self.clock, gear: false };
                //  rest
                k += 1; col += 1;
                
                while k < lx.len() && char::is_ascii_digit(&chars[k]) {
                    
                    //  others
                    if char::is_ascii_digit(&chars[k]) {
                        //  value
                        vx.push(chars[k]);
                        //  location
                        cloc.len = cloc.len + 1;
                        //  rest
                        k += 1; col += 1;
                    }
                }
                
                //  detected
                //
                if let Ok(v) = vx.parse::<usize>() {
                    let dx = Digit { location: cloc, value: v, tick: self.clock, available: true };
                    self.digits.push(dx);
                }
            }
            
            //  detects a symbol
            if k < lx.len() && (chars[k] == '*') {
                //  gears only
                self.patches.push(Patch { row: self.row, col: col - 1, len: 3, tick: self.clock, gear: true });
            }
            
            k += 1;
        }
        
        //  ends line (for clock)
        
        //  match remaining digits ([clock - 1, clock]) and patches; remove older digits and patches
        for p in &self.patches {
            // only when all possible digits have been detected
            if self.clock - p.tick == 1 {

                dbg!(&p);

                //  must be two for the gear to be considered
                let mut digits: Vec<Digit> = Vec::new();

                for d in &self.digits {
                    if overlaps_dilation(d, &p) {
                        println!("# digit overlaps gear: {:?}", d);
                        digits.push(*d);
                        // dbg!(&d);
                    }
                }

                match digits.len() {
                    2 => {
                        println!("# found a gear with two digits:");
                        dbg!(&digits);

                        self.rx += Integer::from(digits[0].value) * Integer::from(digits[1].value);
                    }
                    _ => {
                        println!("# found a gear but without exactly two digits");
                    }
                }
            }
        }
        
        self.digits.retain(|d| self.clock - d.tick <= 2);
        self.patches.retain(|p| self.clock - p.tick <= 2);
       
        println!("# remaining digits {:?}", self.digits);

        self.clock = self.clock + 1;
        
    }
}
    
    #[cfg(test)]
    mod tests {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::BufRead;
        
        use super::*;
        
    #[test]
    fn some_test() {
        let rx = Engine::run(vec![
            String::from("........."),
            String::from("........."),
            String::from("........."),
            String::from("........."),
        ].iter());

        assert_eq!(rx.signature(), (0, 0, 4, &Integer::from(0)));
    }

    #[test]
    fn some_big_test() {
        let m = usize::MAX;
        let p = format!("{}", m).replace(|c| char::is_ascii_digit(&c), ".");
        let rx = Engine::run(vec![
                             format!("...{}...", m),
                             format!("...*{}..", p),
                             format!("...{}...", m),
        ].iter());

        assert_eq!(rx.signature(), (2, 1, 3, &Integer::parse("340282366920938463426481119284349108225").unwrap().complete()));
    }

    #[test]
    fn corner_patches() {
        let rx = Engine::run(vec![
            String::from("A11...11A"),
            String::from(".11...11."),
            String::from(".11...11."),
            String::from("A11...11A"),
        ].iter());

        assert_eq!(rx.signature(), (0, 6, 4, &Integer::from(88)));
    }

    #[test]
    fn gears() {
        let rx = Engine::run(vec![
            String::from(".11*..11."),
            String::from(".11...11."),
            String::from(".11*2.11."),
            String::from(".11...11."),
        ].iter());

        assert_eq!(rx.signature(), (0, 6, 4, &Integer::from(22)));
    }

    #[test]
    fn sample() {

        let f = File::open("data/sample.txt").unwrap();

        let reader = BufReader::new(f);

        let lines = reader.lines();

        let mut eng = Engine::new();

        for line in lines {
            if let Ok(lx) = line { eng.ack(&lx); }
        }

        dbg!(&eng);
    }

    #[test]
    fn solve() {
        let f = File::open("data/input.txt").unwrap();

        let reader = BufReader::new(f);

        let lines = reader.lines();

        let mut eng = Engine::new();

        for line in lines {
            if let Ok(lx) = line { eng.ack(&lx); }
        }

        dbg!(&eng);
    }
}
