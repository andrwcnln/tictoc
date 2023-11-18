import pytest
import tictoc
import time
import math


@pytest.fixture
def t():
    t = tictoc.init()
    return t


class testFunctionality:
    def testBasic(self, t):
        t.tic()
        print("test")
        t.toc()
        assert t.results.seconds > 0

    def testOverwrite(self, t):
        t.tic()
        print("test")
        t.toc()
        firstResult = t.results.seconds
        print("test2")
        t.toc()
        secondResult = t.results.seconds

        assert firstResult < secondResult


class testInvalid:
    def testNoInit(self):
        with pytest.raises(Exception):
            t.tic()

    def testTocBeforeTic(self, t):
        with pytest.raises(Exception):
            t.toc()


@pytest.mark.parametrize("sleepTime", [0.05, 0.5, 1])
class testAccuracy:
    @pytest.fixture(scope="class")
    def tol(self):
        return 0.0006

    def testSingleCall(self, t, sleepTime, tol):
        t.tic()
        time.sleep(sleepTime)
        t.toc()
        assert (t.results.seconds > sleepTime) & (
            t.results.seconds < (t.results.seconds + tol)
        )

    def testMultipleCalls(self, t, sleepTime, tol):
        t.tic()
        time.sleep(sleepTime)
        t.toc()
        time.sleep(sleepTime)
        t.toc()
        assert (t.results.seconds > sleepTime * 2) & (
            t.results.seconds < (t.results.seconds + tol)
        )


class testConsistency:
    def testMicros(self, t):
        t.tic()
        print("test")
        t.toc()
        assert t.results.micros == (math.floor(t.results.nanos * pow(10, -3)))

    def testMillis(self, t):
        t.tic()
        print("test")
        t.toc()
        assert t.results.millis == (math.floor(t.results.nanos * pow(10, -6)))

    def testSeconds(self, t):
        t.tic()
        print("test")
        t.toc()
        assert t.results.seconds == round(
            (t.results.nanos * pow(10, -9)), 9
        )  # f64 vs u128, hence the round
