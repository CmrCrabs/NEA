# A real time physically based ocean simulation and renderer

I have implemented the entire renderer "engine" in wgpu, I have fully implemented 4 out of 6 compute passes. I have implemented a rudimentary inverse discrete fourier transform using compute shaders, however it seems to be smoothing the output so I am treating it as unimplemented until I have a working IFFT. I have implemented the twiddle factor precomputation and have created the butterfly texture for the fourier pass, as well as preparing the fourier components. I have also implemented, without additional external libraries, Dear Imgui as my UI libary of choice. for part of the rendering algorithm i did use code referenced from imgui-wgpu-rs, appropriately accrediting where needed.

[Report](paperwork.pdf)

[Todo List](todo.md)
