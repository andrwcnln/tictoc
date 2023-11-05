use pyo3::prelude::*;
use std::time::Instant;

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
        }
    }

    fn tic(&mut self) {
        self.time = Instant::now()
    }

    fn toc(&mut self) {
        let res = Results {
            nanos: self.time.elapsed().as_nanos(),
            micros: self.time.elapsed().as_micros(),
            millis: self.time.elapsed().as_millis(),
            seconds: self.time.elapsed().as_secs_f64(),
        };
        self.results = res;
    }
}

#[pymodule]
fn tictoc(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Init>()?;
    Ok(())
}
