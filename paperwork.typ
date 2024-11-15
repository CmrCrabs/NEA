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

// jacobian & eigenvalue
// define derivatives
// how to pack 8 ffts into 4
// expand upon technologies
// post processing tonemapping
// bloom pass
// cubemap sampling
//
// cascades
// Objectives
// look into blurring for sea of thieves foam

// explain IDFT in terms of indices (keith lantz & jump trajectory)
// IFFT

// Contents Page
#page(outline(indent: true, depth: 2))



== Abstract
\/\/ synopsis
// the goal of this project was to....
= Analysis

== Client Introduction
The client is Jahleel Abraham. They are a game developer who require a physically based, performant, configurable simulation of an ocean for use in their game. They also require a physically based lighting model derived from microfacet theory, including PBR specular, and empirical subsurface scattering. Also expected is a fully featured GUI allowing direct control over every input parameter, and a functioning orbit camera.

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
  + the simulation is targeted towards mid to high end systems, ideally the solution would also be performant on lower end hardware 

#pagebreak()
== Technologies (Unfinished)
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

== Algorithm Overview (Unfinished)
// add sources per line
\/\/ like 20x more complex than this 
Note that throughout this project we are defining the positive $y$ direction as "up".
\
startup:
- generate gaussian random number pairs and store into texture on cpu
param change:
- generate energy spectrum for every wave and store into texture
- generate dispersion relation for every wave and store into texture
every frame:
- evolve spectrum
- inverse fft for all 3 axes 
- inverse fft for all 5 derivatives
- store results into textures
- displace vertices per textures
- compute jacobian of textures
- inject foam into foam texture
- lighting
- color pixels for foam
- exponential decay function on foam

#pagebreak()
== Spectrum Generation
// overview
=== Dispersion Relation @JTessendorf @Empirical-Spectra
The relation between the travel speed of the waves and their wavelength, written as a function relating angular frequency $omega$ to wave number $arrow(k)$. This simulation involves finite depth, and so we will be using a dispersion relation that considers it. This dispersion relation also considers capillary waves using an approximate relationship between surface tension and density @Empirical-Spectra.

//$ phi(k) = omega =  sqrt(g k tanh (k h)) $
//$ (d phi(k)) / (d k) = (g( tanh (h k) + h k sech^2 (h k))) / (2 sqrt(g k tanh (h k))) $
$ omega = phi(k) =  sqrt((g k + sigma / rho k^3) tanh (k h)) $
$ (d phi(k)) / (d k) = (h(sigma / rho k^3 + g k) sech^2 (h k) +  (g k + sigma / rho k^3) tanh (k h))  / (2 sqrt((g k + sigma / rho k^3) tanh (k h))) $

where
- $g$ is gravity
- $h$ is the ocean depth
- $k = |arrow(k)|$, defined with the wave summation below
- $sigma$ is the surface tension coefficient $N m^(-1)$
- $rho$ is the density $"kg" m^(-3)$

=== Non-Directional Spectrum (JONSWAP) @Empirical-Spectra @OW-Spectra @Jump-Trajectory @Acerola-FFT
The JONSWAP energy spectrum is a more parameterised version of the Pierson-Moskowitz spectrum, and an improvement over the Philips Spectrum used in @JTessendorf, simulating an ocean that is not fully developed (as recent oceanographic literature has determined this does not happen). The increase in parameters allows simulating a wider breadth of real world conditions. 
  $ S_"JONSWAP" (omega) = (alpha g^2) / (omega^5) "exp" [- beta (omega_p / omega)^4] gamma^r $
  $ r = exp [ - (omega -omega_p)^2 / (2w_p ^2 sigma ^2)] $ 
  $ alpha = 0.076 ( (U_(10) ^2) / (F g))^(0.22) $
  $ omega_p = 22( (g^2) / (U_10 F))^(1/3) $
  $ sigma = cases(
      0.07 "if" omega <= omega_p,
      0.09 "if" omega > omega_p,
    ) $
  where
  - $alpha$ is the intensity of the spectra
  - $beta = 5/4$, a "shape factor", rarely changed @OW-Spectra
  - $gamma = 3.3$
  - $omega = phi(k)$, the dispersion relation
  - $omega_p$ is the peak wave frequency
  - $U_(10)$ is the wind speed at $10"m"$ above the sea surface @OW-Spectra
  - $F$ is the distance from a lee shore (a fetch) - distance over which wind blows with constant velocity @OW-Spectra @Empirical-Spectra
  - $g$ is gravity

