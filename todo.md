- [ ] cargo clippy

- [ ] windows compile link1889
- [ ] compile binaries
- [ ] follow https://bevyengine.org/learn/quick-start/getting-started/setup/#compile-with-performance-optimizations for opts
- [ ] prove everything works

- [ ] explain padding and alignment
- [ ] factor lengthscale render calls into seperate function?
- [ ] cargo clippy?

- [ ] fix hidpi
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
- [ ] talk about semantics of holding computepass in sim not engine
- [ ] just heading of file structure semantics
- [ ] explain why engine shaders are file root, how main_vs is arguably both sim and engine


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
- improve camera
- add lengthscales
- add live texture viewers to imgui, explain why i didnt
