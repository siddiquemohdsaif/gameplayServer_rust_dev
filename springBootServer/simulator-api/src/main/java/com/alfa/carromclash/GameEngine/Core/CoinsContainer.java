package com.alfa.carromclash.GameEngine.Core;

import com.alfa.carromclash.GameEngine.Core.Model.PointD;
import com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1.Ball;
import com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1.ConstantsCarrom;
import com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1.ConstantsStriker;
import com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1.Vector3;
import com.alfa.carromclash.NativeBridge;

import java.util.ArrayList;

public class CoinsContainer {

    public static double padding = 0.0006d; //0.6 mm
    public static double ctcd = 2d* ConstantsCarrom.R + padding; //carrom to carrom distance
    public static double ctsd = ConstantsCarrom.R+ ConstantsStriker.R + padding; //carrom to striker distance

    public ArrayList<Carrom> carroms;

    public CoinsContainer(ArrayList<Carrom> carroms) {
        this.carroms = carroms;
    }

    public void insertCarrom(Carrom carrom){
        PointD startPoint = new PointD(carrom.x,carrom.y);
        insertNewCarromToCarromArrayByAnimation(carroms,carrom,startPoint);
    }

    public void restCarrom(Carrom carrom) {
        for (int i=0; i<carroms.size(); i++){
            if (carrom.coinCode == carroms.get(i).coinCode){
                carroms.remove(i);
                insertNewCarromToCarromArray(carroms,carrom);
                return;
            }
        }
    }

    public ArrayList<Ball> getCarromBalls() {
        ArrayList<Ball> balls = new ArrayList<>();

        for (Carrom carrom: carroms) {

            Vector3 pos = new Vector3(carrom.x,carrom.y,0);
            Vector3 vel = new Vector3(0,0,0);
            Vector3 rvel = new Vector3(0,0,0);
            Ball.State state = Ball.State.Stationary;

            Ball ball = new Ball(pos,vel,rvel,state, Ball.Type.Carrom,carrom.coinCode);
            balls.add(ball);
        }

        return balls;
    }

    public void remove(ResultPlay resultPlay) {
        //hisPocketedCoins removed
        for (int i=0; i < resultPlay.hisPocketedCoins.size() ;i++) {
            for (int j=0; j<carroms.size(); j++){
                if (carroms.get(j).coinCode == resultPlay.hisPocketedCoins.get(i).coinCode){
                    carroms.remove(j);
                    break;
                }
            }
        }

        //opponentPocketedCoins removed
        for (int i=0; i < resultPlay.opponentPocketedCoins.size() ;i++) {
            for (int j=0; j<carroms.size(); j++){
                if (carroms.get(j).coinCode == resultPlay.opponentPocketedCoins.get(i).coinCode){
                    carroms.remove(j);
                    break;
                }
            }
        }
    }


    public static CoinsContainer getPractiseMatchArrangement(ArrayList<Carrom> p1Carrom, ArrayList<Carrom> p2Carrom, Carrom queenCarrom){
        ArrayList<Carrom> allCarroms = new ArrayList<>();

        //set queen
        queenCarrom.x = 0;
        queenCarrom.y = 0;
        allCarroms.add(queenCarrom);

        ///////////////////////////////////////////////////////////////////////////////////
        //and 120-degree -> 6 p1Carrom
        double thirty_degree = 30d*Math.PI/180d;
        //1
        p1Carrom.get(0).x = 0; p1Carrom.get(0).y = ctcd;
        allCarroms.add(p1Carrom.get(0));
        p1Carrom.remove(0);
        //2
        p1Carrom.get(0).x = 0; p1Carrom.get(0).y = 2* ctcd;
        allCarroms.add(p1Carrom.get(0));
        p1Carrom.remove(0);

        //3
        p1Carrom.get(0).x = ctcd *Math.cos(thirty_degree); p1Carrom.get(0).y = -ctcd *Math.sin(thirty_degree);
        allCarroms.add(p1Carrom.get(0));
        p1Carrom.remove(0);
        //4
        p1Carrom.get(0).x = 2* ctcd *Math.cos(thirty_degree); p1Carrom.get(0).y = -2* ctcd *Math.sin(thirty_degree);
        allCarroms.add(p1Carrom.get(0));
        p1Carrom.remove(0);

        //5
        p1Carrom.get(0).x = -ctcd *Math.cos(thirty_degree); p1Carrom.get(0).y = -ctcd *Math.sin(thirty_degree);
        allCarroms.add(p1Carrom.get(0));
        p1Carrom.remove(0);
        //6
        p1Carrom.get(0).x = -2* ctcd *Math.cos(thirty_degree); p1Carrom.get(0).y = -2* ctcd *Math.sin(thirty_degree);
        allCarroms.add(p1Carrom.get(0));
        p1Carrom.remove(0);
        ///////////////////////////////////////////////////////////////////////////////////


        //push all nine p2Carrom in array-carrom by lowest distance from centre of board
        for (Carrom carrom:p2Carrom) {
            insertNewCarromToCarromArray(allCarroms,carrom);
        }
        p2Carrom.clear(); // remove all carrom ie already pushed to board



        //push all remaining p1Carrom in array-carrom by lowest distance from centre of board
        for (Carrom carrom:p1Carrom) {
            insertNewCarromToCarromArray(allCarroms,carrom);
        }
        p1Carrom.clear(); // remove all carrom ie already pushed to board



        //rotate carrom
        double angleRad = Math.toRadians(100);
        for (Carrom carrom : allCarroms) {
            float newX = (float) (carrom.x * Math.cos(angleRad) - carrom.y * Math.sin(angleRad));
            float newY = (float) (carrom.x * Math.sin(angleRad) + carrom.y * Math.cos(angleRad));
            // Update the carrom's position
            carrom.x = newX;
            carrom.y = newY;
        }


        return new CoinsContainer(allCarroms);
    }

