package com.alfa.carromclash;

import com.alfa.carromclash.GameEngine.Core.GameState;
import com.alfa.carromclash.GameEngine.Core.Simulator;
import com.alfa.carromclash.GameEngine.Core.StrikerStatic;
import com.alfa.carromclash.GameEngine.Core.TwoPlayerGame;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import com.alfa.carromclash.GameEngine.Core.Carrom;
import com.alfa.carromclash.GameEngine.Core.GameManager2player.Player;
import com.alfa.carromclash.GameEngine.Core.TwoPlayerGame.Result;
import com.alfa.carromclash.GameEngine.Core.SimulatorEvent;


public class SumulationAPI {

    public static synchronized String getSimulation(String gameStateSting, String simulatorEventString){ // it synchronized to prevent native cpp simmulator pointer is static single global pointer.

        //String gameStateSting = "{ \"playerTurn\": 1, \"strikerId_1\": 1, \"strikerId_2\": 1, \"playerExtraInfo1\": \"{ \\\"isPlayerInWar\\\": false, \\\"a\\\": 3, \\\"f\\\": 3, \\\"t\\\": 2, \\\"striker\\\": 1, \\\"power\\\": 1, \\\"puck\\\": 1 }\", \"playerExtraInfo2\": \"{ \\\"isPlayerInWar\\\": false, \\\"a\\\": 3, \\\"f\\\": 3, \\\"t\\\": 2, \\\"striker\\\": 1, \\\"power\\\": 1, \\\"puck\\\": 1 }\", \"map\": 1, \"isOnlyQueenPocketedLast\": false, \"carroms\": [ { \"carrom_drawable_id\": 1, \"coinCode\": 1, \"type\": \"CARROM_MEN_1\", \"x\": -0.10198807666421583, \"y\": 0.027751753333148048, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 2, \"type\": \"CARROM_MEN_1\", \"x\": -0.18562401346867924, \"y\": -0.007534132650602602, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 3, \"type\": \"CARROM_MEN_1\", \"x\": 0.30046876082311724, \"y\": 0.13636116593152162, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 4, \"type\": \"CARROM_MEN_1\", \"x\": 0.05824513423010595, \"y\": 0.26946311199608053, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 5, \"type\": \"CARROM_MEN_1\", \"x\": -0.06306930504458204, \"y\": -0.0842821336089855, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 6, \"type\": \"CARROM_MEN_1\", \"x\": -0.04113171580323588, \"y\": -0.15677742273093112, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 7, \"type\": \"CARROM_MEN_1\", \"x\": -0.03635267886202495, \"y\": 0.16061465889720467, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 8, \"type\": \"CARROM_MEN_1\", \"x\": -0.09411974687010273, \"y\": -0.027187611933993935, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 9, \"type\": \"CARROM_MEN_1\", \"x\": 0.08657181331288874, \"y\": -0.09505868405062687, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 10, \"type\": \"CARROM_MEN_2\", \"x\": 0.01702518356158156, \"y\": 0.030481730183961615, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 11, \"type\": \"CARROM_MEN_2\", \"x\": -0.05171593025859613, \"y\": -0.027133384865352654, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 12, \"type\": \"CARROM_MEN_2\", \"x\": 0.039015426363975904, \"y\": -0.0037779414398430218, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 13, \"type\": \"CARROM_MEN_2\", \"x\": 0.23185757885844333, \"y\": -0.06732640711841809, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 14, \"type\": \"CARROM_MEN_2\", \"x\": -0.0950883596111126, \"y\": 0.17839123123974893, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 15, \"type\": \"CARROM_MEN_2\", \"x\": 0.1963687008531903, \"y\": -0.31297934767126173, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 16, \"type\": \"CARROM_MEN_2\", \"x\": -0.2117846957175122, \"y\": -0.06304964365507698, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 17, \"type\": \"CARROM_MEN_2\", \"x\": 0.03160329013401295, \"y\": -0.3464842066027493, \"isPotted\": 0 }, { \"carrom_drawable_id\": 1, \"coinCode\": 18, \"type\": \"CARROM_MEN_2\", \"x\": 0.24360467938497352, \"y\": 0.6024865731217649, \"isPotted\": 2 }, { \"carrom_drawable_id\": -1, \"coinCode\": 19, \"type\": \"QUEEN\", \"x\": -0.1827136089364998, \"y\": 0.227271457718199, \"isPotted\": 0 } ] }";
        //String simulatorEventString = "{\"eventType\":\"simulationStart\",\"eventData\":{\"px\":0.1396558076384837,\"py\":0.2528478,\"vx\":0.37874329444015886,\"vy\":-1.4957949434257247,\"sp\":1}}";


        GameState gameState = GameState.convertJsonStringToGameState(gameStateSting, Player.PLAYER_1);
        SimulatorEvent simulatorEvent = SimulatorEvent.fromJSONString(simulatorEventString);
        StrikerStatic strikerStatic = new StrikerStatic(simulatorEvent.px, simulatorEvent.py);
        Simulator simulator = new Simulator(simulatorEvent.px, simulatorEvent.py, simulatorEvent.vx, simulatorEvent.vy, strikerStatic);

        TwoPlayerGame twoPlayerGame = new TwoPlayerGame(gameState , simulatorEvent);
        Result result =  twoPlayerGame.evaluateSimulation(simulator);


        
        JSONObject jsonObject = new JSONObject();
        jsonObject.put("balls_history_x", simulator.coinPositionsHistory_x);
        jsonObject.put("balls_history_y", simulator.coinPositionsHistory_y);


        JSONArray carromsJsonArray = new JSONArray();
        for (Carrom carrom : simulator.coinsContainer.carroms) {
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

        return Result.toJsonString(result) + jsonObject.toString();
    }
    
}
