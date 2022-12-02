#/usr/bin/env python3
# Advent of Code 2021 day 12
# Author:  Bill Moss

import argparse
import functools
parser = argparse.ArgumentParser(description="2021 Day 12 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

lines=file.read().splitlines()


def build_path_dict(inparg):    #build the path dictionary for the path list in inparg
    ret={}
    for i in inparg:
        caves=i.split('-')
        for j in [(caves[0],caves[1]),(caves[1],caves[0])]:
            curval=ret.get(j[0],[])
            curval.append(j[1])
            ret[j[0]]=curval
    return(ret)
path_dict=build_path_dict(lines)


def build_path(inp_path,rlist,Pt2_flag,lc_2visits):            #recursive function to build a path - rlist collects the answers.
    global path_dict
    last=inp_path[-1]
    if last == 'end':
        rlist.append(inp_path)
        return()
    if last.islower():
        if last in inp_path[:-1]:
            if Pt2_flag and not(last in ['start','end']) and lc_2visits == 0:
                lc_2visits+=1
            else:
                return()    #don't count this as a valid path... return without adding it to the rlist.
    list(map(lambda x:build_path(inp_path+[x],rlist,Pt2_flag,lc_2visits),path_dict[last]))
    return()

pt1_result_list=[]
build_path(['start'],pt1_result_list,False,0)
print('Part 1 answer is:',len(pt1_result_list))
            
pt2_result_list=[]
build_path(['start'],pt2_result_list,True,0)
print('Part 2 answer is:',len(pt2_result_list))            
    



exit()
