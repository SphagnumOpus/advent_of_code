#/usr/bin/env python3
# Advent of Code 2021 day 9
# Author:  Bill Moss

import argparse
import functools
parser = argparse.ArgumentParser(description="2021 Day 9 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

lines=file.read().splitlines()


heightmap=[[(x,y,int(j),0) for x,j in enumerate(i)] for y,i in enumerate(lines)]
hmap_width=len(lines[0])
hmap_height=len(lines)
next_basin_id=1

#this code works assuming heightmap lines are all equal length
def assess_basin(coordinate):  #recursive function to mark basin entries
    basin_id=coordinate[3]

    x=coordinate[0]
    y=coordinate[1]
    for i,j in [(x-1,coordinate[1]),(x+1,coordinate[1]),(coordinate[0],y-1),(coordinate[0],y+1)]:
            if i>=0 and i<hmap_width and j>=0 and j<hmap_height:
                if heightmap[j][i][2]<9 and heightmap[j][i][2]>coordinate[2]:
                    heightmap[j][i] = (i,j,coordinate[2],basin_id)
                    assess_basin(heightmap[j][i])
    return()


def low_point_risk_level(coordinate):   #returns low point risk level for an (x,y,value) tuple, zero if not low point
    global next_basin_id
    x=coordinate[0]
    y=coordinate[1]
    for i,j in [(x-1,coordinate[1]),(x+1,coordinate[1]),(coordinate[0],y-1),(coordinate[0],y+1)]:
            if i>=0 and i<hmap_width and j>=0 and j<hmap_height:
                if heightmap[j][i][2]<=coordinate[2]:
                   return(0)
    
    #we've found a new basin... put the basin id in the last tuple element this is needed for part 2
    heightmap[y][x]=(x,y,coordinate[2],next_basin_id)
    next_basin_id+=1
    assess_basin(heightmap[y][x])
    return(coordinate[2]+1)
    

print ('Part 1 answer is:',(sum([sum([low_point_risk_level(j) for j in i]) for i in heightmap])))
basin_count=[]
for i in heightmap:
    for j in i:
        if j[3] > 0:
            basin_count.append(j[3])
basin_hist=sorted([basin_count.count(i) for i in range(next_basin_id)], reverse=True)
print ('Part 2 answer is:',basin_hist[0]*basin_hist[1]*basin_hist[2])



exit()
