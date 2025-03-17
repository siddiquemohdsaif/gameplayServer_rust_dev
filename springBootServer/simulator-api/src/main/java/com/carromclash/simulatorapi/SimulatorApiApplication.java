package com.carromclash.simulatorapi;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class SimulatorApiApplication {

	public static final Logger logger = LoggerFactory.getLogger(SimulatorApiApplication.class);


	public static void main(String[] args) {
		SpringApplication.run(SimulatorApiApplication.class, args);
		logger.info("Application started successfully");
	}

}
