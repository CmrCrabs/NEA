// Settings
#set par(justify: true)
#show link: underline
#set page(
  numbering: "1",
  margin: 2cm,
  paper: "us-letter",
) 
#set text(
  hyphenate: false,
  font: "EB Garamond"
)
#set heading(numbering: "1.", offset: 0)
#set text(12pt)
#set enum(numbering: "1.1", full: true)
#set list(marker: ([•], [‣],[--]))
#set math.mat(delim: "[");
#set math.vec(delim: "[");

// Title Page
// disable justification
#page(numbering: none, [
  #v(2fr)
  #align(center, [
    //#image("/Assets/ocean.png", width: 60%)
    #text(23pt, weight: 700, [NEA])
    #v(0.1fr)
    #text(23pt, weight: 700, [Real-Time, Empirical Ocean Simulation & Physically Based Renderer])
    #v(0.1fr)
    #text(23pt, weight: 500, [Zayaan Azam])
    #v(1.1fr)
  ])
  #v(2fr)
])

// TODO: 
// replicate old academic paper style
// https://journals.ametsoc.org/view/journals/phoc/5/3/1520-0485_1975_005_0410_optoer_2_0_co_2.xml?tab_body=pdf

// jacobian & eigenvalue
// define derivatives
// how to pack 8 ffts into 4
// expand upon technologies
// post processing tonemapping
// Objectives
// look into blurring for sea of thieves foam

// explain IDFT in terms of indices (keith lantz & jump trajectory)
// IFFT

// Contents Page
#page(outline(indent: true, depth: 2))

== Abstract
// TODO: SYNOPSIS
\/\/ synopsis
// the goal of this project was to....
= Analysis

== Client Introduction
The client is Jahleel Abraham. They are a game developer who require a physically based, performant, configurable simulation of an ocean for use in their game. They also require a physically based lighting model derived from microfacet theory, including PBR specular, and empirical subsurface scattering. Also expected is a fully featured GUI allowing direct control over every input parameter, and a functioning camera controller.

== Interview Questions
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
  + "does this need to interop with existing code?"
+ Scope
  + "are there limitations due to the target device(s)?"
  + "are there other performance intesive systems in place?"
  + "is the product targeted to low / mid / high end systems?"

#pagebreak()
== Interview Notes
+ Functionality
  + it should simulate waves in all real world conditions and be able to generate foam, if possible simulating other phenomena would be nice.
  + all necessary parameters in order to simulate real world conditions, ability to control tile size / individual wave quantity
  + accompanying GUI to control parameters and tile size. GUI should output frametime, and the current state of every parameter.
+ Visuals
  + a basic skybox would be nice, if possible include an atmosphere shader
  + implement a PBR water shader, include a microfacet BRDF
  + caustics are out of scope, implement approximate subsurface scattering, use GGX distribution in combination with brdf to simulate reflections
+ Technologies
  + client has not started technical implementation of project, so is not beholden to an existing technical stack
  + see response 3.1
+ Scope
  + the simulation is intended to run on both x86 and arm64 devices
  + see response 3.1
  + the simulation is targeted towards mid to high end systems, ideally the solution would also be performant on lower end hardware 

#pagebreak()
== Technologies
- Rust:
  - Fast, memory efficient programming language
- wgpu:
  - Graphics library
- Rust GPU:
  - (Rust as a) shader language
- Winit:
  - cross platform window creation and event loop management library
- Dear ImGui
  - Bloat-free GUI library with minimal dependencies
- GLAM:
  - used for various linear algebra operations
- Pollster / Env Logger:
  - used to read errors, as wgpu does not log as would be standard
- Image:
  - used to read HDRIs from a file into memory for processing
- Nix:
  - Used to create a declarative, reproducible development environment
// TODO LINK TO UI.RS // TODO: properly credit and comment