=== Depth Attenuation Function (Kitaiigorodskii) @Empirical-Spectra
JONSWAP was fit to observations of waves in deep water. This function adapts the JONSWAP spectrum to consider ocean depth, allowing a realistic look based on distance to shore. The actual function is quite complex for a relatively simple graph, so can be well approximated as below @Empirical-Spectra.
$ Phi (omega, h) = cases(
  1 / 2 omega_h ^2 "if" omega_h <= 1,
  1 - 1 / 2 (2 - omega_h)^2 "if" omega_h > 1,
) $
$ omega_h = omega sqrt(h / g) $
where 
- $omega = phi(k)$, the dispersion relation
- $h$ is the ocean depth
- $g$ is gravity

=== Directional Spread Function (Donelan-Banner) @Empirical-Spectra
The directional spread models how waves react to wind direction @Jump-Trajectory. This function is multiplied with the non-directional spectrum in order to produce a direction dependent spectrum @Empirical-Spectra. 

$ theta = arctan (k_z / k_x) - theta_0 $
$ D (omega, theta) = beta_s / (2 tanh (beta_s pi)) sech^2(beta_s theta) $
$ beta_s = cases( 
  2.61 (omega / omega_p)^1.3 "if" omega / omega_p < 0.95,
  2.28 (omega / omega_p)^(-1.3) "if" 0.95 <= omega / omega_p < 1.6,
  10^epsilon "if" omega / omega_p >= 1.6,
) $
$ epsilon = -0.4 + 0.8393 exp[-0.567 ln ( (omega / omega_p)^2 )] $
where
- $theta_0$ is a wind direction offset

//=== Swell (Unfinished) @Empirical-Spectra
//Swell refers to the waves which have travelled out of their generating area @Empirical-Spectra. In practice, these would be the larger waves seen over a greater area. the directional spread function including swell is based on combining donelan-banner with a swell function as below. It is worth noting that, "bar the 'magic value' of 16 seen in $s_xi$, the spectrum (and thus the simulation) is fully empirical" @Empirical-Spectra.
//
//$ D_"final" (omega, theta) = Q_"final" (omega)  D_"base" (omega, theta) D_epsilon (omega, theta) $
//$ Q_"final" (omega) = ( integral_(- pi)^(pi) D_"base" (omega, theta) D_xi (omega, theta) d theta )^(-1)  $
//$ D_xi = Q_xi (s_xi) |cos (theta / 2)|^(2 s_xi) $
//$ s_xi = 16 tanh (omega_p / omega) xi^2 $
//where
// - $xi$ is a "swell" parameter, in the range $0..1$
// - $Q_xi$ is a normalisation factor to satisfy the condition specified in equation (31) in @Empirical-Spectra


=== Directional Spectrum Function @Empirical-Spectra
The TMA spectrum below is an undirectional spectrum that considers depth, combining the above functions. 
$ S_"TMA" (omega, h) = S_"JONSWAP" (omega) Phi (omega, h) $
This takes inputs $omega, h$, whilst we need it to take input $arrow(k)$ per Tessendorf @JTessendorf - in order to do this we apply the following 'transformation'. Similarly, to make the function directional, we also need to multiply it by the directional spread function  @Empirical-Spectra.
it is worth noting that, assuming I do not implement swell, the spectrum - and thus the entire simulation - is fully empirical.
$ S_"TMA" (arrow(k)) = 2 S_"TMA" (omega, h) D (omega, theta) (d omega(|k|)) / (d |k|) 1 / (|k|) Delta arrow(k)_x Delta arrow(k)_z $
$ Delta arrow(k)_x = Delta arrow(k)_z = (2 pi) / L $
where 
- $L$ is the lengthscale defined below

