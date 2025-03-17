#ifndef BALL_H
#define BALL_H

#include "../Maths/Vector3.h"
#include "../Maths/Utils.h"
#include "../Util/Physics.h"
#include "../ConstantsCarrom.h"
#include "../ConstantsStriker.h"
#include "Pocket.h"

class Pocket; // Forward declaration

class Ball {
public:
    enum class Type {
        Carrom,
        Striker
    };

    enum class State {
        Stationary,
        Rolling,
        Sliding,
        Falling,
        InPocket
    };

    Vector3 pos;
    Vector3 vel;
    Vector3 rvel;
    State state;
    Type type;
    Vector3 futurePos;
    int coinCode;
    Pocket *pocket{nullptr};

    double transition{0.006};
    double velocityThreshold {0.004};
    double angularVelocityThreshold {0.004};


    double mu;
    double mur;
    double g;
    double rho;
    double m;
    double R;
    double Mz;
    double Mxy;
    double e;
    double I;

    Ball(const Vector3 &pos);
    Ball(const Vector3 &pos, const Vector3 &vel, const Vector3 &rvel, State state, Type type, int coinCode);
    ~Ball();

    void applyBallSpecificProperty();
    void update(double t);
    void updatePosition(double t);
    void updateFalling(double t);
    void updateVelocity(double t);
    void updateVelocityRolling(double t);
    void updateVelocitySliding(double t);
    void setStationary();
    bool isRolling();
    bool isSurfaceVelocityMinimum();
    double getFutureSurfaceVelocity(double t);
    bool onTable() const;
    bool inMotion() const;
    bool isFalling() const;
    Vector3 futurePosition(double t);

private:
    Vector3 dv;
    Vector3 dw;
};

#endif // BALL_H
