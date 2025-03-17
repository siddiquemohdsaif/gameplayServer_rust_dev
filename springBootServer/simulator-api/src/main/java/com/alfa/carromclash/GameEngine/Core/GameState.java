package com.alfa.carromclash.GameEngine.Core;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;

public class GameState {
    //{"playerTurn":0,"isOnlyQueenPocketedLast":false,"carroms":[{"carrom_drawable_id":0,"isPotted":0,"x":-0.014061474241316319,"y":0.03863358125090599,"type":"CARROM_MEN_1","coinCode":1},{"carrom_drawable_id":0,"isPotted":0,"x":-0.028122948482632637,"y":0.07726716250181198,"type":"CARROM_MEN_1","coinCode":2},{"carrom_drawable_id":0,"isPotted":0,"x":0.04048839956521988,"y":-0.007139197550714016,"type":"CARROM_MEN_1","coinCode":3},{"carrom_drawable_id":0,"isPotted":0,"x":0.08097679913043976,"y":-0.014278395101428032,"type":"CARROM_MEN_1","coinCode":4},{"carrom_drawable_id":0,"isPotted":0,"x":-0.026426926255226135,"y":-0.0314943864941597,"type":"CARROM_MEN_1","coinCode":5},{"carrom_drawable_id":0,"isPotted":0,"x":-0.05285385251045227,"y":-0.0629887729883194,"type":"CARROM_MEN_1","coinCode":6},{"carrom_drawable_id":0,"isPotted":0,"x":0.053994160145521164,"y":0.06434773653745651,"type":"CARROM_MEN_1","coinCode":7},{"carrom_drawable_id":0,"isPotted":0,"x":-0.08272384852170944,"y":0.014586446806788445,"type":"CARROM_MEN_1","coinCode":8},{"carrom_drawable_id":0,"isPotted":0,"x":0.028729692101478577,"y":-0.07893417775630951,"type":"CARROM_MEN_1","coinCode":9},{"carrom_drawable_id":1,"isPotted":0,"x":0.026997080072760582,"y":0.032173868268728256,"type":"CARROM_MEN_2","coinCode":10},{"carrom_drawable_id":1,"isPotted":0,"x":-0.04136192426085472,"y":0.007293223403394222,"type":"CARROM_MEN_2","coinCode":11},{"carrom_drawable_id":1,"isPotted":0,"x":0.014364846050739288,"y":-0.039467088878154755,"type":"CARROM_MEN_2","coinCode":12},{"carrom_drawable_id":1,"isPotted":0,"x":0.06765786558389664,"y":0.024625450372695923,"type":"CARROM_MEN_2","coinCode":13},{"carrom_drawable_id":1,"isPotted":0,"x":0.012502668425440788,"y":0.0709061548113823,"type":"CARROM_MEN_2","coinCode":14},{"carrom_drawable_id":1,"isPotted":0,"x":-0.055155199021101,"y":0.04628070816397667,"type":"CARROM_MEN_2","coinCode":15},{"carrom_drawable_id":1,"isPotted":0,"x":-0.06765786558389664,"y":-0.024625450372695923,"type":"CARROM_MEN_2","coinCode":16},{"carrom_drawable_id":1,"isPotted":0,"x":-0.012502668425440788,"y":-0.0709061548113823,"type":"CARROM_MEN_2","coinCode":17},{"carrom_drawable_id":1,"isPotted":0,"x":0.055155199021101,"y":-0.04628070816397667,"type":"CARROM_MEN_2","coinCode":18},{"carrom_drawable_id":2131165404,"isPotted":0,"x":0,"y":0,"type":"QUEEN","coinCode":19}],"strikerId_1":0,"strikerId_2":0}
    public ArrayList<Carrom> carroms;
    public int playerTurn;
    public int strikerId_1;
    public int strikerId_2;
    public String playerExtraInfo1;
    public String playerExtraInfo2;
    public int map;
    public boolean isOnlyQueenPocketedLast;

    public GameState(ArrayList<Carrom> carroms, int playerTurn, int strikerId_1,int strikerId_2,String playerExtraInfo1 ,String playerExtraInfo2,int map,boolean isOnlyQueenPocketedLast) {
        this.carroms = carroms;
        this.playerTurn = playerTurn;
        this.strikerId_1 = strikerId_1;
        this.strikerId_2 = strikerId_2;
        this.playerExtraInfo1 = playerExtraInfo1;
        this.playerExtraInfo2 = playerExtraInfo2;
        this.map = map;
        this.isOnlyQueenPocketedLast = isOnlyQueenPocketedLast;
    }

