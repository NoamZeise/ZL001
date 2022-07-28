#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct McConnection {
    mc_i : usize,
    io_i : usize,
}


impl McConnection {
    fn new(mc_i : usize, io_i : usize) -> Self {
        McConnection { mc_i, io_i }
    }

    fn get_mc_i(&self) -> usize {
        self.mc_i
    }
    
    fn get_io_i(&self) -> usize {
        self.io_i
    }
}
