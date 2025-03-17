#include <cmath>
#include "Vector3.h"
#include <iostream>
#include <sstream> // include the <sstream> header file for stringstream

Vector3::Vector3() : x(0), y(0), z(0) {}

Vector3::Vector3(double x, double y, double z) : x(x), y(y), z(z) {}

Vector3::Vector3(const Vector3 &vector) : x(vector.x), y(vector.y), z(vector.z) {}

std::string Vector3::toString()
{
    std::stringstream ss;
    ss << "(" << x << ", " << y << ", " << z << ")";
    return ss.str();
}

Vector3 &Vector3::set(double x, double y, double z)
{
    this->x = x;
    this->y = y;
    this->z = z;
    return *this;
}

Vector3 &Vector3::copy(const Vector3 &vector)
{
    return this->set(vector.x, vector.y, vector.z);
}

Vector3 Vector3::clone() const
{
    return Vector3(*this);
}

Vector3 &Vector3::setZ(double z)
{
    this->z = z;
    return *this;
}

Vector3 &Vector3::cross(const Vector3 &vector)
{
    return this->set(y * vector.z - z * vector.y, z * vector.x - x * vector.z, x * vector.y - y * vector.x);
}

Vector3 &Vector3::cross(double x, double y, double z)
{
    return this->set(this->y * z - this->z * y, this->z * x - this->x * z, this->x * y - this->y * x);
}

Vector3 &Vector3::normalize()
{
    double len2 = this->lengthSq();
    if (len2 == 0 || len2 == 1)
        return *this;
    return this->scale(1 / std::sqrt(len2));
}

Vector3 &Vector3::scale(double scalar)
{
    return this->set(this->x * scalar, this->y * scalar, this->z * scalar);
}

double Vector3::lengthSq() const
{
    return x * x + y * y + z * z;
}

double Vector3::length() const
{
    return std::sqrt(x * x + y * y + z * z);
}

Vector3 &Vector3::add(const Vector3 &vector)
{
    return this->add(vector.x, vector.y, vector.z);
}

Vector3 &Vector3::add(double x, double y, double z)
{
    return this->set(this->x + x, this->y + y, this->z + z);
}

double Vector3::dot(const Vector3 &vector) const
{
    return x * vector.x + y * vector.y + z * vector.z;
}

double Vector3::dot(double x, double y, double z) const
{
    return this->x * x + this->y * y + this->z * z;
}

Vector3 &Vector3::addScaledVector(const Vector3 &vec, double scalar)
{
    this->x += vec.x * scalar;
    this->y += vec.y * scalar;
    this->z += vec.z * scalar;
    return *this;
}

Vector3 &Vector3::multiplyScalar(double scalar)
{
    return this->set(this->x * scalar, this->y * scalar, this->z * scalar);
}

Vector3 &Vector3::sub(const Vector3 &vec)
{
    return this->sub(vec.x, vec.y, vec.z);
}

Vector3 &Vector3::sub(double x, double y, double z)
{
    return this->set(this->x - x, this->y - y, this->z - z);
}

double Vector3::distanceTo(const Vector3 &vector) const
{
    double a = vector.x - x;
    double b = vector.y - y;
    double c = vector.z - z;
    return std::sqrt(a * a + b * b + c * c);
}

double Vector3::distanceTo(double x, double y, double z) const
{
    double a = x - this->x;
    double b = y - this->y;
    double c = z - this->z;
    return std::sqrt(a * a + b * b + c * c);
}

double Vector3::distanceToSquared(const Vector3 &point) const
{
    double a = point.x - x;
    double b = point.y - y;
    double c = point.z - z;
    return a * a + b * b + c * c;
}

double Vector3::distanceToSquared(double x, double y, double z) const
{
    double a = x - this->x;
    double b = y - this->y;
    double c = z - this->z;
    return a * a + b * b + c * c;
}

Vector3& Vector3::applyAxisAngle(const Vector3 &axis, double radians) {
    double halfAngle = radians * 0.5;
    double s = sin(halfAngle);
    double c = cos(halfAngle);

    double qX = axis.x * s;
    double qY = axis.y * s;
    double qZ = axis.z * s;
    double qW = c;

    double ix = qW * x + qY * z - qZ * y;
    double iy = qW * y + qZ * x - qX * z;
    double iz = qW * z + qX * y - qY * x;
    double iw = -qX * x - qY * y - qZ * z;

    x = ix * qW + iw * -qX + iy * -qZ - iz * -qY;
    y = iy * qW + iw * -qY + iz * -qX - ix * -qZ;
    z = iz * qW + iw * -qZ + ix * -qY - iy * -qX;

    return *this;
}

