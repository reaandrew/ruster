#[cfg(test)]
use mockall::{predicate::*};   

#[cfg(test)]
use yaml_rust::{YamlLoader, YamlEmitter};

#[cfg_attr(test, mockall::automock)]
trait SpecFinder{
    fn find_specs(&self) -> String;
}

#[test]
fn test_something() {
    let mut mock_something = MockSpecFinder::new();
    mock_something.expect_find_specs()
        .times(1)
        .returning(| | "something".to_string());
    assert_eq!("something", &mock_something.find_specs())
        // test some code using that dependency
}

#[test]
fn test_something_else(){
       let s =
"
---
foo:
    - list1
    - list2
bar:
    - 1
    - 2.0
---
1
";
    let docs = YamlLoader::load_from_str(s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    print!("SOMETHING!!! ->>> {}", docs.len());

    // Debug support
    println!("{:?}", doc);

    // Index access for map & array
    assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // Chained key/array access is checked and won't panic,
    // return BadValue if they are not exist.
    assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}

fn main() {
    println!("Hello, world!");
}

