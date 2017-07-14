#![feature(inclusive_range_syntax)]

mod ines;
use ines::write_bytes_to_file;

const LEVEL_HEIGHT: u8 = 13;
const LEVELS: [u8; 1872] = [
    //level 1-1,
    0x50, 0x21,
    0x07, 0x81, 0x47, 0x24, 0x57, 0x00, 0x63, 0x01, 0x77, 0x01,
    0xc9, 0x71, 0x68, 0xf2, 0xe7, 0x73, 0x97, 0xfb, 0x06, 0x83,
    0x5c, 0x01, 0xd7, 0x22, 0xe7, 0x00, 0x03, 0xa7, 0x6c, 0x02,
    0xb3, 0x22, 0xe3, 0x01, 0xe7, 0x07, 0x47, 0xa0, 0x57, 0x06,
    0xa7, 0x01, 0xd3, 0x00, 0xd7, 0x01, 0x07, 0x81, 0x67, 0x20,
    0x93, 0x22, 0x03, 0xa3, 0x1c, 0x61, 0x17, 0x21, 0x6f, 0x33,
    0xc7, 0x63, 0xd8, 0x62, 0xe9, 0x61, 0xfa, 0x60, 0x4f, 0xb3,
    0x87, 0x63, 0x9c, 0x01, 0xb7, 0x63, 0xc8, 0x62, 0xd9, 0x61,
    0xea, 0x60, 0x39, 0xf1, 0x87, 0x21, 0xa7, 0x01, 0xb7, 0x20,
    0x39, 0xf1, 0x5f, 0x38, 0x6d, 0xc1, 0xaf, 0x26,
    0xfd,

    //level 1-3/5-3,
    0x90, 0x11,
    0x0f, 0x26, 0xfe, 0x10, 0x2a, 0x93, 0x87, 0x17, 0xa3, 0x14,
    0xb2, 0x42, 0x0a, 0x92, 0x19, 0x40, 0x36, 0x14, 0x50, 0x41,
    0x82, 0x16, 0x2b, 0x93, 0x24, 0x41, 0xbb, 0x14, 0xb8, 0x00,
    0xc2, 0x43, 0xc3, 0x13, 0x1b, 0x94, 0x67, 0x12, 0xc4, 0x15,
    0x53, 0xc1, 0xd2, 0x41, 0x12, 0xc1, 0x29, 0x13, 0x85, 0x17,
    0x1b, 0x92, 0x1a, 0x42, 0x47, 0x13, 0x83, 0x41, 0xa7, 0x13,
    0x0e, 0x91, 0xa7, 0x63, 0xb7, 0x63, 0xc5, 0x65, 0xd5, 0x65,
    0xdd, 0x4a, 0xe3, 0x67, 0xf3, 0x67, 0x8d, 0xc1, 0xae, 0x42,
    0xdf, 0x20,
    0xfd,

    //level 2-1,
    0x52, 0x31,
    0x0f, 0x20, 0x6e, 0x40, 0xf7, 0x20, 0x07, 0x84, 0x17, 0x20,
    0x4f, 0x34, 0xc3, 0x03, 0xc7, 0x02, 0xd3, 0x22, 0x27, 0xe3,
    0x39, 0x61, 0xe7, 0x73, 0x5c, 0xe4, 0x57, 0x00, 0x6c, 0x73,
    0x47, 0xa0, 0x53, 0x06, 0x63, 0x22, 0xa7, 0x73, 0xfc, 0x73,
    0x13, 0xa1, 0x33, 0x05, 0x43, 0x21, 0x5c, 0x72, 0xc3, 0x23,
    0xcc, 0x03, 0x77, 0xfb, 0xac, 0x02, 0x39, 0xf1, 0xa7, 0x73,
    0xd3, 0x04, 0xe8, 0x72, 0xe3, 0x22, 0x26, 0xf4, 0xbc, 0x02,
    0x8c, 0x81, 0xa8, 0x62, 0x17, 0x87, 0x43, 0x24, 0xa7, 0x01,
    0xc3, 0x04, 0x08, 0xf2, 0x97, 0x21, 0xa3, 0x02, 0xc9, 0x0b,
    0xe1, 0x69, 0xf1, 0x69, 0x8d, 0xc1, 0xcf, 0x26,
    0xfd,

    //level 2-3/7-3,
    0x90, 0x11,
    0x0f, 0x26, 0x6e, 0x10, 0x8b, 0x17, 0xaf, 0x32, 0xd8, 0x62,
    0xe8, 0x62, 0xfc, 0x3f, 0xad, 0xc8, 0xf8, 0x64, 0x0c, 0xbe,
    0x43, 0x43, 0xf8, 0x64, 0x0c, 0xbf, 0x73, 0x40, 0x84, 0x40,
    0x93, 0x40, 0xa4, 0x40, 0xb3, 0x40, 0xf8, 0x64, 0x48, 0xe4,
    0x5c, 0x39, 0x83, 0x40, 0x92, 0x41, 0xb3, 0x40, 0xf8, 0x64,
    0x48, 0xe4, 0x5c, 0x39, 0xf8, 0x64, 0x13, 0xc2, 0x37, 0x65,
    0x4c, 0x24, 0x63, 0x00, 0x97, 0x65, 0xc3, 0x42, 0x0b, 0x97,
    0xac, 0x32, 0xf8, 0x64, 0x0c, 0xbe, 0x53, 0x45, 0x9d, 0x48,
    0xf8, 0x64, 0x2a, 0xe2, 0x3c, 0x47, 0x56, 0x43, 0xba, 0x62,
    0xf8, 0x64, 0x0c, 0xb7, 0x88, 0x64, 0xbc, 0x31, 0xd4, 0x45,
    0xfc, 0x31, 0x3c, 0xb1, 0x78, 0x64, 0x8c, 0x38, 0x0b, 0x9c,
    0x1a, 0x33, 0x18, 0x61, 0x28, 0x61, 0x39, 0x60, 0x5d, 0x4a,
    0xee, 0x11, 0x0f, 0xb8, 0x1d, 0xc1, 0x3e, 0x42, 0x6f, 0x20,
    0xfd,

    //level 3-1,
    0x52, 0x31,
    0x0f, 0x20, 0x6e, 0x66, 0x07, 0x81, 0x36, 0x01, 0x66, 0x00,
    0xa7, 0x22, 0x08, 0xf2, 0x67, 0x7b, 0xdc, 0x02, 0x98, 0xf2,
    0xd7, 0x20, 0x39, 0xf1, 0x9f, 0x33, 0xdc, 0x27, 0xdc, 0x57,
    0x23, 0x83, 0x57, 0x63, 0x6c, 0x51, 0x87, 0x63, 0x99, 0x61,
    0xa3, 0x06, 0xb3, 0x21, 0x77, 0xf3, 0xf3, 0x21, 0xf7, 0x2a,
    0x13, 0x81, 0x23, 0x22, 0x53, 0x00, 0x63, 0x22, 0xe9, 0x0b,
    0x0c, 0x83, 0x13, 0x21, 0x16, 0x22, 0x33, 0x05, 0x8f, 0x35,
    0xec, 0x01, 0x63, 0xa0, 0x67, 0x20, 0x73, 0x01, 0x77, 0x01,
    0x83, 0x20, 0x87, 0x20, 0xb3, 0x20, 0xb7, 0x20, 0xc3, 0x01,
    0xc7, 0x00, 0xd3, 0x20, 0xd7, 0x20, 0x67, 0xa0, 0x77, 0x07,
    0x87, 0x22, 0xe8, 0x62, 0xf5, 0x65, 0x1c, 0x82, 0x7f, 0x38,
    0x8d, 0xc1, 0xcf, 0x26,
    0xfd,

    //level 3-2,
    0x96, 0x31,
    0x0f, 0x26, 0x0d, 0x03, 0x1a, 0x60, 0x77, 0x42, 0xc4, 0x00,
    0xc8, 0x62, 0xb9, 0xe1, 0xd3, 0x06, 0xd7, 0x07, 0xf9, 0x61,
    0x0c, 0x81, 0x4e, 0xb1, 0x8e, 0xb1, 0xbc, 0x01, 0xe4, 0x50,
    0xe9, 0x61, 0x0c, 0x81, 0x0d, 0x0a, 0x84, 0x43, 0x98, 0x72,
    0x0d, 0x0c, 0x0f, 0x38, 0x1d, 0xc1, 0x5f, 0x26,
    0xfd,

    //level 3-3,
    0x94, 0x11,
    0x0f, 0x26, 0xfe, 0x10, 0x28, 0x94, 0x65, 0x15, 0xeb, 0x12,
    0xfa, 0x41, 0x4a, 0x96, 0x54, 0x40, 0xa4, 0x42, 0xb7, 0x13,
    0xe9, 0x19, 0xf5, 0x15, 0x11, 0x80, 0x47, 0x42, 0x71, 0x13,
    0x80, 0x41, 0x15, 0x92, 0x1b, 0x1f, 0x24, 0x40, 0x55, 0x12,
    0x64, 0x40, 0x95, 0x12, 0xa4, 0x40, 0xd2, 0x12, 0xe1, 0x40,
    0x13, 0xc0, 0x2c, 0x17, 0x2f, 0x12, 0x49, 0x13, 0x83, 0x40,
    0x9f, 0x14, 0xa3, 0x40, 0x17, 0x92, 0x83, 0x13, 0x92, 0x41,
    0xb9, 0x14, 0xc5, 0x12, 0xc8, 0x40, 0xd4, 0x40, 0x4b, 0x92,
    0x78, 0x1b, 0x9c, 0x94, 0x9f, 0x11, 0xdf, 0x14, 0xfe, 0x11,
    0x7d, 0xc1, 0x9e, 0x42, 0xcf, 0x20,
    0xfd,

    //level 4-1,
    0x52, 0x21,
    0x0f, 0x20, 0x6e, 0x40, 0x58, 0xf2, 0x93, 0x01, 0x97, 0x00,
    0x0c, 0x81, 0x97, 0x40, 0xa6, 0x41, 0xc7, 0x40, 0x0d, 0x04,
    0x03, 0x01, 0x07, 0x01, 0x23, 0x01, 0x27, 0x01, 0xec, 0x03,
    0xac, 0xf3, 0xc3, 0x03, 0x78, 0xe2, 0x94, 0x43, 0x47, 0xf3,
    0x74, 0x43, 0x47, 0xfb, 0x74, 0x43, 0x2c, 0xf1, 0x4c, 0x63,
    0x47, 0x00, 0x57, 0x21, 0x5c, 0x01, 0x7c, 0x72, 0x39, 0xf1,
    0xec, 0x02, 0x4c, 0x81, 0xd8, 0x62, 0xec, 0x01, 0x0d, 0x0d,
    0x0f, 0x38, 0xc7, 0x07, 0xed, 0x4a, 0x1d, 0xc1, 0x5f, 0x26,
    0xfd,

    //level 4-3,
    0x90, 0x51,
    0x0f, 0x26, 0xee, 0x10, 0x0b, 0x94, 0x33, 0x14, 0x42, 0x42,
    0x77, 0x16, 0x86, 0x44, 0x02, 0x92, 0x4a, 0x16, 0x69, 0x42,
    0x73, 0x14, 0xb0, 0x00, 0xc7, 0x12, 0x05, 0xc0, 0x1c, 0x17,
    0x1f, 0x11, 0x36, 0x12, 0x8f, 0x14, 0x91, 0x40, 0x1b, 0x94,
    0x35, 0x12, 0x34, 0x42, 0x60, 0x42, 0x61, 0x12, 0x87, 0x12,
    0x96, 0x40, 0xa3, 0x14, 0x1c, 0x98, 0x1f, 0x11, 0x47, 0x12,
    0x9f, 0x15, 0xcc, 0x15, 0xcf, 0x11, 0x05, 0xc0, 0x1f, 0x15,
    0x39, 0x12, 0x7c, 0x16, 0x7f, 0x11, 0x82, 0x40, 0x98, 0x12,
    0xdf, 0x15, 0x16, 0xc4, 0x17, 0x14, 0x54, 0x12, 0x9b, 0x16,
    0x28, 0x94, 0xce, 0x01, 0x3d, 0xc1, 0x5e, 0x42, 0x8f, 0x20,
    0xfd,

    //level 5-1,
    0x95, 0xb1,
    0x0f, 0x26, 0x0d, 0x02, 0xc8, 0x72, 0x1c, 0x81, 0x38, 0x72,
    0x0d, 0x05, 0x97, 0x34, 0x98, 0x62, 0xa3, 0x20, 0xb3, 0x06,
    0xc3, 0x20, 0xcc, 0x03, 0xf9, 0x91, 0x2c, 0x81, 0x48, 0x62,
    0x0d, 0x09, 0x37, 0x63, 0x47, 0x03, 0x57, 0x21, 0x8c, 0x02,
    0xc5, 0x79, 0xc7, 0x31, 0xf9, 0x11, 0x39, 0xf1, 0xa9, 0x11,
    0x6f, 0xb4, 0xd3, 0x65, 0xe3, 0x65, 0x7d, 0xc1, 0xbf, 0x26,
    0xfd,

    //level 5-2,
    0x55, 0xb1,
    0x0f, 0x26, 0xcf, 0x33, 0x07, 0xb2, 0x15, 0x11, 0x52, 0x42,
    0x99, 0x0b, 0xac, 0x02, 0xd3, 0x24, 0xd6, 0x42, 0xd7, 0x25,
    0x23, 0x84, 0xcf, 0x33, 0x07, 0xe3, 0x19, 0x61, 0x78, 0x7a,
    0xef, 0x33, 0x2c, 0x81, 0x46, 0x64, 0x55, 0x65, 0x65, 0x65,
    0xec, 0x74, 0x47, 0x82, 0x53, 0x05, 0x63, 0x21, 0x62, 0x41,
    0x96, 0x22, 0x9a, 0x41, 0xcc, 0x03, 0xb9, 0x91, 0x39, 0xf1,
    0x63, 0x26, 0x67, 0x27, 0xd3, 0x06, 0xfc, 0x01, 0x18, 0xe2,
    0xd9, 0x07, 0xe9, 0x04, 0x0c, 0x86, 0x37, 0x22, 0x93, 0x24,
    0x87, 0x84, 0xac, 0x02, 0xc2, 0x41, 0xc3, 0x23, 0xd9, 0x71,
    0xfc, 0x01, 0x7f, 0xb1, 0x9c, 0x00, 0xa7, 0x63, 0xb6, 0x64,
    0xcc, 0x00, 0xd4, 0x66, 0xe3, 0x67, 0xf3, 0x67, 0x8d, 0xc1,
    0xcf, 0x26,
    0xfd,

    //level 6-1,
    0x52, 0x21,
    0x0f, 0x20, 0x6e, 0x44, 0x0c, 0xf1, 0x4c, 0x01, 0xaa, 0x35,
    0xd9, 0x34, 0xee, 0x20, 0x08, 0xb3, 0x37, 0x32, 0x43, 0x04,
    0x4e, 0x21, 0x53, 0x20, 0x7c, 0x01, 0x97, 0x21, 0xb7, 0x07,
    0x9c, 0x81, 0xe7, 0x42, 0x5f, 0xb3, 0x97, 0x63, 0xac, 0x02,
    0xc5, 0x41, 0x49, 0xe0, 0x58, 0x61, 0x76, 0x64, 0x85, 0x65,
    0x94, 0x66, 0xa4, 0x22, 0xa6, 0x03, 0xc8, 0x22, 0xdc, 0x02,
    0x68, 0xf2, 0x96, 0x42, 0x13, 0x82, 0x17, 0x02, 0xaf, 0x34,
    0xf6, 0x21, 0xfc, 0x06, 0x26, 0x80, 0x2a, 0x24, 0x36, 0x01,
    0x8c, 0x00, 0xff, 0x35, 0x4e, 0xa0, 0x55, 0x21, 0x77, 0x20,
    0x87, 0x07, 0x89, 0x22, 0xae, 0x21, 0x4c, 0x82, 0x9f, 0x34,
    0xec, 0x01, 0x03, 0xe7, 0x13, 0x67, 0x8d, 0x4a, 0xad, 0x41,
    0x0f, 0xa6,
    0xfd,

    //level 6-2,
    0x54, 0x21,
    0x0f, 0x26, 0xa7, 0x22, 0x37, 0xfb, 0x73, 0x20, 0x83, 0x07,
    0x87, 0x02, 0x93, 0x20, 0xc7, 0x73, 0x04, 0xf1, 0x06, 0x31,
    0x39, 0x71, 0x59, 0x71, 0xe7, 0x73, 0x37, 0xa0, 0x47, 0x04,
    0x86, 0x7c, 0xe5, 0x71, 0xe7, 0x31, 0x33, 0xa4, 0x39, 0x71,
    0xa9, 0x71, 0xd3, 0x23, 0x08, 0xf2, 0x13, 0x05, 0x27, 0x02,
    0x49, 0x71, 0x75, 0x75, 0xe8, 0x72, 0x67, 0xf3, 0x99, 0x71,
    0xe7, 0x20, 0xf4, 0x72, 0xf7, 0x31, 0x17, 0xa0, 0x33, 0x20,
    0x39, 0x71, 0x73, 0x28, 0xbc, 0x05, 0x39, 0xf1, 0x79, 0x71,
    0xa6, 0x21, 0xc3, 0x06, 0xd3, 0x20, 0xdc, 0x00, 0xfc, 0x00,
    0x07, 0xa2, 0x13, 0x21, 0x5f, 0x32, 0x8c, 0x00, 0x98, 0x7a,
    0xc7, 0x63, 0xd9, 0x61, 0x03, 0xa2, 0x07, 0x22, 0x74, 0x72,
    0x77, 0x31, 0xe7, 0x73, 0x39, 0xf1, 0x58, 0x72, 0x77, 0x73,
    0xd8, 0x72, 0x7f, 0xb1, 0x97, 0x73, 0xb6, 0x64, 0xc5, 0x65,
    0xd4, 0x66, 0xe3, 0x67, 0xf3, 0x67, 0x8d, 0xc1, 0xcf, 0x26,
    0xfd,

    //level 6-3,
    0x97, 0x11,
    0x0f, 0x26, 0xfe, 0x10, 0x2b, 0x92, 0x57, 0x12, 0x8b, 0x12,
    0xc0, 0x41, 0xf7, 0x13, 0x5b, 0x92, 0x69, 0x0b, 0xbb, 0x12,
    0xb2, 0x46, 0x19, 0x93, 0x71, 0x00, 0x17, 0x94, 0x7c, 0x14,
    0x7f, 0x11, 0x93, 0x41, 0xbf, 0x15, 0xfc, 0x13, 0xff, 0x11,
    0x2f, 0x95, 0x50, 0x42, 0x51, 0x12, 0x58, 0x14, 0xa6, 0x12,
    0xdb, 0x12, 0x1b, 0x93, 0x46, 0x43, 0x7b, 0x12, 0x8d, 0x49,
    0xb7, 0x14, 0x1b, 0x94, 0x49, 0x0b, 0xbb, 0x12, 0xfc, 0x13,
    0xff, 0x12, 0x03, 0xc1, 0x2f, 0x15, 0x43, 0x12, 0x4b, 0x13,
    0x77, 0x13, 0x9d, 0x4a, 0x15, 0xc1, 0xa1, 0x41, 0xc3, 0x12,
    0xfe, 0x01, 0x7d, 0xc1, 0x9e, 0x42, 0xcf, 0x20,
    0xfd,

    //level 7-1,
    0x52, 0xb1,
    0x0f, 0x20, 0x6e, 0x45, 0x39, 0x91, 0xb3, 0x04, 0xc3, 0x21,
    0xc8, 0x11, 0xca, 0x10, 0x49, 0x91, 0x7c, 0x73, 0xe8, 0x12,
    0x88, 0x91, 0x8a, 0x10, 0xe7, 0x21, 0x05, 0x91, 0x07, 0x30,
    0x17, 0x07, 0x27, 0x20, 0x49, 0x11, 0x9c, 0x01, 0xc8, 0x72,
    0x23, 0xa6, 0x27, 0x26, 0xd3, 0x03, 0xd8, 0x7a, 0x89, 0x91,
    0xd8, 0x72, 0x39, 0xf1, 0xa9, 0x11, 0x09, 0xf1, 0x63, 0x24,
    0x67, 0x24, 0xd8, 0x62, 0x28, 0x91, 0x2a, 0x10, 0x56, 0x21,
    0x70, 0x04, 0x79, 0x0b, 0x8c, 0x00, 0x94, 0x21, 0x9f, 0x35,
    0x2f, 0xb8, 0x3d, 0xc1, 0x7f, 0x26,
    0xfd,

    //level 8-1,
    0x92, 0x31,
    0x0f, 0x20, 0x6e, 0x40, 0x0d, 0x02, 0x37, 0x73, 0xec, 0x00,
    0x0c, 0x80, 0x3c, 0x00, 0x6c, 0x00, 0x9c, 0x00, 0x06, 0xc0,
    0xc7, 0x73, 0x06, 0x83, 0x28, 0x72, 0x96, 0x40, 0xe7, 0x73,
    0x26, 0xc0, 0x87, 0x7b, 0xd2, 0x41, 0x39, 0xf1, 0xc8, 0xf2,
    0x97, 0xe3, 0xa3, 0x23, 0xe7, 0x02, 0xe3, 0x07, 0xf3, 0x22,
    0x37, 0xe3, 0x9c, 0x00, 0xbc, 0x00, 0xec, 0x00, 0x0c, 0x80,
    0x3c, 0x00, 0x86, 0x21, 0xa6, 0x06, 0xb6, 0x24, 0x5c, 0x80,
    0x7c, 0x00, 0x9c, 0x00, 0x29, 0xe1, 0xdc, 0x05, 0xf6, 0x41,
    0xdc, 0x80, 0xe8, 0x72, 0x0c, 0x81, 0x27, 0x73, 0x4c, 0x01,
    0x66, 0x74, 0x0d, 0x11, 0x3f, 0x35, 0xb6, 0x41, 0x2c, 0x82,
    0x36, 0x40, 0x7c, 0x02, 0x86, 0x40, 0xf9, 0x61, 0x39, 0xe1,
    0xac, 0x04, 0xc6, 0x41, 0x0c, 0x83, 0x16, 0x41, 0x88, 0xf2,
    0x39, 0xf1, 0x7c, 0x00, 0x89, 0x61, 0x9c, 0x00, 0xa7, 0x63,
    0xbc, 0x00, 0xc5, 0x65, 0xdc, 0x00, 0xe3, 0x67, 0xf3, 0x67,
    0x8d, 0xc1, 0xcf, 0x26,
    0xfd,

    //level 8-2,
    0x50, 0xb1,
    0x0f, 0x26, 0xfc, 0x00, 0x1f, 0xb3, 0x5c, 0x00, 0x65, 0x65,
    0x74, 0x66, 0x83, 0x67, 0x93, 0x67, 0xdc, 0x73, 0x4c, 0x80,
    0xb3, 0x20, 0xc9, 0x0b, 0xc3, 0x08, 0xd3, 0x2f, 0xdc, 0x00,
    0x2c, 0x80, 0x4c, 0x00, 0x8c, 0x00, 0xd3, 0x2e, 0xed, 0x4a,
    0xfc, 0x00, 0xd7, 0xa1, 0xec, 0x01, 0x4c, 0x80, 0x59, 0x11,
    0xd8, 0x11, 0xda, 0x10, 0x37, 0xa0, 0x47, 0x04, 0x99, 0x11,
    0xe7, 0x21, 0x3a, 0x90, 0x67, 0x20, 0x76, 0x10, 0x77, 0x60,
    0x87, 0x07, 0xd8, 0x12, 0x39, 0xf1, 0xac, 0x00, 0xe9, 0x71,
    0x0c, 0x80, 0x2c, 0x00, 0x4c, 0x05, 0xc7, 0x7b, 0x39, 0xf1,
    0xec, 0x00, 0xf9, 0x11, 0x0c, 0x82, 0x6f, 0x34, 0xf8, 0x11,
    0xfa, 0x10, 0x7f, 0xb2, 0xac, 0x00, 0xb6, 0x64, 0xcc, 0x01,
    0xe3, 0x67, 0xf3, 0x67, 0x8d, 0xc1, 0xcf, 0x26,
    0xfd,

    //level 8-3,
    0x90, 0xb1,
    0x0f, 0x26, 0x29, 0x91, 0x7e, 0x42, 0xfe, 0x40, 0x28, 0x92,
    0x4e, 0x42, 0x2e, 0xc0, 0x57, 0x73, 0xc3, 0x25, 0xc7, 0x27,
    0x23, 0x84, 0x33, 0x20, 0x5c, 0x01, 0x77, 0x63, 0x88, 0x62,
    0x99, 0x61, 0xaa, 0x60, 0xbc, 0x01, 0xee, 0x42, 0x4e, 0xc0,
    0x69, 0x11, 0x7e, 0x42, 0xde, 0x40, 0xf8, 0x62, 0x0e, 0xc2,
    0xae, 0x40, 0xd7, 0x63, 0xe7, 0x63, 0x33, 0xa7, 0x37, 0x27,
    0x43, 0x04, 0xcc, 0x01, 0xe7, 0x73, 0x0c, 0x81, 0x3e, 0x42,
    0x0d, 0x0a, 0x5e, 0x40, 0x88, 0x72, 0xbe, 0x42, 0xe7, 0x87,
    0xfe, 0x40, 0x39, 0xe1, 0x4e, 0x00, 0x69, 0x60, 0x87, 0x60,
    0xa5, 0x60, 0xc3, 0x31, 0xfe, 0x31, 0x6d, 0xc1, 0xbe, 0x42,
    0xef, 0x20,
    0xfd,
];

