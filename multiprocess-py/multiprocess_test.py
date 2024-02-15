from multiprocessing import Pool
import timeit

ITERATIONS_OF_WORK = 128

def busy_work(input):
    x = 0
    for i in range(10_000_000):
        x += i
    return x

def test_multi():
    pool = Pool(processes=8)
    results = pool.map(busy_work, range(ITERATIONS_OF_WORK))

def test_single():
    for i in range(ITERATIONS_OF_WORK):
        busy_work(i)

if __name__ == '__main__':
    test_iterations = 1
    print("Single: ", end="")
    print(timeit.timeit("test_single()", setup="from __main__ import test_single", number=test_iterations))
    print("Multi: ", end="")
    print(timeit.timeit("test_multi()", setup="from __main__ import test_multi", number=test_iterations))
   