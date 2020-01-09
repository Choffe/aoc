#include <iostream>
#include <iomanip>
#include <fstream>
#include <cstring>

using namespace std;

int check_valid(int x) {
    int a = ((int)(x * 0.00001));
    int b = ((int)(x * 0.0001)) % 10;
    int c = ((int)(x * 0.001)) % 10;
    int d = ((int)(x * 0.01)) % 10;
    int e = ((int)(x * 0.1)) % 10;
    int f = x % 10;
    int increase = a <= b && b <= c && c <= d && d <= e && e <= f;

    int duplicate = (a == b && b != c) || (a != b && b == c && c != d) || (b != c && c == d && d != e) || (c != d && d == e && e != f) || (d != e && e == f);

    return increase && duplicate;
}

int main()
{
    int start = 256310;
    int end = 732736;
    int nbr_passwords = 0;
    for(int i = start; i < end; i++) {
        //check increasing digits
        if (check_valid(i)) {
            nbr_passwords++;
        }
    }
    cout << nbr_passwords;
    cout << endl;
}