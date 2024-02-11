use rune::runtime::Function;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, Vm};
use rune::{ContextError, Module};
use rune_modules;
use std::sync::Arc;


mod stringy_math_macro;


pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::with_crate_item("gs", ["experiments"])?;
    module.macro_meta(stringy_math_macro::stringy_math)?;
    Ok(module)
}

fn main() -> rune::support::Result<()> {

    let mut context = rune_modules::default_context()?;

    context.install(module()?)?;

    let runtime = Arc::new(context.runtime()?);

    let mut sources = rune::sources! {
        entry => {
            use ::gs::experiments::stringy_math;

            fn foo(a, b) {
                a + b 
            }

            pub fn main() {
               //let output = stringy_math!(add 10 sub 2 div 3 mul 100);
               let output = stringy_math!( add 11 add 100);
               println!("{}", output);
               foo
            }
        }
    };

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;

    let mut vm = Vm::new(runtime, Arc::new(unit));
    let output = vm.call(["main"], ())?;
    let output: Function = rune::from_value(output)?;

    println!("{}", output.call::<(i64, i64), i64>((1, 3)).into_result()?);
    println!("{}", output.call::<(i64, i64), i64>((2, 6)).into_result()?);
    Ok(())
}
