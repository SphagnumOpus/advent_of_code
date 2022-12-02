#!/usr/bin/env python3
# Advent of code 2022 day 01
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

def build_calorie_list(buffer):
    ret=[]
    cur_calories=0
    for i in buffer+ [""]:
        if len(i) == 0:
            ret.append(cur_calories)
            cur_calories=0
        else:
            cur_calories+=int(i)
    return (ret)

calorie_list=build_calorie_list(lines)
calorie_list.sort(reverse=True)
print("part 1 answer is",calorie_list[0])
print("part 2 answer is",sum(calorie_list[0:3]))

