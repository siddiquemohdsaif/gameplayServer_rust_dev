package com.alfa.carromclash.GameEngine.Core;


import org.json.JSONObject;

public class SimulatorEvent {
    public String eventType;
    public double px;
    public double py;
    public double vx;
    public double vy;
    public int sp;

    public SimulatorEvent(String eventType, double px, double py, double vx, double vy, int sp) {
        this.eventType = eventType;
        this.px = px;
        this.py = py;
        this.vx = vx;
        this.vy = vy;
        this.sp = sp;
    }

    public static SimulatorEvent fromJSONString(String jsonString) {
        JSONObject jsonObject = new JSONObject(jsonString);
        String eventType = jsonObject.getString("eventType");
        JSONObject eventData = jsonObject.getJSONObject("eventData");
        double px = eventData.getDouble("px");
        double py = eventData.getDouble("py");
        double vx = eventData.getDouble("vx");
        double vy = eventData.getDouble("vy");
        int sp = eventData.getInt("sp");

        return new SimulatorEvent(eventType, px, py, vx, vy, sp);
    }
}