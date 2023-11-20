use pyo3::prelude::*;
use std::time::Instant;
use pyo3::exceptions::PyException;

#[pyclass]
#[derive(Clone)]
struct Results {
    #[pyo3(get)]
    nanos: u128,
    #[pyo3(get)]
    micros: u128,
    #[pyo3(get)]
    millis: u128,
    #[pyo3(get)]
    seconds: f64,
}

#[pyclass(module = "tictoc", name = "init")]
struct Init {
    time: Instant,
    #[pyo3(get)]
    results: Results,
    status: bool,
}

#[pymethods]
impl Init {
    #[new]
    fn new() -> Self {
        let res = Results {
            nanos: 0,
            micros: 0,
            millis: 0,
            seconds: 0.0,
        };
        Init {
            time: Instant::now(),
            results: res,
            status: false,
        }
    }

    fn tic(&mut self) {
        self.time = Instant::now();
        self.status = true;
    }

    fn toc(&mut self) -> PyResult<()> {
        if self.status == false {
            Err(PyException::new_err("tic() must be called before toc()"))    
        } else {       
            let elapsed_time = self.time.elapsed();
            self.results = Results {
                nanos: elapsed_time.as_nanos(),
                micros: elapsed_time.as_micros(),
                millis: elapsed_time.as_millis(),
                seconds: elapsed_time.as_secs_f64(),
            };
            Ok(())
        }
    }
}

#[pymodule]
fn tictoc(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Init>()?;
    Ok(())
}

#[test]
fn test_new() {
    let init = Init::new();
    assert_eq!(init.results.nanos,0);
}

#[test]
fn test_tic() {
    let mut init = Init::new();
    let time1 = init.time;
    init.tic();
    let time2 = init.time;
    assert!(time2 > time1)
}

#[test]
fn test_toc() {
    let mut init = Init::new();
    init.tic();
    println!("{}","test");
    let _ = init.toc();
    assert!(init.results.nanos > 0)
}
