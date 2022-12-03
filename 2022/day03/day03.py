#!/usr/bin/env python3 
# Advent of code 2022 day 03
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

def find_misplaced_item(buffer):
    firsthalf=buffer[0:int(len(buffer)/2)]
    secondhalf=buffer[int(len(buffer)/2):]
    return ("".join(set(firsthalf).intersection(secondhalf)))

def build_item_list(buffer):
    ret=[]
    for i in buffer:
        ret=ret + [find_misplaced_item(i)]
    return (ret)

def get_priority(item):
    return(" abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(item))


item_list=build_item_list(lines)

print("part 1 answer is",sum([get_priority(i) for i in item_list]))

def build_group_badge_list(buffer):
    ret=[]
    for i in range(0,len(buffer)-1,3):
        ret = ret + ["".join(set(buffer[i]).intersection(buffer[i+1]).intersection(buffer[i+2]))]
    return(ret)


group_badge_list=build_group_badge_list(lines)

print("part 2 answer is",sum([get_priority(i) for i in group_badge_list]))