#pagebreak()
== Ocean Geometry & Foam (Unfinished)
=== the Statistical Wave Summation @Code-Motion @Jump-Trajectory @Acerola-FFT @JTessendorf @Keith-Lantz
For a height field, of dimensions $L_x$ and $L_z$, we calculate the height ($h$) at a position $arrow(x)$ by summating multiple sinusoids with complex, time dependant amplitudes.  @JTessendorf.
The frequency domain representation of the waves are converted to the spatial domain using an inverse discrete fourier transform. This is split into 2 components, with the derivatives computed seperately to find exact normals / the jacobian:

  $ "Vertical Displacement (y)": h(arrow(x),t) = sum_(arrow(k)) hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Horizontal Displacement (x):" lambda D_x (arrow(x), t) = sum_arrow(k) -i arrow(k)_x / k hat(h)(arrow(k), t) e^(i arrow(k) dot arrow(x)) $
  $ "Horizontal Displacement (z):" lambda D_z (arrow(x), t) = sum_arrow(k) -i arrow(k)_z / k hat(h)(arrow(k), t) e^(i arrow(k) dot arrow(x)) $

where
- $t$ is the time
- $arrow(k) = [k_x, k_z]$, the wave vector, direction vector of the spectrum's texture
- $k = |arrow(k)|$, the magnitude of the wave vector
- $arrow(x) = [x_x,x_z]$, the direction vector for the height map for which we are summing
- $hat(h) (arrow(k), t)$ is the frequency spectrum function
- $h(arrow(x),t)$ gives the vertical displacement vector at the point $x$ at time $t$
- $arrow(D) (arrow(x),t) = [arrow(D)_x, arrow(D)_z]$ gives the horizontal displacement at $arrow(x)$ at time $t$, used to simulate "choppy waves". We would split the normalised vector $arrow(k)$ into its components and compute them seperately @JTessendorf
- $lambda$ is a convenient scale factor "choppiness" in order to create sharper wave peaks @JTessendorf

=== Derivatives (Unfinished) @JTessendorf @Jump-Trajectory
  $ "Height Derivative": (delta h(arrow(x),t))/(delta x) = sum_(arrow(k)) i arrow(k) hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Displacement Derivative (x) ?": lambda nabla D_x (arrow(x),t) = sum_(arrow(k)) arrow(k) arrow(k)_x/k hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
where
- $nabla h(arrow(x), t)$ gives the rate of change of the height, used to calculate the normal vector
- $nabla arrow(D)(arrow(x), t)$ gives the rate of change of the displacement, used to calculate the normal vector

=== Frequency Spectrum Function @JTessendorf @Jump-Trajectory @Acerola-FFT
This function defines the amplitude of the wave at a given point in space at a given time depending on it's frequency. The frequency is generated via the combination of 2 gaussian random numbers and a energy spectrum in order to simulate real world ocean variance and energies.
  $ hat(h)(arrow(k), t) = hat(h)_0(arrow(k)) e^(i phi(k)t) + h_0 (-k) e^(-i phi(k) t) $
  $ hat(h)_0(arrow(k)) = 1 / sqrt(2) (xi_r + i xi_i) sqrt( S_"TMA" (arrow(k))) $ 
where
  - $hat(h)$ evolves $hat(h)_0$ through time using eulers formula. by combining a positive and negative version of the wave number you ensure the functions output is real @JTessendorf
  - $hat(h)_0$ is the initial wave state as determined by the energy spectra & gaussian distribution. This is only computed on parameter change / startup and then stored into a texture
  - $epsilon$ are gaussian random numbers defined below
  - $S_"TMA" (arrow(k))$ is the spectrum function defined above

=== Gaussian Random Numbers @Gaussian @Empirical-Spectra
The ocean exhibits gaussian variance in the possible waves. Due to this the frequency spectrum function is varied by gaussian random numbers with mean ($tilde(x)$) 0 and standard deviation ($sigma$) 1. @JTessendorf 
\ In order to create repeatability, we create pseudorandom numbers for $x$, by by seeding a random number generator with a hash of the wave number truncated to 5 digits, preventing numerical imprecision from changing the seeding @Empirical-Spectra. These are generated in pairs and then stored into the red and green channels of a texture to be accessed.

$ 1 / sqrt(2 pi sigma ^2) e^(- ((x - tilde(x)^2))/(2 sigma^2)) $
where
- $sigma$ is the standard deviation
- $tilde(x)$ is the mean
- $x$ is a random number, $-1..1$



