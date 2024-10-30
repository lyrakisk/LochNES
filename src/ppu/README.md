# Definitive guide to NES PPU (top-down approach)

Goal: render the game at 256x240 pixels

TODO: Discuss how PPU sends signal to tv at 60FPS for NTSC or PAL. Explain the difference between the two systems.

The PPU renders 262 scanlines per frame. Each scanline lasts for 341 PPU clock cycles, with each clock cycle producing one pixel.

PPU has some shift registeres. What's their use? 

If each cycle produces one pixel, then we only need 256 cycles to produces the 256 pixes of one line. What are we doing with the rest of the cycles? 

When does the actual rendering take place? 

What is a scanline? Why 262, instead of 240?
- First we have the pre-render scanline (call it 261 or -1)
- Then we have the scanlines that actually put an image on the screen, so we call these "visible scanlines"
- Then we have the post-render scanline (240). Here the PPU does nothing.
- Then we have the v-blank scanlines (241-260). Why do we have these? Because we need a safe time frame for the CPU to write into PPU's memory. 

TODO: Define what should happen during a tick

ppu api:
- tick()
- render_pixel()
- Frame
- 
ppu state machine:
 - IDLE
 - V_BLANK
 - RENDERING_SCANLINE