const ENEMIES: [u8; 620] = [
    //level 1-1,
    0x1e, 0xc2, 0x00, 0x6b, 0x06, 0x8b, 0x86, 0x63, 0xb7, 0x0f, 0x05,
    0x03, 0x06, 0x23, 0x06, 0x4b, 0xb7, 0xbb, 0x00, 0x5b, 0xb7,
    0xfb, 0x37, 0x3b, 0xb7, 0x0f, 0x0b, 0x1b, 0x37,
    0xff,

    //level 1-3/5-3,
    0x2b, 0xd7, 0xe3, 0x03, 0xc2, 0x86, 0xe2, 0x06, 0x76, 0xa5,
    0xa3, 0x8f, 0x03, 0x86, 0x2b, 0x57, 0x68, 0x28, 0xe9, 0x28,
    0xe5, 0x83, 0x24, 0x8f, 0x36, 0xa8, 0x5b, 0x03,
    0xff,

    //level 2-1,
    0x85, 0x86, 0x0b, 0x80, 0x1b, 0x00, 0xdb, 0x37, 0x77, 0x80,
    0xeb, 0x37, 0xfe, 0x2b, 0x20, 0x2b, 0x80, 0x7b, 0x38, 0xab, 0xb8,
    0x77, 0x86, 0xfe, 0x42, 0x20, 0x49, 0x86, 0x8b, 0x06, 0x9b, 0x80,
    0x7b, 0x8e, 0x5b, 0xb7, 0x9b, 0x0e, 0xbb, 0x0e, 0x9b, 0x80,
    0xff,

    //level 2-3/7-3,
    0x0f, 0x02, 0x78, 0x40, 0x48, 0xce, 0xf8, 0xc3, 0xf8, 0xc3,
    0x0f, 0x07, 0x7b, 0x43, 0xc6, 0xd0, 0x0f, 0x8a, 0xc8, 0x50,
    0xff,

    //level 3-1,
    0x9b, 0x8e, 0xca, 0x0e, 0xee, 0x42, 0x44, 0x5b, 0x86, 0x80, 0xb8,
    0x1b, 0x80, 0x50, 0xba, 0x10, 0xb7, 0x5b, 0x00, 0x17, 0x85,
    0x4b, 0x05, 0xfe, 0x34, 0x40, 0xb7, 0x86, 0xc6, 0x06, 0x5b, 0x80,
    0x83, 0x00, 0xd0, 0x38, 0x5b, 0x8e, 0x8a, 0x0e, 0xa6, 0x00,
    0xbb, 0x0e, 0xc5, 0x80, 0xf3, 0x00,
    0xff,

    //level 3-2,
    0x1b, 0x80, 0xbb, 0x38, 0x4b, 0xbc, 0xeb, 0x3b, 0x0f, 0x04,
    0x2b, 0x00, 0xab, 0x38, 0xeb, 0x00, 0xcb, 0x8e, 0xfb, 0x80,
    0xab, 0xb8, 0x6b, 0x80, 0xfb, 0x3c, 0x9b, 0xbb, 0x5b, 0xbc,
    0xfb, 0x00, 0x6b, 0xb8, 0xfb, 0x38,
    0xff,

    //level 3-3,
    0xa5, 0x86, 0xe4, 0x28, 0x18, 0xa8, 0x45, 0x83, 0x69, 0x03,
    0xc6, 0x29, 0x9b, 0x83, 0x16, 0xa4, 0x88, 0x24, 0xe9, 0x28,
    0x05, 0xa8, 0x7b, 0x28, 0x24, 0x8f, 0xc8, 0x03, 0xe8, 0x03,
    0x46, 0xa8, 0x85, 0x24, 0xc8, 0x24,
    0xff,

    //level 4-1,
    0x2e, 0xc2, 0x66, 0xe2, 0x11, 0x0f, 0x07, 0x02, 0x11, 0x0f, 0x0c,
    0x12, 0x11,
    0xff,

    //level 4-3,
    0xc7, 0x83, 0xd7, 0x03, 0x42, 0x8f, 0x7a, 0x03, 0x05, 0xa4,
    0x78, 0x24, 0xa6, 0x25, 0xe4, 0x25, 0x4b, 0x83, 0xe3, 0x03,
    0x05, 0xa4, 0x89, 0x24, 0xb5, 0x24, 0x09, 0xa4, 0x65, 0x24,
    0xc9, 0x24, 0x0f, 0x08, 0x85, 0x25,
    0xff,

    //level 5-1,
    0x0b, 0x80, 0x60, 0x38, 0x10, 0xb8, 0xc0, 0x3b, 0xdb, 0x8e,
    0x40, 0xb8, 0xf0, 0x38, 0x7b, 0x8e, 0xa0, 0xb8, 0xc0, 0xb8,
    0xfb, 0x00, 0xa0, 0xb8, 0x30, 0xbb, 0xee, 0x42, 0x88, 0x0f, 0x0b,
    0x2b, 0x0e, 0x67, 0x0e,
    0xff,

    //level 5-2,
    0x7b, 0x80, 0xae, 0x00, 0x80, 0x8b, 0x8e, 0xe8, 0x05, 0xf9, 0x86 ,
    0x17, 0x86, 0x16, 0x85, 0x4e, 0x2b, 0x80, 0xab, 0x8e, 0x87, 0x85,
    0xc3, 0x05, 0x8b, 0x82, 0x9b, 0x02, 0xab, 0x02, 0xbb, 0x86,
    0xcb, 0x06, 0xd3, 0x03, 0x3b, 0x8e, 0x6b, 0x0e, 0xa7, 0x8e,
    0xff,

    //level 6-1,
    0x0f, 0x02, 0x02, 0x11, 0x0f, 0x07, 0x02, 0x11,
    0xff,

    //level 6-2,
    0x0e, 0xc2, 0xa8, 0xab, 0x00, 0xbb, 0x8e, 0x6b, 0x82, 0xde, 0x00, 0xa0,
    0x33, 0x86, 0x43, 0x06, 0x3e, 0xb4, 0xa0, 0xcb, 0x02, 0x0f, 0x07,
    0x7e, 0x42, 0xa6, 0x83, 0x02, 0x0f, 0x0a, 0x3b, 0x02, 0xcb, 0x37,
    0x0f, 0x0c, 0xe3, 0x0e,
    0xff,

    //level 6-3,
    0xcd, 0xa5, 0xb5, 0xa8, 0x07, 0xa8, 0x76, 0x28, 0xcc, 0x25,
    0x65, 0xa4, 0xa9, 0x24, 0xe5, 0x24, 0x19, 0xa4, 0x0f, 0x07,
    0x95, 0x28, 0xe6, 0x24, 0x19, 0xa4, 0xd7, 0x29, 0x16, 0xa9,
    0x58, 0x29, 0x97, 0x29,
    0xff,

    //level 7-1,
    0xab, 0xce, 0xde, 0x42, 0xc0, 0xcb, 0xce, 0x5b, 0x8e, 0x1b, 0xce,
    0x4b, 0x85, 0x67, 0x45, 0x0f, 0x07, 0x2b, 0x00, 0x7b, 0x85,
    0x97, 0x05, 0x0f, 0x0a, 0x92, 0x02,
    0xff,

    //level 8-1,
    0x2b, 0x82, 0xab, 0x38, 0xde, 0x42, 0xe2, 0x1b, 0xb8, 0xeb,
    0x3b, 0xdb, 0x80, 0x8b, 0xb8, 0x1b, 0x82, 0xfb, 0xb8, 0x7b,
    0x80, 0xfb, 0x3c, 0x5b, 0xbc, 0x7b, 0xb8, 0x1b, 0x8e, 0xcb,
    0x0e, 0x1b, 0x8e, 0x0f, 0x0d, 0x2b, 0x3b, 0xbb, 0xb8, 0xeb, 0x82,
    0x4b, 0xb8, 0xbb, 0x38, 0x3b, 0xb7, 0xbb, 0x02, 0x0f, 0x13,
    0x1b, 0x00, 0xcb, 0x80, 0x6b, 0xbc,
    0xff,

    //level 8-2,
    0x29, 0x8e, 0x52, 0x11, 0x83, 0x0e, 0x0f, 0x03, 0x9b, 0x0e,
    0x2b, 0x8e, 0x5b, 0x0e, 0xcb, 0x8e, 0xfb, 0x0e, 0xfb, 0x82,
    0x9b, 0x82, 0xbb, 0x02, 0xfe, 0x42, 0xe8, 0xbb, 0x8e, 0x0f, 0x0a,
    0xab, 0x0e, 0xcb, 0x0e, 0xf9, 0x0e, 0x88, 0x86, 0xa6, 0x06,
    0xdb, 0x02, 0xb6, 0x8e,
    0xff,

    //level 8-3,
    0xeb, 0x8e, 0x0f, 0x03, 0xfb, 0x05, 0x17, 0x85, 0xdb, 0x8e,
    0x0f, 0x07, 0x57, 0x05, 0x7b, 0x05, 0x9b, 0x80, 0x2b, 0x85,
    0xfb, 0x05, 0x0f, 0x0b, 0x1b, 0x05, 0x9b, 0x05,
    0xff,
];

