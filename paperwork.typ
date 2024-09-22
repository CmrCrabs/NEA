// Settings
#set par(justify: true)
#show link: underline
#set page(numbering: "1", margin: 2cm) 
#set text(
  hyphenate: false,
  //font: "EB Garamond"
)
#set heading(numbering: "1.")
#set text(12pt)
#set enum(numbering: "1.1", full: true)
#set list(marker: ([•], [‣],[--]))
#set math.mat(delim: "[");
#set math.vec(delim: "[");

// Title Page
#page(numbering: none, [
  #v(2fr)
  #align(center, [
    //#image("/Assets/Paperboat.svg", width: 60%)
    #text(23pt, weight: 700, [NEA])
    #v(0.1fr)
    #text(23pt, weight: 700, [Real-Time, Physically Based Ocean Simulation & Rendering])
    #v(0.1fr)
    #text(23pt, weight: 500, [Zayaan Azam])
    #v(1.1fr)
  ])
  #v(2fr)
])

// TODO: 
// write objectives
// Expand upon technologies

// Contents Page
#page(outline(indent: true, depth: 3))

= Analysis

== Prelude
\/\/ Fill Later
== Client

=== Introduction
The client is Jahleel Abraham. They are a game developer who require a physically based, performant, configurable simulation of an ocean for use in their game.

=== Questions
+ Functionality
  + "what specific ocean phenomena need to be simulated? (e.g. waves, foam, spray, currents)"
  + "what parameters of the simulation need to be configurable?"
  + "does there need to be an accompanying GUI?"
+ Visuals
  + "do i need to implement an atmosphere / skybox?"
  + "do i need to implement a pbr water shader?"
  + "do i need to implement caustics, reflections, or other light-related phenomena?"
+ Technologies
  + "are there any limitations due to existing technology?"
  + "does this need to interop with existing shader code?"
+ Scope
  + "are there limitations due to the target device(s)?"
  + "are there other performance intesive systems in place?"
  + "is the product targeted to low / mid / high end systems?"

#pagebreak()
=== Interview Notes
+ Functionality
  + it should simulate waves in all real world conditions and be able to generate foam, if possible simulating other phenomena would be nice.
  + all necessary parameters in order to simulate real world conditions, ability to control tile size / individual wave quantity
  + accompanying GUI to control parameters and tile size. GUI should also output debug information and performance statistics
+ Visuals
  + a basic skybox would be nice, if possible include an atmosphere shader
  + implement a PBR water shader, include a microfacet BRDF
  + caustics are out of scope, implement approximate subsurface scattering, use beckmann distribution in combination with brdf to simulate reflections
+ Technologies
  + client has not started technical implementation of project, so is not beholden to an existing technical stack
  + see response 3.1
+ Scope
  + the simulation is intended to run on both x86 and arm64 devices
  + see response 3.1
  + the simulation is targeted towards mid to high end systems, however it would be ideal for the solution to be performant on lower end hardware 

#pagebreak()
== Research
=== Technologies
- Rust:
  - Fast, memory efficient programming language
- WGPU:
  - Graphics library
- Rust GPU:
  - (Rust as a) shader language
- Winit:
  - cross platform window creation and event loop management library
- Dear ImGui
  - Bloat-free GUI library with minimal dependencies
- Naga:
  - Shader translation library
- GLAM:
  - Linear algebra library
- Nix:
  - Declarative, reproducible development environment

=== Simulation Concepts, Formulae, & Algorithms

_I realise I am defining quite a few concepts here. Defining these in reasonable detail is in my opinon a needed step to understand and implement the overarching algorithm._
==== Wave Summation  @Jump-Trajectory @Acerola-SOS
On a high level, the simulation works by summating multiple varying waves as defined below, displacing the vertices of the mesh by the corresponding amplitude for every timestep. When actually implementing this Gerstner waves would be used instead of sine waves, as their vertical and horizontal displacement components would sine and cosine respectively.
  $ A sin (k dot arrow(x) + omega t + Phi) $
  $ &= A e^(i(k dot arrow(x) + omega t + Phi)) \ 
    &= h e^(i(k dot arrow(x) + omega t)) $
  $ eta (arrow(x), t) = sum_k h_0 (k) e^(i(k dot arrow(x) + omega t)) $
where 
- $A$ controls the amplitude of the wave
- $k$ defines the distance between wave maxima, referred to as the wave number
- $t$ is the time
- $omega$ defines the period of time between two maxima, referred to as its angular frequency
- $Phi$ shifts the wave by adding an initial phase
- $k arrow(x) = k_x x + k_z z$, the wave vector, as the wave is 2D so has $x$ & $z$ components
- $eta (arrow(x), t)$ is the displacement of the surface from the sum of waves
- $m,n = - N/2 .. N/2$
- $h_0$ is the fourier amplitude, explained below

