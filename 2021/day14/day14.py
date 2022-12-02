#/usr/bin/env python3
# Advent of Code 2021 day 14
# Author:  Bill Moss

import argparse
import collections
from typing import Counter
parser = argparse.ArgumentParser(description="2021 Day 14 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-n", "--naive", help="include earlier naive implementation",action="store_true")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

lines=file.read().splitlines()

template=lines[0]
rules={}
for i in lines[2:]:
    x=i.split(' ')
    rules[x[0]]=x[2]


if args.naive:
    def do_step(r,t):   #do a step using rule set r and input t
        ret=''
        for i in range(len(t)-1):
           ret=ret+t[i]+r[t[i:i+2]]
        ret=ret+t[-1]
        return(ret)
    out = template
    for i in range(10):
        out=do_step(rules,out)
    e_dict=Counter(out)
    counts=sorted(list(e_dict.values()))
    print('Naive implementation of Part 1 answer is', counts[-1]-counts[0])
#this approach to Part 1 won't scale to part 2.
#let's try counting elements, and counting active element pairs.
elements=Counter(template)   #count elements we start with.
pair_count_dict={}
for i in range(len(template)-1):
    cur_pair=template[i:i+2]
    pair_count_dict[cur_pair]=pair_count_dict.get(cur_pair,0) +1


def do_fast_step(r,p,e):   #r is the ruleset, t is the input template, p is the pair count dictionary e is the element count dictionary
    ret_pairs={}
    ret_elements=elements
    for i in r.keys():
        if p.get(i,0) > 0:
            new_char=r[i]
            ret_elements[new_char]=ret_elements.get(new_char,0) + p[i]
            ret_pairs[i[0]+new_char]=ret_pairs.get(i[0]+new_char,0)+ p[i]
            ret_pairs[new_char+i[1]]=ret_pairs.get(new_char+i[1],0)+ p[i]
    return(ret_pairs,ret_elements)

out_pairs=pair_count_dict
out_elements=elements
for i in range(10):
    (out_pairs,out_elements)=do_fast_step(rules,out_pairs,out_elements)
counts=sorted(list(out_elements.values()))
print('Part 1 answer is', counts[-1]-counts[0])
for i in range(30):   #do 30 more to get part 2 answer
    (out_pairs,out_elements)=do_fast_step(rules,out_pairs,out_elements)
counts=sorted(list(out_elements.values()))
print('Part 2 answer is', counts[-1]-counts[0])

exit()
