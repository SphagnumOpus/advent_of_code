#!/usr/bin/env python3 
# Advent of code 2022 day 04
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

def parse_assignment_line(buffer):
    assignments=buffer.split(",")
    ret=[]
    for i in assignments:
        breakup=i.split("-")
        ret=ret + [list(range(int(breakup[0]),int(breakup[1])+1))]
    return(ret)


def build_assignment_list(buffer):
    ret=[]
    for i in buffer:
        ret=ret + [parse_assignment_line(i)]
    return (ret)

def one_if_subset(item):
    if set(item[1]).issubset(set(item[0])) or set(item[0]).issubset(set(item[1])):
        return(1)
    else:
        return(0)


assignment_list=build_assignment_list(lines)

print("part 1 answer is",sum([one_if_subset(i) for i in assignment_list]))

def one_if_overlap(item):
    if len(set(item[1]).intersection(set(item[0]))) == 0:
        return(0)
    else:
        return(1)

print("part 2 answer is",sum([one_if_overlap(i) for i in assignment_list]))

