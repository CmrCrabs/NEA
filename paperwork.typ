#import "@preview/zebraw:0.4.7": *
#show table: set par(justify: false)
#show link: underline
#set raw(theme: "assets/kanagawa.tmTheme")
#set page(
  numbering: "1 of 1",
  margin: 2cm,
  paper: "us-letter",
) 
#set text(
  hyphenate: false,
)
#set heading(numbering: "1.", offset: 0)
#set text(12pt)
#set enum(numbering: "1.1", full: true)
#set list(marker: ([•], [‣],[--]))
#set math.mat(delim: "[");
#set math.vec(delim: "[");

#set table(
    fill : (x , y) => 
    if y == 0 { rgb("#F0F4F4") },
    columns: (auto, auto, auto, auto),
    align: left,
    stroke: 0.7pt,
)

#let codeblock(body) = {
  block(
    width: 100%,
    fill: rgb("#181616"),
    inset: 10pt,
    text(
      fill: rgb("#c5c9c5"),
      body,
    ),
  )
}

#let sourcecode(lang, path) = {
  zebraw(
    background-color: rgb("#181616"),
    text(
      fill: rgb("#c5c9c5"),
      size: 10pt,
      raw(lang: lang, block: true, read("./" + path))
    )
  )
}


#page(numbering: none, [
  #v(2fr)
  #align(center, [
    #text(23pt, weight: 700, [Real-Time, Empirical, Ocean Simulation & Physically Based Renderer])
    #v(0.1fr)
    #text(18pt, weight: 600, [A-Level Computer Science NEA])
    #v(0.1fr)
    #text(18pt, weight: 500, [Zayaan Azam])
    #image("assets/ocean.png")
    #v(1.1fr)
  ])
  #v(2fr)
])

#set par(justify: true)
#set page(margin: 2cm, footer: [*Centre Number:* 22147  #h(1fr) #context counter(page).display("1") #h(1fr) *Candidate Number:* 9484])

// TODO: 
// replicate old academic paper style
// https://journals.ametsoc.org/view/journals/phoc/5/3/1520-0485_1975_005_0410_optoer_2_0_co_2.xml?tab_body=pdf

// Contents Page
#page(outline(indent: true, depth: 4))

== Abstract
// TODO: SYNOPSIS
// I am developing a blah blah that doesn blah balh

\/\/ synopsis
= Analysis
== Client Introduction
The client is Jahleel Abraham. They are a game developer who require a physically based, performant, configurable simulation of an ocean for use in their game. They also require a physically based lighting model derived from microfacet theory, including PBR specular, and empirical subsurface scattering. Also expected is a fully featured GUI allowing direct control over every input parameter, and a functioning camera controller.

== Interview Questions
Interview notes are paraphrased.
#grid(
  columns: (auto, 1fr),
  gutter: 8pt,
  
  [*Q:*], [What specific ocean phenomena need to be simulated?],
  [*A:*], [The simulation should include waves in all real-world conditions and generate foam. If possible, it should also simulate other ocean phenomena.],
  
  [*Q:*], [What parameters of the simulation need to be configurable?],
  [*A:*], [All necessary parameters for simulating real-world conditions, including tile size and individual wave quantity.],
  
  [*Q:*], [Does there need to be an accompanying GUI?],
  [*A:*], [Yes, the GUI should allow control over parameters and tile size, display frametime, and show the current state of all parameters.],
  
  [*Q:*], [Do I need to implement an atmosphere/skybox?],
  [*A:*], [A basic skybox would be nice, and an atmosphere shader should be included if possible.],
  
  [*Q:*], [Do I need to implement a PBR water shader?],
  [*A:*], [Yes, the simulation should use a physically based rendering (PBR) water shader with a microfacet BRDF.],
  
  [*Q:*], [Do I need to implement caustics, reflections, or other light-related phenomena?],
  [*A:*], [Caustics are out of scope. Implement approximate subsurface scattering and use GGX distribution with the BRDF to simulate reflections.],
  
  [*Q:*], [Are there any limitations due to existing technology?],
  [*A:*], [The client has not started technical implementation, so they are not limited by an existing technical stack.],
  
  [*Q:*], [Does this need to interoperate with existing code?],
  [*A:*], [See the response to technical limitations (no existing stack constraints).],
  
  [*Q:*], [Are there limitations due to the target device(s)?],
  [*A:*], [The simulation should run on both x86 and ARM64 devices.],
  
  [*Q:*], [Are there other performance-intensive systems in place?],
  [*A:*], [See the response to technical limitations.],
  
  [*Q:*], [Is the product targeted to low/mid/high-end systems?],
  [*A:*], [The simulation is primarily targeted at mid-to-high-end systems, but ideally, it should also be performant on lower-end hardware.],
)

== Similar Solutions
As this is a fairly niche simulation that is generally used in closed source applications, there were only a few sources I could find that are relevant and have available information in relation to their implementation. Understanding existing implementations, particularly in regard to their spectrum synthesis, has allowed me to develop the correct balance between fidelity and performance.
=== Sea of Thieves
Sea of thieves is by far the highest profile game utilising an FFT ocean. It focuses on developing a stylised ocean scene over a realistic one so differs in the following ways:
- Does not have exact PBR lighting, but still derives its lighting from PBR theory
- Is intended to have minimal frametime impact while operating on low end hardware, so has a lower resolution, using a less complicated spectrum
- Has a more detailed foam simulation, where they convolve the foam map with a stylised texture, and use Unreal Engine's particle system to simulate sea spray
The implementation is closed source, so I do not have access to any detailed implementation information. Anything I could find was primarily from a conference they held on its development.
=== Acerola
Acerola's ocean simulation was made as part of a pseudo-educational youtube series on simulating water in games. It has a few notable similarities, and key benefits over others:
- It is a realistic simulation, using a spectrum that is (As far as I am aware) fully in-line with the most recent oceanographic literature
- Uses a highly performant FFT implementation, based on Nvidia WaveWorks
- Is built upon unity and utilises full PBR lighting, as derived from microfacet theory

=== Jump Trajectory
Jump trajectory created a very detailed video explaining the process of creating an FFT-based ocean simulation, also sharing the source code publically. It is similar to acerolas in most ways, but has a few notable differences:
- uses a "simpler" FFT implementation, that is much easier to understand
- Abstracts his data and algorithms into different compute passes
- Decays foam non-exponentially, using a flat base color for foam