fn put(level: &mut Vec<Vec<u8>>, x: usize, p_x: usize, y: u8, c: u8) {
    let x = x as usize + p_x*16;
    let y = if y > LEVEL_HEIGHT { 0 } else { y };

    while level.len() <= x {
        let mut v = vec![b' '; LEVEL_HEIGHT as usize + 1];
        if level.len() > 0 {
            v[LEVEL_HEIGHT as usize] = *level.last().unwrap().get(LEVEL_HEIGHT as usize).unwrap();
        }
        level.push(v);
    }

    *level.get_mut(x).unwrap().get_mut(y as usize).unwrap() = c;
}

fn put_level_data(level_objects: &[u8], level: &mut Vec<Vec<u8>>) {
    let mut i = 2;
    let mut p_x = 0;
    let mut bt = level_objects[1]&0x0F;

    let c = format!("{:X}", bt&0xF).chars().next().unwrap();
    put(level, 0, p_x, LEVEL_HEIGHT, c as u8);

    while level_objects[i] != 0xFD {
        let b = level_objects[i];
        let x = (b&0b11110000)>>4;
        let y = b&0b00001111;

        let b = level_objects[i+1];
        let p = (b&0b10000000)>0;
        let n = b&0b01111111;

        i += 2;

        if p {
            p_x += 1;
        }

        let c = format!("{:X}", bt&0xF).chars().next().unwrap();
        put(level, x as usize, p_x, LEVEL_HEIGHT, c as u8);

        if y == 14 && n < 0x3F {
            bt = n;
            let c = format!("{:X}", bt&0xF).chars().next().unwrap();
            put(level, x as usize + 1, p_x, LEVEL_HEIGHT, c as u8);
        } else if y < 12 && n >= 0x20 && n <= 0x2F {
            for i in 0...(n-0x20) {
                put(level, x as usize + i as usize, p_x, y, b'b');
            }
        } else if y < 12 && n >= 0x10 && n <= 0x1F {
            for i in 0...(n-0x10) {
                put(level, x as usize + i as usize, p_x, y, b'I');
            }
        } else if y < 12 && n >= 0x50 && n <= 0x5F {
            for i in 0...(n-0x50) {
                put(level, x as usize, p_x, y+i, b'b');
            }
        } else if y < 12 && n >= 0x30 && n <= 0x3F {
            for i in 0...(n-0x30) {
                put(level, x as usize + i as usize, p_x, y, b'.');
            }
        } else if y < 12 && n >= 0x70 && n <= 0x77 {
            for i in 0...(n-0x70) {
                put(level, x as usize, p_x, y+i, b'p');
            }
        } else if y < 12 && n >= 0x78 && n <= 0x7F {
            for i in 0...(n-0x78) {
                put(level, x as usize, p_x, y+i, b'p');
            }
        } else if y < 12 && n >= 0x60 && n <= 0x6F {
            for i in 0...(n-0x60) {
                put(level, x as usize, p_x, y+i, b'.');
            }
        } else if y == 12 && n >= 0x60 && n <= 0x6F {
            for i in 0...(n-0x60) {
                put(level, x as usize + i as usize, p_x, 3, b'?');
            }
        } else if y == 12 && n >= 0x70 && n <= 0x7F {
            for i in 0...(n-0x70) {
                put(level, x as usize + i as usize, p_x, 7, b'?');
            }
        } else if y == 13 && n == 0x41 {
            put(level, x as usize, p_x, LEVEL_HEIGHT-1, b'F');
        } else if y == 15 && n >= 0x30 && n <= 0x3F {
            for i in 0...(n-0x30) {
                for j in 0...i {
                    put(level, x as usize + i as usize, p_x, 10-j, b'.');
                }
            }
        } else if y == 15 && n >= 0x40 && n <= 0x4F {
            put(level, x as usize as usize, p_x, (n-0x40), b'U');
        } else if y == 15 && n >= 0x20 && n <= 0x2A {
            put(level, x as usize as usize, p_x, LEVEL_HEIGHT-1, b'^');
        } else if y == 12 && n <= 0x0F {
            put(level, x as usize, p_x, 12 - n, b'h');
        } else if y < 12 && n >= 0x40 && n <= 0x4F {
            for i in 0...(n-0x40) {
                put(level, x as usize + i as usize, p_x, y, b'0');
            }
        } else if y < 12 && n == 0 {
            put(level, x as usize, p_x, y, b'!')
        } else if y < 12 && n == 1 {
            put(level, x as usize, p_x, y, b'?')
        } else if y < 12 && n == 4 {
            put(level, x as usize, p_x, y, b'M')
        } else if y < 12 && n == 6 {
            put(level, x as usize, p_x, y, b'S')
        } else if y < 12 && n == 7 {
            put(level, x as usize, p_x, y, b'C')
        } else if y < 12 && n == 8 {
            put(level, x as usize, p_x, y, b'u')
        } else if y < 12 && n == 0x0f {
            put(level, x as usize, p_x, y, b'n')
        } else {
            println!("Unrecognized level tile: {}, {}, {}, {:X}", x, y, p, n);
            let y = if y >= 12 { 0 } else { y };
            put(level, x as usize, p_x, y, b' ');
        }
    }
}

