package com.alfa.carromclash.GameEngine.Core;




import org.json.JSONException;
import org.json.JSONObject;

import com.alfa.carromclash.GameEngine.Core.GameManager2player.Player;

import java.util.ArrayList;

public class TwoPlayerGame implements  GameManager2player.ActionCallback {



    private GameManager2player gameManager;
    private GameManager2player.Player currentTurn;
    private ArrayList<Carrom> p1CarromPotted;
    private ArrayList<Carrom> p2CarromPotted;
    private CoinsContainer carromsOnBoard;
    private ArrayList<Carrom> allCarroms;


    private int strikerId_1;
    private int strikerId_2;
    public String playerExtraInfo2;
    public String playerExtraInfo1;
    private int aim1, aim2;
    private int power1,power2;
    private int time1,time2;
    public int map;


    public boolean queenVisible = true;


    public TwoPlayerGame (GameState gameState, SimulatorEvent simulatorEvent) {


        if (simulatorEvent.sp == 0){
            currentTurn = GameManager2player.Player.PLAYER_1;
        }else {
            currentTurn = GameManager2player.Player.PLAYER_2;
        }

        strikerId_1 = gameState.strikerId_1;
        strikerId_2 = gameState.strikerId_2;
        playerExtraInfo1 = gameState.playerExtraInfo1;
        playerExtraInfo2 = gameState.playerExtraInfo2;
        map = gameState.map;
        parsePlayerInfo();

        p1CarromPotted = new ArrayList<>();
        p2CarromPotted = new ArrayList<>();
        ArrayList<Carrom> carromsOnBoardGet = new ArrayList<>();


        for (Carrom carrom: gameState.carroms) {

            if (carrom.isPotted == 0){
                carromsOnBoardGet.add(carrom);
            }else if (carrom.isPotted == 1){
                p1CarromPotted.add(carrom);
            }else if (carrom.isPotted == 2){
                p2CarromPotted.add(carrom);
            }
        }
        carromsOnBoard = new CoinsContainer(carromsOnBoardGet);
        allCarroms = new ArrayList<>();
        allCarroms.addAll(p1CarromPotted);
        allCarroms.addAll(p2CarromPotted);
        allCarroms.addAll(carromsOnBoard.carroms);



        gameManager = new GameManager2player(this);
        gameManager.isOnlyQueenPocketedLast = gameState.isOnlyQueenPocketedLast;
        gameManager.previousTurn = currentTurn;

    }

    public Result evaluateSimulation(Simulator simulator){

        long startTime = System.currentTimeMillis();
        long currentSimulatedTime = startTime;

        simulator.start(carromsOnBoard);
        while(!simulator.isStimulationFinish() && ((currentSimulatedTime-startTime) < 30*1000)  ){

            currentSimulatedTime = currentSimulatedTime + 60;
            simulator.stimulateToCurrentTime(currentSimulatedTime);

        }

       
        //return  gameManager.play(ResultPlay.getResultPlay(simulator.table.outcome,currentTurn,carromsOnBoard), p1CarromPotted, p2CarromPotted ,carromsOnBoard ,currentTurn);
        return gameManager.play(ResultPlay.getResultPlay(currentTurn,carromsOnBoard,simulator.isStrikerPocketed), p1CarromPotted, p2CarromPotted ,carromsOnBoard ,currentTurn, strikerId_1,strikerId_2,playerExtraInfo1,playerExtraInfo2, map);
        // result.endGameState = GameState.convertGameStateToJsonString(new GameState(carromsOnBoard.carroms,p1CarromPotted,p2CarromPotted, 0, 0, false));
        // return result;

    }

    
    public static class Result {

        private String action;
        private int player;
        private String endGameState;

        // Constructor for all variables
        public Result(String action, Player player, String endGameState) {
            this.action = action;
            if(player == Player.PLAYER_1){
                this.player = 0;
            }else{
                this.player = 1;
            }
            this.endGameState = endGameState;
        }

        // Public static method to convert a Result object to a JSONObject string
        public static String toJsonString(Result result) {
            JSONObject jsonObject = new JSONObject();
            jsonObject.put("action", result.action);
            jsonObject.put("player", result.player); 
            jsonObject.put("endGameState", result.endGameState);
            return jsonObject.toString();
        }

        // Public static method to convert a JSONObject string to a Result object
        public static Result fromJsonString(String jsonString) {
            JSONObject jsonObject = new JSONObject(jsonString);
            String action = jsonObject.getString("action");
            int player = jsonObject.getInt("player"); 
            String endGameState = jsonObject.getString("endGameState");
            Player mPlayer ;
            if(player == 0){
                mPlayer = Player.PLAYER_1;
            }else{
                mPlayer = Player.PLAYER_2;
            }
            return new Result(action, mPlayer, endGameState);
        }
    }



   

  







    private void parsePlayerInfo() {
        try {
            // Decode playerExtraInfo1 JSON string
            JSONObject playerExtraInfoObj1 = new JSONObject(playerExtraInfo1);
            aim1 = playerExtraInfoObj1.getInt("a");
            power1 = playerExtraInfoObj1.getInt("f");
            time1 = playerExtraInfoObj1.getInt("t");

            // Decode playerExtraInfo2 JSON string
            JSONObject playerExtraInfoObj2 = new JSONObject(playerExtraInfo2);
            aim2 = playerExtraInfoObj2.getInt("a");
            power2 = playerExtraInfoObj2.getInt("f");
            time2 = playerExtraInfoObj2.getInt("t");
        } catch (JSONException e) {
            e.printStackTrace();
            throw new RuntimeException("error:x000235");
        }
    }












    //////////////////////// Game event
    //////////////////////// ///////////////////////////////////////////////////////////////////

    @Override
    public void foulStriker(GameManager2player.Player player, Carrom carromToPace1) {
        // Log.d("shabir", "foulStriker: "+ player);
        // Toast.makeText(view.getContext(), "foulStriker", Toast.LENGTH_LONG).show();
    }

    @Override
    public void foulCoin(GameManager2player.Player player, Carrom carromToPace1, Carrom carromToPace2) {
        // Log.d("shabir", "foulCoin: "+ player);
        // Toast.makeText(view.getContext(), "foulCoin", Toast.LENGTH_LONG).show();
    }

    @Override
    public void needToPocketCover(GameManager2player.Player player) {
        // Log.d("shabir", "needToPocketCover: ");
        // Toast.makeText(view.getContext(), "needToPocketCover",
        // Toast.LENGTH_LONG).show();
    }

    @Override
    public void insertCarromToBoard(GameManager2player.Player player, Carrom carrom) {
        carromsOnBoard.insertCarrom(carrom);
    }

    @Override
    public void coinPocketed(ArrayList<Carrom> carromsP1, ArrayList<Carrom> carromsP2) {
        // Log.d("shabir", "coinPocketed: ");
    }
    //////////////////////// Game event ////////////////////////


}
