// Advent of code 2023 day 01
// Author:  Bill Moss
using System;
using System.ComponentModel;
using System.ComponentModel.DataAnnotations;
using System.Diagnostics.CodeAnalysis;
using System.Diagnostics.Contracts;
using System.IO;
using Microsoft.Win32.SafeHandles;

namespace Day01NS
{
    class Day01
    {
        static void Main(string[] args)
        {
            //Assume argv[1] is a filename and open it, and read it into a buffer
            List<string> inputBuffer = new List<string>();
            using (var inputFile= new StreamReader(args[0]))
            {
                string? buf = inputFile.ReadLine();
                while(buf is not null)
                {
                    inputBuffer.Add(buf);
                    buf = inputFile.ReadLine();
                }
            }
            int part1Sum=0;
            int part2Sum=0;
            for (int i=0;i < inputBuffer.Count;i++){
                part1Sum += FindPart1Num(inputBuffer[i]);
                part2Sum += FindPart2Num(inputBuffer[i]);
            }
            System.Console.Write("Part 1 answer is:");
            System.Console.WriteLine(part1Sum);
            System.Console.Write("Part 2 answer is:");
            System.Console.WriteLine(part2Sum);

        }
        static int FindPart1Num(string inString){
            string onlyNumbers = string.Concat( inString.Where(Char.IsDigit) );
            char [] charNumbers = onlyNumbers.ToCharArray(0,onlyNumbers.Length);
            int returnValue=0;
            if (charNumbers.Length>0){
                returnValue = 10 * (int)Char.GetNumericValue(charNumbers[0]) + (int) Char.GetNumericValue(charNumbers[charNumbers.Length-1]);
            }
            else {
                returnValue = 0;
            }           
            return returnValue;
        }
        static int FindPart2Num(string inString){
            string transformedString="";
            for (int i=0;i< inString.Length;i++){
                transformedString=transformedString+InsertNumeric(inString.Substring(i))+inString.Substring(i,1);
            }
            return(FindPart1Num(transformedString));
        }
        static string InsertNumeric(string inString){
            string[] numNameArray={"one","two","three","four","five","six","seven","eight","nine"};
            string[] numNumArray={"1","2","3","4","5","6","7","8","9"};
            for (int i=0;i<numNameArray.Length;i++){
                if (String.Equals(inString.Substring(0,Math.Min(numNameArray[i].Length,inString.Length)),numNameArray[i])){
                    return numNumArray[i]+inString.Substring(0,1);
                }
            }
            return(inString.Substring(0,1));
        }
    }
}