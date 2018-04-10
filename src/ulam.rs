pub type UlamCoord = (isize, isize);

#[derive(Default)]
pub struct Ulam {
    ring_index: usize,
    in_ring_index: usize,
    ring_diameter: usize,
    cursor: UlamCoord,
}

impl Ulam {
    pub fn new() -> Self { Default::default() }

    fn at_ring_end(&self) -> bool {
        self.cursor.0 == self.ring_index as isize &&
        self.cursor.1 == self.ring_index as isize * -1
    }

    fn edge_end(&self, edge: usize) -> usize {
        match edge {
            0 => (1 * self.ring_diameter) - 2, 
            1 => (2 * self.ring_diameter) - 3,
            2 => (3 * self.ring_diameter) - 4,
            _ => unreachable!(),
        }
    }
}

impl Iterator for Ulam {
    type Item = UlamCoord;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.cursor;

        if self.at_ring_end() {
            self.cursor.0 += 1;
            self.in_ring_index = 0;
            self.ring_index += 1;
            self.ring_diameter = (self.ring_index * 2) + 1;
        } else {
            match self.in_ring_index {
                x if x < self.edge_end(0) => self.cursor.1 += 1,
                x if x < self.edge_end(1) => self.cursor.0 -= 1,
                x if x < self.edge_end(2) => self.cursor.1 -= 1,
                _                         => self.cursor.0 += 1,
            }
            self.in_ring_index += 1;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const REFERENCE: [(isize, isize); 25] = [
        ( 0,  0),

        ( 1,  0),
        ( 1,  1),
        ( 0,  1),
        (-1,  1),
        (-1,  0),
        (-1, -1),
        ( 0, -1),
        ( 1, -1),

        ( 2, -1),
        ( 2,  0),
        ( 2,  1),
        ( 2,  2),
        ( 1,  2),
        ( 0,  2),
        (-1,  2),
        (-2,  2),
        (-2,  1),
        (-2,  0),
        (-2, -1),
        (-2, -2),
        (-1, -2),
        ( 0, -2),
        ( 1, -2),
        ( 2, -2),
    ];

    #[test]
    fn twenty_five() {
        let mut refr = REFERENCE.iter().cloned();
        let mut ulam = Ulam::new().take(25);
        for _ in 0..REFERENCE.len() {
            assert_eq!(refr.next(), ulam.next());
        }
        assert_eq!(None, ulam.next());
    }

    #[test]
    fn infinite() {
        let count = 10000;
        assert_eq!(count, Ulam::new().take(count).count());
    }
}
