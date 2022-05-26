#!/usr/bin/env sh

# run:
# > ./wasm-build-all.sh
# > miniserve generated
# > xdg-open "localhost:8080/all.html"

echo "<!DOCTYPE html>
<html>

<head>
	<meta charset="UTF-8" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0" />
	<script type="module" src="./load.js"></script>
</head>

<body>
	<h1>Demos</h1>
	<ul>" > "generated/all.html"
	
clear

for x in examples/*;
do
	EXAMPLE=${x#"examples/"}
	OUTDIR="generated/${EXAMPLE}" EXAMPLE=${EXAMPLE} ./wasm-build.sh
	echo "		<li><a href='/${EXAMPLE}/index.html'>${EXAMPLE}</a></li>" >> "generated/all.html"
done

echo "
	</ul>
</body>

</html>" >> "generated/all.html"