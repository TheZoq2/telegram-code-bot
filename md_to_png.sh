#!/bin/bash

pdfname="/tmp/cody.pdf"
pngname="/tmp/cody.png"

# Convert the markdown to a pdf
pandoc -f gfm -o "$pdfname" --template=nonumbertemplate.tex -i

convert -density 125 $pdfname $pngname
convert -trim -border 5x5 $pngname $pngname
