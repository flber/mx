### variables ###
set TERM xterm-kitty
set fish_greeting
set BROWSER /usr/bin/qutebrowser
set EDITOR /usr/local/bin/micro
set VISUAL /usr/local/bin/micro
set PAGER /usr/bin/bat
set LS_COLORS "*.html=37;41:*.ts=34"
export PATH="/usr/local/bin/node:$PATH"
export PATH="$HOME/.emacs.d/bin:$PATH"
export PATH="$HOME/todo:$PATH"
export GOMPHOTHERIUM_SERVER='https://merveilles.town'
export GOMPHOTHERIUM_ACCESS_TOKEN='LiQnf95Iao8iNWraiIlwiO1lpWM3uC5kKJvMwRVy6UU'

# eval (python -m virtualfish)
# export WORKON_HOME=$HOME/.virtualenvs
# export VIRTUALENVWRAPPER_PYTHON=/usr/bin/python3
# export VIRTUALENVWRAPPER_VIRTUALENV_ARGS=' -p /usr/bin/python3 '
# export PROJECT_HOME=$HOME/Devel
# /usr/bin/virtualenvwrapper.sh


### startup commands ###
starship init fish | source
# picom --experimental-backends &
# /home/benh/wal -a "95" -i 'Pictures/backgrounds/iceland_road.jpg' -xq
# ~/.fehbg &
# wal -a "95" -i ~/Pictures/backgrounds/dark_city_nyc.jpeg -n

### aliases ###
alias cls='clear && ls'
alias cl='clear'
alias cdsh='cd Documents/School/Year4/Semester2/'
alias cdmx='cd ~/projects/mx'
alias cdwn='cd ~/.wine/drive_c'
alias ..='cd ..'

alias lx='ls -X'
alias ls='exa --sort=ext'
alias la="exa -lah"

alias div='echo -------------------------------------'
alias calconnect='tilp --calc=ti84+ --cable=DirectLink'
alias battery='upower -i /org/freedesktop/UPower/devices/battery_BAT0 | grep -E "state|to\ full|percentage"'
alias typora='./bin/Typora-linux-x64/Typora'
alias nmrn='nim c -r --verbosity:0'
alias oxipng='oxipng -o 3 -i 1 --strip all'
alias dt='date +"%d%m%y"'
alias sshpi='ssh benh@(cat ~/repos/mxip/ip.txt)'
alias shot='grim -g (slurp) > (date +"%d%m%y").png && rm *_grim.png'
alias getall='wget -m -k -K -E -l 7 -t 6 -w 5 -e robots=off'

alias gs='git status'

alias confish='micro ~/.config/fish/config.fish'
alias constar='micro ~/.config/starship.toml'

alias pt='proctrack'

# alias cmus='proctrack cmus'
# alias micro='proctrack micro'


### symlinks ###

### functions ###
function clean-unzip --argument zipfile
    if not test (echo $zipfile | string sub --start=-4) = .zip
        echo (status function): argument must be a zipfile
        return 1
    end

    if is-clean-zip $zipfile
        unzip $zipfile
    else
        set zipname (echo $zipfile | trim-right '.zip')
        mkdir $zipname || return 1
        unzip $zipfile -d $zipname
    end
end

function file-exists --argument file
    test -e $file
end

function ip-addr
    curl icanhazip.com
end

function random-file
    find . -type f | shuf -n1
end

function fontviewer
    set choice (fc-list | awk '{print $1}' | sed 's/://g' | dmenu -l 20 -p 'Fontviewer: ')
    display "$choice"
end

function gq --argument remote branch
    git add .
    git commit -m "quick update"
    git push $remote $branch
end

function gp --argument remote branch
    git push $remote $branch
end

function gacm --argument message
    set dir (basename $PWD)
    # exec /home/benh/projects/mx/twtxtc tweet "[$dir] $message"
    git add -A
    git commit -m $message
end

function imgopt --argument size colors from
    set to (echo $from | sed 's/\.[^.]*$//')-dith.gif

    magick $from \
        -format png \
        -filter Triangle \
        -define filter:support=2 \
        # a number
        -thumbnail $size \
        -unsharp 0.25x0.08+8.3+0.045 \
        -dither FloydSteinberg \
        # -ordered-dither 4x4 \
        -colors $colors \
        -posterize 136 \
        -quality 82 \
        -define jpeg:fancy-upsampling=off -define png:compression-filter=5 -define png:compression-level=9 -define png:compression-strategy=1 -define png:exclude-chunk=all \
        -interlace none \
        # -colorspace gray \
        -normalize \
        # a source image
        $to

    # rotate $to 180
end

function imgbow --argument size from
    mogrify \
        -format gif \
        # a number
        -thumbnail $size \
        # a source image
        -type Grayscale \
        -threshold 30% \
        $from
end

function rotate --argument img ang
    convert $img -rotate $ang $img
end

function contrast --argument percent img
    convert -modulate 100,$percent,100 $img $img
end

function to_mp3 --argument INFILE OUTFILE
    ffmpeg -n -i $INFILE -c:a libmp3lame -q:a 1 -ar 44100 -map_metadata 0 -map_metadata 0:s:0 -id3v2_version 3 -vn $OUTFILE
end

function vidcut --argument INFILE START END OUTFILE
    ffmpeg -i $INFILE -ss $START -t $END -async 1 $OUTFILE
end

function rpg
    rpg-cli $argv
    cd (rpg-cli "pwd")
end

function edmx
    cd ~/projects/mx
    micro templates/default.html docs/style.css .pillar.toml pages/home.gn
end

function sanitize
    for file in *
        set ext (string match -r "\.\w*\$" $file | \
            string trim --chars=.)

        set new (echo "{$file}" | \
            iconv -cf UTF-8 -t ASCII//TRANSLIT | \
            tr -d '[:punct:]' | \
            tr '[:upper:]' '[:lower:]' | \
            tr ' ' '-' | \
            sed "s/-\+/-/g;s/\(^-\|-\$\)//g")

        set final (string replace "$ext" ".$ext" "$new")

        if [ "$file" != "$final" ]
            mv "$file" "$final"
        end
    end
end

function uxnrn --argument name
    set tal $name".tal"
    set rom "bin/"$name".rom"
    ./bin/uxnasm $tal $rom
    ./bin/uxnemu $rom
end

function syncmx
    set ip "107.202.150.167" # (cat ~/repos/mxip/ip.txt)
    echo "syncing to" $ip
    rsync -vauzhP --exclude=.git ~/projects/mx/ benh@[$ip]:/home/benh/Documents/Mineral-Existence
end

function fm --argument func
    awk '$2 == "'$func'" { print $0 }' ~/.config/fish/config.fish | sed 's/function //' | sed 's/ --argument /: /'
end



# opam configuration
source /home/benh/.opam/opam-init/init.fish >/dev/null 2>/dev/null; or true