==== The (Inverse) Discrete Fourier Transform (DFT) (Unfinished) @Jump-Trajectory
The sum of waves can be computed as an (inverse) DFT if the following conditions are met:
- The Number of Points ($N$)= The Number of Waves ($M$)
- The Coordinates $arrow(x)$ = $m / n L$
- The wavenumbers $k = (2 pi n) / L$
- the coordinates & wavenumbers lie on regular grids
Assuming the above, it can be shown that the summation is equivalent to the DFT ($F_n$) as defined below, just with differing summation limits. Derivation can be seen at 4:35 in @Jump-Trajectory.

  $ "Inverse DFT": F_n = sum_(m=0)^(N-1) f_m e^(2 pi i m/n) $
  $ "Waves Sum": eta_"axis" (t) = sum_(m = - (n / 2))^(N / 2) h_m (t) e^(2 pi i m / N n) $
the parameters here are defined further in the wave spectra.

==== Cooley-Tukey Fast Fourier Transform (FFT) (Unfinished) @FFT-Wiki
The Cooley-Tukey FFT is a common implementation of the FFT algorithm used for fast calculation of the discrete fourier transform. The direct DFT is computed in $O(N^2)$ time whilst the FFT is computed in $O(N log N)$. This is a significant improvement as we are dealing with $M$ (and $N$) in the millions.
// fft is a faster implementation of the dft, computed on O(Nlogn) instead of O(N^2), massive improvement when you are dealing with potentially millions of waves
  $ "this algorithm is ridiculous, will write up after learning roots of unity & partial derivatives" $
\

==== JONSWAP (Joint North Sea Wave Observation Project) Spectrum @OW-Spectra @JONSWAP-2 @Jump-Trajectory @Empirical-Spectra
The fourier amplitude is made up of 2 components, being the directional spread $D (theta, omega)$ and energy spectrum $S(omega)$. The energy spectrum determines the height of the wave at a given frequency. The JONSWAP energy spectrum is a more parameterised version of the Philips Spectrum used in  @JTessendorf, simulating an ocean that is not fully developed (as recent oceanographic literature has determined this does not happen). The increase in parameters allows simulating a wider breadth of real world conditions. 
  $ S(omega) = (alpha g^2) / (omega^5) "exp" [- beta (omega_p / omega)^4] gamma^r $
  $ r = exp [ - (omega -omega_p)^2 / (2w_p ^2 sigma ^2)] $ 
  $ alpha = 0.076 ( (U_(10) ^2) / (F g))^(0.22) $
  where
  - $alpha$ is the intensity of the spectra
  - $beta = 5/4$, a "shape factor", rarely changed @JONSWAP-2
  - $gamma = 3.3$
  - $sigma = cases(
      0.07 "if" omega <= omega_p,
      0.09 "if" omega > omega_p,
    )$ @OW-Spectra
  - $omega$ is the wave frequency ($(2pi) / s$) @JONSWAP-2
  - $omega_p$ is the peak wave frequency
  - $omega_p = 22( (g^2) / (U_10 F))^(1/3) $
  - $U_(10)$ is the wind speed at $10"m"$ above the sea surface @JONSWAP-2
  - $F$ is the distance from a lee shore (a fetch) - distance over which wind blows with constant velocity @OW-Spectra
  - $g$ is gravity
\

==== Hasselman Directional Spread Function (Unfinished) @JTessendorf @Jump-Trajectory @Empirical-Spectra
  - 
\

==== Gaussian Random Numbers (Unfinished)

==== Fourier Amplitude Function (Unfinished) @JTessendorf @Jump-Trajectory @Empirical-Spectra
This is the function that determines the amplitude of the displacement at a given position. It takes together the direction spread function, energy spectra, and gaussian random numbers in order to produce a function $h_0$ for the vertical displacement. this function is then multiplied by $i$ in order to give the horizontal displacement (cosine) instead of the vertical displacement (sine) of the gerstner waves. this is then multiplied by the normalised wave vector $k$ to get the correct direction.
  $ h_0^((y)) = 1 / sqrt(2) (epsilon_1 + i epsilon_2) sqrt( 2 S(omega) D(theta, omega) ( (d omega (k)) / (d k)) Delta k_x Delta k_z) $ 
  $ h_0^((x)) = i k_x 1 / k h_0^((y)) $
  $ h_0^((z)) = i k_z 1 / k h_0^((y)) $
