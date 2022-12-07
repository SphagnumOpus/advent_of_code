#!/usr/bin/env python3 
# Advent of code 2022 day 05
# Author:  Bill Moss

from argparse import ArgumentParser
import copy

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

def parse_configuration(buffer):
    lineptr=len(buffer)-1
    while buffer[lineptr][0:4]=="move":
        lineptr-=1
    lineptr-=1
    #should be on the line that contains the stack numbers
    ret=[[]]
    for i in range(0,len(buffer[lineptr])):
        if buffer[lineptr][i:i+1].isnumeric():
            stknum=int(buffer[lineptr][i:i+1])
            if len(ret) != stknum:
                print("error in data")
            else:
                ret.append([])
            for j in range(lineptr-1,-1,-1):
                if buffer[j][i:i+1].isalpha():
                    ret[stknum].append(buffer[j][i:i+1])
    return(ret)

init_config=parse_configuration(lines)

def perform_movements(buffer,bufconfig,part):
    ret=copy.deepcopy(bufconfig)
    for i in range(0,len(buffer)):
        if buffer[i][0:4]=="move":
            cmd=buffer[i].split(" ")
            moveamt=int(cmd[1])
            movefrom=int(cmd[3])
            moveto=int(cmd[5])
            for j in range(-1*moveamt,0):
                if part==1:
                    ret[moveto].append(ret[movefrom].pop())
                else:
                    ret[moveto].append(ret[movefrom].pop(j))
    return(ret)


def top_of_stack(buffer):
    ret=""
    for i in buffer:
        if len(i)>0:
            ret=ret+i[-1]
    return(ret)

print("part 1 answer is",top_of_stack(perform_movements(lines,init_config,1)))
print("part 1 answer is",top_of_stack(perform_movements(lines,init_config,2)))