=== Foam, The Jacobian & Eigenvalues @JTessendorf @Acerola-FFT @Code-Motion @Empirical-Spectra
The jacobian describes the "uniqueness" of a transformation. This is useful as where the waves would crash, the jacobian determinant of the displacements goes negative. Per Tessendorf @JTessendorf, we compute the determinant of the jacobian for the horizontal displacement, $D(arrow(x), t)$.
  $ J(x) = J_"xx" J_"zz" - J_"xz" J_"zx" $
  $ J_"xx" = 1 + lambda (delta D_x (arrow(x)))/(delta x) $
  $ J_"zz" = 1 + lambda ( D_z (arrow(x)))/(delta z) $
  $ J_"xz" = J_"zx" = 1 + lambda (delta D_x (arrow(x)))/(delta z) $
we then threshold the value such that $J(x) < mu$, wherein if true we "inject" foam ($I_"injected"$) into the simulation at the given point. This value should accumulate (and decay) over time to mimic actual ocean foam, which can be achieved with the following, the result stored into a folding map texture. Finally, the folding map is multiplied by an artistic foam texture to add some detail.

$ I_"decayed" = I_0 e^(- zeta) $
$ I_"final" = I_"decayed" + I_"injected" $

where
- $I_0$ is the previous foam value
- $mu$ is a threshold value determining whether foam is injected
- $zeta$ is a constant which determines the rate of decay
- $lambda$ is the choppiness parameter

=== Level of Detail (LOD) Optimisations (Unfinished) @Code-Motion //@Crysis paper they mentioned, acerola video
\/\/ i do not want to do this
\/\/ will include frustum culling, gpu instancing & LOD scaling based on distance to camera

#pagebreak()
== The IFFT (Unfinished)
Everything in this section is subject to significant change, I am opting not to work on this now so I can begin implementation faster
=== Cooley-Tukey Fast Fourier Transform (FFT) (Unfinished) @Code-Motion @JTessendorf @Jump-Trajectory
The Cooley-Tukey FFT is a common implementation of the FFT algorithm used for fast calculation of the DFT. The direct DFT is computed in $O(N^2)$ time whilst the FFT is computed in $O(N log N)$. This is a significant improvement as we are dealing with $M$ (and $N$) in the millions.
  $ "complex, will write up after learning roots of unity & partial derivatives" $

=== The Inverse Discrete Fourier Transform (IDFT) (Unfinished) @Jump-Trajectory @Keith-Lantz @JTessendorf @Code-Motion
The IDFT can be computed using the fast fourier transform if the following conditions are met:
- $N = M = L_x = L_z$
- the coordinates & wavenumbers lie on regular grids
- $N,M,L_x,L_z = 2^x$, for any positive integer $x$
For implementation, the statistical wave summation is represented in terms of the indices $n'$ and $m'$, where $n',m'$ are of bounds $0 <= n' < N$ & $0 <= m' < M$

where
- $N,M$ are the number of points & waves respectively, the simulation resolution
- $L_x,L_z$ are the worldspace dimensions
- $arrow(k) = [(2 pi n) / L_x, (2 pi m) / L_z]$  
- $arrow(x) = [(n L_x) / N, (m L_z) / M]$

note that in Tessendorf's paper @JTessendorf, $n$ & $m$ are defined from $-N / 2 <= n < N / 2, -M / 2 <= m < M / 2$, but for ease of implemntation we shift the bounds (and all subsequent values) to begin at 0. I am thus glossing over some redundant information, further details on how / why are seen at @Jump-Trajectory @Keith-Lantz


#pagebreak()
== Post Processing
=== Rendering Equation @Atlas-Water @Acerola-FFT @Acerola-SOS
This abstract equation models how a light ray incoming to a viewer is "formed" (in the context of this simulation). Due to there only being a single light source (the sun), subsurface scattering @Atlas-Water can be used to replace the standard $L_"diffuse"$ and $L_"ambient"$ terms.

To include surface foam, we _lerp_ between the foam color and $L_"scatter"$ based on foam density @Atlas-Water. We also Increase the roughness in areas covered with foam for $L_"specular"$ @Atlas-Water.  

