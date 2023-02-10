const playerAPI = "http://localhost:27232/player";
const lyricsAPI = "http://localhost:27232/api/lyric?id=";

const min_update_interval = 50;
const max_update_interval = 500;

const lyrics_div = document.getElementById("lyric");
let songName = null;
let songId = null;
let lyrics_text = null;
let tlyrics_text = null;
let progress = 0;

function lyric_parser(lrc) {
    // Code from YesPlayerMusicOSD
    if (!lrc) {
        return null;
    }
    lines = lrc
        .split("\n")
        .map((nx) => {
            text = nx.split("]");
            lrc = text[1];
            if (!lrc) {
                return null;
            }
            _time = text[0].split("[")[1];
            if (!_time) {
                return null;
            }
            time = _time.split(":");
            min = Number(time[0]);
            sec = Number(time[1]);
            if (lrc.length < 1) {
                return [min * 60 + sec, " "];
            } else {
                return [min * 60 + sec, lrc];
            }
        })
        .filter((x) => Boolean(x));
    if (lines.length > 0) {
        return lines;
    } else {
        return null;
    }
}

function getLyricsInfo() {
    // 如果没有播放歌曲的话，那么就不用获取歌词了
    if (!songId) lyrics = songName || "No song is playing";
    console.log(lyricsAPI + songId);
    fetch(lyricsAPI + songId)
        .then((resp) => resp.json())
        .then((data) => {
            let lyric = data.lrc?.lyric || null; // 歌词
            let tlyric = data.tlyric?.lyric || null; // 翻译歌词

            if (!lyric) {
                // 暂时没有歌词，使用歌名来替代
                lyrics_text = "[999:99.99]" + songName;
                // 翻译歌词就不用放了
                tlyrics_text = null;
            } else if (lyric.search("纯音乐，请欣赏") != -1) {
                // 纯音乐，用歌名替代lyc，用纯音乐替代tlyc、
                lyric = "[999:99.99]" + songName;
                tlyric = "[999:99.99]纯音乐，请欣赏";
            }
            lyrics_text = lyric_parser(lyric);
            tlyrics_text = lyric_parser(tlyric);
        });
}

function getPlayerInfo() {
    let sameSong = false;
    fetch(playerAPI)
        .then((resp) => resp.json())
        .then((data) => {
            // 获取当前播放信息
            if (!data) {
                songName = null;
                songId = null;
            } else {
                // 如果能够获取到当前播放信息的话
                currentTrack = data.currentTrack;
                if (currentTrack) {
                    if (currentTrack.id == songId) {
                        // 如果播放的还是同一首歌的话那就不用再去获取歌词了
                        sameSong = true;
                    }
                    songName = currentTrack.name;
                    songId = currentTrack.id;
                }
                // 获取当前播放进度
                progress = data.progress;
            }
        });
    return sameSong;
}

function updateInfo() {
    // 获取当前播放信息
    if (!getPlayerInfo() || !lyrics_text) {
        // 如果不是同一首歌，那么获取歌词信息
        // 然后如果歌词之前获取失败了的话（可能吗？），也要重新获取
        getLyricsInfo();
    }
}

function resizeWindow() {
    // 要是歌词栏高度超过了窗口高度的话，稍微拉一下。
    let height = lyrics_div.offsetHeight;
    if (height > window.innerHeight) {
        lyricsapi.resizeLyricsWindow(height);
    }
}

async function updateLyrics() {
    while (true) {
        updateInfo();
        resizeWindow();
        await new Promise((r) => setTimeout(r, 5000));
    }
}

updateLyrics();
