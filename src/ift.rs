use ndarray::prelude::*;

trait IFT
{
    // initializes queue
    fn init(&self) -> ();

    // resets auxiliary data structures
    fn reset(&self) -> ();

    // updates auxiliary infomation with recently popped node
    fn update_tree(&self, p: usize) -> ();

    // try to conquer node q from node p
    fn conquer(&self, p: usize, q: usize) -> ();

    // runs the image forest transform
    fn ift(&self) -> ()
    {
        self.reset();
        self.init();

        while !self.queue.is_empty()
        {
            let p = self.queue.pop();
            if self.roots[p] == p {
                self.costs[p] = 0;
            }

            self.update_tree(p);

            // or self.shape.neighbors
            // self.something_grid.neighbors(p)
            for q in self.neighbors(p)
            {
                self.conquer(p, q);
            }
        }
    }
}