// INCLUDE COOL GRAPH OF TEXTURES
== Algorithm Overview (Unfinished) @JTessendorf @Empirical-Spectra 
// TODO: link to documented design
Below is a high-level explanation of the algorithm used in this project, primarily for providing context for the upcoming theory. It is explained in more detail in the documented design. 
\
On Startup:
- Generate gaussian random number pairs, and store them into a texture, on the CPU
- Compute the butterfly operation's twiddle factors, and indices, and store them into a texture
On Parameter Change (For Every Lengthscale):
- Compute the initial wave vectors and dispersion relation, and store into a texture
- Compute the initial frequency spectrum for each wave vector, and store into a texture
- Compute the conjugate of each wave vector, and store into a texture
Frame-by-Frame:
- For Every Lengthscale:
  - Evolve initial frequency spectrum through time
  - Pre-compute & store amplitudes for FFT into textures
  - perform IFFT for height displacement
  - perform IFFT for normal calculation
  - perform IFFT(s) for jacobian
  - Process & merge IFFT results into displacement, normal, and foam maps 
- Create a fullscreen quad and render skybox, sun & fog to framebuffer
- Offset tiles based on instance index & centering
- Combine values for all 3 lengthscales & offset vertices based on displacement map
- Compute lighting value and render result to framebuffer

== Spectrum Generation
=== Nomenclature
To compute the inital spectra, we rely on various input parameters and definitions shared between various functions and parts of the program. @sim-variables lists the relevant symbols, meanings and relationships where appropriate. Note that throughout this project we are defining the positive $y$ direction as "up".
\
\
\
#figure(
  table(
    columns: 2,
    table.header[*Variable*][*Definition*],
    [$arrow(k)$], [2D Wave Vector],
    [$k_x$], [X Component of wave vector],
    [$k_z$], [Z Component of wave vector],
    [$k$], [Magnitude of the wave vector],
    [$t$], [time],
    [$arrow(x) = [x_x, x_z]$], [Position vector in world space],
    [$omega = phi(k)$], [Dispersion relation],
    [$omega_p$], [Peak Frequency],
    [$g$], [Gravitational Constant = 9.81],
    [$h$], [Ocean Depth],
    [$U_(10)$], [Wind speed 10m above surface],
    [$F$], [Fetch, distance over which wind blows],
    [$theta$], [Wind direction = $"arctan2"(k_z, k_x) - theta_0$],
    [$theta_0$], [Wind direction offset],
    [$L$], [Lengthscale],
    [$L_x = L_z$], [Simulation Dimensions, power of two],
    [$xi$], [Swell amount],
    [$xi_r, xi_i$], [Gaussian Random Numbers],
    [$lambda$], [Choppiness factor],
    [$lambda$], [Choppiness factor],
    [$kappa$], [Foam bias],
    [$mu$], [Foam injection threshold],
    [$zeta$], [Foam decay rate],
    [$I$], [Foam value],
  ),
  caption: [Simulation Variable Definitions],
) <sim-variables>

#pagebreak()
=== Dispersion Relation @JTessendorf @Empirical-Spectra
The relation between the travel speed of the waves and their wavelength, written as a function relating angular frequency $omega$ to wave number $arrow(k)$. This simulation involves finite depth, and so we will be using a dispersion relation that considers it.@Empirical-Spectra

$ omega = phi(k) =  sqrt(g k tanh (k h)) $
$ (d phi(k)) / (d k) = (g( tanh (h k) + h k sech^2 (h k))) / (2 sqrt(g k tanh (h k))) $

