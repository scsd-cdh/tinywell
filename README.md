# Tinywell - Microfluidic Experiment Platform for Yeast Growth

## Overview
This project presents a microfluidic experiment platform designed to test yeast growth and viability under various LED wavelengths. It allows for simultaneous testing of up to 16 wells, providing an efficient and innovative approach to studying yeast behavior.

## Contents
The repository is organized into four main folders:

### 1. Altium
Contains all the schematics and assembly files for the PCBs (Printed Circuit Boards) used in the project. These files are essential for understanding the electronic design and layout.

### 2. CAD
Holds all CAD (Computer-Aided Design) files, created using NX software. These files represent the physical design and structure of the experimental platform.

### 3. Dashboard
Includes the user interface code, developed in Rust. This folder is crucial for those looking to modify or understand the front-end aspect of the platform.

### 4. Teensyduino
Contains the Teensyduino code responsible for controlling the boards designed in the Altium folder. This is where you'll find the firmware and related software for board operations.

## Getting Started

### Prerequisites
1. Assemble the PCBs designed in the altium folder. This requires some skill as both the LEDs and light sensors were quite small. Our team struggled at first and recommend getting extras for practice.
2. Print the CADs in the provided folder. Our team used PLA.
3. Download the exe provided in the release or build the program in the dashboard folder.
4. Upload the .ino file in the Teensyduino folder onto your Teensyduino.

### Usage
Once everything is assembled, you should be able to run the dashboard, connect your Teensyduino to your computer, design the pattern for the LEDs and run the sequence.
A file with the results will be generated in the folder where the executable is. You may change the file by going to File in the menu bar and clicking "Results Directory".

## License
This project is licensed under the GNU General Public License (GPL). This means that it's free to use and modify, and it ensures that all derivative works are also open-source. The GPL is a widely used free software license, guaranteeing end users the freedom to run, study, share, and modify the software.
For more details, see the GPLv3 license or the LICENSE file included in the repository.
