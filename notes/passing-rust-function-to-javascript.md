# Passing a Rust function to JavaScript

The following function is an example from the rusty_v8 project tests showing how to do this:

[https://github.com/denoland/rusty_v8/blob/master/tests/test_api.rs#L2827](https://github.com/denoland/rusty_v8/blob/master/tests/test_api.rs#L2827)

```rust
#[test]
fn object_template() {
  let _setup_guard = setup();
  let isolate = &mut v8::Isolate::new(Default::default());
  {
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
```
