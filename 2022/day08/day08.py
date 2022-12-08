#!/usr/bin/env python3 
# Advent of code 2022 day 08
# Author:  Bill Moss

from argparse import ArgumentParser

parser = ArgumentParser()
parser.add_argument("-f", "--file", dest="inputfile",
                            help="Take input from FILE", metavar="FILE")
parser.add_argument("-d", "--debug",
                            action="store_true", dest="debug", default=False,
                                                help="print status/debug messages to stdout")

args = parser.parse_args()

file = open (args.inputfile, 'r')
if args.debug:
    print (args)

lines=file.read().splitlines()

def build_forest(buffer):
    ret=[]
    for i in reversed(buffer):
        ret.append([int(j) for j in i])
    return(ret)

def visible(buffer,x,y):
    current_tree=buffer[x][y]
    ret=False
    ret=ret or current_tree>max([-1]+[buffer[i][y] for i in range(x)])
    ret=ret or current_tree>max([-1]+[buffer[i][y] for i in range(x+1,len(buffer[y]))])
    ret=ret or current_tree>max([-1]+[buffer[x][i] for i in range(y)])
    ret=ret or current_tree>max([-1]+[buffer[x][i] for i in range(y+1,len(buffer[x]))])
    return(ret)

def viewable(thistree,buffer):
    #return the count of trees viewable from thistree 
    #assuming 1 dim list of trees ascending closest to furthest
    notblocked=True
    ret=0
    for i in buffer:
        if notblocked:
            if i >=thistree:
                notblocked=False
            ret+=1
    return(ret)

def scenic_score(buffer,x,y):
    current_tree=buffer[x][y]
    ret=1
    ret=ret * viewable(current_tree,[buffer[i][y] for i in reversed(range(x))])
    ret=ret * viewable(current_tree,[buffer[i][y] for i in range(x+1,len(buffer[y]))])
    ret=ret * viewable(current_tree,[buffer[x][i] for i in reversed(range(y))])
    ret=ret * viewable(current_tree,[buffer[x][i] for i in range(y+1,len(buffer[x]))])
    return(ret)

def perform_parts(buffer):
    ret=[0,0,0]
    ret[1]=sum([visible(buffer,i,j) for i in range(len(buffer[0])) for j in range(len(buffer))])
    ret[2]=max([scenic_score(buffer,i,j) for i in range(len(buffer[0])) for j in range(len(buffer))])
    return(ret)

forest = build_forest(lines)       
results=perform_parts(forest)  
for i in range(1,3):
    print("part",i," answer is",results[i])