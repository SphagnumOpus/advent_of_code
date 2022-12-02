#/usr/bin/env python3
# Advent of Code 2021 day 13
# Author:  Bill Moss

import argparse
import functools
parser = argparse.ArgumentParser(description="2021 Day 13 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

lines=file.read().splitlines()
dots=[]
folds=[]
dots_done=False
for i in lines:
    if dots_done:
        s=i.split('=')
        folds.append((s[0][-1],int(s[1])))
    else:
        if i == "":
            dots_done=True
        else:
            s=i.split(',')
            dots.append((int(s[0]),int(s[1])))


def foldit(dotlist,fold_inst):
    if fold_inst[0]=='x':
        ret=list(map(lambda x: x if x[0]<fold_inst[1] else (fold_inst[1]-(x[0]-fold_inst[1]),x[1]),dotlist))
    else:
        ret=list(map(lambda x: x if x[1]<fold_inst[1] else (x[0],fold_inst[1]-(x[1]-fold_inst[1])),dotlist))
    return(ret)

print('Part 1 answer is:',len(list(set(foldit(dots,folds[0])))))

#now perform all the folds
for i in range(len(folds)):
    dots = list(set(foldit(dots,folds[i])))
def sortkey(a):
    return((a[1],a[0]))
sorted_dots=sorted(dots,key=sortkey)
#print the array
print("The Part 2 answer is:")
cursor=(0,0)
while len(sorted_dots)>0:
    next_dot=sorted_dots.pop(0)
    if (cursor[1]<next_dot[1]):
        print()   #put the newline at the end
        cursor=(0,next_dot[1])
    while (cursor[0]<next_dot[0]):
        print('.',end="")
        cursor=(cursor[0]+1,cursor[1])
    print('#',end="")
    cursor=(cursor[0]+1,cursor[1])
print()
exit()