fn put_enemy_data(level_objects: &[u8], level: &mut Vec<Vec<u8>>) {
    let mut i = 0;
    let mut p_x = 0;

    while level_objects[i] != 0xFF {
        let b = level_objects[i];
        let x = (b&0b11110000)>>4;
        let y = b&0b00001111;

        let b = level_objects[i+1];
        let p = (b&0b10000000)>0;
        let n = b&0b00111111;

        i += 2;

        if p {
            p_x += 1;
        }

        if y == 14 {
            i += 1;
            continue;
        } else if y == 15 {
            p_x = n as usize;
            continue;
        }

        let y = if y == 0 { 0 } else { y - 1 };

        if n == 0x06 {
            put(level, x as usize, p_x, y, b'g');
        } else if n >= 0x37 && n <= 0x38 {
            for i in 0...(n-0x37+1) {
                put(level, x as usize + i as usize, p_x, 10-1, b'g');
            }
        } else if n >= 0x39 && n <= 0x3A {
            for i in 0...(n-0x39+1) {
                put(level, x as usize + i as usize, p_x, 7-1, b'g');
            }
        } else if n == 0x00 {
            put(level, x as usize, p_x, y, b'k');
        } else {
            println!("Unrecognized Enemy: {}, {}, {}, {:X}", x, y+1, p, n);
        }
    }
}

