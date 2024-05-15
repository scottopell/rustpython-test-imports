use rustpython_vm as vm;

fn main() -> vm::PyResult<()> {
    let settings = vm::Settings::default().with_path("testlibrary".to_owned());
    let interp = vm::Interpreter::with_init(settings, |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
    });
    interp.enter(|vm| {
        {
            println!("START hello world");
            let scope = vm.new_scope_with_builtins();
            let source = r#"print("Hello World!")"#;
            let code_obj = vm
                .compile(source, vm::compiler::Mode::Exec, "<embedded>".to_owned())
                .map_err(|err| vm.new_syntax_error(&err, Some(source)))?;

            vm.run_code_obj(code_obj, scope)?;
            println!("END hello world");
        }

        {
            println!("START run import mylib");
            let scope = vm.new_scope_with_builtins();
            let source = r#"import mylib; print(mylib.AgentCheck)"#;
            let code_obj = vm
                .compile(source, vm::compiler::Mode::Exec, "<embedded>".to_owned())
                .map_err(|err| vm.new_syntax_error(&err, Some(source)))?;

            match vm.run_code_obj(code_obj, scope) {
                Ok(_) => {}
                Err(e) => {
                    vm.print_exception(e);
                }
            }
            println!("END run import mylib");
        }

        {
            println!("START direct import mylib");
            match vm.import("mylib", 0) {
                Ok(_) => {
                    println!("Imported mylib successfully!");
                }
                Err(e) => {
                    vm.print_exception(e);
                }
            }
            println!("END direct import mylib");
        }

        Ok(())
    })
}
