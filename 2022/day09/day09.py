#!/usr/bin/env python3 
# Advent of code 2022 day 09
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

class Rope():
    def __init__(self,head=(0,0),tail=(0,0)):
        #locals
        self.head=head
        self.tail=tail
        self.tailvisitset={self.tail}
    
    def __repr__(self):
        ret = ""
        ret += f"Head: {self.head}\n"
        ret += f"Tail: {self.tail}\n"
        ret += f"Visited List: {self.tailvisitset}\n"
        return ret

    def TailVisited(self):
        return self.tailvisitset

    def GetTail(self):
        return self.tail
    def GetHead(self):
        return self.head


    def PerformMove(self,inst):
        parse=inst.split(" ")
        moves=int(parse[1])
        movedirection=(0,0)
        if parse[0]=="L":
            movedirection=(-1,0)
        elif parse[0]=="R":
            movedirection=(1,0)
        elif parse[0]=="U":
            movedirection=(0,1)
        elif parse[0]=="D":
            movedirection=(0,-1)
        else:
            print(f"error in instructions {inst}")
            exit()
        for i in range(moves):
            newhead=tuple(map(sum, zip(self.GetHead(), movedirection)))
            self.NewHeadPosition(newhead)
        return self
    
    def NewHeadPosition(self,newhead):
        self.head=newhead
        if abs(self.head[0]-self.tail[0])<2 and abs(self.head[1]-self.tail[1])<2:
            pass #tail doesn't have to move
        elif self.head[0]==self.tail[0]:
            self.tail=(self.tail[0],self.tail[1]+ (1 if self.head[1]>self.tail[1] else -1))
        elif self.head[1]==self.tail[1]:
            self.tail=(self.tail[0]+ (1 if self.head[0]>self.tail[0] else -1),self.tail[1])
        else: #diagonal move necessary
            self.tail=(self.tail[0]+ (1 if self.head[0]>self.tail[0] else -1),self.tail[1]+ (1 if self.head[1]>self.tail[1] else -1))
        self.tailvisitset.add(self.tail)
        return self

class SuperRope(Rope):
    def __init__(self):
        #locals
        self.ropes=[Rope() for i in range(9)]
    
    def __repr__(self):
        ret = ""
        ret += "Ropes\n"
        for i in range(len(self.ropes)):
            ret += f"====Rope{i}: {self.ropes[i]}\n"
        return ret

    def TailVisited(self):
        return self.ropes[len(self.ropes)-1].TailVisited()

    def GetHead(self):
        return self.ropes[0].head

    def NewHeadPosition(self,newhead):
        nexthead=newhead
        for i in range(len(self.ropes)):
            beforetail=self.ropes[i].GetTail()
            self.ropes[i].NewHeadPosition(nexthead)
            aftertail=self.ropes[i].GetTail()
            if beforetail==aftertail:
                break
            else:
                nexthead=aftertail
        return(self)

def perform_parts(buffer):
    ret=[0,0,0]
    myrope=Rope()
    ret[1]=len([myrope.PerformMove(buffer[i]) for i in range(len(buffer))][-1].TailVisited())
    mysuperrope=SuperRope()
    ret[2]=len([mysuperrope.PerformMove(buffer[i]) for i in range(len(buffer))][-1].TailVisited())
    return(ret)

     
results=perform_parts(lines)  
for i in range(1,3):
    print("part",i,"answer is",results[i])