=== Non-Directional Spectrum (JONSWAP) @Empirical-Spectra @OW-Spectra @Jump-Trajectory @Acerola-FFT
The JONSWAP energy spectrum is a more parameterised version of the Pierson-Moskowitz spectrum, and an improvement over the Philips Spectrum used in @JTessendorf, simulating an ocean that is not fully developed (as recent oceanographic literature has determined this does not happen). The increase in parameters allows simulating a wider breadth of real world conditions. 
  $ S_"JONSWAP" (omega) = (alpha g^2) / (omega^5) "exp" [- 5/4 (omega_p / omega)^4] 3.3^r $
  $ r = exp [ - (omega -omega_p)^2 / (2omega_p ^2 sigma ^2)] $ 
  $ alpha = 0.076 ( (U_(10) ^2) / (F g))^(0.22) $
  $ omega_p = 22( (g^2) / (U_10 F))^(1/3) $
  $ sigma = cases(
      0.07 "if" omega <= omega_p,
      0.09 "if" omega > omega_p,
    ) $

=== Depth Attenuation Function (Approximation of Kitaiigorodskii) @Empirical-Spectra
JONSWAP was fit to observations of waves in deep water. This function adapts the JONSWAP spectrum to consider ocean depth, allowing a realistic look based on distance to shore. The actual function is quite complex for a relatively simple graph, so can be well approximated as below @Empirical-Spectra. 
$ Phi (omega, h) = cases(
  1 / 2 omega_h ^2 "if" omega_h <= 1,
  1 - 1 / 2 (2 - omega_h)^2 "if" omega_h > 1,
) $
$ omega_h = omega sqrt(h / g) $

=== Directional Spread Function (Donelan-Banner) @Empirical-Spectra <Donelan-Banner>
The directional spread models how waves react to wind direction @Jump-Trajectory. This function is multiplied with the non-directional spectrum in order to produce a direction dependent spectrum @Empirical-Spectra. 

$ D (omega, theta) = beta_s / (2 tanh (beta_s pi)) sech^2(beta_s theta) $
$ beta_s = cases( 
  2.61 (omega / omega_p)^1.3 "if" omega / omega_p < 0.95,
  2.28 (omega / omega_p)^(-1.3) "if" 0.95 <= omega / omega_p < 1.6,
  10^epsilon "if" omega / omega_p >= 1.6,
) $
$ epsilon = -0.4 + 0.8393 exp[-0.567 ln ( (omega / omega_p)^2 )] $

=== Swell @Empirical-Spectra
Swell refers to the waves which have travelled out of their generating area @Empirical-Spectra. In practice, these would be the larger waves seen over a greater area. the directional spread function including swell is based on combining donelan-banner with a swell function as below. The integral seen in $Q_"final"$ is computed numerically using the rectangle rule. $Q_xi$ is a normalisation factor to satisfy the condition specified in equation (31) in @Empirical-Spectra, approximation taken from @Jump-Trajectory

$ D_"final" (omega, theta) = Q_"final" (omega)  D_"base" (omega, theta) D_epsilon (omega, theta) $
$ Q_"final" (omega) = ( integral_(- pi)^(pi) D_"base" (omega, theta) D_xi (omega, theta) d theta )^(-1)  $
$ D_xi = Q_xi (s_xi) |cos (theta / 2)|^(2 s_xi) $
$ s_xi = 16 tanh (omega_p / omega) xi^2 $
$ Q_xi (s_xi) = cases(
  -0.000564 s_xi^4 + 0.00776 s_xi^3 - 0.044 s_xi^2 + 0.192 s_xi + 0.163 "if" s_xi < 5,
  -4.80 times 10^-8 s_xi^4 + 1.07 times 10^(-5) s_xi^3 - 9.53 times 10^(-4) s_xi^2 + 5.90 times 10^(-2) s_xi + 3.93e-01 "otherwise"
)
$ 
#pagebreak()
=== Directional Spectrum Function @Empirical-Spectra
The TMA spectrum below is an undirectional spectrum that considers depth.
$ S_"TMA" (omega, h) = S_"JONSWAP" (omega) Phi (omega, h) $
This takes inputs $omega, h$, whilst we need it to take input $arrow(k)$ per Tessendorf @JTessendorf - in order to do this we apply the following transformation @Empirical-Spectra. Similarly, to make the function directional, we also need to multiply it by the directional spread function  @Empirical-Spectra.
$ S_"TMA" (arrow(k)) = 2 S_"TMA" (omega, h) (d omega) / (d k) 1 / k Delta k_x Delta k_z D (omega, theta) $
$ Delta k_x = Delta k_z = (2 pi) / L $