#pagebreak()
== Success Criteria
+  The application opens with a titled, resizable, movable, and closable OS window that follows OS styling.
+  A single skybox texture can be loaded from a specified filepath.
+  The camera can be controlled via mouse input, with left-click activation, full panning, scroll-based zoom, and stable field of view across window resizes.
+  A user interface (UI) is present, featuring resizable, movable, and collapsable panels, color pickers, sliders, an FPS display, and simulation details.

+  The ocean simulation exhibits Gaussian wave variance with three controllable length scales and frequency cutoffs.
+  The simulation runs at a minimum of 60 FPS on mid-range gaming hardware (GTX 1060 and above).
+  The simulation has sensible default parameters
+  The ocean has adjustable size, including tile size, instance count, and simulation resolution.
+  Foam effects are implemented with adjustable color, decay rate, visibility, and wave-breaking injection settings.
+  The ocean conditions are user-adjustable, including depth, gravity, wind parameters, fetch, choppiness, and swell properties.

+  A skybox with a correctly transformed sun is rendered, responding correctly to camera view and zoom
+  The ocean surface has no obvious pixellation beyond that as a result of the simulation resolution
+  The ocean has approximate subsurface scattering, with adjustable scatter color, bubble effects, and lighting properties.
+  Real-time reflections from the environment are visible, influenced by user-controllable fresnel effects and refractive indices.
+  Specular reflections are implemented with toggleable PBR/non-PBR modes, adjustable shininess, and roughness settings.
+  Distance fog is implemented with user-adjustable density, offset, falloff, and height settings.

#pagebreak()
== Spectrum Synthesis
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
    [$m, n$], [Indices of summation],
    [$M, N$], [Dimensions of summation],
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
    [$kappa$], [Foam bias],
    [$mu$], [Foam injection threshold],
    [$zeta$], [Foam decay rate],
    [$I$], [Foam value],
  ),
  caption: [Simulation Variable Definitions],
) <sim-variables>

#pagebreak()
=== Dispersion Relation 
Citations: @JTessendorf @Empirical-Spectra \
The relation between the travel speed of the waves and their wavelength, written as a function relating angular frequency $omega$ to wave number $arrow(k)$. This simulation involves finite depth, and so we will be using a dispersion relation that considers it.@Empirical-Spectra

$ omega = phi(k) =  sqrt(g k tanh (k h)) $
$ (d phi(k)) / (d k) = (g( tanh (h k) + h k sech^2 (h k))) / (2 sqrt(g k tanh (h k))) $

=== Non-Directional Spectrum (JONSWAP) 
Citations: @Empirical-Spectra @OW-Spectra @Jump-Trajectory @Acerola-FFT \
The JONSWAP energy spectrum is a more parameterised version of the Pierson-Moskowitz spectrum, and an improvement over the Philips Spectrum used in @JTessendorf, simulating an ocean that is not fully developed (as recent oceanographic literature has determined this does not happen). The increase in parameters allows simulating a wider breadth of real world conditions. 
  $ S_"JONSWAP" (omega) = (alpha g^2) / (omega^5) "exp" [- 5/4 (omega_p / omega)^4] 3.3^r $
  $ r = exp [ - (omega -omega_p)^2 / (2omega_p ^2 sigma ^2)] $ 
  $ alpha = 0.076 ( (U_(10) ^2) / (F g))^(0.22) $
  $ omega_p = 22( (g^2) / (U_10 F))^(1/3) $
  $ sigma = cases(
      0.07 "if" omega <= omega_p,
      0.09 "if" omega > omega_p,
    ) $

=== Depth Attenuation Function (Approximation of Kitaiigorodskii) 
Citations: @Empirical-Spectra \
JONSWAP was fit to observations of waves in deep water. This function adapts the JONSWAP spectrum to consider ocean depth, allowing a realistic look based on distance to shore. The actual function is quite complex for a relatively simple graph, so can be well approximated as below @Empirical-Spectra. 
$ Phi (omega, h) = cases(
  1 / 2 omega_h ^2 "if" omega_h <= 1,
  1 - 1 / 2 (2 - omega_h)^2 "if" omega_h > 1,
) $
$ omega_h = omega sqrt(h / g) $

=== Directional Spread Function (Donelan-Banner) <donelan-banner>
Citations: @Empirical-Spectra \
The directional spread models how waves react to wind direction @Jump-Trajectory. This function is multiplied with the non-directional spectrum in order to produce a direction dependent spectrum @Empirical-Spectra. 

$ D (omega, theta) = beta_s / (2 tanh (beta_s pi)) sech^2(beta_s theta) $
$ beta_s = cases( 
  2.61 (omega / omega_p)^1.3 "if" omega / omega_p < 0.95,
  2.28 (omega / omega_p)^(-1.3) "if" 0.95 <= omega / omega_p < 1.6,
  10^epsilon "if" omega / omega_p >= 1.6,
) $
$ epsilon = -0.4 + 0.8393 exp[-0.567 ln ( (omega / omega_p)^2 )] $

=== Swell 
Citations: @Empirical-Spectra \
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
=== Directional Spectrum Function 
Citations: @Empirical-Spectra \
The TMA spectrum below is an undirectional spectrum that considers depth.
$ S_"TMA" (omega, h) = S_"JONSWAP" (omega) Phi (omega, h) $
This takes inputs $omega, h$, whilst we need it to take input $arrow(k)$ per Tessendorf @JTessendorf - in order to do this we apply the following transformation @Empirical-Spectra. Similarly, to make the function directional, we also need to multiply it by the directional spread function  @Empirical-Spectra.
$ S_"TMA" (arrow(k)) = 2 S_"TMA" (omega, h) (d omega) / (d k) 1 / k Delta k_x Delta k_z D (omega, theta) $
$ Delta k_x = Delta k_z = (2 pi) / L $

#pagebreak()
== Ocean Geometry & Foam
=== Displacements 
Citations:  @Jump-Trajectory @Acerola-FFT @JTessendorf @Keith-Lantz \
For a field of dimensions $L_x$ and $L_z$, we calculate the displacements at positions $arrow(x)$ by summating multiple sinusoids with complex, time dependant amplitudes.  @JTessendorf. By arranging the equations into a specific format, we can convert the frequency domain representation of the wave into the spatial domain using the inverse discrete fourier transform.
  $ "Vertical Displacement (y)": h(arrow(x),t) = sum_(arrow(k)) hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Horizontal Displacement (x):" lambda D_x (arrow(x), t) = sum_arrow(k) -i arrow(k)_x / k hat(h)(arrow(k), t) e^(i arrow(k) dot arrow(x)) $
  $ "Horizontal Displacement (z):" lambda D_z (arrow(x), t) = sum_arrow(k) -i arrow(k)_z / k hat(h)(arrow(k), t) e^(i arrow(k) dot arrow(x)) $

