// Settings
#set par(justify: true)
#show link: underline
#set page(numbering: "1", margin: 2cm) 
#set text(hyphenate: false)
#set heading(numbering: "1.")
#set text(12pt)
#set enum(numbering: "1.1", full: true)

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

== Client

=== Introduction
The client is Jahleel Abraham. They are a game developer who require a physically based, performant, configurable simulation of an ocean for use in their game.

=== Questions
+ Functionality
  + "What specific ocean phenomena need to be simulated? (e.g. waves, foam, spray, currents)"
  + "What parameters of the simulation need to be configurable?"
  + "Does there need to be an accompanying GUI?"
+ Visuals
  + "Do I need to implement an atmosphere / skybox?"
  + "Do I need to implement a PBR water shader?"
  + "Do I need to implement caustics, reflections, or other light-related phenomena?"
+ Technologies
  + "Are there any limitations due to existing technology?"
  + "Does this need to interop with existing shader code?"
+ Scope
  + "Are there limitations due to the target device(s)?"
  + ""

=== Notes

== Research
=== Technologies
- mention naga for cross compilation
- can interop with any major engine
- wgpu and rust can work directly w/ c / cpp bindings
=== Algorithms
=== Formulae
=== Prototyping
prototyped using tech stack for basic project
https://github.com/CmrCrabs/chaotic-attractors

== Objectives

#pagebreak()
= Bibliography
#bibliography(
  "bibliography.yml",
  title:none,
  full:true,
  style: "ieee"
)
