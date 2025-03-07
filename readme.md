# A real time physically based ocean simulation and renderer

fft works. 90fps at 512x512x1 on an intel integrated iris xe gpu

- you need to download a hdri and store it at assets/hdris/<hdri_name>.exr
- currently defaulting to "kloofendal.exr"
- default hdri can be changed in src/renderer.rs line 42

https://polyhaven.com/a/kloofendal_43d_clear_puresky

[Report](paperwork.pdf)
[Todo List](todo.md)
