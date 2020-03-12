from collections import defaultdict
TARGET = 4

from time import time

def canon(str_):
    L = [str_]
    for _ in range(len(str_)):
        str_ = str_[1:] + str_[0]
        L.append(str_)
    return min(L)    

def solve_problem(file):
    with open(file) as f:
        text = f.read().split('\n')

    D = defaultdict(list)

    a = time()
    for word in sorted(text, key=len):
        idx = canon(word)
        D[idx].append(word)  
        if len(D[idx]) ==TARGET:
            return (D[idx], time() - a)

print(solve_problem('inputs/enable1.txt'))
