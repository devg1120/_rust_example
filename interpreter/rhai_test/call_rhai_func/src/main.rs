use rhai::{Engine, Scope, EvalAltResult};
/*
pub fn main() -> Result<(), Box<EvalAltResult>>
//                          ^^^^^^^^^^^^^^^^^^
//                          Rhai API error type
{
    // Create an 'Engine'
    let engine = Engine::new();

    // Your first Rhai Script
    let script = "print(40 + 2);";

    // Run the script - prints "42"
    engine.run(script)?;

    // Done!
    Ok(())
}
*/

pub fn main() -> Result<(), Box<EvalAltResult>>
{
let engine = Engine::new();
// Compile the script to AST
//let ast = engine.compile(script)?;
let ast = engine.compile_file("script.rhai".into())?;

// Create a custom 'Scope'
let mut scope = Scope::new();

// A custom 'Scope' can also contain any variables/constants available to
// the functions
scope.push("my_var", 42_i64);
scope.push("my_string", "hello, world!");
scope.push_constant("MY_CONST", true);

// Evaluate a function defined in the script, passing arguments into the
// script as a tuple.
//
// Beware, arguments must be of the correct types because Rhai does not
// have built-in type conversions. If arguments of the wrong types are passed,
// the Engine will not find the function.
//
// Variables/constants pushed into the custom 'Scope'
// (i.e. 'my_var', 'my_string', 'MY_CONST') are visible to the function.

let result = engine.call_fn::<i64>(&mut scope, &ast, "hello", ( "abc", 123_i64 ) )?;
//                            ^^^                             ^^^^^^^^^^^^^^^^^^
//              return type must be specified                 put arguments in a tuple
println!("{:?}", result);
let result = engine.call_fn::<i64>(&mut scope, &ast, "hello", ( 123_i64, ) )?;
println!("{:?}", result);
//                                                            ^^^^^^^^^^^^ tuple of one

//let result = engine.call_fn::<i64>(&mut scope, &ast, "hello", () )?;
//                                                            ^^ unit = tuple of zero
    Ok(())
}

