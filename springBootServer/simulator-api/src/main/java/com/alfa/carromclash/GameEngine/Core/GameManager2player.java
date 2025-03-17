package com.alfa.carromclash.GameEngine.Core;

import java.util.ArrayList;

import com.alfa.carromclash.GameEngine.Core.TwoPlayerGame.Result;

public class GameManager2player {

    private ActionCallback callback;

    public GameManager2player(ActionCallback callback) {
        this.callback = callback;
    }

    public Player previousTurn = Player.PLAYER_1;
    public boolean isOnlyQueenPocketedLast = false;

    private int strikerId_1;
    private int strikerId_2;
    public String playerExtraInfo2;
    public String playerExtraInfo1;
    public int map;

    public Result play(ResultPlay resultPlay, ArrayList<Carrom> p1CarromPotted, ArrayList<Carrom> p2CarromPotted,
            CoinsContainer carromsOnBoard, Player currentTurn, int strikerId_1, int strikerId_2, String playerExtraInfo1, String playerExtraInfo2, int map) {

        this.strikerId_1 = strikerId_1;
        this.strikerId_2 = strikerId_2;
        this.playerExtraInfo2 = playerExtraInfo2;
        this.playerExtraInfo1 = playerExtraInfo1;
        this.map = map;
        carromsOnBoard.remove(resultPlay);

        if (checkOpponentWin(carromsOnBoard, currentTurn)) {
            isOnlyQueenPocketedLast = false;
            if (currentTurn == Player.PLAYER_1) {
                // callback.win(Player.PLAYER_2);
                GameState gameState = new GameState(carromsOnBoard.carroms, p1CarromPotted, p2CarromPotted, 0,
                strikerId_1,strikerId_2,playerExtraInfo1,playerExtraInfo2,map,isOnlyQueenPocketedLast);
                return new Result("win", Player.PLAYER_2, GameState.convertGameStateToJsonString(gameState));

            } else {
                // callback.win(Player.PLAYER_1);
                GameState gameState = new GameState(carromsOnBoard.carroms, p1CarromPotted, p2CarromPotted, 1,
                        strikerId_1,strikerId_2,playerExtraInfo1,playerExtraInfo2,map,isOnlyQueenPocketedLast);
                return new Result("win", Player.PLAYER_1, GameState.convertGameStateToJsonString(gameState));
            }

        } else if (checkIsStrikerPocketed(resultPlay)) {
            // add all coin potted
            addOpponentPottedCoin(p1CarromPotted, p2CarromPotted, currentTurn, resultPlay);

            // Insert queen on board hence failed to pocket cover
            if (checkFailedToPocketCoverForQueen(currentTurn)) {
                // remove queen from player and place to board
                Carrom carromQueen;
                if (currentTurn == Player.PLAYER_1) {
                    carromQueen = pullQueen(p1CarromPotted);
                } else {
                    carromQueen = pullQueen(p2CarromPotted);
                }
                callback.insertCarromToBoard(currentTurn, carromQueen);
            }

            // insert potted carrom
            for (Carrom carrom : resultPlay.hisPocketedCoins) {
                callback.insertCarromToBoard(currentTurn, carrom);
            }

            // insert one carrom on board for striker foul
            Carrom carrom = null;
            if (currentTurn == Player.PLAYER_1) {
                if (p1CarromPotted.size() >= 1) {
                    carrom = pullCarrom(p1CarromPotted);
                }
            } else {
                if (p2CarromPotted.size() >= 1) {
                    carrom = pullCarrom(p2CarromPotted);
                }
            }
            if (carrom != null) {
                callback.insertCarromToBoard(currentTurn, carrom);
            }
            // feedback
            callback.foulStriker(currentTurn, carrom);

            // turn change
            if (currentTurn == Player.PLAYER_1) {
                return returnTurnChange(p1CarromPotted, p2CarromPotted, carromsOnBoard, Player.PLAYER_2);
            } else {
                return returnTurnChange(p1CarromPotted, p2CarromPotted, carromsOnBoard, Player.PLAYER_1);
            }

        } else if (checkMyWin(p1CarromPotted, p2CarromPotted, currentTurn, resultPlay.hisPocketedCoins,
                carromsOnBoard)) {
            // callback.win(currentTurn);
            // turn change
            int playerTurn;
            Player turnChangTo;
            if (currentTurn == Player.PLAYER_1) {
                playerTurn = 0;
                turnChangTo = Player.PLAYER_1;
            } else {
                playerTurn = 1;
                turnChangTo = Player.PLAYER_2;
            }
            isOnlyQueenPocketedLast = false;
            GameState gameState = new GameState(carromsOnBoard.carroms, p1CarromPotted, p2CarromPotted, playerTurn,
                    strikerId_1,strikerId_2,playerExtraInfo1,playerExtraInfo2,map,isOnlyQueenPocketedLast);
            return new Result("win", turnChangTo, GameState.convertGameStateToJsonString(gameState));

        } else if (checkCoinFoul(carromsOnBoard, currentTurn)) {
            Carrom carromToPace1;
            Carrom carromToPace2;

            if (currentTurn == Player.PLAYER_1) {
                // add all potted coin
                addAllPottedCoin(p1CarromPotted, p2CarromPotted, currentTurn, resultPlay);

                // get last two coin
                carromToPace1 = p1CarromPotted.get(p1CarromPotted.size() - 1);
                carromToPace2 = p1CarromPotted.get(p1CarromPotted.size() - 2);

                // remove last two coin
                p1CarromPotted.remove(p1CarromPotted.size() - 1);
                p1CarromPotted.remove(p1CarromPotted.size() - 1);
            } else {
                // add all potted coin
                addAllPottedCoin(p1CarromPotted, p2CarromPotted, currentTurn, resultPlay);

                // get last two coin
                carromToPace1 = p2CarromPotted.get(p2CarromPotted.size() - 1);
                carromToPace2 = p2CarromPotted.get(p2CarromPotted.size() - 2);

                // remove last two coin
                p2CarromPotted.remove(p2CarromPotted.size() - 1);
                p2CarromPotted.remove(p2CarromPotted.size() - 1);
            }
            // place on board
            callback.insertCarromToBoard(currentTurn, carromToPace1);
            callback.insertCarromToBoard(currentTurn, carromToPace2);

            // feedback
            callback.foulCoin(currentTurn, carromToPace1, carromToPace2);

            // turn change
            if (currentTurn == Player.PLAYER_1) {
                return returnTurnChange(p1CarromPotted, p2CarromPotted, carromsOnBoard, Player.PLAYER_2);
            } else {
                return returnTurnChange(p1CarromPotted, p2CarromPotted, carromsOnBoard, Player.PLAYER_1);
            }

        } else if (checkOnlyPockedQueenButNoCover(resultPlay.hisPocketedCoins)) {
            // add queen and opponent coin
            addAllPottedCoin(p1CarromPotted, p2CarromPotted, currentTurn, resultPlay);

            // pocket cover
            previousTurn = currentTurn;
            isOnlyQueenPocketedLast = true;
            callback.needToPocketCover(currentTurn);

            // play again current player
            return returnTurnChangeWithQueenLastPocket(p1CarromPotted, p2CarromPotted, carromsOnBoard, currentTurn);

            // return; // for cover to pocket in next turn

        } else if (checkAtleastPocketedOneCoinToGetTurnAgain(resultPlay.hisPocketedCoins)) {
            // add all coin pocketed
            addAllPottedCoin(p1CarromPotted, p2CarromPotted, currentTurn, resultPlay);

            // play again current player
            return returnTurnChange(p1CarromPotted, p2CarromPotted, carromsOnBoard, currentTurn);

        } else if (checkFailedToPocketCoverForQueen(currentTurn)) {
            // off cover to pocket
            isOnlyQueenPocketedLast = false;

            // add all coin potted
            addAllPottedCoin(p1CarromPotted, p2CarromPotted, currentTurn, resultPlay);

            // remove queen from player and place to board
            Carrom carromQueen;
            if (currentTurn == Player.PLAYER_1) {
                carromQueen = pullQueen(p1CarromPotted);
            } else {
                carromQueen = pullQueen(p2CarromPotted);
            }
            callback.insertCarromToBoard(currentTurn, carromQueen);

            // turn change
            if (currentTurn == Player.PLAYER_1) {
                return returnTurnChange(p1CarromPotted, p2CarromPotted, carromsOnBoard, Player.PLAYER_2);
            } else {
                return returnTurnChange(p1CarromPotted, p2CarromPotted, carromsOnBoard, Player.PLAYER_1);
            }

        } else {

            // add all coin potted
            addAllPottedCoin(p1CarromPotted, p2CarromPotted, currentTurn, resultPlay);

            // turn change
            if (currentTurn == Player.PLAYER_1) {
                return returnTurnChange(p1CarromPotted, p2CarromPotted, carromsOnBoard, Player.PLAYER_2);
            } else {
                return returnTurnChange(p1CarromPotted, p2CarromPotted, carromsOnBoard, Player.PLAYER_1);
            }
        }

    }