    public static void insertNewCarromToCarromArray(ArrayList<Carrom> allCarroms, Carrom carromNew) {

        //check a COIN ALREADY IN BOARD
        for (Carrom carrom: allCarroms) {
            if (carrom.coinCode == carromNew.coinCode){
                throw new RuntimeException("Unexpected state: 0x005");  // same coin Insert on board
            }
        }

        // Convert ArrayList<Carrom> allCarroms to double[][3]
        double[][] allCarromsArray = new double[allCarroms.size()][3];
        for (int i = 0; i < allCarroms.size(); i++) {
            Carrom carrom = allCarroms.get(i);
            allCarromsArray[i][0] = carrom.x;
            allCarromsArray[i][1] = carrom.y;
            allCarromsArray[i][2] = carrom.coinCode;
        }

        // Convert Carrom carromNew to double[3]
        double[] carromNewArray = new double[]{carromNew.x, carromNew.y, carromNew.coinCode};

        // Call the native function
        double[][] updatedCarroms = NativeBridge.insertNewCarrom(allCarromsArray, carromNewArray);

        // Process the returned array
        double x =0;
        double y =0;
        boolean found = false;
        for (double[] carromData : updatedCarroms) {
            if ((int)carromData[2] == carromNew.coinCode){
                found = true;
                x = carromData[0];
                y = carromData[1];
            }
        }
        if (found){
            // insert new carrom
            carromNew.x = x;
            carromNew.y = y;
            allCarroms.add(carromNew);
        }
    }


    public static void insertNewCarromToCarromArrayByAnimation(ArrayList<Carrom> allCarroms, Carrom carromNew, PointD startPoint){

        insertNewCarromToCarromArray(allCarroms,carromNew);

    }

    public static boolean notOverlapsWithOthersCarroms(ArrayList<Carrom> allCarroms, Carrom main) {
        for (Carrom carrom: allCarroms) {
            if (distanceBetweenCarrom(carrom,main) < ctcd){
                return false;
            }
        }
        return true;
    }

    public static boolean notOverlapsWithOthersCarromsStriker(ArrayList<Carrom> allCarroms, StrikerStatic strikerStatic) {
        for (Carrom carrom: allCarroms) {
            if (distanceBetweenCarromStriker(carrom,new PointD(strikerStatic.x,strikerStatic.y)) < ctsd){
                return false;
            }
        }
        return true;
    }

    public static boolean notOverlapsWithOthersCarromsStriker(ArrayList<Carrom> allCarroms, PointD strikerStatic) {
        for (Carrom carrom: allCarroms) {
            if (distanceBetweenCarromStriker(carrom,strikerStatic) < ctsd){
                return false;
            }
        }
        return true;
    }

    public static boolean notOverlapCarromStriker(Carrom carrom, PointD strikerStatic) {
        return distanceBetweenCarromStriker(carrom, strikerStatic) >= ctsd;
    }

    public static double distanceBetweenCarrom(Carrom carrom1,Carrom carrom2) {
        double xDiff = carrom2.x - carrom1.x;
        double yDiff = carrom2.y - carrom1.y;
        return Math.sqrt(xDiff * xDiff + yDiff * yDiff);
    }

    public static double distanceBetweenCarromStriker(Carrom carrom,PointD strikerStatic) {
        double xDiff = strikerStatic.x - carrom.x;
        double yDiff = strikerStatic.y - carrom.y;
        return Math.sqrt(xDiff * xDiff + yDiff * yDiff);
    }

}
