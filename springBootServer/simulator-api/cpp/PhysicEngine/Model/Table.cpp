#include "Table.h"
#include "Pocket.h"
#include <stdexcept>

Table::Table(const std::vector<Ball> balls) {
    initialiseBalls(balls);
}

bool Table::deactivatePocket = false;

void Table::initialiseBalls(const std::vector<Ball> &balls) {
    this->balls = balls;
    for (size_t a = 0; a < balls.size(); ++a) {
        for (size_t b = 0; b < balls.size(); ++b) {
            if (a < b) {
                pairs.emplace_back(&this->balls[a], &this->balls[b]);
            }
        }
    }
}

void Table::advance(double t) {
    int depth = 0;
    while (!prepareAdvanceAll(t)) {
        if (depth++ > 100) {
            throw std::runtime_error("Depth exceeded resolving collisions");
        }
    }

    for (Ball &a : balls) {
        a.update(t);
    }
}

bool Table::prepareAdvanceAll(double t) {
    // collision ball-ball check
    for (auto &pair : pairs) {
        if (!prepareAdvancePair(*pair.first, *pair.second, t)) {
            return false;
        }
    }

    // collision wall-ball check
    for (Ball &ball : balls) {
        if (!prepareAdvanceToCushions(ball, t)) {
            return false;
        }
    }

    return true; // Returns true if all balls can advance by t without collision
}

bool Table::prepareAdvancePair(Ball &a, Ball &b, double t) {
    if (Collision::willCollide(a, b, t)) {
        double incidentSpeed = Collision::collide(a, b);
        outcomes.push_back(Outcome::collision(a, b, incidentSpeed));
        return false;
    }
    return true;
}

bool Table::prepareAdvanceToCushions(Ball &a, double t) {
    if (Cushion::willBounce(a, t)) {
        double incidentSpeed = Cushion::bounce(a, t);
        outcomes.push_back(Outcome::cushion(a, incidentSpeed));
        return false;
    }

    if (a.onTable()) {
        if(!deactivatePocket){
            Pocket *p = Pocket::willFallAny(a, t);
            if (p != nullptr) {
                double pocketIncidentSpeed = p->fall(a, t);
                outcomes.push_back(Outcome::pot(a, pocketIncidentSpeed));
                return false;
            }
        }
    }

    return true;
}

bool Table::allStationary() {
    bool allStationary = true;

    for (Ball &b : balls) {
        if (b.inMotion() || b.isFalling()) {
            allStationary = false;
        }
    }
    return allStationary;
}

Ball* Table::getBallByCoinId(int coinCode) {
    for (Ball &ball : balls) {
        if (ball.coinCode == coinCode) {
            return &ball;
        }
    }
    throw std::runtime_error("Unexpected state: 0x001"); // coinCode not found
}

