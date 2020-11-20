### Grimoire

Inspired by the [[grimoire of Fred Bednarski]](https://fdisk.space/grimoire/), this is my place to put assorted snippets of code (which may or may not be cursed, so use at your own risk). Have fun, poke around, but remember that this information may be totally borked.

To change the extensions of audio files in a directory:
> for file in *.opus; do ffmpeg -i ${file%.opus}.opus -c copy ${file%.opus}.ogg && rm ${file%.opus}.opus; done

To download the mp3 audio (or the best audio if that fails) of a youtube video:
> youtube-dl --add-metadata -i -x -f mp3/bestaudio "video_link"

To download a playlist of music from youtube (more stable for playlists than youtube-dl):
> youtube-dlc -x -f mp3/bestaudio

To forward your port 8388 to a server's port 8384 (can be used to get the web interface for syncthing remotely):
> ssh -L 127.0.0.1:8388:127.0.0.1:8384 username@yourserver

To sanitize a file name (the .opus can be any extension):
> mv "$file" "$(echo "${file%.opus}" | iconv -cf UTF-8 -t ASCII//TRANSLIT | tr -d '[:punct:]' | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | sed "s/-\+/-/g;s/\(^-\|-\$\)//g").opus"

To push file changes over ssh:
> rsync -azP --delete ~/Folder/ user@123.456.8.910:/home/name/Folder

To dither and shrink an image (courtesy of [[Thomasorus]](https://merveilles.town/web/accounts/33648)):
> mogrify -path $3 -filter Triangle -define filter:support=2 -thumbnail $2 -unsharp 0.25x0.08+8.3+0.045 -dither FloydSteinberg -type Grayscale -colors 2 -posterize 136 -quality 82 -define jpeg:fancy-upsampling=off -define png:compression-filter=5 -define png:compression-level=9 -define png:compression-strategy=1 -define png:exclude-chunk=all -interlace none -colorspace sRGB $1/*

*Last updated: {{date}}*
