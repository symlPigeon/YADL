const playerAPI = "http://localhost:27232/player";
const lyricsAPI = "http://localhost:27232/api/lyric?id=";

const min_update_interval = 50;
const max_update_interval = 500;

let songName = null; // 歌曲名
let songId = null; // 歌曲ID
let songAlb = null; // 专辑名
let lyrics_text = null; // 歌词
let tlyrics_text = null; // 翻译的歌词
let progress = 0; // 进度
let current_idx = 0; // 当前下标
let next_time = 0; // 下一句歌词的时间
let offset = 0.5; // 歌词偏移量


function resizeWindow() {
    // 要是歌词栏高度超过了窗口高度的话，稍微拉一下。
    var lyrics_div = document.getElementById("all");
    var height = lyrics_div.offsetHeight + 20;
    if (height != window.innerHeight) {
        // 宽度别拉，会出奇怪的问题……
        lyricsapi.resizeLyricsWindow(height, window.innerWidth);
    }
}

function getTranslationLine() {
    if (!tlyrics_text || tlyrics_text.length == 0) {
        return;
    }
    if (progress < tlyrics_text[0][0]) {
        // 还没放到第一句歌词的时间，应该吧
        return;
    }
    for (i = 1; i < tlyrics_text.length; i++) {
        if (tlyrics_text[i][0] > progress) {
            return tlyrics_text[i - 1][1];
        }
    }
    // 应该是最后一句了
    return tlyrics_text[tlyrics_text.length - 1][1];
}

function getCurrentLine() {
    // 获取当前的歌词
    for (i = 1; i < lyrics_text.length; i++) {
        if (lyrics_text[i][0] > progress) {
            current_idx = i - 1;
            next_time = lyrics_text[i][0];
            return;
        }
    }
    // 应该是最后一句了
    current_idx = lyrics_text.length - 1;
}

function setLyrics() {
    // 歌词部分
    var lyrics_div = document.getElementById("lyric");
    lyrics_div.innerHTML = "<p>" + lyrics_text[current_idx][1] + "</p>";
    // 翻译部分
    if (tlyrics_text) {
        // 如果有当前的翻译歌词
        var tlyric = getTranslationLine();
        if (tlyric) {
            lyrics_div.innerHTML += "<p>" + tlyric + "</p>";
        }
    }
    resizeWindow();
}

function updateLyricsLine() {
    // 下次更新的时间
    var next_update = max_update_interval;
    if (!lyrics_text) {
        // 没有获取到歌曲信息，应该不会出现吧
        var lyrics_div = document.getElementById("lyric");
        lyrics_div.innerHTML = "<p>Happy Coding With Music!</p>";
        resizeWindow();
    } else {
        var lyc_length = lyrics_text.length;
        if (progress > lyrics_text[current_idx][0]) {
            // 已经过了当前这句歌词的时间了
            var next_index = current_idx + 1;
            if (next_index < lyc_length) {
                // 还有下一句歌词
                if (progress < lyrics_text[next_index][0]) {
                    // 还没到下一句歌词，不要等下一句歌词到了再更新
                    next_update = (next_time - progress) * 1000;
                    setLyrics(); // 防止出现开始的时候歌词不显示的问题
                } else {
                    // 到了
                    var next_2_index = next_index + 1;
                    if (next_2_index < lyc_length) {
                        // 还不是最后一句歌词
                        if (progress < lyrics_text[next_2_index][0]) {
                            // 还没到下下句歌词
                            current_idx = next_index;
                            setLyrics();
                            // 设置下次更新时间
                            next_time = lyrics_text[next_2_index][0];
                            next_update = (next_time - progress) * 1000;
                        } else {
                            // 已经到了至少下下句歌词了，很神奇，不知道发生了什么
                            getCurrentLine();
                            setLyrics();
                            next_update = (next_time - progress) * 1000;
                        }
                    } else {
                        // 最后一句歌词
                        current_idx = next_index;
                        setLyrics();
                        next_update = max_update_interval;
                    }
                }
            } else {
                // 没下一句了
                next_update = max_update_interval;
            }
        } else {
            // 还没到当前这句歌词的时间
            getCurrentLine();
            setLyrics();
            next_update = (next_time - progress) * 1000;
        }
    }
    if (next_update > max_update_interval) {
        next_update = max_update_interval;
    }
    if (next_update < min_update_interval) {
        next_update = min_update_interval;
    }
    setTimeout(updateInfo, next_update);
}

function lyricParser(lrc) {
    // Code from YesPlayerMusicOSD
    if (!lrc) {
        return null;
    }
    lines = lrc
        .split("\n")
        .map((nx) => {
            text = nx.split("]");
            lrc = text[1];
            // 应该是空的段落
            if (!lrc) {
                return "";
            }
            _time = text[0].split("[")[1];
            if (!_time) {
                return "";
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

async function getLyricsInfo() {
    // 如果没有播放歌曲的话，那么就不用获取歌词了
    if (!songId) {
        console.log("No song playing. No lyrics to fetch.");
        console.log("Song ID: " + songId);
        if (songName != null && songAlb != null) {
            lyrics_text = lyricParser("[999:99.99]" + songName);
            tlyrics_text = lyricParser("[000:00.00]" + songAlb);
        } else {
            lyrics_text = null;
            tlyrics_text = null;
        }
        return;
    }
    await fetch(lyricsAPI + songId)
        .then((resp) => resp.json())
        .then((data) => {
            var lyric = data.lrc?.lyric || ""; // 歌词
            var tlyric = data.tlyric?.lyric || ""; // 翻译歌词

            if (!lyric) {
                // 暂时没有歌词，使用歌名来替代
                lyric = "[999:99.99]" + songName;
                // 翻译歌词就不用放了，不如放个专辑名字:)
                tlyric = "[000:00.00]" + songAlb;
            } else if (lyric.search("纯音乐，请欣赏") != -1) {
                // 纯音乐，用歌名替代lyc，用纯音乐替代tlyc、
                lyric = "[999:99.99]" + songName;
                tlyric = "[000:00.00]纯音乐，请欣赏";
            }
            lyrics_text = lyricParser(lyric);
            tlyrics_text = lyricParser(tlyric);
        });
}

async function getPlayerInfo() {
    return await fetch(playerAPI)
        .then((resp) => resp.json())
        .then((data) => {
            var sameSong = false;
            // 获取当前播放信息
            if (!data) {
                // 无法获取
                songName = null;
                songId = null;
                songAlb = null;
                progress = 0;
                lyrics_text = null;
                tlyrics_text = null;
                current_idx = 0;
                next_time = 0;
            } else {
                // 如果能够获取到当前播放信息的话
                currentTrack = data.currentTrack;
                if (currentTrack.id == songId) {
                    // 如果播放的还是同一首歌的话那就不用再去获取歌词了
                    sameSong = true;
                }
                songName = currentTrack.name;
                songId = currentTrack.id;
                songAlb = currentTrack.al?.name || "";
                // 获取当前播放进度
                progress = data.progress + offset;
            }
            return sameSong;
        });
}

async function updateInfo() {
    // 获取当前播放信息
    var sameSong = await getPlayerInfo().then((sameSong) => {
        return sameSong;
    });
    if (!sameSong) {
        console.log("Reload lyrics");
        // 如果不是同一首歌，那么获取歌词信息
        // 然后如果歌词之前获取失败了的话（可能吗？），也要重新获取
        await getLyricsInfo();
        // 要重新获取当前行
        getCurrentLine();
    }
    // 更新歌词
    updateLyricsLine();
}
