package com.carromclash.simulatorapi.controller;

import org.json.JSONObject;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.alfa.carromclash.NativeBridge;
import com.alfa.carromclash.SumulationAPI;
import com.alfa.carromclash.GameEngine.Core.Simulator;

@RestController
public class ServerController {

    @RequestMapping("/helloserver")
    public String helloServer(){
        return NativeBridge.helloWorld();
    }

    @RequestMapping("/hellojavaserver")
    public String hellojavaServer(){
        return "Hello world from java.";
    }



    @PostMapping("/simulate")
    public String getSimulation(@RequestBody SimulationRequest simulationRequest){
        String gameStateSting = simulationRequest.getGameStateSting();
        String simulatorEventString = simulationRequest.getSimulatorEventString();

        try{
            
        //    return SumulationAPI.getSimulation(gameStateSting, simulatorEventString) + "hii";

        String r = SumulationAPI.getSimulation(gameStateSting, simulatorEventString);

        JSONObject jsonObject = new JSONObject();
        jsonObject.put("posX", Simulator.posX);
        jsonObject.put("posY", Simulator.posY);
        jsonObject.put("pointer", Simulator.pointer);


        return r + jsonObject.toString();

        }catch(Exception e){
            return e.toString();
        }
    
    }

    public static class SimulationRequest {
        private String gameStateSting;
        private String simulatorEventString;
    
        // Default constructor
        public SimulationRequest() {
        }
    
        // Constructor with fields
        public SimulationRequest(String gameStateSting, String simulatorEventString) {
            this.gameStateSting = gameStateSting;
            this.simulatorEventString = simulatorEventString;
        }
    
        // Getters
        public String getGameStateSting() {
            return gameStateSting;
        }
    
        public String getSimulatorEventString() {
            return simulatorEventString;
        }
    
        // Setters
        public void setGameStateSting(String gameStateSting) {
            this.gameStateSting = gameStateSting;
        }
    
        public void setSimulatorEventString(String simulatorEventString) {
            this.simulatorEventString = simulatorEventString;
        }
    }
    
}
