#!/usr/bin/env python3 
# Advent of code 2022 day 14
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

class Cave():
    def __init__(self) -> None:
        self._cavemap={}
        self.upper_left_corner=(500,0)
        self.lower_right_corner=(500,0)
        self.floor=None

    def AddRock(self,start,finish):
        if start[0]==finish[0]:  #horizontal wall
            for i in range(abs(start[1]-finish[1])+1):
                self._cavemap[(start[0],min([start[1],finish[1]])+i)]="#"
        else: #verticle wall
            for i in range(abs(start[0]-finish[0])+1):
                self._cavemap[(min([start[0],finish[0]])+i),start[1]]="#"
        #set new boundaries for the cave map
        self.upper_left_corner=(min([self.upper_left_corner[0],start[0],finish[0]]),min([self.upper_left_corner[1],start[1],finish[1]]))
        self.lower_right_corner=(max([self.lower_right_corner[0],start[0],finish[0]]),max([self.lower_right_corner[1],start[1],finish[1]]))

    def SetFloor(self):
        self.floor=self.lower_right_corner[1]+2
        self.lower_right_corner=(self.lower_right_corner[0],self.floor)

    def IsOccupied(self,point):
        return(self._cavemap.get(point)) or self.floor == point[1]

    def DropSand(self,PourPoint) -> bool : #returns true if sand placed
        (x,y)=PourPoint
        PourPointEmpty=not self.IsOccupied(PourPoint)
        at_rest=False
        while (not at_rest) and y<=self.lower_right_corner[1] and PourPointEmpty: 
            if not self.IsOccupied((x,y+1)):
                y+=1
            elif not self.IsOccupied((x-1,y+1)):
                x-=1
                y+=1
            elif not self.IsOccupied((x+1,y+1)):
                x+=1
                y+=1
            else:
                self._cavemap[(x,y)]="o"
                at_rest=True
        return at_rest  #true if sand places, false otherwise
 
def build_cave(buffer):
    ret=Cave()
    for i in buffer:
        parse=i.split(" -> ")
        for j in range(len(parse)-1):
            ret.AddRock(eval ("("+parse[j]+")"),eval("("+parse[j+1]+")"))
    return ret

myCave=build_cave(lines)
answer=0
for i in range(1,3):
    while myCave.DropSand((500,0)):
        answer+=1
    print("Part",i,"answer is",answer)
    myCave.SetFloor()