'''Art Remover
==============

Findings squares of blocks of specific sizes that are all the same.
'''

# The goal is to find the 3x3 square of `1`.
DATA = [ list(x) for x in '''
000000000000
111111111111
111111111111
000001110000
'''.strip().split("\n") ]

def ranges(xmax, ymax, xmin=0, ymin=0):
    '''Returns a flattened iterator of x by y positions.
    '''
    return ((x, y) for y in range(ymin, ymax)
            for x in range(xmin, xmax))

def neighcount(x, y, data, d='1', size=3):
    '''Checks a square of `size` full of `d`.
    '''
    return all(data[x][y] == d for x, y in ranges(x+size, y+size, x, y))

xy = ranges(len(DATA)-2, len(DATA[0])-2)
nc = ((x, y) for x, y in xy if neighcount(x, y, DATA))

print(list(nc))
# [(1, 5)]