$ L_"eye" = (1 - F) L_"scatter" + L_"specular" + F L_"env_reflected" $

  where
  - $F$ is the fresnel reflectance term
  - $L_"scatter"$ is the light re-emitted due to subsurface scattering
  - $L_"specular"$ is the reflected light from the sun 
  - $L_"env_reflected"$ is the reflected light from the environemnt

=== Normalisation & Vector Definitions @Blinn-Phong
When computing lighting using vectors, we are only concerned with the direction of a given vector not the magnitude. In order to ensure the dot product of 2 vectors is equal to the cosine of their angle we normalise the vectors. Henceforth, a vector $arrow(A)$ when normalised is represented with $hat(A)$. Throughout all post processing effects a set of distinct vectors are used, defined as:
  - $hat(H)$ is the halfway vector
  - $hat(N)$ is the surface normal
  - $hat(V)$ is the camera view vector
  - $hat(L)$ is the light source vector

=== Surface Normals @JTessendorf @Jump-Trajectory
In order to compute the surface normals we need the derivatives of the displacement(s), the values for which are obtained from the fourier transform above.
  $ arrow(N) = vec(
  - ( (delta h) / (delta x) ) / (1 + (delta D_x) / (delta x) ), 
  1, 
  - ( (delta h) / (delta z) ) / (1 + (delta D_z) / (delta z) ), 
  ) $
note that we need to normalise this for actual usage.

=== Subsurface Scattering @Atlas-Water @Acerola-FFT
This is the phenomenon where some light absorbed by a material eventually re-exits and reaches the viewer. Modelling this realistically is impossible in a real time context (with my hardware). Specifically within the context of the ocean, we can approximate it particularly well as the majority of light is absorbed. An approximate formula taking into account geometric attenuation, a crude fresnel factor, lamberts cosine law, and an ambient light is used, alongside various artistic parameters to allow for adjustments. @Atlas-Water
  $ L_"scatter" = ((k_1 W_"max" angle.l hat(L), -hat(V) angle.r ^4 (0.5 - 0.5(hat(L) dot hat(N)))^3 + k_2 angle.l hat(V), hat(N) angle.r ^2) C_"ss" L_"sun") / (1 + lambda_"GGX") $
  $ L_"scatter" += k_3 angle.l hat(L), hat(N) angle.r C_"ss" L_"sun" + k_4 P_f C_f L_"sun" $
  where
  - $W_"max"$ is the $"max"(0, "wave height")$
  - $k_1, k_2, k_3, k_4$ are artistic parameters //explain what each param does
  - $C_"ss"$ is the water scatter color
  - $L_"sun"$ is the color of the sun
  - $C_f$ is the air bubbles color
  - $P_f$ is the density of air bubbles spread in water
  - $angle.l omega_a, omega_b angle.r$ is the $"max"(0, (omega_a dot omega_b))$
  - $lambda_"GGX"$ is the masking function defined under Smith's $G_1$
\

=== Fresnel Reflectance (Schlick's Approximation)  @Acerola-SOS @Blinn-Phong @Schlicks @Acerola-BRDF
The fresnel factor is a multiplier that scales the amount of reflected light based on the viewing angle. The more grazing the angle the more light is refleceted.
  $ F(hat(N),hat(V)) = F_0 + (1 - F_0)(1 - hat(N) dot hat(V))^5 $
  where 
  - $F_0 = ((n_1 - n_2) / (n_1 + n_2))^2$
  - $n_1$ & $n_2$ are the refractive indices of the two media @Schlicks
  - if using a microfacet model, replace $hat(N)$ with the Halfway vector, $hat(H)$) @Schlicks
\

=== Blinn-Phong Specular Reflection @Blinn-Phong @Acerola-BRDF
This is a simplistic, empirical model to determine the specular reflections of a material. It allows you to simulate isotropic surfaces with varying roughnesses whilst remaining very computationally efficient. The model uses "shininess" as an input parameter, whilst the standard to use roughness (due to how PBR models work). In order to account for this when wishing to increase roughness we decrease shininess.
  $ L_"specular" = (hat(N) dot hat(H))^S $
  $ hat(H) = hat(L) + hat(V) $
  where
  - $S$ is the shininess of the material

