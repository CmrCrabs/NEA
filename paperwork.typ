// Settings
#set par(justify: true)
#show link: underline
#set page(numbering: "1", margin: 2cm)
#set text(hyphenate: false)
#set heading(numbering: "1.")
#set text(12pt)

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
