#!/usr/bin/python

import sys
import re

lines = []
match = 0

for line in sys.stdin:
	lines.append(line)

for line in lines:
	if "{{title}}" in line:
		title = "Some Page :)"
		for c_line in lines:
		    #DOC: finds first h2 and uses text inside as title
			if "<h2>" in c_line and match == 0:
				title = re.search(r"<h2>[^<]+<\/h2>", c_line).group()[4:-5]
				match = 1
		line = line.replace("{{title}}", title)
	sys.stdout.write(line)