\


==== The Jacobian (Unfinished) @JTessendorf @Acerola-FFT @Atlas-Water @SimSlides
The jacobian describes the "uniqueness" of a transformation. This is useful as where the waves would crash, the jacobian of the displacements goes negative. We can then offset this to bias the results and generate more foam.  

  $ "Will write up once I better understand partial derivatives." $
\

==== Exponential Decay @Exponential-Decay @Atlas-Water @Acerola-FFT @JTessendorf
In order to dissipate the stored foam over time instead of instantaneously, we apply an exponential decay function to each pixel in the texture. This may potentially be replaced by a gaussian blur and fade pass depending on results produced.
  $ N(t) = N_0 e ^(-lambda t) $ 
  where 
  - $N_0$ is the initial quantity
  - $lambda$ is the rate constant
\

==== The Ocean Simulation Algorithm @JTessendorf @Jump-Trajectory @Acerola-FFT @Atlas-Water
\/\/ like 20x more complex than this 
- generate gaussian noise (4 bands)
- jonswap it based on params
- fft to frequency domain (for all 4 bands)
- evolve frequencies with time
- inverse fft to spatial domain
- postprocess results for brdf / pbr
- recombine 4 bands in vertex shader?


=== Lighting Algorithms & Formulae

==== Rendering Equation @Atlas-Water @Acerola-FFT @Acerola-SOS
This abstract equation models how a light ray incoming to a viewer is "formed" (in the context of this simulation). Due to there only being a single light source (the sun), and subsurface scattering @Atlas-Water allowing us to replace the $L_"diffuse"$ and $L_"ambient"$ terms, we are able to take an analytical approach to solving this.

To include surface foam, we _lerp_ between the foam color and $L_"eye"$ based on foam density @Atlas-Water. We also Increase the roughness in areas covered with foam for $L_"specular"$ @Atlas-Water.  $ L_"eye" = (1 - F) L_"scatter" + F(L_"specular" + L_"env_reflected") $
  where
  - $F$ is the fresnel reflectance term
  - $L_"scatter"$ is the light emitted due to subsurface scattering
  - $L_"specular"$ is the reflected light from the source
  - $L_"env_reflected"$ is the reflectioned light from the environemnt

==== Normalisation @Blinn-Phong @Specular-Highlight
When computing lighting using vectors, we are only concerned with the direction of a given vector not the magnitude. In order to ensure the dot product of 2 vectors is equal to the cosine of their angle we normalise the vectors. Throughout this document, a vector $arrow(A)$ when normalised is represented with $hat(A)$.
  $ arrow(A) dot arrow(B) = abs(A) abs(B) cos theta $
  $ hat(A) = arrow(A) / abs(A) => abs(hat(A)) = 1 $ 
  $ therefore hat(A) dot hat(B) = cos theta $
where
  - $theta$ is the angle between vectors $A$ and $B$

==== Surface Normals (Unfinished) @Empirical-Spectra @JTessendorf @Jump-Trajectory
In order to compute the surface normals we need the derivatives of the displacement(s). the values for the derivatives are obtained from the derivative fft above.
  $ arrow(n) = vec(- s_x, 1, -s_z) $
  $ arrow(s) = vec( ((delta eta_y) / (delta x)) / (1 + ((delta eta_x) / (delta x))) ,  ((delta eta_y) / (delta z)) / (1 + ((delta eta_z) / (delta z)))) $

==== Subsurface Scattering @Atlas-Water
This is the phenomenon where some light absorbed by a material eventually re-exits and reaches the viewer. Modelling this realistically is impossible in a real time context with current computing power. Specifically within the context of the ocean, we can approximate it particularly well as the majority of light is absorbed. An approximate formula taking into account geometric attenuation, a crude fresnel factor, lamberts cosine law, and an ambient light is used, alongside various artistic parameters to allow for adjustments. @Atlas-Water
  $ L_"scatter" = ((k_1 H angle.l omega_i dot -omega_o angle.r ^4 (0.5 - 0.5(omega_i dot omega_n))^3 + k_2 angle.l omega_o dot omega_n angle.r ^2) C_"ss" L_"sun") / (1 + lambda (omega_i)) $
  $ L_"scatter" += k_3 angle.l omega_i dot w_n angle.r C_"ss" L_"sun" + k_4 P_f C_f L_"sun" $
  where
  - $H$ is the $"max"(0, "wave height")$
  - $k_1, k_2, k_3, k_4$ are artistic parameters
  - $C_"ss"$ is the water scatter color
  - $C_f$ is the air bubbles color
  - $P_f$ is the density of air bubbles spread in water
  - $angle.l omega_a, omega_b angle.r$ is the $"max"(0, omega_a dot omega_b)$
  - $omega_n$ is the normal
  - $lambda$ is the masking function defined under Smith's $G_1$
