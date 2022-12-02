#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <queue>

using namespace std;

class Submarine {
  public:
    Submarine(int d) {
        this->depth = d;
    }
    int depth;
    queue<int> depthCache;
    void incrementDepth() {
        this->depth++;
    }
    void evaluateDepth(int d) {
        if (depthCache.size() < 3) {
            depthCache.push(d);
        } else {
            int outgoing = depthCache.front();
            depthCache.pop();
            depthCache.push(d);
            if (outgoing < d) { this->incrementDepth(); }
        }
    }
};

int main(int argc, char* argv[]) {
    Submarine* sub = new Submarine(0);
    if (argc > 1) {
        ifstream fs(argv[1]);

        string line;
        int prev = 0;
        while (getline(fs, line)) {
            istringstream iss(line);
            int a;
            if (!(iss >> a)) { break; } // error

            iss >> a;
            sub->evaluateDepth(a);
        }

    }
    printf("Depth: %i\n", sub->depth);
    return 0;
}
