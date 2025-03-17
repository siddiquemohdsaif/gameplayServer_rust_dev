package com.alfa.carromclash;


import java.io.File;

import com.alfa.carromclash.GameEngine.Core.Simulator;
import com.carromclash.simulatorapi.SimulatorApiApplication;


public class NativeBridge {

    static {
        try{ 
            // # for debug local
            // String jarPath = NativeBridge.class.getProtectionDomain().getCodeSource().getLocation().toURI().getPath();
            // String dirPath = new File(jarPath).getParentFile().getPath();
            // System.load(dirPath + "/libcarromclashnativebridge.so");
            // System.out.println("loaded:" +dirPath + "/libcarromclashnativebridge.so");

            
            // # for debug local in wsl ubuntu 20.x
            //System.load("/home/shabir/SpringBoot/CarromClash/simulator-api/release/" + "/libcarromclashnativebridge.so");
            

            // # for release run on server
            System.load("/rust/GamePlayServerNode/springBootServer/simulator-api/release/" + "libcarromclashnativebridge.so");
            

            
            SimulatorApiApplication.logger.info("loaded-----------");

        }catch(Exception e){
            e.printStackTrace();
            System.out.println("not loaded");
            SimulatorApiApplication.logger.info("not loaded:" +e.getCause());
        }
   }


    public static native void startSimulation(Simulator simulator);
    public static native void stimulateToCurrentTime(Simulator simulator);

    public static native double[][] insertNewCarrom(double[][] carromArray,double[] carrom);
    public static native String helloWorld();

}
