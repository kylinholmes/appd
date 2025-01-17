use webd::utils::get_runtime_hardware_info;

#[test]
pub fn get_rt_hw_info() {
    println!("{}", get_runtime_hardware_info());
}