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

    pub fn given(v: Vec<usize>) -> UF {
        UF {
            count: v.len(),
            id: v,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn quick_find(&self, p: usize) -> usize {
        self.id[p]
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.quick_find(p) == self.quick_find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let p_id = self.quick_find(p);
        let q_id = self.quick_find(q);
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

    pub fn find(&self, mut p: usize) -> usize {
        while p != self.id[p] {
            p = self.id[p];
        }
        p
    }

    pub fn quick_union(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);
        if p_root == q_root {
            return;
        }
        self.id[p_root] = q_root;
        self.count -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uf_count_works() {
        let uf = UF::new(2);
        assert_eq!(uf.count(), 2);
    }

    #[test]
    fn uf_quick_find_works() {
        let uf = UF::new(2);
        assert_eq!(uf.quick_find(1), 1);
    }

    #[test]
    fn uf_connected_works() {
        let mut uf = UF::new(3);
        uf.union(0, 2);
        assert_eq!(uf.connected(0, 2), true);
        assert_eq!(uf.connected(0, 1), false);
    }

    #[test]
    fn uf_union_works() {
        let mut uf = UF::new(3);
        uf.union(0, 1);
        uf.union(1, 2);
        assert_eq!(uf.connected(0, 1), true);
        assert_eq!(uf.connected(1, 2), true);
        assert_eq!(uf.connected(0, 2), true);
    }

    #[test]
    fn uf_find_works() {
        let uf = UF::given(vec![1, 1, 1, 8, 3, 0, 5, 1, 8, 8]);
        assert_eq!(uf.find(5), 1);
        assert_eq!(uf.find(9), 8);
    }

    #[test]
    fn uf_quick_union_works() {
        let mut uf = UF::given(vec![1, 1, 1, 8, 3, 0, 5, 1, 8, 8]);
        uf.quick_union(5, 9);
        assert_eq!(uf.id, vec![1, 8, 1, 8, 3, 0, 5, 1, 8, 8]);
    }
}
