#!/usr/bin/env python3 
# Advent of code 2022 day 12
# Author:  Bill Moss

from argparse import ArgumentParser
import sys

sys.setrecursionlimit(2000)

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

class Square():
    def __init__(self,inp_arg):
        self.charvalue=inp_arg
        if inp_arg=="S":
            self.shortestdistance=0
            self.height=ord("a")
        else:
            self.shortestdistance=sys.maxsize
            self.height=ord(inp_arg if inp_arg != "E" else "z")
        return

    def __repr__(self):
        return self.charvalue
    
    def isEnd(self,thingtomatch):
        return self.charvalue==thingtomatch

    def ShortestPathLength(self):
        return self.shortestdistance

    def SetShortestPathLength(self,newdistance):
        self.shortestdistance=newdistance
        return

    def Height(self):
        return self.height
       
def map(buffer):
    ret1=[[[] for i in range(len(buffer))] for j in range(len(buffer[0]))]
    ret2=[]
    for j in range(len(buffer)):
        for i in range(len(buffer[j])):
            cur_char=buffer[len(buffer)-j-1][i]
            ret1[i][j]=Square(cur_char)
            if cur_char=="S":
                ret2=[i,j]
    return(ret1,ret2)

def can_make_step(new,current,direct):
    if direct=="UP":
        return((new-current)<2)
    else:
        return((current-new)<2)

def find_route(path_prefix,map_arg,sp_xarg,sp_yarg,direction,target):
    ret_br=path_prefix
    ret_bpl=sys.maxsize
    cur_best_path_length=map_arg[sp_xarg][sp_yarg].ShortestPathLength()
    results=[]
    for x,y in (sp_xarg+1,sp_yarg),(sp_xarg-1,sp_yarg),(sp_xarg,sp_yarg-1),(sp_xarg,sp_yarg+1):   
            if x<0 or y<0 or x>=len(map_arg) or y>=len(map_arg[0]):
                continue
            if (cur_best_path_length+1)< map_arg[x][y].ShortestPathLength():
                if can_make_step(map_arg[x][y].Height(),map_arg[sp_xarg][sp_yarg].Height(),direction):
                    #if map_arg[x][y].Height()-map_arg[sp_xarg][sp_yarg].Height()<2:
                    map_arg[x][y].SetShortestPathLength(cur_best_path_length+1)
                    if map_arg[x][y].isEnd(target):
                        return(path_prefix+[[x,y]],cur_best_path_length+1)
                    else:
                        fr_tmp_result=find_route(path_prefix + [[x,y]],map_arg,x,y,direction,target)
                        if fr_tmp_result[1] < sys.maxsize:
                            results.append(fr_tmp_result)
    for i in results:
        if i[1]<ret_bpl:
            ret_br=i[0]
            ret_bpl=i[1]
    return(ret_br,ret_bpl)

(mymap,mystart)=map(lines)
(part1_bestroute,part1_routelength)=find_route([],mymap,mystart[0],mystart[1],"UP","E")
Part1_Endpoint=part1_bestroute[-1]
print( "Part 1 answer is",part1_routelength)

(mymap,mystart)=map(lines)   #reset mymap for Part 2
mymap[mystart[0]][mystart[1]].SetShortestPathLength(sys.maxsize)   #S point is now target, set path length appropriately
mymap[Part1_Endpoint[0]][Part1_Endpoint[1]].SetShortestPathLength(0) #E point is now start, set path length appropriately
(part2_bestroute,part2_routelength)=find_route([],mymap,Part1_Endpoint[0],Part1_Endpoint[1],"DOWN","S")
shortest_a_path=sys.maxsize
for i in mymap:
    for j in i:
        if j.Height() == ord("a") and j.ShortestPathLength() < shortest_a_path:
            shortest_a_path=j.ShortestPathLength()
print( "Part 2 answer is",shortest_a_path)
