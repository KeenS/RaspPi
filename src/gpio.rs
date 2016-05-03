// Raspberry Pi2
//pub const GPIO_BASE: u32 = 0x3F200000;
// other
pub const GPIO_BASE: u32 = 0x20200000;



// Raspberrp Pi+ or Raspberry Pi2
//pub const LED_GPFSEL: isize   = GPIO_GPFSEL4;
//pub const LED_GPFBIT: i32     = 21;
//pub const LED_GPSET: isize    = GPIO_GPSET1;
//pub const LED_GPCLR: isize    = GPIO_GPCLR1;
//pub const LED_GPIO_BIT: isize = 15;
//  other
pub const LED_GPFSEL: isize = GPIO_GPFSEL1;
pub const LED_GPFBIT: i32 = 18;
pub const LED_GPCLR: isize = GPIO_GPCLR0;
pub const LED_GPSET: isize = GPIO_GPSET0;
pub const LED_GPIO_BIT: i32 = 16;

pub const GPIO_GPFSEL0: isize   = 0;
pub const GPIO_GPFSEL1: isize   = 1;
pub const GPIO_GPFSEL2: isize   = 2;
pub const GPIO_GPFSEL3: isize   = 3;
pub const GPIO_GPFSEL4: isize   = 4;
pub const GPIO_GPFSEL5: isize   = 5;
pub const GPIO_GPSET0: isize    = 7;
pub const GPIO_GPSET1: isize    = 8;
pub const GPIO_GPCLR0: isize    = 10;
pub const GPIO_GPCLR1: isize    = 11;
pub const GPIO_GPLEV0: isize    = 13;
pub const GPIO_GPLEV1: isize    = 14;
pub const GPIO_GPEDS0: isize    = 16;
pub const GPIO_GPEDS1: isize    = 17;
pub const GPIO_GPREN0: isize    = 19;
pub const GPIO_GPREN1: isize    = 20;
pub const GPIO_GPFEN0: isize    = 22;
pub const GPIO_GPFEN1: isize    = 23;
pub const GPIO_GPHEN0: isize    = 25;
pub const GPIO_GPHEN1: isize    = 26;
pub const GPIO_GPLEN0: isize    = 28;
pub const GPIO_GPLEN1: isize    = 29;
pub const GPIO_GPAREN0: isize   = 31;
pub const GPIO_GPAREN1: isize   = 32;
pub const GPIO_GPAFEN0: isize   = 34;
pub const GPIO_GPAFEN1: isize   = 35;
pub const GPIO_GPPUD: isize     = 37;
pub const GPIO_GPPUDCLK0: isize = 38;
pub const GPIO_GPPUDCLK1: isize = 39;