=== Derivatives 
Citations: @JTessendorf @Jump-Trajectory \
For lighting calculations and the computation of the jacobian, we require the derivatives of the above displacements. As the derivative of a sum is equal to the sum of the derivatives, we compute exact derivatives using the following summations.
  $ "X Component of Normal": epsilon_x (arrow(x), t) = sum_(arrow(k)) i k_x hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Z Component of Normal": epsilon_z (arrow(x), t) = sum_(arrow(k)) i k_z hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $

  $ "Jacobian xx": (d D_x) / (d x) = sum_(arrow(k)) -k_x^2 /k hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Jacobian zz": (d D_x) / (d x) = sum_(arrow(k)) -k_z^2 /k hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $
  $ "Jacobian xz": (d D_x) / (d z) = sum_(arrow(k)) -(k_x k_z) /k hat(h) (arrow(k), t) e ^ (i arrow(k) dot arrow(x)) $

=== Frequency Spectrum Function <spectrum_eq>
Citations: @JTessendorf @Jump-Trajectory @Acerola-FFT \
This function defines the amplitude of the wave at a given point in space at a given time depending on it's frequency. The frequency is generated via the combination of 2 gaussian random numbers and a energy spectrum in order to simulate real world ocean variance and energies. Note that the time dependant component of the exponent is pushed into this amplitude, in order to simplify the summation.
  $ hat(h)_0(arrow(k)) = 1 / sqrt(2) (xi_r + i xi_i) sqrt( S_"TMA" (arrow(k))) $ 
  $ hat(h)(arrow(k), t) = hat(h)_0(arrow(k)) e^(i phi(k)t) + h_0 (-k) e^(-i phi(k) t) $

=== Box-Muller Transform 
Citations: @Gaussian \
The ocean exhibits gaussian variance in the possible waves. Due to this the frequency spectrum function is varied by gaussian random numbers with mean 0 and standard deviation 1, which we generate using the box-muller transform converting from uniform variates from 0..1. @JTessendorf. Derivation is from polar coordinates, by treating x and y as cartesian coordinates, more details at @Gaussian
$ xi_r, xi_i ~ N(0, 1) $
$ xi_r = sqrt(-2.0 ln (u_1)) cos (2 pi u_2) $
$ xi_i = sqrt(-2.0 ln (u_1)) sin (2 pi u_2) $

=== Foam, The Jacobian, and Decay 
Citations: @JTessendorf @Acerola-FFT  @Empirical-Spectra \
The jacobian describes the "uniqueness" of a transformation. This is useful as where the waves would crash, the jacobian determinant of the displacements goes negative. Per Tessendorf @JTessendorf, we compute the determinant of the jacobian for the horizontal displacement, $D(arrow(x), t)$.
  $ J(arrow(x)) = J_"xx" J_"zz" - J_"xz" J_"zx" $
  $ J_"xx" = 1 + lambda (d D_x)/(d x) , J_"zz" = 1 + lambda (d D_z)/(d z) $
  $ J_"xz" = J_"zx" = 1 + lambda (d D_x)/(d z) $
we then threshold the value such that $J(arrow(x)) < mu$, wherein if true we inject foam into the simulation at the given point. This value should accumulate (and decay) over time to mimic actual ocean foam, which is achieved by modulating the previous foam value ($I_0$) by an exponential decay function:

$ J_"biased" = kappa - J(arrow(x)) $
$ I = cases(
  I_0 e^(-zeta) + J_"biased" "if" J_"biased" > mu,
  I_0 e^(-zeta) "if" J_"biased" < mu,
) $

#pagebreak()
=== Cascades 
Citations: @Jump-Trajectory @JTessendorf @Empirical-Spectra \
The periodicity of sinusoidal waves leads to visible tiling, even with an amount of waves of order $10^6$. It is possible to increase the simulation resolution to counteract this, but even the FFT becomes computationally impractical at large enough scales. To counteract this, we instead compute the entire simulation multiples times with different lengthscales at a lower resolution - combining the results such that there is no longer any visual tiling. This results in an output with comparable quality to an increase in resolution, while requiring less overall calculations, e.g $3(256^2) < 512^2$. To do this, we have to select low and high cutoffs for each lengthscale such that the waves do not overlap and superposition.

=== 2D GPGPU Cooley-Tukey Radix-2 Inverse Fast Fourier Transform <ifft_exp>
Citations: @JTessendorf @Jump-Trajectory \
The Cooley-Tukey FFT is a common implementation of the FFT, used for fast calculation of the DFT. The direct DFT is computed in $O(n^2)$ time whilst the FFT is computed in $O(n log n)$. This is a significant improvement as we are dealing with $n$ in the order of $10^4 - 10^6$, computed multiple times per frame. The FFT exploits the redundancy in DFT computation in order to increase performance, being able to do so only when $L_x = L_z = M = N = 2^x, x in ZZ$, the coordinates and wave vectors lie on a regular grid, and the summation is in the following format, which are all true save some differences in summation limits.
$ "Inverse DFT:" F_n = sum_(m=0)^(N - 1) f_m e^(2 pi i m/N n ) $
$ "Wave Summation:" h (arrow(x), t) = sum_(m=-N/2)^(N / 2) h (t) e^(2 pi i m/N n ) $

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

=== Rendering Equation 
Citations: @Atlas-Water @Acerola-FFT @Acerola-SOS \
This abstract equation models how a light ray incoming to a viewer is "formed" (in the context of this simulation). Due to there only being a single light source (the sun), subsurface scattering @Atlas-Water can be used to replace the standard $L_"diffuse"$ and $L_"ambient"$ terms.

To include surface foam, we _lerp_ between the foam color and $L_"eye"$ based on foam density @Atlas-Water. We also Increase the roughness in areas covered with foam for $L_"specular"$ @Atlas-Water.  

$ L_"eye" = (1 - F) L_"scatter" + L_"specular" + F L_"env_reflected" $

=== Normalisation  & Surface Normals 
Citations: @Blinn-Phong @JTessendorf @Jump-Trajectory \
When computing lighting using vectors, we are only concerned with the direction of a given vector not the magnitude. In order to ensure the dot product of 2 vectors is equal to the cosine of their angle we normalise the vectors.
\

In order to compute the surface normals we sum, for each lengthscale, the value of $epsilon (arrow(x), t)$ such that the following is true, which is then normalised.
  $ arrow(N) = vec(
  -epsilon_x (arrow(x), t),
  1, 
  -epsilon_z (arrow(x), t),
  ) $

