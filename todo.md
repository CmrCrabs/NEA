- [ ] fft data 6 1 instead of correct
- [ ] pingpong textures all 0

- [ ] debug fft
    - [ ] pingpong data lost
- [ ] setup dwhatever maps in cascace
- [ ] encode data into them
- [ ] unpack fft data
    - [ ] normal
    - [ ] displacement
    - [ ] foam
- [ ] optimise maps


compute pass -> ifft -> normal map -> lighting -> foam -> lengthscales -> misc

- [ ] factor lengthscale render calls into seperate function?
- [ ] update standardpass to handle render calls locally
- [ ] compute pass impl
- [ ] 5th compute pass, prep fft
- [ ] fft
    - [ ] impl
    - [ ] cpu alg
    - [ ] 1d ifft gpu
- [ ] swell
- [ ] lengthscales
- [ ] derivatvies
- [ ] normal map
- [ ] foam
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


### future improvements
- handle state better
- use push constants
- better optimise
- shore interactions
- more advanced foam
- sea spray
