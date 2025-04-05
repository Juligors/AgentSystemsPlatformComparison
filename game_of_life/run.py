#!/usr/bin/env python3
import time
import subprocess
import os

MAPS_FOLDER = './maps'
INPUT_NAMES = ['10x10board.csv', '100x100board.csv', '1000x1000board.csv']
BIN_PATH = 'game_of_life'

REPETITIONS = 10
NUM_STEPS = 10000  # Probably should be in a separate file since its same for all simulations

directory = os.getcwd()
os.chmod(BIN_PATH,  0b111101101)

for input_name in INPUT_NAMES:
    start = time.time()

    print([BIN_PATH, os.path.join(MAPS_FOLDER,  input_name), str(NUM_STEPS)])
    for rep in range(REPETITIONS):
        subprocess.Popen([BIN_PATH, os.path.join(MAPS_FOLDER,  input_name), str(NUM_STEPS)])

    end = time.time()

    print(f"input: {input}, rep: {REPETITIONS}, iterations: {NUM_STEPS}")
    print("1 iteration time:", (end - start) / (REPETITIONS * NUM_STEPS))
    print("Total time: ", (end - start))
    print("---")