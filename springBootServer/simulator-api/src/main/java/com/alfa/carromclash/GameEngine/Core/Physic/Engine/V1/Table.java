package com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1;


import java.util.ArrayList;

public class Table {

    public ArrayList<Ball> balls = new ArrayList<>();

    public Table(ArrayList<Ball> balls) {
        this.balls = balls;
    }

    public Ball getBallByCoinId(int coinCode) {
        for (Ball ball:balls) {
            if (ball.coinCode == coinCode){
                return ball;
            }
        }
        throw new RuntimeException("Unexpected state: 0x001");  // coinCode not found
    }

}
