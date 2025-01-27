#include <iostream>
#include <map>

int romanToInt(std::string s)
{

    std::map<std::string, int> roman_numbers;
    roman_numbers["I"] = 1;
    roman_numbers["V"] = 5;
    roman_numbers["X"] = 10;
    roman_numbers["L"] = 50;
    roman_numbers["C"] = 100;
    roman_numbers["D"] = 500;
    roman_numbers["M"] = 1000;


}

int main()
{
    std::string roman_runmber = "IVX";

    return 0;
}
