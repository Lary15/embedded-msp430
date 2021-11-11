MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0400 /* END=0x05FF, size 1024 */
  ROM (rx)         : ORIGIN = 0x8000, LENGTH = 0x7FE0 /* END=0xFFDF, size 32736 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
