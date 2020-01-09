#include <iostream>
#include <iomanip>
#include <fstream>
#include <cstring>


using namespace std;


class position
{
    public :
    int x;
    int y;
    position() {
        x = 0;
        y = 0;
    }
    position(int px, int py) {
        x = px;
        y = py;
    }
};


int calc_manhattan(position p1, position p2) {
    return abs(p1.x - p2.x) + abs(p1.y - p2.y);
}

#define POSISTIONS (150000)

int main()
{
    ifstream inFile;
    char tmp[100000];

    inFile.open("input.txt");

    if (!inFile) {
        cout << "Unable to open file\n";
        exit(1);
    }
    int total_length = 0;
    int x = 0;
    int y = 0;
    position first[POSISTIONS];
    position second[POSISTIONS];
    position *pointer = &first[0];
    int nbr_first = 0;
    int nbr_second = 0;

    while (inFile.getline(tmp, 100000)) {
        int index = 1;
        std::stringstream ss(tmp);
        std::string token;
        while (std::getline(ss, token, ',')) {
            // cout << token[0] << " ";
            // cout << &token[1] << " ; ";
            int dist = stoi(&token[1]);
            switch (token[0]) {
                case 'U' :
                for (int i = 0; i < dist; i++) {
                    y++;
                    pointer[index].x = x;
                    pointer[index].y = y;
                    index++;
                }
                break;
                case 'D' :
                for (int i = 0; i < dist; i++) {
                    y--;
                    pointer[index].x = x;
                    pointer[index].y = y;
                    index++;
                }
                break;
                case 'R' :
                for (int i = 0; i < dist; i++) {
                    x++;
                    pointer[index].x = x;
                    pointer[index].y = y;
                    index++;
                }
                break;
                case 'L' :
                for (int i = 0; i < dist; i++) {
                    x--;
                    pointer[index].x = x;
                    pointer[index].y = y;
                    index++;
                }
                break;
            }
        }
        if (nbr_first == 0) {
            nbr_first = index;
        } else {
            nbr_second = index;
        }
        pointer = &second[0];
        index = 1;
        x = 0;
        y = 0;
    }
    position home(0,0);
    int closest = INT32_MAX;
    for (int i = 1; i < nbr_first; i++) {
        for (int j = 1; j < nbr_second; j++) {
            if (first[i].x == second[j].x && first[i].y == second[j].y) {
                // int dist = calc_manhattan(home, first[i]);
                int dist = i + j;
                if (dist < closest) {
                    closest = dist;
                }
            }
        }
    }
    cout << "closest " << closest;
    inFile.close();
    cout << endl;
    return 0;
}