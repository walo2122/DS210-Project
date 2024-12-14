mod clean1;
mod clean2;
mod lr;
mod graph;
mod le;
mod test1;
mod test2;

use std::thread;
use std::error::Error;

//main function
 

fn main() -> Result<(), Box<dyn Error>> {
    clean1::clean1();
    clean2::clean2();
    lr::Lr();
    le::Le();
    graph::graph();
    test1::test_1();
    test2::test_2();
    

    Ok(())
    
}