=== Subsurface Scattering
Citations: @Atlas-Water @Acerola-FFT \
This is the phenomenon where some light absorbed by a material eventually re-exits and reaches the viewer. Modelling this realistically is impossible in a real time context (with my hardware). Specifically within the context of the ocean, we can approximate it particularly well as the majority of light is absorbed. An approximate formula taking into account geometric attenuation, a crude fresnel factor, lamberts cosine law, and an ambient light is used, alongside various artistic parameters to allow for adjustments. @Atlas-Water
  $ L_"scatter" = ((k_1 W_"max" angle.l hat(L), -hat(V) angle.r ^4 (0.5 - 0.5(hat(L) dot hat(N)))^3 + k_2 angle.l hat(V), hat(N) angle.r ^2) C_"ss" L_"sun") / (1 + lambda_"GGX") $
  $ L_"scatter" += k_3 angle.l hat(L), hat(N) angle.r C_"ss" L_"sun" + k_4 P_f C_f L_"sun" $

=== Fresnel Reflectance (Schlick's Approximation)  @Acerola-SOS @Blinn-Phong @Schlicks 
The fresnel factor is a multiplier that scales the amount of reflected light based on the viewing angle. The more grazing the angle the more light is refleceted.
  $ F(hat(N),hat(V)) = F_0 + (1 - F_0)(1 - hat(N) dot hat(V))^(S) $
  $ F_0 = ((n_1 - n_2) / (n_1 + n_2))^2 $

=== Blinn-Phong Specular Reflection 
Citations: @Blinn-Phong  \
This is a simplistic, empirical model to determine the specular reflections of a material. It allows you to simulate isotropic surfaces with varying roughnesses whilst remaining very computationally efficient. The model uses "shininess" as an input parameter, whilst the standard to use roughness (due to how PBR models work). In order to account for this when wishing to increase roughness we decrease shininess.
  $ L_"specular" = (hat(N) dot hat(H))^B $
  $ hat(H) = hat(L) + hat(V) $

=== Environment Reflections 
Citations: @Acerola-SOS @Blinn-Phong \
In order to get the color of the reflection for a given pixel, we compute the reflected vector from the normal and view vector. We then sample the corresponding point on the skybox and use that color as the reflected color.
  $ hat(R) = 2 hat(N) ( hat(N) dot hat(V)) - hat(V) $

=== GGX Distribution 
Citations: @CC-BRDF  \
The distribution function used in the BRDF to model the proportion of microfacet normals aligned with the halfway vector. This is an improvement over the beckmann distribution due to the graph never reaching 0 and only tapering off at the extremes.
  $ D_"GGX" = (alpha ^2) / (pi ( (alpha^2 - 1)(hat(N) dot hat(H))^2 + 1)^2) $

=== Geometric Attenuation 
Citations: @CC-BRDF \
Used to counteract the fresnel term, mimics the phenomena of masking & shadowing presented by the microfactets. The $lambda_"GGX"$ term changes depending on the distribution function used (GGX). $hat(S)$ is either the light or view vector.
  $ G_2 = G_1 (hat(H), hat(L)) G_1 (hat(H), hat(V)) $
  $ G_1 (hat(H), hat(S)) = 1 / (1 + a lambda_"GGX") $
  $ a = (hat(H) dot hat(S)) / (alpha sqrt(1 - (hat(H) dot hat(S))^2)) $
  $ lambda_"GGX" = (-1 + sqrt( 1 + a^(-2))) / 2 $

=== Microfacet BRDF 
Citations: @Atlas-Water @Acerola-FFT @CC-BRDF  \
This BRDF (Bidirectional Reflectance Distribution Function) is used to determine the specular reflectance of a sample. There are many methods of doing this - the one used here is derived from microfacet theory. $D$ can be any distribution function - the geometric attenuation function $G$ changing accordingly.
$ L_"specular" = (L_"sun" F G_2 D_"GGX") / (4(hat(N) dot hat(L)) (hat(N) dot hat(V))   ) $ 

=== Reinhard Tonemapping
Citations: @HDR \
To account for the HDR output values of some lighting functions, we tonemap the final output to be within a 0..1 range by dividing a color $c$ as follows (given that $c$ is effectively a 3D Vector)
$ c_"final" = c / (c + [1, 1, 1]) $

=== Distance Fog & Sun
Citations: @Acerola-SOS \
To hide the imperfect horizon line we use a distance fog attenuated based on height and distance. This is generated by exponentially decaying a fog factor based on the relative height of the fragment compared to the ocean, then _lerping_ between the fog color and sky_color based on this. For the ocean surface we instead _lerp_ based on the distance from camera compared to a max distance, and then decaying and offsetting based on input parameters.
\
To render the sun, I compare the dot product of the ray and sun directions to the cosine of the maximum sky angle the sun can occupy and then _lerp_ between the sun and sky color based on a linear falloff factor.

#pagebreak()
== Prototyping
A prototype was made in order to test the technical stack and gain experience with graphics programming and managing shaders. I created a Halvorsen strange attractor @Halvorsen, using differential equations, and then did some trigonometry to create a basic camera controller using Winit's event loop.

#figure(
  image("assets/chaotic_attractor.png", width: 50%),
  caption: [
    Shader Output, Found at https://github.com/CmrCrabs/chaotic-attractors
  ],
)

== Additional Features
If given enough time I would like to implement the following:
- Swell, the waves which have travelled out of their generating area @Empirical-Spectra.
- Further post processing effects, such as varying tonemapping options and a toggleable bloom pass
- A sky color simulation, as this would allow the complete simulation of a realistic day night cycle for any real world ocean condition.
- LEADR environment reflections, based on the paper by the same name (Linear Efficient Antialiased Displacement and Reflectance Mapping)


#pagebreak()
== Project Objectives

=== Engine
+ there is an os window created on startup
  + the window has a appropriate title
  + the window follows the OS's conventions and compositor styling
  + the window can be resized without breaking the simulation
  + the window can be moved
  + the window can move between monitors without breaking
  + the window can be closed by pressing the escape key
+ the scene has a single mesh stored on the cpu
  + the user can resize the mesh
  + there are multiple instances of the mesh visible
  + the user can control how many instances there are
+ a filepath can be specified such that a single skybox .exr texture is read into GPU memory
+ the user can control the camera
  + the camera can only be controlled when left-click is held down
  + the camera's pitch can be controlled by moving the mouse
  + the camera's yaw can be controlled by moving the mouse
  + the camera's zoom can be controlled by using the scroll wheel
  + the camera's render distance is large enough to see the entire ocean
  + the cameras field of view does not change upon window resize
  + the cameras view direction does not change upon window resize
