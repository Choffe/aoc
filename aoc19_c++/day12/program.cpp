#include <iostream>
#include <iomanip>
#include <fstream>
#include <vector>
#include <sstream>
#include <bits/stdc++.h>

using namespace std;

class Moon {
    public :
    int x = 0;
    int y = 0;
    int z = 0;
    int vel_x = 0;
    int vel_y = 0;
    int vel_z = 0;

    Moon(int _x, int _y, int _z) {
        x = _x;
        y = _y;
        z = _z;
    }

    string print_coord() {
        ostringstream oss;
        oss << x << "," << y << "," << z << ":" << vel_x << "," << vel_y << "," << vel_z << "|";
        return  oss.str();
    }


    int total_energy() {
        return ((abs(x) + abs(y) + abs(z)) * (abs(vel_x) + abs(vel_y) + abs(vel_z)));
    }
};

void update_gravity(vector<Moon>& milkyway)
{
    for (int i = 0; i < 4; i++) {
        Moon& first = milkyway.at(i);
        for (int j = i + 1; j < 4; j++) {
            Moon& second = milkyway.at(j);
            if (first.x > second.x) {
                first.vel_x -= 1;
                second.vel_x += 1;
            } else if (first.x < second.x) {
                first.vel_x += 1;
                second.vel_x -= 1;
            }

            if (first.y > second.y) {
                first.vel_y -= 1;
                second.vel_y += 1;
            } else if (first.y < second.y) {
                first.vel_y += 1;
                second.vel_y -= 1;
            }

            if (first.z > second.z) {
                first.vel_z -= 1;
                second.vel_z += 1;
            } else if (first.z < second.z) {
                first.vel_z += 1;
                second.vel_z -= 1;
            }
        }
    }
}
void update_gravity_fast(Moon& a, Moon& b, Moon& c, Moon& d)
{
    for (int i = 0; i < 4; i++) {
        Moon& first = a;
        if (i == 1) {
            first = b;
        }
        if (i == 2) {
            first = c;
        }
        if (i == 3) {
            first = d;
        }
        for (int j = i + 1; j < 4; j++) {
            Moon& second = b;
            if (j == 2) {
                second = c;
            }
            if (j == 3) {
                second = d;
            }
            if (first.x > second.x) {
                first.vel_x -= 1;
                second.vel_x += 1;
            } else if (first.x < second.x) {
                first.vel_x += 1;
                second.vel_x -= 1;
            }

            if (first.y > second.y) {
                first.vel_y -= 1;
                second.vel_y += 1;
            } else if (first.y < second.y) {
                first.vel_y += 1;
                second.vel_y -= 1;
            }

            if (first.z > second.z) {
                first.vel_z -= 1;
                second.vel_z += 1;
            } else if (first.z < second.z) {
                first.vel_z += 1;
                second.vel_z -= 1;
            }
        }
    }
}

void update_velocity(vector<Moon>& milkyway)
{
    for (int i = 0; i < milkyway.size(); i++) {
        Moon& m = milkyway.at(i);
        m.x += m.vel_x;
        m.y += m.vel_y;
        m.z += m.vel_z;
    }
}

string get_hash(vector<Moon>& milkyway)
{
    ostringstream oss;
    for (Moon& m : milkyway) {
        // oss << m.print_coord();
        oss << m.x << "," << m.vel_x;
    }
    return oss.str();
}
//4686774924

int main()
{
    vector<Moon> milkyway;
    //first
// <x=-1, y=0, z=2>
// <x=2, y=-10, z=-7>
// <x=4, y=-8, z=8>
// <x=3, y=5, z=-1>
    // milkyway.push_back(Moon(-1,0,2));
    // milkyway.push_back(Moon(2,-10,-7));
    // milkyway.push_back(Moon(4,-8,8));
    // milkyway.push_back(Moon(3,5,-1));

    //real
// <x=-16, y=15, z=-9>
// <x=-14, y=5, z=4>
// <x=2, y=0, z=6>
// <x=-3, y=18, z=9>
    milkyway.push_back(Moon(-16,15,-9));
    milkyway.push_back(Moon(-14,5,4));
    milkyway.push_back(Moon(2,0,6));
    milkyway.push_back(Moon(-3,18,9));

// <x=-8, y=-10, z=0>
// <x=5, y=5, z=10>
// <x=2, y=-7, z=3>
// <x=9, y=-8, z=-3>

    // milkyway.push_back(Moon(-8,-10,0));
    // milkyway.push_back(Moon(5,5,10));
    // milkyway.push_back(Moon(2,-7,3));
    // milkyway.push_back(Moon(9,-8,-3));
    // Moon& a = milkyway.at(0);
    // Moon& b = milkyway.at(1);
    // Moon& c = milkyway.at(2);
    // Moon& d = milkyway.at(3);

    unordered_set<string> s;

    // cout << "start gravity" << endl;
    // for (int i = 0; i < 100000000; i++) {
    //     // update_gravity(milkyway);
    //     update_gravity_fast(a, b, c, d);
    // }
    // cout << "start velocity" << endl;
    // for (int i = 0; i < 10000000; i++) {
    //     update_velocity(milkyway);
    // }
    // cout << "start hash" << endl;
    // for (int i = 0; i < 10000000; i++) {
    //     get_hash(milkyway);
    // }
    // cout << "measure done" << endl;

    int size = 0;
    while(size < 100000000000) {
        update_gravity(milkyway);
        update_velocity(milkyway);
        string h = get_hash(milkyway);
        s.insert(h);
        if (s.size() == size) {
            cout << size << endl;
            return 0;
        }
        if (size % 10000000 == 0) {
            cout << "1/4" <<endl;
        }
        size++;
    }


    cout << "done";
    cout << endl;
    return 0;
}