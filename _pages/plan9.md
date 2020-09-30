### My Experience with Plan9

I first encountered Plan9 on Devine Lu Linvega's site and was immediately interested in its minimalist design and "more unix than unix" philosophy. If you've never heard of it before, Plan9 is a research operating system developed by Bell Labs in 1992, and takes the unix principle of everything as a file to the extreme. It has a lot of other interesting features and quirks, and if you're interested in learning more about them I would highly recommend the the Plan9 [[about page]](https://9p.io/plan9/about.html) or the [[several papers]](https://9p.io/sys/doc/) written about the operating system.

However, for all of the documentation I had a hard time actually getting Plan9 (or, more accurately, 9front, a more maintained fork of Plan9) to run on my pi 4. Though a few of the issues were certainly due to my own inexperience, I think there were a few which others might also face while trying to get it working on their own pi 4.

To start, the image file for the pi 4 can be found [[here]](http://9front.org/releases/) (the pi3.img.gz file).

Confusingly though (because the 9front website says otherwise), the image file cannot simply be moved to a FAT32 formated micro sd card, it has to be flashed to the drive, using something like [[balenaetcher]](https://www.balena.io/etcher). 

Additionally, by default the pi does not boot in something called "hdmi safe mode", which allows for maximum compatibility with hdmi output. This means that though the pi may be booting properly (as indicated by the green status led quickly blinking once it is first turned on and then more or less turning off), the screen will not display any output. To fix this I needed to set a parameter called `hdmi_safe` in the `config.txt` file to `1`. That puts the pi into safe mode, which fixed the issue for me. 

(Also, something stupid I accidentally did which caught me up for a minute was trying to get hdmi output from the second micro hdmi port on the pi, which by default is disabled)

And with that done, your pi 4 should be good to go! For me that was it, but if you have any issues by all means feel free to contact me (though be aware that I'm not at all experienced with this, I just managed to get it working on my system).

*Last updated: {{date}}*
