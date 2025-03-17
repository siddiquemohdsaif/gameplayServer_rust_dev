package com.alfa.carromclash.GameEngine.Core;

public class StrikerStatic {


    //rest position
    public double xR; // byGame
    public double yR; // byGame

    // position
    public volatile double x; // byGame
    public volatile double y; // byGame
    public double depth; // in meter - (z axis)
    public boolean isInPocket; //pocket or not

    public StrikerStatic( double x, double y){
        this.x = x;
        this.y = y;
        this.xR = x;
        this.yR = y;
    }

}
