#!/usr/bin/env python3 
# Advent of code 2022 day 06
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

def are_dupes(buffer,count):
    for i in range(0,count-1):
        if buffer.find(buffer[i],i+1,count) != -1:
            return(True)
    return(False)

def perform_parts(buffer,part):
    ret=0
    length=4 + (part-1)*10    
    while are_dupes(buffer[ret:],length):
        ret += 1
    ret += length
    return(ret)

for i in range(1,3):
    print("part",i," answer is",perform_parts(lines[0],i))