fn output_level(index: usize, out: &mut Vec<u8>) {
    let mut level: Vec<Vec<u8>> = vec![];

    {
        let mut bt_idx = 0;
        for _ in 0..index {
            while LEVELS[bt_idx] != 0xFD { bt_idx += 1; }
            bt_idx += 1;
        }
        let level_objects = &LEVELS[bt_idx..];

        out.push(format!("{:X}", 1).chars().next().unwrap() as u8);
        out.push(format!("{:X}", (level_objects[1]&0b11000000)>>6).chars().next().unwrap() as u8);
        out.push(format!("{:X}", (level_objects[1]&0b00110000)>>4).chars().next().unwrap() as u8);
        out.push(b'\n');

        put_level_data(&level_objects, &mut level);
    }

    {
        let mut bt_idx = 0;
        for _ in 0..index {
            while ENEMIES[bt_idx] != 0xFF { bt_idx += 1; }
            bt_idx += 1;
        }
        let level_objects = &ENEMIES[bt_idx..];

        put_enemy_data(&level_objects, &mut level);
    }

    for x in 0..level.len() {
        for y in (0...LEVEL_HEIGHT).rev() {
            out.push(*level.get(x).unwrap().get(y as usize).unwrap());
        }
        out.push(b'\n');
    }
}

fn main() {
    let mut out = vec![];

    for i in 0..1 {//18 {
        output_level(i, &mut out);
    }

    write_bytes_to_file(format!("assets/0.level"), out.as_slice());
}