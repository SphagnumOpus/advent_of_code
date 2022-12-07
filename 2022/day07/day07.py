#!/usr/bin/env python3 
# Advent of code 2022 day 07
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


def build_filesystem(buffer):
    #root directory is inode 0
    ret=[[{'Name':'..','Type':'dir','Size':0,'DirElement':0}]]
    linecounter=0
    while linecounter < len(buffer):
        parse=buffer[linecounter].split(" ")
        if parse[0]!='$':
            print("Error in processing... expect $ got ",parse[0])
            exit()
        else:
            if parse[1]=='cd':
                if parse[2]=='/':
                    pwd=0
                else:
                    for i in ret[pwd]:
                        if i['Type']=='dir' and i['Name']==parse[2]:
                            pwd=i['DirElement']
                            break
                linecounter+=1
            elif parse[1]=='ls':
                linecounter+=1
                while linecounter<len(buffer) and buffer[linecounter][0] != '$':
                    parse=buffer[linecounter].split(" ")
                    if parse[0]=='dir':
                        newDirElement=len(ret)
                        ret[pwd].append({'Name':parse[1],'Type':'dir','Size':0,'DirElement':newDirElement}) #add dir item to this directory
                        ret.append([{'Name':'..','Type':'dir','Size':0,'DirElement':pwd}]) # add new ret item for new directory
                    else:
                        ret[pwd].append({'Name':parse[1],'Type':'file','Size':int(parse[0])})
                    linecounter+=1
            else:
                print("error in parsing got",parse)
                exit()
    return(ret)

filesystem=build_filesystem(lines)

def calc_size(fs,dir_entry):
    counter=0
    if args.debug:
        print(dir_entry)
    if dir_entry['Type']=='file' or dir_entry['Name']=='..':
        counter = dir_entry['Size']
    elif dir_entry['Type']=='dir':
        counter=0
        for i in fs[dir_entry['DirElement']]:   #skip the .. entry in the directory
            counter+=calc_size(fs,i)      
    else:
        print("logic error in calc_size")
        exit()
    return(counter)

if args.debug:
    print('build filesystem =',filesystem)
    print('calc_size =',calc_size(filesystem,{'Type':'dir','Name':'/','Size':0,'DirElement':0}))

def perform_parts(fs,part):
    ret=0
    if part==2:
        total_used=calc_size(fs,{'Type':'dir','Name':'/','Size':0,'DirElement':0})
        space_needed=total_used-40000000
        ret=total_used
    for i in fs:
        for j in i:    #scan through each directory list
            if j['Type']=='dir':
                dirsize=calc_size(fs,j)
                if part==1:
                    if dirsize<=100000:
                        ret+=dirsize
                else:
                    if dirsize < ret and dirsize > space_needed:
                        ret=dirsize
    return(ret)


for i in range(1,3):
    print("part",i," answer is",perform_parts(filesystem,i))