    private Result returnTurnChange(ArrayList<Carrom> p1CarromPotted, ArrayList<Carrom> p2CarromPotted,
            CoinsContainer carromsOnBoard, Player turnChangeTo) {
        isOnlyQueenPocketedLast = false;
        int playerTurn;
        if (turnChangeTo == Player.PLAYER_1) {
            playerTurn = 0;
        } else {
            playerTurn = 1;
        }
        GameState gameState = new GameState(carromsOnBoard.carroms, p1CarromPotted, p2CarromPotted, playerTurn,
                strikerId_1,strikerId_2,playerExtraInfo1,playerExtraInfo2,map,isOnlyQueenPocketedLast);
        return new Result("turnChange", turnChangeTo, GameState.convertGameStateToJsonString(gameState));
    }

    private Result returnTurnChangeWithQueenLastPocket(ArrayList<Carrom> p1CarromPotted,
            ArrayList<Carrom> p2CarromPotted,
            CoinsContainer carromsOnBoard, Player turnChangeTo) {
        isOnlyQueenPocketedLast = true;
        int playerTurn;
        if (turnChangeTo == Player.PLAYER_1) {
            playerTurn = 0;
        } else {
            playerTurn = 1;
        }
        GameState gameState = new GameState(carromsOnBoard.carroms, p1CarromPotted, p2CarromPotted, playerTurn,
                strikerId_1,strikerId_2,playerExtraInfo1,playerExtraInfo2,map,isOnlyQueenPocketedLast);
        return new Result("turnChange", turnChangeTo, GameState.convertGameStateToJsonString(gameState));
    }