#pagebreak()
== Ocean Geometry & Foam
All of the following is repeated for every lengthscale, with the final output values being the sum of the individual displacement, normal, and foam maps.
=== Displacements @Code-Motion @Jump-Trajectory @Acerola-FFT @JTessendorf @Keith-Lantz
For a field of dimensions $L_x$ and $L_z$, we calculate the displacements at positions $arrow(x)$ by summating multiple sinusoids with complex, time dependant amplitudes.  @JTessendorf. By arranging the equations into a specific format, we can convert the frequency domain representation of the wave into the spatial domain using the inverse discrete fourier transform.
  $ "Vertical Displacement (y)": h(arrow(x),t) = sum_(arrow(k)) hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Horizontal Displacement (x):" lambda D_x (arrow(x), t) = sum_arrow(k) -i arrow(k)_x / k hat(h)(arrow(k), t) e^(i arrow(k) dot arrow(x)) $
  $ "Horizontal Displacement (z):" lambda D_z (arrow(x), t) = sum_arrow(k) -i arrow(k)_z / k hat(h)(arrow(k), t) e^(i arrow(k) dot arrow(x)) $

=== Derivatives @JTessendorf @Jump-Trajectory
For lighting calculations and the computation of the jacobian, we require the derivatives of the above displacements. As the derivative of a sum is equal to the sum of the derivatives, we compute exact derivatives using the following summations.
  $ "X Component of Normal": epsilon_x (arrow(x), t) = sum_(arrow(k)) i k_x hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Z Component of Normal": epsilon_z (arrow(x), t) = sum_(arrow(k)) i k_z hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $

  $ "Jacobian xx": (d D_x) / (d x) = sum_(arrow(k)) -k_x^2 /k hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Jacobian zz": (d D_x) / (d x) = sum_(arrow(k)) -k_z^2 /k hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Jacobian xz": (d D_x) / (d z) = sum_(arrow(k)) -(k_x k_z) /k hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $

=== Frequency Spectrum Function @JTessendorf @Jump-Trajectory @Acerola-FFT
This function defines the amplitude of the wave at a given point in space at a given time depending on it's frequency. The frequency is generated via the combination of 2 gaussian random numbers and a energy spectrum in order to simulate real world ocean variance and energies. Note that the time dependant component of the exponent is pushed into this amplitude, in order to simplify the summation.
  $ hat(h)_0(arrow(k)) = 1 / sqrt(2) (xi_r + i xi_i) sqrt( S_"TMA" (arrow(k))) $ 
  $ hat(h)(arrow(k), t) = hat(h)_0(arrow(k)) e^(i phi(k)t) + h_0 (-k) e^(-i phi(k) t) $

=== Box-Muller Transform @Gaussian
The ocean exhibits gaussian variance in the possible waves. Due to this the frequency spectrum function is varied by gaussian random numbers with mean 0 and standard deviation 1, which we generate using the box-muller transform converting from uniform variates from 0..1. @JTessendorf. Derivation is from polar coordinates, by treating x and y as cartesian coordinates, more details at @Gaussian
$ xi_r, xi_i ~ N(0, 1) $
$ xi_r = sqrt(-2.0 ln (u_1)) cos (2 pi u_2) $
$ xi_i = sqrt(-2.0 ln (u_1)) sin (2 pi u_2) $