    public int getPowerId(GameManager2player.Player player){
        try {
            JSONObject jsonObject;
            if (player == GameManager2player.Player.PLAYER_1){
                jsonObject = new JSONObject(playerExtraInfo1);
            }else {
                jsonObject = new JSONObject(playerExtraInfo2);
            }
            return jsonObject.getInt("power");
        }catch (Exception e){
            throw new RuntimeException("error : x00892367");
        }
    }

    public GameState(ArrayList<Carrom> carromsOnBoard , ArrayList<Carrom> carromsP1 , ArrayList<Carrom> carromsP2 , int playerTurn ,int strikerId_1,int strikerId_2,String playerExtraInfo1,String playerExtraInfo2, int map, boolean isOnlyQueenPocketedLast ) {

        for (Carrom carrom : carromsOnBoard) {
            carrom.isPotted = 0;
        }

        for (Carrom carrom : carromsP1) {
            carrom.isPotted = 1;
        }

        for (Carrom carrom : carromsP2) {
            carrom.isPotted = 2;
        }

        carroms = new ArrayList<>();
        carroms.addAll(carromsOnBoard);
        carroms.addAll(carromsP1);
        carroms.addAll(carromsP2);

        this.playerTurn = playerTurn;
        this.strikerId_1 = strikerId_1;
        this.strikerId_2 = strikerId_2;
        this.playerExtraInfo1 = playerExtraInfo1;
        this.playerExtraInfo2 = playerExtraInfo2;
        this.map = map;
        this.isOnlyQueenPocketedLast = isOnlyQueenPocketedLast;
    }


    public static String convertGameStateToJsonString(GameState gameState) {
        JSONObject jsonObject = new JSONObject();
        try {
            jsonObject.put("playerTurn", gameState.playerTurn);
            jsonObject.put("strikerId_1", gameState.strikerId_1);
            jsonObject.put("strikerId_2", gameState.strikerId_2);
            jsonObject.put("playerExtraInfo1", gameState.playerExtraInfo1);
            jsonObject.put("playerExtraInfo2", gameState.playerExtraInfo2);
            jsonObject.put("map", gameState.map);
            jsonObject.put("isOnlyQueenPocketedLast", gameState.isOnlyQueenPocketedLast);

            // Sort carroms in ascending order based on coinCode
            Collections.sort(gameState.carroms, new Comparator<Carrom>() {
                @Override
                public int compare(Carrom c1, Carrom c2) {
                    return Integer.compare(c1.coinCode, c2.coinCode);
                }
            });

            JSONArray carromsJsonArray = new JSONArray();
            for (Carrom carrom : gameState.carroms) {
                JSONObject carromJsonObject = new JSONObject();
                try {
                    carromJsonObject.put("carrom_drawable_id", carrom.carrom_drawable_id);
                    carromJsonObject.put("coinCode", carrom.coinCode);
                    carromJsonObject.put("type", carrom.type.toString());
                    carromJsonObject.put("x", carrom.x);
                    carromJsonObject.put("y", carrom.y);
                    carromJsonObject.put("isPotted", carrom.isPotted);
                } catch (JSONException e) {
                    e.printStackTrace();
                }
                carromsJsonArray.put(carromJsonObject);
            }

            jsonObject.put("carroms", carromsJsonArray);
        } catch (JSONException e) {
            e.printStackTrace();
        }
        return jsonObject.toString();
    }



