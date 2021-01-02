mod types;

use std::thread::sleep;
use std::time::Duration;
use std::os::raw::c_long;
use widestring::WideChar;

#[no_mangle]
pub extern fn process() {
    println!("Hello, world!");
    sleep(Duration::from_secs(5));
}



//////////// NATIVE API MUST HAVE THIS
#[no_mangle]
pub extern fn GetClassNames() -> WideChar {
    return 32
}

#[no_mangle]
pub extern fn GetClassObject(
    clsName:WideChar,
    // pIntf:IComponentBase // todo IComponent
) -> c_long {
    return 32
}

// pub extern fn DestroyObject(pIntf: IComponentBase) // IComponentBase todo

pub enum AppCapabilities {
    eAppCapabilitiesInvalid = -1,
    eAppCapabilities1 = 1,
    eAppCapabilitiesLast = 1 -1
}

#[no_mangle]
pub extern fn SetPlatformCapabilities(capabilities: AppCapabilities) -> AppCapabilities {
    return capabilities
}
///////////////// END NATIVE API

#[no_mangle]
pub extern fn Init(Interface: std::ffi::c_void) -> bool {
    return true
}

// bool setMemManager(void* memManager) todo

#[no_mangle]
pub extern fn GetInfo() -> c_long {
    return 1000
}

#[no_mangle]
pub extern fn Done() {}

// bool RegisterExtensionAs(WCHAR_T** wsExtName) todo

#[no_mangle]
pub extern fn GetNProps() -> c_long {
    return 10
}

#[no_mangle]
pub extern fn FindProp(wsPropName: WideChar) -> c_long {
    return 10
}

#[no_mangle]
pub extern fn GetPropName(lPropNum: c_long, lPropAlias: c_long) -> WideChar {
    return 10
}


#[no_mangle]
pub extern fn GetPropVal(lPropNum: c_long, pvarPropVal: *const types::tVariant) -> bool {
    return true
}















