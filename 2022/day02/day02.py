#!/usr/bin/env python3
# Advent of code 2022 day 02
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
Part_1_Table={'A X':3+1,'A Y':6+2,'A Z':0+3,'B X':0+1,'B Y':3+2,'B Z':6+3,'C X':6+1,'C Y':0+2,'C Z':3+3}
Part_2_Table={'A X':0+3,'A Y':3+1,'A Z':6+2,'B X':0+1,'B Y':3+2,'B Z':6+3,'C X':0+2,'C Y':3+3,'C Z':6+1}
def build_score_list(buffer,xlate):
    ret=[]
    for i in buffer:
        ret=ret + [xlate[i]]
    return (ret)

score_list=build_score_list(lines,Part_1_Table)
print("part 1 answer is",sum(build_score_list(lines,Part_1_Table)))
print("part 2 answer is",sum(build_score_list(lines,Part_2_Table)))

