#include "Simulator.h"
#include <cmath>

Simulator::Simulator(double px, double py, double vx, double vy,uint64_t last) :
    px(px), py(py), vx(vx), vy(vy), table(nullptr), StimulationFinish(false), last(last) {}

Simulator::~Simulator() {

    for (Ball &ball : table->balls) {  // call one at simulation end
        if (ball.pocket != nullptr){
            delete ball.pocket;
            ball.pocket = nullptr;
        }
    }

    if (table != nullptr) {
        delete table;
        table = nullptr;
    }
}

void Simulator::start(std::vector<Ball> &carroms_balls) {
    Vector3 pos(px, py, 0);
    Vector3 vel(vx, vy, 0);
    Vector3 rvel(0, 0, 0);
    Ball::State state = Ball::State::Sliding;

    Ball ball(pos, vel, rvel, state, Ball::Type::Striker, strikerCoinCode);
    std::vector<Ball> balls{ball};
    balls.insert(balls.end(), carroms_balls.begin(), carroms_balls.end());

    table = new Table(balls);
    StimulationFinish = false;
    stimulateToCurrentTime(last);
}

std::vector<Ball> Simulator::stimulateToCurrentTime(uint64_t timestamp) {
    if (StimulationFinish) {
        return table->balls;
    }

    advance((timestamp - last) / 1000.0);
    last = timestamp;

    if (table->allStationary()) {
        StimulationFinish = true;
    }

    return table->balls;
}

void Simulator::advance(double elapsed) {
    double steps = std::floor(elapsed / step);
    for (int i = 0; i < steps; ++i) {
        table->advance(step);
        if (table->allStationary()) {
            return;
        }
    }
}

bool Simulator::isStimulationFinish() const {
    return StimulationFinish;
}

void Simulator::stop() {
    StimulationFinish = true;
}
