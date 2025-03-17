#include "Utils.h"

Vector3 Utils::zero(0, 0, 0);
Vector3 Utils::up(0, 0, 1);

Vector3 Utils::vec(const Vector3& v) {
    return Vector3(v.x, v.y, v.z);
}

Vector3 Utils::upCross(const Vector3& v) {
    Vector3 clonedUp = Utils::up.clone();
    return clonedUp.cross(v);
}

Vector3 Utils::norm(const Vector3& v) {
    Vector3 clonedV = v.clone();
    return clonedV.normalize();
}

bool Utils::passesThroughZero(const Vector3& v, const Vector3& dv) {
    Vector3 clonedV = v.clone();
    clonedV.add(dv);
    return clonedV.dot(v) <= 0;
}
