#!/usr/bin/env python3 
# Advent of code 2022 day 13
# Author:  Bill Moss

from argparse import ArgumentParser
import functools
import numpy

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
LEFT=0
RIGHT=1

def make_List(buffer):
    ret=[] 
    bufptr=0
    while bufptr < len(buffer):
        tmplist=[]
        for _ in range(2):
            tmplist.append(eval(buffer[bufptr]))  #first is left, second is right
            bufptr+=1
        ret.append(tmplist)
        bufptr+=1   #skip blank line
    return ret


def is_right_order(packet_pair,topflag):
    if topflag==0:
        ret=True   #If evaluation is inconclusive at the end it's TRUE, otherwise
    else:
        ret=None   #we say None and continue the evaluation at the higher level in the recursion
    if type(packet_pair[LEFT])==type(packet_pair[RIGHT]): #if types match
        if type(packet_pair[LEFT])==type(0):  #they are both ints
            if packet_pair[LEFT]==packet_pair[RIGHT]:
                return None  #ints equal... neither true nor false yet
            else:
                return(packet_pair[LEFT]<packet_pair[RIGHT])
        else:   #they are both lists
            for i in range(len(packet_pair[LEFT])):  #walk through the left list
                if len(packet_pair[RIGHT])<=i:
                    return False   #right side ran out of items
                rc = is_right_order([packet_pair[LEFT][i],packet_pair[RIGHT][i]],topflag+1)
                if rc != None:
                    return rc
            if len(packet_pair[LEFT])==len(packet_pair[RIGHT]):
                return(None)  #both lists the same length and no comparison made a decision
            else:
                return(True)
    else:   # one is an int, one is a list
        recall_list=[[],[]]
        for leftright in range(2):
            if type(packet_pair[leftright])==type(0):
                recall_list[leftright]=[packet_pair[leftright]]  #put singleton int in list
            else:
                recall_list[leftright]=packet_pair[leftright]
        rc=is_right_order(recall_list,topflag+1)
        if rc != None:
            return rc
    return ret

myList=make_List(lines)
print("The answer to part 1 is",sum([i+1 for i in range(len(myList)) if is_right_order(myList[i],0)]))

def Part2_cmp_function(left,right):
    rc=is_right_order([left,right],0)
    if rc==True:
        return(-1)
    else:
        return(1)

myListPart2=[eval(lines[i]) for i in range(len(lines)) if lines[i] != ""]
#add the markers
myListPart2.append([[2]])
myListPart2.append([[6]])
myListPart2Sorted=sorted(myListPart2,key = functools.cmp_to_key(Part2_cmp_function))
print("The answer to part 2 is",numpy.prod([i+1 for i in range(len(myListPart2Sorted)) if myListPart2Sorted[i] == [[2]] or myListPart2Sorted[i] ==[[6]]]))

