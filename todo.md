- [ ] setup dwhatever maps in cascace
- [ ] encode data into them
- [ ] unpack fft data
    - [ ] normal
    - [ ] displacement
    - [ ] foam


compute pass -> ifft -> normal map -> lighting -> foam -> lengthscales -> misc

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

- [ ] move renderer to /gfx/ (?)
- [ ] compute pass impl (?)
- [ ] optimise compute pass
- [ ] optimise ui
- [ ] update ui code to not hardcode and isntead use enum val
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
- [ ] credit biebras


### future improvements
- handle state better
- use push constants
- better optimise
- shore interactions
- more advanced foam
- sea spray
