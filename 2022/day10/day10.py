#!/usr/bin/env python3 
# Advent of code 2022 day 10
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

class Cpu():
    def __init__(self):
        #locals
        self.x=1   #the register
        self.xhist=[0,0] #start with value "during" 0th cycle and value during 1st cycle
    
    
    def __repr__(self):
        ret = "CPU data"
        ret += f"x register contains: {self.x}\n"
        ret += f"Clock cycles elapsed: {len(self.xhist)-2}\n"
        return ret

    def ParseCode(self,cmdline):
        parse=cmdline.split(" ")
        if parse[0]=="noop":
            return(self.Noop())
        elif parse[0]=="addx":
            return(self.Addx(int(parse[1])))
        else:
            print(f"error, unknown code {cmdline}")
            exit()

    def Noop(self):
        self.xhist=self.xhist+[self.x]
        return self.x

    def Addx(self,operand):
        self.xhist=self.xhist+[self.x] #first clock cycle, no change
        self.x += operand
        self.xhist=self.xhist+[self.x]
        return self.x
    
    def Getx(self):
        return self.x

    def GetxDuring(self,clock):
        return self.xhist[clock]

def buildCRTimage(thiscpu):
    ret=f"\n"   #start with a newline to pretty the output
    for i in range(6):
        for j in range(40):
            index=1+i*40 + j #the added one is to account for starting on clock cycle 1, not zero
            thisx=thiscpu.GetxDuring(index)
            if abs(thisx-j)<2:
                ret=ret+"#"
            else:
                ret=ret+"."
        ret=ret+f"\n"
    return ret

 

    

def perform_parts(buffer):
    ret=[0,0,0]
    myCpu=Cpu()
    for i in buffer:
        myCpu.ParseCode(i)
    ret[1]=sum([(myCpu.GetxDuring(i) * i) for i in [20,60,100,140,180,220]])
    ret[2]=buildCRTimage(myCpu)
    return(ret)

     
results=perform_parts(lines)  
for i in range(1,3):
    print("part",i,"answer is",results[i])