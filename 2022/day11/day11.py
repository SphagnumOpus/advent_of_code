#!/usr/bin/env python3 
# Advent of code 2022 day 11
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

class Item():   #one of the items being tossed
    def __init__(self,startingworry):
        self.worry=startingworry
        return

    def __repr__(self):
        ret=f"|Item with worry={self.worry}|"
        return ret
    
    def GetWorry(self):
        return self.worry

    def SetWorry(self,newworry):
        self.worry=newworry
        return self

    def ApplyRelief(self):
        global PART_FLAG
        global DIVISOR_PRODUCT
        if PART_FLAG==1:
            self.worry=self.worry//3
        else:
            self.worry=self.worry%DIVISOR_PRODUCT
        return(self.worry)
    
class Monkey():  
    def __init__(self, monkeydef):
        global PART_FLAG
        global DIVISOR_PRODUCT
        #locals
        self.itemlist=[]
        self.monkeynum=int(monkeydef[0].split(" ")[1].split(":")[0])
        self.inspectioncount=0
        self.operation=monkeydef[2].split(" ")[6]
        self.operand=monkeydef[2].split(" ")[7]
        if self.operation == "*" and self.operand == "old":
            self.operation = "**"
            self.operand = 2
        else:
            self.operand=int(monkeydef[2].split(" ")[7])
        self.divisor=int(monkeydef[3].split(" ")[5])
        if PART_FLAG == 2:
            DIVISOR_PRODUCT*= self.divisor
        self.truemonkey=int(monkeydef[4].split(" ")[9])
        self.falsemonkey=int(monkeydef[5].split(" ")[9])        
        parse=monkeydef[1].split(" ")
        [self.itemlist.append(Item(int(parse[i].split(",")[0]))) for i in range(4,len(parse))]
        return

    def DoInspect(self,thisitem):
        #calc new worry level (function pointers would be more elegant)
        if self.operation=="+":
            thisitem.SetWorry(thisitem.GetWorry() + self.operand)
        elif self.operation=="*":
            thisitem.SetWorry(thisitem.GetWorry() * self.operand)
        else:
            thisitem.SetWorry(thisitem.GetWorry() ** self.operand)
        thisitem.ApplyRelief()
        self.inspectioncount+=1
        return

    def GetInspectionCount(self):
        return self.inspectioncount
    
    def GetMonkeyItems(self):
        return(self.itemlist)

    def AddItems(self,newitemslist):
        for i in newitemslist:
            self.itemlist.append(i)
        return
    
    def ClearItems(self):
        self.itemlist=[]
        return

    def TakeTurn(self):
        ret=[[0,[]],[0,[]]]#return a list of lists, each list has a monkey num and a list of items
        ret[True][0]=self.truemonkey
        ret[False][0]=self.falsemonkey
        for i in self.GetMonkeyItems():
            self.DoInspect(i)
            retindex=i.GetWorry()//self.divisor==i.GetWorry()/self.divisor
            ret[retindex][1].append(i)
        self.ClearItems()
        return ret
        
class Barrel():  #did you know that a recognized collective noun for monkeys is barrel?
    def __init__(self,inputdata):
        #locals
        self.monkeys=[]
        i=0
        while i < len(inputdata):
            self.monkeys.append(Monkey([inputdata[j] for j in range(i,i+6)]))
            i+=7
        
    def DoRound(self):
        for i in self.monkeys:
            newitemlists=i.TakeTurn()
            for j in range(len(newitemlists)):
                self.monkeys[newitemlists[j][0]].AddItems(newitemlists[j][1])

    def InspectTotalList(self):
        return [self.monkeys[i].GetInspectionCount() for i in range(len(self.monkeys))]
          
PART_FLAG=1
DIVISOR_PRODUCT=1
def perform_parts(buffer):
    global PART_FLAG
    global DIVISOR_PRODUCT
    ret=[0,0,0]
    myBarrel=Barrel(buffer)
    [myBarrel.DoRound() for _ in range(20)]
    ilist=myBarrel.InspectTotalList()
    ilist.sort()
    ret[1]=ilist[-2] * ilist[-1]
    PART_FLAG=2
    myBarrel=Barrel(buffer)
    [myBarrel.DoRound() for _ in range(10000)]
    ilist=myBarrel.InspectTotalList()
    ilist.sort()
    ret[2]=ilist[-2] * ilist[-1]
    return(ret)
     
results=perform_parts(lines)
for i in range(1,3):
    print("part",i,"answer is",results[i])