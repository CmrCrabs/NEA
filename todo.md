- offset top right
- move whole thing such that centred again

- [ ] move files for engine / sim

- [ ] vectors confirm
- [ ] full pbr
- [ ] sun pos
- [ ] still fix reflections
- [ ] tiling
- [ ] lengthscales
- [ ] frametime graph
- [ ] showcase textures in ui?
- [ ] fix pbr speckles


- [ ] windows compile link1889
- [ ] compile binaries
- [ ] follow https://bevyengine.org/learn/quick-start/getting-started/setup/#compile-with-performance-optimizations for opts
- [ ] upgrade to 16k textures for hdri
- [ ] prove everything works
- [ ] better camera!!!
- [ ] remake camera struct to not be bad
- [ ] update fragment tex map reads to use sampler
    - [ ] bind same texture multiple ways

- [ ] explain padding and alignment
- [ ] factor lengthscale render calls into seperate function?
- [ ] gaussian prng?
- [ ] debug all shader warnings
- [ ] cargo clippy?

- [ ] move renderer to /gfx/ (?)
- [ ] optimise ui, include texture 
- [ ] fix hidpi
- [ ] go through and add labels to everything
- [ ] write up summation, indices etc
- [ ] platform check so it wokrs on windows
- [ ] credit 
    - [ ] imgui-wgpu-rs
    - [ ] imgui-winit-support
    - [ ] all the to x key
    - [ ] prng -> update to no prng just rng -> add prng?
- [ ] credit gasgiant fftocean if not alreaey
- [ ] mention nyquist theorem for bounds
- [ ] add section and explain complex mult, ih
- [ ] credit biebras  using actual name
- [ ] credit reinhard tonemapping
- [ ] talk about equirectangular sampling for an "alg"
    - [ ] credit equirect sample and skybox sample
- [ ] credit learn wgpu


### future improvements
- handle state better
- use push constants
- better optimise
- shore interactions
- bloom pass
- fix skybox offset
- proper tonemapping based on exposure
- more advanced foam
- sea spray
- level of detail
