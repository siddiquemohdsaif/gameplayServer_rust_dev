// #include "SimulationMain.h"
// #include "Maths/Vector3.h"
// #include "Model/Ball.h"
// #include "Simulator.h"
// #include <vector>
// #include <ctime>
// #include <iostream>
// #include <sstream>
// #include <android/log.h>

// #define LOG_TAG "native-lib"
// #define LOGI(...) __android_log_print(ANDROID_LOG_INFO, LOG_TAG, __VA_ARGS__)

// void SimulationMain::startSimulation()
// {

//     std::ostringstream log_message;
//     log_message << "started: " << std::endl;
//     LOGI("%s", log_message.str().c_str());

//     std::vector<Ball> balls;
//     std::vector<Ball> ballsOut;

//     Vector3 pos(0.001, 0.001, 0);
//     Vector3 vel(0, 0, 0);
//     Vector3 rvel(0, 0, 0);
//     Ball::State state = Ball::State::Stationary;

//     Ball ball(pos, vel, rvel, state, Ball::Type::Carrom, 1);
//     balls.push_back(ball);


//     auto now = std::chrono::system_clock::now();
//     uint64_t epoch_time = std::chrono::duration_cast<std::chrono::milliseconds>(now.time_since_epoch()).count();

//     uint64_t sct = 1683732235851;

//     Simulator simulator(0, 0.3, 0, -1,sct);
//     simulator.start(balls);


//     while (!simulator.isStimulationFinish())
//     {
//         ballsOut = simulator.stimulateToCurrentTime(sct);
//         //log(ballsOut, sct);
//         sct += 15;
//     }

//     for (size_t i = 0; i < ballsOut.size(); ++i)
//     {
//         log_message << " exit coincode: " << ballsOut[i].coinCode << " x:" << ballsOut[i].pos.x << " y:" << ballsOut[i].pos.y << std::endl;
//         LOGI("%s", log_message.str().c_str());
//     }
// }

// void SimulationMain::log(const std::vector<Ball>& balls, uint64_t sct) {
//     std::cout << "shabir : stc: " << sct << " coincode 1:" << balls[0].coinCode << " x:" << balls[0].pos.x << " y:" << balls[0].pos.y << " state:" << stateToString(balls[0].state)
//          << "         coincode 2:" << balls[1].coinCode << " x:" << balls[1].pos.x << " y:" << balls[1].pos.y << " state:" << stateToString(balls[1].state) << std::endl;
// }


// std::string SimulationMain::stateToString(const Ball::State& state) {
//     switch (state) {
//         case Ball::State::Stationary:
//             return "Stationary";
//         case Ball::State::Rolling:
//             return "Rolling";
//         case Ball::State::Sliding:
//             return "Sliding";
//         case Ball::State::Falling:
//             return "Falling";
//         case Ball::State::InPocket:
//             return "InPocket";
//         default:
//             return "Unknown";
//     }
// }