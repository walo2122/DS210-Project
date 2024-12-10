mod clean1;
mod clean2;
mod lr;
mod graph;
mod le;

use std::thread;
use std::error::Error;

//main function
// call clean 1 as function 

fn main() -> Result<(), Box<dyn Error>> {
    clean1::clean1();
    clean2::clean2();
    lr::Lr();
    le::Le();
    graph::graph();
    

    Ok(())
    
}
