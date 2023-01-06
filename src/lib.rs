#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python};


fn foo(_py: Python, val: Vec<i32>) -> PyResult<bool> {
    for item in val.iter() {
        println!("item: {}", item);
    }
    Ok(true)
}

fn color(_py: Python, val: String) -> PyResult<String> {
    match val.as_str() {
        "online" => Ok("green".to_string()),
        _ => Ok("red".to_string()),
    }
}

py_module_initializer!(status, initstatus, Pyinit_status, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "color", py_fn!(py, color(val: String)))?;
    m.add(py, "foo", py_fn!(py, foo(val: Vec<i32>)))?;
    Ok(())
});
