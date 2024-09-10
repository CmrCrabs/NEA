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

// Title Page
#page(numbering: none, [
  #v(2fr)
  #align(center, [
    //#image("/Assets/Paperboat.svg", width: 60%)
    #text(24pt, weight: 700, [NEA])
    #v(0.1fr)
    #text(24pt, weight: 700, [Physically Based Ocean Simulation])
    #v(0.1fr)
    #text(16pt, weight: 500, [Zayaan Azam])
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
- Dear IMGUI
  - Bloat-free GUI library with minimal dependencies
- Naga:
  - Shader translation library
- GLAM:
  - Linear algebra library
- Nix:
  - Declarative, reproducible development environment

=== Algorithms
- Discrete Fourier Transform @FT-Wiki
- Fast Fourier Transform (Cooley-Tukey) @FFT-Wiki

#pagebreak()
=== Formulae
\
*Fresnel Specular Reflection (Schlicks Approximation)* []
  - $R(theta) = R_0 + (1 - R_0)(1 - cos(theta))^5$
  - $R_0 = ((n_1 - n_2) / (n_1 + n_2))^2$
  - where $theta$ is the angle between the direction from which incident light is coming and the normal
\
*Dual JONSWAP (4 layered frequency bands) @OW-Spectra*
  - 
\
*Microfacet BRDF / BSDFSFSFSF*
  - 
\
*Beckmann Distribution*
  - 
\
diffuse atmospheric skylight
  - 
\


=== Prototyping
prototyped using tech stack for basic project
https://github.com/CmrCrabs/chaotic-attractors

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