    private void addAllPottedCoin(ArrayList<Carrom> p1CarromPotted, ArrayList<Carrom> p2CarromPotted,
            Player currentTurn, ResultPlay resultPlay) {
        if (currentTurn == Player.PLAYER_1) {
            p1CarromPotted.addAll(resultPlay.hisPocketedCoins);
            p2CarromPotted.addAll(resultPlay.opponentPocketedCoins);
        } else {
            p2CarromPotted.addAll(resultPlay.hisPocketedCoins);
            p1CarromPotted.addAll(resultPlay.opponentPocketedCoins);
        }
    }

    private void addOpponentPottedCoin(ArrayList<Carrom> p1CarromPotted, ArrayList<Carrom> p2CarromPotted,
            Player currentTurn, ResultPlay resultPlay) {
        if (currentTurn == Player.PLAYER_1) {
            p2CarromPotted.addAll(resultPlay.opponentPocketedCoins);
        } else {
            p1CarromPotted.addAll(resultPlay.opponentPocketedCoins);
        }
    }

    private Carrom pullQueen(ArrayList<Carrom> carromPotted) {
        for (int i = 0; i < carromPotted.size(); i++) {
            if (carromPotted.get(i).type == Carrom.Type.QUEEN) {
                Carrom carromQueen = carromPotted.get(i);
                carromPotted.remove(i);
                return carromQueen;
            }
        }
        throw new RuntimeException("Unexpected state: 0x003"); // queen not found but already potted
    }

    private Carrom pullCarrom(ArrayList<Carrom> carromPotted) {
        for (int i = carromPotted.size() - 1; i >= 0; i--) {
            if (carromPotted.get(i).type != Carrom.Type.QUEEN) {
                Carrom carromMen = carromPotted.get(i);
                carromPotted.remove(i);
                return carromMen;
            }
        }
        return null;
    }