+ the user can only control the camera if the UI is not selected
+ the user can access a user interface
  + the user can resize the UI
  + the user can move the UI
  + the user can collapse the UI
  + the user can edit colors using the UI
  + the user can control sliders using the UI
  + the user can read the fps from the UI
  + the user can get the simulation resolution from the UI
  + the user can see the time elapsed from the UI


=== Simulation
+ The simulation exhibits gaussian variance in the possible waves
+ The simulation is performant, being able to run at above 60fps on mid-range gaming hardware (gtx 1060+)
+ the simulation should have sensible default parameters
+ the simulations input parameters should be clamped such that changing parameters cannot permanently break the simulation
+ the simulation should not have visible tiling
+ the simulation should have 3 lengthscales, with user control over
  + all 3 lengthscales
  + all 3 low frequency cutoffs
  + all 3 high frequency cutoffs
+ the simulation should have a controllable size
  + user adjustable tile size
  + user adjustable instance quantity
  + user adjustable simulation resolution
+ the simulation should support foam, with the following controllable conditions
  + the foam color
  + how fast the foam decays
  + how much foam is visible
  + when foam is injected on breaking waves
  + how much foam is injected on breaking waves
+ the user should be able to control the ocean conditions
  + user adjustable ocean depth
  + user adjustable gravitational field strength
  + user adjustable wind speed
  + user adjustable wind angle
  + user adjustable fetch
  + user adjustable choppiness
  + user adjustable amount of swell
  + user adjustable integration step for swell computation
  + user adjustable height offset for the ocean surface

=== Renderer
+ the scene has a skybox
  + the skybox is correctly transformed when the camera view direction is changed
  + the skybox is correctly transformed when the camera zoom is changed
  + the skybox is correctly transformed when the window is resized
  + there is a sun interpolated into the skybox
    + the sun is in the correct position in the sky relative to the light vector
    + the sun is correctly transformed when the camera view direction is changed
    + the sun is correctly transformed when the camera zoom is changed
    + the sun is correctly transformed when the window is resized
    + the sun has user controllable parameters
      + user adjustable sun color
      + user adjustable sun x direction
      + user adjustable sun y direction
      + user adjustable sun z direction
      + user adjustable sun angle
      + user adjustable sun distance
      + user adjustable sun size
      + user adjustable sun falloff factor
+ displacement map sampled such that there is no visible pixelation in output
+ normal map sampled such that there is no visible pixelation in output
+ foam map sampled such that there is no visible pixelation in output
+ there is visible, user controllable, subsurface scattering for ocean base color
  + user adjustable scatter color
  + user adjustable bubble color
  + user adjustable bubble density
  + user adjustable height attenuation factor
  + user adjustable reflection strength
  + user adjustable diffuse lighting strength
  + user adjustable ambient lighting strength
+ there is visible reflections from the environment
  + user adjustable reflection strength
  + there is visible fresnel reflectance affecting reflections
    + user adjustable water refractive index
    + user adjustable air refractive index
    + user adjustable fresnel shine
    + user adjustable fresnel effect scale factor
    + user adjustable fresnel normals scale factor
+ there is non physically based specular reflections
  + user toggleable pbr / non pbr specular
  + user adjustable shininess of surface
+ there is user controllable physically based specular reflections
  + user adjustable specular scale factor
  + user adjustable pbr fresnel scale factor
  + user adjustable pbr cutoff low
  + user adjustable water roughness
  + user adjustable foam roughness factor
+ there is user controllable distance fog
  + user adjustable fog density
  + user adjustable fog distance offset
  + user adjustable fog falloff factor
  + user adjustable fog height offset

