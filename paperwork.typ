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
    #text(23pt, weight: 700, [Real-Time Physically Based Ocean Simulation])
    #v(0.1fr)
    #text(20pt, weight: 500, [Zayaan Azam])
  ])
  #v(2fr)
])

// Contents Page
#page(outline(indent: true))

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
  + the game is intended to run on both x86 and arm64 devices
  + see response 3.1
  + the game is targeted towards mid to high end systems, however it would be ideal for the solution to be performant on lower end hardware 

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

#pagebreak()
=== Algorithms & Formulae
*Fast Fourier Transform (Cooley-Tukey)* @FFT-Wiki
- \/\/ Currently do not have the prerequisite math to properly understand this - waiting until ive learnt roots of unity
\

*JONSWAP (Joint North Sea Wave Observation Project) Spectrum* @OW-Spectra @JONSWAP-2
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
*Fresnel Specular Reflection (Schlick's Approximation)*  @Schlicks @Blinn-Phong
  $ R(theta) = R_0 + (1 - R_0)(1 - cos theta)^5 $
  where 
  - $R_0 = ((n_1 - n_2) / (n_1 + n_2))^2$
  - $theta$ is the angle between the incident light and the halfway vector @Blinn-Phong
  - $n_1$ & $n_2$ are the refractive indices of the two media @Schlicks
\
*Beckmann Distribution* @Specular-Highlight
  $ k_s = (exp((-tan^2 alpha) / m)) / (pi m^2 cos^4 alpha) $
  where
  - $alpha = arccos(N dot H)$
  - $m$ is the $"RMS"$ slope of the surface microfacets
\

*Microfacet BRDF*
  - 
\
*(Approximate) Subsurface Scattering (Atlas Paper)*
  - 
\
*Distance fog post processing*
  - 
\
*attenuate distance fog based based on height*
  - 
\
*sample hdri skybox for reflections, multiple yiwth schlicks*
  - 
\
*jacobian*
  - 
\

*exponential decay*
  - 
\

*Asynchronous GPU Readback*
- 
\


#pagebreak()
=== Prototyping
A project was undertook in order to test the technical stack and gain experience with graphics programming and managing shaders. I created a halvorsen strange attractor @Halvorsen, and then did some trigonometry to create a basic camera controller.
\
#figure(
  image("assets/chaotic_attractor.png", width: 50%),
  caption: [
    Found at https://github.com/CmrCrabs/chaotic-attractors
  ],
)


#pagebreak()
== Objectives

#pagebreak()
= Bibliography
#bibliography(
  "bibliography.yml",
  title:none,
  full:true,
  style: "ieee"
)
