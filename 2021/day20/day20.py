#/usr/bin/env python3
# Advent of Code 2021 day 20
# Author:  Bill Moss

import argparse
import functools
import itertools


parser = argparse.ArgumentParser(description="2021 Day 20 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-p", "--print", help="print picture",action="store_true")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')






lines=file.read().splitlines()

def iea_table_create(buf):
    #take a string as per day20 specs and turn into an array 0-255 of values f
    ret=[]
    [ret.append(0 if i=='.' else 1) for i in buf]
    if len(ret) != 512:
        print("Error!  Image enhancement algorithm is incomplete - length = ",len(ret))
        exit()
    return(ret)

iea = iea_table_create(lines[0])


def picture_create(buf):
    default=0   #always start with the default being zero... seems like the puzzle assumes that
    pixel={}
    for x in range(len(buf[2])):
        for j in range(2,len(buf)):
            y=j-2
            pixel[(x,y)]=1 if buf[j][x]=='#' else 0
    
    xform_map={}
    return((default,pixel,xform_map))



def picture_print(p):
    minp=functools.reduce(lambda a, b: (min(a[0],b[0]),min(a[1],b[1])),p[1].keys())
    maxp=functools.reduce(lambda a, b: (max(a[0],b[0]),max(a[1],b[1])),p[1].keys())
    for j in range (minp[1],maxp[1]+1):
        for i in range (minp[0],maxp[0]+1):
            print ('#' if p[1][(i,j)] == 1 else '.',end="")
        print()
    return()

def picture_enhance(p,t):
    default_mask=511 if p[0]==1 else 0     #mask to use for infinite expanse around picture for this pic
    for key,value in p[1].items():
        shift=0
        for i in range(-1,2):
            for j in range(-1,2):
                if value==0:
                    p[2][(key[0]+j,key[1]+i)] = p[2].setdefault((key[0]+j,key[1]+i),default_mask) & ~(1<<shift)
                else:
                    p[2][(key[0]+j,key[1]+i)] = p[2].setdefault((key[0]+j,key[1]+i),default_mask) | (1<<shift)
                shift+=1
    #we have the transform bitmasks for all the values in p[2], now create a new picture
    new_default=t[default_mask]
    new_pixel={}
    for key,value in p[2].items():
        new_pixel[key]=t[value]
    xform_map={}
    return((new_default,new_pixel,xform_map))

def picture_pixel_count(p):
    return(list(p[1].values()).count(1))

pic = picture_create(lines)
if args.print:
    picture_print(pic)
for i in range(2):
    pic=picture_enhance(pic,iea)
    if args.print:
        picture_print(pic)

print('Part 1 answer is',picture_pixel_count(pic))

#do it again for part 2
pic = picture_create(lines)
if args.print:
    picture_print(pic)
for i in range(50):
    pic=picture_enhance(pic,iea)
    if args.print:
        picture_print(pic)

print('Part 2 answer is',picture_pixel_count(pic))
    
exit()
