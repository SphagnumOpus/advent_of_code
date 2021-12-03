#/usr/bin/env python3

import argparse
parser = argparse.ArgumentParser(description="2021 Day 1 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
args = parser.parse_args()

file = open (args.file,'r')

int_array = list(map (int,file.read().splitlines()))

def compute_list(x):
    prev=int(x[0])
    answer = 0
    for i in x:
        if prev < i:
            answer += 1
        prev = i
    return (answer)






print (compute_list(int_array))
from operator import add
window_int_array = map(add,map(add,int_array[:-2], int_array[1:-1]),int_array[2:])
print (compute_list(window_int_array))
