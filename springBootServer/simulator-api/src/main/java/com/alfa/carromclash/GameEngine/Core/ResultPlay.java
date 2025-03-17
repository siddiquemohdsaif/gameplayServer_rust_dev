package com.alfa.carromclash.GameEngine.Core;

import java.util.ArrayList;

public class ResultPlay {

    public ArrayList<Carrom> hisPocketedCoins;
    public ArrayList<Carrom> opponentPocketedCoins;
    public boolean isStrikerPotted;

    public ResultPlay(ArrayList<Carrom> hisPocketedCoins, ArrayList<Carrom> opponentPocketedCoins, boolean isStrikerPotted) {
        this.hisPocketedCoins = hisPocketedCoins;
        this.opponentPocketedCoins = opponentPocketedCoins;
        this.isStrikerPotted = isStrikerPotted;
    }

    public static ResultPlay getResultPlay(GameManager2player.Player currentTurn, CoinsContainer carromsOnBoard, boolean isStrikerPocketed) {

        boolean isStrikerPotted = isStrikerPocketed;
        ArrayList<Carrom> hisPocketedCoins = new ArrayList<>();
        ArrayList<Carrom> opponentPocketedCoins = new ArrayList<>();


        for (Carrom carrom: carromsOnBoard.carroms) {
            if (carrom.isInPocket != null){
                if (carrom.type == Carrom.Type.QUEEN){
                    hisPocketedCoins.add(carrom);
                }else if ((carrom.type == Carrom.Type.CARROM_MEN_1) && (currentTurn == GameManager2player.Player.PLAYER_1) || (carrom.type == Carrom.Type.CARROM_MEN_2) && (currentTurn == GameManager2player.Player.PLAYER_2)){
                    hisPocketedCoins.add(carrom);
                }else {
                    opponentPocketedCoins.add(carrom);
                }
            }
        }

        return new ResultPlay(hisPocketedCoins,opponentPocketedCoins,isStrikerPotted);
    }

    public static Carrom getCarromByCoinId(int coinCode, ArrayList<Carrom> carroms) {
        for (Carrom carrom:carroms) {
            if (carrom.coinCode == coinCode){
                return carrom;
            }
        }
        throw new RuntimeException("Unexpected state: 0x002");  // coinCode not found
    }
}
