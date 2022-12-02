#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <queue>

using namespace std;

class Submarine {
  public:
    Submarine(int x, int y, int a) : x(x), y(y), aim(a) {}
    int y;
    int x;
    int aim;
    queue<int> depthCache;
    void incrementY(int num) {
        this->y += num;
    }
    void incrementX(int num) {
        this->x += num;
    }
    void incrementAim(int num) {
        this->aim += num;
    }
    void evaluateLocation(string direction, int d) {
        if (direction == "forward") {
            this->incrementX(d);
            this->incrementY(d * this->aim);
        }
        else if (direction == "up") {
            this->incrementAim(-d);
        }
        else if (direction == "down") {
            this->incrementAim(d);
        }
    }
};

int main(int argc, char* argv[]) {
    Submarine* sub = new Submarine(0, 0, 0);
    if (argc > 1) {
        ifstream fs(argv[1]);

        string line;
        int prev = 0;
        while (getline(fs, line)) {
            istringstream iss(line);
            string direction;
            int a;
            if (!(iss >> direction >> a)) { break; } // error

            iss >> direction >> a;
            sub->evaluateLocation(direction, a);
        }

    }
    printf("Location: %i\n", (sub->x * sub->y));
    return 0;
}
