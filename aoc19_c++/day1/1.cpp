#include <iostream>
#include <iomanip>
#include <fstream>

using namespace std;
int main()
{
    int sum = 0;
    int x;
    int module;
    ifstream inFile;

    inFile.open("input.txt");
    if (!inFile) {
        cout << "Unable to open file\n";
        exit(1);
    }
    while (inFile >> x) {
        module =(int) (x / 3);
        module -= 2;
        int extra_weight = module;
        int extra_fuel = 0;
        int tmp;
        do {
            tmp = (int) (extra_weight / 3);
            tmp -= 2;
            if (tmp >= 0) {
                extra_weight = tmp;
                extra_fuel += tmp;
            }
        } while (tmp > 0);

        sum += (module + extra_fuel);
    }
    inFile.close();
    cout << sum << endl;

// 1577012 is too low

    return 0;
}