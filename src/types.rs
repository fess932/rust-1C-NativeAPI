enum EnumVar {
    TypeEmpty = 0,
    TypeNull,
    TypeI2,                   //int16_t
    TypeI4,                   //int32_t
    TypeR4,                   //float
    TypeR8,                   //double
    TypeDate,                 //DATE (double)
    TypeTm,                   //struct tm
    TypePstr,                 //struct str    string
    TypeInterface,            //struct iface
    TypeError,                //int32_t errCode
    TypeBool,                 //bool
    TypeVariant,              //struct _tVariant *
    TypeI1,                   //int8_t
    TypeUi1,                  //uint8_t
    TypeUi2,                  //uint16_t
    TypeUi4,                  //uint32_t
    TypeI8,                   //int64_t
    TypeUi8,                  //uint64_t
    TypeInt,                  //int   Depends on architecture
    TypeUint,                 //unsigned int  Depends on architecture
    TypeHresult,              //long hRes
    TypePwstr,                //struct wstr
    TypeBlob,                 //means in struct str binary data contain
    TypeClsid,                //UUID
    TypeStrBlob = 0xfff,
    TypeVector = 0x1000,
    TypeArray = 0x2000,
    TypeByref = 0x4000,    //Only with struct _tVariant *
    TypeReserved = 0x8000,
    TypeIllegal = 0xffff,
    // VTYPE_ILLEGALMASKED    = 0xfff,
    // VtypeTypemask = 0xfff
}

#[no_mangle]
pub struct tVariant {
    i8Val: i8,
    shortVal: i16,
    lVal: i32,
    intVal: usize,
}