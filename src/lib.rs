#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    deref_nullptr
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn get_sdk_ver() -> u32 {
    match std::env::consts::OS {
        "linux" => 0x3010300,
        "windows" => 0x3050303,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ver = unsafe { MV_CC_GetSDKVersion() };
        assert_eq!(ver, get_sdk_ver());
    }
}
