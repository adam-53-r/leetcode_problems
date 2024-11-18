#include <iostream>
#include <string>

bool isPalindrome(int x)
{

    std::string number1 = std::to_string(x);
    std::string number2 = "";

    for (int i = number1.size() - 1; i >= 0; i--)
    {
        number2.append( std::string( 1, number1[i] ) );
    }
    
    return number1 == number2;
}

int main()
{

    int number = -121;

    if (isPalindrome(number))
    {
        std::cout << "True";
    }
    else
    {
        std::cout << "False";
    }
    

    return 0;
}
