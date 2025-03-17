#ifndef VECTOR3_H
#define VECTOR3_H

#include <string> // Required for std::string

class Vector3
{
public:
    double x, y, z;

    Vector3();
    Vector3(double x, double y, double z);
    Vector3(const Vector3 &vector);

    std::string toString();

    Vector3 &set(double x, double y, double z);
    Vector3 &copy(const Vector3 &vector);
    Vector3 clone() const;
    Vector3 &setZ(double z);
    Vector3 &cross(const Vector3 &vector);
    Vector3 &cross(double x, double y, double z);
    Vector3 &normalize();
    Vector3 &scale(double scalar);
    double lengthSq() const;
    double length() const;
    Vector3 &add(const Vector3 &vector);
    Vector3 &add(double x, double y, double z);
    double dot(const Vector3 &vector) const;
    double dot(double x, double y, double z) const;
    Vector3 &addScaledVector(const Vector3 &vec, double scalar);
    Vector3 &multiplyScalar(double scalar);
    Vector3 &sub(const Vector3 &vec);
    Vector3 &sub(double x, double y, double z);
    double distanceTo(const Vector3 &vector) const;
    double distanceTo(double x, double y, double z) const;
    double distanceToSquared(const Vector3 &point) const;
    double distanceToSquared(double x, double y, double z) const;
    Vector3 &applyAxisAngle(const Vector3 &axis, double radians);
};

#endif // VECTOR3_H