=== Environment Reflections @Acerola-SOS @Blinn-Phong
In order to get the color of the reflection for a given pixel, we compute the reflected vector from the normal and view vector. We then sample the corresponding point on the skybox's cubemap and use that color as the reflected color. This method is somewhat simplistic, and not physically based.
  $ hat(R) = 2 hat(N) ( hat(N) dot hat(V)) - hat(V) $
  where
  - $hat(R)$ is the normalised vector that points to the point on the cubemap which we sample

=== Microfacet BRDF @Atlas-Water @Acerola-FFT @CC-BRDF @Acerola-BRDF
This BRDF (Bidirectional Reflectance Distribution Function) is used to determine the specular reflectance of a sample. There are many methods of doing this - the one used here is derived from microfacet theory. $D$ can be any distribution function - the geometric attenuation function $G$ changing accordingly.
$ L_"specular" = L_"sun" f_"microfacet" (hat(N),hat(H),hat(L),hat(V)) = (F(hat(N),hat(H)) G(hat(L), hat(H)) D(hat(N),hat(H))) / (4(hat(N) dot hat(L)) (hat(N) dot hat(V))   ) $ 
  where
  - $L_"sun"$ is the color of the sun
  - $F(hat(N),hat(H))$ is the Fresnel Reflectance
  - $D(hat(N),hat(H))$ is the Distribution Function
  - $G(hat(L), hat(V), hat(H))$ is the Geometric Attenuation Function

=== GGX Distribution @CC-BRDF @Acerola-BRDF
The distribution function used in the BRDF to model the proportion of microfacet normals aligned with the halfway vector. This is an improvement over the beckmann distribution due to the graph never reaching 0 and only tapering off at the extremes.
  $ D_"GGX" = (alpha ^2) / (pi ( (alpha^2 - 1)(hat(N) dot hat(H))^2 + 1)^2) $
where
  - $alpha = "roughness"^2$

=== Geometric Attenuation Function (Smith's $G_1$ Function) @CC-BRDF
Used to counteract the fresnel term, mimics the phenomena of masking & shadowing presented by the microfactets. The $lambda_"GGX"$ term changes depending on the distribution function used (GGX). 
  $ G_1 = 1 / (1 + a lambda_"GGX") $
  $ a = (hat(H) dot hat(L)) / (alpha sqrt(1 - (hat(H) dot hat(L))^2)) $
  $ lambda_"GGX" = (-1 + sqrt( 1 + a^(-2))) / 2 $
where
- $alpha = "roughness"^2$

=== Distance Fog & Sun @Acerola-SOS
To hide the imperfect horizon line we use a distance fog attenuated based on height. In order to do this we use the depth buffer to determine the depth of each pixel and then based on that scale ($"lerp"$?) the light color to be closer to a defined fog color. Finally we blend a sun into the skybox based on the light position.

//=== Color Grading (Unfinished) @Acerola-SOS
//in order to really sell the sun being as bright as it would be on an open ocean, we apply a bloom pass to the whole image. In order to prevent it from being completely blown out we then apply a tone mapping to rebalance the colors. 


#pagebreak()
== Prototyping
A prototype was made in order to test the technical stack and gain experience with graphics programming and managing shaders. I created a Halvorsen strange attractor @Halvorsen, and then did some trigonometry to create a basic camera controller using Winit's event loop.

#figure(
  image("paperwork_assets/chaotic_attractor.png", width: 50%),
  caption: [
    Found at https://github.com/CmrCrabs/chaotic-attractors
  ],
)

== Project Considerations
The project will be split into 4 major stages - the simulation, implementing the IFFT, non PBR lighting, and PBR lighting. The simulation will most likely take the bulk of the project duration as implementing the spectrums, DFT and a GUI with just a graphics library is already a major undertaking. I will then implement the Blinn-Phong lighting model @Blinn-Phong in conjunction with the subsurface scattering seen in Atlas @Atlas-Water. Beyond this I will implement full PBR lighting using a microfacet BRDF and statistical distribution functions in order to simulate surface microfacets.

== Additional Features
If given enough time I would like to implement the following:
- Swell @Empirical-Spectra, the waves which have travelled out of their generating area @Empirical-Spectra.
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
