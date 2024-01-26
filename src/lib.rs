
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Patch {
    row: usize,
    col: usize,
    len: usize,

    tick: usize
}

#[derive(Debug)]
struct Digit {
    location: Patch,
    value: u32,

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

#[derive(Debug)]
pub struct Engine {
    digits: Vec<Digit>,
    patches: Vec<Patch>,

    clock: usize,

    row: usize,

    rx: u32
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            digits: Vec::new(), patches: Vec::new(), clock: 0, row: 0, rx: 0
        }
    }

    pub fn signature(&self) -> (usize,usize,usize,u32) {
        (self.digits.len(), self.patches.len(), self.clock, self.rx)
    }
    
    pub fn run<'a, I>(lines: I) -> Engine where I: Iterator<Item = &'a String> {
        
        let mut eng = Engine {
            digits: Vec::new(), patches: Vec::new(), clock: 0, row: 0, rx: 0
        };
        
        for lx in lines {
            eng.ack(lx);
            dbg!(&eng);
        }
        
        eng
    }
    
    pub fn ack(&mut self, lx: &String) {
        
        // let lx = String::from("A....123...567..A890A..");
        
        let mut col = 0usize;
        
        self.row = self.row + 1;
        
        let chars: Vec<char> = lx.chars().into_iter().collect();
        
        let mut k = 0usize;
        
        while k < lx.len() {
            
            col = col + 1;
            
            //  detects a digit; store it with current clock
            if char::is_ascii_digit(&chars[k]) {
                //  value
                let mut vx = chars[k].to_string();
                //  location
                let mut cloc = Patch { row: self.row, col, len: 1, tick: self.clock };
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
                if let Ok(v) = vx.parse::<u32>() {
                    let dx = Digit { location: cloc, value: v, tick: self.clock, available: true };
                    self.digits.push(dx);
                }
            }
            
            //  detects a symbol
            if k < lx.len() && !char::is_ascii_digit(&chars[k]) && (chars[k] != '.') {
                //  patches
                //  construct patches of neighbours and store them with current clock
                self.patches.push(Patch { row: self.row - 1, col: col - 1, len: 3, tick: self.clock });
                self.patches.push(Patch { row: self.row, col: col - 1, len: 3, tick: self.clock });
                self.patches.push(Patch { row: self.row + 1, col: col - 1, len: 3, tick: self.clock });
            }
            
            k += 1;
        }
        
        //  ends line (for clock)
        
        //  match remaining digits ([clock - 1, clock]) and patches; remove older digits and patches
        for p in &self.patches {
            for d in &mut self.digits {
                if d.available && overlaps(d, &p) {
                    // dbg!(&d);
                    
                    self.rx = self.rx + d.value;
                    d.available = false;
                }
            }
        }
        
        self.clock = self.clock + 1;
        
        self.digits.retain(|d| (self.clock - d.tick <= 1) && (d.available));
        self.patches.retain(|p| self.clock - p.tick <= 1);
        
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

        assert_eq!(rx.signature(), (0, 0, 4, 0));
    }

    #[test]
    fn corner_patches() {
        let rx = Engine::run(vec![
            String::from("A11...11A"),
            String::from(".11...11."),
            String::from(".11...11."),
            String::from("A11...11A"),
        ].iter());

        assert_eq!(rx.signature(), (0, 6, 4, 88));
    }

    #[test]
    fn from_file() {

        let f = File::open("data/dtx32s001.txt").unwrap();

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
