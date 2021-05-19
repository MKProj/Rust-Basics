build: 
	make mdbook
	make tex
	make git

git: 
	git add -A
	git commit -m "update" 
	git push -u origin main

mdbook: 
	mdbook build

tex: 
	cd Tex && pdflatex main x 2