#[derive(Debug)]
pub struct UF {
    id: Vec<usize>,
    count: usize,
}

impl UF {
    pub fn new(n: usize) -> UF {
        UF {
            id: (0..n).collect::<Vec<usize>>(),
            count: n,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn find(&self, p: usize) -> usize {
        self.id[p]
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let p_id = self.find(p);
        let q_id = self.find(q);
        if p_id == q_id {
            return;
        }
        for i in 0..self.id.len() {
            if self.id[i] == p_id {
                self.id[i] = q_id;
            }
        }
        self.count -= 1;
    }
}
