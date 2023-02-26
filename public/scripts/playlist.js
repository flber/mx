// Playlist array
var files = [
  "music/<|º_º|>/1-lone-digger.ogg",
  "music/<|º_º|>/2-comics.ogg",
  "music/<|º_º|>/3-mighty-feat-jfth.ogg",
  "music/<|º_º|>/4-aftermath.ogg",
  "music/<|º_º|>/5-wonderland.ogg",
  "music/<|º_º|>/6-tattoos.ogg",
  "music/<|º_º|>/7-midnight-official-music-video.ogg",
  "music/<|º_º|>/8-russian.ogg",
  "music/<|º_º|>/9-wonda.ogg",
  "music/<|º_º|>/10-human-leather-shoes-for-crocodile-dandies.ogg",
  "music/<|º_º|>/11-lay-down.ogg"
];

// Current index of the files array
var i = 0;

// Get the audio element
var music_player = document.querySelector("#music_list audio");

// function for moving to next audio file
function next() {
  // Check for last audio file in the playlist
  if (i === files.length - 1) {
    i = 0;
  } else {
    i++;
  }

  // Change the audio element source
  music_player.src = files[i];
}

// Start the player
music_player.src = files[i];

// Listen for the music ended event, to play the next audio file
music_player.addEventListener('ended', next, false)