    private boolean checkMyWin(ArrayList<Carrom> p1CarromPotted, ArrayList<Carrom> p2CarromPotted, Player currentTurn,
            ArrayList<Carrom> hisPocketedCoins, CoinsContainer carromsOnBoard) {
        // If pocketed my all coin and queen => my win
        boolean isQueenOnBoard = false;
        for (Carrom carrom : carromsOnBoard.carroms) {
            if (carrom.type == Carrom.Type.QUEEN) {
                isQueenOnBoard = true;
                break;
            }
        }
        if (!isQueenOnBoard) {
            if (currentTurn == Player.PLAYER_1) {
                // no my carrom on board
                for (Carrom carrom : carromsOnBoard.carroms) {
                    if (carrom.type == Carrom.Type.CARROM_MEN_1) {
                        return false;
                    }
                }
                return true; // my carrom on board not found
            } else {
                // no my carrom on board
                for (Carrom carrom : carromsOnBoard.carroms) {
                    if (carrom.type == Carrom.Type.CARROM_MEN_2) {
                        return false;
                    }
                }
                return true; // my carrom on board not found
            }
        } else {
            return false; // queen on board game not over
        }
    }

    private boolean checkOpponentWin(CoinsContainer carromsOnBoard, Player currentTurn) {
        // opponentPocketed coin + already pocketed opponent coins => so that no
        // opponent coin on board => opponent win
        if (currentTurn == Player.PLAYER_1) {
            for (Carrom carrom : carromsOnBoard.carroms) {
                if (carrom.type == Carrom.Type.CARROM_MEN_2) {
                    return false; // one or more opponent coin are still on board
                }
            }

            return true; // no opponent coin found on board

        } else {
            for (Carrom carrom : carromsOnBoard.carroms) {
                if (carrom.type == Carrom.Type.CARROM_MEN_1) {
                    return false; // one or more opponent coin are still on board
                }
            }

            return true; // no opponent coin found on board
        }
    }

    private boolean checkCoinFoul(CoinsContainer carromsOnBoard, Player currentTurn) {
        // If pocket all mu coin and queen remain on board => foulCoin

        boolean isQueenOnBoard = false;
        for (Carrom carrom : carromsOnBoard.carroms) {
            if (carrom.type == Carrom.Type.QUEEN) {
                isQueenOnBoard = true;
                break;
            }
        }

        if (currentTurn == Player.PLAYER_1) {
            for (Carrom carrom : carromsOnBoard.carroms) {
                if (carrom.type == Carrom.Type.CARROM_MEN_1) {
                    return false; // one or more my coin are still on board
                }
            }

            return isQueenOnBoard; // if queen is on board => coinFoul
        } else {
            for (Carrom carrom : carromsOnBoard.carroms) {
                if (carrom.type == Carrom.Type.CARROM_MEN_2) {
                    return false; // one or more my coin are still on board
                }
            }

            return isQueenOnBoard; // if queen is on board => coinFoul
        }
    }

    private boolean checkOnlyPockedQueenButNoCover(ArrayList<Carrom> hisPocketedCoins) {
        // check on pocketed queen => my turn again , need to pocked cover
        return hisPocketedCoins.size() == 1 && hisPocketedCoins.get(0).type == Carrom.Type.QUEEN;
    }

    private boolean checkAtleastPocketedOneCoinToGetTurnAgain(ArrayList<Carrom> hisPocketedCoins) {
        // If My pocketed coin at least 1 => my turn again
        return hisPocketedCoins.size() >= 1;
    }

    private boolean checkFailedToPocketCoverForQueen(Player currentTurn) {
        // If My previous my turn and only queen pocketed and this time failed to pocket
        // cover => queen on board
        return (previousTurn == currentTurn) && isOnlyQueenPocketedLast;
    }

    private boolean checkIsStrikerPocketed(ResultPlay resultPlay) {
        // If i am pocketed striker => insert one carrom on board and turn change
        return resultPlay.isStrikerPotted;
    }

    public interface ActionCallback {
        void foulStriker(Player player, Carrom carromToPace1);

        void foulCoin(Player player, Carrom carromToPace1, Carrom carromToPace2);

        void needToPocketCover(Player player);

        void insertCarromToBoard(Player player, Carrom carrom);

        void coinPocketed(ArrayList<Carrom> carromsP1, ArrayList<Carrom> carromsP2);
    }

    public enum Player {
        PLAYER_1,
        PLAYER_2
    }

}
