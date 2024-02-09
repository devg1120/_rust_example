use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<EvalAltResult>>
{
    // Define external function
    fn compute_something(x: i64) -> bool {
       println!("t func n:{}",x);
        (x % 40) == 0
    }

    // Create scripting engine
    let mut engine = Engine::new();

    // Register external function as 'compute'
    engine.register_fn("compute", compute_something);

    // Evaluate the script, expecting a 'bool' result
    let result: bool = engine.eval_file("my_script.rhai".into())?;

    assert_eq!(result, true);

    Ok(())
}
