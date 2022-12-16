#!/usr/bin/env python3 
# Advent of code 2022 day 15
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

def manhattan_distance(pos1,pos2):
    return(abs(pos2[0]-pos1[0])+abs(pos2[1]-pos1[1]))

class Tunnel():
    def __init__(self) -> None:
        self._tunnelmap={}
        self.upper_left_corner=(0,0)
        self.lower_right_corner=(0,0)
        self.sensor_list=[]

    def __repr__(self) -> str:
        ret=""
        for y in range(self.upper_left_corner[1],self.lower_right_corner[1]+1):
            for x in range(self.upper_left_corner[0],self.lower_right_corner[0]+1):
                ret=ret+self._tunnelmap.get((x,y),"#" if self.IsNoBeacon((x,y)) else ".")
            ret=ret+"\n"
        return ret

    def AddBeacon(self,position):
        self._tunnelmap[position]="B"
        self.ExtendBorders(position)
        return position

    def ExtendBorders(self,position):
        self.upper_left_corner=(min([self.upper_left_corner[0],position[0]]),min([self.upper_left_corner[1],position[1]]))
        self.lower_right_corner=(max([self.lower_right_corner[0],position[0]]),max([self.lower_right_corner[1],position[1]]))

    def MarkNoBeacon(self,position):
        if not self._tunnelmap.get(position):
            self._tunnelmap[position]="#"
            self.ExtendBorders(position)
        return
    
    def AddSensor(self,position,nearest_beacon):
        self.AddBeacon(nearest_beacon)
        beacon_distance=manhattan_distance(position,nearest_beacon)
        self.ExtendBorders((position[0]-beacon_distance,position[1] - beacon_distance))
        self.ExtendBorders((position[0]+beacon_distance,position[1]+beacon_distance))

        self._tunnelmap[position]="S"
        self.sensor_list.append([position,beacon_distance])

    def GetMinX(self):
        return self.upper_left_corner[0]

    def GetMaxX(self):
        return self.lower_right_corner[0]

    def IsOccupied(self,point):
        return(self._tunnelmap.get(point))

    def IsNoBeacon(self,point):
        (ret,_)=self.LastXCoordinate(point)
        if ret == None:
            ret=True
        return ret

    def LastXCoordinate(self,point):
        if self._tunnelmap.get(point):
            return (False,point[0]+1)
        for i in self.sensor_list:
            if manhattan_distance(point,i[0])<=i[1]:
                beaconx=i[0][0]
                beacony=i[0][1]
                pointy=point[1]
                rc=(True,beaconx+i[1]-abs(pointy-beacony)+1)
                break
        else:
            rc=(False,None)
        return rc



def build_tunnel(buffer):
    ret=Tunnel()
    for i in buffer:
        parse=i.split(" ")
        sensorx=int(parse[2].split("=")[1].split(",")[0])
        sensory=int(parse[3].split("=")[1].split(":")[0])
        beaconx=int(parse[8].split("=")[1].split(",")[0])
        beacony=int(parse[9].split("=")[1].split(":")[0])
        ret.AddSensor((sensorx,sensory),(beaconx,beacony))
    return ret

myTunnel=build_tunnel(lines)
if len(lines)<20:
    relevant_part1_row=10
    relevant_part2_upperlimit=20
else:
    relevant_part1_row=2000000
    relevant_part2_upperlimit=4000000
print ("Part 1 answer is",len([i for i in range(myTunnel.GetMinX(),myTunnel.GetMaxX()+1) if myTunnel.IsNoBeacon((i,relevant_part1_row))]))

def part2_answer(t,ul):
    for y in range(ul+1):
        x=0
        while x <= ul:
            if (not t.IsOccupied((x,y))) and (not t.IsNoBeacon((x,y))):
                return (x*4000000)+y
            else:
                (_,x)=t.LastXCoordinate((x,y))

print ("Part 2 answer is",part2_answer(myTunnel,relevant_part2_upperlimit))