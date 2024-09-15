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

=== Simulation Algorithms & Formulae
*Fast Fourier Transform (Cooley-Tukey)* @FFT-Wiki
- Currently do not have the prerequisite math to properly understand this - waiting until ive learnt roots of unity
- this is where most of the complexity of the project comes from
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

*Jacobian*
  - \/\/ Similar to the FFT, I need more math knowledge to properly understand how to do this - waiting until Ive completed all of matrices.
  - need to find the jacobian determinant of the transform
  - of water displacement vectors
  - and then offset to bias negative results
\

*Exponential Decay* @Exponential-Decay
  $ N(t) = N_0 e ^(-lambda t) $ 
  where 
  - $N_0$ is the initial quantity
  - $lambda$ is the rate constant
\

=== Non-PBR Lighting Algorithms & Formulae

*Rendering Equation* @Atlas-Water @Acerola-FFT @Acerola-SOS
  $ L_"eye" = (1 - F) L_"scatter" + L_"specular" + F L_"env_reflected" $
  where
  - $F$ is the fresnel reflectance
  - $L_"scatter"$ (atlas subs approx) (includes ambient)
  - $L_"specular"$ either atlas approx or blinn-phong
  - $L_"env_reflected"$ cubemap reflections per acerola or atlas 
  - multiply specular and env. reflections by fresnel
  to include surface foam, _lerp_ between the foam color and $L_"eye"$ based on foam density. Increase the roughness in areas covered with foam for $L_"specular"$.

*Subsurface Scattering* @Atlas-Water
  $ L_"scatter" = ((k_1 H angle.l omega_i dot -omega_o angle.r ^4 (0.5 - 0.5(omega_i dot omega_n))^3 + k_2 angle.l omega_o dot omega_n angle.r ^2) C_"ss" L_"sun") / (1 + Lambda (omega_i)) $
  $ L_"scatter" += k_3 angle.l omega_i dot w_n angle.r C_"ss" L_"sun" + k_4 P_f C_f L_"sun" $
  where
  - $H$ is the $"max"(0, "wave height")$
  - $k_1, k_2, k_3, k_4$ are artistic parameters
  - $C_"ss"$ is the water scatter color
  - $C_f$ is the air bubbles color
  - $P_f$ is the density of air bubbles spread in water
  - $angle.l omega_a, omega_b angle.r$ is the $"max"(0, omega_a dot omega_b)$
  - $omega_n$ is the normal
\

*Blinn-Phong Specular Reflection* @Blinn-Phong
  $ L_"specular" = arrow(H) dot arrow(N) $
  $ arrow(H) = (arrow(L) + arrow(V)) / (abs(arrow(L) + arrow(V))) $
  where
  - $arrow(H)$ is the normalised halfway vector
  - $arrow(N)$ is the normalised surface normal
  - $arrow(V)$ is the camera view vector
  - $arrow(L)$ is the light source vector

*Fresnel Reflectance (Schlick's Approximation )*  @Acerola-SOS @Blinn-Phong @Schlicks
  $ F(theta) = F_0 + (1 - F_0)(1 - arrow(N) dot arrow(V))^5 $
  where 
  - $F_0 = ((n_1 - n_2) / (n_1 + n_2))^2$
  - $theta$ is the angle between the incident light and the halfway vector @Blinn-Phong
  - $n_1$ & $n_2$ are the refractive indices of the two media @Schlicks
  - $arrow(N)$ is the normal vector
  - $arrow(V)$ is the view vector
\

*Environment Reflections* @Acerola-SOS
  $ arrow(R) = 2 arrow(N) ( arrow(N) dot arrow(V)) - arrow(V) $
  where
  - $arrow(N)$ is the normal vector for the point
  - $arrow(V)$ is the camera view vector
  - $arrow(R)$ is the vector that points to the point on the cubemap which we sample

*Distance Fog Post Processing (Unfinished)* @Acerola-SOS
  - hides issues with fresnel at grazing angles @Atlas-Water
\

*Atmospheric Scattering (Unfinished)* @Acerola-SOS
  - attenuate distance fog based on height 
\

*General Effects (Unfinished)* @Acerola-SOS
  - additively blend a sun with skybox
  - apply a bloom pass
  - cinematic tone mapping
\



=== PBR Lighting Algorithms / Formulae
*Microfacet BRDF* @Atlas-Water
  $ f_"microfacet" = (F(omega_i, h) G(omega_i, omega_o, h) D(h)) / (4(n dot omega_i) (n dot omega_o)   ) $ 
  where
  - $F(omega_i, h)$ is the Fresnel Reflectance
  - $D(h)$ is the Distribution Function
  - $G(omega_i, omega_o, h)$ is the Geometric Attenuation
\

*Beckmann Distribution* @Atlas-Water @Specular-Highlight
  $ k_s = (exp((-tan^2 alpha) / m)) / (pi m^2 cos^4 alpha) $
  where
  - $alpha = arccos(N dot H)$
  - $m$ is the $"RMS"$ slope of the surface microfacets
\

*Geometric Attenuation Function, Smith GGX (Unfinished)*
  - 
\

*Fresnel Reflectance (Schlick's Approximation )*  @Acerola-SOS @Blinn-Phong @Schlicks
  $ F(theta) = F_0 + (1 - F_0)(1 - arrow(N) dot arrow(V))^5 $
  where 
  - $F_0 = ((n_1 - n_2) / (n_1 + n_2))^2$
  - $theta$ is the angle between the incident light and the halfway vector @Blinn-Phong
  - $n_1$ & $n_2$ are the refractive indices of the two media @Schlicks
  - $arrow(N)$ is the normal vector
  - $arrow(V)$ is the view vector
\

*Specular Reflection* @Atlas-Water
  $ L_"specular" = (L_"sun" F(omega_h, omega_"sun") p_22 (omega_h)) / (4 (omega_n dot omega_"eye") (1 + Lambda (omega_"sun")) + Lambda (omega_"eye")) $
  where
  - $omega_"sun", omega_"eye", omega_h$ is the sun / eye / half vector direction
  - $omega_n$ is the macronormal, in this case $vec(0, 0, 1)$
\

#pagebreak()
=== Prototyping
A project was undertook in order to test the technical stack and gain experience with graphics programming and managing shaders. I created a Halvorsen strange attractor @Halvorsen, and then did some trigonometry to create a basic camera controller using Winit's event loop.
\
#figure(
  image("assets/chaotic_attractor.png", width: 50%),
  caption: [
    Found at https://github.com/CmrCrabs/chaotic-attractors
  ],
)

=== Project Considerations
- talk abt pbr complexities in each part
- complexitites of distribution functions
- microfacet theory
- (so) using blinn phong
- if time allows will also use pbr cubemap reflection sampling


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
