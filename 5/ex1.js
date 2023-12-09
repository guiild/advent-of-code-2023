import { input } from "./input.js";

const inputToInstr = (str) => str.split('\n').slice(1).map((row) => row.split(' ').map((e) => Number(e)));

const toArr = input.split('\n\n');
const seeds = toArr[0].split(': ')[1].split(' ').map((e) => Number(e));
const seedToSoilInstr = inputToInstr(toArr[1]);
const soilToFertilizerInstr = inputToInstr(toArr[2]);
const fertilizerToWaterInstr = inputToInstr(toArr[3]);
const waterToLightInstr = inputToInstr(toArr[4]);
const lightToTempInstr = inputToInstr(toArr[5]);
const tempToHumidityInstr = inputToInstr(toArr[6]);
const humidityToLocationInstr = inputToInstr(toArr[7]);

const applyInstr = (instructions, input) => {
    for (const instruction of instructions) {
        if (input >= instruction[1] && input <= instruction[1] + instruction[2]) {
            return instruction[0] + (input - instruction[1]);
        }
    }
    return input;
};

console.log(Math.min(...seeds.map((seed) => applyInstr(humidityToLocationInstr,
    applyInstr(tempToHumidityInstr,
        applyInstr(lightToTempInstr,
            applyInstr(waterToLightInstr,
                applyInstr(fertilizerToWaterInstr,
                    applyInstr(soilToFertilizerInstr,
                        applyInstr(seedToSoilInstr, seed))))))))));