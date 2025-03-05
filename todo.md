- [ ] CUBEMAP LOAD
- [ ] EQUIRECTANGULAR SAMPLE
- [ ] REFLECTIONS
- [ ] CAMERA
- [ ] VECTORS
- [ ] PBR SPECULAR
- [ ] TILING?
- [ ] LENGTHSCALES?

- [ ] move computepass to gfx

- [ ] cubemap load pass

- [ ] bevy runtime for faster texture loading

- [ ] update permute scale to permute
- [ ] eventualyl convolve foam with a texture

- [ ] focus foam first
- [ ] vectors
- [ ] sampler
- [ ] foam lerp
- [ ] full pbr
- [ ] sun pos

- [ ] windows compile link1889
- [ ] compile binaries
- [ ] follow https://bevyengine.org/learn/quick-start/getting-started/setup/#compile-with-performance-optimizations for opts

- [ ] prove everything works

- [ ] better camera!!!
- [ ] remake camera struct to not be bad

- [ ] frametime graph

- [ ] move bind group descs into impl

- [ ] update fragment tex map reads to use sampler
    - [ ] bind same texture multiple ways

- [ ] sampled layout stg textures bulk
- [ ] sampled layout fourier textures for ui

- [ ] create impl RenderPass and use that instead of StandardPipeline

- [ ] explain padding and alignment
- [ ] foam accumulation change to exponential decay

- [ ] factor lengthscale render calls into seperate function?
- [ ] update standardpass to handle render calls locally
- [ ] swell
- [ ] lengthscales
- [ ] lighting
- [ ] skybox
- [ ] tiling
- [ ] post processing
    - [X] add update constants fn, multiply light vector by matrix
- [ ] gaussian prng?
- [ ] move pingpong to seperate buf
- [ ] debug all shader warnings
- [ ] cargo clippy?

- [ ] move renderer to /gfx/ (?)
- [ ] optimise ui
- [ ] move all textures to one bind group per cascade, remove limit
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
- [ ] talk about equirectangular sampling for an "alg"


### future improvements
- handle state better
- use push constants
- better optimise
- shore interactions
- more advanced foam
- sea spray
