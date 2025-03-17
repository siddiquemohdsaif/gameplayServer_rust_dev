// carrom_physics.cpp
#include "CoinsContainer.h"
#include <cmath>
#include <algorithm>

bool CoinsContainer::notOverlapsWithOthersCarroms(const std::vector<std::array<double, 3>>& allCarroms, const std::array<double, 3>& main) {
    for (const auto& carrom : allCarroms) {
        if (distanceBetweenCarrom(carrom, main) < CoinsContainer::ctcd) {
            return false;
        }
    }
    return true;
}

double CoinsContainer::distanceBetweenCarrom(const std::array<double, 3>& carrom1, const std::array<double, 3>& carrom2) {
    double xDiff = carrom2[0] - carrom1[0];
    double yDiff = carrom2[1] - carrom1[1];
    return std::sqrt(xDiff * xDiff + yDiff * yDiff);
}

std::vector<std::array<double, 3>> CoinsContainer::insertNewCarrom(std::vector<std::array<double, 3>>& carromArray, const std::array<double, 3>& carrom) {
    std::array<double, 3> carromNew = carrom;
    carromNew[0] = 0;
    carromNew[1] = 0;

    if (notOverlapsWithOthersCarroms(carromArray, carromNew)) {
        carromArray.push_back(carromNew);
        return carromArray;
    }

    bool inserted = false;
    double radius = 0.001; // 1mm

    while (!inserted) {
        for (int angle = 0; angle < 360; angle++) {
            carromNew[0] = radius * std::cos(angle * (3.14159265358979323846 / 180.0));
            carromNew[1] = radius * std::sin(angle * (3.14159265358979323846 / 180.0));

            if (notOverlapsWithOthersCarroms(carromArray, carromNew)) {
                inserted = true;
                carromArray.push_back(carromNew);
                break;
            }
        }
        radius += 0.001;
    }

    return carromArray;
}
