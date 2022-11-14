// rpi3b+
// #define GPIO_BASE (0x3F000000 + 0x200000)

// rpi4 
#define GPIO_BASE (0xFE000000 + 0x200000)
#define LONG_PAUSE 800
#define SHORT_PAUSE 100
#define MEDIUM_PAUSE 400

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_SET0  = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0  = (volatile unsigned *)(GPIO_BASE + 0x28);

static void spin_sleep_us(unsigned int us) {
  for (unsigned int i = 0; i < us * 6; i++) {
    asm volatile("nop");
  }
}

static void spin_sleep_ms(unsigned int ms) {
  spin_sleep_us(ms * 1000);
}

static void dot() {
  *GPIO_SET0 = 0b1 << 16;
  spin_sleep_ms(SHORT_PAUSE);
  *GPIO_CLR0 = 0b1 << 16; 
}

static void dash() {
  *GPIO_SET0 = 0b1 << 16;
  spin_sleep_ms(MEDIUM_PAUSE);
  *GPIO_CLR0 = 0b1 << 16; 
}

static void space() {
  spin_sleep_ms(LONG_PAUSE);
}

void main() {
  *GPIO_FSEL1 = 0b1 << 18;
  // .--- .- -.-. -.- .. . = jackie
  dot();
  dash();
  dash();
  dash();

  space();
  
  dot();
  dash();
  
  space();

  dash();
  dot();
  dash();
  dot();

  space();

  dash();
  dot();
  dash();

  space();

  dot();
  dot();

  space();

  dot();
}
