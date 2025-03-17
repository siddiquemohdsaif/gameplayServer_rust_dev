#include "com_alfa_carromclash_NativeBridge.h"
#include <jni.h>
#include <string>
#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <sstream>
#include <string>
#include <cstring>
#include <iomanip>
#include <ctime>
#include <array>
#include "PhysicEngine/Simulator.h"
#include "PhysicEngine/Util/CoinsContainer.h"
#include "PhysicEngine/Model/Ball.h"
#include "PhysicEngine/Model/Table.h"
#include "PhysicEngine/Maths/Vector3.h"


static Simulator *simulator {nullptr};

void releaseOldSimulation() {
    if (simulator != nullptr){
        delete simulator;
        simulator = nullptr;
    }
}


extern "C" {

    //Java_com_carromclash_simulatorapi_HelloWorldNative_helloWorld
    JNIEXPORT jstring JNICALL Java_com_alfa_carromclash_NativeBridge_helloWorld(JNIEnv *env, jobject obj) {
        return env->NewStringUTF("Hello World from C++ .");
    }


    JNIEXPORT void JNICALL Java_com_alfa_carromclash_NativeBridge_startSimulation(JNIEnv *env, jclass clazz, jobject simulatorJObject) {
        jclass simulatorClass = env->GetObjectClass(simulatorJObject);

        // Get field IDs
        jfieldID pxFieldID = env->GetFieldID(simulatorClass, "px", "D");
        jfieldID pyFieldID = env->GetFieldID(simulatorClass, "py", "D");
        jfieldID vxFieldID = env->GetFieldID(simulatorClass, "vx", "D");
        jfieldID vyFieldID = env->GetFieldID(simulatorClass, "vy", "D");
        jfieldID deactivatePocketFieldID = env->GetFieldID(simulatorClass, "deactivatePocket", "Z");
        jfieldID tableFieldID = env->GetFieldID(simulatorClass, "table", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Table;");
    
        // Get values from Java Simulator object
        double px = env->GetDoubleField(simulatorJObject, pxFieldID);
        double py = env->GetDoubleField(simulatorJObject, pyFieldID);
        double vx = env->GetDoubleField(simulatorJObject, vxFieldID);
        double vy = env->GetDoubleField(simulatorJObject, vyFieldID);
        jboolean j_deactivatePocket = env->GetBooleanField(simulatorJObject, deactivatePocketFieldID);
        bool deactivatePocket = (j_deactivatePocket == JNI_TRUE);
        jobject tableJObject = env->GetObjectField(simulatorJObject, tableFieldID);
    
        jclass tableClass = env->GetObjectClass(tableJObject);
        jfieldID ballsFieldID = env->GetFieldID(tableClass, "balls", "Ljava/util/ArrayList;");
    
        jobject ballsJObject = env->GetObjectField(tableJObject, ballsFieldID);
        jclass arrayListClass = env->GetObjectClass(ballsJObject);
        jmethodID arrayListSizeMethodID = env->GetMethodID(arrayListClass, "size", "()I");
        jmethodID arrayListGetMethodID = env->GetMethodID(arrayListClass, "get", "(I)Ljava/lang/Object;");
    
        int ballsSize = env->CallIntMethod(ballsJObject, arrayListSizeMethodID);
        std::vector<Ball> balls;
    
        for (int i = 0; i < ballsSize; i++) {
            jobject ballJObject = env->CallObjectMethod(ballsJObject, arrayListGetMethodID, i);
            jclass ballClass = env->GetObjectClass(ballJObject);
    
            jfieldID posFieldID = env->GetFieldID(ballClass, "pos", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Vector3;");
            jfieldID velFieldID = env->GetFieldID(ballClass, "vel", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Vector3;");
            jfieldID rvelFieldID = env->GetFieldID(ballClass, "rvel", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Vector3;");
            jfieldID stateFieldID = env->GetFieldID(ballClass, "state", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Ball$State;");
            jfieldID typeFieldID = env->GetFieldID(ballClass, "type", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Ball$Type;");
            jfieldID coinCodeFieldID = env->GetFieldID(ballClass, "coinCode", "I");
            jobject posJObject = env->GetObjectField(ballJObject, posFieldID);
            jobject velJObject = env->GetObjectField(ballJObject, velFieldID);
            jobject rvelJObject = env->GetObjectField(ballJObject, rvelFieldID);
    
            jclass vector3Class = env->GetObjectClass(posJObject);
            jfieldID xFieldID = env->GetFieldID(vector3Class, "x", "D");
            jfieldID yFieldID = env->GetFieldID(vector3Class, "y", "D");
            jfieldID zFieldID = env->GetFieldID(vector3Class, "z", "D");
    
            double posX = env->GetDoubleField(posJObject, xFieldID);
            double posY = env->GetDoubleField(posJObject, yFieldID);
            double posZ = env->GetDoubleField(posJObject, zFieldID);
    
            double velX = env->GetDoubleField(velJObject, xFieldID);
            double velY = env->GetDoubleField(velJObject, yFieldID);
            double velZ = env->GetDoubleField(velJObject, zFieldID);
    
            double rvelX = env->GetDoubleField(rvelJObject, xFieldID);
            double rvelY = env->GetDoubleField(rvelJObject, yFieldID);
            double rvelZ = env->GetDoubleField(rvelJObject, zFieldID);
    
            Ball::State state = Ball::State::Stationary;
            Ball::Type type = Ball::Type::Carrom;
    
            int coinCode = env->GetIntField(ballJObject, coinCodeFieldID);
            if (coinCode != 99){
                Vector3 pos(posX, posY, posZ);
                Vector3 vel(velX, velY, velZ);
                Vector3 rvel(rvelX, rvelY, rvelZ);
    
                Ball ball(pos, vel, rvel, state, type, coinCode);
                balls.push_back(ball);
            }
    
        }
    
    
    
        // Release the previous simulation (if any) and create a new one
        releaseOldSimulation();
        jfieldID lastTimeFieldID = env->GetFieldID(simulatorClass, "lastTime", "J"); // Field ID for lastTime
        jlong lastTimeJLong = env->GetLongField(simulatorJObject, lastTimeFieldID); // Get lastTime as jlong
        uint64_t lastTime = static_cast<uint64_t>(lastTimeJLong);
        uint64_t last = lastTime;
        Table::deactivatePocket = deactivatePocket;  // use to disable pocket
        simulator = new Simulator(px, py, vx, vy,last);
        simulator->start(balls);

    }


    JNIEXPORT void JNICALL Java_com_alfa_carromclash_NativeBridge_stimulateToCurrentTime(JNIEnv *env, jclass clazz, jobject simulatorJObject) {
        jclass simulatorClass = env->GetObjectClass(simulatorJObject);
        jfieldID lastTimeFieldID = env->GetFieldID(simulatorClass, "lastTime", "J"); // Field ID for lastTime
        jlong lastTimeJLong = env->GetLongField(simulatorJObject, lastTimeFieldID); // Get lastTime as jlong
        uint64_t lastTime = static_cast<uint64_t>(lastTimeJLong);

        if (simulator) {
            simulator->stimulateToCurrentTime(lastTime);

            // Update (isStimulationFinish, (table.balls.pos.x, table.balls.pos.y, table.balls.pos.z) for all balls find by coin id, table.balls.pocket = true (if != null) ) variable of simulatorJObject by simulator
            jfieldID isStimulationFinishField = env->GetFieldID(simulatorClass, "isStimulationFinish", "Z");
            env->SetBooleanField(simulatorJObject, isStimulationFinishField, simulator->isStimulationFinish());

            // Get table field from simulatorJObject
            jfieldID tableField = env->GetFieldID(simulatorClass, "table", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Table;");
            jobject tableJObject = env->GetObjectField(simulatorJObject, tableField);
            jclass tableClass = env->GetObjectClass(tableJObject);

            std::vector<Ball> balls = simulator->table->balls;
            for (Ball &ball : balls) {
                int coinId = ball.coinCode;
                jobject ballJObject = env->CallObjectMethod(tableJObject, env->GetMethodID(tableClass, "getBallByCoinId", "(I)Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Ball;"), coinId);

                jclass ballClass = env->GetObjectClass(ballJObject);
                jfieldID posField = env->GetFieldID(ballClass, "pos", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Vector3;");
                jobject posJObject = env->GetObjectField(ballJObject, posField);
                jclass vector3Class = env->GetObjectClass(posJObject);
                jfieldID posXField = env->GetFieldID(vector3Class, "x", "D");
                jfieldID posYField = env->GetFieldID(vector3Class, "y", "D");
                jfieldID posZField = env->GetFieldID(vector3Class, "z", "D");
                jfieldID pocketField = env->GetFieldID(ballClass, "pocket", "Z");

                jfieldID rvelField = env->GetFieldID(ballClass, "rvel", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Vector3;");
                jobject rvelJObject = env->GetObjectField(ballJObject, rvelField);

                jfieldID velField = env->GetFieldID(ballClass, "vel", "Lcom/alfa/carromclash/GameEngine/Core/Physic/Engine/V1/Vector3;");
                jobject velJObject = env->GetObjectField(ballJObject, velField);


                env->SetDoubleField(posJObject, posXField, ball.pos.x);
                env->SetDoubleField(posJObject, posYField, ball.pos.y);
                env->SetDoubleField(posJObject, posZField, ball.pos.z);
                env->SetBooleanField(ballJObject, pocketField, ball.pocket != nullptr);
                env->SetDoubleField(rvelJObject, posXField, ball.rvel.x);
                env->SetDoubleField(rvelJObject, posYField, ball.rvel.y);
                env->SetDoubleField(rvelJObject, posZField, ball.rvel.z);
                env->SetDoubleField(velJObject, posXField, ball.vel.x);
                env->SetDoubleField(velJObject, posYField, ball.vel.y);
                env->SetDoubleField(velJObject, posZField, ball.vel.z);

            }


        
            // Clear outcomes
            simulator->table->outcomes.clear();

        }
    }

    
    JNIEXPORT jobjectArray JNICALL Java_com_alfa_carromclash_NativeBridge_insertNewCarrom(JNIEnv *env, jclass clazz, jobjectArray carrom_array, jdoubleArray carrom) {
        // Convert Java arrays to C++ vectors
        jsize carromArrayLength = env->GetArrayLength(carrom_array);
        std::vector<std::array<double, 3>> carromArray(carromArrayLength);
        for (jsize i = 0; i < carromArrayLength; i++) {
            jdoubleArray carromEntry = (jdoubleArray) env->GetObjectArrayElement(carrom_array, i);
            jdouble *carromEntryElements = env->GetDoubleArrayElements(carromEntry, nullptr);
            carromArray[i] = {carromEntryElements[0], carromEntryElements[1], carromEntryElements[2]};
            env->ReleaseDoubleArrayElements(carromEntry, carromEntryElements, JNI_ABORT);
        }
    
        jdouble *carromElements = env->GetDoubleArrayElements(carrom, nullptr);
        std::array<double, 3> carromCpp = {carromElements[0], carromElements[1], carromElements[2]};
        env->ReleaseDoubleArrayElements(carrom, carromElements, JNI_ABORT);
    
        // Call the C++ function
        std::vector<std::array<double, 3>> updatedCarroms = CoinsContainer::insertNewCarrom(carromArray, carromCpp);
    
        // Convert the result back to a Java array
        jobjectArray result = env->NewObjectArray(updatedCarroms.size(), env->FindClass("[D"), nullptr);
        for (size_t i = 0; i < updatedCarroms.size(); i++) {
            jdoubleArray carromEntry = env->NewDoubleArray(3);
            env->SetDoubleArrayRegion(carromEntry, 0, 3, updatedCarroms[i].data());
            env->SetObjectArrayElement(result, i, carromEntry);
            env->DeleteLocalRef(carromEntry);
        }
    
        return result;

    }
}



/**
 * command for .so build:

# 32 bit windows :
g++ -m64 -shared -o libhelloworld.so -fPIC HelloWorld.cpp -I"C:\Program Files\Java\jdk-17\include" -I"C:\Program Files\Java\jdk-17\include\win32"
g++ -m64 -shared -o libcarromclashnativebridge.so -fPIC carromClashNativeBridge.cpp -I"C:\Program Files\Java\jdk-17\include" -I"C:\Program Files\Java\jdk-17\include\win32"

# 64 bit windows :
g++ -m64 -shared -o libcarromclashnativebridge.so -fPIC carromClashNativeBridge.cpp PhysicEngine/*.cpp PhysicEngine/Maths/*.cpp PhysicEngine/Model/*.cpp PhysicEngine/Util/*.cpp -I"C:\Program Files\Java\jdk-17\include" -I"C:\Program Files\Java\jdk-17\include\win32"

# 64 bit linux/ubuntu 20.x :
g++ -m64 -shared -o libcarromclashnativebridge.so -fPIC carromClashNativeBridge.cpp PhysicEngine/*.cpp PhysicEngine/Maths/*.cpp PhysicEngine/Model/*.cpp PhysicEngine/Util/*.cpp -I"/usr/lib/jvm/java-17-openjdk-amd64/include" -I"/usr/lib/jvm/java-17-openjdk-amd64/include/linux"

*/