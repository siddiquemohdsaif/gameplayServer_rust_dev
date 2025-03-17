package com.alfa.carromclash.GameEngine.Core;
import com.alfa.carromclash.GameEngine.Core.Model.PointD;

public class Carrom {



    // position
    public double x; // byGame
    public double y; // byGame
    public double rotationAngleInDegree = 0;
    public double depth; // in meter - (z axis)
    public Pocket isInPocket; //pocket or not

    //Identifier
    public final int coinCode; //id
    public final Type type; //id

    // animation
    public int carrom_drawable_id;
    public int isPotted;
    public boolean isEnable = true; //pocket of player
    public double transparency = 1;
    public int id = 0;



    public Carrom(int carrom_drawable_id, int id, int coinCode, Type type, double x, double y,int isPotted) {
        this.coinCode = coinCode;
        this.type = type;
        this.isPotted = isPotted;
        this.carrom_drawable_id = carrom_drawable_id;
        this.id = id;
        this.x = x;
        this.y = y;
    }


    public static enum Type{
        CARROM_MEN_1,
        CARROM_MEN_2,
        QUEEN
    }


    public static class Pocket {
        public boolean inPocket;
        public long pocketTime;

        public Pocket(boolean inPocket, long pocketTime) {
            this.inPocket = inPocket;
            this.pocketTime = pocketTime;
        }
    }


    public void activate(PointD endPoint){
        depth = 0;
        isInPocket = null;
        x = endPoint.x;
        y = endPoint.y;
    }


}
