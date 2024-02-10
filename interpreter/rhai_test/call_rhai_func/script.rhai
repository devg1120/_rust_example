//import "process" as proc;           // this is evaluated every time

fn hello(x, y) {
    // hopefully 'my_var' is in scope when this is called
    //x.len + y + my_var
     y + my_var
}

fn hello(x) {
    // hopefully 'my_string' is in scope when this is called
    x * my_string.len()
}
/*
fn hello() {
    // hopefully 'MY_CONST' is in scope when this is called
    if MY_CONST {
        proc::process_data(42);     // can access imported module
    }
}
*/
