package com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1;

public class Vector3 {

    public double x, y, z;
    public Vector3 (double x, double y, double z) {
        this.set(x, y, z);
    }

    public Vector3 set(double x, double y, double z) {
        this.x = x;
        this.y = y;
        this.z = z;
        return this;
    }
    public Vector3 copy (final Vector3 vector) {
        return this.set(vector.x, vector.y, vector.z);
    }

    public Vector3 __clone__() {
        return new Vector3(this.x,this.y,this.z);
    }

}
