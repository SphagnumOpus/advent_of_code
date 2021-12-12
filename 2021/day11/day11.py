#/usr/bin/env python3
# Advent of Code 2021 day 11
# Author:  Bill Moss

import argparse
import functools
parser = argparse.ArgumentParser(description="2021 Day 11 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-g", "--grid", help="print the grid as we go",action="store_true")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

lines=file.read().splitlines()
grid=[[(x,y,int(j),0) for x,j in enumerate(i)] for y,i in enumerate(lines)]
gwidth=len(lines[0])
gheight=len(lines)
inc_all=list(functools.reduce(lambda x,y:x+y,[[(j,i) for j in range(gwidth)]for i in range(gheight)])) #build a set of tuples to list all cell
pt1_answer=0   #global variable to count the flashes for pt 1 answer

def inc_one_cell(thisgrid,cell,initial_run):
    global gwidth
    global gheight
    global pt1_answer
    ret=[]
    curr_value=thisgrid[cell[1]][cell[0]]
    if curr_value[2] != 0 or initial_run :
        if curr_value[2]<9:
            thisgrid[cell[1]][cell[0]] = (curr_value[0],curr_value[1],curr_value[2]+1,curr_value[3])
        else:
            thisgrid[cell[1]][cell[0]] = (curr_value[0],curr_value[1],0,curr_value[3])   #this octopus flashed....
            pt1_answer+=1
            for i in [-1,0,+1]:
                if cell[1]+i >= 0 and cell[1]+i < gheight:
                    for j in [-1,0,+1]:
                        if cell[0]+j >= 0 and cell[0]+j < gwidth:
                            ret.append((cell[0]+j,cell[1]+i))
    return(ret)



def do_a_step(thisgrid):
    global inc_all
    inc_list=inc_all
    first_time=True
    while len(inc_list) > 0:
        #increment each element of the grid, then return list of all elements that now need incrementing
        inc_list = list(functools.reduce(lambda x,y: x+y, list(map(lambda x: inc_one_cell(thisgrid,x,first_time),inc_list))))
        first_time=False   #needed so we don't keep incrementing cells with 0 in them after flashes
    return(inc_list)

def printgrid(thisgrid):
    for i in thisgrid:
        for j in i:
            print(j[2],end="")
        print()
    print()
    return()

for i in range(100):
    do_a_step(grid)
    if args.grid:
        printgrid(grid)
print('Part 1 answer is:',pt1_answer)
grid=[[(x,y,int(j),0) for x,j in enumerate(i)] for y,i in enumerate(lines)] #reset the grid for part 2
total = 1   #start with a non-zero value
runs=0
while total >0:
    do_a_step(grid)
    if args.grid:
        printgrid(grid)
    total=sum(list(map(lambda x:x[2],list(functools.reduce(lambda x,y:x+y,grid)))))
    runs+=1
print('Part 2 answer is:',runs)


exit()

