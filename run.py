import time

import tictoc

t = tictoc.init()

t.tic()  # start timing
time.sleep(3)  # sleep for 3 seconds
t.toc()  # stop timing

print(t.results.seconds)