#pagebreak()
= Documented Design
== Technologies
#table(
  columns: 4,
  align: left,
  [*Library*], [*Version*], [*Purpose*], [*Link*],
  [Rust], [-], [Fast, memory-efficient programming language], [https://www.rust-lang.org/],
  [wgpu], [23.0.1], [Graphics library], [https://github.com/gfx-rs/wgpu],
  [Rust GPU], [git], [(Rust as a) shader language], [https://github.com/Rust-GPU/rust-gpu],
  [winit], [0.29], [Cross-platform window creation and event loop management], [https://github.com/rust-windowing/winit],
  [Dear ImGui], [0.12.0], [Bloat-free GUI library with minimal dependencies], [https://github.com/imgui-rs/imgui-rs],
  [imgui-wgpu-rs], [0.24.0], [only rendering code used, snippets taken directly from source instead of library being imported], [https://github.com/yatekii/imgui-wgpu-rs],
  [imgui-winit-support], [0.13.0], [only datatype translation code used, snippet taken directly from source instead of library being imported], [https://github.com/imgui-rs/imgui-winit-support],
  [GLAM], [0.29], [Linear algebra operations], [https://github.com/bitshifter/glam-rs],
  [Pollster], [0.3], [Used to read errors (async executor for wgpu)], [https://github.com/zesterer/pollster],
  [Env Logger], [0.10], [Logging for debugging and error tracing], [https://github.com/env-logger-rs/env_logger],
  [Image], [0.24], [Used to read a HDRI from a file into memory], [https://github.com/image-rs/image],
  [Nix], [-], [Creating a declarative, reproducible development environment], [https://nixos.org/],
)

// TODO LINK TO UI.RS // TODO: properly credit and comment

#pagebreak()
== Core Algorithm @JTessendorf @Empirical-Spectra 
Below is a high-level explanation of the core algorithm for the simulation. A visual representation can be seen on the next page.
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

#page(
  flipped: true,
  figure(
    align(
      center,
      image("assets/core_algorithm.png", fit: "contain", width: 100%),
    ),
    caption: [
      Visualisation of the core algorithm. Texture white / black points are not consistent for ease of demonstration
    ]
  )
)

#pagebreak()
== The Fourier Transform
Citations: @GPGPU @Biebras @Jump-Trajectory @Acerola-FFT \
The IFFT algorithm I am using consists of 3 phases. The first phase involves precomputing the butterfly texture that holds the twiddle factors and input indices on the GPU. The second phase executes horizontal IFFT operations on the GPU, alternating between reading and writing to/from the input texture and a pingpong texture based on the pingpong push constant. The Third phase performs vertical IFFT operations on the same 2 textures. 
\
Note that the "id" referenced in any pseudocode corresponds to the workgroup global invocation id, which is analogous in this context to the pixel coordinate of the texture we are operating over.

#codeblock(
```rust
let pingpong = 0;
for i in 0..log_2(size) {
  dispatch_horizontal_IFFT_shader(i);
  pingpong = (pingpong + 1) % 2;
}
for i in 0..log_2(size) {
  dispatch_vertical_IFFT_shader(i);
  pingpong = (pingpong + 1) % 2;
}
dispatch_permute_shader();
```
)
#figure(
    align(
      center,
      image("assets/fft_alg.png", fit: "contain", height: 50%),
    ),
    caption: [
      Visualised IFFT Algorithm showing the horizontal and vertical operations on an input texture
    ]
  ) <IFFT>
#pagebreak()

=== Butterfly Texture
Citations: @GPGPU \
For the following butterfly operations, input twiddle factors and indices are required. Following @GPGPU, we generate a texture with width $log_2 N$ and height $N$, storing the real and complex parts of the twiddle factor into the r and g channels of the texture. We then store the input x/y indices into the b and a channels.
\ 
the twiddle factor at n is a complex number $W_n^k$ such that 
$ k = y_"current" n / 2^("stage" + 1) "mod" n $
$ W_n^k = exp((- 2 pi i k) / n) $ 
which we represent in code using eulers formula as 
$ W_n^k = cos((- 2 pi i k) / n) + i sin((-2 pi i k) / n) $ 
for the input indices, we compute a $y_t$ and $y_b$, being the indices of the top and bottom wing respectively. for the first stage we sort our data in bit reversed order, meaning if we are in the first column of the texture, we should perform a bit reverse on the resulting indices.
\
We determine whether we are operating in the upper or lower "wing" based on the following equation, where we map 1 to be true and 0 to be false.
$ "wing" = y_"current" mod 2^("stage" + 1) $

#codeblock(
```rust
let yt = id.y;
let yb = id.y;
if id.x == 0 {
  if wing {
      yb += 1;
  } else {
      yt -= 1;
  }
  yt.bit_reverse();
  yb.bit_reverse();
} else {
  if wing {
      yb += step as u32;
  } else {
      yt -= step as u32;
  }
}
```
)
#pagebreak()
=== Butterfly Operations
Citations: @GPGPU @Biebras \
The FFT algorithm is designed to operate over 1D data, however our input is a 2D texture. To amend this, we perform a 1D IFFT on each row horizontally, and each column vertically, as shown in @IFFT. As our textures have 4 channels that can store 2 complex numbers each, we perform the butterfly operation twice per invocation, effectively cutting the needed IFFTs in half.

#figure(
codeblock(
```rust
let twiddle_factor = butterfly_texture.read(stage, id.x).rg();
let indices = butterfly_texture.read(stage, id.x).ba();
let top_signal = pingpong0.read(indices.x, id.y);
let bottom_signal = pingpong0.read(indices.y, id.y);
output = top_signal + complex_mult(twiddle_factor, bottom_signal);
```
),
    caption: [
      Horizontal Butterfly Operation, over two sets of inputs
    ]
)

#figure(
codeblock(
```rust
let twiddle_factor = butterfly_texture.read(stage, id.y).xy();
let indices = butterfly_texture.read(stage, id.y).zw();
let top_signal = pingpong0.read(id.x, indices.x);
let bottom_signal = pingpong0.read(id.x, indices.y);
output = top_signal + complex_mult(twiddle_factor, bottom_signal);
```
),
    caption: [
      Vertical Butterfly Operation, over two sets of inputs
    ]
)

=== Permutation
Citations: @GPGPU \
Our data needs to be permuted, as per @ifft_exp our summation limits are offset compared to those used by the IFFT algorithm. The offset causes our data to flip signs in a gridlike pattern, as is visible in @IFFT. 

#codeblock(
```rust
if (id.x+ id.y) % 2 == 0.0 {
  sign = 1.0;
} else {
  sign = -1.0;
}
output = sign * pingpong0.read(id.x, id.y);
```
)

#pagebreak()
=== FFT Stage Visualisation
#figure(
    align(
      center,
      image("assets/fft_steps.png", fit: "contain", height: 90%),
    ),
    caption: [
      Visualised FFT Steps, black / white points are not consistent for demonstrative purposes. 
    ]
  )


#pagebreak()
== Event Loop Control Flow
#figure(
    align(
      center,
      image("assets/control_flow.png", fit: "contain", height: 90%),
    ),
    caption: [
      Abstracted event loop control flow flowchart
    ]
  )
=== UI Event Flow
#figure(
    align(
      center,
      image("assets/ui_flow.png", fit: "contain", height: 90%),
    ),
    caption: [
      UI Event Handling Flowchart
    ]
  )
=== Camera Controller Flow
#figure(
    align(
      center,
      image("assets/camera_flow.png", fit: "contain", height: 60%),
    ),
    caption: [
      UI Event Handling Flowchart
    ]
  )
#pagebreak()
== Other Algorithms
=== Cascades
Citations: @Jump-Trajectory @Biebras \
To prevent any overlap and wasted computation, we choose lengthscales and cutoffs such that we only have to sample the spectrum at a given lengthscale only once. We select a larger lengthscale for bigger waves, and smaller lengthscale for smaller waves. For a visually pleasing ocean, we follow 3 key principles:
1. Avoid smaller waves in larger cascades as they have a course spatial resolution
2. Avoid larger waves in small cascades, as they will cause visible tiling
3. Cutoffs should be chosen such that there are no large gaps in spectrum coverage
Given this, we arrive at the following base cascade setup, altho there is room to change any of the parameters depending on the desired outcome.
#figure(
    align(
      center,
      image("assets/cascades.png", fit: "contain", height: 50%),
    ),
    caption: [
      Graphical approximation of the spectrum for demonstrative purposes
    ]
  )
#pagebreak()
=== Index Buffer
To enable proper backface culling, the vertices of the mesh are connected in a specific (counterclockwise) ordering such that the GPU can discern whether the face is pointed towards the camera. The GPU decides the ordering based on an index buffer, which specifies the index of which vertex is to be connected. The algorithm below only works for a square mesh, and was designed by me so is more than likely innefficient.
#codeblock(
```rust
fn square_mesh_indices(size: u32) -> Vec<u32> {
  let mut indices: Vec<u32> = vec![];
  for y in 0..size - 1 {
      for x in 0..size - 1 {
          indices.push(x + y * size);
          indices.push((x + 1) + (y + 1) * size);
          indices.push(x + (y + 1) * size);
          indices.push(x + y * size);
          indices.push((x + 1) + y * size);
          indices.push((x + 1) + (y + 1) * size);
      }
  }
  indices
}
```
)

