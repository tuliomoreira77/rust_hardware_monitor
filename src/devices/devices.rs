use crate::win_ring0_c::runner::SimpleWinDriverRunner;

use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::{Nvml, Device};
use super::sensor_info::{self, SensorInfo, SensorTuple, DeviceTuple};

pub struct NvidiaVideoCard<'nvml> {
    device: Device<'nvml>,
    sensors_info: [SensorInfo; 6]
}

impl <'nvml> NvidiaVideoCard<'nvml> {
    const DEVICE_NAME: &'static str = "GPU";

    pub fn new(nvml: &'nvml Nvml) -> NvidiaVideoCard<'nvml> {
        return NvidiaVideoCard {
            device: nvml.device_by_index(0).unwrap(),
            sensors_info: [
                build_sensor_info(sensor_info::SENSOR_TEMP, sensor_info::GPU_INFO, 0),
                build_sensor_info(sensor_info::SENSOR_CLK_MEM, sensor_info::GPU_INFO, 0),
                build_sensor_info(sensor_info::SENSOR_CLK_PRC, sensor_info::GPU_INFO, 0),
                build_sensor_info(sensor_info::SENSOR_MEM_USE, sensor_info::GPU_INFO, 0),
                build_sensor_info(sensor_info::SENSOR_PRC_USE, sensor_info::GPU_INFO, 0),
                build_sensor_info(sensor_info::SENSOR_FPS, sensor_info::GPU_INFO, 0),
                ]
        }
    }

    pub fn memory_usage(&self) -> u32 {
        return ((self.device.memory_info().unwrap().used * 100)/self.device.memory_info().unwrap().total) as u32;
    }

    pub fn memory_clock(&self) -> u32 {
        return self.device.clock_info(Clock::Memory).unwrap();
    }

    pub fn processing_usage(&self) -> u32 {
        return self.device.utilization_rates().unwrap().gpu;
    }

    pub fn processing_clock(&self) -> u32 {
        return self.device.clock_info(Clock::Graphics).unwrap();
    }

    pub fn get_temperature(&self) -> u32 {
        return self.device.temperature(TemperatureSensor::Gpu).unwrap();
    }

    pub fn get_fps(&self) -> u32 {
        return  self.device.fbc_stats().unwrap().average_fps;
    }
 
    pub fn refresh(&mut self) {
        self.sensors_info[0].value = self.get_temperature();
        self.sensors_info[1].value = self.memory_clock();
        self.sensors_info[2].value = self.processing_clock();
        self.sensors_info[3].value = self.memory_usage();
        self.sensors_info[4].value = self.processing_usage();
        self.sensors_info[5].value = self.get_fps();
    }

    pub fn get_sensor_info(&self) -> &[SensorInfo] {
        return &self.sensors_info;
    }

}

const AMD_TEMP_ADDR: u32 = 0x00059800;
const FAMILY_17H_M01H_THM_TCON_TEMP_RANGE_SEL: u32 = 0x80000;
pub struct AmdProcessor<'driver> {
    driver: &'driver SimpleWinDriverRunner,
    sensors_info: [SensorInfo; 1]
}

impl <'driver> AmdProcessor<'driver> {
    pub fn new(win_driver: &SimpleWinDriverRunner) -> AmdProcessor {
        return AmdProcessor { 
            driver: win_driver , 
            sensors_info: [
                build_sensor_info(sensor_info::SENSOR_TEMP, sensor_info::CPU_INFO, 0)
                ]
        }
    }

    pub fn get_temperature(&self) -> u32 {
        let out = self.driver.read_smu(AMD_TEMP_ADDR);
        let mut raw_temp = (((out >> 21) & 0x7FF) / 8) as u32;
        if (out & FAMILY_17H_M01H_THM_TCON_TEMP_RANGE_SEL as u64) != 0 {
            raw_temp -= 49;
        }
        return raw_temp;
    }

    pub fn refresh(&mut self) {
        self.sensors_info[0].value = self.get_temperature();
    }

    pub fn get_sensor_info(&self) -> &[SensorInfo] {
        return &self.sensors_info;
    }
}

fn build_sensor_info(sensor_info: SensorTuple, device_info: DeviceTuple, value: u32) -> SensorInfo {
    return SensorInfo {
        code: sensor_info.0,
        device_code: device_info.0 ,
        name: sensor_info.1, 
        device_name: device_info.1,
        value: value
    };
}