#include <algorithm>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <iterator>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

using namespace std;

pair <char, char> getMostLeast(vector<string>* arr, int i) {
    int counter = 0;
    for (string el : *arr) {
        if (el != "") {
            if (el[i] == '1') counter++;
        }
    }
    return (counter * 2) >= arr->size() ? make_pair('1', '0') : make_pair('0', '1');
}

int calculateFromBString(string first, string second) {
    return stoi(first, 0, 2) * stoi(second, 0, 2);
}

void partOne(vector<string>* values) {
    string gamma = "";
    string epsilon = "";
    for (int i = 0; i < values->front().size(); i++) {
        auto [ g, e ] = getMostLeast(values, i);
        gamma += g;
        epsilon += e;
    }
    int result = calculateFromBString(gamma, epsilon);
    printf("Part 1: %i\n", result);
}

string recursiveFilter(vector<string> values, int i, string option) {
    if (values.size() == 1) return values[0];
    vector<string> newValues;
    auto res = getMostLeast(&values, i);
    copy_if(values.begin(), values.end(), back_inserter(newValues), [i, res, option](string element){
        char result = option == "most" ? res.first : res.second;
        return element[i] == result;
    });
    return recursiveFilter(newValues, ++i, option);
}

void partTwo(vector<string>* values) {
    string oxygen, co2;
    int i = 0;
    oxygen = recursiveFilter(*values, i, "most");
    co2 = recursiveFilter(*values, i, "least");
    int result = calculateFromBString(oxygen, co2);
    printf("Part 2: %i\n", result);
}

void readInput(ifstream *fs) {
    string line;
    vector<string> values = {};
    while (getline(*fs, line)) {
        istringstream iss(line);
        string a;
        if (!(iss >> a)) { break; } // error

        iss >> a;
        values.push_back(a);
    }
    partOne(&values);
    partTwo(&values);
}

int main(int argc, char* argv[]) {
    if (argc > 1) {
        ifstream fs(argv[1]);
        readInput(&fs);
    }
    return 0;
}