== The Skybox & Equirectangular Projection 
Citations: @Fullscreen-Tri \
A standard skybox is rendered using a cubemap, which consists of 6 square textures that are projected onto a cube that is placed surrounding the scene. An alternative method is to use an equirectangular skybox, which is where a single 2D texture is sampled using a 3D vector which is transformed using polar coordinates. Ordinarily, cubemaps are chosen because they can be sampled directly, saving computation, however given that I would have to load the cubemap, pass it to the GPU and manually render the actual sky cube, the performance overhead is worth the significant ease of implementation. 
\
\
Instead, we send a draw call of only 3 vertices to the GPU, which we then map to create a triangle large enough to cover the screen @Fullscreen-Tri, which all skybox details are rendererd to. Below we offset the vertices based on their index to form the triangle in the $[0, 1]$ space, and then convert it to clip space $[-1, 1]$.
#codeblock(
```rust
// Vertex Shader
let out_uv1 = Vec2::new(
    ((vertex_index << 1) & 2) as f32,
    (vertex_index & 2) as f32,
);
*out_pos = Vec4::new(out_uv1.x * 2.0 - 1.0,out_uv1.y * 2.0 - 1.0, 0.0, 1.0);
```
)

in the fragment shader, we take the 2D fragment coord and convert it to the same $[-1, 1]$ clip space, such that we can then inverse the camera view and projection affine transforms, converting the screen space 2 dimensional coordinate to a 3 dimensional world space coordinate. 
#codeblock(
```rust
// Fragment Shader
let uv = Vec2::new(
    frag_coord.x / consts.width * 2.0 - 1.0,
    1.0 - frag_coord.y / consts.height * 2.0,
);

let target = proj_inverse * Vec4::new(uv.x, uv.y, 1.0, 1.0);
let view_pos = (target.truncate() / target.w).extend(1.0);
let world_pos = view_inverse * view_pos;
let ray_dir = world_pos.truncate().normalize();
```
)
The 3D normalised direction vector is then used to sample the equirectangular HDRI texture, with the resulting value used to color the skybox. 
#codeblock(
```rust
// 3D direction vector -> 2D vector for texture reading
fn equirectangular_to_uv(v: Vec3) -> Vec2 {
    Vec2::new(
        (v.z.atan2(v.x) + consts::PI) / consts::TAU,
        v.y.acos() / consts::PI,
    )
}
```
)

== Spectrum Conjugates
Citations: @Biebras @JTessendorf @Jump-Trajectory \
From Tessendorf @JTessendorf, we need to compute the spectrum and its conjugate in order to ensure a real output per @spectrum_eq. "The symmetry of the fourier series allows us to mirror the amplitudes,eliminiating the need for complex conjugate recalculation" @Biebras. Below is pseudocode of the concept from @Biebras / @Jump-Trajectory, where $N$ corresponds to the simulation resolution.
#codeblock(
```rust
// The Computed Spectrum
let h0 = spectrum_tex.read(id.x, id.y);
// The Conjugate Spectrum
let h0c = spectrum_tex.read(
  (N - id.x) % N,
  (N - id.y) % N,
);
```
)

#pagebreak()
== Xoshiro256plus Pseudorandom Number Generator
Citations: @xoshiro \
In order to have consistency between different instances of the simulation, we seed the gaussian numbers such that there is reproducability. To do so, I have chosen the Xoshiro256plus PRNG as it operates in O(1) in both space & time complexity, runs in nanoseconds and is relatively simple to implement. Below is pseudocode showcasing the method.
#codeblock(
```rust
  fn next(&mut self) -> u64 {
      let result = self.seed[0].wrapping_add(self.seed[3]);
      let t = self.s[1] << 17;

      self.seed[2] ^= self.seed[0];
      self.seed[3] ^= self.seed[1];
      self.seed[1] ^= self.seed[2];
      self.seed[0] ^= self.seed[3];

      self.seed[2] ^= t;
      self.seed[3] = rol64(self.seed[3], 45);

      result
  }
```
)

#page(
  flipped: true,
  figure(
    [
      == Code Structure
      #image("assets/data_structure.png", fit: "contain", width: 100%),
    ],
    caption: [
      Projects Data structure, in terms of "key" data types - structs, pipelines & textures
    ]
  )
)
#set par(justify: false)
#pagebreak()
= Technical Solution
#page(
  flipped: true,
  [
== Skills Demonstrated
I believe the GPU / CPU can be seen as an equivalent model to Client / Server. This is because they follow similar key concepts, being:
  - Task Delegation, the CPU sends tasks to the GPU in a similar way to server processing requests from clients
  - Concurrency / Parallellism, executing and managing data from many GPU threads in parallel is similar to a server handling multiple clients, as both must handle resource allocation and data synchronization
  - Communication Overhead, there are similar data transfer bottlenecks for both client/server & gpu/cpu, as I have had to consider data alignment, padding and typing when passing data to the gpu, as well as synchronizing the updating of data between passes and individual invocations
  - Synchronization, I have to manage updating data throughout an individual invocation through push constants in a similar manner to how a server would have to update data on a client
#table(
  columns: (auto, auto, auto, auto),
  inset: 10pt,
  align: left,
  stroke: 0.7pt,
  [*Group*], [*Skill*], [*Description / Reasoning*], [*Evidence*],
  [A], [Complex Scientific Model], [Entire Spectrum Synthesis], [`shaders/sim/initial_spectra.rs` entire file],
  [A], [Complex Mathematical Model], [Implementation of a PBR Lighting Model], [`shaders/lib.rs` entire file],
  [A], [Complex Mathematical Model], [The computing, storing and processing of a large amount of complex numbers], [`shaders/evolve_spectra.rs`, `shaders/process_deltas.rs` entire file(s)],
  [A], [Complex Control Model], [The entire applications event loop and resultitng control flow], [`engine/mod.rs` line 92 onwards],
  [A], [List Operations], [Generation of gaussian texture data, generation of index buffer, manipulation of indices for use in fft], [`shaders/fft.rs`, `sim/simdata.rs`, `engine/scene.rs`],
  [A], [Hashing], [Xoshiro256++ PRNG Implementation], [`sim/simdata.rs` line 83 onwards],
  [A], [Advanced Matrix Operations], [Various screen / world space transformations, reversing affine transformations], [`shaders/skybox.rs` fragment shader, `shaders/lib.rs` vertex shader],
  [A], [Recursive Algorithms], [the 2D GPGPU IFFT I have manually recurses, due to it being on the GPU], [`sim/fft.rs` `66-120`, should be clearer after seeing `shaders/fft.rs`],
  [A], [Complex User Defined Algorithms], [Computation of the index buffer for the mesh drawing], [`engine/scene.rs` lines `181-191` ],
  [A], [Complex OOP model], [Program is based around different objects and classes, objects are generated / regenerated based on user input, designed with composition and inheritence in mind], [most demonstrated in `engine/mod.rs` entire file],
  [A], [Complex client-server model], [explained in preamble], [`engine/mod.rs` `run()` function, `sim/compute.rs`, `engine/renderer.rs`, `sim/fft.rs`, `sim/mod.rs` whole file(s)],
  [B], [Multi-Dimensional Arrays], [Usage of textures throughout entire project (analagous to 2D arrays), managing data between FFT passes], [`shaders/*`],
  [B], [Simple Mathematical Model], [Box-Muller Transform, gaussian random number generation], [`sim/simdata.rs` line `62` function(s)]
)
]
)

