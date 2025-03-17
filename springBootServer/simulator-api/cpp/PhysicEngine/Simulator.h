#ifndef SIMULATOR_H
#define SIMULATOR_H
#include <vector>
#include <ctime>
#include "Model/Ball.h"
#include "Model/Table.h"
#include "Maths/Vector3.h"

class Simulator {
public:
    double px, py, vx, vy;
    Table *table;
    static const int strikerCoinCode = 99;
    Simulator(double px, double py, double vx, double vy, uint64_t last);
    ~Simulator();
    void start(std::vector<Ball> &carroms_balls);
    std::vector<Ball> stimulateToCurrentTime(uint64_t timestamp);
    bool isStimulationFinish() const;
    void stop();

private:

    uint64_t last;
    bool StimulationFinish;
    const double step = 0.001;
    void advance(double elapsed);
};

#endif // SIMULATOR_H
