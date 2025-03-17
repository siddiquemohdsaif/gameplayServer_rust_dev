package com.alfa.carromclash.GameEngine.Core.Physic.Engine.V1;

public class ConstantsCarrom {
    public static double mu = 0.18;
    public static double mur = 0.085;
    public static double g =  9.80665; //9.80
    public static double rho = 0.002;
    public static double m = 0.018;
    public static double R = 0.0202565;
    public static double Mz = ((mu * m * g * 2) / 3) * rho;
    public static double Mxy = (7d / (5 * Math.sqrt(2))) * R * mur * m * g;
    public static double e = 0.97d;
    public static double I = (2d / 5d) * m * R * R;

}
