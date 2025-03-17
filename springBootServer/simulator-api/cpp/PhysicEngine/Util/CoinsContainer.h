// carrom_physics.h
#ifndef CARROM_PHYSICS_H
#define CARROM_PHYSICS_H

#include <vector>
#include <array>

class CoinsContainer{
public:
    static std::vector<std::array<double, 3>> insertNewCarrom(std::vector<std::array<double, 3>>& carromArray, const std::array<double, 3>& carrom);
    static double distanceBetweenCarrom(const std::array<double, 3>& carrom1, const std::array<double, 3>& carrom2);
    static bool notOverlapsWithOthersCarroms(const std::vector<std::array<double, 3>>& allCarroms, const std::array<double, 3>& main);
    static constexpr double padding = 0.0006;                       // 0.6 mm
    static constexpr double ctcd = 2 * 0.0202565 + padding;         // carrom to carrom distance
    static constexpr double ctsd = 0.0202565 + 0.0202565 + padding; // carrom to striker distance
};


#endif // CARROM_PHYSICS_H
