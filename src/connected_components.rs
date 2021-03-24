struct DisjointSets {
    buf: Vec<i32>
}

impl DisjointSets {
    fn new(size: usize) -> Self {
        let mut dj_set = DisjointSets {
            buf: Vec::with_capacity(size)
        };
        for _i in 0..size {
            dj_set.buf.push(-1);
        }
        dj_set
    }

    fn merge(&mut self, x: i32, y: i32) {
        self.buf[x] = y;
    }

    fn find(&mut self, x: i32) -> i32 {
        let mut path = vec![];
        let mut idx = x;
        while self.buf[idx] != -1 {
            path.push(idx);
            idx = self.buf[idx];
        }
        for y in path {
            self.buf[y] = idx;
        }
        idx
    }
}

fn connected_components(edges: &[[usize]]) -> DisjointSets {
    let n_nodes = edges.len();
    let mut dj_set = DisjointSets::new(n_nodes);
    for i in 0..n_nodes {
        for j in 0..n_nodes {
            if edges[i][j] == 1 {
                dj_set.merge(i as i32, j as i32);
            }
        }
    }
    dj_set
}