    public static GameState convertJsonStringToGameState(String jsonString, GameManager2player.Player player) {


        int playerTurn;
        int strikerId_1;
        int strikerId_2;
        String playerExtraInfo1;
        String playerExtraInfo2;
        int map;
        boolean isOnlyQueenPocketedLast;
        ArrayList<Carrom> carroms = new ArrayList<>();
        try {

            int replaceCoinId = -1;

            JSONObject jsonObject = new JSONObject(jsonString);
            if (isBothPlayerHaveSameCoin(jsonObject)){
                int myCarrrom = getCarromIdOfPlayer(jsonObject,player);
                if (myCarrrom != 0){
                    replaceCoinId = 0;
                }else {
                    replaceCoinId = 1;
                }
            }

            playerTurn = jsonObject.getInt("playerTurn");
            strikerId_1 = jsonObject.getInt("strikerId_1");
            strikerId_2 = jsonObject.getInt("strikerId_2");
            playerExtraInfo1 = jsonObject.getString("playerExtraInfo1");
            playerExtraInfo2 = jsonObject.getString("playerExtraInfo2");
            map = jsonObject.getInt("map");
            isOnlyQueenPocketedLast = jsonObject.getBoolean("isOnlyQueenPocketedLast");

            JSONArray carromsJsonArray = jsonObject.getJSONArray("carroms");
            for (int i = 0; i < carromsJsonArray.length(); i++) {
                JSONObject carromJsonObject = carromsJsonArray.getJSONObject(i);
                int carrom_drawable_id = carromJsonObject.getInt("carrom_drawable_id");
                int coinCode = carromJsonObject.getInt("coinCode");
                Carrom.Type type = Carrom.Type.valueOf(carromJsonObject.getString("type"));
                double x = carromJsonObject.getDouble("x");
                double y = carromJsonObject.getDouble("y");
                int isPotted = carromJsonObject.getInt("isPotted");
                Carrom carrom;
                if (replaceCoinId != -1){

                    if (player == GameManager2player.Player.PLAYER_1 && type == Carrom.Type.CARROM_MEN_2){
                        carrom = new Carrom(carrom_drawable_id, replaceCoinId , coinCode, type, x, y, isPotted);
                    }else if (player == GameManager2player.Player.PLAYER_2 && type == Carrom.Type.CARROM_MEN_1){
                        carrom = new Carrom(carrom_drawable_id, replaceCoinId, coinCode, type, x, y, isPotted);
                    }else {
                        carrom = new Carrom(carrom_drawable_id, carrom_drawable_id, coinCode, type, x, y, isPotted);
                    }

                }else {
                    carrom = new Carrom(carrom_drawable_id, carrom_drawable_id, coinCode, type, x, y, isPotted);
                }
                carroms.add(carrom);
            }

            // Sort carroms in ascending order based on coinCode
            Collections.sort(carroms, new Comparator<Carrom>() {
                @Override
                public int compare(Carrom c1, Carrom c2) {
                    return Integer.compare(c1.coinCode, c2.coinCode);
                }
            });

        } catch (JSONException e) {
            e.printStackTrace();
            return null;
        }
        return new GameState(carroms, playerTurn,strikerId_1 ,strikerId_2, playerExtraInfo1 , playerExtraInfo2,map, isOnlyQueenPocketedLast);
    }

    private static boolean isBothPlayerHaveSameCoin(JSONObject jsonObject) throws JSONException {
        int carromId1 = 0;
        int carromId2 = 0;
        JSONArray carromsJsonArray = jsonObject.getJSONArray("carroms");
        for (int i = 0; i < carromsJsonArray.length(); i++) {
            JSONObject carromJsonObject = carromsJsonArray.getJSONObject(i);
            int carrom_drawable_id = carromJsonObject.getInt("carrom_drawable_id");
            Carrom.Type type = Carrom.Type.valueOf(carromJsonObject.getString("type"));
            if (type == Carrom.Type.CARROM_MEN_1){
                carromId1 = carrom_drawable_id;
            }
            if (type == Carrom.Type.CARROM_MEN_2){
                carromId2 = carrom_drawable_id;
            }
        }

        return carromId1 == carromId2;
    }

    private static int getCarromIdOfPlayer(JSONObject jsonObject, GameManager2player.Player player) throws JSONException {
        int carromId1 = 0;
        int carromId2 = 0;
        JSONArray carromsJsonArray = jsonObject.getJSONArray("carroms");
        for (int i = 0; i < carromsJsonArray.length(); i++) {
            JSONObject carromJsonObject = carromsJsonArray.getJSONObject(i);
            int carrom_drawable_id = carromJsonObject.getInt("carrom_drawable_id");
            Carrom.Type type = Carrom.Type.valueOf(carromJsonObject.getString("type"));
            if (type == Carrom.Type.CARROM_MEN_1){
                carromId1 = carrom_drawable_id;
            }
            if (type == Carrom.Type.CARROM_MEN_2){
                carromId2 = carrom_drawable_id;
            }
        }

        if (player == GameManager2player.Player.PLAYER_1){
            return carromId1;
        }else {
            return carromId2;
        }
    }


}
