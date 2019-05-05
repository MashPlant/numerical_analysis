import numpy as np


def make_hilbert(n):
    ret = np.ndarray([n, n])
    for i in range(n):
        for j in range(n):
            ret[i][j] = 1 / (i+j+1)
    return ret


h4 = make_hilbert(4)
h8 = make_hilbert(8)
h10 = make_hilbert(10)
h12 = make_hilbert(12)

print(np.linalg.cond(h4, p=np.inf))
print(np.linalg.cond(h8, p=np.inf))
print(np.linalg.cond(h10, p=np.inf))
print(np.linalg.cond(h12, p=np.inf))
