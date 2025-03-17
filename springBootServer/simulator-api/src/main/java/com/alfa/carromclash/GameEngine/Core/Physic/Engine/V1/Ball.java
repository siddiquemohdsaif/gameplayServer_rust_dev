package com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1;


public class Ball {

    public Vector3 pos;
    public Vector3 vel;
    public Vector3 rvel;
    public State state;
    public Type type ;
    public int coinCode;
    public boolean pocket;

    public Ball(Vector3 pos, Vector3 vel, Vector3 rvel, State state,Type type,int coinCode) {
        this.pos = pos.__clone__();
        this.vel = vel.__clone__();
        this.rvel = rvel.__clone__();
        this.state = state;
        this.type = type;
        this.coinCode = coinCode;
    }


    public enum Type {
        Carrom,
        Striker
    }

    public enum State {
        Stationary,
        Rolling,
        Sliding,
        Falling,
        InPocket
    }

}
