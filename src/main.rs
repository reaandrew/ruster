mod models;
mod finders;
mod utils;
mod adapters;
mod factories;
mod executors;
mod app;
mod core;
mod errors;
  
use self::app::App; 

use self::errors::{RusterError};
use rusty_v8 as v8;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;


fn main() {

    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, "'Hello' + ' World!'").unwrap();
    println!("javascript code: {}", code.to_rust_string_lossy(scope));

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(scope));

    let spec_finder = finders::FileSpecFinder{
        path: String::from("."),
    };

    let executor_factory = factories::DefaultExecutorFactory{};

    let app_factory = factories::DefaultAppFactory{
        spec_finder: &spec_finder,
        executor_factory: &executor_factory
    };
    match app_factory.create(){
        Ok(app) => {
            match app.execute(){
                Ok(result) => {
                    println!("Success: {}", result.success);
                },
                Err(e) => {
                    match e {
                        RusterError::Of(error_type) => {
                            println!("{}", error_type);
                        }
                    }
                }
            }; 
        },
        Err(e) => {
            println!("{}", e);
        }
    };
}

mod tests{

    #[cfg(test)]
    use rusty_v8 as v8;

    #[cfg(test)]
    use std::convert::{Into};
    #[cfg(test)]
    use std::sync::Mutex;
    #[cfg(test)]
    use std::collections::HashMap;


    #[cfg(test)]
    lazy_static! {
      static ref INIT_LOCK: Mutex<u32> = Mutex::new(0);
    }

    #[cfg(test)]
    #[must_use]
    struct SetupGuard {}

    #[cfg(test)]
    impl Drop for SetupGuard {
      fn drop(&mut self) {
        // TODO shutdown process cleanly.
      }
    }

    #[cfg(test)]
    struct X {
        things: HashMap<String, Vec<String>>,
    }

    #[cfg(test)]
    impl X {
        fn add_to_things(&mut self, key: &str, value: &str) {
            let elements = self.things.entry(key.to_string()).or_insert(vec![]);
            elements.push(value.to_string());
        }
    }

    #[cfg(test)]
    fn setup() -> SetupGuard {
      let mut g = INIT_LOCK.lock().unwrap();
      *g += 1;
      if *g == 1 {
        v8::V8::initialize_platform(v8::new_default_platform().unwrap());
        v8::V8::initialize();
      }
      SetupGuard {}
    }

    #[cfg(test)]
    fn fortytwo_callback(
      scope: &mut v8::HandleScope,
      _: v8::FunctionCallbackArguments,
      mut rv: v8::ReturnValue,
    ) {
        {
          let s = scope.get_slot::<X>().unwrap();
          println!("{:?}", s.things);
        }
      rv.set(v8::Integer::new(scope, 42).into());
    }

    #[cfg(test)]
    fn eval<'s>(
      scope: &mut v8::HandleScope<'s>,
      code: &str,
    ) -> Option<v8::Local<'s, v8::Value>> {
      let scope = &mut v8::EscapableHandleScope::new(scope);
      let source = v8::String::new(scope, code).unwrap();
      let script = v8::Script::compile(scope, source, None).unwrap();
      let r = script.run(scope);
      r.map(|v| scope.escape(v))
    }

    #[test]
    fn object_template() {
      let _setup_guard = setup();
      let isolate = &mut v8::Isolate::new(Default::default());
      {
        let mut state = X { things: HashMap::new()};
        state.add_to_things("fu","bar");
        isolate.set_slot(state);

        let scope = &mut v8::HandleScope::new(isolate);
        

        let object_templ = v8::ObjectTemplate::new(scope);

        let function_templ = v8::FunctionTemplate::new(scope, fortytwo_callback);
        let name = v8::String::new(scope, "f").unwrap();
        let attr = v8::READ_ONLY + v8::DONT_ENUM + v8::DONT_DELETE;
        object_templ.set_with_attr(name.into(), function_templ.into(), attr);

        
        let context = v8::Context::new(scope);

        let scope = &mut v8::ContextScope::new(scope, context);
        let object = object_templ.new_instance(scope).unwrap();
        assert!(!object.is_null_or_undefined());

        let name = v8::String::new(scope, "g").unwrap();
        context.global(scope).define_own_property(
          scope,
          name.into(),
          object.into(),
          v8::DONT_ENUM,
        );

        let source = r#"
          {
            const d = Object.getOwnPropertyDescriptor(globalThis, "g");
            console.log('Settings for d', d);
            [d.configurable, d.enumerable, d.writable].toString()
          }
        "#;
        let actual = eval(scope, source).unwrap();
        let expected = v8::String::new(scope, "true,false,true").unwrap();
        assert!(expected.strict_equals(actual));
        let actual = eval(scope, "g.f()").unwrap();
        let expected = v8::Integer::new(scope, 42);
        assert!(expected.strict_equals(actual));
        let source = r#"
          {
            const d = Object.getOwnPropertyDescriptor(g, "f");
            [d.configurable, d.enumerable, d.writable].toString()
          }
        "#;
        let actual = eval(scope, source).unwrap();
        let expected = v8::String::new(scope, "false,false,false").unwrap();
        assert!(expected.strict_equals(actual));
      }
    }
}