=== Foam, The Jacobian, and Decay @JTessendorf @Acerola-FFT @Code-Motion @Empirical-Spectra
The jacobian describes the "uniqueness" of a transformation. This is useful as where the waves would crash, the jacobian determinant of the displacements goes negative. Per Tessendorf @JTessendorf, we compute the determinant of the jacobian for the horizontal displacement, $D(arrow(x), t)$.
  $ J(x) = J_"xx" J_"zz" - J_"xz" J_"zx" $
  $ J_"xx" = 1 + lambda (d D_x)/(d x) , J_"zz" = 1 + lambda (d D_z)/(d z) $
  $ J_"xz" = J_"zx" = 1 + lambda (d D_x)/(d z) $
we then threshold the value such that $J(arrow(x)) < mu$, wherein if true we inject foam into the simulation at the given point. This value should accumulate (and decay) over time to mimic actual ocean foam, which is achieved by modulating the previous foam value ($I_0$) by an exponential decay function:

$ J_"biased" = kappa - J(arrow(x)) $
$ I = cases(
  I_0 e^(-zeta) + J_"biased" "if" J_"biased" > mu,
  I_0 e^(-zeta) "if" J_"biased" < mu,
) $

=== Cascades (Unfinished) @Jump-Trajectory @JTessendorf @Empirical-Spectra

#pagebreak()
== The Fourier Transform (Unfinished)
=== 2D GPGPU Cooley-Tukey Radix-2 Inverse Fast Fourier Transform (Unfinished) @Code-Motion @JTessendorf @Jump-Trajectory
The Cooley-Tukey FFT is a common implementation of the FFT algorithm used for fast calculation of the DFT. The direct DFT is computed in $O(N^2)$ time whilst the FFT is computed in $O(N log N)$. This is a significant improvement as we are dealing with summations in the order of $10^4 - 10^6$ multiple times per frame. The FFT exploits the redundancy in DFT computation in order to increase performance, but is only applicable when the following conditions are met:
- $L_x = L_z = 2^x, x in ZZ$
- the coordinates and wave vectors lie on a regular grid
\
The overarching FFT algorithm is as follows:
- kA


=== Butterfly Texture (Unfinished)

=== Butterfly Operations (Unfinished)

=== Permutation (Unfinished)

