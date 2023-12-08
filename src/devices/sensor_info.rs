
pub const SENSOR_TEMP_CODE: u16 = 0x0000;
pub const SENSOR_CLK_MEM_CODE: u16 = 0x0101;
pub const SENSOR_CLK_PRC_CODE: u16 = 0x0102;
pub const SENSOR_MEM_USE_CODE: u16 = 0x0201;
pub const SENSOR_PRC_USE_CODE: u16 = 0x0202;

pub const SENSOR_TEMP_NAME: &str = "Temp";
pub const SENSOR_CLK_MEM_NAME: &str = "Mem Clock";
pub const SENSOR_CLK_PRC_NAME: &str = "Core Clock";
pub const SENSOR_MEM_USE_NAME: &str = "Mem Use";
pub const SENSOR_PRC_USE_NAME: &str = "Core Use";

pub const GPU_DVC_CODE: u16 = 0x00;
pub const CPU_DVC_CODE: u16 = 0x01;

pub const GPU_DVC_NAME: &str = "GPU";
pub const CPU_DVC_NAME: &str = "CPU";


pub type SensorTuple = (u16, &'static str);
pub const SENSOR_TEMP: SensorTuple = (SENSOR_TEMP_CODE, SENSOR_TEMP_NAME);
pub const SENSOR_CLK_MEM: SensorTuple = (SENSOR_CLK_MEM_CODE, SENSOR_CLK_MEM_NAME);
pub const SENSOR_CLK_PRC: SensorTuple = (SENSOR_CLK_PRC_CODE, SENSOR_CLK_PRC_NAME);
pub const SENSOR_MEM_USE: SensorTuple = (SENSOR_MEM_USE_CODE, SENSOR_MEM_USE_NAME);
pub const SENSOR_PRC_USE: SensorTuple = (SENSOR_PRC_USE_CODE, SENSOR_PRC_USE_NAME);

pub type DeviceTuple = (u16, &'static str);
pub const GPU_INFO: DeviceTuple = (GPU_DVC_CODE, GPU_DVC_NAME);
pub const CPU_INFO: DeviceTuple = (CPU_DVC_CODE, CPU_DVC_NAME);

pub struct SensorInfo {
    pub code: u16,
    pub device_code: u16,
    pub name: &'static str, 
    pub device_name: &'static str,
    pub value: u32
}