\

==== Blinn-Phong Specular Reflection @Blinn-Phong
This is a (relatively) simplistic, empirical model to determine the specular reflections of a material. It allows you to simulate isotropic surfaces with varying roughnesses whilst remaining very computationally efficient. The model uses "shininess" as an input parameter, whilst the standard to use roughness (due to how PBR models work). In order to account for this when wishing to increase roughness we decrease shininess.
  $ L_"specular" = (hat(N) dot hat(H))^S $
  $ hat(H) = hat(L) + hat(V) $
  where
  - $hat(H)$ is the normalised halfway vector
  - $hat(N)$ is the normalised surface normal
  - $hat(V)$ is the camera view vector
  - $hat(L)$ is the light source vector
  - $S$ is the shininess of the material

==== Fresnel Reflectance (Schlick's Approximation)  @Acerola-SOS @Blinn-Phong @Schlicks
The fresnel factor is a multiplier that scales the amount of reflected light based on the viewing angle. The more grazing the angle the more light is refleceted.
  $ F(theta) = F_0 + (1 - F_0)(1 - arrow(N) dot arrow(V))^5 $
  where 
  - $F_0 = ((n_1 - n_2) / (n_1 + n_2))^2$
  - $theta$ is the angle between the incident light and the halfway vector @Blinn-Phong
  - $n_1$ & $n_2$ are the refractive indices of the two media @Schlicks
  - $arrow(N)$ is the normal vector
  - $arrow(V)$ is the view vector
\

==== Environment Reflections @Acerola-SOS
In order to get the color of the reflection for a given pixel, we compute the reflected vector from the normal and view vector. We then sample the corresponding point on the skybox's cubemap and use that color as the reflected color. This method is somewhat simplistic, and not physically based.
  $ hat(R) = 2 hat(N) ( hat(N) dot hat(V)) - hat(V) $
  where
  - $hat(N)$ is the normal vector for the point
  - $hat(V)$ is the camera view vector
  - $hat(R)$ is the vector that points to the point on the cubemap which we sample

==== Post Processing @Acerola-SOS
To hide the imperfect horizon line we use a distance fog. This is then attenuated based oon height. In order to do this we use the depth buffer to determine the depth of each pixel and then based on that scale the light color to be closer to a defined fog color. Finally we blend a sun into the skybox based on the light position.
\

==== Color Grading @Acerola-SOS
in order to really sell the sun being as bright as it would be on an open ocean, we apply a bloom pass to the whole image. In order to prevent it from being completely blown out we then apply a tone mapping to rebalance the colors. 

\

=== PBR-Specific Algorithms / Formulae
==== Microfacet BRDF @Atlas-Water @Acerola-FFT @CC-BRDF
The BRDF (Bidirectional Reflectance Distribution Function) is used to determine the reflectance of a sample. There are many methods of doing this - the one used here is derived from microfacet theory. $D(h)$ can be any distribution function. The geometric attenuation is a function that models how some reflections are masked / shadowed by the microfacets "geometry" and serves to counteract the fresnel.
  $ f_"microfacet" = (F(omega_i, h) G(omega_i, omega_o, h) D(h)) / (4(n dot omega_i) (n dot omega_o)   ) $ 
  where
  - $F(omega_i, h)$ is the Fresnel Reflectance
  - $D(h)$ is the Distribution Function
  - $G(omega_i, omega_o, h)$ is the Geometric Attenuation
\

==== GGX Distribution @CC-BRDF
The distribution function used in the BRDF to model the proportion of microfacet normals aligned with the halfway vector. This is an improvement over the beckmann distribution due to the graph never reaching 0 and only tapering off at the extremes.
  $ D_"GGX" = (alpha ^2) / (pi ( (alpha^2 - 1)cos^2 theta_h + 1)^2) $
where
  - $alpha = "roughness" ^2$
  - $cos theta_h = hat(n) dot hat(H)$
  - where $hat(n)$ and $hat(H)$ are the surface normal and halfway vector respectively

==== Geometric Attenuation Function (Smith's $G_1$ Function) @CC-BRDF
The geometric attenuation function used within the microfacet BRDF. The $lambda$ term changes depending on the distribution function used. 
  $ G_1 = 1 / (1 + lambda(a)) $
  $ a = (hat(H) dot hat(S)) / (alpha sqrt(1 - (hat(H) dot hat(S))^2)) $
  $ lambda = (-1 + sqrt( 1 + a^(-2))) / 2 $
where
- $alpha = "roughness"^2$
- $hat(H)$ is a microfacet normal
- $hat(S)$ is either the (normalised) light or view vector
\

==== Specular Reflection (Unfinished) @Atlas-Water
A physically based specular reflection model. This is an analytical approach to the indefinete integral which determines the specular color @Atlas-Water. Undecided on whether this will be used as depending on other facets may result in a minimal visual difference for maximal effort.
  $ L_"specular" = (L_"sun" F(omega_h, omega_"sun") p_22 (omega_h)) / (4 (omega_n dot omega_"eye") (1 + lambda (omega_"sun")) + lambda (omega_"eye")) $
  where
  - $omega_"sun", omega_"eye", omega_h$ is the sun / eye / half vector direction
  - $omega_n$ is the macronormal
  - $p_22$ is a distribution function, defined further in LEADR @LEADR
  - $lambda$ is the function defined under Smith's $G_1$
\

==== Environment Reflections (Unfinished) @Atlas-Water @CC-BRDF @LEADR
Bleeding edge environment reflections based on the LEADR paper on the topic @LEADR. The implementation of this will be heavily dependent on how the simple cubemap reflections look as implementing this in conjunction with other PBR lighting can constitute an nea on its own.

#pagebreak()
=== Prototyping
A prototype was made in order to test the technical stack and gain experience with graphics programming and managing shaders. I created a Halvorsen strange attractor @Halvorsen, and then did some trigonometry to create a basic camera controller using Winit's event loop.
\
#figure(
  image("assets/chaotic_attractor.png", width: 50%),
  caption: [
    Found at https://github.com/CmrCrabs/chaotic-attractors
  ],
)

=== Project Considerations
The project will be split into 3 major stages, being the simulation, non PBR lighting and PBR lighting. The simulation will most likely take the bulk of the project duration as implementing the FFT and a GUI with just a graphics library is already a major undertaking. I will then implement the Blinn-Phong lighting model @Blinn-Phong in conjunction with the subsurface scattering seen in atlas @Atlas-Water. Beyond this I will implement full PBR lighting using a microfacet BRDF and statistical distribution functions in order to simulate surface microfacets.

If time were to allow so, I would also implement the PBR environment reflections model as seen in the LEADR paper @LEADR, however doing so would require overhauling most of my lighting systems, and implementing math I barely understand.

finally, I would also like to look into implementing a sky color simulation based on sun position - as this would allow the complete simulation of a realistic day night cycle of any real world ocean conditions.

#pagebreak()
== Objectives (Unfinished)

+ Scene
  + Language & Environment Setup
    + setup all dependencies 
    + have development shell to ensure correct execution
    + ensure compatability for all engines
  + Window & Compatability
    + ensure compatability with windows, macos & wayland (& X11?) linux
    + title & respects client side rendering of respective os
  + Data Structure
    + talk abt shared data structures
    + create struct for all variables 
    + camera struct etc
  + Render Pipeline
    + list steps and that it works
    // alot of yap
  + Event Loop
    + able to detect mouse movement for camera inputs
    + able to detect mouse down for camera inputs
    + escape to close
    + resize
    + redraw requested
+ Simulation
  + Startup
  + On Parameter Change
  + Every Frame
// generate spectrum -> evolve in frequency domain -> transform to spatial domain using inverse FFT,
+ Rendering
  + Lighting
    + calculate light / view / halfway / normal vectors
    + normalise all vectors
    + fresnel
    + subsurface scattering
    + specular reflections 
      + blinn-phong
      + pbr
        + microfacet brdf
        + distribution function
        + geometric attenuation
    + env reflections 
      + acerola
      + LEADR
    + lerp between this and foam
    + adjust roughness of areas with foam
  + Post Processing / Scene
    + HDRI
    + Sun
    + distance fog
    + attenuation of fog
    + bloom pass for sun
    + tone mapping
+ Interaction
  + Orbit Camera
    + zoom
    + revolve
    + aspect ratio
  + Graphical User Interface
    + select hdri - file picker
    + parameter sliders
    + parameter input boxes
    + parameter checkboxes
      + toggle between pbr / non pbr lighting
    + color select wheel (imgui) for parameters 

#pagebreak()
= Bibliography
#bibliography(
  "bibliography.yml",
  title:none,
  full:true,
  style: "ieee"
)
