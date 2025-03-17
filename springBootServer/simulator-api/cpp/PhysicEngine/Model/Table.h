#ifndef TABLE_H
#define TABLE_H
#include <vector>
#include "Ball.h"
#include "Outcome.h"
#include "../Util/Collision.h"
#include "../Util/Cushion.h"

class Table {
public:
    Table(const std::vector<Ball> balls);
    void advance(double t);
    bool allStationary();
    Ball* getBallByCoinId(int coinCode);
    std::vector<Ball> balls;
    std::vector<Outcome> outcomes;
    static bool deactivatePocket;
private:
    std::vector<std::pair<Ball*, Ball*>> pairs;
    void initialiseBalls(const std::vector<Ball> &balls);
    bool prepareAdvanceAll(double t);
    bool prepareAdvancePair(Ball &a, Ball &b, double t);
    bool prepareAdvanceToCushions(Ball &a, double t);
};

#endif // TABLE_H