#pagebreak()
== Post Processing
=== Nomenclature
Definitions of the initial variables and parameters for post processing. Note that there are alot of omitted scaling variables and similar that are applied throughout the shader, these are all of the format ```rust consts.sim/shader.variable_name```
\
\
#figure(
  table(
    columns: 2,
    table.header[*Variable*][*Definition*],
    [$L_"eye"$], [final light value for a fragment],
    [$L_"scatter"$], [light re-emitted due to subsurface scattering],
    [$L_"specular"$], [light reflected from the sun],
    [$L_"env_reflected"$], [light reflected from the environment],
    [$F$], [Fresnel Reflectance],
    [$hat(H)$], [Halfway vector],
    [$hat(N)$], [Surface normal],
    [$hat(V)$], [Camera view vector],
    [$hat(L)$], [Light source vector],
    [$k_1$], [Subsurface Height Attenuation],
    [$k_2$], [Subsurface Reflection Scale Factor],
    [$k_3$], [Subsurface Diffuse Scale Factor],
    [$k_4$], [Subsurface Ambient Scale Factor],
    [$C_"ss"$], [Water Scatter Color],
    [$C_f$], [Air Bubbles Color],
    [$L_"sun"$], [Sun Color],
    [$P_f$], [Air Bubbles Density],
    [$W_"max"$], [Max between 0 and wave height],
    [$angle.l omega_a, omega_b angle.r$], [$"max"(0, (omega_a dot omega_b))$],
    [$lambda_"GGX"$], [Smith's $G_1$ masking function],
    [$n_1$], [Refractive index of water],
    [$n_1$], [Refractive index of air],
    [$S$], [The Shininess of the material for fresnel],
    [$B$], [The Shininess of the material for blinn phong],
    [$G_2$], [Smith's $G_2$ geometric attenuation function],
    [$D_"GGX"$], [GGX Distribution of microfacet normals],
    [$alpha$], [Roughness of the surface],
  ),
  caption: [Post Processing Variable Definitions],
) <pp-variables>

=== Rendering Equation @Atlas-Water @Acerola-FFT @Acerola-SOS
This abstract equation models how a light ray incoming to a viewer is "formed" (in the context of this simulation). Due to there only being a single light source (the sun), subsurface scattering @Atlas-Water can be used to replace the standard $L_"diffuse"$ and $L_"ambient"$ terms.

To include surface foam, we _lerp_ between the foam color and $L_"eye"$ based on foam density @Atlas-Water. We also Increase the roughness in areas covered with foam for $L_"specular"$ @Atlas-Water.  

$ L_"eye" = (1 - F) L_"scatter" + L_"specular" + F L_"env_reflected" $

=== Normalisation  & Surface Normals @Blinn-Phong @JTessendorf @Jump-Trajectory
When computing lighting using vectors, we are only concerned with the direction of a given vector not the magnitude. In order to ensure the dot product of 2 vectors is equal to the cosine of their angle we normalise the vectors.
\

In order to compute the surface normals we sum, for each lengthscale, the value of $epsilon (arrow(x), t)$ such that the following is true, which is then normalised.
  $ arrow(N) = vec(
  -epsilon_x (arrow(x), t),
  1, 
  -epsilon_z (arrow(x), t),
  ) $

=== Subsurface Scattering @Atlas-Water @Acerola-FFT
This is the phenomenon where some light absorbed by a material eventually re-exits and reaches the viewer. Modelling this realistically is impossible in a real time context (with my hardware). Specifically within the context of the ocean, we can approximate it particularly well as the majority of light is absorbed. An approximate formula taking into account geometric attenuation, a crude fresnel factor, lamberts cosine law, and an ambient light is used, alongside various artistic parameters to allow for adjustments. @Atlas-Water
  $ L_"scatter" = ((k_1 W_"max" angle.l hat(L), -hat(V) angle.r ^4 (0.5 - 0.5(hat(L) dot hat(N)))^3 + k_2 angle.l hat(V), hat(N) angle.r ^2) C_"ss" L_"sun") / (1 + lambda_"GGX") $
  $ L_"scatter" += k_3 angle.l hat(L), hat(N) angle.r C_"ss" L_"sun" + k_4 P_f C_f L_"sun" $

=== Fresnel Reflectance (Schlick's Approximation)  @Acerola-SOS @Blinn-Phong @Schlicks @Acerola-BRDF
The fresnel factor is a multiplier that scales the amount of reflected light based on the viewing angle. The more grazing the angle the more light is refleceted.
  $ F(hat(N),hat(V)) = F_0 + (1 - F_0)(1 - hat(N) dot hat(V))^(S) $
  $ F_0 = ((n_1 - n_2) / (n_1 + n_2))^2 $

=== Blinn-Phong Specular Reflection @Blinn-Phong @Acerola-BRDF
This is a simplistic, empirical model to determine the specular reflections of a material. It allows you to simulate isotropic surfaces with varying roughnesses whilst remaining very computationally efficient. The model uses "shininess" as an input parameter, whilst the standard to use roughness (due to how PBR models work). In order to account for this when wishing to increase roughness we decrease shininess.
  $ L_"specular" = (hat(N) dot hat(H))^B $
  $ hat(H) = hat(L) + hat(V) $

=== Environment Reflections @Acerola-SOS @Blinn-Phong
In order to get the color of the reflection for a given pixel, we compute the reflected vector from the normal and view vector. We then sample the corresponding point on the skybox and use that color as the reflected color.
  $ hat(R) = 2 hat(N) ( hat(N) dot hat(V)) - hat(V) $

=== GGX Distribution @CC-BRDF @Acerola-BRDF
The distribution function used in the BRDF to model the proportion of microfacet normals aligned with the halfway vector. This is an improvement over the beckmann distribution due to the graph never reaching 0 and only tapering off at the extremes.
  $ D_"GGX" = (alpha ^2) / (pi ( (alpha^2 - 1)(hat(N) dot hat(H))^2 + 1)^2) $

=== Geometric Attenuation @CC-BRDF
Used to counteract the fresnel term, mimics the phenomena of masking & shadowing presented by the microfactets. The $lambda_"GGX"$ term changes depending on the distribution function used (GGX). $hat(S)$ is either the light or view vector.
  $ G_2 = G_1 (hat(H), hat(L)) G_1 (hat(H), hat(V)) $
  $ G_1 (hat(H), hat(S)) = 1 / (1 + a lambda_"GGX") $
  $ a = (hat(H) dot hat(S)) / (alpha sqrt(1 - (hat(H) dot hat(S))^2)) $
  $ lambda_"GGX" = (-1 + sqrt( 1 + a^(-2))) / 2 $

=== Microfacet BRDF @Atlas-Water @Acerola-FFT @CC-BRDF @Acerola-BRDF
This BRDF (Bidirectional Reflectance Distribution Function) is used to determine the specular reflectance of a sample. There are many methods of doing this - the one used here is derived from microfacet theory. $D$ can be any distribution function - the geometric attenuation function $G$ changing accordingly.
$ L_"specular" = (L_"sun" F G_2 D_"GGX") / (4(hat(N) dot hat(L)) (hat(N) dot hat(V))   ) $ 

=== Reinhard Tonemapping @HDR
To account for the HDR output values of some lighting functions, we tonemap the final output to be within a 0..1 range by dividing a color $c$ as follows (given that $c$ is effectively a 3D Vector)
$ c_"final" = c / (c + [1, 1, 1]) $

=== Distance Fog & Sun @Acerola-SOS
To hide the imperfect horizon line we use a distance fog attenuated based on height and distance. This is generated by exponentially decaying a fog factor based on the relative height of the fragment compared to the ocean, then _lerping_ between the fog color and sky_color based on this. For the ocean surface we instead _lerp_ based on the distance from camera compared to a max distance, and then decaying and offsetting based on input parameters.
\
To render the sun, I compare the dot product of the ray and sun directions to the cosine of the maximum sky angle the sun can occupy and then _lerp_ between the sun and sky color based on a linear falloff factor.

#pagebreak()
== Prototyping
A prototype was made in order to test the technical stack and gain experience with graphics programming and managing shaders. I created a Halvorsen strange attractor @Halvorsen, and then did some trigonometry to create a basic camera controller using Winit's event loop.

#figure(
  image("assets/chaotic_attractor.png", width: 50%),
  caption: [
    Found at https://github.com/CmrCrabs/chaotic-attractors
  ],
)

== Project Considerations
The project will be split into 4 major stages - the simulation, implementing the IFFT, non PBR lighting, and PBR lighting. The simulation will most likely take the bulk of the project duration as implementing the spectrums, DFT and a GUI with just a graphics library is already a major undertaking. I will then implement the Blinn-Phong lighting model @Blinn-Phong in conjunction with the subsurface scattering seen in Atlas @Atlas-Water. Beyond this I will implement full PBR lighting using a microfacet BRDF and statistical distribution functions in order to simulate surface microfacets.

== Additional Features
If given enough time I would like to implement the following:
- Swell, the waves which have travelled out of their generating area @Empirical-Spectra.
- Further post processing effects, such as varying tonemapping options and a toggleable bloom pass
- A sky color simulation, as this would allow the complete simulation of a realistic day night cycle for any real world ocean condition.
- LEADR environment reflections, based on the paper by the same name (Linear Efficient Antialiased Displacement and Reflectance Mapping)

#pagebreak()
== Project Objectives (Unfinished)

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
  + Optimisations
    + dynamic render scaling stuff
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