== Completeness of Solution
Everything specified in objectives has been completed, the performance has not been tested on a gtx 1060 level GPU, but given that it is performant at ~40-50fps on an Intel Iris Xe integrated graphics card at 256x256x3, it will be easily performant on a discrete GPU. At a more reasonable (given the system) resolution of 128x128x3, the simulation easily reaches 90+fps, while still looking reasonably good. From the additional features, swell has been included in the spectrum synthesis.

== Coding Style
I have followed the rust programming conventions and principles for this project, using the result type for error handling where possible. As the vast majority of possible errors come from wgpu/rust-gpu, errors are in majority handled via the pollster and env_logger crates, where any errors comign from wgpu / rust-gpu are passed up the chain and outputted from the standard log using env_logger. Shader code is written following "standard" shader conventions, with notable stylings being an emphasis on defining variables for every interim step, and swizzling vectors / avoiding unncessary abstractions where convenient. Smaller things like manually computing the power where convenient and "pre"computing certain values into variables and reusing them where possible are also done.

== Source Code
=== main.rs <main>
#sourcecode("rust", "src/main.rs")
#pagebreak()
=== engine
==== mod.rs <enginemod>
#sourcecode("rust", "src/engine/mod.rs")
#pagebreak()
==== renderer.rs <renderer>
#sourcecode("rust", "src/engine/renderer.rs")
#pagebreak()
==== scene.rs <scene>
#sourcecode("rust", "src/engine/scene.rs")
#pagebreak()
==== ui.rs <ui>
#sourcecode("rust", "src/engine/ui.rs")
#pagebreak()
==== util.rs <util>
#sourcecode("rust", "src/engine/util.rs")
#pagebreak()
=== simulation
==== mod.rs <simmod>
#sourcecode("rust", "src/sim/mod.rs")
#pagebreak()
==== simdata.rs <simdata>
#sourcecode("rust", "src/sim/simdata.rs")
#pagebreak()
==== cascade.rs <cascade>
#sourcecode("rust", "src/sim/cascade.rs")
#pagebreak()
==== compute.rs <compute>
#sourcecode("rust", "src/sim/compute.rs")
#pagebreak()
==== fft.rs <simfft>
#sourcecode("rust", "src/sim/fft.rs")
#pagebreak()

=== shaders
==== lib.rs <shaderlib>
#sourcecode("rust", "shaders/src/lib.rs")
#pagebreak()
==== skybox.rs <skybox>
#sourcecode("rust", "shaders/src/skybox.rs")
#pagebreak()
==== ui.rs <ui>
#sourcecode("rust", "shaders/src/ui.rs")
#pagebreak()
==== sim/mod.rs <simmod>
#sourcecode("rust", "shaders/src/sim/mod.rs")
#pagebreak()
===== evolve_spectra.rs <evolve_spectra>
#sourcecode("rust", "shaders/src/sim/evolve_spectra.rs")
#pagebreak()
===== fft.rs <shaderfft>
#sourcecode("rust", "shaders/src/sim/fft.rs")
#pagebreak()
===== initial_spectra.rs <initial_spectra>
#sourcecode("rust", "shaders/src/sim/initial_spectra.rs")
#pagebreak()
===== process_deltas.rs <process_deltas>
#sourcecode("rust", "shaders/src/sim/process_deltas.rs")
#pagebreak()
=== build.rs <build>
#sourcecode("rust", "build.rs")
#pagebreak()
=== shared/lib.rs <sharedlib>
#sourcecode("rust", "shared/src/lib.rs")
#pagebreak()
=== cargo.toml <cargotoml>
#sourcecode("toml", "Cargo.toml")
=== rust-toolchain.toml <toolchaintoml>
#sourcecode("toml", "rust-toolchain.toml")
=== default.nix <nix>
#sourcecode("nix", "default.nix")

#pagebreak()
= Testing
== Testing Strategy
== Testing Table

#pagebreak()
= Evaluation
== Results
#figure(
    align(
      center,
      image("assets/ocean.png", fit: "contain", width: 100%),
    ),
    caption: [
      Calm Ocean. $L_0 = 40$, $L_1 = 106$, $L_2 = 180$, $U_10 = 5$, $F = 4000$, $lambda = 0.2$, $h = 500$
    ]
  )
#figure(
    align(
      center,
      image("assets/ocean2.png", fit: "contain", width: 100%),
    ),
    caption: [
      Choppy Ocean. $L_0 = 55$, $L_1 = 102$, $L_2 = 256$, $U_10 = 36$, $F = 10000$, $lambda = 0.2$, $h = 500$
    ]
  )
#figure(
    align(
      center,
      image("assets/ocean4.png", fit: "contain", width: 100%),
    ),
    caption: [
      Stormy Ocean. $L_0 = 41$, $L_1 = 106$, $L_2 = 180$, $U_10 = 62$, $F = 10000$, $lambda = 0.8$, $h = 500$
    ]
  )
#figure(
    align(
      center,
      image("assets/ocean3.png", fit: "contain", width: 100%),
    ),
    caption: [
      Alternative Angle. $L_0 = 41$, $L_1 = 106$, $L_2 = 180$, $U_10 = 62$, $F = 10000$, $lambda = 0.8$, $h = 500$
    ]
  )
#figure(
    align(
      center,
      image("assets/ocean5.png", fit: "contain", width: 100%),
    ),
    caption: [
      Early Morning Ocean. $L_0 = 20$, $L_1 = 124$, $L_2 = 256$, $U_10 = 0.5$, $F = 100000$, $lambda = 0.1$, $h = 30$
    ]
  )
== Evaluation Against Criteria
== Client Feedback
== Evaluation of Feedback

#pagebreak()
= Bibliography
#bibliography(
  "bibliography.yml",
  title:none,
  full:true,
  style: "ieee"
)
