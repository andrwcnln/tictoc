![A logo with the word tictoc followed by a stopwatch emoji](./.docs/logoLightMode.png#gh-light-mode-only)
![A logo with the word tictoc followed by a stopwatch emoji](./.docs/logoDarkMode.png#gh-dark-mode-only)

# Fast, simple and accurate Python timing. Written in Rust.

## Installation
Install with [pip](https://pypi.org/project/pip):
```bash
$ python -m pip install tictoc
```

## Usage
Import and initialise. **The module must be initialised to be used!** 
```python
import tictoc
t = tictoc.init()
```
Begin timing with `tic()`, and stop with `toc()`.
```python
t.tic()
# some code
t.toc()
```
When `toc` is called, the results are saved. They can be accessed with the following syntax: 
```python
t.results.{unit}
```
The available units are:
```python
t.results.nanos   # u128
t.results.micros  # u128
t.results.millis  # u128
t.results.seconds # f64
```

## Full example
```python
import time

import tictoc
t = tictoc.init()

t.tic()       # start timing
time.sleep(3) # sleep for 3 seconds
t.toc()       # stop timing

print(t.results.seconds)
# >>> 
``` 3.000457715
