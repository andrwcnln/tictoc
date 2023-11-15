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
        let elapsed_time = self.time.elapsed();
        self.results = Results {
            nanos: elapsed_time.as_nanos(),
            micros: elapsed_time.as_micros(),
            millis: elapsed_time.as_millis(),
            seconds: elapsed_time.as_secs_f64(),
        };
    }
}

#[pymodule]
fn tictoc(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Init>()?;
    Ok(())
}
