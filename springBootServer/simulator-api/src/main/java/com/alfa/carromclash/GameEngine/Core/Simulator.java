package com.alfa.carromclash.GameEngine.Core;


import com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1.Ball;
import com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1.Table;
import com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1.Vector3;
import com.alfa.carromclash.NativeBridge;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

public class Simulator {

    public double px;
    public double py;
    public double vx;
    public double vy;
    public boolean deactivatePocket;

    public Table table;
    public StrikerStatic strikerStatic;
    public static int strikerCoinCode = 99;
    public boolean isStimulationFinish = false;
    public boolean isStrikerPocketed = false;
    public double strikerRotationAngleInDegree = 0;
    public long lastTime;
    public CoinsContainer coinsContainer;
    public static double[] posX = new double[1000];
    public static int pointer = 0;
    public static double[] posY = new double[1000];

    
    // Map to record positions of all coins including striker by their coinCode
    public Map<Integer, ArrayList<Double>> coinPositionsHistory_x = new HashMap<>();
    public Map<Integer, ArrayList<Double>> coinPositionsHistory_y = new HashMap<>();

    public Simulator(double px, double py, double vx, double vy, StrikerStatic strikerStatic) {
        this.px = px;
        this.py = py;
        this.vx = vx;
        this.vy = vy;
        this.strikerStatic = strikerStatic;
        this.deactivatePocket = false;
    }

    public Simulator(double px, double py, double vx, double vy, StrikerStatic strikerStatic, boolean deactivatePocket) {
        this.px = px;
        this.py = py;
        this.vx = vx;
        this.vy = vy;
        this.strikerStatic = strikerStatic;
        this.deactivatePocket = deactivatePocket;
    }

    public void start(CoinsContainer coinsContainer) {
        ArrayList<Ball> carroms_balls = coinsContainer.getCarromBalls();

        Vector3 pos = new Vector3(px,py,0);
        Vector3 vel = new Vector3(vx,vy,0);
        Vector3 rvel = new Vector3(0,0,0);
        Ball.State state = Ball.State.Sliding;

        Ball ball = new Ball(pos,vel,rvel,state, Ball.Type.Striker,strikerCoinCode);
        ArrayList<Ball> balls = new ArrayList<>();
        balls.add(ball);
        balls.addAll(carroms_balls);


        this.table = new Table(balls);
        this.coinsContainer = coinsContainer;
        isStimulationFinish = false;

        // Initialize position recording
        initializeCoinPositions(balls);

        lastTime = System.currentTimeMillis();
        NativeBridge.startSimulation(this);
        this.stimulateToCurrentTime(this.lastTime);

        // SoundPlayerForGame.playStrikerReleaseSound();
    }

    private void initializeCoinPositions(ArrayList<Ball> balls) {
        for (Ball ball : balls) {
            coinPositionsHistory_x.put(ball.coinCode, new ArrayList<>());
            coinPositionsHistory_x.get(ball.coinCode).add(ball.pos.x);

            coinPositionsHistory_y.put(ball.coinCode, new ArrayList<>());
            coinPositionsHistory_y.get(ball.coinCode).add(ball.pos.y);
        }
    }


    private void recordPosition(Ball ball) {
        ArrayList<Double> positions_x = coinPositionsHistory_x.get(ball.coinCode);
        if (positions_x != null) {
            positions_x.add(ball.pos.x);
        }

        ArrayList<Double> positions_y = coinPositionsHistory_y.get(ball.coinCode);
        if (positions_y != null) {
            positions_y.add(ball.pos.y);
        }
    }
    

    public void stimulateToCurrentTime(long currentTime) {
        if (isStimulationFinish){
            return;
        }

        lastTime = currentTime;
        NativeBridge.stimulateToCurrentTime(this);
        updateUIBalls();

        if (isStimulationFinish){
            // SoundManager.stopAllSoundsStatic();
            // soundVolumes.clear();
        }

    }

   


    public boolean isStimulationFinish(){
        return isStimulationFinish;
    }

    public void stop(){
        isStimulationFinish = true;
    }


    private void updateUIBalls() {
        // striker
        Ball ball_striker = table.getBallByCoinId(strikerCoinCode);
        strikerStatic.x = ball_striker.pos.x;
        strikerStatic.y = ball_striker.pos.y;
        strikerStatic.depth = ball_striker.pos.z;
        strikerStatic.isInPocket = ball_striker.pocket;
        if (ball_striker.pocket){
            isStrikerPocketed = true;
        }

        if (Simulator.pointer < 1000) {
            posX[pointer] = strikerStatic.x;
            posY[pointer] = strikerStatic.y;
            pointer ++;
        }

        recordPosition(ball_striker);


        //rotation time ref
        long currentTime = System.currentTimeMillis();
        double deltaTime = (currentTime - lastTime) / 1000.0; // convert milliseconds to seconds
        double amplification = 360 * Math.sqrt(ball_striker.vel.x*ball_striker.vel.x + ball_striker.vel.y*ball_striker.vel.y);

        //striker rotation cal
        double angularVelocityDegreesPerSecond = Math.toDegrees(ball_striker.rvel.z);
        double deltaAngle = angularVelocityDegreesPerSecond * deltaTime*amplification;
        strikerRotationAngleInDegree += deltaAngle;
        strikerRotationAngleInDegree = (strikerRotationAngleInDegree + 360) % 360;


        //carroms
        for (Carrom carrom :coinsContainer.carroms) {
            Ball ball = table.getBallByCoinId(carrom.coinCode);
            carrom.x = ball.pos.x;
            carrom.y = ball.pos.y;
            carrom.depth = ball.pos.z;

            if (ball.pocket){
                carrom.isInPocket = new Carrom.Pocket(true,System.currentTimeMillis());
            }

            recordPosition(ball);


            angularVelocityDegreesPerSecond = Math.toDegrees(ball.rvel.z);
            amplification = 360 * Math.sqrt(ball.vel.x*ball.vel.x + ball.vel.y*ball.vel.y);
            deltaAngle = angularVelocityDegreesPerSecond * deltaTime * amplification;
            carrom.rotationAngleInDegree += deltaAngle;
            carrom.rotationAngleInDegree = (carrom.rotationAngleInDegree + 360) % 360;

        }

    }

}
